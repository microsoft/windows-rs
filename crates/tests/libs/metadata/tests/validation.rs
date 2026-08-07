use windows_metadata::writer::RowHandle;
use windows_metadata::*;

fn index(file: writer::File) -> reader::Index {
    file.into_index()
}

#[test]
fn writer_handles_match_finalized_row_ids() {
    let mut file = writer::File::new("test");
    let handle = file.TypeDef(
        "Test",
        "Value",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::ExplicitLayout,
    );
    let field = file.Field("Value", &Type::I32, FieldAttributes::Public);
    let class_layout = file.ClassLayout(handle, 4, 4);
    let field_layout = file.FieldLayout(field, 0);
    let expected = handle.row_id(0);
    let expected_class_layout = class_layout.row_id(0);
    let expected_field_layout = field_layout.row_id(0);
    let index = index(file);
    let actual = index.expect("Test", "Value").row_id();
    let actual_class_layout = index
        .expect("Test", "Value")
        .class_layout()
        .unwrap()
        .row_id();
    let actual_field_layout = index
        .expect("Test", "Value")
        .fields()
        .next()
        .unwrap()
        .layout()
        .unwrap()
        .row_id();

    assert_eq!(actual, expected);
    assert_eq!(actual_class_layout, expected_class_layout);
    assert_eq!(actual_field_layout, expected_field_layout);
    assert_eq!(actual.table(), reader::TableId::TypeDef);
    assert_eq!(actual.table() as u8, 0x02);
}

#[test]
fn duplicate_type_identity_is_rejected() {
    let mut file = writer::File::new("test");
    let extends = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));

    file.TypeDef(
        "Test",
        "Value",
        extends,
        TypeAttributes::Public | TypeAttributes::Sealed,
    );
    file.TypeDef(
        "Test",
        "Value",
        extends,
        TypeAttributes::Public | TypeAttributes::Sealed,
    );

    let errors = validator::validate(&index(file));
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), "duplicate type `Test.Value`");
    assert!(errors[0].related().is_some());
}

#[test]
fn duplicate_field_and_method_identities_are_rejected() {
    let mut file = writer::File::new("test");
    file.TypeDef(
        "Test",
        "IValue",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );

    file.Field("Value", &Type::I32, FieldAttributes::Public);
    file.Field("Value", &Type::U32, FieldAttributes::Public);

    let signature = Signature {
        return_type: Type::Void,
        types: vec![Type::I32],
        ..Default::default()
    };
    file.MethodDef(
        "Get",
        &signature,
        MethodAttributes::Public,
        Default::default(),
    );
    file.Param("value", 1, ParamAttributes::In);
    file.MethodDef(
        "Get",
        &signature,
        MethodAttributes::Public,
        Default::default(),
    );
    file.Param("value", 1, ParamAttributes::In);

    let errors = validator::validate(&index(file));
    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0].message(), "duplicate field `Value`");
    assert_eq!(
        errors[1].message(),
        "duplicate method `Get` on `Test.IValue`"
    );
}

#[test]
fn malformed_parameter_associations_are_rejected() {
    let mut file = writer::File::new("test");
    file.TypeDef(
        "Test",
        "IValue",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );

    let signature = Signature {
        return_type: Type::Void,
        types: vec![Type::I32],
        ..Default::default()
    };
    file.MethodDef(
        "Get",
        &signature,
        MethodAttributes::Public,
        Default::default(),
    );
    file.Param("first", 1, ParamAttributes::In);
    file.Param("duplicate", 1, ParamAttributes::In);

    let errors = validator::validate(&index(file));
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        "invalid parameters for `Test.IValue` method `Get`: duplicate Param.Sequence 1"
    );
    assert!(errors[0].related().is_none());
}

#[test]
fn duplicate_properties_events_and_semantics_are_rejected() {
    let mut file = writer::File::new("test");
    let ty = file.TypeDef(
        "Test",
        "IValue",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );

    let method_signature = Signature {
        return_type: Type::I32,
        ..Default::default()
    };
    let getter = file.MethodDef(
        "get_Value",
        &method_signature,
        MethodAttributes::Public | MethodAttributes::SpecialName,
        Default::default(),
    );

    let first_property = file.Property("Value", &Type::I32);
    file.PropertyMap(ty, first_property);
    file.MethodSemantics(
        0x0002,
        getter,
        writer::HasSemantics::Property(first_property),
    );
    file.MethodSemantics(
        0x0002,
        getter,
        writer::HasSemantics::Property(first_property),
    );
    file.Property("Value", &Type::I32);

    let event_type = Type::ClassName(TypeName::named("Test", "Handler"));
    let first_event = file.Event("Changed", &event_type);
    file.EventMap(ty, first_event);
    file.Event("Changed", &event_type);

    let errors = validator::validate(&index(file));
    assert_eq!(errors.len(), 3);
    assert_eq!(
        errors[0].message(),
        "property `Value` has duplicate method semantics 0x0002"
    );
    assert_eq!(
        errors[1].message(),
        "duplicate property `Value` on `Test.IValue`"
    );
    assert_eq!(
        errors[2].message(),
        "duplicate event `Changed` on `Test.IValue`"
    );
}

#[test]
fn property_overloads_and_split_accessors_are_accepted() {
    let mut file = writer::File::new("test");
    let ty = file.TypeDef(
        "Test",
        "IValue",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );

    let getter = file.MethodDef(
        "get_Value",
        &Signature {
            return_type: Type::I32,
            ..Default::default()
        },
        MethodAttributes::Public | MethodAttributes::SpecialName,
        Default::default(),
    );
    let setter = file.MethodDef(
        "put_Value",
        &Signature {
            return_type: Type::Void,
            types: vec![Type::I32],
            ..Default::default()
        },
        MethodAttributes::Public | MethodAttributes::SpecialName,
        Default::default(),
    );

    let getter_property = file.Property("Value", &Type::I32);
    file.PropertyMap(ty, getter_property);
    file.MethodSemantics(
        0x0002,
        getter,
        writer::HasSemantics::Property(getter_property),
    );
    let setter_property = file.Property("Value", &Type::I32);
    file.MethodSemantics(
        0x0001,
        setter,
        writer::HasSemantics::Property(setter_property),
    );
    file.PropertyWithSignature(
        "Value",
        &Signature {
            return_type: Type::I32,
            types: vec![Type::U32],
            ..Default::default()
        },
        Default::default(),
    );

    assert!(validator::validate(&index(file)).is_empty());
}

#[test]
fn return_types_do_not_distinguish_member_identities() {
    let mut file = writer::File::new("test");
    let ty = file.TypeDef(
        "Test",
        "IValue",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );
    file.MethodDef(
        "Get",
        &Signature {
            return_type: Type::I32,
            ..Default::default()
        },
        MethodAttributes::Public,
        Default::default(),
    );
    file.MethodDef(
        "Get",
        &Signature {
            return_type: Type::U32,
            ..Default::default()
        },
        MethodAttributes::Public,
        Default::default(),
    );

    let property = file.Property("Value", &Type::I32);
    file.PropertyMap(ty, property);
    file.Property("Value", &Type::U32);

    let event = file.Event(
        "Changed",
        &Type::ClassName(TypeName::named("Test", "FirstHandler")),
    );
    file.EventMap(ty, event);
    file.Event(
        "Changed",
        &Type::ClassName(TypeName::named("Test", "SecondHandler")),
    );

    let errors = validator::validate(&index(file));
    assert_eq!(errors.len(), 3);
    assert_eq!(
        errors[0].message(),
        "duplicate property `Value` on `Test.IValue`"
    );
    assert_eq!(
        errors[1].message(),
        "duplicate event `Changed` on `Test.IValue`"
    );
    assert_eq!(
        errors[2].message(),
        "duplicate method `Get` on `Test.IValue`"
    );
    assert!(
        errors
            .iter()
            .all(|error| error.category() == validator::ValidationCategory::Duplicate)
    );
}

#[test]
fn duplicate_interface_implementations_are_rejected() {
    let mut file = writer::File::new("test");
    let object = file.TypeRef("System", "Object");
    let class = file.TypeDef(
        "Test",
        "Value",
        writer::TypeDefOrRef::TypeRef(object),
        TypeAttributes::Public,
    );
    let interface = Type::ClassName(TypeName::named("Test", "IValue"));
    file.InterfaceImpl(class, &interface);
    file.InterfaceImpl(class, &interface);

    let errors = validator::validate(&index(file));
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        "duplicate interface `Test.IValue` on `Test.Value`"
    );
    assert_eq!(
        errors[0].category(),
        validator::ValidationCategory::Duplicate
    );
}

#[test]
fn attribute_multiplicity_is_validated() {
    let mut reference = writer::File::new("reference");
    let system_attribute = reference.TypeRef("System", "Attribute");
    let attribute_targets = TypeName::named("Windows.Foundation.Metadata", "AttributeTargets");

    reference.TypeDef(
        "Windows.Foundation.Metadata",
        "AttributeUsageAttribute",
        writer::TypeDefOrRef::TypeRef(system_attribute),
        TypeAttributes::Public,
    );
    let usage_ref = reference.TypeRef("Windows.Foundation.Metadata", "AttributeUsageAttribute");
    let usage_ctor = reference.MemberRef(
        ".ctor",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            types: vec![Type::ValueName(attribute_targets.clone())],
        },
        writer::MemberRefParent::TypeRef(usage_ref),
    );
    reference.TypeDef(
        "Windows.Foundation.Metadata",
        "AllowMultipleAttribute",
        writer::TypeDefOrRef::TypeRef(system_attribute),
        TypeAttributes::Public,
    );
    let allow_multiple_ref =
        reference.TypeRef("Windows.Foundation.Metadata", "AllowMultipleAttribute");
    let allow_multiple_ctor = reference.MemberRef(
        ".ctor",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            ..Default::default()
        },
        writer::MemberRefParent::TypeRef(allow_multiple_ref),
    );

    let method_only = reference.TypeDef(
        "Test",
        "MethodOnlyAttribute",
        writer::TypeDefOrRef::TypeRef(system_attribute),
        TypeAttributes::Public,
    );
    reference.Attribute(
        writer::HasAttribute::TypeDef(method_only),
        writer::AttributeType::MemberRef(usage_ctor),
        &[(
            String::new(),
            Value::EnumValue(attribute_targets, Box::new(Value::I32(64))),
        )],
    );
    let repeatable = reference.TypeDef(
        "Test",
        "RepeatableAttribute",
        writer::TypeDefOrRef::TypeRef(system_attribute),
        TypeAttributes::Public,
    );
    reference.Attribute(
        writer::HasAttribute::TypeDef(repeatable),
        writer::AttributeType::MemberRef(usage_ctor),
        &[(
            String::new(),
            Value::EnumValue(
                TypeName::named("Windows.Foundation.Metadata", "AttributeTargets"),
                Box::new(Value::I32(64)),
            ),
        )],
    );
    reference.Attribute(
        writer::HasAttribute::TypeDef(repeatable),
        writer::AttributeType::MemberRef(allow_multiple_ctor),
        &[],
    );
    reference.TypeDef(
        "Test",
        "UnspecifiedAttribute",
        writer::TypeDefOrRef::TypeRef(system_attribute),
        TypeAttributes::Public,
    );
    let reference = index(reference);

    let mut file = writer::File::new("test");
    let method_only_ref = file.TypeRef("Test", "MethodOnlyAttribute");
    let method_only_ctor = file.MemberRef(
        ".ctor",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            ..Default::default()
        },
        writer::MemberRefParent::TypeRef(method_only_ref),
    );
    let repeatable_ref = file.TypeRef("Test", "RepeatableAttribute");
    let repeatable_ctor = file.MemberRef(
        ".ctor",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            ..Default::default()
        },
        writer::MemberRefParent::TypeRef(repeatable_ref),
    );
    let unspecified_ref = file.TypeRef("Test", "UnspecifiedAttribute");
    let unspecified_ctor = file.MemberRef(
        ".ctor",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            ..Default::default()
        },
        writer::MemberRefParent::TypeRef(unspecified_ref),
    );
    file.TypeDef(
        "Test",
        "Value",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public,
    );
    let field = file.Field("Value", &Type::I32, FieldAttributes::Public);
    let expected_parent = field.row_id(0);
    for _ in 0..2 {
        file.Attribute(
            writer::HasAttribute::Field(field),
            writer::AttributeType::MemberRef(method_only_ctor),
            &[],
        );
        file.Attribute(
            writer::HasAttribute::Field(field),
            writer::AttributeType::MemberRef(repeatable_ctor),
            &[],
        );
        file.Attribute(
            writer::HasAttribute::Field(field),
            writer::AttributeType::MemberRef(unspecified_ctor),
            &[],
        );
    }

    let output = index(file);
    assert!(validator::validate(&output).is_empty());

    let errors = validator::validate_with_references(&output, &reference);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        "duplicate attribute `Test.MethodOnlyAttribute`"
    );
    assert_eq!(
        errors[0].category(),
        validator::ValidationCategory::Duplicate
    );
    assert_eq!(errors[0].related(), Some(expected_parent));
}

#[test]
fn invalid_method_semantics_are_rejected() {
    let mut file = writer::File::new("test");
    let ty = file.TypeDef(
        "Test",
        "IValue",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );
    let method = file.MethodDef(
        "Value",
        &Signature::default(),
        MethodAttributes::Public,
        Default::default(),
    );
    let property = file.Property("Value", &Type::I32);
    file.PropertyMap(ty, property);
    file.MethodSemantics(0x0040, method, writer::HasSemantics::Property(property));

    let errors = validator::validate(&index(file));
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        "property `Value` has invalid method semantics 0x0040"
    );
    assert!(errors[0].related().is_some());
}

#[test]
fn malformed_property_and_event_ownership_is_rejected() {
    let mut file = writer::File::new("test");
    let ty = file.TypeDef(
        "Test",
        "IValue",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );

    file.Property("Orphaned", &Type::I32);
    let property = file.Property("Value", &Type::I32);
    file.PropertyMap(ty, property);
    file.PropertyMap(ty, property);

    let event_type = Type::ClassName(TypeName::named("Test", "Handler"));
    file.Event("Orphaned", &event_type);
    let event = file.Event("Changed", &event_type);
    file.EventMap(ty, event);
    file.EventMap(ty, event);

    let errors = validator::validate(&index(file));
    assert_eq!(errors.len(), 4);
    assert_eq!(
        errors[0].message(),
        "duplicate property map for `Test.IValue`"
    );
    assert_eq!(errors[1].message(), "property `Orphaned` has no owner");
    assert_eq!(errors[2].message(), "duplicate event map for `Test.IValue`");
    assert_eq!(errors[3].message(), "event `Orphaned` has no owner");
}

#[test]
fn malformed_layouts_are_rejected() {
    let mut file = writer::File::new("test");
    let value_type = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    let ty = file.TypeDef(
        "Test",
        "Value",
        value_type,
        TypeAttributes::Public | TypeAttributes::SequentialLayout,
    );
    let field = file.Field("Value", &Type::I32, FieldAttributes::Public);
    file.ClassLayout(ty, 3, 4);
    file.ClassLayout(ty, 4, 4);
    file.FieldLayout(field, 0);
    file.FieldLayout(field, 4);

    let errors = validator::validate(&index(file));
    assert_eq!(errors.len(), 5);
    assert_eq!(
        errors[0].message(),
        "class layout for `Test.Value` has invalid packing size 3"
    );
    assert_eq!(
        errors[1].message(),
        "duplicate class layout for `Test.Value`"
    );
    assert_eq!(
        errors[2].message(),
        "field layout for `Test.Value.Value` requires explicit layout"
    );
    assert_eq!(errors[3].message(), "duplicate field layout for `Value`");
    assert_eq!(
        errors[4].message(),
        "field layout for `Test.Value.Value` requires explicit layout"
    );
}

#[test]
fn committed_windows_metadata_is_valid() {
    let index = reader::Index::new(
        [windows_default::WINRT, windows_default::WIN32]
            .into_iter()
            .map(|bytes| reader::File::new(bytes.to_vec()).unwrap())
            .collect(),
    );

    let errors = validator::validate(&index);
    assert!(
        errors.is_empty(),
        "committed metadata validation failed:\n{}",
        errors
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}
