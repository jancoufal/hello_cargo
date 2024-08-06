use std::rc::Rc;

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    let c = Rc::new(Cons(4, Rc::clone(&a)));

    print!("a (strong count {}) = ", Rc::strong_count(&a));
    print_cons(&a);

    print!("b (strong count {}) = ", Rc::strong_count(&b));
    print_cons(&b);

    print!("c (strong count {}) = ", Rc::strong_count(&c));
    print_cons(&c);
}

fn print_cons(l: &List) {
    match l {
        Cons(v, ll) => {
            print!("{}..", v);
            print_cons(ll);
        }
        Nil => {
            println!("/");
        }
    }
}