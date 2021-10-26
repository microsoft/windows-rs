use reader::*;
use std::collections::BTreeMap;
use std::io::prelude::*;

fn main() {
    let mut cmd = std::process::Command::new("where");
    cmd.arg("dlltool.exe");
    
    if !cmd.output().unwrap().status.success() {
        println!("dlltool.exe not found");
        return;
    }

    let reader = TypeReader::get_mut();

    let mut libraries = BTreeMap::<String, BTreeMap<&'static str, usize>>::new();
    let root = reader.types.get_namespace("Windows.Win32").unwrap();
    load_functions(root, &mut libraries);

    let mut output = std::path::PathBuf::from(reader::workspace_dir());
    output.push("crates\\targets");

    let lib = output.join("i686_gnu\\lib");
    let _ = std::fs::remove_dir_all(&lib);
    std::fs::create_dir_all(&lib).unwrap();

    let lib = output.join("x86_64_gnu\\lib");
    let _ = std::fs::remove_dir_all(&lib);
    std::fs::create_dir_all(&lib).unwrap();

    for (library, functions) in &libraries {
        build_library(&output, library, functions);
    }
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

fn build_library(
    output: &std::path::Path,
    library: &str,
    functions: &BTreeMap<&'static str, usize>,
) {
    println!("{}", library);

    // Note that we don't use set_extension as it confuses PathBuf when the library name includes a period.

    let def_path = output.join(format!("{}.def", library));
    let mut def = std::fs::File::create(&def_path).unwrap();

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
        def.write_all(format!("{}@{}\n", function, params).as_bytes()).unwrap();
    }

    drop(def);

    let mut cmd = std::process::Command::new("dlltool");
    cmd.current_dir(&output);
    cmd.arg("-m");
    cmd.arg("i386");
    cmd.arg("-d");
    cmd.arg(format!("{}.def", library));
    cmd.arg("-l");
    cmd.arg(format!("i686_gnu\\lib\\{}.a", library));
    cmd.output().unwrap();

    let mut cmd = std::process::Command::new("dlltool");
    cmd.current_dir(&output);
    cmd.arg("-m");
    cmd.arg("i386:x86-64");
    cmd.arg("-d");
    cmd.arg(format!("{}.def", library));
    cmd.arg("-l");
    cmd.arg(format!("x86_64_gnu\\lib\\{}.a", library));
    cmd.output().unwrap();

    std::fs::remove_file(output.join(format!("{}.def", library))).unwrap();
}
