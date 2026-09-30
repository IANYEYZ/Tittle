use crate::token::{Token, TokenType, TokenValue};

#[derive(Debug, PartialEq)]
pub struct LexError{
    message: String,
    line_num: usize
}

const OPERATORS: &[&str] = &[
    "+", "-", "*", "/", ">=", "<=", ">", "<", "==", "!=", "=", "|>", "=>",
    "||", "&&", "!", ".", "(", ")", "[", "]", "{", "}", ":", ";"
];
const KEYWORDS: &[&str] = &[
    "match", "fn", "for"
];

pub fn lexer(code: String) -> Result<Vec<Token>, LexError> {
    let mut chars = code.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut line_num: usize = 0;
    while let Some(&c) = chars.peek() {
        match c {
            '\n' => { line_num += 1; chars.next(); },
            ch if ch.is_whitespace() => { chars.next(); },
            '0'..='9' => {
                let mut number: String = "".to_string();
                let mut has_dot = false;
                while let Some(c) = chars.peek() {
                    if !c.is_ascii_digit() && (has_dot || *c != '.') {
                        break;
                    }
                    if *c == '.' {
                        has_dot = true;
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
            ch if OPERATORS.iter().any(|op| op.chars().next() == Some(ch)) => {
                let mut flag = false;
                chars.next();
                if let Some(nxt) = chars.peek() {
                    let tok = ch.to_string() + &nxt.to_string();
                    if OPERATORS.contains(&tok.as_str()) {
                        tokens.push(Token {
                            kind: TokenType::Op,
                            value: TokenValue::Op(tok),
                            line_num: line_num
                        });
                        chars.next();
                        flag = true;
                    }
                }
                if !flag {
                    if OPERATORS.contains(&ch.to_string().as_str()) {
                        tokens.push(Token {
                            kind: TokenType::Op,
                            value: TokenValue::Op(ch.to_string()),
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
            'a'..='z' | '_' => {
                let mut id: String = "".to_string();
                while let Some(c) = chars.peek() {
                    if !c.is_ascii_alphanumeric() && *c != '_' {
                        break;
                    }
                    id.push(*c);
                    chars.next();
                }
                if KEYWORDS.contains(&id.as_str()) {
                    tokens.push(Token {
                        kind: TokenType::Key,
                        value: TokenValue::Key(id),
                        line_num: line_num
                    });
                } else {
                    tokens.push(Token {
                        kind: TokenType::Id,
                        value: TokenValue::Id(id),
                        line_num: line_num
                    });
                }
            },
            '"' | '\'' => {
                let mut str: String = "".to_string();
                let guard = chars.next().unwrap();
                while let Some(c) = chars.peek() {
                    if *c == guard {
                        break;
                    }
                    str.push(*c);
                    chars.next();
                }
                chars.next();
                tokens.push(Token {
                    kind: TokenType::Str,
                    value: TokenValue::Str(str),
                    line_num: line_num
                });
            },
            _ => return Err(LexError { 
                message: String::from("Unknown character {c}"),
                line_num: line_num
            })
        }
    };
    return Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lex_number() {
        assert_eq!(lexer("1".to_string()), Ok(vec![Token {
            kind: TokenType::Num,
            value: TokenValue::Num(1.0),
            line_num: 0
        }]));
        assert_eq!(lexer("10".to_string()), Ok(vec![Token {
            kind: TokenType::Num,
            value: TokenValue::Num(10.0),
            line_num: 0
        }]));
        assert_eq!(lexer("10.01".to_string()), Ok(vec![Token {
            kind: TokenType::Num,
            value: TokenValue::Num(10.01),
            line_num: 0
        }]));
    }
    #[test]
    fn lex_operator() {
        assert_eq!(lexer("+-*/>=<===!=".to_string()), Ok(vec![Token {
            kind: TokenType::Op,
            value: TokenValue::Op("+".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Op,
            value: TokenValue::Op("-".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Op,
            value: TokenValue::Op("*".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Op,
            value: TokenValue::Op("/".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Op,
            value: TokenValue::Op(">=".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Op,
            value: TokenValue::Op("<=".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Op,
            value: TokenValue::Op("==".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Op,
            value: TokenValue::Op("!=".to_string()),
            line_num: 0
        }]));
    }
    #[test]
    fn lex_id() {
        assert_eq!(lexer("a".to_string()), Ok(vec![Token {
            kind: TokenType::Id,
            value: TokenValue::Id("a".to_string()),
            line_num: 0
        }]));
        assert_eq!(lexer("_".to_string()), Ok(vec![Token {
            kind: TokenType::Id,
            value: TokenValue::Id("_".to_string()),
            line_num: 0
        }]));
        assert_eq!(lexer("_1a".to_string()), Ok(vec![Token {
            kind: TokenType::Id,
            value: TokenValue::Id("_1a".to_string()),
            line_num: 0
        }]));
    }
    #[test]
    fn lex_key() {
        assert_eq!(lexer("fn".to_string()), Ok(vec![Token {
            kind: TokenType::Key,
            value: TokenValue::Key("fn".to_string()),
            line_num: 0
        }]));
        assert_eq!(lexer("for".to_string()), Ok(vec![Token {
            kind: TokenType::Key,
            value: TokenValue::Key("for".to_string()),
            line_num: 0
        }]));
        assert_eq!(lexer("match".to_string()), Ok(vec![Token {
            kind: TokenType::Key,
            value: TokenValue::Key("match".to_string()),
            line_num: 0
        }]));
    }
    #[test]
    fn lex_str() {
        assert_eq!(lexer("'a'".to_string()), Ok(vec![Token {
            kind: TokenType::Str,
            value: TokenValue::Str("a".to_string()),
            line_num: 0
        }]));
        assert_eq!(lexer("\"a\"".to_string()), Ok(vec![Token {
            kind: TokenType::Str,
            value: TokenValue::Str("a".to_string()),
            line_num: 0
        }]));
    }
    #[test]
    fn lex_mixed() {
        assert_eq!(lexer("a = 1 + 1 >= 2 + \"a\"".to_string()), Ok(vec![Token {
            kind: TokenType::Id,
            value: TokenValue::Id("a".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Op,
            value: TokenValue::Op("=".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Num,
            value: TokenValue::Num(1.0),
            line_num: 0
        }, Token {
            kind: TokenType::Op,
            value: TokenValue::Op("+".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Num,
            value: TokenValue::Num(1.0),
            line_num: 0
        }, Token {
            kind: TokenType::Op,
            value: TokenValue::Op(">=".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Num,
            value: TokenValue::Num(2.0),
            line_num: 0
        }, Token {
            kind: TokenType::Op,
            value: TokenValue::Op("+".to_string()),
            line_num: 0
        }, Token {
            kind: TokenType::Str,
            value: TokenValue::Str("a".to_string()),
            line_num: 0
        }]));
    }
}