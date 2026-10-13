#![cfg(target_arch = "x86_64")]

use std::path::Path;
use windows_clang2::*;
use windows_metadata::{HasAttributes, Type, Value, reader::*};

mod support;

fn compiler() -> Compiler {
    Compiler::with_layout_bridge(
        Path::new(&std::env::var_os("WINDOWS_CLANG2_LAYOUT_NATIVE").unwrap())
            .join("windows_clang_layout.dll"),
    )
    .unwrap()
}

fn input(source: &str, target: Target, names: &[&str]) -> Input {
    let mut input = Input::new("interfaces.cpp", source, target);
    input
        .required
        .extend(names.iter().map(|name| name.to_string()));
    input
}

#[test]
fn reviewed_interfaces_preserve_native_identity_slots_and_signatures() {
    if support::worker("reviewed_interfaces_preserve_native_identity_slots_and_signatures") {
        return;
    }
    let compiler = compiler();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let snapshot = compiler
            .capture(&input(
                include_str!("..\\input\\interfaces.h"),
                target,
                &["Stream"],
            ))
            .unwrap();
        let rdl = snapshot.rdl("Test").unwrap();
        assert_eq!(
            rdl,
            include_str!("..\\input\\interfaces.rdl").replace("\r\n", "\n")
        );
        let root = snapshot
            .interfaces()
            .find(|(decl, _)| decl.qualified_name == "Root")
            .unwrap()
            .1;
        assert_eq!(root.guid, "00112233-4455-6677-8899-aabbccddeeff");
        assert_eq!(
            (root.vtable_size, root.inherited, root.packing),
            (3, 0, Some(8))
        );
        let stream = snapshot
            .interfaces()
            .find(|(decl, _)| decl.qualified_name == "Stream")
            .unwrap()
            .1;
        assert_eq!((stream.vtable_size, stream.inherited), (5, 3));
        let mut methods = snapshot
            .methods()
            .map(|(_, method, _)| (method.name.as_str(), method.slot))
            .collect::<Vec<_>>();
        methods.sort_by_key(|(_, slot)| *slot);
        assert_eq!(
            methods,
            [
                ("Query", 0),
                ("Retain", 1),
                ("Drop", 2),
                ("Read", 3),
                ("Write", 4)
            ]
        );
        let helpers = snapshot.interface_helpers().collect::<Vec<_>>();
        assert_eq!(helpers.len(), 1);
        assert_eq!(helpers[0].1.qualified_name, "Root::Query");
        assert_eq!(helpers[0].1.kind, "FunctionTemplate");

        let directory = Path::new(env!("OUT_DIR")).join(format!("interfaces_{target:?}"));
        std::fs::create_dir_all(&directory).unwrap();
        let vocabulary = directory.join("vocabulary.winmd");
        windows_rdl::reader()
            .input_text(include_str!("..\\..\\..\\..\\..\\metadata\\metadata.rdl"))
            .output(&vocabulary)
            .write()
            .unwrap();
        let binary = directory.join("interfaces.winmd");
        windows_rdl::reader()
            .reference(&vocabulary)
            .input_text(&rdl)
            .output(&binary)
            .write()
            .unwrap();
        let index = Index::read(&binary).unwrap();
        index.validate_native_contracts(None).unwrap();
        index.validate_native_slots(None).unwrap();
        let root = index.expect("Test", "Root");
        let stream = index.expect("Test", "Stream");
        assert_eq!(
            root.attributes()
                .find(|attribute| attribute.name() == "GuidAttribute")
                .unwrap()
                .value()
                .into_iter()
                .map(|(_, value)| value)
                .collect::<Vec<_>>(),
            [
                Value::U32(0x00112233),
                Value::U16(0x4455),
                Value::U16(0x6677),
                Value::U8(0x88),
                Value::U8(0x99),
                Value::U8(0xaa),
                Value::U8(0xbb),
                Value::U8(0xcc),
                Value::U8(0xdd),
                Value::U8(0xee),
                Value::U8(0xff),
            ]
        );
        let bases = stream.interface_impls().collect::<Vec<_>>();
        assert_eq!(bases.len(), 1);
        assert_eq!(bases[0].interface(&[]), Type::class_named("Test", "Root"));
        assert_eq!(root.native_vtable().unwrap().unwrap().size, 3);
        assert_eq!(stream.native_vtable().unwrap().unwrap().inherited, 3);
        assert_eq!(
            root.methods()
                .map(|method| method.native_slot().unwrap().unwrap().index)
                .collect::<Vec<_>>(),
            [0, 1, 2]
        );
        assert_eq!(
            stream
                .methods()
                .map(|method| method.native_slot().unwrap().unwrap().index)
                .collect::<Vec<_>>(),
            [3, 4]
        );
        let query = root.methods().next().unwrap().signature(&[]);
        assert_eq!(query.return_type, Type::value_named("Test", "Result"));
        assert_eq!(
            query.types,
            [
                Type::RefConst(Box::new(Type::value_named("Test", "Identifier"))),
                Type::PtrMut(Box::new(Type::Void), 2),
            ]
        );
        let text = directory.join("roundtrip.rdl");
        windows_rdl::writer()
            .input(&binary)
            .output(&text)
            .write()
            .unwrap();
        let roundtrip = directory.join("roundtrip.winmd");
        windows_rdl::reader()
            .reference(&vocabulary)
            .input(&text)
            .output(&roundtrip)
            .write()
            .unwrap();
        Index::read(&roundtrip)
            .unwrap()
            .validate_native_slots(None)
            .unwrap();
        assert_eq!(snapshot.clone().rdl("Test").unwrap(), rdl);
    }
    let retained = compiler
        .capture(&input(
            include_str!("..\\input\\interfaces.h"),
            Target::X64,
            &["Stream"],
        ))
        .unwrap();
    drop(compiler);
    assert_eq!(
        retained.rdl("Test").unwrap(),
        include_str!("..\\input\\interfaces.rdl").replace("\r\n", "\n")
    );
}

#[test]
fn unsupported_interface_behavior_is_not_a_new_slot_or_storage() {
    if support::worker("unsupported_interface_behavior_is_not_a_new_slot_or_storage") {
        return;
    }
    let compiler = compiler();
    let base = "struct __declspec(uuid(\"00112233-4455-6677-8899-aabbccddeeff\")) Base {
        virtual int __stdcall Method() = 0;
    };";
    for body in [
        "virtual int __stdcall Method() = 0;",
        "int field; virtual int __stdcall Other() = 0;",
        "virtual int __stdcall Other() { return 1; }",
        "virtual ~Rejected() {}",
        "virtual int __stdcall Other(int) = 0; virtual int __stdcall Other(float) = 0;",
        "virtual int __stdcall Other() const = 0;",
        "virtual int __stdcall Other() & = 0;",
        "virtual int __stdcall Other() noexcept = 0;",
    ] {
        let source = format!(
            "{base} struct __declspec(uuid(\"11223344-5566-7788-99aa-bbccddeeff00\")) Rejected : Base {{ {body} }};"
        );
        assert!(
            compiler
                .capture(&input(&source, Target::X64, &["Rejected"]))
                .is_err(),
            "{source}"
        );
    }
    for bases in ["private Base", "virtual Base", "Base, Other"] {
        let source = format!(
            "{base} struct Other {{ virtual int __stdcall More() = 0; }};
            struct __declspec(uuid(\"11223344-5566-7788-99aa-bbccddeeff00\")) Rejected : {bases} {{
                virtual int __stdcall Extra() = 0;
            }};"
        );
        assert!(
            compiler
                .capture(&input(&source, Target::X64, &["Rejected"]))
                .is_err(),
            "{source}"
        );
    }
    let source = "struct __declspec(uuid(\"00112233-4455-6677-8899-aabbccddeeff\")) Rejected {
        virtual int __cdecl Method() = 0;
    };";
    let snapshot = compiler
        .capture(&input(source, Target::X86, &["Rejected"]))
        .unwrap();
    assert_eq!(
        snapshot.rdl("Test").unwrap_err().stage,
        Stage::Representation
    );
}
