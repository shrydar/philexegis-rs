#[macro_use]
extern crate serde_derive;
mod core;

fn main() {
    core::test_deserialize();
}
