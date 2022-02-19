pub fn lex(input: &str) -> Vec<String> {
    let _types = ["num" , "op" , "id"]; // types
    let operators = ['+' , '-' , '/' , '*' , '=' , '(' , ')' , ',']; // operators
    let debug = false;
    let starters = ['v']; // starters
    let mut found : Vec<String> = Vec::new(); //vector to return values

    let mut current_number = "".to_string(); //number parsing

    // parse
    for item in input.chars() {
        // parse number
        if item.is_numeric() {
            current_number.push(item);
        }
        else {
            let t_type = "num";
            if !current_number.is_empty() {
                if debug {
                    println!("[{t_type} : {current_number}]");
                }
                found.push((*current_number).to_string());
                current_number.clear();
            }
        }

        // parse operators
        if operators.contains(&item) {
            let t_type = "op";
            let o_type = match item {
                    '+' => '+',
                    '-' => '-',
                    '/' => '/',
                    '*' => '*',
                    '=' => '=',
                    '(' => '(',
                    ')' => ')',
                    ',' => ',',
                    _   => ' ',
                };

            //println!("MATCH {}" , item);
            //let _current = &input.find(o_type).unwrap();

            if debug {
                println!("[{} : '{}']" , t_type , o_type);
            }
            found.push(o_type.to_string());
        }

        // parse id's
        if starters.contains(&item) {
            let t_type = "id";

            #[allow(unused_assignments)]
            let mut o_type = "";
            let c_type = match item {
                'v' => 'v',
                _ => ' ',
            };
            if c_type == 'v' {
                o_type = "var";
            }
            else {
                o_type = "";
            }

            if debug {
                println!("[{t_type} : {o_type}]");
            }
            found.push(o_type.to_string());
        }

    }

    // number checking bug fix
    if !current_number.is_empty() {
        let t_type = "num";
        if debug {
            println!("[{t_type} : {current_number}]");
        }
        found.push((*current_number).to_string());
        current_number.clear();
    }

    return found;

}
