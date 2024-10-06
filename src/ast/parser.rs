use crate::ast::lexer::{Lexer, Token, TokenKind};
use crate::ast::{ASTExpression, ASTStatement};

use super::{ASTBinaryExpression, ASTBinaryOperator, ASTBinaryOperatorKind};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens
                .iter()
                .filter(|token| token.kind != TokenKind::Whitespace)
                .map(|token| token.clone())
                .collect(),
            cursor: 0,
        }
    }

    pub fn from_input(input: String) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            if token.kind != TokenKind::Whitespace {
                tokens.push(token);
            }
        }
        Self { tokens, cursor: 0 }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        let token = self.current_token()?;
        let expr = self.parse_expression()?;
        Some(ASTStatement::expression(expr))
    }

    fn current_token(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get((self.cursor as isize + offset) as usize)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.cursor += 1;
        let token = self.peek(-1)?;
        Some(token)
    }

    fn parse_expression(&mut self) -> Option<ASTExpression> {
        self.parse_binary_expression(0)
    }

    fn parse_primary_expression(&mut self) -> Option<ASTExpression> {
        let token = self.consume()?;
        match token.kind {
            TokenKind::Number(i) => Some(ASTExpression::integer(i)),
            TokenKind::Floating(i) => Some(ASTExpression::float(i)),
            _ => None,
        }
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<ASTExpression> {
        let mut left = self.parse_primary_expression()?;

        while let Some(operator) = self.parse_binary_operator() {
            self.consume();

            let operator_precedence = operator.precedence();
            if operator_precedence >= precedence {
                let right = self.parse_binary_expression(operator_precedence)?;
                left = ASTExpression::binary(operator, left, right);
            } else {
                break;
            }
        }
        Some(left)
    }

    fn parse_binary_operator(&mut self) -> Option<ASTBinaryOperator> {
        let token = self.current_token()?;
        let kind = match token.kind {
            TokenKind::Plus => Some(ASTBinaryOperatorKind::Plus),
            TokenKind::Minus => Some(ASTBinaryOperatorKind::Minus),
            TokenKind::Astrisk => Some(ASTBinaryOperatorKind::Multiply),
            TokenKind::Slash => Some(ASTBinaryOperatorKind::Divide),
            _ => None,
        };
        kind.map(|kind| {
            return ASTBinaryOperator {
                kind,
                token: token.clone(),
            };
        })
    }
}
