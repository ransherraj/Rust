use std::{io, cmp::Ordering};

pub fn input_string() -> String{
    println!("Enter ----");

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Enter valid input");
    return inp.trim().to_string();
}

