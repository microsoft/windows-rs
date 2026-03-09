use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/sockets.rdl")
        .output("tests/sockets.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/sockets.winmd")
        .output("tests/sockets.rdl")
        .namespace("Test")
        .write()
        .unwrap();

    // Verify that Windows.Networking.Sockets attributes, which use
    // SERIALIZATION_TYPE_ENUM (0x55 per ECMA-335 §II.23.1.16) for named
    // enum-typed arguments, are handled without panicking.
    let output = std::env::temp_dir().join("windows-rdl-sockets-ns-test");
    Writer::new()
        .input("../bindgen/default/Windows.winmd")
        .output(output.to_str().unwrap())
        .namespace("Windows.Networking.Sockets")
        .split()
        .write()
        .unwrap();
}
