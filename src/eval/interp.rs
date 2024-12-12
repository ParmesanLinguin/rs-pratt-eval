use crate::parse::{TokenKind, Expr};

pub fn interp(expr: &Expr) -> Result<i64, ()> {
    match expr {
        Expr::IntegerExpr(exp) => { 
            let parsed = i64::from_str_radix(exp.value.content.as_str(), 10);
            match parsed {
                Err(_) => {
                    println!("eval: failed to parse integer value {}", exp.value.content);
                    Err(())
                },
                Ok(i) => Ok(i)
            }
        },
        Expr::BinaryOpExpr(exp) => {
            let left = interp(&exp.left)?;
            let right = interp(&exp.right)?;
            Ok (match &exp.op.kind {
                TokenKind::Plus => left + right,
                TokenKind::Minus => left - right,
                TokenKind::Slash => {
                    if right != 0 { 
                        left / right 
                    } 
                    else { 
                        println!("eval: div by 0"); return Err(()) 
                    }
                },
                TokenKind::Asterisk => left * right,
                TokenKind::Caret => left.pow(right.try_into().unwrap()),
                k => panic!("binary operation not implemented for {:?}", k)
            })
        },
        Expr::UnaryOpExpr(exp) => {
            let operand = interp(&exp.operand)?;
            Ok(match &exp.op.kind {
                TokenKind::Plus => operand,
                TokenKind::Minus => -operand,
                TokenKind::Bang => (1..=operand).product(),
                k => panic!("binary operation not implemented for {:?}", k)
            })
        },
        Expr::ParenExpr(exp) => {
            interp(&exp.expr)
        }
    }
}