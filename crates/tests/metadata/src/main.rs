use windows_metadata::reader2::*;

fn main() {
    let winrt = File::new("crates/libs/metadata/default/Windows.winmd").unwrap();
    let win32 = File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap();
    let _scope = Scope::new(&[winrt, win32]);
}
