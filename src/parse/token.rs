use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Slash,
    Asterisk,
    Period,
    Caret,
    LParen,
    RParen,
    Integer,
    Whitespace,
    Eof,
}

pub struct Token {
    pub pos: Position,
    pub kind: TokenKind,
    pub content: String,
    pub leading_trivia: Vec<Trivia>,
    pub trailing_trivia: Vec<Trivia>,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{}) {{ {:?} '{}' }}",
            self.pos.ind,
            self.pos.ind + self.pos.len,
            self.kind,
            self.content,
        )
    }
}

pub struct Position {
    pub ind: u32,
    pub len: u32,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.ind, self.ind + self.len)
    }
}

pub enum TriviaKind {
    Whitespace(Token),
}

pub struct Trivia {
    pub pos: Position,
    pub kind: TriviaKind
}


