// 4-26-const-and-static.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

const _N: i32 = 5;
static _M: i32 = 5;
static _NAME: &'static str = "Steve";
static mut N: i32 = 5;

fn main() {
    unsafe {
        N += 1;

        println!("N: {}", N);
    }
}
