// TODO: why not make this the default
fn bindgen(args: &str) {
    // TODO: why not call it "run" - prior art?
    windows_bindgen::bindgen(args.split_whitespace());
}

#[test]
#[should_panic(expected = "failed to open file `file_not_found.txt`")]
fn file_not_found() {
    bindgen("--etc file_not_found.txt");
}

#[test]
#[should_panic(expected = "invalid option `-etc`")]
fn invalid_option() {
    bindgen("-etc");
}

#[test]
#[should_panic(expected = "invalid `--derive` must be `<type name>=Comma,Separated,List")]
fn invalid_derive() {
    bindgen("--in default --out out.txt --filter POINT --derive invalid");
}
