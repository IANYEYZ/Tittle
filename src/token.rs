#[derive(Debug, PartialEq)]
pub enum TokenValue {
    Num(f64),
    Id(String),
    Op(String),
    Str(String),
    Key(String)
}
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Num,
    Id,
    Op,
    Str,
    Key
}
#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub value: TokenValue,
    pub line_num: usize
}