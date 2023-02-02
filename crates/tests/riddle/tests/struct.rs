use test_riddle::run_riddle;
use windows_metadata::reader::*;

#[test]
fn riddle_struct() {
    let output = run_riddle("tests/struct.idl");
    let files = File::with_default(&[&output]).expect("Failed to open winmd files");
    let reader = &Reader::new(&files);

}
