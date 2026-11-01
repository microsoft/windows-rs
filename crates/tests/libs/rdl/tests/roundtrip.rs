use std::path::Path;

// Round-trip test for windows-rdl: reads each `.rdl` file from the `input/` directory, compiles it
// to a `.winmd` via the RDL reader, then uses the RDL writer to regenerate `.rdl` from that
// `.winmd`. The canonical form is written back to the input; CI rejects any resulting `git diff`.

include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));

fn run(name: &str) {
    let input_path = format!("input/{name}.rdl");
    let out_dir = Path::new(env!("OUT_DIR"));
    std::fs::create_dir_all(out_dir).unwrap();

    let winmd_path = out_dir.join(format!("{name}.winmd"));

    windows_rdl::reader()
        .input(&input_path)
        .output(&winmd_path)
        .write()
        .unwrap_or_else(|e| panic!("{name}: reader failed: {e}"));

    let rdl_out_path = out_dir.join(format!("{name}.rdl"));

    windows_rdl::writer()
        .input(&winmd_path)
        .output(&rdl_out_path)
        .write()
        .unwrap_or_else(|e| panic!("{name}: writer failed: {e}"));

    let actual = std::fs::read_to_string(&rdl_out_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", rdl_out_path.display()));

    std::fs::write(&input_path, actual).unwrap();
}
