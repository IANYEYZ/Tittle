#[derive(Debug)]
pub enum TokenValue {
    Num(f64),
    Id(String),
    Op(String)
}
#[derive(Debug)]
pub enum TokenType {
    Num,
    Id,
    Op
}
#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub value: TokenValue,
    pub line_num: i16
}