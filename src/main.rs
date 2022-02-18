mod lexer;
mod utilties;

fn main() {
    let parser = utilties::read_file();
    match parser {
        Ok(v) => println!("{:?}" , v), //print result
        Err(ref e) => println!("{:?}", e), // catch errors
        //_ => ()
    };
}
