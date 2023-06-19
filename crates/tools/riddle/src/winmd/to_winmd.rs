use crate::winmd::{self, writer};

pub fn from_reader(
    reader: &metadata::Reader,
    filter: &metadata::Filter,
    _config: std::collections::BTreeMap<&str, &str>,
    output: &str,
) -> crate::Result<()> {
    let mut writer = winmd::Writer::new(output);

    for def in reader.types(filter) {
        let extends = if let Some(extends) = reader.type_def_extends(def) {
            writer.insert_type_ref(extends.namespace, extends.name)
        } else {
            0
        };

        writer.tables.TypeDef.push(writer::TypeDef {
            Extends: extends,
            FieldList: writer.tables.Field.len() as _,
            Flags: reader.type_def_flags(def).0,
            MethodList: writer.tables.MethodDef.len() as _,
            TypeName: writer.strings.insert(reader.type_def_name(def)),
            TypeNamespace: writer.strings.insert(reader.type_def_namespace(def)),
        });

        for field in reader.type_def_fields(def) {
            let ty = field_writer_type(reader, field, Some(def));
            let signature = writer.insert_field_sig(&ty);

            writer.tables.Field.push(writer::Field {
                Flags: reader.field_flags(field).0,
                Name: writer.strings.insert(reader.field_name(field)),
                Signature: signature,
            });
        }
    }

    // TODO: In theory, `config` could instruct this function to balance the types across a number of winmd files
    // like mdmerge supports for namespace-splitting.
    crate::write_to_file(output, writer.into_stream()).map_err(|err| err.with_path(output))
}

fn field_writer_type(
    reader: &metadata::Reader,
    row: metadata::Field,
    enclosing: Option<metadata::TypeDef>,
) -> winmd::Type {
    match reader.field_type(row, enclosing) {
        metadata::Type::Void => winmd::Type::Void,
        metadata::Type::Bool => winmd::Type::Bool,
        metadata::Type::Char => winmd::Type::Char,
        metadata::Type::I8 => winmd::Type::I8,
        metadata::Type::U8 => winmd::Type::U8,
        metadata::Type::I16 => winmd::Type::I16,
        metadata::Type::U16 => winmd::Type::U16,
        metadata::Type::I32 => winmd::Type::I32,
        metadata::Type::U32 => winmd::Type::U32,
        metadata::Type::I64 => winmd::Type::I64,
        metadata::Type::U64 => winmd::Type::U64,
        metadata::Type::F32 => winmd::Type::F32,
        metadata::Type::F64 => winmd::Type::F64,
        metadata::Type::ISize => winmd::Type::ISize,
        metadata::Type::USize => winmd::Type::USize,
        metadata::Type::String => winmd::Type::String,
        metadata::Type::GUID => winmd::Type::GUID,
        metadata::Type::IUnknown => winmd::Type::IUnknown,
        metadata::Type::IInspectable => winmd::Type::IInspectable,
        metadata::Type::HRESULT => winmd::Type::HRESULT,
        metadata::Type::PSTR => winmd::Type::PSTR,
        metadata::Type::PWSTR => winmd::Type::PWSTR,
        metadata::Type::PCSTR => winmd::Type::PCSTR,
        metadata::Type::PCWSTR => winmd::Type::PCWSTR,
        metadata::Type::BSTR => winmd::Type::BSTR,
        metadata::Type::TypeName => winmd::Type::TypeName,
        // Type::TypeRef(TypeDefOrRef) =>
        // Type::GenericParam(GenericParam) =>
        // Type::TypeDef(TypeDef, Vec<Self>) =>
        // Type::MutPtr(Box<Self>, usize) =>
        // Type::ConstPtr(Box<Self>, usize) =>
        // Type::Win32Array(Box<Self>, usize) =>
        // Type::WinrtArray(Box<Self>) =>
        // Type::WinrtArrayRef(Box<Self>) =>
        // Type::ConstRef(Box<Self>) =>
        _ => unimplemented!(),
    }
}
