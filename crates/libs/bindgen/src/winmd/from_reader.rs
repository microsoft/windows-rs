use super::*;
use crate::winmd::{self, writer};
use metadata::RowReader;

pub fn from_reader(reader: &metadata::Reader, filter: &metadata::Filter, config: std::collections::BTreeMap<&str, &str>, output: &str) -> crate::Result<()> {
    let mut writer = winmd::Writer::new(output);

    // TODO: do we need any configuration values for winmd generation?
    // Maybe per-namespace winmd files for namespace-splitting - be sure to use
    // the same key as for winmd generation.

    if let Some((key, _)) = config.first_key_value() {
        return Err(crate::Error::new(&format!("invalid configuration value `{key}`")));
    }

    // TODO: just use the reader directly since we now have everything in the reader, there's no need to abstract
    // away the source format. Few reprs is always better.

    for item in reader.items(filter) {
        // TODO: cover all variants
        let metadata::Item::Type(def) = item else {
            continue;
        };

        let generics = &metadata::type_def_generics(reader, def);

        let extends = if let Some(extends) = reader.type_def_extends(def) { writer.insert_type_ref(extends.namespace, extends.name) } else { 0 };

        writer.tables.TypeDef.push(writer::TypeDef {
            Extends: extends,
            FieldList: writer.tables.Field.len() as u32,
            Flags: reader.type_def_flags(def).0,
            MethodList: writer.tables.MethodDef.len() as u32,
            TypeName: writer.strings.insert(reader.type_def_name(def)),
            TypeNamespace: writer.strings.insert(reader.type_def_namespace(def)),
        });

        for generic in reader.type_def_generics(def) {
            writer.tables.GenericParam.push(writer::GenericParam {
                Number: reader.generic_param_number(generic),
                Flags: 0,
                Owner: writer::TypeOrMethodDef::TypeDef(writer.tables.TypeDef.len() as u32 - 1).encode(),
                Name: writer.strings.insert(reader.generic_param_name(generic)),
            });
        }

        for imp in reader.type_def_interface_impls(def) {
            let ty = reader.interface_impl_type(imp, generics);
            let ty = winmd_type(reader, &ty);

            let reference = match &ty {
                winmd::Type::TypeRef(type_name) if type_name.generics.is_empty() => writer.insert_type_ref(&type_name.namespace, &type_name.name),
                winmd::Type::TypeRef(_) => writer.insert_type_spec(ty),
                winmd::Type::IUnknown => writer.insert_type_ref("Windows.Win32.System.Com", "IUnknown"),
                winmd::Type::IInspectable => writer.insert_type_ref("Windows.Win32.System.WinRT", "IInspectable"),
                rest => unimplemented!("{rest:?}"),
            };

            writer.tables.InterfaceImpl.push(writer::InterfaceImpl { Class: writer.tables.TypeDef.len() as u32 - 1, Interface: reference });
        }

        // TODO: if the class is "Apis" then should we sort the fields (constants) and methods (functions) for stability

        for field in reader.type_def_fields(def) {
            let ty = winmd_type(reader, &reader.field_type(field, Some(def)));
            let signature = writer.insert_field_sig(&ty);

            writer.tables.Field.push(writer::Field { Flags: reader.field_flags(field).0, Name: writer.strings.insert(reader.field_name(field)), Signature: signature });
        }

        for method in reader.type_def_methods(def) {
            let signature = reader.method_def_signature(method, generics);
            let return_type = winmd_type(reader, &signature.return_type);
            let param_types: Vec<Type> = signature.params.iter().map(|param| winmd_type(reader, param)).collect();

            let signature = writer.insert_method_sig(signature.call_flags, &return_type, &param_types);

            writer.tables.MethodDef.push(winmd::MethodDef {
                RVA: 0,
                ImplFlags: reader.method_def_impl_flags(method).0,
                Flags: reader.method_def_flags(method).0,
                Name: writer.strings.insert(reader.method_def_name(method)),
                Signature: signature,
                ParamList: writer.tables.Param.len() as u32,
            });

            for param in reader.method_def_params(method) {
                writer.tables.Param.push(writer::Param { Flags: reader.param_flags(param).0, Sequence: reader.param_sequence(param), Name: writer.strings.insert(reader.param_name(param)) });
            }
        }
    }

    // TODO: In theory, `config` could instruct this function to balance the types across a number of winmd files
    // like mdmerge supports for namespace-splitting.
    crate::write_to_file(output, writer.into_stream()).map_err(|err| err.with_path(output))
}

// TODO: keep the basic type conversion
fn winmd_type(reader: &metadata::Reader, ty: &metadata::Type) -> winmd::Type {
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
            generics: generics.iter().map(|ty| winmd_type(reader, ty)).collect(),
        }),
        metadata::Type::GenericParam(generic) => winmd::Type::GenericParam(reader.generic_param_number(*generic)),
        metadata::Type::ConstRef(ty) => winmd::Type::ConstRef(Box::new(winmd_type(reader, ty))),
        metadata::Type::WinrtArrayRef(ty) => winmd::Type::WinrtArrayRef(Box::new(winmd_type(reader, ty))),
        metadata::Type::WinrtArray(ty) => winmd::Type::WinrtArray(Box::new(winmd_type(reader, ty))),
        metadata::Type::MutPtr(ty, pointers) => winmd::Type::MutPtr(Box::new(winmd_type(reader, ty)), *pointers),
        metadata::Type::ConstPtr(ty, pointers) => winmd::Type::ConstPtr(Box::new(winmd_type(reader, ty)), *pointers),
        metadata::Type::Win32Array(ty, len) => winmd::Type::Win32Array(Box::new(winmd_type(reader, ty)), *len),
        rest => unimplemented!("{rest:?}"),
    }
}
