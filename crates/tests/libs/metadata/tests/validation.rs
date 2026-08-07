use windows_metadata::writer::RowHandle;
use windows_metadata::*;

fn index(file: writer::File) -> reader::Index {
    file.into_index()
}

fn attribute_ctor(file: &mut writer::File, name: &str, signature: &Signature) -> writer::MemberRef {
    let ty = file.TypeRef("Test", name);
    file.MemberRef(".ctor", signature, writer::MemberRefParent::TypeRef(ty))
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
fn invalid_attribute_constructors_are_rejected() {
    let mut file = writer::File::new("test");
    file.TypeDef(
        "Test",
        "Value",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public,
    );
    let field = file.Field("Value", &Type::I32, FieldAttributes::Public);
    let expected_parent = field.row_id(0);
    let attribute = writer::MemberRefParent::TypeRef(file.TypeRef("Test", "MarkerAttribute"));

    let wrong_name = file.MemberRef("Create", &Signature::default(), attribute);
    file.Attribute(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(wrong_name),
        &[],
    );

    let static_ctor = file.MemberRef(
        ".ctor",
        &Signature {
            flags: MethodCallAttributes(0),
            ..Default::default()
        },
        attribute,
    );
    file.Attribute(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(static_ctor),
        &[],
    );

    let returning_ctor = file.MemberRef(
        ".ctor",
        &Signature {
            return_type: Type::I32,
            ..Default::default()
        },
        attribute,
    );
    file.Attribute(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(returning_ctor),
        &[],
    );

    let vararg_ctor = file.MemberRef(
        ".ctor",
        &Signature {
            flags: MethodCallAttributes::HASTHIS | MethodCallAttributes::VARARG,
            ..Default::default()
        },
        attribute,
    );
    file.Attribute(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(vararg_ctor),
        &[],
    );

    let valid_ctor = file.MemberRef(".ctor", &Signature::default(), attribute);
    file.AttributeBlob(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(valid_ctor),
        &[0, 0],
    );

    let invalid_parameters = file.MemberRef(
        ".ctor",
        &Signature {
            types: vec![
                Type::ISize,
                Type::PtrMut(Box::new(Type::Void), 1),
                Type::ClassName(TypeName::named("Test", "Value")),
            ],
            ..Default::default()
        },
        attribute,
    );
    file.AttributeBlob(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(invalid_parameters),
        &[1, 0],
    );

    let errors = validator::validate(&index(file));
    assert_eq!(errors.len(), 8);
    assert_eq!(
        errors[0].message(),
        "attribute `Test.MarkerAttribute` constructor is named `Create` instead of `.ctor`"
    );
    assert_eq!(
        errors[1].message(),
        "attribute `Test.MarkerAttribute` constructor must be an instance method"
    );
    assert_eq!(
        errors[2].message(),
        "attribute `Test.MarkerAttribute` constructor must return void"
    );
    assert_eq!(
        errors[3].message(),
        "attribute `Test.MarkerAttribute` constructor must use the default calling convention"
    );
    assert_eq!(
        errors[4].message(),
        "attribute `Test.MarkerAttribute` value is invalid at byte 0: invalid custom-attribute prolog"
    );
    assert_eq!(
        errors[5].message(),
        "attribute `Test.MarkerAttribute` constructor parameter 1 has invalid type `ISize`"
    );
    assert_eq!(
        errors[6].message(),
        "attribute `Test.MarkerAttribute` constructor parameter 2 has invalid type `PtrMut(Void, 1)`"
    );
    assert_eq!(
        errors[7].message(),
        "attribute `Test.MarkerAttribute` constructor parameter 3 has invalid type `Test.Value`"
    );
    assert!(errors.iter().all(|error| {
        error.category() == validator::ValidationCategory::Invalid
            && error.related() == Some(expected_parent)
    }));
}

#[test]
fn invalid_attribute_values_are_rejected() {
    let mut file = writer::File::new("test");
    file.TypeDef(
        "Test",
        "Value",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public,
    );
    let field = file.Field("Value", &Type::I32, FieldAttributes::Public);
    let expected_parent = field.row_id(0);

    let truncated = attribute_ctor(&mut file, "TruncatedAttribute", &Signature::default());
    file.AttributeBlob(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(truncated),
        &[1, 0],
    );

    let boolean = attribute_ctor(
        &mut file,
        "BooleanAttribute",
        &Signature {
            types: vec![Type::Bool],
            ..Default::default()
        },
    );
    file.AttributeBlob(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(boolean),
        &[1, 0, 2],
    );

    let tag = attribute_ctor(&mut file, "TagAttribute", &Signature::default());
    file.AttributeBlob(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(tag),
        &[1, 0, 1, 0, 0x52],
    );

    let trailing = attribute_ctor(&mut file, "TrailingAttribute", &Signature::default());
    file.AttributeBlob(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(trailing),
        &[1, 0, 0, 0, 0],
    );

    let character = attribute_ctor(
        &mut file,
        "CharacterAttribute",
        &Signature {
            types: vec![Type::Char],
            ..Default::default()
        },
    );
    file.AttributeBlob(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(character),
        &[1, 0, 65, 0, 0, 0],
    );

    let index = index(file);
    let errors = validator::validate(&index);
    assert_eq!(errors.len(), 4);
    assert_eq!(
        errors[0].message(),
        "attribute `Test.TruncatedAttribute` value is invalid at byte 2: truncated custom-attribute value"
    );
    assert_eq!(
        errors[1].message(),
        "attribute `Test.BooleanAttribute` value is invalid at byte 2: invalid Boolean value"
    );
    assert_eq!(
        errors[2].message(),
        "attribute `Test.TagAttribute` value is invalid at byte 4: invalid named-argument tag"
    );
    assert_eq!(
        errors[3].message(),
        "attribute `Test.TrailingAttribute` value is invalid at byte 4: trailing custom-attribute data"
    );
    assert!(errors.iter().all(|error| {
        error.category() == validator::ValidationCategory::Invalid
            && error.related() == Some(expected_parent)
    }));

    let character = index
        .attributes()
        .find(|attribute| attribute.name() == "CharacterAttribute")
        .unwrap();
    assert_eq!(
        character.try_value().unwrap(),
        [(String::new(), Value::Char(65))]
    );
}

#[test]
fn attribute_enum_values_use_reference_backing_types() {
    let mut reference = writer::File::new("reference");
    let system_enum = reference.TypeRef("System", "Enum");
    reference.TypeDef(
        "Test",
        "SmallEnum",
        writer::TypeDefOrRef::TypeRef(system_enum),
        TypeAttributes::Public | TypeAttributes::Sealed,
    );
    reference.Field("value__", &Type::U8, FieldAttributes::Public);
    let reference = index(reference);

    let mut file = writer::File::new("test");
    file.TypeDef(
        "Test",
        "Value",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public,
    );
    let field = file.Field("Value", &Type::I32, FieldAttributes::Public);
    let enum_name = TypeName::named("Test", "SmallEnum");
    let ctor = attribute_ctor(
        &mut file,
        "EnumAttribute",
        &Signature {
            types: vec![Type::ValueName(enum_name.clone())],
            ..Default::default()
        },
    );
    file.AttributeBlob(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(ctor),
        &[1, 0, 7, 0, 0],
    );
    let output = index(file);
    let attribute = output.attributes().next().unwrap();

    assert!(attribute.try_value().unwrap_err().is_unsupported());
    assert_eq!(
        attribute.try_value_with_references(&reference).unwrap(),
        [(
            String::new(),
            Value::EnumValue(enum_name, Box::new(Value::U8(7)))
        )]
    );
    assert!(
        validator::Validator::new(&output)
            .references(&reference)
            .validate()
            .is_empty()
    );
}

#[test]
fn attribute_char_values_preserve_utf16_code_units() {
    let mut file = writer::File::new("test");
    file.TypeDef(
        "Test",
        "Value",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public,
    );
    let field = file.Field("Value", &Type::I32, FieldAttributes::Public);
    let ctor = attribute_ctor(
        &mut file,
        "CharAttribute",
        &Signature {
            types: vec![Type::Char],
            ..Default::default()
        },
    );
    file.Attribute(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(ctor),
        &[(String::new(), Value::Char(0xd800))],
    );

    let index = index(file);
    let attribute = index.attributes().next().unwrap();
    assert_eq!(
        attribute.try_value().unwrap(),
        [(String::new(), Value::Char(0xd800))]
    );
    assert!(validator::validate(&index).is_empty());
}

#[test]
fn attribute_named_arguments_are_validated() {
    let mut reference = writer::File::new("reference");
    let system_attribute = reference.TypeRef("System", "Attribute");
    let definition = reference.TypeDef(
        "Test",
        "NamedAttribute",
        writer::TypeDefOrRef::TypeRef(system_attribute),
        TypeAttributes::Public,
    );
    reference.Field("Field", &Type::I32, FieldAttributes::Public);
    let property = reference.PropertyWithSignature(
        "Property",
        &Signature {
            return_type: Type::U32,
            ..Default::default()
        },
        0,
    );
    let setter = reference.MethodDef(
        "set_Property",
        &Signature {
            return_type: Type::Void,
            types: vec![Type::U32],
            ..Default::default()
        },
        MethodAttributes::Public | MethodAttributes::SpecialName,
        MethodImplAttributes::default(),
    );
    reference.MethodSemantics(0x0001, setter, writer::HasSemantics::Property(property));
    reference.Field(
        "StaticField",
        &Type::I32,
        FieldAttributes::Public | FieldAttributes::Static,
    );
    reference.PropertyWithSignature(
        "ReadOnly",
        &Signature {
            return_type: Type::U32,
            ..Default::default()
        },
        0,
    );
    reference.PropertyMap(definition, property);
    let reference = index(reference);

    let mut file = writer::File::new("test");
    file.TypeDef(
        "Test",
        "Value",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public,
    );
    let field = file.Field("Value", &Type::I32, FieldAttributes::Public);
    let expected_parent = field.row_id(0);
    let ctor = attribute_ctor(&mut file, "NamedAttribute", &Signature::default());
    let mut blob = vec![1, 0, 7, 0];
    blob.extend([0x53, 0x08, 5, b'F', b'i', b'e', b'l', b'd', 1, 0, 0, 0]);
    blob.extend([
        0x54, 0x09, 8, b'P', b'r', b'o', b'p', b'e', b'r', b't', b'y', 2, 0, 0, 0,
    ]);
    blob.extend([
        0x53, 0x08, 7, b'M', b'i', b's', b's', b'i', b'n', b'g', 3, 0, 0, 0,
    ]);
    blob.extend([0x53, 0x09, 5, b'F', b'i', b'e', b'l', b'd', 4, 0, 0, 0]);
    blob.extend([
        0x54, 0x09, 8, b'P', b'r', b'o', b'p', b'e', b'r', b't', b'y', 5, 0, 0, 0,
    ]);
    blob.extend([
        0x53, 0x08, 11, b'S', b't', b'a', b't', b'i', b'c', b'F', b'i', b'e', b'l', b'd', 6, 0, 0,
        0,
    ]);
    blob.extend([
        0x54, 0x09, 8, b'R', b'e', b'a', b'd', b'O', b'n', b'l', b'y', 7, 0, 0, 0,
    ]);
    file.AttributeBlob(
        writer::HasAttribute::Field(field),
        writer::AttributeType::MemberRef(ctor),
        &blob,
    );
    let output = index(file);

    let errors = validator::Validator::new(&output)
        .references(&reference)
        .validate();
    assert_eq!(errors.len(), 6);
    assert_eq!(
        errors[0].message(),
        "attribute `Test.NamedAttribute` has no named field `Missing`"
    );
    assert_eq!(
        errors[1].message(),
        "attribute `Test.NamedAttribute` has duplicate named field argument `Field`"
    );
    assert_eq!(
        errors[2].message(),
        "attribute `Test.NamedAttribute` named field `Field` expects `I32` but found `U32`"
    );
    assert_eq!(
        errors[3].message(),
        "attribute `Test.NamedAttribute` has duplicate named property argument `Property`"
    );
    assert_eq!(
        errors[4].message(),
        "attribute `Test.NamedAttribute` named field `StaticField` is not a public writable instance member"
    );
    assert_eq!(
        errors[5].message(),
        "attribute `Test.NamedAttribute` named property `ReadOnly` is not a public writable instance member"
    );
    assert!(errors.iter().all(|error| {
        error.related() == Some(expected_parent)
            && matches!(
                error.category(),
                validator::ValidationCategory::Invalid | validator::ValidationCategory::Duplicate
            )
    }));
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

    let errors = validator::Validator::new(&output)
        .references(&reference)
        .validate();
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
fn invalid_member_signatures_are_rejected() {
    let mut file = writer::File::new("test");
    let ty = file.TypeDef(
        "Test",
        "Value",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public,
    );
    file.Field("Bad", &Type::Void, FieldAttributes::Public);
    file.MethodDef(
        "StaticWithThis",
        &Signature::default(),
        MethodAttributes::Public | MethodAttributes::Static,
        MethodImplAttributes::default(),
    );
    file.MethodDef(
        "InstanceWithoutThis",
        &Signature {
            flags: MethodCallAttributes(0),
            ..Default::default()
        },
        MethodAttributes::Public,
        MethodImplAttributes::default(),
    );
    file.MethodDef(
        "BadParameter",
        &Signature {
            types: vec![Type::Void],
            ..Default::default()
        },
        MethodAttributes::Public,
        MethodImplAttributes::default(),
    );
    let bad_value = file.PropertyWithSignature(
        "BadValue",
        &Signature {
            return_type: Type::Void,
            ..Default::default()
        },
        0,
    );
    file.PropertyWithSignature(
        "BadIndex",
        &Signature {
            return_type: Type::I32,
            types: vec![Type::Array(Box::new(Type::Void))],
            ..Default::default()
        },
        0,
    );
    file.PropertyMap(ty, bad_value);

    let errors = validator::validate(&index(file));
    assert_eq!(errors.len(), 5);
    assert_eq!(
        errors[0].message(),
        "field `Test.Value.Bad` has invalid type `Void`"
    );
    assert_eq!(
        errors[1].message(),
        "property `Test.Value.BadValue` has invalid value type `Void`"
    );
    assert_eq!(
        errors[2].message(),
        "property `Test.Value.BadIndex` index parameter 1 has invalid type `Void[]`"
    );
    assert_eq!(
        errors[3].message(),
        "static method `Test.Value.StaticWithThis` has an instance calling convention"
    );
    assert_eq!(
        errors[4].message(),
        "method `Test.Value.BadParameter` parameter 1 has invalid type `Void`"
    );
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
