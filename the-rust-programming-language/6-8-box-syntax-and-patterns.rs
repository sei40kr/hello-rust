#![feature(box_syntax)]

// 6-8-box-syntax-and-patterns.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

#[allow(dead_code)]
struct BigStruct {
    one: i32,
    two: i32,
    // etc
    one_hundred: i32,
}

fn foo(x: Box<BigStruct>) -> BigStruct {
    *x
}

fn main() {
    let x = Box::new(BigStruct {
        one: 1,
        two: 2,
        one_hundred: 100,
    });

    let _y: Box<BigStruct> = box foo(x);
}
