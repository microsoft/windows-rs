use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("module_attributes", "winrt", &[]);
    let _reader = Reader::new(files);
}
