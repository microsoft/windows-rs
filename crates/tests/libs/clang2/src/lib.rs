use windows_clang2::Input;

pub fn input_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input")
}

pub fn fixture(name: &str) -> Input {
    let file = format!("{name}.h");
    let path = input_dir().join(&file);
    Input {
        name: file,
        source: std::fs::read_to_string(path).unwrap(),
        ..Default::default()
    }
}

pub fn selected_fixture(name: &str, roots: &[&str]) -> Input {
    let mut input = fixture(name);
    input.roots = roots.iter().map(|root| (*root).into()).collect();
    input.arguments = vec![
        "-I".into(),
        input_dir().display().to_string(),
        "-DCAPTURE_TEST=7".into(),
    ];
    input
}

pub fn sdk_fixture() -> Input {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let version = helpers::read_str_const(
        root.join("crates\\tools\\win32\\src\\main.rs"),
        "SDK_VERSION",
    );
    let include = helpers::nuget_package("microsoft.windows.sdk.cpp", &version)
        .join("c")
        .join("Include")
        .join(helpers::marketing_dir(&version));
    let mut input = fixture("sdk");
    input.roots.push("ListHolder".into());
    let resource = std::env::var_os("CLANG_RESOURCE_DIR").map_or_else(
        || {
            root.join("target\\tool-clang\\clang-resource")
                .join(helpers::LIBCLANG_VERSION)
        },
        std::path::PathBuf::from,
    );
    assert!(
        resource.join("include").join("intrin.h").is_file(),
        "SDK fixture requires clang resource headers at {} or CLANG_RESOURCE_DIR",
        resource.display()
    );
    input.arguments = vec![
        "-DWIN32_LEAN_AND_MEAN".into(),
        "-resource-dir".into(),
        std::fs::canonicalize(resource)
            .unwrap()
            .display()
            .to_string(),
    ];
    for directory in ["ucrt", "um", "shared"] {
        input.arguments.extend([
            "-isystem".into(),
            include.join(directory).display().to_string(),
        ]);
    }
    input
}

pub fn shared_graph(count: usize) -> [Input; 2] {
    assert!(count > 0);
    let mut source = String::new();
    for id in 0..count {
        source.push_str(&format!("struct R{id};\n"));
    }
    source.push_str("struct R0 { int value; R0* next; };\n");
    for id in 1..count {
        let previous = id - 1;
        source.push_str(&format!(
            "struct R{id} {{ R{previous}* left; R{previous}* right; R{id}* next; }};\n"
        ));
    }
    [
        Input {
            name: "graph_a.h".into(),
            source: source.clone(),
            ..Default::default()
        },
        Input {
            name: "graph_b.h".into(),
            source,
            ..Default::default()
        },
    ]
}
