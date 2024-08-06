use std::rc::Rc;

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    print!("a = ");
    print_cons(&a);

    print!("b = ");
    print_cons(&b);

    print!("c = ");
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