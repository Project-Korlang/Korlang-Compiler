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
}

impl Diagnostic {
    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message: message.into(),
            span,
        }
    }

    pub fn warning(message: impl Into<String>, span: Span) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            span,
        }
    }

    pub fn note(message: impl Into<String>, span: Span) -> Self {
        Self {
            level: DiagnosticLevel::Note,
            message: message.into(),
            span,
        }
    }

    pub fn bug(message: impl Into<String>, span: Span) -> Self {
        Self {
            level: DiagnosticLevel::Bug,
            message: message.into(),
            span,
        }
    }

    pub fn report(&self, source: &str, file_name: &str) {
        let color = match self.level {
            DiagnosticLevel::Error => "\x1b[31;1m", // Bold Red
            DiagnosticLevel::Warning => "\x1b[33;1m", // Bold Yellow
            DiagnosticLevel::Note => "\x1b[36;1m", // Bold Cyan
            DiagnosticLevel::Bug => "\x1b[35;1m", // Bold Magenta
        };
        let reset = "\x1b[0m";

        eprintln!("{}[{}]{}: {}", color, self.level.to_str(), reset, self.message);
        eprintln!("  --> {}:{}:{}", file_name, self.span.start.line, self.span.start.column);

        let lines: Vec<&str> = source.lines().collect();
        if self.span.start.line > 0 && self.span.start.line <= lines.len() {
            let line_idx = self.span.start.line - 1;
            let line = lines[line_idx];
            eprintln!("   |");
            eprintln!("{:3} | {}", self.span.start.line, line);
            
            let padding = " ".repeat(self.span.start.column.saturating_sub(1));
            let highlight_len = if self.span.end.offset > self.span.start.offset {
                self.span.end.offset - self.span.start.offset
            } else {
                1
            };
            let highlight = "^".repeat(highlight_len);
            eprintln!("   | {}{}{}{}", padding, color, highlight, reset);
        }
        eprintln!();
    }
}
