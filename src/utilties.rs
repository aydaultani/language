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
