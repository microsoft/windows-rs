use windows_metadata::TypeAttributes;
use windows_metadata::reader::TypeIndex;
use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/use-declarations.rdl")
        .output("tests/use-declarations.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/use-declarations.winmd")
        .output("tests/use-declarations-out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}

#[test]
pub fn parse_with_reference() {
    reader()
        .input("tests/use-declarations-ref.rdl")
        .input("../../../libs/bindgen/default/Windows.winmd")
        .output("tests/use-declarations-ref.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/use-declarations-ref.winmd")
        .output("tests/use-declarations-ref-out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}

// Verify that `#[ExclusiveTo(Foo)]` written with a `use` glob import correctly suppresses
// `TypeAttributes::Public` on the exclusive interface (while the class remains public).
#[test]
pub fn exclusive_to_with_use() {
    reader()
        .input_str(
            r#"
use Windows::Foundation::Metadata::*;

#[winrt]
mod Test {
    class Foo {}
    #[ExclusiveTo(Foo)]
    interface IFoo {}
}
"#,
        )
        .input("../../../libs/bindgen/default/Windows.winmd")
        .output("tests/exclusive-to-use.winmd")
        .write()
        .unwrap();

    let index = TypeIndex::read("tests/exclusive-to-use.winmd").expect("failed to read winmd");

    let foo = index.expect("Test", "Foo");
    assert!(
        foo.flags().contains(TypeAttributes::Public),
        "class Foo should be Public"
    );

    let ifoo = index.expect("Test", "IFoo");
    assert!(
        !ifoo.flags().contains(TypeAttributes::Public),
        "exclusive interface IFoo should not be Public"
    );
}
