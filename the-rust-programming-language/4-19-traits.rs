use std::fmt::Debug;

// 4-19-traits.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

// fn foo<T: Clone + Debug>(x: T) {
//     x.clone();
//     println!("{:?}", x);
// }

#[allow(unused_must_use)]
fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

#[allow(unused_must_use)]
fn bar<T, K>(x: T, y: K)
where
    T: Clone,
    K: Clone + Debug,
{
    x.clone();
    y.clone();
    println!("{:?}", y);
}

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 {
        *self as i64
    }
}

#[allow(dead_code)]
fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}

#[allow(dead_code)]
fn inverse<T>() -> T
where
    i32: ConvertTo<T>,
{
    42.convert()
}

trait Foo {
    fn foo(&self);

    fn is_valid(&self) -> bool;

    fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}

trait FooBar: Foo {
    fn foobar(&self) {
        println!("foobar");
    }
}

fn main() {
    foo("Hello", "world");
    bar("Hello", "world");
}
