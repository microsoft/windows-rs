use crate::winmd::{self, writer};

pub fn from_reader(
    reader: &metadata::Reader,
    filter: &metadata::Filter,
    config: std::collections::BTreeMap<&str, &str>,
    output: &str,
) -> crate::Result<()> {
    let mut writer = winmd::Writer::new(output);

    // TODO: do we need any configuration values for winmd generation?
    // Maybe per-namespace winmd files for namespace-splitting - be sure to use
    // the same key as for winmd generation.

    if let Some((key, _)) = config.first_key_value() {
        return Err(crate::Error::new(&format!(
            "invalid configuration value `{key}`"
        )));
    }

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
            let ty = writer_type(reader, &reader.field_type(field, Some(def)));
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

fn writer_type(reader: &metadata::Reader, ty: &metadata::Type) -> winmd::Type {
    match ty {
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
        metadata::Type::TypeDef(def, generics) => winmd::Type::TypeRef(winmd::TypeName {
            namespace: reader.type_def_namespace(*def).to_string(),
            name: reader.type_def_name(*def).to_string(),
            generics: generics.iter().map(|ty| writer_type(reader, ty)).collect(),
        }),
        rest => unimplemented!("{rest:?}"),
    }
}
