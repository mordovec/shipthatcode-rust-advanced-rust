// TODO: define macro_rules! sum! that takes any number of expressions and
// returns their sum
macro_rules! sum {
    ($v:expr) => {
        println!("{}", $v.iter().sum::<i32>())
    }
}

fn main() {
    // TODO: read a single line of integers and parse each one
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let a = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    sum!(a);
}