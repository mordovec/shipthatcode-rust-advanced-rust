trait Shape { fn area(&self) -> f64; }
struct Square {
    side: f64,
}
struct Triangle {
    base: f64,
    height: f64,
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Square { side: 3.0 }),
        Box::new(Triangle { base: 4.0, height: 5.0 }),
    ];
    for s in &shapes {
        println!("{:.2}", s.area());
    }
}
