use std::io;
mod Problem1;

fn main() {
    println!("Type a number: ");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error dumbass");
    let a = input.trim().parse::<u16>().unwrap();
    // println!("Type another smaller number: ");
    // input = String::new();
    // io::stdin().read_line(&mut input).expect("Error dumbass");
    // let b = input.trim().parse::<u16>().unwrap();
    println!("The number is prime: {}", Problem1::prime(a));
}
