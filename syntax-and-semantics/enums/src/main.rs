// main.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

#[allow(dead_code)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

#[allow(dead_code)]
enum BoardGameTurn {
    Move { squares: i32 },
    Pass,
}

fn main() {
    let _x = Message::Move { x: 3, y: 4 };
    let _y = BoardGameTurn::Move { squares: 1 };
}
