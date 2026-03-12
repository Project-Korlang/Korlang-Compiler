#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Bug,
}

impl DiagnosticLevel {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Bug => "internal compiler error",
        }
    }
}

#[derive(Debug)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Span,
    pub code: Option<&'static str>,
    pub labels: Vec<(Span, String)>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message: message.into(),
            span,
            code: None,
            labels: Vec::new(),
        }
    }

    pub fn with_code(mut self, code: &'static str) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push((span, message.into()));
        self
    }

    pub fn warning(message: impl Into<String>, span: Span) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            span,
            code: None,
            labels: Vec::new(),
        }
    }

    pub fn note(message: impl Into<String>, span: Span) -> Self {
        Self {
            level: DiagnosticLevel::Note,
            message: message.into(),
            span,
            code: None,
            labels: Vec::new(),
        }
    }

    pub fn bug(message: impl Into<String>, span: Span) -> Self {
        Self {
            level: DiagnosticLevel::Bug,
            message: message.into(),
            span,
            code: None,
            labels: Vec::new(),
        }
    }

    pub fn report(&self, source: &str, file_name: &str) {
        let level_color = match self.level {
            DiagnosticLevel::Error => "\x1b[31;1m", // Bold Red
            DiagnosticLevel::Warning => "\x1b[33;1m", // Bold Yellow
            DiagnosticLevel::Note => "\x1b[36;1m", // Bold Cyan
            DiagnosticLevel::Bug => "\x1b[35;1m", // Bold Magenta
        };
        let reset = "\x1b[0m";

        let label = if let Some(code) = self.code {
            format!("{}[{}]", self.level.to_str(), code)
        } else {
            self.level.to_str().to_string()
        };

        eprintln!("{}[{}]{}: {}", level_color, label, reset, self.message);
        eprintln!("  --> {}:{}:{}", file_name, self.span.start.line, self.span.start.column);

        let lines: Vec<&str> = source.lines().collect();
        
        // Report primary span
        self.draw_span(&lines, self.span, "", level_color, true);

        // Report secondary labels
        for (span, msg) in &self.labels {
            self.draw_span(&lines, *span, msg, "\x1b[36m", false); // Cyan for notes
        }
        eprintln!();
    }

    fn draw_span(&self, lines: &[&str], span: Span, message: &str, color: &str, primary: bool) {
        let reset = "\x1b[0m";
        if span.start.line > 0 && span.start.line <= lines.len() {
            let line_idx = span.start.line - 1;
            let line = lines[line_idx];
            eprintln!("   |");
            eprintln!("{:3} | {}", span.start.line, line);
            
            let padding = " ".repeat(span.start.column.saturating_sub(1));
            let highlight_char = if primary { "^" } else { "-" };
            let highlight_len = if span.end.offset > span.start.offset && span.end.line == span.start.line {
                span.end.offset - span.start.offset
            } else {
                1
            };
            let highlight = highlight_char.repeat(highlight_len);
            if message.is_empty() {
                eprintln!("   | {}{}{}{}", padding, color, highlight, reset);
            } else {
                eprintln!("   | {}{}{}{} {}", padding, color, highlight, reset, message);
            }
        }
    }
}
