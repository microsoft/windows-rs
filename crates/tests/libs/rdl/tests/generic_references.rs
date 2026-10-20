use windows_metadata::reader::*;

// Generic references must resolve through the referenced assembly like non-generic ones. The
// CsWinRT side of this (#5140) is covered by `test_winrt_interop`.
#[test]
fn generic_type_refs_use_assembly_scope() {
    let winmd = std::path::Path::new(env!("OUT_DIR")).join("generic_references.winmd");
    windows_rdl::reader()
        .input_text(
            "#[winrt]\nmod Test {\n    interface IWidget {\n        event Changed: Windows::Foundation::TypedEventHandler<Object, Object>;\n    }\n}\n",
        )
        .output(&winmd)
        .reference_default()
        .write()
        .unwrap();

    let index = Index::read(&winmd).unwrap();
    let method = index
        .expect("Test", "IWidget")
        .methods()
        .find(|method| method.name() == "add_Changed")
        .unwrap();

    let mut blob = method.blob(4);
    let header: Vec<u8> = (0..3).map(|_| blob.read_u8()).collect();
    let _: TypeDefOrRef = blob.decode();
    let instance = [blob.read_u8(), blob.read_u8()];
    let reference: TypeDefOrRef = blob.decode();
    let arguments: Vec<u8> = (0..3).map(|_| blob.read_u8()).collect();
    drop(blob);

    // HASTHIS, one parameter, VALUETYPE return (EventRegistrationToken).
    assert_eq!(header, [0x20, 1, 0x11]);
    // GENERICINST CLASS TypedEventHandler`2<OBJECT, OBJECT>
    assert_eq!(instance, [0x15, 0x12]);
    assert_eq!(arguments, [2, 0x1C, 0x1C]);
    let TypeDefOrRef::TypeRef(reference) = reference else {
        panic!("expected TypeRef");
    };
    assert_eq!(reference.namespace(), "Windows.Foundation");
    assert_eq!(reference.name(), "TypedEventHandler`2");
    assert!(
        matches!(reference.scope(), ResolutionScope::AssemblyRef(_)),
        "generic TypeRef must be scoped to its assembly"
    );
}
