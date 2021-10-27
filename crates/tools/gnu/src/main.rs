use reader::*;
use std::collections::BTreeMap;
use std::io::prelude::*;

fn main() {
    let mut cmd = std::process::Command::new("where");
    cmd.arg("dlltool.exe");
    let output = cmd.output().unwrap();

    if !output.status.success() {
        println!("dlltool.exe not found");
        return;
    }

    let output = unsafe { std::str::from_utf8_unchecked(&output.stdout) };

    let platform = if output.contains("mingw64") {
        "x86_64_gnu"
    } else if output.contains("mingw32") {
        "i686_gnu"
    } else {
        println!("mingw not found");
        return;
    };

    let reader = TypeReader::get_mut();

    let mut libraries = BTreeMap::<String, BTreeMap<&'static str, usize>>::new();
    let root = reader.types.get_namespace("Windows.Win32").unwrap();
    load_functions(root, &mut libraries);

    let mut output = std::path::PathBuf::from(reader::workspace_dir());
    output.push(format!("crates\\targets\\{}\\lib", platform));

    let _ = std::fs::remove_dir_all(&output);
    std::fs::create_dir_all(&output).unwrap();

    let def_path = output.join("windows.def");
    let mut def = std::fs::File::create(&def_path).unwrap();

    for (library, functions) in &libraries {
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

        if platform == "i686_gnu" {
            for (function, params) in functions {
                def.write_all(format!("{}@{}\n", function, params).as_bytes())
                    .unwrap();
            }
        } else {
            for function in functions.keys() {
                def.write_all(format!("{}\n", function).as_bytes()).unwrap();
            }
        }
    }

    drop(def);

    let mut cmd = std::process::Command::new("dlltool");
    cmd.current_dir(&output);
    cmd.arg("-d");
    cmd.arg("windows.def");
    cmd.arg("-l");
    cmd.arg("libwindows.a");
    cmd.output().unwrap();

    std::fs::remove_file(def_path).unwrap();
}

fn load_functions(
    tree: &TypeTree,
    libraries: &mut BTreeMap<String, BTreeMap<&'static str, usize>>,
) {
    tree.types
        .values()
        .map(|entry| entry.def.iter())
        .flatten()
        .for_each(|def| load_function(def, libraries));

    tree.namespaces
        .values()
        .for_each(|tree| load_functions(tree, libraries));
}

fn load_function(
    def: &ElementType,
    libraries: &mut BTreeMap<String, BTreeMap<&'static str, usize>>,
) {
    if let ElementType::MethodDef(def) = def {
        let library = def
            .impl_map()
            .expect("Function")
            .scope()
            .name()
            .to_lowercase();

        let params = def.signature(&[]).size();

        libraries
            .entry(library)
            .or_default()
            .insert(def.name(), params);
    }
}
