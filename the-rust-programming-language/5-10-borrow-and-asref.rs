use std::borrow::Borrow;
use std::fmt::Display;

// 5-10-borrow-and-asref.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

fn foo<T: Borrow<i32> + Display>(a: T) {
    println!("a is borrowed: {}", a);
}

#[allow(dead_code)]
fn bar<T: AsRef<i32>>(s: T) {
    let _slice = s.as_ref();
}

fn main() {
    let mut i = 5;

    foo(&i);
    foo(&mut i);
}
