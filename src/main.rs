#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate png;
mod core;

fn main() {
    core::tests::test_deserialize();
}
