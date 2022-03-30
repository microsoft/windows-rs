// use windows_metadata::reader2::*;

// fn main() {
//     let winrt = File::new("crates/libs/metadata/default/Windows.winmd").unwrap();
//     let win32 = File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap();
//     let files = [winrt, win32];
//     let scope = Scope::new(&files);

//     for ns in scope.namespaces() {
//         println!("{}", ns);
//     }

//     println!("");

//     for ns in scope.nested_namespaces("Windows.UI.Xaml") {
//         println!("{}", ns);
//     }
// }

use windows_metadata::writer::*;

fn main() {
    test();
}