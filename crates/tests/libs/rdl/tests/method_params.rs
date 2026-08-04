use std::path::{Path, PathBuf};
use windows_metadata as metadata;
use windows_metadata::HasAttributes;

const METADATA_NAMESPACE: &str = "Windows.Win32.Metadata";

struct AttributeSpec {
    name: String,
    values: Vec<(String, metadata::Value)>,
}

struct ParamSpec {
    name: String,
    sequence: u16,
    flags: metadata::ParamAttributes,
    attributes: Vec<AttributeSpec>,
}

fn marker(name: &str) -> AttributeSpec {
    AttributeSpec {
        name: name.to_string(),
        values: vec![],
    }
}

fn property(name: &str, property: &str, value: metadata::Value) -> AttributeSpec {
    AttributeSpec {
        name: name.to_string(),
        values: vec![(property.to_string(), value)],
    }
}

fn positional(name: &str, value: metadata::Value) -> AttributeSpec {
    AttributeSpec {
        name: name.to_string(),
        values: vec![(String::new(), value)],
    }
}

fn attribute_ctor(
    file: &mut metadata::writer::File,
    attribute: &AttributeSpec,
) -> metadata::writer::MemberRef {
    let parent = metadata::writer::MemberRefParent::TypeRef(
        file.TypeRef(METADATA_NAMESPACE, &attribute.name),
    );
    let signature = metadata::Signature {
        flags: metadata::MethodCallAttributes::HASTHIS,
        return_type: metadata::Type::Void,
        types: attribute
            .values
            .iter()
            .filter(|(name, _)| name.is_empty())
            .map(|(_, value)| value.ty())
            .collect(),
    };
    file.MemberRef(".ctor", &signature, parent)
}

fn write_method_winmd(
    path: &Path,
    return_type: metadata::Type,
    types: Vec<metadata::Type>,
    rows: &[ParamSpec],
) {
    let mut file = metadata::writer::File::new("test");
    file.TypeDef(
        "Test",
        "I",
        metadata::writer::TypeDefOrRef::default(),
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Interface
            | metadata::TypeAttributes::Abstract,
    );
    let signature = metadata::Signature {
        return_type,
        types,
        ..Default::default()
    };
    file.MethodDef(
        "Method",
        &signature,
        metadata::MethodAttributes::Public,
        Default::default(),
    );

    for row in rows {
        let param = file.Param(&row.name, row.sequence, row.flags);
        for attribute in &row.attributes {
            let ctor = attribute_ctor(&mut file, attribute);
            file.Attribute(
                metadata::writer::HasAttribute::Param(param),
                metadata::writer::AttributeType::MemberRef(ctor),
                &attribute.values,
            );
        }
    }

    std::fs::write(path, file.into_stream()).unwrap();
}

fn write_winmd(path: &Path, rows: &[(&str, u16, metadata::ParamAttributes, Option<&str>)]) {
    let rows: Vec<_> = rows
        .iter()
        .map(|(name, sequence, flags, attribute)| ParamSpec {
            name: (*name).to_string(),
            sequence: *sequence,
            flags: *flags,
            attributes: attribute.map(marker).into_iter().collect(),
        })
        .collect();

    write_method_winmd(
        path,
        metadata::Type::I32,
        vec![
            metadata::Type::I32,
            metadata::Type::PtrMut(Box::new(metadata::Type::I32), 1),
            metadata::Type::I64,
        ],
        &rows,
    );
}

fn define_attribute(
    file: &mut metadata::writer::File,
    name: &str,
    ctor_types: &[metadata::Type],
    properties: &[(&str, metadata::Type)],
) {
    let extends = file.TypeRef("System", "Attribute");
    file.TypeDef(
        METADATA_NAMESPACE,
        name,
        metadata::writer::TypeDefOrRef::TypeRef(extends),
        metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed,
    );
    let signature = metadata::Signature {
        flags: metadata::MethodCallAttributes::HASTHIS,
        return_type: metadata::Type::Void,
        types: ctor_types.to_vec(),
    };
    file.MethodDef(
        ".ctor",
        &signature,
        metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::SpecialName
            | metadata::MethodAttributes::RTSpecialName,
        Default::default(),
    );
    for (property, ty) in properties {
        file.Field(property, ty, metadata::FieldAttributes::Public);
    }
}

fn write_attribute_definitions(path: &Path) {
    let mut file = metadata::writer::File::new("attributes");
    define_attribute(&mut file, "RetValAttribute", &[], &[]);
    define_attribute(&mut file, "ComOutPtrAttribute", &[], &[]);
    define_attribute(
        &mut file,
        "NativeArrayInfoAttribute",
        &[],
        &[
            ("CountParamIndex", metadata::Type::I16),
            ("CountConst", metadata::Type::I32),
        ],
    );
    define_attribute(
        &mut file,
        "MemorySizeAttribute",
        &[],
        &[("BytesParamIndex", metadata::Type::I16)],
    );
    define_attribute(&mut file, "ReservedAttribute", &[], &[]);
    define_attribute(&mut file, "DoesNotReturnAttribute", &[], &[]);
    define_attribute(&mut file, "ScopedEnumAttribute", &[], &[]);
    define_attribute(
        &mut file,
        "NativeEncodingAttribute",
        &[metadata::Type::String],
        &[],
    );
    std::fs::write(path, file.into_stream()).unwrap();
}

fn scratch(name: &str) -> PathBuf {
    let path = Path::new(env!("OUT_DIR")).join(name);
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn assert_attribute(
    param: metadata::reader::MethodParam<'_>,
    name: &str,
    values: &[(String, metadata::Value)],
) {
    let attribute = param.find_attribute(name).unwrap();
    assert_eq!(attribute.value(), values);
}

#[test]
fn sparse_out_of_order_params_round_trip_with_flags_and_pseudos() {
    let scratch = scratch("method_params");
    let input = scratch.join("input.winmd");
    let rdl = scratch.join("output.rdl");
    let roundtrip = scratch.join("roundtrip.winmd");

    write_winmd(
        &input,
        &[
            (
                "third",
                3,
                metadata::ParamAttributes::Out,
                Some("RetValAttribute"),
            ),
            (
                "",
                0,
                metadata::ParamAttributes::default(),
                Some("DoesNotReturnAttribute"),
            ),
            (
                "first",
                1,
                metadata::ParamAttributes::In
                    | metadata::ParamAttributes::Out
                    | metadata::ParamAttributes::Optional,
                Some("ReservedAttribute"),
            ),
        ],
    );

    windows_rdl::writer()
        .input(input.to_str().unwrap())
        .output(rdl.to_str().unwrap())
        .write()
        .unwrap();

    let source = std::fs::read_to_string(&rdl).unwrap();
    let method = source.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(method.contains("#[reserved] #[in] #[out] #[opt] first: i32"));
    assert!(method.contains("p1: *mut i32"));
    assert!(!method.contains("#[in] p1"));
    assert!(method.contains("#[retval] #[out] third: i64"));
    assert!(method.contains("-> #[noreturn] i32"));

    windows_rdl::reader()
        .input(rdl.to_str().unwrap())
        .output(roundtrip.to_str().unwrap())
        .write()
        .unwrap();

    let index = metadata::reader::Index::read(&roundtrip).unwrap();
    let method = index.expect("Test", "I").methods().next().unwrap();
    let params = method.params_by_sequence(3).unwrap();

    let first = params.params()[0].unwrap();
    assert_eq!(first.name(), "first");
    assert!(first.flags().contains(metadata::ParamAttributes::In));
    assert!(first.flags().contains(metadata::ParamAttributes::Out));
    assert!(first.flags().contains(metadata::ParamAttributes::Optional));
    assert!(first.has_attribute("ReservedAttribute"));

    let second = params.params()[1].unwrap();
    assert_eq!(second.name(), "p1");
    assert_eq!(second.flags(), metadata::ParamAttributes::Out);

    let third = params.params()[2].unwrap();
    assert_eq!(third.name(), "third");
    assert_eq!(third.flags(), metadata::ParamAttributes::Out);
    assert!(third.has_attribute("RetValAttribute"));

    let return_param = params.return_param().unwrap();
    assert!(return_param.has_attribute("DoesNotReturnAttribute"));
}

#[test]
fn all_supported_param_attributes_and_directions_round_trip() {
    let scratch = scratch("all_method_params");
    let input = scratch.join("input.winmd");
    let attributes = scratch.join("attributes.winmd");
    let rdl = scratch.join("output.rdl");
    let roundtrip = scratch.join("roundtrip.winmd");

    let rows = vec![
        ParamSpec {
            name: "decorated".to_string(),
            sequence: 11,
            flags: metadata::ParamAttributes::In,
            attributes: vec![
                marker("ReservedAttribute"),
                marker("DoesNotReturnAttribute"),
                marker("ScopedEnumAttribute"),
                positional(
                    "NativeEncodingAttribute",
                    metadata::Value::Utf8("utf-16".to_string()),
                ),
            ],
        },
        ParamSpec {
            name: String::new(),
            sequence: 0,
            flags: metadata::ParamAttributes::default(),
            attributes: vec![
                marker("DoesNotReturnAttribute"),
                positional(
                    "NativeEncodingAttribute",
                    metadata::Value::Utf8("ansi".to_string()),
                ),
            ],
        },
        ParamSpec {
            name: "inout_optional".to_string(),
            sequence: 7,
            flags: metadata::ParamAttributes::In
                | metadata::ParamAttributes::Out
                | metadata::ParamAttributes::Optional,
            attributes: vec![marker("RetValAttribute"), marker("ComOutPtrAttribute")],
        },
        ParamSpec {
            name: "implicit_in".to_string(),
            sequence: 3,
            flags: metadata::ParamAttributes::In,
            attributes: vec![],
        },
        ParamSpec {
            name: "scalar_out".to_string(),
            sequence: 4,
            flags: metadata::ParamAttributes::Out,
            attributes: vec![],
        },
        ParamSpec {
            name: "mutable_in".to_string(),
            sequence: 5,
            flags: metadata::ParamAttributes::In,
            attributes: vec![],
        },
        ParamSpec {
            name: "constant_out".to_string(),
            sequence: 6,
            flags: metadata::ParamAttributes::Out,
            attributes: vec![],
        },
        ParamSpec {
            name: "counted".to_string(),
            sequence: 8,
            flags: metadata::ParamAttributes::In,
            attributes: vec![property(
                "NativeArrayInfoAttribute",
                "CountParamIndex",
                metadata::Value::I16(2),
            )],
        },
        ParamSpec {
            name: "fixed".to_string(),
            sequence: 9,
            flags: metadata::ParamAttributes::In,
            attributes: vec![property(
                "NativeArrayInfoAttribute",
                "CountConst",
                metadata::Value::I32(4),
            )],
        },
        ParamSpec {
            name: "bytes".to_string(),
            sequence: 10,
            flags: metadata::ParamAttributes::Out,
            attributes: vec![property(
                "MemorySizeAttribute",
                "BytesParamIndex",
                metadata::Value::I16(0),
            )],
        },
    ];

    write_method_winmd(
        &input,
        metadata::Type::I32,
        vec![
            metadata::Type::I32,
            metadata::Type::PtrMut(Box::new(metadata::Type::I32), 1),
            metadata::Type::I32,
            metadata::Type::I32,
            metadata::Type::PtrMut(Box::new(metadata::Type::I32), 1),
            metadata::Type::PtrConst(Box::new(metadata::Type::I32), 1),
            metadata::Type::PtrMut(Box::new(metadata::Type::I32), 2),
            metadata::Type::PtrConst(Box::new(metadata::Type::I32), 1),
            metadata::Type::PtrConst(Box::new(metadata::Type::I32), 1),
            metadata::Type::PtrMut(Box::new(metadata::Type::U8), 1),
            metadata::Type::I32,
        ],
        &rows,
    );
    write_attribute_definitions(&attributes);

    windows_rdl::writer()
        .input(input.to_str().unwrap())
        .output(rdl.to_str().unwrap())
        .write()
        .unwrap();

    let source = std::fs::read_to_string(&rdl).unwrap();
    let method = source.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(method.contains("p0: i32"));
    assert!(method.contains("p1: *mut i32"));
    assert!(!method.contains("#[in] p0"));
    assert!(!method.contains("#[out] p1"));
    assert!(method.contains("implicit_in: i32"));
    assert!(method.contains("#[out] scalar_out: i32"));
    assert!(method.contains("#[in] mutable_in: *mut i32"));
    assert!(method.contains("#[out] constant_out: *const i32"));
    assert!(
        method.contains("#[iid_is] #[retval] #[in] #[out] #[opt] inout_optional: *mut *mut i32")
    );
    assert!(method.contains("#[len_param(2)] counted: *const i32"));
    assert!(method.contains("#[len_const(4)] fixed: *const i32"));
    assert!(method.contains("#[size_param(0)] bytes: *mut u8"));
    assert!(
        method.contains("#[encoding(\"utf-16\")] #[noreturn] #[reserved] #[scoped] decorated: i32")
    );
    assert!(method.contains("-> #[encoding(\"ansi\")] #[noreturn] i32"));

    windows_rdl::reader()
        .input(rdl.to_str().unwrap())
        .reference(attributes.to_str().unwrap())
        .output(roundtrip.to_str().unwrap())
        .write()
        .unwrap();

    let index = metadata::reader::Index::read(&roundtrip).unwrap();
    let method = index.expect("Test", "I").methods().next().unwrap();
    let params = method.params_by_sequence(11).unwrap();

    let p0 = params.params()[0].unwrap();
    assert_eq!(p0.name(), "p0");
    assert_eq!(p0.flags(), metadata::ParamAttributes::In);

    let p1 = params.params()[1].unwrap();
    assert_eq!(p1.name(), "p1");
    assert_eq!(p1.flags(), metadata::ParamAttributes::Out);

    let implicit_in = params.params()[2].unwrap();
    assert_eq!(implicit_in.flags(), metadata::ParamAttributes::In);

    let scalar_out = params.params()[3].unwrap();
    assert_eq!(scalar_out.flags(), metadata::ParamAttributes::Out);

    let mutable_in = params.params()[4].unwrap();
    assert_eq!(mutable_in.flags(), metadata::ParamAttributes::In);

    let constant_out = params.params()[5].unwrap();
    assert_eq!(constant_out.flags(), metadata::ParamAttributes::Out);

    let inout_optional = params.params()[6].unwrap();
    assert_eq!(
        inout_optional.flags(),
        metadata::ParamAttributes::In
            | metadata::ParamAttributes::Out
            | metadata::ParamAttributes::Optional
    );
    assert_attribute(inout_optional, "RetValAttribute", &[]);
    assert_attribute(inout_optional, "ComOutPtrAttribute", &[]);

    let counted = params.params()[7].unwrap();
    assert_eq!(counted.flags(), metadata::ParamAttributes::In);
    assert_attribute(
        counted,
        "NativeArrayInfoAttribute",
        &[("CountParamIndex".to_string(), metadata::Value::I16(2))],
    );

    let fixed = params.params()[8].unwrap();
    assert_eq!(fixed.flags(), metadata::ParamAttributes::In);
    assert_attribute(
        fixed,
        "NativeArrayInfoAttribute",
        &[("CountConst".to_string(), metadata::Value::I32(4))],
    );

    let bytes = params.params()[9].unwrap();
    assert_eq!(bytes.flags(), metadata::ParamAttributes::Out);
    assert_attribute(
        bytes,
        "MemorySizeAttribute",
        &[("BytesParamIndex".to_string(), metadata::Value::I16(0))],
    );

    let decorated = params.params()[10].unwrap();
    assert_eq!(decorated.flags(), metadata::ParamAttributes::In);
    assert_attribute(decorated, "ReservedAttribute", &[]);
    assert_attribute(decorated, "DoesNotReturnAttribute", &[]);
    assert_attribute(decorated, "ScopedEnumAttribute", &[]);
    assert_attribute(
        decorated,
        "NativeEncodingAttribute",
        &[(String::new(), metadata::Value::Utf8("utf-16".to_string()))],
    );

    let return_param = params.return_param().unwrap();
    assert_eq!(return_param.flags(), metadata::ParamAttributes::default());
    assert_attribute(return_param, "DoesNotReturnAttribute", &[]);
    assert_attribute(
        return_param,
        "NativeEncodingAttribute",
        &[(String::new(), metadata::Value::Utf8("ansi".to_string()))],
    );
}

#[test]
fn production_parameter_directions_are_representable() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));

    for name in ["Windows.winmd", "Windows.Win32.winmd"] {
        let path = manifest.join("../../../libs/default").join(name);
        let index = metadata::reader::Index::read(&path).unwrap();

        for ty in index.types() {
            let generics: Vec<_> = ty
                .generic_params()
                .map(|generic| {
                    metadata::Type::Generic(generic.name().to_string(), generic.sequence())
                })
                .collect();
            for method in ty.methods() {
                let signature = method.signature(&generics);
                let params = method.params_by_sequence(signature.types.len()).unwrap();

                for param in params.params().iter().flatten() {
                    let flags = param.flags();
                    assert!(
                        flags.contains(metadata::ParamAttributes::In)
                            || flags.contains(metadata::ParamAttributes::Out),
                        "{name}: {}.{} parameter {} has neither In nor Out",
                        ty.namespace(),
                        method.name(),
                        param.sequence()
                    );
                }
            }
        }
    }
}

#[test]
fn malformed_param_sequence_is_reported() {
    let scratch = scratch("malformed_method_params");
    let input = scratch.join("input.winmd");
    let rdl = scratch.join("output.rdl");
    write_winmd(
        &input,
        &[
            ("first", 1, metadata::ParamAttributes::In, None),
            ("duplicate", 1, metadata::ParamAttributes::Out, None),
        ],
    );

    let error = windows_rdl::writer()
        .input(input.to_str().unwrap())
        .output(rdl.to_str().unwrap())
        .write()
        .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("method `Method` has invalid parameter metadata: duplicate Param.Sequence 1")
    );
}
