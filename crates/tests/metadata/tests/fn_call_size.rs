#[test]
fn size() {
    assert_eq!(
        function_size("Windows.Win32.System.Console", "ReadConsoleOutputA"),
        20
    );
    assert_eq!(
        function_size("Windows.Win32.System.Console", "ReadConsoleOutputAttribute"),
        20
    );
    assert_eq!(
        function_size(
            "Windows.Win32.UI.Accessibility",
            "ItemContainerPattern_FindItemByProperty"
        ),
        32
    );
    assert_eq!(function_size("Windows.Win32.System.Ole", "VarI2FromCy"), 12);
    assert_eq!(
        function_size(
            "Windows.Win32.UI.Accessibility",
            "UiaRaiseAutomationPropertyChangedEvent"
        ),
        40
    );
    assert_eq!(
        function_size("Windows.Win32.Graphics.Gdi", "AlphaBlend"),
        44
    );
    assert_eq!(
        function_size("Windows.Win32.UI.Accessibility", "TextRange_FindAttribute"),
        32
    );
}

fn function_size(namespace: &str, name: &str) -> usize {
    let files =
        vec![
            metadata::reader::File::new("../../libs/metadata/default/Windows.Win32.winmd").unwrap(),
        ];
    let reader = &metadata::reader::Reader::new(&files);
    if let Some(def) = reader
        .get(metadata::reader::TypeName::new(namespace, "Apis"))
        .next()
    {
        for method in reader.type_def_methods(def) {
            if reader.method_def_name(method) == name {
                return reader.method_def_size(method);
            }
        }
    }
    0
}
