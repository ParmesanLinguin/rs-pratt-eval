use super::ast::*;
use super::{Token, TokenKind};
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    pos: usize,
}

const PRECEDENCE_ADDITION: u32 = 0;
const PRECEDENCE_MULTIPLY: u32 = 1;
const PRECEDENCE_UNARYNEG: u32 = 2;
const PRECEDENCE_EXPONENT: u32 = 3;
const PRECEDENCE_FACTORIAL:u32 = 4;


const fn left_associative(level: u32) -> (u32, u32) {
    ((level + 1) * 2 - 1, (level + 1) * 2)
}

const fn right_associative(level: u32) -> (u32, u32) {
    ((level + 1) * 2, (level + 1) * 2 - 1)
}

const fn unary(level: u32) -> u32 {
    (level + 1) * 2
}

impl Token {
    fn get_unary_precedence_prefix(&self) -> u32 {
        match self.kind {
            TokenKind::Plus |
            TokenKind::Minus => unary(PRECEDENCE_UNARYNEG),
            _ => 0,
        }
    }

    fn get_unary_precedence_postfix(&self) -> u32 {
        match self.kind {
            TokenKind::Bang => unary(PRECEDENCE_FACTORIAL),
            _ => 0,
        }
    }

    fn get_binary_precedence(&self) -> (u32, u32) {
        match self.kind {
            TokenKind::Plus | TokenKind::Minus
                => left_associative(PRECEDENCE_ADDITION),
            TokenKind::Asterisk | TokenKind::Slash 
                => left_associative(PRECEDENCE_MULTIPLY),
            TokenKind::Caret 
                => right_associative(PRECEDENCE_EXPONENT),
            _ => (0, 0)
        }
    }
}

impl<'a> Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
            pos: 0,
        }
    }

    pub fn parse(&'a mut self) -> Result<Expr, ()> {
        let exp = self.parse_expr()?;
        if self.peek().kind != TokenKind::Eof {
            let peeked = self.peek();
            println!("{}: parsing expr: unexpected token {:?}", peeked.pos, peeked.kind);
            Err (())
        } else {
            Ok (exp)
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ()> {
        let exp = self.parse_expr_bind(0)?;
        Ok(exp)
    }
    
    fn parse_atomic_expr(&mut self) -> Result<Expr, ()> {
        match self.peek() {
            Token {kind: TokenKind::Integer,..} => self.emit_integer(),
            Token {kind: TokenKind::LParen,..} => {
                let left = self.consume();
                let expr = self.parse_expr()?;
                let right = self.consume();
                if right.kind != TokenKind::RParen {
                    print!("{}: while parsing paren_expr: got {:?} but expected {:?}", 
                        right.pos, 
                        right.kind, 
                        TokenKind::RParen
                    );
                }
                Ok (Expr::ParenExpr(ParenExprNode {
                    lparen: left,
                    expr: Box::new(expr),
                    rparen: right,
                }))
            }
            k => { println!("{}: parsing atom: unexpected token {:?}", k.pos, k.kind); Err (()) }
        }
    }

    fn parse_expr_bind(&mut self, min_bp: u32) -> Result<Expr, ()> {
        // adapted largely from Minsk
        // https://github.com/terrajobst/minsk/blob/master/src/Minsk/CodeAnalysis/Syntax/Parser.cs#L374
        // in addition to alex kladov's blog
        // https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
        let prefix_prec = self.peek().get_unary_precedence_prefix();
        let mut left: Expr;

        // must be unary prefix 
        if prefix_prec != 0 && prefix_prec >= min_bp {
            let op = self.consume();
            let operand = self.parse_expr_bind(prefix_prec)?;
            left = Expr::UnaryOpExpr(UnaryOpExprNode {
                op: op,
                operand: Box::new(operand)
            });
        } else {
            left = self.parse_atomic_expr()?;
        }

        loop {
            let postfix_prec = self.peek().get_unary_precedence_postfix();
            if postfix_prec == 0 || postfix_prec < min_bp {
                break;
            }

            if postfix_prec != 0 && postfix_prec >= min_bp {
                let op = self.consume();
                let operand = left;
                left = Expr::UnaryOpExpr(UnaryOpExprNode {
                    op: op,
                    operand: Box::new(operand)
                });
            }
        }

        loop {
            let (left_prec, right_prec) = self.peek().get_binary_precedence();
            if left_prec == 0 || left_prec < min_bp {
                break;
            }

            let op = self.consume();
            let right: Expr = self.parse_expr_bind(right_prec)?;
            left = Expr::BinaryOpExpr(BinaryOpExprNode {
                left: Box::new(left),
                op: op,
                right: Box::new(right),
            })
        }

        Ok(left)
    } 

    fn emit_integer(&mut self) -> Result<Expr, ()> {
        let consumed = self.consume();
        if !matches!(consumed.kind, TokenKind::Integer) {
            println!("{}: parsing number: got {:?} but expected {:?}",
                consumed.pos, 
                consumed.kind, 
                TokenKind::Integer
            );
            Err (())
        } else {
            Ok(Expr::IntegerExpr(IntegerExprNode { value: consumed }))
        }
    } 

    fn fail_eoi() -> ! {
        panic!("unexpected end of input");
    }

    fn peek(&mut self) -> &Token {
        match self.tokens.peek() {
            Some(a) => a,
            None => Self::fail_eoi()
        }
    }

    fn consume(&mut self) -> Token {
        self.pos += 1;
        self.tokens.next().unwrap()
    }
}