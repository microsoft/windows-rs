use windows_clang2::Input;

pub fn fixture(name: &str) -> Input {
    let file = format!("{name}.h");
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .join(&file);
    Input {
        name: file,
        source: std::fs::read_to_string(path).unwrap(),
    }
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
        },
        Input {
            name: "graph_b.h".into(),
            source,
        },
    ]
}
