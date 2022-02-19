use crate::lexer::lex;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::BufReader;

// read file
pub fn read_file() -> Result<Vec<String> , io::Error> {
    let mut fi : Vec<String> = Vec::new(); //vector to return values
    let f = (File::open("./test.txt"))?; //open file
    let file = BufReader::new(&f); // buffer
    for line in file.lines() { // loop over
        let l = line.unwrap();
        let mut v = lex(&l);
        fi.append(&mut v); // put one thing to another
    }
    return Ok(fi) //give it back
}

// check if operator
pub fn is_operator(token : &String) -> bool {
    let operators = ["+" , "-" , "/" , "*" , "=" , "(" , ")" , ","]; // operators
    return operators.iter().any(|e|e == token); // this will never get sent but okay
}

// check if digit
pub fn is_digit(token : &String) -> bool {
    return token.parse::<f64>().is_ok(); // can't really use .is_numeric() since its a 'String'
}