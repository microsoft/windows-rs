use windows_bindgen as r;
use windows_ecma335::{writer as w, *};

fn main() {
    let time = std::time::Instant::now();

    let input = r::Reader::new(vec![
        r::File::new(std::fs::read("crates/libs/bindgen/default/Windows.winmd").unwrap()).unwrap(),
        r::File::new(std::fs::read("crates/libs/bindgen/default/Windows.Win32.winmd").unwrap())
            .unwrap(),
        r::File::new(std::fs::read("crates/libs/bindgen/default/Windows.Wdk.winmd").unwrap())
            .unwrap(),
    ]);

    let mut output = w::File::new("test");

    for ty in input.values().flat_map(|values| values.values()).flatten() {
        write_type(&mut output, ty);
    }

    let bytes = output.into_stream();
    std::fs::write("target/copy.winmd", bytes).unwrap();

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

fn write_type(output: &mut w::File, ty: &r::Type) {
    match ty {
        r::Type::Enum(ty) => write_def(output, ty.def, false),
        r::Type::Struct(ty) => write_def(output, ty.def, false),
        r::Type::Delegate(ty) => write_def(output, ty.def, true),
        r::Type::Interface(ty) => write_def(output, ty.def, true),
        r::Type::Class(ty) => write_def(output, ty.def, false),

        r::Type::CppEnum(ty) => write_def(output, ty.def, false),
        //r::Type::CppStruct(ty) => write_def(output, ty.def, false),
        _ => {}
    }
}

fn write_attributes<R: r::HasAttributes>(output: &mut w::File, parent: w::HasAttribute, row: R) {
    for attribute in row.attributes() {
        let ty = attribute.ty().parent();
        let attribute_ref = w::MemberRefParent::TypeRef(output.TypeRef(ty.namespace(), ty.name()));
        let args = attribute.args();

        let mut signature = attribute.ty().signature();
        let _call_flags = signature.read_u8();
        let param_count = signature.read_usize();
        let mut types = vec![];
        let _return_type = r::Type::from_blob(&mut signature, None, &[]);

        for _ in 0..param_count {
            types.push(convert_type(&r::Type::from_blob(&mut signature, None, &[])));
        }

        let signature = output.MethodDefSig(&types, &Type::Void, MethodCallAttributes::HASTHIS);
        let ctor = output.MemberRef(".ctor", signature, attribute_ref);

        let fixed: Vec<Value> = args
            .iter()
            .filter_map(|(name, value)| {
                if name.is_empty() {
                    Some(convert_value(value))
                } else {
                    None
                }
            })
            .collect();

        let named: Vec<(&str, Value)> = args
            .iter()
            .filter_map(|(name, value)| {
                if !name.is_empty() {
                    Some((*name, convert_value(value)))
                } else {
                    None
                }
            })
            .collect();

        let value = output.AttributeValue(&fixed, &named);
        output.Attribute(parent, w::AttributeType::MemberRef(ctor), value);
    }
}

fn write_def(output: &mut w::File, def: r::TypeDef, include_methods: bool) {
    let flags = TypeAttributes(def.flags().0);

    let extends = if let Some(extends) = def.extends() {
        w::TypeDefOrRef::TypeRef(output.TypeRef(extends.namespace(), extends.name()))
    } else {
        w::TypeDefOrRef::zeroed()
    };

    let type_def = output.TypeDef(def.namespace(), def.raw_name(), extends, flags);

    for field in def.fields() {
        let flags = FieldAttributes(field.flags().0);
        let signature = output.FieldSig(&convert_type(&field.ty(None)));

        let parent = output.Field(field.name(), signature, flags);

        if let Some(constant) = field.constant() {
            let value = convert_value(&constant.value());
            let ty = value.ty();
            let value = output.ConstantValue(&value);

            output.Constant(w::HasConstant::Field(parent), ty as u8, value);
        }
    }

    write_attributes(output, w::HasAttribute::TypeDef(type_def), def);

    let generics = def.generics();

    for def_interface in def.interface_impls() {
        let interface = convert_type(&def_interface.ty(&generics));

        let interface = if let Type::Name(tn) = interface {
            if tn.generics.is_empty() {
                w::TypeDefOrRef::TypeRef(output.TypeRef(&tn.namespace, &tn.name))
            } else {
                w::TypeDefOrRef::TypeSpec(output.TypeSpec(&tn.namespace, &tn.name, &tn.generics))
            }
        } else {
            panic!();
        };

        let interface_impl = output.InterfaceImpl(type_def, interface);

        write_attributes(
            output,
            w::HasAttribute::InterfaceImpl(interface_impl),
            def_interface,
        );
    }

    for generic in def.generic_params() {
        output.GenericParam(
            generic.name(),
            w::TypeOrMethodDef::TypeDef(type_def),
            generic.sequence(),
            generic.flags(),
        );
    }

    // Methods on classes is a huge overhead on .winmd size but adds no value as all of this information
    // is redundantly stored elsewhere.
    if include_methods {
        for method in def.methods() {
            let signature = method.signature("", &generics);
            let types: Vec<Type> = signature
                .params
                .iter()
                .map(|param| convert_type(&param.ty))
                .collect();
            let signature_blob = output.MethodDefSig(
                &types,
                &convert_type(&signature.return_type),
                MethodCallAttributes(signature.call_flags.0),
            );
            let flags = MethodAttributes(method.flags().0);
            let impl_flags = MethodImplAttributes(method.impl_flags().0);

            let method_def = output.MethodDef(method.name(), signature_blob, flags, impl_flags);

            if let Some(return_param) = signature.return_param {
                output.Param(
                    return_param.name(),
                    return_param.sequence(),
                    ParamAttributes(return_param.flags().0),
                );
            }

            for param in signature.params {
                output.Param(
                    param.def.name(),
                    param.def.sequence(),
                    ParamAttributes(param.def.flags().0),
                );
            }

            write_attributes(output, w::HasAttribute::MethodDef(method_def), method);
        }
    }
}

fn convert_type(input: &r::Type) -> Type {
    match input {
        r::Type::Void => Type::Void,
        r::Type::Bool => Type::Bool,
        r::Type::Char => Type::Char,
        r::Type::I8 => Type::I8,
        r::Type::U8 => Type::U8,
        r::Type::I16 => Type::I16,
        r::Type::U16 => Type::U16,
        r::Type::I32 => Type::I32,
        r::Type::U32 => Type::U32,
        r::Type::I64 => Type::I64,
        r::Type::U64 => Type::U64,
        r::Type::F32 => Type::F32,
        r::Type::F64 => Type::F64,
        r::Type::ISize => Type::ISize,
        r::Type::USize => Type::USize,
        r::Type::String => Type::String,
        r::Type::Object => Type::Object,
        r::Type::GUID => Type::named("System", "Guid"),
        r::Type::BOOL => Type::named("Windows.Win32.Foundation", "BOOL"),
        // TODO: Type::HRESULT is ambiguous... since it can refer to either the WinRT or Win32 HRESULT
        r::Type::HRESULT => Type::named("Windows.Foundation", "HResult"),
        r::Type::Array(ty) => Type::Array(Box::new(convert_type(ty))),
        r::Type::ArrayRef(ty) => Type::ArrayRef(Box::new(convert_type(ty))),
        r::Type::ConstRef(ty) => Type::ConstRef(Box::new(convert_type(ty))),
        r::Type::Enum(ty) => Type::named(ty.def.namespace(), ty.def.name()),
        r::Type::CppEnum(ty) => Type::named(ty.def.namespace(), ty.def.name()),
        r::Type::Struct(ty) => Type::named(ty.def.namespace(), ty.def.name()),
        r::Type::CppStruct(ty) => Type::named(ty.def.namespace(), ty.def.name()),
        r::Type::Class(ty) => Type::named(ty.def.namespace(), ty.def.name()),
        r::Type::Delegate(ty) => {
            convert_generic(ty.def.namespace(), ty.def.raw_name(), &ty.generics)
        }
        r::Type::CppDelegate(ty) => Type::named(ty.def.namespace(), ty.def.name()),
        r::Type::Interface(ty) => {
            convert_generic(ty.def.namespace(), ty.def.raw_name(), &ty.generics)
        }
        r::Type::Generic(name) => Type::Generic(name.sequence() as usize),
        r::Type::Type => Type::named("System", "Type"),
        r::Type::PtrMut(ty, pointers) => Type::PtrMut(Box::new(convert_type(ty)), *pointers),
        r::Type::PtrConst(ty, pointers) => Type::PtrConst(Box::new(convert_type(ty)), *pointers),
        r::Type::ArrayFixed(ty, len) => Type::ArrayFixed(Box::new(convert_type(ty)), *len),
        rest => panic!("{rest:?}"),
    }
}

fn convert_generic(namespace: &str, name: &str, generics: &[r::Type]) -> Type {
    Type::Name(TypeName {
        namespace: namespace.to_string(),
        name: name.to_string(),
        generics: generics.iter().map(|ty| convert_type(ty)).collect(),
    })
}

fn convert_value(value: &r::Value) -> Value {
    match value {
        r::Value::Bool(value) => Value::Bool(*value),
        r::Value::U8(value) => Value::U8(*value),
        r::Value::I8(value) => Value::I8(*value),
        r::Value::U16(value) => Value::U16(*value),
        r::Value::I16(value) => Value::I16(*value),
        r::Value::U32(value) => Value::U32(*value),
        r::Value::I32(value) => Value::I32(*value),
        r::Value::U64(value) => Value::U64(*value),
        r::Value::I64(value) => Value::I64(*value),
        r::Value::F32(value) => Value::F32(*value),
        r::Value::F64(value) => Value::F64(*value),
        r::Value::Str(value) => Value::Str(value),
        r::Value::TypeName(tn) => Value::TypeName(tn),
        rest => panic!("{rest:?}"),
    }
}
