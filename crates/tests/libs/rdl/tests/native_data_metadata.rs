use std::path::{Path, PathBuf};
use windows_metadata::*;

fn path(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("native_data_metadata_{name}"))
}

fn attribute(
    file: &mut writer::File,
    owner: writer::HasAttribute,
    namespace: &str,
    name: &str,
    args: &[(String, Value)],
) {
    let reference = file.TypeRef(namespace, name);
    let signature = Signature {
        flags: MethodCallAttributes::HASTHIS,
        return_type: Type::Void,
        types: args
            .iter()
            .filter(|(name, _)| name.is_empty())
            .map(|(_, value)| value.ty())
            .collect(),
    };
    let ctor = file.MemberRef(
        ".ctor",
        &signature,
        writer::MemberRefParent::TypeRef(reference),
    );
    file.Attribute(owner, writer::AttributeType::MemberRef(ctor), args);
}

fn fixture(case: &str) -> writer::File {
    let mut file = writer::File::new("native_data_metadata");
    let base = file.TypeRef(
        "System",
        if case == "record" {
            "ValueType"
        } else {
            "Object"
        },
    );
    let flags = TypeAttributes::Public
        | TypeAttributes::Sealed
        | if case == "winrt" {
            TypeAttributes::WindowsRuntime
        } else {
            TypeAttributes::default()
        };
    let owner = file.TypeDef(
        if case == "empty_namespace" {
            ""
        } else {
            "Test"
        },
        if case == "record" { "Record" } else { "Apis" },
        writer::TypeDefOrRef::TypeRef(base),
        flags,
    );
    let ty = match case {
        "void" => Type::Void,
        "managed" => Type::String,
        _ => Type::U32,
    };
    let mut flags = FieldAttributes::Public | FieldAttributes::Static;
    match case {
        "literal" => flags |= FieldAttributes::Literal,
        "default_flag" => flags |= FieldAttributes::HasDefault,
        "storage_flag" => flags |= FieldAttributes(0x100),
        "instance" => flags = FieldAttributes::Public,
        _ => {}
    }
    let field = file.Field("Value", &ty, flags);
    if case == "container_arch" {
        attribute(
            &mut file,
            writer::HasAttribute::TypeDef(owner),
            "Windows.Win32.Metadata",
            "SupportedArchitectureAttribute",
            &[(String::new(), Value::I32(1))],
        );
    }
    if case == "value" {
        file.Constant(writer::HasConstant::Field(field), &Value::U32(0));
    }
    let parent = if case == "owner" {
        writer::HasAttribute::TypeDef(owner)
    } else {
        writer::HasAttribute::Field(field)
    };
    let dll = match case {
        "empty_dll" => "",
        "nul_dll" => "a\0.dll",
        _ => "a.dll",
    };
    let name = match case {
        "no_target" | "ordinal_negative" | "ordinal_large" => "",
        "nul_name" => "bad\0name",
        _ => "NativeValue",
    };
    let ordinal = match case {
        "both_targets" => 1,
        "ordinal_negative" => -2,
        "ordinal_large" => 65536,
        _ => -1,
    };
    let kind = match case {
        "code" => 0,
        "kind_unknown" => 3,
        _ => 1,
    };
    let mut args: Vec<_> = [
        Value::Utf8(dll.into()),
        Value::Utf8(name.into()),
        Value::I32(ordinal),
        Value::I32(kind),
    ]
    .into_iter()
    .map(|value| (String::new(), value))
    .collect();
    if case == "named_args" {
        args.push(("Extra".into(), Value::I32(0)));
    }
    if case == "argument_type" {
        args[2].1 = Value::U32(17);
    }
    if case == "argument_count" {
        args.pop();
    }
    let namespace = if case == "foreign" {
        "Other"
    } else {
        "Windows.Win32.Metadata"
    };
    if case != "unmarked" {
        attribute(
            &mut file,
            parent,
            namespace,
            "NativeDataImportAttribute",
            &args,
        );
    }
    if case == "duplicate" {
        attribute(
            &mut file,
            parent,
            namespace,
            "NativeDataImportAttribute",
            &args,
        );
    }
    file
}

#[test]
fn invalid_raw_imports_fail_before_emission_rendering_or_cleanup() {
    for (case, diagnostic) in [
        ("literal", "without a value or storage flags"),
        ("default_flag", "without a value or storage flags"),
        ("storage_flag", "without a value or storage flags"),
        ("instance", "without a value or storage flags"),
        ("value", "without a value or storage flags"),
        ("owner", "requires a field"),
        ("record", "native Apis"),
        ("winrt", "native Apis"),
        ("empty_namespace", "native Apis"),
        ("container_arch", "native Apis"),
        ("void", "nonvoid"),
        ("managed", "nonvoid"),
        ("empty_dll", "nonempty DLL"),
        ("nul_dll", "NUL-free"),
        ("nul_name", "NUL-free"),
        ("no_target", "fit u16"),
        ("ordinal_negative", "fit u16"),
        ("ordinal_large", "fit u16"),
        ("both_targets", "exactly one"),
        ("code", "data or constant"),
        ("kind_unknown", "data or constant"),
        ("named_args", "positional"),
        ("argument_type", "invalid native data import arguments"),
        ("argument_count", "invalid native data import arguments"),
        ("duplicate", "duplicate"),
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
                [(String::from("Value"), String::from("value"))]
                    .into_iter()
                    .collect(),
            )
            .write()
            .unwrap_err();
        assert_eq!(std::fs::read(&sentinel).unwrap(), b"last good output");
        for remap in [false, true] {
            let output = path(&format!("{case}_{remap}.winmd"));
            std::fs::write(&output, b"last good output").unwrap();
            if remap {
                merge::Remapper::new()
                    .input(&input)
                    .source("Test")
                    .fallback("Routed")
                    .output(&output)
                    .remap()
                    .unwrap_err();
            } else {
                merge().input(&input).output(&output).merge().unwrap_err();
            }
            assert_eq!(std::fs::read(output).unwrap(), b"last good output");
        }
    }
}

#[test]
fn explicit_descriptors_do_not_reclassify_unmarked_or_foreign_fields() {
    for case in ["valid", "unmarked", "foreign"] {
        let index = reader::Index::new(vec![
            reader::File::new(fixture(case).try_into_stream().unwrap()).unwrap(),
        ]);
        let item = index.expect_item("Test", "Value");
        let field = match item {
            reader::Item::Data(field) if case == "valid" => field,
            reader::Item::Const(field) if case != "valid" => field,
            _ => panic!("unexpected classification for {case}"),
        };
        assert_eq!(field.data_import().unwrap().is_some(), case == "valid");
        assert_eq!(
            field.flags(),
            FieldAttributes::Public | FieldAttributes::Static
        );
        assert!(field.constant().is_none());
        let mut blob = field.blob(2);
        assert_eq!(blob.read_u8(), 0x06);
        assert_eq!(blob.read_u8(), 0x09);
        assert!(blob.is_empty());
    }
}
