use windows_metadata::reader::TypeIndex;
use windows_metadata::TypeAttributes;
use windows_rdl::*;

#[test]
pub fn test() {
    reader()
        .input_str(
            r#"
#[winrt]
mod Test {
    class Foo {
        IFoo,
    }
    #[Windows::Foundation::Metadata::ExclusiveTo(Foo)]
    interface IFoo {}
}
"#,
        )
        .input("../../../libs/bindgen/default/Windows.winmd")
        .output("tests/exclusive-to.winmd")
        .write()
        .unwrap();

    let index = TypeIndex::read("tests/exclusive-to.winmd").unwrap();

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
