use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;

struct Operator {
    precedence: i8,
    arguments: i8,
}

pub enum Token {
    Op(char),
    Num(f64),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Op(c) => write!(f, "({})", c),
            Token::Num(i) => write!(f, "({})", i),
        }
    }
}

pub fn evaluate(equation: &str) {
    let tokens = parse_tokens(equation);
    let upn = create_upn(tokens);
    let res = solve_upn(upn);

    println!(
        "{} = {}",
        equation.strip_suffix("\n").unwrap(),
        res.unwrap()
    );
}

fn parse_tokens(equation: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    println!("{}", equation);

    let mut num_str: String = String::new();
    equation.chars().for_each(|c: char| match c {
        '/' => tokens.push(Token::Op(c)),
        '*' => tokens.push(Token::Op(c)),
        '-' => tokens.push(Token::Op(c)),
        '+' => tokens.push(Token::Op(c)),
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
                tokens.push(Token::Num(num));
                num_str.clear();
            }
        }
    });

    // Print the parsed tokens
    // for token in tokens.iter() {
    //     match token {
    //         Token::Op(c) => println!("token Op {}", c),
    //         Token::Num(i) => println!("token Num {}", i),
    //     }
    // }

    tokens
}

fn create_upn(tokens: Vec<Token>) -> VecDeque<Token> {
    let operator_precedence: HashMap<char, Operator> = HashMap::from([
        (
            '/',
            Operator {
                precedence: 2,
                arguments: 2,
            },
        ),
        (
            '*',
            Operator {
                precedence: 2,
                arguments: 2,
            },
        ),
        (
            '+',
            Operator {
                precedence: 1,
                arguments: 2,
            },
        ),
        (
            '-',
            Operator {
                precedence: 1,
                arguments: 2,
            },
        ),
    ]);

    let mut holding_stack: Vec<Token> = Vec::new();
    let mut upn: VecDeque<Token> = VecDeque::new();

    for token in tokens.iter() {
        match token {
            Token::Num(i) => {
                upn.push_back(Token::Num(i.to_owned()));
            }
            Token::Op(new_op) => {
                while !holding_stack.is_empty() {
                    match holding_stack.last().unwrap() {
                        Token::Op(stack_op) => {
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

                holding_stack.push(Token::Op(*new_op));
            }
        }
    }
    while !holding_stack.is_empty() {
        upn.push_back(holding_stack.pop().unwrap());
    }

    print!("UPN: ");
    for s in upn.iter() {
        match s {
            Token::Op(c) => print!("{} ", c),
            Token::Num(i) => print!("{} ", i),
        }
    }
    println!("");

    upn
}

fn solve_upn(upn: VecDeque<Token>) -> Result<f64, String> {
    let mut solve: VecDeque<f64> = VecDeque::new();

    for token in upn.iter() {
        match token {
            Token::Op(op) => match op {
                '/' => {
                    let args = get_op_args(&mut solve, 2);
                    solve.push_front(args[1] / args[0]);
                }
                '*' => {
                    let args = get_op_args(&mut solve, 2);
                    solve.push_front(args[1] * args[0]);
                }
                '+' => {
                    let args = get_op_args(&mut solve, 2);
                    solve.push_front(args[1] + args[0]);
                }
                '-' => {
                    let args = get_op_args(&mut solve, 2);
                    solve.push_front(args[1] - args[0]);
                }
                _ => todo!(),
            },
            Token::Num(i) => {
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
