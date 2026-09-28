#[derive(Debug)]
pub enum TokenValue {
    NUM(f64),
    ID(String),
    OP(String)
}
#[derive(Debug)]
pub struct Token {
    pub kind: String,
    pub value: TokenValue
}