use std::path::Path;

// Round-trip test for windows-rdl: reads each `.rdl` file from the `input/`
// directory, compiles it to a `.winmd` via the RDL reader, then uses the RDL
// writer to regenerate `.rdl` from that `.winmd`. The output must match the
// original input exactly (canonical form).
//
// If a test fails, the input file is overwritten with the canonical form so
// you can review the diff and commit it.

include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));

fn run(name: &str) {
    let input_path = format!("input/{name}.rdl");
    let original = std::fs::read_to_string(&input_path)
        .unwrap_or_else(|e| panic!("failed to read {input_path}: {e}"));

    let out_dir = Path::new(env!("OUT_DIR"));
    std::fs::create_dir_all(out_dir).unwrap();

    let winmd_path = out_dir.join(format!("{name}.winmd"));
    let winmd_str = winmd_path.to_str().unwrap();

    // RDL → winmd
    windows_rdl::reader()
        .input(&input_path)
        .output(winmd_str)
        .write()
        .unwrap_or_else(|e| panic!("{name}: reader failed: {e}"));

    // winmd → RDL
    let rdl_out_path = out_dir.join(format!("{name}.rdl"));
    let rdl_out_str = rdl_out_path.to_str().unwrap();

    windows_rdl::writer()
        .input(winmd_str)
        .output(rdl_out_str)
        .write()
        .unwrap_or_else(|e| panic!("{name}: writer failed: {e}"));

    let actual = std::fs::read_to_string(&rdl_out_path)
        .unwrap_or_else(|e| panic!("failed to read {rdl_out_str}: {e}"));

    if actual != original {
        // Overwrite input with canonical form so the user can review the diff.
        std::fs::write(&input_path, &actual).unwrap();
        panic!(
            "{name}: round-trip mismatch — input has been updated to canonical form.\n\
             Review the diff with `git diff` and commit if correct."
        );
    }
}
