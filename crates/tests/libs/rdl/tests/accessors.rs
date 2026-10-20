use windows_metadata as metadata;

// Explicit `#[special]` accessors and the property/event shorthand must produce the same
// Property/Event rows; the shorthand only changes how the RDL is spelled.
#[test]
fn accessors() {
    let path = std::env::temp_dir().join("windows_rdl_accessors.winmd");

    windows_rdl::reader()
        .input("input/member_attributes.rdl")
        .output(&path)
        .write()
        .unwrap();

    let index = metadata::reader::Index::read(&path).unwrap();
    let ty = index.expect("Test", "IInterface");

    let properties: Vec<_> = ty
        .properties()
        .map(|property| {
            (
                property.name(),
                name(property.getter()),
                name(property.setter()),
            )
        })
        .collect();

    assert_eq!(
        properties,
        [
            ("Both", Some("get_Both"), Some("put_Both")),
            ("Getter", Some("get_Getter"), None),
            ("Setter", None, Some("put_Setter")),
            ("Mismatched", Some("get_Mismatched"), Some("put_Mismatched")),
            ("Annotated", Some("get_Annotated"), Some("put_Annotated")),
        ]
    );

    let events: Vec<_> = ty
        .events()
        .map(|event| (event.name(), name(event.adder()), name(event.remover())))
        .collect();

    assert_eq!(
        events,
        [("Changed", Some("add_Changed"), Some("remove_Changed"))]
    );
}

// Explicit accessors are grouped regardless of declaration order, a `remove` without an `add`
// has no event to describe, and Win32 accessors stay ordinary methods.
#[test]
fn explicit_accessors() {
    let path = std::env::temp_dir().join("windows_rdl_explicit_accessors.winmd");

    windows_rdl::reader()
        .input_text(
            r#"
#[winrt]
mod Windows {
    mod Foundation {
        struct EventRegistrationToken {
            Value: i64,
        }
    }
}
#[winrt]
mod Test {
    delegate fn Handler();
    interface IInterface {
        #[special]
        fn remove_Reversed(&self, token: Windows::Foundation::EventRegistrationToken);
        #[special]
        fn add_Reversed(&self, handler: Handler) -> Windows::Foundation::EventRegistrationToken;
        #[special]
        fn remove_Orphan(&self, token: Windows::Foundation::EventRegistrationToken);
    }
}
#[win32]
mod Native {
    #[no_guid]
    interface IInterface {
        #[special]
        fn get_Value(&self) -> i32;
    }
}
"#,
        )
        .output(&path)
        .write()
        .unwrap();

    let index = metadata::reader::Index::read(&path).unwrap();

    let events: Vec<_> = index
        .expect("Test", "IInterface")
        .events()
        .map(|event| (event.name(), name(event.adder()), name(event.remover())))
        .collect();

    assert_eq!(
        events,
        [("Reversed", Some("add_Reversed"), Some("remove_Reversed"))]
    );

    assert_eq!(index.expect("Native", "IInterface").properties().len(), 0);
}

fn name(method: Option<metadata::reader::MethodDef<'_>>) -> Option<&str> {
    method.map(|method| method.name())
}
