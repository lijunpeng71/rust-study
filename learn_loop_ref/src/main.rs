use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

enum List{
    Cons(i32,RefCell<Rc<List>),
    Nil,
}

impl List{
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_,item) => Some(item),
            Nil=>None,
        }
    }
}

fn main() {
    let a=Rc::new(Cons(5,RefCell::new(Rc::new(Nil))));
    println!("1, a rc count={}", Rc::strong_count(&a));
}
