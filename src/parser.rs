use std::process::exit;

use crate::utilties::{is_operator, is_digit};
pub enum Op {
    PLUS,
    MINUS,
    DIVIDE,
    MULTIPLY,
    EQUALS,
    RPAREN,
    LPAREN,
    COMMA,
}

pub enum AST {
    InfixOp {
        op: Op,
        lhs: String,
        rhs: String
    }
}

impl AST {
    fn eval(&self) {
        match self {
            AST::InfixOp { op: Op::PLUS, rhs , lhs } => {
                let lhs_parsed = match lhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Error parsing LHS");
                        exit(1);
                    }
                };
                let rhs_parsed = match rhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Error parsing RHS");
                        exit(1)
                    }
                };
                let ans = lhs_parsed + rhs_parsed;
                println!("{}" , ans);

            }
            AST::InfixOp { op: Op::MINUS, rhs , lhs } => {
                let lhs_parsed = match lhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Error parsing LHS");
                        exit(1);
                    }
                };
                let rhs_parsed = match rhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Error parsing RHS");
                        exit(1)
                    }
                };
                let ans = lhs_parsed - rhs_parsed;
                println!("{}" , ans);

            }
            AST::InfixOp { op: Op::MULTIPLY, rhs , lhs } => {
                let lhs_parsed = match lhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Error parsing LHS");
                        exit(1);
                    }
                };
                let rhs_parsed = match rhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Error parsing RHS");
                        exit(1)
                    }
                };
                let ans = lhs_parsed * rhs_parsed;
                println!("{}" , ans);
            }
            AST::InfixOp { op: Op::DIVIDE, rhs , lhs } => {
                let lhs_parsed = match lhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Error parsing LHS");
                        exit(1);
                    }
                };
                let rhs_parsed = match rhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Error parsing RHS");
                        exit(1)
                    }
                };
                let ans = lhs_parsed / rhs_parsed;
                println!("{}" , ans);
            }
            _  => println!("AST Err")
        }
    }
}



pub fn parse(tokens : Vec<String>) {
    for (index , token) in tokens.iter().enumerate() {
        if is_operator(&token) {
            match token.as_str() {
                "+" => {
                    let rhs = tokens.get(index + 1);
                    let lhs = tokens.get(index - 1);
                    
                    let rhs_parsed = match rhs {
                        Some(r) => r,
                        None => {
                            println!("You did something wrong with RHS, I'm sure you did");
                            exit(1)
                        }
                    };

                    let lhs_parsed = match lhs {
                        Some(r) => r,
                        None => {
                            println!("You did something wrong with LHS, I'm sure you did");
                            exit(1)
                        }
                    };
                    AST::eval(&AST::InfixOp {op : Op::PLUS , rhs :  rhs_parsed.clone() , lhs : lhs_parsed.clone()})
                }
                "-" => {
                    let rhs = tokens.get(index + 1);
                    let lhs = tokens.get(index - 1);
                    
                    let rhs_parsed = match rhs {
                        Some(r) => r,
                        None => {
                            println!("You did something wrong with RHS, I'm sure you did");
                            exit(1)
                        }
                    };

                    let lhs_parsed = match lhs {
                        Some(r) => r,
                        None => {
                            println!("You did something wrong with LHS, I'm sure you did");
                            exit(1)
                        }
                    };
                    AST::eval(&AST::InfixOp {op : Op::MINUS , rhs :  rhs_parsed.clone() , lhs : lhs_parsed.clone()})
                }
                "/" => {
                    let rhs = tokens.get(index + 1);
                    let lhs = tokens.get(index - 1);
                    
                    let rhs_parsed = match rhs {
                        Some(r) => r,
                        None => {
                            println!("You did something wrong with RHS, I'm sure you did");
                            exit(1)
                        }
                    };

                    let lhs_parsed = match lhs {
                        Some(r) => r,
                        None => {
                            println!("You did something wrong with LHS, I'm sure you did");
                            exit(1)
                        }
                    };
                    AST::eval(&AST::InfixOp {op : Op::DIVIDE , rhs :  rhs_parsed.clone() , lhs : lhs_parsed.clone()})
                }
                "*" => {
                    let rhs = tokens.get(index + 1);
                    let lhs = tokens.get(index - 1);
                    
                    let rhs_parsed = match rhs {
                        Some(r) => r,
                        None => {
                            println!("You did something wrong with RHS, I'm sure you did");
                            exit(1)
                        }
                    };

                    let lhs_parsed = match lhs {
                        Some(r) => r,
                        None => {
                            println!("You did something wrong with LHS, I'm sure you did");
                            exit(1)
                        }
                    };
                    AST::eval(&AST::InfixOp {op : Op::MULTIPLY , rhs :  rhs_parsed.clone() , lhs : lhs_parsed.clone()})
                }
                "," => AST::eval(&AST::InfixOp {op : Op::COMMA , rhs :  tokens[index + 1].clone() , lhs : tokens[index - 1].clone()}),
                "(" => AST::eval(&AST::InfixOp {op : Op::LPAREN , rhs :  tokens[index + 1].clone() , lhs : tokens[index - 1].clone()}),
                ")" => AST::eval(&AST::InfixOp {op : Op::RPAREN , rhs :  tokens[index + 1].clone() , lhs : tokens[index - 1].clone()}),
                "=" => AST::eval(&AST::InfixOp {op : Op::EQUALS , rhs :  tokens[index + 1].clone() , lhs : tokens[index - 1].clone()}),
                _ => println!("AST Err")
            }
        }
        else if is_digit(&token) {
            //println!("c'est une digit {}" , token)
        }
        else {
            //println!("{}" , &token);
        }
    }
}