use metadata::*;

#[test]
fn test() {
    let files = tool_lib::default_metadata();
    let reader = Reader::filter(
        files,
        &["Windows", "BadNamespace", "Windows.AI"],
        &["Windows.Foundation.Rect", "Windows.Foundation.BadType"],
        &Default::default(),
    );
    let unused: Vec<&str> = reader.unused().collect();

    assert_eq!(unused, ["Windows.Foundation.BadType", "BadNamespace"]);
}
