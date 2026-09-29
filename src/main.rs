use crate::token::{Token, TokenType, TokenValue};
use crate::lexer::lexer;
mod token;
mod lexer;

fn main() {
    let tok = Token {
        kind: TokenType::Num,
        value: TokenValue::Num(1.0),
        line_num: 1
    };
    println!("{:?}", tok);
    println!("{:?}", lexer(String::from("1 + 1")));
}