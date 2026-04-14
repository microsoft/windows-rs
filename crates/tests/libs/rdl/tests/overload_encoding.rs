use windows_metadata::reader::{HasAttributes, TypeIndex};
use windows_metadata::Value;
use windows_rdl::*;

/// Verify that `#[overload("Common")] fn Unique(...)` is encoded into WINMD with:
///   - MethodDef.Name = "Common" (the logical/shared name)
///   - OverloadAttribute.value = "Unique" (the vtable-unique name)
///
/// This matches the WinRT metadata layout described in
/// <https://github.com/microsoft/windows-rs/issues/4166>.
#[test]
fn overload_encoding() {
    reader()
        .input_str(
            r#"
#[winrt]
mod Test {
    interface IFoo {
        #[overload("Method")]
        fn MethodOne(&self) -> i32;
        #[overload("Method")]
        fn MethodTwo(&self, a: i32) -> i32;
    }
}
"#,
        )
        .output("tests/overload_encoding.winmd")
        .write()
        .unwrap();

    let index = TypeIndex::read("tests/overload_encoding.winmd").unwrap();
    let ifoo = index.expect("Test", "IFoo");

    let methods: Vec<_> = ifoo.methods().collect();
    assert_eq!(methods.len(), 2);

    // MethodDef.Name = "Method" (common/logical name)
    // OverloadAttribute.value = "MethodOne" / "MethodTwo" (unique vtable name)
    assert_eq!(methods[0].name(), "Method");
    assert_eq!(
        overload_unique_name(&methods[0]).as_deref(),
        Some("MethodOne")
    );

    assert_eq!(methods[1].name(), "Method");
    assert_eq!(
        overload_unique_name(&methods[1]).as_deref(),
        Some("MethodTwo")
    );
}

/// Returns the unique vtable name stored in `OverloadAttribute.value`, if present.
fn overload_unique_name(method: &windows_metadata::reader::MethodDef<'_>) -> Option<String> {
    method
        .find_attribute("OverloadAttribute")?
        .value()
        .into_iter()
        .find(|(name, _)| name.is_empty())
        .and_then(|(_, v)| {
            if let Value::Utf8(s) = v {
                Some(s)
            } else {
                None
            }
        })
}
