// main.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

fn main() {
    // safe
    {
        let x = 5;
        let _raw = &x as *const i32;

        let mut y = 10;
        let _raw_mut = &mut y as *mut i32;
    }

    // dangerous
    unsafe {
        let x = 5;
        let raw = &x as *const i32;

        println!("raw points at {}", *raw);
    }
}
