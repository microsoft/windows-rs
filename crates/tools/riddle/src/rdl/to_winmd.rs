use super::*;
use crate::{rdl, winmd, Result};

// TODO: store span in winmd so that errors resolving type references can be traced back to file/line/column
use std::collections::HashMap;
//use syn::spanned::Spanned;

// TODO: this creates a temporary in-memory winmd used to treat the IDL content uniformly as metadata.
// The winmd_to_winmd does the harder job of validating and producing canonical winmd for public consumption.

pub fn rdl_to_winmd(file: &rdl::File) -> Result<Vec<u8>> {
    // Local-to-qualified type names found in use declaration - e.g. "IStringable" -> "Windows.Foundation.IStringable"
    // This is just a convenience for the developer to shorten common references like this but would not support globs or renames.
    // Note that none of these are verified to be real until much later when the winmd is validated since we don't
    // know what other metadata may be combined
    let mut _use_map = HashMap::<String, String>::new();

    // TODO: read file and populate use_map

    // Types are collected here in two passes - this allows us to figure out whether a local name points to a relative type
    // or a type from a use declaration...?
    let mut collector = HashMap::<String, HashMap<&str, rdl::ModuleMember>>::new();

    file.modules
        .iter()
        .for_each(|module| collect_module(&mut collector, module));

    // TODO: collect type names into hashmap (phase 1) and just drop clones of the IDL members into the collector

    // TODO: Can we just walk the collector at this point and populate the winmd writer and thus need the read-phase?
    // this second walking of the collector is basically the "define" phase

    let mut writer = winmd::Writer::new("temp.winmd");

    collector.iter().for_each(|(namespace, members)| {
        members
            .iter()
            .for_each(|(name, member)| write_member(&mut writer, namespace, name, member))
    });

    Ok(writer.into_stream())
}

fn collect_module<'a>(
    collector: &mut HashMap<String, HashMap<&'a str, rdl::ModuleMember>>,
    module: &'a rdl::Module,
) {
    module
        .members
        .iter()
        .for_each(|member| collect_member(collector, module, member));
}

fn collect_member<'a>(
    collector: &mut HashMap<String, HashMap<&'a str, rdl::ModuleMember>>,
    module: &'a rdl::Module,
    member: &'a rdl::ModuleMember,
) {
    match member {
        rdl::ModuleMember::Module(module) => collect_module(collector, module),
        rdl::ModuleMember::Constant(_) | rdl::ModuleMember::Function(_) => {
            collector
                .entry(module.namespace.to_string())
                .or_default()
                .entry("Apis")
                .or_insert(member.clone());
        }
        _ => {
            collector
                .entry(module.namespace.to_string())
                .or_default()
                .entry(member.name())
                .or_insert(member.clone());
        }
    }
}

fn write_member(
    writer: &mut winmd::Writer,
    namespace: &str,
    name: &str,
    member: &rdl::ModuleMember,
) {
    match member {
        rdl::ModuleMember::Interface(member) => write_interface(writer, namespace, name, member),
        rdl::ModuleMember::Struct(member) => write_struct(writer, namespace, name, member),
        rdl::ModuleMember::Enum(member) => write_enum(writer, namespace, name, member),
        rdl::ModuleMember::Class(member) => write_class(writer, namespace, name, member),
        rest => unimplemented!("{rest:?}"),
    }
}

fn write_interface(
    writer: &mut winmd::Writer,
    namespace: &str,
    name: &str,
    member: &rdl::Interface,
) {
    let mut flags = metadata::TypeAttributes::Public
        | metadata::TypeAttributes::Interface
        | metadata::TypeAttributes::Abstract;

    if member.winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime
    }

    writer.tables.TypeDef.push(winmd::TypeDef {
        Extends: 0,
        FieldList: 0,
        Flags: flags.0,
        MethodList: writer.tables.MethodDef.len() as u32,
        TypeName: writer.strings.insert(name),
        TypeNamespace: writer.strings.insert(namespace),
    });

    for method in &member.methods {
        let sig = syn_signature(namespace, &method.sig);
        let signature = writer.insert_method_sig(&sig);

        writer.tables.MethodDef.push(winmd::MethodDef {
            RVA: 0,
            ImplFlags: 0,
            Flags: 0,
            Name: writer.strings.insert(&method.sig.ident.to_string()),
            Signature: signature,
            ParamList: writer.tables.Param.len() as u32,
        });

        for (sequence, param) in sig.params.iter().enumerate() {
            writer.tables.Param.push(winmd::Param {
                Flags: 0,
                Sequence: (sequence + 1) as u16,
                Name: writer.strings.insert(&param.name),
            });
        }
    }
}

fn write_struct(writer: &mut winmd::Writer, namespace: &str, name: &str, member: &rdl::Struct) {
    let mut flags = metadata::TypeAttributes::Public
        | metadata::TypeAttributes::Sealed
        | metadata::TypeAttributes::Import
        | metadata::TypeAttributes::SequentialLayout;

    if member.winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime
    }

    let extends = writer.insert_type_ref("System", "ValueType");

    writer.tables.TypeDef.push(winmd::TypeDef {
        Extends: extends,
        FieldList: writer.tables.Field.len() as u32,
        Flags: flags.0,
        MethodList: 0,
        TypeName: writer.strings.insert(name),
        TypeNamespace: writer.strings.insert(namespace),
    });

    for field in &member.fields {
        let ty = syn_type(namespace, &field.ty);
        let signature = writer.insert_field_sig(&ty);

        writer.tables.Field.push(winmd::Field {
            Flags: 0,
            Name: writer.strings.insert(&field.name),
            Signature: signature,
        });
    }
}

fn write_enum(_writer: &mut winmd::Writer, _namespace: &str, _name: &str, _member: &rdl::Enum) {}

fn write_class(_writer: &mut winmd::Writer, _namespace: &str, _name: &str, _member: &rdl::Class) {}

fn syn_signature(namespace: &str, sig: &syn::Signature) -> winmd::Signature {
    let params = sig
        .inputs
        .iter()
        .map(|param| match param {
            syn::FnArg::Typed(pat_type) => {
                let name = match &*pat_type.pat {
                    syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                    rest => unimplemented!("{rest:?}"),
                };
                let ty = syn_type(namespace, &pat_type.ty);
                winmd::SignatureParam { name, ty }
            }
            rest => unimplemented!("{rest:?}"),
        })
        .collect();

    let return_type = if let syn::ReturnType::Type(_, ty) = &sig.output {
        syn_type(namespace, ty)
    } else {
        winmd::Type::Void
    };

    winmd::Signature {
        params,
        return_type,
        call_flags: 0,
    }
}

fn syn_type(namespace: &str, ty: &syn::Type) -> winmd::Type {
    match ty {
        syn::Type::Path(ty) => syn_type_path(namespace, ty),
        syn::Type::Ptr(ptr) => syn_type_ptr(namespace, ptr),
        syn::Type::Array(array) => syn_type_array(namespace, array),
        rest => unimplemented!("{rest:?}"),
    }
}

fn syn_type_array(namespace: &str, array: &syn::TypeArray) -> winmd::Type {
    let ty = syn_type(namespace, &array.elem);

    if let syn::Expr::Lit(lit) = &array.len {
        if let syn::Lit::Int(lit) = &lit.lit {
            if let Ok(len) = lit.base10_parse() {
                return ty.into_array(len);
            }
        }
    }

    unimplemented!()
}

fn syn_type_ptr(namespace: &str, ptr: &syn::TypePtr) -> winmd::Type {
    let ty = syn_type(namespace, &ptr.elem);
    if ptr.mutability.is_some() {
        ty.into_mut_ptr()
    } else {
        ty.into_const_ptr()
    }
}

fn syn_type_path(namespace: &str, ty: &syn::TypePath) -> winmd::Type {
    if ty.qself.is_none() {
        return syn_path(namespace, &ty.path);
    }

    unimplemented!()
}

fn syn_path(namespace: &str, path: &syn::Path) -> winmd::Type {
    if let Some(segment) = path.segments.first() {
        if path.segments.len() == 1 {
            let name = segment.ident.to_string();

            return match name.as_str() {
                "void" => winmd::Type::Void,
                "bool" => winmd::Type::Bool,
                "char" => winmd::Type::Char,
                "i8" => winmd::Type::I8,
                "u8" => winmd::Type::U8,
                "i16" => winmd::Type::I16,
                "u16" => winmd::Type::U16,
                "i32" => winmd::Type::I32,
                "u32" => winmd::Type::U32,
                "i64" => winmd::Type::I64,
                "u64" => winmd::Type::U64,
                "f32" => winmd::Type::F32,
                "f64" => winmd::Type::F64,
                "isize" => winmd::Type::ISize,
                "usize" => winmd::Type::USize,
                "HSTRING" => winmd::Type::String,
                "GUID" => winmd::Type::GUID,
                "IUnknown" => winmd::Type::IUnknown,
                "IInspectable" => winmd::Type::IInspectable,
                "HRESULT" => winmd::Type::HRESULT,
                "PSTR" => winmd::Type::PSTR,
                "PWSTR" => winmd::Type::PWSTR,
                "PCSTR" => winmd::Type::PCSTR,
                "PCWSTR" => winmd::Type::PCWSTR,
                "BSTR" => winmd::Type::BSTR,
                _ => winmd::Type::TypeRef(winmd::TypeName {
                    namespace: namespace.to_string(),
                    name,
                    generics: vec![],
                }),
            };
        }
    }

    // TODO: Here we assume that paths are absolute since there's no way to disambiguate between nested and absolute paths
    // The canonicalize function preprocesses the IDL to make this work

    let mut builder = vec![];

    for segment in &path.segments {
        let segment = segment.ident.to_string();
        builder.push(segment);
    }

    // Unwrapping as there are more one segments
    let (name, namespace) = builder.split_last().unwrap();
    let namespace = namespace.join(".");

    winmd::Type::TypeRef(winmd::TypeName {
        namespace,
        name: name.to_string(),
        generics: vec![],
    })
}
