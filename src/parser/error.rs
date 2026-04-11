use std::ops::Range;
use ariadne::{Color, Report, ReportKind, Span};

use crate::token::Token;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct CodeArea(pub usize, pub usize);

impl CodeArea {
    pub fn from_span(range: Range<usize>) -> Self {
        Self(range.start, range.end)
    }
}

impl Span for CodeArea {
    type SourceId = &'static str;

    fn source(&self) -> &Self::SourceId {
        &"code"
    }

    fn start(&self) -> usize {
        self.0
    }

    fn end(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Clone)]
pub enum ParsingError {
    CustomError {
        text: String,
        area: CodeArea,
    },
    SyntaxError {
        expected: String,
        found: Token,
        area: CodeArea,
    },
    UnexpectedEndOfFile {
        area: CodeArea,
    },
    InvalidEscapeCharacter {
        character: char,
        area: CodeArea,
    },
}

pub fn print_error(code: &str, error: ParsingError) {
    let (title, area, kind) = match error {
        ParsingError::SyntaxError { expected, found, area } => (format!("Expected '{expected}' (Found '{found:?}')"), area, ReportKind::Error),
        ParsingError::CustomError { text, area } => (text.to_string(), area, ReportKind::Error),
        ParsingError::UnexpectedEndOfFile { area } => ("Unexpected end of file".to_string(), area, ReportKind::Error),
        ParsingError::InvalidEscapeCharacter { character, area } => (format!("Invalid escape character: '{character}'"), area, ReportKind::Error),
    };

    let report = Report::build(kind, area)
        .with_message(&title)
        .with_label(
            ariadne::Label::new(area)
                .with_message(&title)
                .with_color(Color::Red),
        )
        .finish();

    report.print(ariadne::sources([("code", code)]))
        .expect("Failed to print error report");
}
