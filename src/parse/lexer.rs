use std::{iter::Peekable, str::Chars};
use super::{Token, TokenKind, Position};

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    chars: Peekable<Chars<'a>>,
    pos: u32, 
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            tokens: vec!(),
            chars: input.chars().peekable(),
            pos: 0,
        }
    }

    pub fn lex(&'a mut self) -> Result<Vec<Token>, ()> {
        let mut peeked: char;
        while { peeked = self.peek(); peeked } != '\0' {
            let pos = self.pos;
            
            match peeked {
                '+' => { 
                    self.emit_contentful(TokenKind::Plus, pos, 1, "+".to_string());
                    self.consume();
                },
                '-' => {
                    self.emit_contentful(TokenKind::Minus, pos, 1, "-".to_string());
                    self.consume();
                },
                '/' => { 
                    self.emit_contentful(TokenKind::Slash, pos, 1, "/".to_string());
                    self.consume();
                },
                '*' => { 
                    self.emit_contentful(TokenKind::Asterisk, pos, 1, "*".to_string()); 
                    self.consume();
                },
                '^' => { 
                    self.emit_contentful(TokenKind::Caret, pos, 1, "^".to_string()); 
                    self.consume();
                },
                '(' => { 
                    self.emit_contentful(TokenKind::LParen, pos, 1, "(".to_string()); 
                    self.consume();
                },
                ')' => { 
                    self.emit_contentful(TokenKind::RParen, pos, 1, ")".to_string()); 
                    self.consume();
                },
                '!' => {
                    self.emit_contentful(TokenKind::Bang, pos, 1, "!".to_string());
                    self.consume();
                }
                _ if Self::match_whitespace(peeked, 0) => {
                    self.lex_whitespace();
                }
                _ if Self::match_integer(peeked, 0) => { 
                    self.lex_integer();
                },
                _ => { println!("while lexing: unexpected character '{}'", peeked); return Err (()) }
            }
        }

        self.emit(TokenKind::Eof, self.pos, 0);
        let tokens = std::mem::replace(&mut self.tokens, vec!() as Vec<Token>);

        Ok (tokens)
    }

    fn lex_integer<'b>(&'b mut self) {
        let mut buff: Vec<char> = vec!();
        let start_pos = self.pos;
        let mut peeked: char; 

        while { peeked = self.peek(); peeked != '\0' }  {
            match peeked {
                _ if Self::match_integer(peeked, self.pos - start_pos) => { buff.push(self.consume()); },
                _ => break
            }
        }
    
        let content = String::from_iter(buff.iter());
        self.emit_contentful(TokenKind::Integer, start_pos, self.pos - start_pos, content);
    }

    fn lex_whitespace<'b>(&'b mut self) {
        let start_pos = self.pos;
        let mut peeked: char;

        while { peeked = self.peek(); peeked != '\0' } {
            match peeked {
                _ if Self::match_whitespace(peeked, self.pos - start_pos) => { self.consume(); }
                _ => break
            }
        }

        // self.emit(TokenKind::Whitespace, start_pos, self.pos - start_pos);
    }

    #[allow(unused)]
    /// Determines whether a character at a given position matches the 'whitespace' pattern
    fn match_whitespace(c: char, pos: u32) -> bool { 
        match c {
            '\n' | '\r' | '\t' | ' ' => true,
            _ => false
        }
    }

    #[allow(unused)]
    /// Determines whether a character at a given position matches the 'whitespace' pattern
    fn match_integer(c: char, pos: u32) -> bool {
        match c {
            '_' | _ if c.is_ascii_digit() => true,
            _ => false,
        }
    }

    fn emit(&mut self, type_: TokenKind, pos: u32, length: u32) {
        self.emit_contentful(type_, pos, length, String::new());
    }

    fn emit_contentful(&mut self, type_: TokenKind, pos: u32, length: u32, content: String) {
        let pos = Position { ind: pos, len: length };
        self.tokens.push(Token { 
            pos: pos, 
            kind: type_,
            content: content,
        });
    }

    fn peek(&mut self) -> char {
        match self.chars.peek() {
            Some(c) => *c,
            None => '\0'
        }
    }

    fn consume(&mut self) -> char {
        match self.chars.next() {
            Some(c) => { self.pos += 1; c }
            None => '\0'
        }
    }
}