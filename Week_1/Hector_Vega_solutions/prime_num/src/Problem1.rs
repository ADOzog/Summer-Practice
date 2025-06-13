fn GCD(a: u16, b: u16) -> u16 {
    let mut arr = vec![0; a as usize];

    arr[0] = a;
    arr[1] = b;

    for i in 2..arr.len() {
        arr[i] = arr[i - 2] % arr[i - 1];
        if arr[i] == 0 {
            return arr[i - 1];
        }
    }
    return 1;
}

pub fn prime(a: u16) -> bool {
    for i in 1..a {
        if GCD(a, i) != 1 as u16 {
            return false;
        }
    }
    return true;
}
