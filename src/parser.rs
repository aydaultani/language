use crate::utilties::{is_operator, is_digit};
use crate::err::{ast_error , lhs_error , rhs_error};
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
                        lhs_error();
                    }
                };
                let rhs_parsed = match rhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        rhs_error();
                    }
                };
                let ans = lhs_parsed + rhs_parsed;
                println!("{}" , ans);

            }
            AST::InfixOp { op: Op::MINUS, rhs , lhs } => {
                let lhs_parsed = match lhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        lhs_error();
                    }
                };
                let rhs_parsed = match rhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        rhs_error();
                    }
                };
                let ans = lhs_parsed - rhs_parsed;
                println!("{}" , ans);

            }
            AST::InfixOp { op: Op::MULTIPLY, rhs , lhs } => {
                let lhs_parsed = match lhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        lhs_error();
                    }
                };
                let rhs_parsed = match rhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        rhs_error();
                    }
                };
                let ans = lhs_parsed * rhs_parsed;
                println!("{}" , ans);
            }
            AST::InfixOp { op: Op::DIVIDE, rhs , lhs } => {
                let lhs_parsed = match lhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        lhs_error();
                    }
                };
                let rhs_parsed = match rhs.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        rhs_error();
                    }
                };
                let ans = lhs_parsed / rhs_parsed;
                println!("{}" , ans);
            }
            _  => ast_error()
        }
    }
}



pub fn parse(tokens : Vec<String>) {
    for (index , mut token) in tokens.iter().enumerate() {
        if is_operator(&token) {
            match token.as_str() {
                "+" => {

                    //let rhs = tokens.get(index);
                    //let lhs = tokens.get(index);

                    let rhs = match tokens.get(index + 1) {
                        Some(v) => v,
                        None => rhs_error()
                    };

                    let lhs = match tokens.get(index.checked_sub(1).unwrap_or(0)) {
                        Some(v) => v,
                        None => lhs_error()
                    };
                    
                    AST::eval(&AST::InfixOp {op : Op::PLUS , rhs :  rhs.clone() , lhs : lhs.clone()})
                }
                "-" => {

                    let rhs = match tokens.get(index + 1) {
                        Some(v) => v,
                        None => rhs_error()
                    };

                    let lhs = match tokens.get(index.checked_sub(1).unwrap_or(0)) {
                        Some(v) => v,
                        None => lhs_error()
                    };
                    
                    AST::eval(&AST::InfixOp {op : Op::MINUS , rhs :  rhs.clone() , lhs : lhs.clone()})
                }
                "/" => {
                    
                    let rhs = match tokens.get(index + 1) {
                        Some(v) => v,
                        None => rhs_error()
                    };

                    let lhs = match tokens.get(index.checked_sub(1).unwrap_or(0)) {
                        Some(v) => v,
                        None => lhs_error()
                    };

                    AST::eval(&AST::InfixOp {op : Op::DIVIDE , rhs :  rhs.clone() , lhs : lhs.clone()})
                }
                "*" => {
                    
                    let rhs = match tokens.get(index + 1) {
                        Some(v) => v,
                        None => rhs_error()
                    };

                    let lhs = match tokens.get(index.checked_sub(1).unwrap_or(0)) {
                        Some(v) => v,
                        None => lhs_error()
                    };

                    AST::eval(&AST::InfixOp {op : Op::MULTIPLY , rhs :  rhs.clone() , lhs : lhs.clone()})
                }
                "," => AST::eval(&AST::InfixOp {op : Op::COMMA , rhs :  tokens[index + 1].clone() , lhs : tokens[index - 1].clone()}),
                "(" => AST::eval(&AST::InfixOp {op : Op::LPAREN , rhs :  tokens[index + 1].clone() , lhs : tokens[index - 1].clone()}),
                ")" => AST::eval(&AST::InfixOp {op : Op::RPAREN , rhs :  tokens[index + 1].clone() , lhs : tokens[index - 1].clone()}),
                "=" => AST::eval(&AST::InfixOp {op : Op::EQUALS , rhs :  tokens[index + 1].clone() , lhs : tokens[index - 1].clone()}),
                _ => ast_error()
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