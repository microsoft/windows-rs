#[test]
#[should_panic(expected = "failed to open file `file_not_found.txt`")]
fn file_not_found() {
    windows_bindgen::bindgen(["--etc", "file_not_found.txt"]);
}

#[test]
#[should_panic(expected = "invalid option `-etc`")]
fn invalid_option() {
    windows_bindgen::bindgen(["-etc"]);
}
