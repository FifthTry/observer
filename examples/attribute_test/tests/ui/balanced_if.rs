#[macro_use]
extern crate observer_attribute;

#[balanced_if]
fn main() {
    if true {
        let x = 5;
    } else {
    }
}