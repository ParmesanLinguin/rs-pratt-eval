use crate::parse::{ Lexer, Parser };
use crate::eval::interp;

#[cfg(test)]
#[test]
fn test_arithmetic_results() {
    let exprs: Vec<(&str, i64)> = vec![
        // basics
        ("-2 + 2", 0),
        ("14 * 12", 168),
        ("168 / 12", 14),
        ("168 / -12", -14),
        
        // unary
        ("-2", -2),
        ("+2", 2),

        // exponentiation
        ("-2 ^ 2", -4),
        ("2 ^ 3 ^ 2", 512),
        ("(2 ^ 3) ^ 2", 64),

        // factorial
        ("2!!!", 2),
        ("(1 + 3)!", 24),
        ("1 + 3!", 7),

        // modulo
        ("2 % 4", 2),
        ("27 % 4", 3),

        // misc
        ("2 + 3 * 4", 14),
        ("-(8 + 8) * 2 ^ 2!", -64),

    ];

    for expr in exprs {
        let mut lexer = Lexer::new(expr.0);
        let mut parser = Parser::new(lexer.lex().unwrap());
        let root = parser.parse().unwrap();
        assert_eq!(interp(&root).unwrap(), expr.1);
    }
}