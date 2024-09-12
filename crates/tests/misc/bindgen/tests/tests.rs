#[test]
fn test() {
    let e = windows_bindgen::bindgen(["--etc", "file_not_found.txt"]).unwrap_err();

    assert_eq!(
        format!("{e}"),
        "error: failed to read lines\n  --> file_not_found.txt\n"
    );

    let e = windows_bindgen::bindgen(["-etc"]).unwrap_err();

    assert_eq!(format!("{e}"), "error: invalid option `-etc`\n");
}
