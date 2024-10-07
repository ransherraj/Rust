use crate::modules::{input_string, to_int, sum};

use std::{io, cmp::Ordering};
mod modules;

fn main() {
    println!("Guess Game!");

    //let rand_num = rand::thread_rng().gen_range(1..101);
    let left = 1;
    let right = 100;
    let rand_num = modules::gen_random(left, right);
    println!("Random Number! :  {}", rand_num);
    
    println!("Enter a number");
    let num = input_string();
    let num_int = to_int(num);

    match num_int.cmp(&rand_num){
        Ordering::Less=>println!("Too Small"),
        Ordering::Greater=>println!("Too Large"),
        Ordering::Equal=>println!("You Win!!!!")
    }
    
    println!("rand num + guess int = {}", sum(num_int, rand_num));
        
}
