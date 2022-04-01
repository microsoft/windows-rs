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

use windows_metadata::*;

fn main() {
    writer::test();
    let test = reader2::File::new("/git/test.winmd").unwrap();
    let files = [test];
    let scope = reader2::Scope::new(&files);

    for ns in scope.raw_types() {
        println!("{}.{}", ns.namespace(), ns.name());
    }
}
