use std::path::Path;
use windows_metadata as metadata;

fn out_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("test_rdl_metadata_{name}.rdl"))
}

fn writer_error(name: &str, file: metadata::writer::File) -> windows_rdl::Error {
    windows_rdl::writer()
        .input_bytes(&file.into_stream())
        .output(out_path(name))
        .write()
        .unwrap_err()
}

fn attribute_file(signature: metadata::Signature) -> metadata::writer::File {
    let mut file = metadata::writer::File::new("test");
    let extends = file.TypeRef("System", "Attribute");
    file.TypeDef(
        "Test",
        "ValueAttribute",
        metadata::writer::TypeDefOrRef::TypeRef(extends),
        metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed,
    );
    file.MethodDef(
        ".ctor",
        &signature,
        metadata::MethodAttributes::Public,
        Default::default(),
    );
    file
}

fn interface_file(
    signature: metadata::Signature,
) -> (metadata::writer::File, metadata::writer::MethodDef) {
    let mut file = metadata::writer::File::new("test");
    file.TypeDef(
        "Test",
        "IValue",
        metadata::writer::TypeDefOrRef::default(),
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Interface
            | metadata::TypeAttributes::Abstract,
    );
    let method = file.MethodDef(
        "Method",
        &signature,
        metadata::MethodAttributes::Public | metadata::MethodAttributes::Abstract,
        Default::default(),
    );
    (file, method)
}

fn marker_attribute(file: &mut metadata::writer::File) -> metadata::writer::AttributeType {
    let attribute = file.TypeRef("Test", "MarkerAttribute");
    let constructor = file.MemberRef(
        ".ctor",
        &metadata::Signature {
            flags: metadata::MethodCallAttributes::HASTHIS,
            ..Default::default()
        },
        metadata::writer::MemberRefParent::TypeRef(attribute),
    );
    metadata::writer::AttributeType::MemberRef(constructor)
}

#[test]
fn writer_rejects_unrepresentable_attribute_constructor_metadata() {
    let error = writer_error(
        "attribute_return",
        attribute_file(metadata::Signature {
            return_type: metadata::Type::I32,
            ..Default::default()
        }),
    );
    assert_eq!(
        error.message,
        "attribute constructor `.ctor` has an unrepresentable return type"
    );

    let error = writer_error(
        "attribute_variadic",
        attribute_file(metadata::Signature {
            flags: metadata::MethodCallAttributes::VARARG,
            ..Default::default()
        }),
    );
    assert_eq!(
        error.message,
        "attribute constructor method `.ctor` has an unrepresentable variadic signature"
    );
}

#[test]
fn writer_rejects_unrepresentable_interface_method_metadata() {
    let (file, _) = interface_file(metadata::Signature {
        flags: metadata::MethodCallAttributes::VARARG,
        ..Default::default()
    });
    let error = writer_error("interface_variadic", file);
    assert_eq!(
        error.message,
        "interface method `Method` has an unrepresentable variadic signature"
    );

    let (mut file, method) = interface_file(metadata::Signature::default());
    file.GenericParam(
        "T",
        metadata::writer::TypeOrMethodDef::MethodDef(method),
        0,
        metadata::GenericParamAttributes::None,
    );
    let error = writer_error("method_generic", file);
    assert_eq!(
        error.message,
        "method `Method` has unrepresentable generic parameter `T`"
    );
}

#[test]
fn writer_rejects_unrepresentable_generic_parameter_flags() {
    let mut file = metadata::writer::File::new("test");
    let extends = file.TypeRef("System", "MulticastDelegate");
    let delegate = file.TypeDef(
        "Test",
        "Handler`1",
        metadata::writer::TypeDefOrRef::TypeRef(extends),
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Sealed
            | metadata::TypeAttributes::WindowsRuntime,
    );
    file.GenericParam(
        "T",
        metadata::writer::TypeOrMethodDef::TypeDef(delegate),
        0,
        metadata::GenericParamAttributes(1),
    );
    file.MethodDef(
        "Invoke",
        &metadata::Signature::default(),
        metadata::MethodAttributes::Public,
        Default::default(),
    );

    let error = writer_error("generic_flags", file);
    assert_eq!(
        error.message,
        "generic parameter `T` on `Handler`1` has unsupported flags GenericParamAttributes(1)"
    );
}

#[test]
fn writer_rejects_unrepresentable_property_and_event_attributes() {
    let mut file = metadata::writer::File::new("test");
    let interface = file.TypeDef(
        "Test",
        "IValue",
        metadata::writer::TypeDefOrRef::default(),
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Interface
            | metadata::TypeAttributes::Abstract
            | metadata::TypeAttributes::WindowsRuntime,
    );
    let property = file.Property("Value", &metadata::Type::I32);
    file.PropertyMap(interface, property);
    let attribute = marker_attribute(&mut file);
    file.Attribute(
        metadata::writer::HasAttribute::Property(property),
        attribute,
        &[],
    );

    let error = writer_error("property_attribute", file);
    assert_eq!(
        error.message,
        "property `Value` has unrepresentable custom attributes"
    );

    let mut file = metadata::writer::File::new("test");
    let interface = file.TypeDef(
        "Test",
        "IValue",
        metadata::writer::TypeDefOrRef::default(),
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Interface
            | metadata::TypeAttributes::Abstract
            | metadata::TypeAttributes::WindowsRuntime,
    );
    let event = file.Event("Changed", &metadata::Type::class_named("Test", "Handler"));
    file.EventMap(interface, event);
    let attribute = marker_attribute(&mut file);
    file.Attribute(metadata::writer::HasAttribute::Event(event), attribute, &[]);

    let error = writer_error("event_attribute", file);
    assert_eq!(
        error.message,
        "event `Changed` has unrepresentable custom attributes"
    );
}

#[test]
fn writer_rejects_property_and_event_rows_without_semantics() {
    let mut file = metadata::writer::File::new("test");
    let interface = file.TypeDef(
        "Test",
        "IValue",
        metadata::writer::TypeDefOrRef::default(),
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Interface
            | metadata::TypeAttributes::Abstract
            | metadata::TypeAttributes::WindowsRuntime,
    );
    let property = file.Property("Value", &metadata::Type::I32);
    file.PropertyMap(interface, property);

    let error = writer_error("property_semantics", file);
    assert_eq!(error.message, "property `Value` has no accessor semantics");

    let mut file = metadata::writer::File::new("test");
    let interface = file.TypeDef(
        "Test",
        "IValue",
        metadata::writer::TypeDefOrRef::default(),
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Interface
            | metadata::TypeAttributes::Abstract
            | metadata::TypeAttributes::WindowsRuntime,
    );
    let event = file.Event("Changed", &metadata::Type::class_named("Test", "Handler"));
    file.EventMap(interface, event);

    let error = writer_error("event_semantics", file);
    assert_eq!(
        error.message,
        "event `Changed` requires add and remove semantics"
    );
}

#[test]
fn writer_rejects_attributes_on_reference_rows() {
    let mut file = metadata::writer::File::new("test");
    let type_ref = file.TypeRef("Test", "External");
    let attribute = marker_attribute(&mut file);
    file.Attribute(
        metadata::writer::HasAttribute::TypeRef(type_ref),
        attribute,
        &[],
    );

    let error = writer_error("type_ref_attribute", file);
    assert_eq!(
        error.message,
        "custom attribute `Test.MarkerAttribute` on type reference `Test.External` has no RDL spelling"
    );

    let mut file = metadata::writer::File::new("test");
    let type_ref = file.TypeRef("Test", "External");
    let member_ref = file.MemberRef(
        "Method",
        &metadata::Signature::default(),
        metadata::writer::MemberRefParent::TypeRef(type_ref),
    );
    let attribute = marker_attribute(&mut file);
    file.Attribute(
        metadata::writer::HasAttribute::MemberRef(member_ref),
        attribute,
        &[],
    );

    let error = writer_error("member_ref_attribute", file);
    assert_eq!(
        error.message,
        "custom attribute `Test.MarkerAttribute` on member reference `External.Method` has no RDL spelling"
    );

    let mut file = metadata::writer::File::new("test");
    let type_spec = file.TypeSpec("Test", "External", &[metadata::Type::I32]);
    let attribute = marker_attribute(&mut file);
    file.Attribute(
        metadata::writer::HasAttribute::TypeSpec(type_spec),
        attribute,
        &[],
    );

    let error = writer_error("type_spec_attribute", file);
    assert_eq!(
        error.message,
        "custom attribute `Test.MarkerAttribute` on a type specification has no RDL spelling"
    );
}

#[test]
fn writer_rejects_unrepresentable_field_layouts() {
    let mut file = metadata::writer::File::new("test");
    let value_type = file.TypeRef("System", "ValueType");
    file.TypeDef(
        "Test",
        "Explicit",
        metadata::writer::TypeDefOrRef::TypeRef(value_type),
        metadata::TypeAttributes::Public | metadata::TypeAttributes::ExplicitLayout,
    );
    let field = file.Field(
        "Value",
        &metadata::Type::I32,
        metadata::FieldAttributes::Public,
    );
    file.FieldLayout(field, 4);

    let error = writer_error("nonzero_field_layout", file);
    assert_eq!(
        error.message,
        "field `Value` on explicit-layout type `Explicit` has unrepresentable offset 4"
    );

    let mut file = metadata::writer::File::new("test");
    let value_type = file.TypeRef("System", "ValueType");
    file.TypeDef(
        "Test",
        "Explicit",
        metadata::writer::TypeDefOrRef::TypeRef(value_type),
        metadata::TypeAttributes::Public | metadata::TypeAttributes::ExplicitLayout,
    );
    file.Field(
        "Value",
        &metadata::Type::I32,
        metadata::FieldAttributes::Public,
    );

    let error = writer_error("missing_field_layout", file);
    assert_eq!(
        error.message,
        "field `Value` on explicit-layout type `Explicit` has no FieldLayout row"
    );
}
