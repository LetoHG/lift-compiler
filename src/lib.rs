pub mod ast; // Declares the `ast` module

use crate::ast::lexer::{Token, TokenKind};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum UpnToken {
    Op(char),
    Num(f64),
}

pub struct Upn {
    upn: VecDeque<Token>,
}

impl Upn {
    pub fn new() {}

    fn get_operator_precedence(op: &TokenKind) -> u8 {
        match op {
            TokenKind::Plus => 1,
            TokenKind::Minus => 1,
            TokenKind::Astrisk => 2,
            TokenKind::Slash => 2,
            _ => 0,
        }
    }

    fn unwind_holding_stack(
        upn: &mut VecDeque<Token>,
        holding_stack: &mut Vec<Token>,
        op: &TokenKind,
    ) {
        while !holding_stack.is_empty() {
            let stack_op = holding_stack.last().unwrap();
            let new_op_prec = Self::get_operator_precedence(op);
            let stack_op_prec = Self::get_operator_precedence(&stack_op.kind);

            if stack_op_prec >= new_op_prec {
                upn.push_back(holding_stack.pop().unwrap());
            } else {
                break;
            }
        }
    }
    pub fn create(tokens: Vec<Token>) -> Self {
        let mut holding_stack: Vec<Token> = Vec::new();
        let mut upn: VecDeque<Token> = VecDeque::new();

        for token in tokens.iter() {
            println!("TOKEN: {:?}", token);
            match token.kind {
                TokenKind::Number(_) => upn.push_back(token.clone()),
                TokenKind::Floating(_) => upn.push_back(token.clone()),
                TokenKind::Plus => {
                    Self::unwind_holding_stack(&mut upn, &mut holding_stack, &token.kind);
                    holding_stack.push(token.clone());
                }
                TokenKind::Minus => {
                    Self::unwind_holding_stack(&mut upn, &mut holding_stack, &token.kind);
                    holding_stack.push(token.clone());
                }
                TokenKind::Astrisk => {
                    Self::unwind_holding_stack(&mut upn, &mut holding_stack, &token.kind);
                    holding_stack.push(token.clone());
                }
                TokenKind::Slash => {
                    Self::unwind_holding_stack(&mut upn, &mut holding_stack, &token.kind);
                    holding_stack.push(token.clone());
                }
                TokenKind::LeftParen => todo!(),
                TokenKind::RightParen => todo!(),
                TokenKind::Whitespace => todo!(),
                TokenKind::Bad => todo!(),
                TokenKind::Eof => (),
            }
        }
        while !holding_stack.is_empty() {
            upn.push_back(holding_stack.pop().unwrap());
        }

        print!("UPN: ");
        for token in upn.iter() {
            print!("{:?} ", token);
        }
        println!("");

        Self { upn }
    }

    pub fn solve(&self) -> Result<f64, String> {
        let mut solve: VecDeque<f64> = VecDeque::new();

        for token in self.upn.iter() {
            dbg!(token);
            match token.kind {
                TokenKind::Plus => {
                    let args = Self::get_op_args(&mut solve, 2);
                    solve.push_front(args[1] + args[0]);
                }
                TokenKind::Minus => {
                    let args = Self::get_op_args(&mut solve, 2);
                    solve.push_front(args[1] - args[0]);
                }
                TokenKind::Astrisk => {
                    let args = Self::get_op_args(&mut solve, 2);
                    solve.push_front(args[1] * args[0]);
                }
                TokenKind::Slash => {
                    let args = Self::get_op_args(&mut solve, 2);
                    solve.push_front(args[1] / args[0]);
                }
                TokenKind::Number(i) => solve.push_front(i as f64),
                TokenKind::Floating(f) => solve.push_front(f),
                _ => (),
            }
        }

        match solve.front() {
            Some(i) => Ok(i.to_owned()),
            None => Err("Error".to_string()),
        }
    }

    fn get_op_args(args: &mut VecDeque<f64>, num_args: i8) -> Vec<f64> {
        let mut op_args = Vec::new();

        for _ in 0..num_args {
            match args.pop_front() {
                Some(value) => op_args.push(value),
                None => todo!(),
            }
        }

        op_args
    }
}

pub fn evaluate(equation: &str) -> Result<f64, String> {
    let tokens = parse_tokens(equation);
    // let upn = Upn::create(tokens);
    // upn.solve()
    return Ok(0.0);
}

fn parse_tokens(equation: &str) -> Vec<UpnToken> {
    let mut tokens = Vec::new();
    println!("{}", equation);

    let mut num_str: String = String::new();
    equation.chars().for_each(|c: char| match c {
        '/' => tokens.push(UpnToken::Op(c)),
        '*' => tokens.push(UpnToken::Op(c)),
        '-' => tokens.push(UpnToken::Op(c)),
        '+' => tokens.push(UpnToken::Op(c)),
        '0'..='9' => num_str.push(c),
        '.' => {
            if num_str.contains('.') {
                todo!();
                panic!();
            }
            num_str.push(c)
        }
        _ => {
            if !num_str.is_empty() {
                let num = num_str.parse::<f64>().unwrap();
                tokens.push(UpnToken::Num(num));
                num_str.clear();
            }
        }
    });

    // incase line ends with a number
    if !num_str.is_empty() {
        let num = num_str.parse::<f64>().unwrap();
        tokens.push(UpnToken::Num(num));
        num_str.clear();
    }

    println!("{:?}", tokens);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_basic_equation() {
        {
            let result = evaluate("4 + 2").unwrap_or(0.0);
            assert_eq!(result, 6.0);
        }
        {
            let result = evaluate("67 * 21").unwrap_or(0.0);
            assert_eq!(result, 67.0 * 21.0);
        }
        {
            let result = evaluate("67 * 21").unwrap_or(0.0);
            assert_eq!(result, 67.0 * 21.0);
        }
        {
            let result = evaluate("2.718281828 / 1.654 + 675 / 54 * 3").unwrap_or(0.0);
            assert_eq!(result, 2.718281828 / 1.654 + 675.0 / 54.0 * 3.0);
        }
    }
}
