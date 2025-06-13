use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("error dumbass");
    let mut k: i32 = input.trim().parse().unwrap();
    let mut num: String = String::new();
    while k > 0 {
        if k % 2 == 1 {
            num.insert(0, '4');
        } else {
            num.insert(0, '7');
        }
        k -= 2;
    }
    println!("{}", num);
}
