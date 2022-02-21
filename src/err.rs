use std::process::exit;

pub fn lhs_error() -> ! {
    println!("Something went wrong parsing LHS");
    exit(1)
}

pub fn rhs_error() -> ! {
    println!("Something went wrong parsing RHS");
    exit(1)
}

pub fn ast_error() -> ! {
    println!("Something went wrong with the parser");
    exit(1)
}