use std::collections::BTreeMap;
use std::io::prelude::*;

fn main() {
    let platform = if let Some(platform) = option_env!("Platform") {
        match platform {
            "x86" => "i686_msvc",
            "x64" => "x86_64_msvc",
            "arm64" => "aarch64_msvc",
            _ => {
                println!("Unknown platform");
                return;
            }
        }
    } else {
        println!("Please run this tool from a Visual Studio command prompt");
        return;
    };

    let files = vec![metadata::reader::File::new("crates/libs/metadata/default/Windows.winmd").unwrap(), metadata::reader::File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap(), metadata::reader::File::new("crates/libs/metadata/default/Windows.Win32.Interop.winmd").unwrap()];
    let reader = &metadata::reader::Reader::new(&files);

    let mut libraries = BTreeMap::<String, BTreeMap<String, usize>>::new();
    let root = &reader.tree().nested["Windows"];
    for tree in root.flatten() {
        if let Some(apis) = reader.get(metadata::reader::TypeName::new(tree.namespace, "Apis")).next() {
            for method in reader.type_def_methods(apis) {
                let impl_map = reader.method_def_impl_map(method).expect("ImplMap not found");
                let scope = reader.impl_map_scope(impl_map);
                let library = reader.module_ref_name(scope).to_lowercase();
                let params = reader.method_def_size(method);
                libraries.entry(library).or_default().insert(reader.method_def_name(method).to_string(), params);
            }
        }
    }

    let output = std::path::PathBuf::from(format!("crates/targets/{}/lib", platform));
    let _ = std::fs::remove_dir_all(&output);
    std::fs::create_dir_all(&output).unwrap();

    for (library, functions) in &libraries {
        build_library(&output, library, functions);
    }

    let mut cmd = std::process::Command::new("lib");
    cmd.current_dir(&output);
    cmd.arg("/nologo");
    cmd.arg("/out:windows.lib");

    for library in libraries.keys() {
        cmd.arg(format!("{}.lib", library));
    }

    cmd.output().unwrap();

    for library in libraries.keys() {
        std::fs::remove_file(output.join(format!("{}.lib", library))).unwrap();
    }
}

fn build_library(output: &std::path::Path, library: &str, functions: &BTreeMap<String, usize>) {
    println!("{}", library);

    // Note that we don't use set_extension as it confuses PathBuf when the library name includes a period.

    let mut path = std::path::PathBuf::from(output);
    path.push(format!("{}.c", library));
    let mut c = std::fs::File::create(&path).unwrap();

    path.pop();
    path.push(format!("{}.def", library));
    let mut def = std::fs::File::create(&path).unwrap();

    def.write_all(
        format!(
            r#"
LIBRARY {}
EXPORTS
"#,
            library
        )
        .as_bytes(),
    )
    .unwrap();

    for (function, params) in functions {
        let mut buffer = format!("void __stdcall {}(", function);

        for param in 0..*params {
            use std::fmt::Write;
            write!(&mut buffer, "int p{}, ", param).unwrap();
        }

        if buffer.ends_with(' ') {
            buffer.truncate(buffer.len() - 2);
        }

        buffer.push_str(") {}\n");

        c.write_all(buffer.as_bytes()).unwrap();
        def.write_all(format!("{}\n", function).as_bytes()).unwrap();
    }

    drop(c);
    drop(def);

    let mut cmd = std::process::Command::new("cl");
    cmd.current_dir(output);
    cmd.arg("/nologo");
    cmd.arg("/c");
    cmd.arg(format!("{}.c", library));
    cmd.output().unwrap();

    let mut cmd = std::process::Command::new("lib");
    cmd.current_dir(output);
    cmd.arg("/nologo");
    cmd.arg(format!("/out:{}.lib", library));
    cmd.arg(format!("/def:{}.def", library));
    cmd.arg(format!("{}.obj", library));
    cmd.output().unwrap();

    path.pop();
    path.push(format!("{}.c", library));
    std::fs::remove_file(&path).unwrap();

    path.pop();
    path.push(format!("{}.def", library));
    std::fs::remove_file(&path).unwrap();

    path.pop();
    path.push(format!("{}.exp", library));
    std::fs::remove_file(&path).unwrap_or_else(|_| panic!("{:?}", path));

    path.pop();
    path.push(format!("{}.obj", library));
    std::fs::remove_file(&path).unwrap();
}
