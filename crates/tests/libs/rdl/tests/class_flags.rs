use windows_metadata as metadata;
use windows_metadata::TypeAttributes;

#[test]
fn class_flags() {
    let path = std::env::temp_dir().join("windows_rdl_class_flags.winmd");

    windows_rdl::reader()
        .input_text(
            r#"
#[winrt]
mod Test {
    class Static {}

    class Instance {
        IInstance,
    }

    #[unsealed]
    class Base {
        IBase,
    }

    interface IInstance {}
    interface IBase {}
}
"#,
        )
        .output(&path)
        .write()
        .unwrap();

    let index = metadata::reader::Index::read(&path).unwrap();
    let flags = |name| {
        index.expect("Test", name).flags() & (TypeAttributes::Abstract | TypeAttributes::Sealed)
    };

    // Like MIDL, a class without interfaces has no instances, so it is abstract.
    assert_eq!(
        flags("Static"),
        TypeAttributes::Abstract | TypeAttributes::Sealed
    );
    assert_eq!(flags("Instance"), TypeAttributes::Sealed);
    assert_eq!(flags("Base"), TypeAttributes::default());
}
