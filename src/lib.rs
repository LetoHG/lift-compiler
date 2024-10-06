use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;

struct Operator {
    precedence: i8,
    arguments: i8,
}

impl Operator {
    fn new(precedence: i8, arguments: i8) -> Self {
        Self {
            precedence,
            arguments,
        }
    }
}

#[derive(Debug)]
pub enum UpnToken {
    Op(char),
    Num(f64),
}

struct Upn {
    upn: VecDeque<UpnToken>,
}

impl Upn {
    fn create(tokens: Vec<UpnToken>) -> Self {
        let operator_precedence: HashMap<char, Operator> = HashMap::from([
            ('/', Operator::new(2, 2)),
            ('*', Operator::new(2, 2)),
            ('+', Operator::new(1, 2)),
            ('-', Operator::new(1, 2)),
        ]);

        let mut holding_stack: Vec<UpnToken> = Vec::new();
        let mut upn: VecDeque<UpnToken> = VecDeque::new();

        for token in tokens.iter() {
            match token {
                UpnToken::Num(i) => {
                    upn.push_back(UpnToken::Num(i.to_owned()));
                }
                UpnToken::Op(new_op) => {
                    while !holding_stack.is_empty() {
                        match holding_stack.last().unwrap() {
                            UpnToken::Op(stack_op) => {
                                let new_op_prec = operator_precedence.get(new_op).unwrap();
                                let stack_op_prec = operator_precedence.get(stack_op).unwrap();

                                if stack_op_prec.precedence >= new_op_prec.precedence {
                                    upn.push_back(holding_stack.pop().unwrap());
                                } else {
                                    break;
                                }
                            }
                            _ => {
                                // unreachable do nothing
                            }
                        }
                    }

                    holding_stack.push(UpnToken::Op(*new_op));
                }
            }
        }
        while !holding_stack.is_empty() {
            upn.push_back(holding_stack.pop().unwrap());
        }

        print!("UPN: ");
        for s in upn.iter() {
            match s {
                UpnToken::Op(c) => print!("{} ", c),
                UpnToken::Num(i) => print!("{} ", i),
            }
        }
        println!("");

        Self { upn }
    }

    fn solve(&self) -> Result<f64, String> {
        let mut solve: VecDeque<f64> = VecDeque::new();

        for token in self.upn.iter() {
            match token {
                UpnToken::Op(op) => match op {
                    '/' => {
                        let args = Self::get_op_args(&mut solve, 2);
                        solve.push_front(args[1] / args[0]);
                    }
                    '*' => {
                        let args = Self::get_op_args(&mut solve, 2);
                        solve.push_front(args[1] * args[0]);
                    }
                    '+' => {
                        let args = Self::get_op_args(&mut solve, 2);
                        solve.push_front(args[1] + args[0]);
                    }
                    '-' => {
                        let args = Self::get_op_args(&mut solve, 2);
                        solve.push_front(args[1] - args[0]);
                    }
                    _ => todo!(),
                },
                UpnToken::Num(i) => {
                    solve.push_front(i.to_owned());
                }
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
    // let upn = create_upn(tokens);
    // let res = solve_upn(upn.upn);
    let upn = Upn::create(tokens);
    upn.solve()
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
