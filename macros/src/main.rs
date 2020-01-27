// main.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}
