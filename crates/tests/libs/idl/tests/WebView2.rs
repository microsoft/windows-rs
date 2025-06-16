use windows_idl as idl;

#[test]
fn test() {
    let file = std::include_str!("WebView2.idl");
    let _file = idl::parse(file).unwrap();
}
