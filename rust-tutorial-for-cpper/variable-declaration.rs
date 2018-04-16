// variable-declaration.rs --- Variable declarations
// author: Seong Yong-ju <sei40kr@gmail.com>

fn main() {
    // equivalent to "const int a = 1;"
    let a = 1;
    // equivalent to "int b = 3;"
    let mut b = 3;
    let c: i32 = 0;

    let i;
    i = 1;

    let mut j;
    j = 3;
    j = 7;

    let (t, u, v) = ("alpha", "beta", "charlie");
    let (mut w, mut x) = ("delta", "echo");
    let (y, z): (&str, &str) = ("delta", "echo");
}
