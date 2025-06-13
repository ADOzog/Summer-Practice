use std::io;

fn main() {
    let mut nums: Vec<u8> = Vec::new();
    for _i in 0..10 {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("error dumbass");
        let num: u8 = input.trim().parse().unwrap();
        if !nums.contains(&(num % 42)) {
            nums.push(num % 42);
        }
    }
    println!("{}", nums.len());
}
