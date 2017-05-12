#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate conrod;
extern crate base64;
extern crate png;
mod core;
mod ui;


fn main() {
    if true {
        core::tests::test_deserialize();
    }
    if true {
        ui::run();
    }
}
