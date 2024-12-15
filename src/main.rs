#![allow(unused)]
mod parse;
mod eval;
mod repl;
mod test;

fn main() {
    repl::repl();
}