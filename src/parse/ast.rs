use crate::parse::token::Token; 

pub struct ExprNode {
    expr: Expr,
    trivia: Vec<Trivia>,
}

pub enum Trivia {
    Whitespace(Token),
    Comment(Token),
}

pub enum Expr {
    BinaryOpExpr(BinaryOpExprNode),
    UnaryOpExpr(UnaryOpExprNode),
    IntegerExpr(IntegerExprNode),
    ParenExpr(ParenExprNode),
}

impl Expr {
    pub fn pretty_print(&self) {
        fn pretty_print_internal(exp: &Expr, indent: String, last: bool) {
            let mut indent = indent;
            print!("{}", indent);
            if last {
                print!("└── ");
                indent = format!("{}    ", indent);
            } else {
                print!("├── ");
                indent = format!("{}|   ", indent);
            }
    
            match exp {
                Expr::IntegerExpr(exp) => { 
                    println!("Integer {}", exp.value.content);
                },
                Expr::BinaryOpExpr(exp) => {
                    println!("BinaryOp {}", exp.op.content);
                    pretty_print_internal(&exp.left, indent.clone(), false);
                    pretty_print_internal(&exp.right, indent, true);
                },
                Expr::UnaryOpExpr(exp) => {
                    println!("UnaryOp {}", exp.op.content);
                    pretty_print_internal(&exp.operand, indent, true);
                },
                Expr::ParenExpr(exp) => {
                    println!("ParenExpr");
                    pretty_print_internal(&exp.expr, indent, true);
                }
            }
        }
        pretty_print_internal(self, "".to_string(), true);
    }
}

pub struct BinaryOpExprNode {
    pub left: Box<Expr>,
    pub op: Token,
    pub right: Box<Expr>,
}

pub struct UnaryOpExprNode {
    pub operand: Box<Expr>,
    pub op: Token,
}

pub struct IntegerExprNode {
    pub value: Token,
}


pub struct ParenExprNode {
    pub lparen: Token,
    pub expr: Box<Expr>,
    pub rparen: Token,
}