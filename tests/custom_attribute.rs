use std::path::{Path, PathBuf};
use winrt::*;

fn find_winmds<P: AsRef<Path>>(directory: P) -> Vec<PathBuf> {
    let mut result = Vec::new();
    for entry in std::fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some("winmd") = path.extension().and_then(|s| s.to_str()) {
                result.push(path);
            }
        }
    }
    result
}

#[test]
fn named_arguments() -> Result<()> {
    // Get our WinMD files from the target directory
    let files = {
        let mut files = Vec::new();
        for entry in std::fs::read_dir("target/nuget").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                files.append(&mut find_winmds(path));
            }
        }
        files
    };

    let reader = winrt_gen::TypeReader::from_iter(files);
    let type_def = reader.resolve_type_def(("TestComponent", "TestRunner"));

    // TestRunner should have a custom attribute on it
    let mut some_string = 0;
    let mut some_int = 0;
    let mut some_bool = 0;
    for attribute in type_def.attributes(&reader) {
        match attribute.name(&reader) {
            ("TestComponent", "CustomTestAttribute") => {
                for (name, arg) in attribute.args(&reader) {
                    match (&name as &str, &arg) {
                        ("SomeString", winrt_gen::winmd::AttributeArg::String(value)) => {
                            assert_eq!(value, "Hello, World!");
                            some_string += 1;
                        }
                        ("SomeInt", winrt_gen::winmd::AttributeArg::I32(value)) => {
                            assert_eq!(*value, 1975);
                            some_int += 1;
                        }
                        ("SomeBool", winrt_gen::winmd::AttributeArg::Bool(value)) => {
                            assert_eq!(*value, true);
                            some_bool += 1;
                        }
                        _ => panic!(
                            "Unexpected named argument! Name: '{}'  Arg: {:?}",
                            name, arg
                        ),
                    }
                }
            }
            _ => {}
        }
    }
    assert_eq!(some_string, 1);
    assert_eq!(some_int, 1);
    assert_eq!(some_bool, 1);

    Ok(())
}
