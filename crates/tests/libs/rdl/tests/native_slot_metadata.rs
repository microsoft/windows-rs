use std::path::{Path, PathBuf};
use windows_metadata::*;

fn path(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("native_slot_metadata_{name}"))
}

fn attribute(file: &mut writer::File, owner: writer::HasAttribute, name: &str, args: Vec<Value>) {
    let ty = file.TypeRef("Windows.Win32.Metadata", name);
    let signature = Signature {
        flags: MethodCallAttributes::HASTHIS,
        return_type: Type::Void,
        types: args.iter().map(Value::ty).collect(),
    };
    let ctor = file.MemberRef(".ctor", &signature, writer::MemberRefParent::TypeRef(ty));
    file.Attribute(
        owner,
        writer::AttributeType::MemberRef(ctor),
        &args
            .into_iter()
            .map(|value| (String::new(), value))
            .collect::<Vec<_>>(),
    );
}

fn fixture(case: &str) -> writer::File {
    let mut file = writer::File::new("native_slot_metadata");
    let flags = TypeAttributes::Public | TypeAttributes::Abstract | TypeAttributes::Interface;
    let root = file.TypeDef("Test", "Root", Default::default(), flags);
    attribute(
        &mut file,
        writer::HasAttribute::TypeDef(root),
        "NativeVtableAttribute",
        vec![Value::I32(1), Value::I32(0)],
    );
    let signature = Signature {
        flags: MethodCallAttributes::HASTHIS,
        return_type: Type::Void,
        types: vec![],
    };
    let method_flags = MethodAttributes::Public
        | MethodAttributes::Abstract
        | MethodAttributes::Virtual
        | MethodAttributes::HideBySig;
    let root_method = file.MethodDef(
        "A",
        &signature,
        method_flags | MethodAttributes::NewSlot,
        Default::default(),
    );
    attribute(
        &mut file,
        writer::HasAttribute::MethodDef(root_method),
        "NativeSlotAttribute",
        vec![Value::I32(0)],
    );
    let owner_name = if case == "missing_declaration" {
        let middle = file.TypeDef("Test", "Middle", Default::default(), flags);
        file.InterfaceImpl(middle, &Type::class_named("Test", "Root"));
        attribute(
            &mut file,
            writer::HasAttribute::TypeDef(middle),
            "NativeVtableAttribute",
            vec![Value::I32(1), Value::I32(1)],
        );
        "Middle"
    } else {
        "Root"
    };
    let mut flags = flags;
    if case == "winrt" {
        flags |= TypeAttributes::WindowsRuntime;
    }
    let interface = file.TypeDef("Test", "Derived", Default::default(), flags);
    file.InterfaceImpl(interface, &Type::class_named("Test", owner_name));
    if case != "missing_table" {
        let size = if case == "negative_size" { -1 } else { 2 };
        let inherited = if case == "negative_prefix" {
            -1
        } else if case == "base_count" {
            0
        } else {
            1
        };
        attribute(
            &mut file,
            writer::HasAttribute::TypeDef(interface),
            "NativeVtableAttribute",
            vec![Value::I32(size), Value::I32(inherited)],
        );
        if case == "duplicate_table" {
            attribute(
                &mut file,
                writer::HasAttribute::TypeDef(interface),
                "NativeVtableAttribute",
                vec![Value::I32(2), Value::I32(1)],
            );
        }
    }
    let new = file.MethodDef(
        "B",
        &signature,
        method_flags
            | if case == "new_flag" {
                MethodAttributes::default()
            } else {
                MethodAttributes::NewSlot
            },
        Default::default(),
    );
    let new_owner = if case == "descriptor_owner" {
        writer::HasAttribute::TypeDef(interface)
    } else {
        writer::HasAttribute::MethodDef(new)
    };
    if case != "missing_slot" {
        let index = match case {
            "negative_slot" => -1,
            "bounds" => 2,
            "new_prefix" => 0,
            _ => 1,
        };
        let value = if case == "argument_type" {
            Value::U32(1)
        } else {
            Value::I32(index)
        };
        attribute(&mut file, new_owner, "NativeSlotAttribute", vec![value]);
        if case == "duplicate_slot" {
            attribute(
                &mut file,
                new_owner,
                "NativeSlotAttribute",
                vec![Value::I32(1)],
            );
        }
    }
    if case == "duplicate_index" {
        let extra = file.MethodDef(
            "C",
            &signature,
            method_flags | MethodAttributes::NewSlot,
            Default::default(),
        );
        attribute(
            &mut file,
            writer::HasAttribute::MethodDef(extra),
            "NativeSlotAttribute",
            vec![Value::I32(1)],
        );
    }
    let reuse = file.MethodDef(
        "A",
        &signature,
        method_flags
            | if case == "reuse_flag" {
                MethodAttributes::NewSlot
            } else {
                MethodAttributes::default()
            },
        Default::default(),
    );
    let target = writer::HasAttribute::MethodDef(reuse);
    if case != "orphan_reuse" {
        attribute(
            &mut file,
            target,
            "NativeSlotAttribute",
            vec![Value::I32(if case == "reuse_suffix" { 1 } else { 0 })],
        );
    }
    attribute(
        &mut file,
        target,
        "NativeSlotReuseAttribute",
        vec![if case == "reuse_type" {
            Value::Utf8("Root".into())
        } else {
            Value::TypeName(TypeName::named(
                "Test",
                if case == "nonancestor" {
                    "Derived"
                } else {
                    owner_name
                },
            ))
        }],
    );
    file
}

#[test]
fn malformed_native_slots_reject_before_emission_and_output_cleanup() {
    for (case, diagnostic) in [
        ("missing_table", "requires a vtable"),
        ("missing_slot", "missing its slot"),
        ("negative_size", "inherited <= size"),
        ("negative_prefix", "inherited <= size"),
        ("base_count", "disagrees"),
        ("descriptor_owner", "invalid owner"),
        ("winrt", "native interfaces"),
        ("negative_slot", "nonnegative"),
        ("bounds", "within the vtable"),
        ("new_prefix", "inherited prefix"),
        ("argument_type", "invalid native slot arguments"),
        ("new_flag", "flags disagree"),
        ("reuse_flag", "flags disagree"),
        ("reuse_suffix", "unique"),
        ("duplicate_table", "duplicate"),
        ("duplicate_slot", "duplicate"),
        ("duplicate_index", "unique"),
        ("orphan_reuse", "requires a slot"),
        ("reuse_type", "type-valued owner"),
        ("nonancestor", "ancestor"),
        ("missing_declaration", "no declaration"),
    ] {
        let error = fixture(case).try_into_stream().unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{case}: {error}");
        let input = path(&format!("{case}.winmd"));
        std::fs::write(&input, fixture(case).into_stream()).unwrap();
        let rendered = path(&format!("{case}.rdl"));
        std::fs::write(&rendered, b"last good output").unwrap();
        let error = windows_rdl::writer()
            .input(&input)
            .output(&rendered)
            .write()
            .unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{case}: {error}");
        assert_eq!(std::fs::read(&rendered).unwrap(), b"last good output");
        let split = path(&format!("{case}_split"));
        std::fs::create_dir_all(&split).unwrap();
        let sentinel = split.join("keep.rdl");
        std::fs::write(&sentinel, b"last good output").unwrap();
        windows_rdl::writer()
            .input(&input)
            .output(&split)
            .split()
            .write()
            .unwrap_err();
        assert_eq!(std::fs::read(&sentinel).unwrap(), b"last good output");
        windows_rdl::writer()
            .input(&input)
            .output(&split)
            .partition(
                [(String::from("Derived"), String::from("derived"))]
                    .into_iter()
                    .collect(),
            )
            .write()
            .unwrap_err();
        assert_eq!(std::fs::read(&sentinel).unwrap(), b"last good output");
        let output = path(&format!("{case}_merged.winmd"));
        std::fs::write(&output, b"last good output").unwrap();
        merge().input(&input).output(&output).merge().unwrap_err();
        assert_eq!(std::fs::read(&output).unwrap(), b"last good output");
        merge::Remapper::new()
            .input(&input)
            .source("Test")
            .fallback("Routed")
            .output(&output)
            .remap()
            .unwrap_err();
        assert_eq!(std::fs::read(output).unwrap(), b"last good output");
    }
}

#[test]
fn slot_descriptors_do_not_add_method_rows_or_change_signatures() {
    let index = reader::Index::new(vec![
        reader::File::new(fixture("valid").try_into_stream().unwrap()).unwrap(),
    ]);
    let interface = index.expect("Test", "Derived");
    let methods: Vec<_> = interface.methods().collect();
    assert_eq!(methods.len(), 2);
    for method in &methods {
        let mut blob = method.blob(4);
        assert_eq!(blob.read_u8(), 0x20);
        assert_eq!(blob.read_u8(), 0);
        assert_eq!(blob.read_u8(), 0x01);
        assert!(blob.is_empty());
    }
    assert!(methods[0].flags().contains(MethodAttributes::NewSlot));
    assert!(!methods[1].flags().contains(MethodAttributes::NewSlot));
}
