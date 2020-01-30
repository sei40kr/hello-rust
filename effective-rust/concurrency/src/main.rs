use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;

            tx.send(()).unwrap();
        });
    }

    for _ in 0..3 {
        rx.recv().unwrap();
    }

    assert_eq!(*data.lock().unwrap(), vec![2, 3, 4]);
}
