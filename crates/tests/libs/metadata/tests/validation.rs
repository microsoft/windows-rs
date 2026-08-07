use windows_metadata::*;

fn index(file: writer::File) -> reader::Index {
    file.into_index()
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
    assert_eq!(
        errors[0].message(),
        "duplicate field `Value` on `Test.IValue`"
    );
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
