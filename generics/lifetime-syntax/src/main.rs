// main.rs --- Validating References with Lifetimes
// author: Seong Yong-ju <sei40kr@gmail.com>

#[allow(dead_code)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
