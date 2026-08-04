use windows_metadata::*;

fn index(rows: &[(&str, u16, ParamAttributes)], parameter_count: usize) -> reader::Index {
    let mut file = writer::File::new("test");

    file.TypeDef(
        "Test",
        "I",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );

    let signature = Signature {
        return_type: Type::I32,
        types: vec![Type::I32; parameter_count],
        ..Default::default()
    };
    file.MethodDef(
        "Method",
        &signature,
        MethodAttributes::Public,
        Default::default(),
    );
    for (name, sequence, flags) in rows {
        file.Param(name, *sequence, *flags);
    }

    reader::Index::new(vec![reader::File::new(file.into_stream()).unwrap()])
}

fn index_with_markers(rows: &[(&str, u16, ParamAttributes, &[&str])]) -> reader::Index {
    let mut file = writer::File::new("test");

    file.TypeDef(
        "Test",
        "I",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );

    let signature = Signature {
        return_type: Type::Void,
        types: vec![Type::I32; rows.len()],
        ..Default::default()
    };
    file.MethodDef(
        "Method",
        &signature,
        MethodAttributes::Public,
        Default::default(),
    );

    for (name, sequence, flags, attributes) in rows {
        let param = file.Param(name, *sequence, *flags);
        for attribute in *attributes {
            let parent =
                writer::MemberRefParent::TypeRef(file.TypeRef("Windows.Win32.Metadata", attribute));
            let signature = Signature {
                flags: MethodCallAttributes::HASTHIS,
                return_type: Type::Void,
                ..Default::default()
            };
            let ctor = file.MemberRef(".ctor", &signature, parent);
            file.Attribute(
                writer::HasAttribute::Param(param),
                writer::AttributeType::MemberRef(ctor),
                &[],
            );
        }
    }

    reader::Index::new(vec![reader::File::new(file.into_stream()).unwrap()])
}

fn index_with_buffer_relationships(
    rows: &[(&str, &[(&str, &[(String, Value)])])],
) -> reader::Index {
    let mut file = writer::File::new("test");

    file.TypeDef(
        "Test",
        "I",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );

    let signature = Signature {
        return_type: Type::Void,
        types: vec![Type::I32; rows.len()],
        ..Default::default()
    };
    file.MethodDef(
        "Method",
        &signature,
        MethodAttributes::Public,
        Default::default(),
    );

    for (position, (name, attributes)) in rows.iter().enumerate() {
        let param = file.Param(name, position as u16 + 1, ParamAttributes::In);
        for (attribute, values) in *attributes {
            let parent =
                writer::MemberRefParent::TypeRef(file.TypeRef("Windows.Win32.Metadata", attribute));
            let signature = Signature {
                flags: MethodCallAttributes::HASTHIS,
                return_type: Type::Void,
                ..Default::default()
            };
            let ctor = file.MemberRef(".ctor", &signature, parent);
            file.Attribute(
                writer::HasAttribute::Param(param),
                writer::AttributeType::MemberRef(ctor),
                values,
            );
        }
    }

    reader::Index::new(vec![reader::File::new(file.into_stream()).unwrap()])
}

fn method(index: &reader::Index) -> reader::MethodDef<'_> {
    index.expect("Test", "I").methods().next().unwrap()
}

fn names(params: &[Option<reader::MethodParam<'_>>]) -> Vec<Option<String>> {
    params
        .iter()
        .map(|param| param.map(|param| param.name().to_string()))
        .collect()
}

#[test]
fn parameter_facts_remain_independent() {
    let index = index_with_markers(&[
        ("unspecified", 1, ParamAttributes::default(), &[]),
        (
            "input",
            2,
            ParamAttributes::In | ParamAttributes::Optional,
            &["NativeArrayInfoAttribute"],
        ),
        ("output", 3, ParamAttributes::Out, &["ReservedAttribute"]),
        (
            "input_output",
            4,
            ParamAttributes::In | ParamAttributes::Out,
            &["RetValAttribute", "MemorySizeAttribute"],
        ),
    ]);
    let params = method(&index).params_by_sequence(4).unwrap();
    let [
        Some(unspecified),
        Some(input),
        Some(output),
        Some(input_output),
    ] = params.params()
    else {
        panic!()
    };

    assert_eq!(unspecified.direction(), reader::ParamDirection::Unspecified);
    assert!(!unspecified.is_optional());
    assert!(!unspecified.is_reserved());
    assert!(!unspecified.is_retval_attribute());

    assert_eq!(input.direction(), reader::ParamDirection::Input);
    assert!(input.is_optional());
    assert!(!input.is_reserved());
    assert!(!input.is_retval_attribute());
    assert!(input.has_attribute("NativeArrayInfoAttribute"));

    assert_eq!(output.direction(), reader::ParamDirection::Output);
    assert!(!output.is_optional());
    assert!(output.is_reserved());
    assert!(!output.is_retval_attribute());

    assert_eq!(
        input_output.direction(),
        reader::ParamDirection::InputOutput
    );
    assert!(!input_output.is_optional());
    assert!(!input_output.is_reserved());
    assert!(input_output.is_retval_attribute());
    assert!(input_output.has_attribute("MemorySizeAttribute"));
}

#[test]
fn buffer_relationships_preserve_raw_signed_values() {
    let elements_param = vec![("CountParamIndex".to_string(), Value::I16(-1))];
    let bytes_param = vec![("BytesParamIndex".to_string(), Value::I16(2))];
    let elements_const = vec![("CountConst".to_string(), Value::I32(16))];
    let index = index_with_buffer_relationships(&[
        (
            "elements_param",
            &[("NativeArrayInfoAttribute", &elements_param)],
        ),
        ("bytes_param", &[("MemorySizeAttribute", &bytes_param)]),
        (
            "elements_const",
            &[("NativeArrayInfoAttribute", &elements_const)],
        ),
    ]);
    let params = method(&index).params_by_sequence(3).unwrap();

    assert_eq!(
        params.params()[0].unwrap().buffer_relationship(),
        Some(reader::BufferRelationship::ElementsParam(-1))
    );
    assert_eq!(
        params.params()[1].unwrap().buffer_relationship(),
        Some(reader::BufferRelationship::BytesParam(2))
    );
    assert_eq!(
        params.params()[2].unwrap().buffer_relationship(),
        Some(reader::BufferRelationship::ElementsConst(16))
    );
}

#[test]
fn malformed_buffer_relationships_are_ignored() {
    let invalid = vec![("CountParamIndex".to_string(), Value::I32(1))];
    let conflicting = vec![
        ("CountParamIndex".to_string(), Value::I16(1)),
        ("CountConst".to_string(), Value::I32(4)),
    ];
    let ignored = vec![("Unrelated".to_string(), Value::I16(3))];
    let index = index_with_buffer_relationships(&[
        ("invalid", &[("NativeArrayInfoAttribute", &invalid)]),
        ("conflicting", &[("NativeArrayInfoAttribute", &conflicting)]),
        ("ignored", &[("NativeArrayInfoAttribute", &ignored)]),
    ]);
    let params = method(&index).params_by_sequence(3).unwrap();

    assert_eq!(params.params()[0].unwrap().buffer_relationship(), None);
    assert_eq!(params.params()[1].unwrap().buffer_relationship(), None);
    assert_eq!(params.params()[2].unwrap().buffer_relationship(), None);
}

#[test]
fn dense_rows_follow_sequence() {
    let index = index(
        &[
            ("one", 1, ParamAttributes::In),
            ("two", 2, ParamAttributes::Out),
        ],
        2,
    );
    let params = method(&index).params_by_sequence(2).unwrap();

    assert_eq!(
        names(params.params()),
        [Some("one".to_string()), Some("two".to_string())]
    );
    assert!(params.return_param().is_none());
}

#[test]
fn absent_rows_leave_every_position_empty() {
    let index = index(&[], 2);
    let params = method(&index).params_by_sequence(2).unwrap();

    assert_eq!(params.params(), [None, None]);
    assert!(params.return_param().is_none());
}

#[test]
fn return_row_is_separate_from_parameters() {
    let index = index(
        &[
            ("return", 0, ParamAttributes::Out),
            ("value", 1, ParamAttributes::In),
        ],
        1,
    );
    let params = method(&index).params_by_sequence(1).unwrap();

    assert_eq!(params.return_param().unwrap().name(), "return");
    assert_eq!(names(params.params()), [Some("value".to_string())]);
}

#[test]
fn sparse_rows_leave_only_missing_positions_empty() {
    let index = index(
        &[
            ("one", 1, ParamAttributes::In),
            ("three", 3, ParamAttributes::Optional),
        ],
        3,
    );
    let params = method(&index).params_by_sequence(3).unwrap();

    assert_eq!(
        names(params.params()),
        [Some("one".to_string()), None, Some("three".to_string())]
    );
}

#[test]
fn out_of_order_rows_follow_sequence_without_changing_physical_iteration() {
    let index = index(
        &[
            ("three", 3, ParamAttributes::Optional),
            ("one", 1, ParamAttributes::In),
            ("two", 2, ParamAttributes::Out),
        ],
        3,
    );
    let method = method(&index);
    let params = method.params_by_sequence(3).unwrap();

    assert_eq!(
        method
            .params()
            .map(|param| param.sequence())
            .collect::<Vec<_>>(),
        [3, 1, 2]
    );
    assert_eq!(
        names(params.params()),
        [
            Some("one".to_string()),
            Some("two".to_string()),
            Some("three".to_string())
        ]
    );
}

#[test]
fn duplicate_sequence_is_an_error() {
    let index = index(
        &[
            ("first", 1, ParamAttributes::In),
            ("second", 1, ParamAttributes::Out),
        ],
        1,
    );

    let error = method(&index).params_by_sequence(1).unwrap_err();
    assert_eq!(
        error,
        reader::MethodParamSequenceError::DuplicateSequence { sequence: 1 }
    );
    assert_eq!(error.to_string(), "duplicate Param.Sequence 1");
}

#[test]
fn out_of_range_sequence_is_an_error() {
    let index = index(&[("three", 3, ParamAttributes::In)], 2);

    let error = method(&index).params_by_sequence(2).unwrap_err();
    assert_eq!(
        error,
        reader::MethodParamSequenceError::SequenceOutOfRange {
            sequence: 3,
            parameter_count: 2,
        }
    );
    assert_eq!(
        error.to_string(),
        "Param.Sequence 3 is out of range for 2 signature parameters"
    );
}
