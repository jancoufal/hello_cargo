fn main() {
    println!("closure_1");
    closure_1();

    println!("closure_2");
    closure_2();

    println!("closure_3");
    closure_3();

    println!("closure_4");
    closure_4();
}

fn closure_1() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

fn closure_2() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || {
        println!("from borrows_mutably closure (1): {list:?}");
        list.push(7);
        println!("from borrows_mutably closure (2): {list:?}");
    };

    borrows_mutably();
    // println!("After calling closure (1): {list:?}");

    borrows_mutably();
    println!("After calling closure (2): {list:?}");
}

fn closure_3() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut move_ownership = move || {
        println!("from borrows_mutably closure (1): {list:?}");
        list.push(7);
        println!("from borrows_mutably closure (2): {list:?}");
    };

    move_ownership();
    // println!("After calling closure (1): {list:?}");

    move_ownership();
    // println!("After calling closure (2): {list:?}");
}

// "FnOnce", "FnMut", "Fn" tests
fn closure_4() {
    c_fn_once_call();
}

fn c_fn_once<T>(f: impl FnOnce() -> T) -> T {
    f()
}

fn c_fn_once_call() {
    let s1 = "abc".to_string();
    println!("s1 = {s1:?}");

    let s2 = c_fn_once(|| s1.to_string());
    println!("s1 = {s1:?}");
    println!("s2 = {s2:?}");
}