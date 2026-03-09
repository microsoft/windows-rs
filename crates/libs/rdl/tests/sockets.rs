use windows_rdl::*;

#[test]
pub fn parse() {
    let output = std::env::temp_dir().join("windows-rdl-sockets-test");
    Writer::new()
        .input("../bindgen/default/Windows.winmd")
        .output(output.to_str().unwrap())
        .namespace("Windows.Networking.Sockets")
        .split()
        .write()
        .unwrap();
}
