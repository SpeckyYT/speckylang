use std::ops::Range;
use colored::Colorize;
use lyneate::{Report, Theme, ThemeChars, ThemeSizing, ThemeEffects};

use crate::token::Token;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CodeArea(pub usize, pub usize);

impl CodeArea {
    pub fn from_span(range: Range<usize>) -> Self {
        Self(range.start, range.end)
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
    }
}

pub fn print_error(code: &str, error: ParsingError) {
    let (title, area) = match error {
        ParsingError::SyntaxError { area, .. } => ( "Syntax error".to_string(), area ),
        ParsingError::CustomError { text, area } => ( text.to_string(), area ),
        ParsingError::UnexpectedEndOfFile { area } => ( "Unexpected end of file".to_string(), area ),
    };

    println!(
        "{} {}",
        "Error:".bright_red(),
        title,
    );

    Report::new_char_spanned(
        code,
        [
            (
                area.0..area.1,
                "shit".to_string(),
                (255,0,100)
            )
        ]
    )
    .with_theme(
        Theme {
            chars: ThemeChars::box_drawing_chars(),
            effects: ThemeEffects::none(),
            sizing: ThemeSizing::default(),
        }
    )
    .display();
}
