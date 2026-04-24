#![cfg(target_pointer_width = "64")]

#[test]
fn input_str() {
    windows_rdl::clang()
        .input_str(
            r#"
#include "tests/input_str.h"
struct Thing {
    Included a;
};
        "#,
        )
        .output("tests/input_str.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
