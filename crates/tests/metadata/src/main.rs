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
    let now = Instant::now();
    let _ = reader::TypeReader::get();
    println!("TypeReader: {}", now.elapsed().as_millis());

    writer::test();
    let mut files = Vec::new();
    files.push(reader2::File::new("crates/libs/metadata/default/Windows.winmd").unwrap());
    files.push(reader2::File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap());
   // files.push(reader2::File::new("/git/test.winmd").unwrap());

    let now = Instant::now();
    let scope = reader2::Scope::new(&files);
    println!("Scope: {}", now.elapsed().as_millis());

    let now = Instant::now();
    let _tree = scope.tree();
    println!("Tree: {}", now.elapsed().as_millis());

    for ty in scope.resolve(&reader2::TypeName::new("Windows.Foundation", "Rect")) {
        for m in ty.fields() {
            println!("{}", m.name());
        }
    }
}
