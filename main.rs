use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    let counter = Rc::new(RefCell::new(0));
    for _ in 0..3 {
        let c = Rc::clone(&counter);
        *c.borrow_mut() += 1;
    }
    println!("{}", counter.borrow());
}