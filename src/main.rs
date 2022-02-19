mod lexer;
mod utilties;
mod parser;

fn main() {
    let lexer = utilties::read_file(); // read the file
    match lexer {
        Ok(v) => parser::parse(v), //print result
        Err(ref e) => println!("{:?}", e), // catch errors
    };
}
