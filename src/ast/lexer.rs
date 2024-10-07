use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Integer(i64),
    Floating(f64),
    Plus,
    Minus,
    Astrisk,
    Slash,
    LeftParen,
    RightParen,
    Whitespace,
    Bad,
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Integer(_) => write!(f, "Integer"),
            TokenKind::Floating(_) => write!(f, "Floating"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Astrisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Bad => write!(f, "Bad"),
            TokenKind::Eof => write!(f, "Eof"),
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
        let c = self.current_char();

        if Self::is_number_start(&c) {
            kind = self.consume_number();
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

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn is_decimal_dot(c: &char) -> bool {
        *c == '.'
    }

    fn current_char(&mut self) -> char {
        self.input.chars().nth(self.cursor).unwrap()
    }

    fn consume(&mut self) -> Option<char> {
        if self.cursor >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.cursor += 1;
        Some(c)
    }

    fn consume_number(&mut self) -> TokenKind {
        let mut integer_part: i64 = 0;
        let mut fractional_part: i64 = 0;
        let mut divisior_for_fraction: i64 = 1;

        let mut dot_found = false;
        while let Some(c) = self.consume() {
            if c.is_digit(10) {
                if !dot_found {
                    integer_part = integer_part * 10 + c.to_digit(10).unwrap() as i64;
                } else {
                    fractional_part = fractional_part * 10 + c.to_digit(10).unwrap() as i64;
                    divisior_for_fraction *= 10;
                }
            } else if Self::is_decimal_dot(&c) {
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

    fn consume_punctuation(&mut self) -> TokenKind {
        match self.consume().unwrap() {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Astrisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            _ => TokenKind::Bad,
        }
    }
}
