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
