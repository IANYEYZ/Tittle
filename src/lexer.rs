use crate::token::{Token, TokenType, TokenValue};

#[derive(Debug)]
pub struct LexError{
    message: String,
    line_num: i16
}

const operators: &[&str] = &[
    "+", "-", "*", "/", ">=", "<=", ">", "<", "==", "!=", "=", "|>", "=>",
    "||", "&&", "!"
];

pub fn lexer(code: String) -> Result<Vec<Token>, LexError> {
    let mut chars = code.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut line_num: i16 = 0;
    while let Some(&c) = chars.peek() {
        match c {
            '\n' => { line_num += 1; chars.next(); },
            ch if ch.is_whitespace() => { chars.next(); },
            '0'..'9' => {
                let mut number: String = "".to_string();
                while let Some(c) = chars.peek() {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    number.push(*c);
                    chars.next();
                }
                tokens.push(Token {
                    kind: TokenType::Num,
                    value: TokenValue::Num(number.parse::<f64>().unwrap()),
                    line_num: line_num
                });
            },
            ch if operators.iter().any(|op| op.chars().next() == Some(ch)) => {
                chars.next();
                if let Some(nxt) = chars.peek() {
                    let tok = ch.to_string() + &nxt.to_string();
                    if operators.contains(&tok.as_str()) {
                        tokens.push(Token {
                            kind: TokenType::Id,
                            value: TokenValue::Id(tok),
                            line_num: line_num
                        });
                    }
                } else {
                    if operators.contains(&ch.to_string().as_str()) {
                        tokens.push(Token {
                            kind: TokenType::Id,
                            value: TokenValue::Id(ch.to_string()),
                            line_num: line_num
                        });
                    } else {
                        return Err(LexError { 
                            message: "Unknown operator {ch}".to_string(),
                            line_num: line_num
                        })
                    }
                }
            },
            _ => return Err(LexError { 
                message: String::from("Unknown character {c}"),
                line_num: line_num
            })
        }
    };
    return Ok(tokens)
}