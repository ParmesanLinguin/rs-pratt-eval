# Pratt Parser and Expression Evaluator
Simple Pratt parser and expression evaluator implemented in Rust.

Supports unary prefix, unary postfix and binary infix operators with varying
precedences and associativities.

Created as a reason for me to learn some Rust basics, as well as explore
Pratt parsing.

## Usage
enter an infix expression to evaluate it.

```
$ ./eval
> -(8 + 8) * 2 ^ 4.
-64
```

use the `#tree` command to print out the parse tree.
```
> #tree
tree printing enabled.
> -(8 + 8) * 2 ^ 4.
parse tree:
└── BinaryOp *
    ├── ParenExpr
    |   └── BinaryOp +
    |       ├── Integer 8
    |       └── Integer 8
    └── BinaryOp ^
        ├── Integer 2
        └── UnaryOp .
            └── Integer 4

-64
```

use the `#tokens` command to print out the token list.
```
> #tokens
token printing enabled.
> -(8 + 8) * 2 ^ 4.
token stream:
0000 (0:1) { Minus '-' }
0001 (1:2) { LParen '(' }
0002 (2:3) { Integer '8' }
0003 (4:5) { Plus '+' }
0004 (6:7) { Integer '8' }
0005 (7:8) { RParen ')' }
0006 (9:10) { Asterisk '*' }
0007 (11:12) { Integer '2' }
0008 (13:14) { Caret '^' }
0009 (15:16) { Integer '4' }
0010 (16:17) { Period '.' }
0011 (19:19) { Eof '' }

-64
```
## Operators
Higher precedences bind first.

| Operator | Precedence | Description                | Arity  | Associativity |
|----------|------------|----------------------------|--------|---------------|
| +<br>-   | 0          | Addition<br>Subtraction    | Binary | Left          |
| *<br>/   | 1          | Multiplication<br>Division | Binary | Left          |
| +<br>-   | 2          | Positive<br>Negative       | Unary  | Right         |
| ^        | 3          | Exponentiation             | Binary | Right         |
| .        | 4          | Div2                       | Unary  | Left          |