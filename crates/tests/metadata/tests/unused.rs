use metadata::*;

#[test]
fn test() {
    let files = tool_lib::default_metadata();
    let reader = &Reader::new(files);
    let filter = Filter::new(
        &["Windows", "BadNamespace", "Windows.AI"],
        &["Windows.Foundation.Rect", "Windows.Foundation.BadType"],
    );
    let unused: Vec<&str> = filter.unused(reader).collect();

    assert_eq!(unused, ["Windows.Foundation.BadType", "BadNamespace"]);
}
