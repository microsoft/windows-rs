use super::*;

pub fn read_winmd(module: &mut Module, paths: &[String], filter: &Filter) -> Result<()> {
    let mut files = vec![];

    for path in paths {
        if extension(path).1 == "winmd" {
            files.push(reader::File::new(path)?);
        }
    }

    let reader = reader::Reader::new(&files);

    for ns in reader.namespaces() {
        if filter.includes_namespace(ns) {
            for ty in reader.namespace_types(ns, filter) {
                module.insert(reader.type_def_namespace(ty), 0).types.entry(crate::reader::trim_tick(reader.type_def_name(ty)).to_string()).or_default().push(read_type_def(&reader, ty)?);
            }
        }
    }

    Ok(())
}

fn read_type_def(reader: &reader::Reader, ty: reader::TypeDef) -> Result<TypeDef> {
    let mut result = TypeDef { flags: reader.type_def_flags(ty), ..Default::default() };
    result.attributes = read_attributes(reader, reader.type_def_attributes(ty))?;
    result.extends = reader.type_def_extends(ty).map(|extends| TypeRef { namespace: extends.namespace.to_string(), name: extends.name.to_string(), ..Default::default() });

    if result.flags.contains(TypeAttributes::INTERFACE) || !result.flags.contains(TypeAttributes::WINDOWS_RUNTIME) {
        for method in reader.type_def_methods(ty) {
            let flags = reader.method_def_flags(method);
            let sig = reader.method_def_signature(method, &[]);
            let name = reader.method_def_name(method).to_string();
            let attributes = read_attributes(reader, reader.method_def_attributes(method))?;
            let mut params = vec![];

            for param in sig.params {
                let flags = reader.param_flags(param.def);
                let name = reader.param_name(param.def).to_string();
                let ty = read_type(reader, &param.ty)?;
                params.push(Param { flags, name, ty });
            }

            let return_type = Param { ty: read_type(reader, &sig.return_type)?, ..Default::default() };
            result.methods.push(Method { flags, name, params, return_type, attributes, ..Default::default() });
        }
    }

    for field in reader.type_def_fields(ty) {
        let flags = reader.field_flags(field);
        let name = reader.field_name(field).to_string();
        let ty = read_type(reader, &reader.field_type(field, Some(ty)))?;

        let value = if flags.contains(FieldAttributes::LITERAL) {
            let constant = reader.field_constant(field).unwrap();
            read_value(reader, &reader.constant_value(constant)).ok()
        } else {
            None
        };

        result.fields.push(Field { flags, name, ty, value });
    }

    Ok(result)
}

fn read_attributes(reader: &reader::Reader, attributes: impl Iterator<Item = reader::Attribute>) -> Result<Vec<Attribute>> {
    let mut result = vec![];

    for attribute in attributes {
        let ty = reader.attribute_type_name(attribute);
        let ty = TypeRef { namespace: ty.namespace.to_string(), name: ty.name.to_string(), generics: vec![] };
        let mut args = vec![];

        for (name, value) in reader.attribute_args(attribute) {
            args.push((name, read_value(reader, &value)?));
        }

        result.push(Attribute { ty, args });
    }

    Ok(result)
}

fn read_value(reader: &reader::Reader, value: &reader::Value) -> Result<Value> {
    match value {
        reader::Value::Bool(value) => Ok(Value::Bool(*value)),
        reader::Value::U8(value) => Ok(Value::U8(*value)),
        reader::Value::I8(value) => Ok(Value::I8(*value)),
        reader::Value::U16(value) => Ok(Value::U16(*value)),
        reader::Value::I16(value) => Ok(Value::I16(*value)),
        reader::Value::U32(value) => Ok(Value::U32(*value)),
        reader::Value::I32(value) => Ok(Value::I32(*value)),
        reader::Value::U64(value) => Ok(Value::U64(*value)),
        reader::Value::I64(value) => Ok(Value::I64(*value)),
        reader::Value::F32(value) => Ok(Value::F32(*value)),
        reader::Value::F64(value) => Ok(Value::F64(*value)),
        reader::Value::String(value) => Ok(Value::String(value.clone())),
        reader::Value::TypeDef(def) => Ok(Value::TypeName(format!("{}", reader.type_def_type_name(*def)))),
        reader::Value::TypeRef(code) => Ok(Value::TypeName(format!("{}", reader.type_def_or_ref(*code)))),
        reader::Value::EnumDef(def, value) => Ok(Value::Enum(format!("{}", reader.type_def_type_name(*def)), Box::new(read_value(reader, value)?))),
        reader::Value::EnumRef(code, value) => Ok(Value::Enum(format!("{}", reader.type_def_or_ref(*code)), Box::new(read_value(reader, value)?))),
    }
}

fn read_type(reader: &reader::Reader, ty: &reader::Type) -> Result<Type> {
    Ok(match ty {
        reader::Type::Void => Type::Void,
        reader::Type::Bool => Type::Bool,
        reader::Type::Char => Type::Char,
        reader::Type::I8 => Type::I8,
        reader::Type::U8 => Type::U8,
        reader::Type::I16 => Type::I16,
        reader::Type::U16 => Type::U16,
        reader::Type::I32 => Type::I32,
        reader::Type::U32 => Type::U32,
        reader::Type::I64 => Type::I64,
        reader::Type::U64 => Type::U64,
        reader::Type::F32 => Type::F32,
        reader::Type::F64 => Type::F64,
        reader::Type::ISize => Type::ISize,
        reader::Type::USize => Type::USize,
        reader::Type::String => Type::String,
        reader::Type::GUID => Type::GUID,
        reader::Type::IUnknown => Type::IUnknown,
        reader::Type::IInspectable => Type::IInspectable,
        reader::Type::HRESULT => Type::HRESULT,
        reader::Type::PSTR => Type::PSTR,
        reader::Type::PWSTR => Type::PWSTR,
        reader::Type::PCSTR => Type::PCSTR,
        reader::Type::PCWSTR => Type::PCWSTR,
        reader::Type::BSTR => Type::BSTR,
        reader::Type::TypeName => Type::TypeName,
        reader::Type::GenericParam(param) => Type::GenericParam(reader.generic_param_name(*param).to_string()),
        reader::Type::TypeDef((ty, reader_generics)) => {
            let mut generics = vec![];

            for ty in reader_generics {
                generics.push(read_type(reader, ty)?);
            }

            Type::TypeRef(TypeRef { namespace: reader.type_def_namespace(*ty).to_string(), name: reader.type_def_name(*ty).to_string(), generics })
        }
        reader::Type::TypeRef(code) => {
            let type_name = reader.type_def_or_ref(*code);
            Type::TypeRef(TypeRef { namespace: type_name.namespace.to_string(), name: type_name.name.to_string(), generics: vec![] })
        }
        reader::Type::MutPtr((ty, pointers)) => Type::MutPtr((Box::new(read_type(reader, ty)?), *pointers)),
        reader::Type::ConstPtr((ty, pointers)) => Type::ConstPtr((Box::new(read_type(reader, ty)?), *pointers)),
        reader::Type::Win32Array((ty, pointers)) => Type::Win32Array((Box::new(read_type(reader, ty)?), *pointers)),
        reader::Type::WinrtArray(ty) => Type::WinrtArray(Box::new(read_type(reader, ty)?)),
        reader::Type::WinrtArrayRef(ty) => Type::WinrtArrayRef(Box::new(read_type(reader, ty)?)),
        reader::Type::ConstRef(ty) => Type::ConstRef(Box::new(read_type(reader, ty)?)),
    })
}
