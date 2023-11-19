use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    // <operator> <value>
    #[token("|<")]
    Load,
    #[token("[<]")]
    Define,
    #[token("[>]")]
    Jump,
    #[token("<=")]
    Assign,
    #[token("=>")]
    Overwrite,
    #[token("<=>")]
    Swap,

    // <operator> <value>
    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token(">-<")]
    Xor,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("%")]
    Percent,
    #[token("^")]
    Circumflex,
    #[token("><")]
    Unequal,
    #[token("=")]
    Equal,
    #[token("<")]
    LessThan,
    #[token("=<")]
    LessThanOrEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanOrEqual,

    // if statements
    // <operator>
    #[token("?")]
    Truthy,
    #[token("!")]
    Falsy,
    #[token("$")]
    Exists,
    #[token("°")]
    Empty,

    // parentheses
    #[token("{")]
    CurlyBracketOpen,
    #[token("}")]
    CurlyBracketClose,

    #[token("@")]
    At,
    #[token(r"\")]
    Backslash,
    #[token("~")]
    Tilde,

    // <operator> <unaryoperator> <value>
    #[token("§")]
    Reader,

    // symbol
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Symbol,

    // value
    #[token("null")]
    Null,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("µ")]
    Mu,
    #[regex(r"/(?:\.|[^/])*/")]
    StringLiteral,
    #[regex(r"[0-9]+")]
    IntegerLiteral,
    #[regex(r"[0-9]+\.[0-9]+")]
    FloatLiteral,

    #[regex(r"[ \r\t\n]*|(#[^\n]*\n)", logos::skip)]
    Error,
}
