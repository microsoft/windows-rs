use windows_clang::Input;

pub fn buffer_extent<'a>(
    owner: impl windows_metadata::HasAttributes<'a>,
    access: windows_metadata::BufferAccess,
    phase: windows_metadata::BufferPhase,
) -> Option<windows_metadata::BufferExtent> {
    let mut extents = owner
        .attributes()
        .filter(|attr| attr.name() == "BufferExtentAttribute")
        .map(|attr| windows_metadata::BufferExtent::from_values(&attr.value()).unwrap())
        .filter(|extent| extent.access == access && extent.phase == phase);
    let result = extents.next();
    assert!(
        extents.next().is_none(),
        "multiple extents for one access and phase"
    );
    result
}

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
