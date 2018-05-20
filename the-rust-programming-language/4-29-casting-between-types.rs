use std::mem;

// 4-29-casting-between-types.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

fn main() {
    unsafe {
        let a = [0u8, 0u8, 0u8, 0u8];
        let _b = mem::transmute::<[u8; 4], u32>(a);

        // error
        // let _b = mem::transmute::<[u8; 4], u64>(a);
    }
}
