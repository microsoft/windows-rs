#![allow(overflowing_literals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use windows::foundation::PropertyValue;

fn main() {
    let _value = PropertyValue::create_boolean(true).unwrap();
}
