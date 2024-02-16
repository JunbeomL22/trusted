use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let x = Rc::new(RefCell::new(42));
    println!("counter of x: {}", Rc::strong_count(&x));
    let y = x.clone();
    println!("counter of x: {}", Rc::strong_count(&x));
    let z = x.clone();
    println!("counter of x: {}", Rc::strong_count(&x));
    *y.borrow_mut() += 1;
    println!("x = {:?}, y = {:?}, z = {:?}", x.borrow(), y.borrow(), z.borrow());
    *z.borrow_mut() += 1;
    println!("x = {:?}, y = {:?}, z = {:?}", x.borrow(), y.borrow(), z.borrow());
}
