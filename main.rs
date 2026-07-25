use std::io::{self, BufRead};
use std::rc::Rc;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let shared = Rc::new(nums);
    let _a = shared.clone();
    let _b = shared.clone();
    println!("count: {}", Rc::strong_count(&shared));
    println!("sum: {}", shared.iter().sum::<i32>());
}