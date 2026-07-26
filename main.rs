use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();
    let raw: *const i32 = &n;
    unsafe {
        println!("{}", *raw);
    }
}