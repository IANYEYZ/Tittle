use crate::token::Token;
use crate::token::TokenValue;
pub mod token;

fn main() {
    let tok = Token {
        kind: String::from("Number"),
        value: TokenValue::NUM(1.0)
    };
    println!("{:?}", tok)
}