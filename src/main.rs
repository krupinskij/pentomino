use std::collections::HashMap;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // let mut scores = HashMap::new();

    // scores.insert((0, 0), 10);
    // scores.insert((0, 1), 50);

    // let score = scores.get(&(0, 0)).copied().unwrap_or(0);
    // println!("{}", score)

    pentomino::run();

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
