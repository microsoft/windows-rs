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
    let files = vec![
    reader2::File::new("crates/libs/metadata/default/Windows.winmd").unwrap(),
    reader2::File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap(),
   // files.push(reader2::File::new("/git/test.winmd").unwrap());
    ];

    let now = Instant::now();
    let reader = &reader2::Reader::new(&files);
    println!("Scope: {}", now.elapsed().as_millis());

    let now = Instant::now();
    let _tree = reader.tree();
    println!("Tree: {}", now.elapsed().as_millis());

    for ty in reader.get(reader2::TypeName::new("Windows.UI.Composition", "Compositor")) {
       //if ty.attributes(scope).any(|a|a.name(scope) == "ActivatableAttribute") {
           println!("found!");
       //}
    }
}
