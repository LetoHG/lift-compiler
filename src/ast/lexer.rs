use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Integer(i64),
    Floating(f64),
    Identifier,
    Let,
    Func,
    Return,
    Plus,
    Minus,
    Astrisk,
    Slash,
    Equal,
    BitwiseOR,
    BitwiseAND,
    BitwiseXOR,
    BitwiseNOT,
    ExclemationMark,
    EqualTo,
    NotEqualTo,
    LogicAND,
    LogicOR,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftAngleBracket,
    RightAngleBracket,
    Comma,
    SemiColon,
    Whitespace,
    Bad,
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Integer(_) => write!(f, "Integer"),
            TokenKind::Floating(_) => write!(f, "Floating"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::Func => write!(f, "Func"),
            TokenKind::Return => write!(f, "Return"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Astrisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Equal => write!(f, "="),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{}", '{'),
            TokenKind::RightBrace => write!(f, "{}", '}'),
            TokenKind::Comma => write!(f, ","),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Bad => write!(f, "Bad"),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::EqualTo => write!(f, "=="),
            TokenKind::NotEqualTo => write!(f, "!="),
            TokenKind::LogicAND => write!(f, "&&"),
            TokenKind::LogicOR => write!(f, "||"),
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}

impl TextSpan {
    fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub(crate) span: TextSpan,
}

impl Token {
    fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer {
    input: String,
    cursor: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input, cursor: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.cursor > self.input.len() {
            return None;
        }
        if self.cursor == self.input.len() {
            self.cursor += 1;
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(0, 0, '\0'.to_string()),
            ));
        }

        let start = self.cursor;
        let mut kind = TokenKind::Bad;
        let c = self.current_char()?;

        if Self::is_number_start(&c) {
            kind = self.consume_number();
        } else if Self::is_identifier_start(&c) {
            let identifier = self.consume_identifier();
            kind = match identifier.as_str() {
                "let" => TokenKind::Let,
                "func" => TokenKind::Func,
                "return" => TokenKind::Return,
                _ => TokenKind::Identifier,
            };
        } else if Self::is_operator_start(&c) {
            kind = self.consume_boolean_operator();
        } else if Self::is_whitespace(&c) {
            self.consume();
            kind = TokenKind::Whitespace;
        } else {
            kind = self.consume_punctuation();
        }

        let end = self.cursor;
        let literal = self.input[start..end].to_string();

        Some(Token::new(kind, TextSpan::new(start, end, literal)))
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic()
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn is_decimal_dot(c: &char) -> bool {
        *c == '.'
    }

    fn is_operator_start(c: &char) -> bool {
        false
        // "=="
        // "&&"
        // "||"
        // "+="
        // "-="
        // "*="
        // "/="
    }

    fn current_char(&mut self) -> Option<char> {
        self.input.chars().nth(self.cursor)
    }

    fn consume(&mut self) -> Option<char> {
        if self.cursor >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.cursor += 1;
        c
    }

    fn consume_number(&mut self) -> TokenKind {
        let mut integer_part: i64 = 0;
        let mut fractional_part: i64 = 0;
        let mut divisior_for_fraction: i64 = 1;

        let mut oct_format = false;
        let mut hex_format = false;
        if self.current_char().unwrap() == '0' {
            self.consume();
            oct_format = true;

            if self.current_char().unwrap() == 'x' {
                self.consume();
                hex_format = true;
                oct_format = false;
            }
        }

        let mut dot_found = false;

        while let Some(c) = self.current_char() {
            if oct_format && c.is_digit(8) {
                self.consume();
                integer_part = integer_part * 8 + c.to_digit(8).unwrap() as i64;
            } else if hex_format && c.is_digit(16) {
                self.consume();
                integer_part = integer_part * 16 + c.to_digit(16).unwrap() as i64;
            } else if c.is_digit(10) {
                self.consume();
                if !dot_found {
                    integer_part = integer_part * 10 + c.to_digit(10).unwrap() as i64;
                } else {
                    fractional_part = fractional_part * 10 + c.to_digit(10).unwrap() as i64;
                    divisior_for_fraction *= 10;
                }
            } else if Self::is_decimal_dot(&c) {
                self.consume();
                if dot_found {
                    break;
                }
                dot_found = true;
            } else {
                break;
            }
        }
        if dot_found {
            return TokenKind::Floating(
                integer_part as f64 + (fractional_part as f64 / divisior_for_fraction as f64),
            );
        } else {
            return TokenKind::Integer(integer_part);
        }
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.consume();
            } else {
                break;
            }
        }
        identifier
    }

    fn consume_boolean_operator(&mut self) -> TokenKind {
        // let c1 = self.consume().unwrap();
        // let c1 = self.consume().unwrap();

        TokenKind::Bad
    }

    fn consume_punctuation(&mut self) -> TokenKind {
        match self.consume().unwrap() {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Astrisk,
            '/' => TokenKind::Slash,
            '=' => TokenKind::Equal,
            '|' => TokenKind::BitwiseOR,
            '&' => TokenKind::BitwiseAND,
            '^' => TokenKind::BitwiseXOR,
            '~' => TokenKind::BitwiseNOT,
            '!' => TokenKind::ExclemationMark,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '<' => TokenKind::LeftAngleBracket,
            '>' => TokenKind::RightAngleBracket,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::SemiColon,
            _ => TokenKind::Bad,
        }
    }
}
