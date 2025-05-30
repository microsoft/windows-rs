use windows_idl as idl;

#[test]
fn test() {
    let file = std::fs::read_to_string("tests/WebView2.idl").unwrap();
    let _file = idl::parse(&file).unwrap();

}
