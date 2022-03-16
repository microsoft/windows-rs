fn main() {
    windows_metagen::test();
    windows_metadata::File::new("/git/test.winmd");
    //windows_metadata::File::new(r#"D:\git\windows-rs\crates\libs\metadata\default\windows.winmd"#);
}
