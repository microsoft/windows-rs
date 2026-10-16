fn main() -> windows_core::Result<()> {
    let fixture = std::env::args().nth(1).unwrap();
    test_reactor::run(&fixture)
}
