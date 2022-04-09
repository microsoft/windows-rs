use metadata::reader::*;
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

    let reader = TypeReader::get();

    let mut libraries = BTreeMap::<String, BTreeMap<&'static str, usize>>::new();
    let root = reader.types.get_namespace("Windows.Win32").unwrap();
    load_functions(root, &mut libraries);

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

fn load_functions(tree: &TypeTree, libraries: &mut BTreeMap<String, BTreeMap<&'static str, usize>>) {
    tree.types.values().flat_map(|entry| entry.iter()).for_each(|def| load_function(def, libraries));

    tree.namespaces.values().for_each(|tree| load_functions(tree, libraries));
}

fn load_function(def: &Type, libraries: &mut BTreeMap<String, BTreeMap<&'static str, usize>>) {
    if let Type::MethodDef(def) = def {
        let library = def.impl_map().expect("Function").scope().name().to_lowercase();

        let params = def.signature(&[]).size() / 4;

        libraries.entry(library).or_default().insert(def.name(), params);
    }
}

fn build_library(output: &std::path::Path, library: &str, functions: &BTreeMap<&'static str, usize>) {
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
            buffer.push_str(&format!("int p{}, ", param));
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
