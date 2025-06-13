use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("error dumbass");
    let arr = parseline(input);
    if arr[0] == 1 {
        for i in 1..arr.len() {
            if arr[i] < arr[i - 1] {
                print!("mixed");
                return;
            }
        }
        print!("ascending");
    } else if arr[0] == 8 {
        for i in 1..arr.len() {
            if arr[i] > arr[i - 1] {
                print!("mixed");
                return;
            }
        }
        print!("descending");
    } else {
        println!("mixed");
    }
}

fn parseline(line: String) -> [u8; 8] {
    let mut arr: [u8; 8] = [0; 8];
    let mut i = 0;
    for n in line.chars().into_iter() {
        if (n >= '0') && (n <= '8') {
            arr[i] = n as u8 - '0' as u8;
            i += 1;
        }
    }
    return arr;
}
