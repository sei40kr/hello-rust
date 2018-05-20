struct HasDrop;

// 4-20-drop.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    let _x = HasDrop;
}
