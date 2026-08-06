use windows_metadata as metadata;

fn output(name: &str, extension: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("OUT_DIR")).join(format!("rdl_import_{name}.{extension}"))
}

fn compile(name: &str, source: &str) -> Result<metadata::reader::Index, windows_rdl::Error> {
    let path = output(name, "winmd");
    windows_rdl::reader()
        .input_text_named("imports.rdl", source)
        .output(&path)
        .write()?;
    Ok(metadata::reader::Index::read(&path).unwrap())
}

fn compile_with_defaults(
    name: &str,
    source: &str,
) -> Result<metadata::reader::Index, windows_rdl::Error> {
    let path = output(name, "winmd");
    windows_rdl::reader()
        .input_text_named("imports.rdl", source)
        .reference_default()
        .output(&path)
        .write()?;
    Ok(metadata::reader::Index::read(&path).unwrap())
}

#[test]
fn named_grouped_and_namespace_aliases_resolve() {
    let index = compile(
        "named",
        r#"
use Other::{self as Types, Point, Size as Extent, IVector as Vector};

#[winrt]
mod Test {
    struct Shape {
        point: Point,
        extent: Extent,
        other: Types::Point,
    }
    interface IUses {
        fn Use(&self, value: Vector<i32>);
    }
}

#[winrt]
mod Other {
    struct Point {
        x: i32,
        y: i32,
    }
    struct Size {
        width: i32,
        height: i32,
    }
    interface IVector<T> {
        fn Append(&self, value: T);
    }
}
"#,
    )
    .unwrap();

    let shape = index.expect("Test", "Shape");
    let fields: Vec<_> = shape.fields().map(|field| field.ty()).collect();
    assert_eq!(
        fields,
        [
            metadata::Type::value_named("Other", "Point"),
            metadata::Type::value_named("Other", "Size"),
            metadata::Type::value_named("Other", "Point"),
        ]
    );

    let uses = index.expect("Test", "IUses");
    let signature = uses.methods().next().unwrap().signature(&[]);
    let metadata::Type::ClassName(vector) = &signature.types[0] else {
        panic!()
    };
    assert_eq!(
        (&vector.namespace, &vector.name),
        (&"Other".into(), &"IVector`1".into())
    );
    assert_eq!(vector.generics, [metadata::Type::I32]);
}

#[test]
fn named_attribute_import_resolves_source_spelling() {
    let index = compile(
        "attribute",
        r#"
use Attributes::Marker;
use Attributes::Marker as Mark;
use Attributes as Metadata;

#[win32]
mod Test {
    #[Marker]
    struct Value {}
    #[Mark]
    struct Aliased {}
    #[Metadata::Marker]
    struct Qualified {}
}

#[win32]
mod Attributes {
    attribute MarkerAttribute {
        fn();
    }
}
"#,
    )
    .unwrap();

    assert!(metadata::HasAttributes::has_attribute(
        &index.expect("Test", "Value"),
        "MarkerAttribute"
    ));
    assert!(metadata::HasAttributes::has_attribute(
        &index.expect("Test", "Aliased"),
        "MarkerAttribute"
    ));
    assert!(metadata::HasAttributes::has_attribute(
        &index.expect("Test", "Qualified"),
        "MarkerAttribute"
    ));
}

#[test]
fn relative_attribute_paths_resolve() {
    let index = compile(
        "relative_attribute",
        r#"
#[win32]
mod Root {
    mod Attributes {
        attribute MarkerAttribute {
            fn();
        }
    }

    #[Attributes::Marker]
    struct Relative {}

    mod Api {
        #[super::Attributes::Marker]
        struct ParentRelative {}
    }
}
"#,
    )
    .unwrap();

    assert!(metadata::HasAttributes::has_attribute(
        &index.expect("Root", "Relative"),
        "MarkerAttribute"
    ));
    assert!(metadata::HasAttributes::has_attribute(
        &index.expect("Root.Api", "ParentRelative"),
        "MarkerAttribute"
    ));
}

#[test]
fn explicit_import_disambiguates_globs() {
    let index = compile(
        "explicit",
        r#"
use A::*;
use B::*;
use A::Value;

#[win32]
mod Test {
    struct Holder {
        value: Value,
    }
}

#[win32]
mod A {
    struct Value {
        a: i32,
    }
}

#[win32]
mod B {
    struct Value {
        b: i32,
    }
}
"#,
    )
    .unwrap();

    assert_eq!(
        index.expect("Test", "Holder").fields().next().unwrap().ty(),
        metadata::Type::value_named("A", "Value")
    );
}

#[test]
fn duplicate_identical_imports_are_accepted() {
    compile(
        "duplicate",
        r#"
use Other::Value;
use Other::Value;

#[win32]
mod Test {
    struct Holder {
        value: Value,
    }
}

#[win32]
mod Other {
    struct Value {}
}
"#,
    )
    .unwrap();
}

#[test]
fn local_type_takes_precedence_over_imports() {
    let index = compile(
        "local",
        r#"
use Other::Value;

#[win32]
mod Test {
    struct Value {}
    struct Holder {
        value: Value,
    }
}

#[win32]
mod Other {
    struct Value {}
}
"#,
    )
    .unwrap();

    assert_eq!(
        index.expect("Test", "Holder").fields().next().unwrap().ty(),
        metadata::Type::value_named("Test", "Value")
    );
}

#[test]
fn named_generic_import_resolves_from_reference_metadata() {
    let index = compile_with_defaults(
        "reference",
        r#"
use Windows::Foundation::Collections::IIterable as Iterable;

#[winrt]
mod Test {
    interface IUses {
        fn Use(&self, value: Iterable<i32>);
    }
}
"#,
    )
    .unwrap();

    let signature = index
        .expect("Test", "IUses")
        .methods()
        .next()
        .unwrap()
        .signature(&[]);
    let metadata::Type::ClassName(iterable) = &signature.types[0] else {
        panic!()
    };
    assert_eq!(iterable.namespace, "Windows.Foundation.Collections");
    assert_eq!(iterable.name, "IIterable`1");
    assert_eq!(iterable.generics, [metadata::Type::I32]);
}

#[test]
fn ambiguous_glob_type_reports_every_import() {
    let error = compile(
        "ambiguous_type",
        r#"
use A::*;
use B::*;

#[win32]
mod Test {
    struct Holder {
        value: Value,
    }
}

#[win32]
mod A {
    struct Value {}
}

#[win32]
mod B {
    struct Value {}
}
"#,
    )
    .err()
    .unwrap();

    assert_eq!(error.code.as_deref(), Some("RDL0004"));
    assert!(error.message.contains("type name `Value` is ambiguous"));
    assert_eq!(error.labels.len(), 3);
    assert!(
        error
            .help
            .iter()
            .any(|help| help.contains("qualified path"))
    );
}

#[test]
fn ambiguous_glob_attribute_reports_every_import() {
    let error = compile(
        "ambiguous_attribute",
        r#"
use A::*;
use B::*;

#[win32]
mod Test {
    #[Marker]
    struct Value {}
}

#[win32]
mod A {
    attribute MarkerAttribute {
        fn();
    }
}

#[win32]
mod B {
    attribute MarkerAttribute {
        fn();
    }
}
"#,
    )
    .err()
    .unwrap();

    assert_eq!(error.code.as_deref(), Some("RDL0004"));
    assert!(
        error
            .message
            .contains("attribute name `Marker` is ambiguous")
    );
    assert_eq!(error.labels.len(), 3);
}

#[test]
fn conflicting_and_unknown_imports_have_import_diagnostics() {
    let conflict = compile(
        "conflict",
        r#"
use A::Value as Item;
use B::Value as Item;

#[win32]
mod A {
    struct Value {}
}

#[win32]
mod B {
    struct Value {}
}
"#,
    )
    .err()
    .unwrap();
    assert_eq!(conflict.code.as_deref(), Some("RDL0004"));
    assert!(conflict.message.contains("defined more than once"));
    assert_eq!(conflict.labels.len(), 2);

    let unknown = compile(
        "unknown",
        r#"
use Missing::Value;

#[win32]
mod Test {
    struct Value {}
}
"#,
    )
    .err()
    .unwrap();
    assert_eq!(unknown.code.as_deref(), Some("RDL0003"));
    assert!(
        unknown
            .message
            .contains("import target `Missing.Value` not found")
    );
}

#[test]
fn writer_keeps_canonical_paths_instead_of_emitting_imports() {
    let winmd = output("writer", "winmd");
    let rdl = output("writer", "rdl");
    windows_rdl::reader()
        .input_text(
            r#"
use Other::Point;

#[win32]
mod Test {
    struct Holder {
        point: Point,
    }
}

#[win32]
mod Other {
    struct Point {}
}
"#,
        )
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .output(&rdl)
        .write()
        .unwrap();

    let source = std::fs::read_to_string(rdl).unwrap();
    assert!(!source.contains("use "));
    assert!(source.contains("point: Other::Point"), "{source}");
}
