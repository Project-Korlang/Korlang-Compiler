use korlang_compiler::lexer::Lexer;
use korlang_compiler::parser::Parser;
use korlang_compiler::sema::Sema;
use std::collections::HashMap;
use std::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Default)]
struct Backend {
    client: Client,
    docs: Mutex<HashMap<Url, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::FULL,
            )),
            semantic_tokens_provider: Some(
                SemanticTokensServerCapabilities::SemanticTokensOptions(
                    SemanticTokensOptions {
                        legend: SemanticTokensLegend {
                            token_types: vec![
                                SemanticTokenType::KEYWORD,
                                SemanticTokenType::STRING,
                                SemanticTokenType::NUMBER,
                                SemanticTokenType::VARIABLE,
                            ],
                            token_modifiers: vec![],
                        },
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                        range: None,
                        ..Default::default()
                    },
                ),
            ),
            document_formatting_provider: Some(OneOf::Left(true)),
            definition_provider: Some(OneOf::Left(true)),
            rename_provider: Some(OneOf::Left(true)),
            ..Default::default()
        };
        Ok(InitializeResult {
            capabilities,
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        let _ = self.client.log_message(MessageType::INFO, "Korlang LSP initialized").await;
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokens>> {
        let text = self
            .docs
            .lock()
            .unwrap()
            .get(&params.text_document.uri)
            .cloned()
            .unwrap_or_default();
        let mut data = Vec::new();
        let tokens = Lexer::new(&text).tokenize().unwrap_or_default();
        let mut last_line = 0u32;
        let mut last_col = 0u32;
        for tok in tokens {
            let (token_type, lexeme_len) = match tok.kind {
                korlang_compiler::lexer::TokenKind::Keyword(_) => (0u32, 1u32),
                korlang_compiler::lexer::TokenKind::StringLiteral(_) => (1u32, 1u32),
                korlang_compiler::lexer::TokenKind::IntLiteral(_) | korlang_compiler::lexer::TokenKind::FloatLiteral(_) => (2u32, 1u32),
                korlang_compiler::lexer::TokenKind::Identifier(_) => (3u32, 1u32),
                _ => continue,
            };
            let line = (tok.span.start.line as u32).saturating_sub(1);
            let col = (tok.span.start.column as u32).saturating_sub(1);
            let delta_line = line - last_line;
            let delta_col = if delta_line == 0 { col - last_col } else { col };
            data.extend([delta_line, delta_col, lexeme_len, token_type, 0]);
            last_line = line;
            last_col = col;
        }
        Ok(Some(SemanticTokens { result_id: None, data }))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.docs
            .lock()
            .unwrap()
            .insert(params.text_document.uri.clone(), params.text_document.text.clone());
        self.publish_diagnostics(&params.text_document.uri, &params.text_document.text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.last() {
            self.docs
                .lock()
                .unwrap()
                .insert(params.text_document.uri.clone(), change.text.clone());
            self.publish_diagnostics(&params.text_document.uri, &change.text).await;
        }
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let text = self
            .docs
            .lock()
            .unwrap()
            .get(&params.text_document.uri)
            .cloned()
            .unwrap_or_default();
        Ok(Some(vec![TextEdit {
            range: Range::new(Position::new(0, 0), Position::new(u32::MAX, 0)),
            new_text: text,
        }]))
    }

    async fn goto_definition(&self, _: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        Ok(None)
    }

    async fn rename(&self, _: RenameParams) -> Result<Option<WorkspaceEdit>> {
        Ok(Some(WorkspaceEdit::default()))
    }
}

impl Backend {
    async fn publish_diagnostics(&self, uri: &Url, text: &str) {
        let mut diags = Vec::new();
        if let Ok(tokens) = Lexer::new(text).tokenize() {
            if let Ok(program) = Parser::new(tokens).parse_program() {
                if let Err(errs) = Sema::new().check_program(&program) {
                    for e in errs {
                        diags.push(Diagnostic {
                            range: Range::new(
                                Position::new((e.span.start.line - 1) as u32, (e.span.start.column - 1) as u32),
                                Position::new((e.span.end.line - 1) as u32, (e.span.end.column - 1) as u32),
                            ),
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: e.message,
                            ..Default::default()
                        });
                    }
                }
            }
        }
        let _ = self.client.publish_diagnostics(uri.clone(), diags, None).await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend {
        client,
        docs: Mutex::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
