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
use std::time::*;

fn main() {
    writer::test();
    // let winrt = reader2::File::new("crates/libs/metadata/default/Windows.winmd").unwrap();
    // let win32 = reader2::File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap();
    // let files = [winrt, win32];
    let files = [ reader2::File::new("/git/test.winmd").unwrap()];

    let now = Instant::now();
    let scope = reader2::Scope::new(&files);
    println!("scope: {}", now.elapsed().as_millis());

    // for ty in scope.raw_types() {
    //     println!("raw: {}.{}", ty.namespace(), ty.name());
    // }

    // for ns in scope.namespaces() {
    //     println!("ns: {}", ns);
    // }

    // for ns in scope.nested_namespaces("") {
    //     println!(". {}", ns);
    // }

    // for ns in scope.nested_namespaces("Windows") {
    //     println!("Windows. {}", ns);
    // }

    let now = Instant::now();
    let tree = scope.tree();
    println!("tree: {}", now.elapsed().as_millis());

    println!("{:#?}", tree);
}
