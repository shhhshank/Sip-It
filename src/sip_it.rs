use std::fmt::{self, write};

// CONSTANTS
const DIGITS: &str = "0123456789";

// ERRORS
#[derive(Debug)]
struct Position {
    idx: isize,
    ln: usize,
    col: isize,
    fn_name: String,
    ftxt: String,
}

impl Position {
    fn new(idx: isize, ln: usize, col: isize, fn_name: &str, ftxt: &str) -> Position {
        Position {
            idx,
            ln,
            col,
            fn_name: fn_name.to_string(),
            ftxt: ftxt.to_string(),
        }
    }

    fn advance(&mut self, current_char: Option<char>) -> &Position {
        self.idx += 1;
        self.col += 1;

        if let Some('\n') = current_char {
            self.ln += 1;
            self.col = 0;
        }

        self
    }

    fn copy(&self) -> Position {
        Position::new(self.idx, self.ln, self.col, &*self.fn_name, &*self.ftxt)
    }
}

#[derive(Debug)]
enum TokenType {
    INT(isize),
    FLOAT(f64),
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
}

#[derive(Debug)]
pub struct Token {
    type_: TokenType,
}

impl Token {
    fn new(type_: TokenType) -> Token {
        Token { type_ }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.type_ {
            TokenType::INT(int_val) => write!(f, "INT({})", int_val),
            TokenType::FLOAT(float_val) => write!(f, "FLOAT({})", float_val),
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::MUL => write!(f, "MULTIPLY"),
            TokenType::DIV => write!(f, "DIVIDE"),
            TokenType::LPAREN => write!(f, "LEFT-PAREN"),
            TokenType::RPAREN => write!(f, "RPAREN"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}

// LEXER
struct Lexer {
    fn_name: String,
    text: String,
    pos: Position,
    current_char: Option<char>,
}

impl Lexer {
    fn new(fn_name: &str, text: &str) -> Lexer {
        let mut lexer = Lexer {
            fn_name: fn_name.to_string(),
            text: text.to_string(),
            pos: Position::new(-1, 0, -1, fn_name, text),
            current_char: None,
        };

        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.pos.advance(self.current_char);
        self.current_char = self.text.chars().nth(self.pos.idx as usize);
    }

    fn make_tokens(&mut self) -> (Vec<Token>, Option<Error>) {
        let mut tokens = Vec::new();

        while let Some(current_char) = self.current_char {
            if current_char.is_whitespace() {
                self.advance();
            } else if DIGITS.contains(current_char) {
                tokens.push(self.make_number());
            } else {
                match current_char {
                    '+' => {
                        tokens.push(Token::new(TokenType::PLUS));
                        self.advance();
                    }
                    '-' => {
                        tokens.push(Token::new(TokenType::MINUS));
                        self.advance();
                    }
                    '*' => {
                        tokens.push(Token::new(TokenType::MUL));
                        self.advance();
                    }
                    '/' => {
                        tokens.push(Token::new(TokenType::DIV));
                        self.advance();
                    }
                    '(' => {
                        tokens.push(Token::new(TokenType::LPAREN));
                        self.advance();
                    }
                    ')' => {
                        tokens.push(Token::new(TokenType::RPAREN));
                        self.advance();
                    }
                    _ => {
                        let pos_start = self.pos.copy();

                        let char_str = current_char.to_string();
                        self.advance();
                        return (
                            vec![],
                            Some(Error::new(
                                "Illegal Char Error",
                                pos_start,
                                self.pos.copy(),
                                &char_str,
                            )),
                        );
                    }
                }
            }
        }

        (tokens, None)
    }

    fn make_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut dot_count = 0;

        while let Some(current_char) = self.current_char {
            if current_char.is_ascii_digit() {
                num_str.push(current_char);
            } else if current_char == '.' {
                if dot_count == 1 {
                    break;
                }
                dot_count += 1;
                num_str.push('.');
            } else {
                break;
            }

            self.advance();
        }

        if dot_count == 0 {
            Token::new(TokenType::INT(
                num_str.parse().expect("A valid integer was expected"),
            ))
        } else {
            Token::new(TokenType::FLOAT(
                num_str.parse().expect("A valid float was expected"),
            ))
        }
    }
}

pub struct Error {
    type_: String,
    pos_start: Position,
    pos_end: Position,
    details: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}\nFile {}, line {}",
            self.type_,
            self.details,
            self.pos_start.fn_name,
            self.pos_start.ln + 1
        )
    }
}

impl Error {
    fn new(type_: &str, pos_start: Position, pos_end: Position, details: &str) -> Error {
        Error {
            type_: type_.to_string(),
            pos_start,
            pos_end,
            details: details.to_string(),
        }
    }
}

// RUN
pub fn run(fn_name: &str, text: &str) -> (Vec<Token>, Option<Error>) {
    let mut lexer = Lexer::new(fn_name, text);
    lexer.make_tokens()
}
