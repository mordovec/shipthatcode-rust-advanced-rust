async fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let _fut = double(7);
    println!("created future");
}