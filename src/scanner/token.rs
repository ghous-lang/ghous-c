/*
 * @author: dwclake
 */

use crate::prelude::*;

use std::sync::Arc;

/// Contains a token's type and position, and file it belongs to
#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    pub ttype: TokenType<'a>,
    pub file: Arc<str>,
    pub pos: Position
}

impl<'a> Token<'a> {
    pub fn new(ttype: TokenType<'a>, file: Arc<str>, pos: Position) -> Self {
        Self {
            ttype,
            file,
            pos
        }
    }
}

/// Represents the type of a token
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType<'a> {
    Identifier(&'a str),
    Keyword(Keyword),
    Mode(Mode),
    String(&'a str),
    Regex(&'a str),
    Integer(&'a str),
    Float(&'a str),
    // Assignment
    Assign,               // =
    AssignExp,            // :=
    // Punctuation
    Dot,                  // .
    Comma,                // ,
    LeftParen,            // (
    RightParen,           // )
    LeftBracket,          // [
    RightBracket,         // ]
    LeftSquirly,          // {
    RightSquirly,         // }
    Colon,                // :
    Semicolon,            // ;
    Arrow,                // ->
    WideArrow,            // =>
    // Operators
    Address,              // @
    Cash,                 // $
    Pound,                // #
    Bang,                 // !
    Question,             // ?
    RangeExc,             // ..
    RangeInc,             // ...
    ForwardApp,           // |>
    ReverseApp,           // <|
    // Arithmetic
    Plus,                 // +
    Minus,                // -
    Asterisk,             // *
    Slash,                // /
    Percent,              // %
    Increment,            // ++
    Decrement,            // --
    Power,                // **
    // Bitwise
    Ampersand,            // &
    Pipe,                 // |
    Caret,                // ^
    Tilde,                // ~
    LeftShift,            // <<
    RightShift,           // >>
    // Comparators
    Lesser,               // <
    LesserEq,             // <=
    Greater,              // >
    GreaterEq,            // >=
    Equal,                // ==
    NotEqual,             // !=
    PatternMatch,         // ~=
    PatternNotMatch,      // !~
    // Others
    Newline,
    Illegal,
    Eof
}

impl<'a> TokenType<'a> {

}

/// Enumeration of the keywords supported
#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Static,
    Const,
    Let,
    Public,
    Return,
    Fn,
    Do,
    Begin,
    End,
    Record,
    Union,
    Use,
    Trait,
    Module,
    Defer,
    When,
    True,
    False,
    For,
    While,
    Break,
    Continue,
    Match,
    If,
    Else,
    And,
    Or,
    Not,
    Comptime,
    // Reserved
    Any,
    Private,
    Inline,
    From,
    As,
    In
}

impl Keyword {

}

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    Comptime,
    Mut,
    Mov,
    Loc
}

impl Mode {

}
