use super::*;
use crate::winmd::{self, writer};
use crate::{rdl, Result};

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

    file.modules.iter().for_each(|module| collect_module(&mut collector, module));

    // TODO: collect type names into hashmap (phase 1) and just drop clones of the IDL members into the collector

    // TODO: Can we just walk the collector at this point and populate the winmd writer and thus need the read-phase?
    // this second walking of the collector is basically the "define" phase

    let mut writer = winmd::Writer::new("temp.winmd");

    collector.iter().for_each(|(namespace, members)| members.iter().for_each(|(name, member)| write_member(&mut writer, namespace, name, member)));

    Ok(writer.into_stream())
}

fn collect_module<'a>(collector: &mut HashMap<String, HashMap<&'a str, rdl::ModuleMember>>, module: &'a rdl::Module) {
    module.members.iter().for_each(|member| collect_member(collector, module, member));
}

fn collect_member<'a>(collector: &mut HashMap<String, HashMap<&'a str, rdl::ModuleMember>>, module: &'a rdl::Module, member: &'a rdl::ModuleMember) {
    match member {
        rdl::ModuleMember::Module(module) => collect_module(collector, module),
        rdl::ModuleMember::Constant(_) | rdl::ModuleMember::Function(_) => {
            collector.entry(module.namespace.to_string()).or_default().entry("Apis").or_insert(member.clone());
        }
        _ => {
            collector.entry(module.namespace.to_string()).or_default().entry(member.name()).or_insert(member.clone());
        }
    }
}

fn write_member(writer: &mut winmd::Writer, namespace: &str, name: &str, member: &rdl::ModuleMember) {
    match member {
        rdl::ModuleMember::Interface(member) => write_interface(writer, namespace, name, member),
        rdl::ModuleMember::Struct(member) => write_struct(writer, namespace, name, member),
        rdl::ModuleMember::Enum(member) => write_enum(writer, namespace, name, member),
        rdl::ModuleMember::Class(member) => write_class(writer, namespace, name, member),
        rest => unimplemented!("{rest:?}"),
    }
}

fn write_interface(writer: &mut winmd::Writer, namespace: &str, name: &str, member: &rdl::Interface) {
    let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Interface | metadata::TypeAttributes::Abstract;

    if member.winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime
    }

    writer.tables.TypeDef.push(winmd::TypeDef {
        Extends: 0,
        FieldList: writer.tables.Field.len() as u32,
        MethodList: writer.tables.MethodDef.len() as u32,
        Flags: flags.0,
        TypeName: writer.strings.insert(name),
        TypeNamespace: writer.strings.insert(namespace),
    });

    for (number, generic) in member.generics.iter().enumerate() {
        writer.tables.GenericParam.push(writer::GenericParam {
            Number: number as u16,
            Flags: 0,
            Owner: writer::TypeOrMethodDef::TypeDef(writer.tables.TypeDef.len() as u32 - 1).encode(),
            Name: writer.strings.insert(generic),
        });
    }

    for type_path in &member.extends {
        let ty = syn_type_path(namespace, &member.generics, type_path);

        let reference = match &ty {
            winmd::Type::TypeRef(type_name) if type_name.generics.is_empty() => writer.insert_type_ref(&type_name.namespace, &type_name.name),
            winmd::Type::TypeRef(_) => writer.insert_type_spec(ty),
            rest => unimplemented!("{rest:?}"),
        };

        writer.tables.InterfaceImpl.push(writer::InterfaceImpl { Class: writer.tables.TypeDef.len() as u32 - 1, Interface: reference });
    }

    for method in &member.methods {
        let signature = syn_signature(namespace, &member.generics, &method.sig);

        let params: Vec<winmd::Type> = signature.params.iter().map(|param| param.ty.clone()).collect();

        let signature_blob = writer.insert_method_sig(metadata::MethodCallAttributes(0), &signature.return_type, &params);

        let flags = metadata::MethodAttributes::Abstract | metadata::MethodAttributes::HideBySig | metadata::MethodAttributes::HideBySig | metadata::MethodAttributes::NewSlot | metadata::MethodAttributes::Public | metadata::MethodAttributes::Virtual;

        writer.tables.MethodDef.push(winmd::MethodDef {
            RVA: 0,
            ImplFlags: 0,
            Flags: flags.0,
            Name: writer.strings.insert(&method.sig.ident.to_string()),
            Signature: signature_blob,
            ParamList: writer.tables.Param.len() as u32,
        });

        for (sequence, param) in signature.params.iter().enumerate() {
            writer.tables.Param.push(winmd::Param { Flags: 0, Sequence: (sequence + 1) as u16, Name: writer.strings.insert(&param.name) });
        }
    }
}

fn write_struct(writer: &mut winmd::Writer, namespace: &str, name: &str, member: &rdl::Struct) {
    let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed | metadata::TypeAttributes::SequentialLayout;

    if member.winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime
    }

    let extends = writer.insert_type_ref("System", "ValueType");

    writer.tables.TypeDef.push(winmd::TypeDef {
        Extends: extends,
        FieldList: writer.tables.Field.len() as u32,
        MethodList: writer.tables.MethodDef.len() as u32,
        Flags: flags.0,
        TypeName: writer.strings.insert(name),
        TypeNamespace: writer.strings.insert(namespace),
    });

    for field in &member.fields {
        let flags = metadata::FieldAttributes::Public;
        let ty = syn_type(namespace, &[], &field.ty);
        let signature = writer.insert_field_sig(&ty);

        writer.tables.Field.push(winmd::Field { Flags: flags.0, Name: writer.strings.insert(&field.name), Signature: signature });
    }
}

fn write_enum(_writer: &mut winmd::Writer, _namespace: &str, _name: &str, _member: &rdl::Enum) {}

fn write_class(writer: &mut winmd::Writer, namespace: &str, name: &str, member: &rdl::Class) {
    let flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed | metadata::TypeAttributes::WindowsRuntime;

    let extends = if let Some(base) = &member.base {
        match syn_type_path(namespace, &[], base) {
            winmd::Type::TypeRef(base) => writer.insert_type_ref(&base.namespace, &base.name),
            rest => unimplemented!("{rest:?}"),
        }
    } else {
        writer.insert_type_ref("System", "Object")
    };

    writer.tables.TypeDef.push(winmd::TypeDef {
        Extends: extends,
        // Even though ECMA-335 says these can be "null", bugs in ILDASM necessitate this to avoid "misreading" the list terminators.
        FieldList: writer.tables.Field.len() as u32,
        MethodList: writer.tables.MethodDef.len() as u32,
        Flags: flags.0,
        TypeName: writer.strings.insert(name),
        TypeNamespace: writer.strings.insert(namespace),
    });

    for (index, extends) in member.extends.iter().enumerate() {
        let ty = syn_type_path(namespace, &[], extends);

        let reference = match &ty {
            winmd::Type::TypeRef(type_name) if type_name.generics.is_empty() => writer.insert_type_ref(&type_name.namespace, &type_name.name),
            winmd::Type::TypeRef(_) => writer.insert_type_spec(ty),
            winmd::Type::IUnknown => writer.insert_type_ref("Windows.Win32.System.Com", "IUnknown"),
            winmd::Type::IInspectable => writer.insert_type_ref("Windows.Win32.System.WinRT", "IInspectable"),
            rest => unimplemented!("{rest:?}"),
        };

        writer.tables.InterfaceImpl.push(writer::InterfaceImpl { Class: writer.tables.TypeDef.len() as u32 - 1, Interface: reference });

        if index == 0 {
            // TODO: add the DefaultAttribute to the first interface
        }
    }
}

fn syn_signature(namespace: &str, generics: &[String], sig: &syn::Signature) -> winmd::Signature {
    let params = sig
        .inputs
        .iter()
        .map(|param| match param {
            syn::FnArg::Typed(pat_type) => {
                let name = match &*pat_type.pat {
                    syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                    rest => unimplemented!("{rest:?}"),
                };
                let ty = syn_type(namespace, generics, &pat_type.ty);
                winmd::SignatureParam { name, ty }
            }
            rest => unimplemented!("{rest:?}"),
        })
        .collect();

    let return_type = if let syn::ReturnType::Type(_, ty) = &sig.output { syn_type(namespace, generics, ty) } else { winmd::Type::Void };

    winmd::Signature { params, return_type, call_flags: 0 }
}

fn syn_type(namespace: &str, generics: &[String], ty: &syn::Type) -> winmd::Type {
    match ty {
        syn::Type::Path(ty) => syn_type_path(namespace, generics, ty),
        syn::Type::Ptr(ptr) => syn_type_ptr(namespace, ptr),
        syn::Type::Array(array) => syn_type_array(namespace, array),
        rest => unimplemented!("{rest:?}"),
    }
}

fn syn_type_array(namespace: &str, array: &syn::TypeArray) -> winmd::Type {
    let ty = syn_type(namespace, &[], &array.elem);

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
    let ty = syn_type(namespace, &[], &ptr.elem);
    if ptr.mutability.is_some() {
        ty.into_mut_ptr()
    } else {
        ty.into_const_ptr()
    }
}

fn syn_type_path(namespace: &str, generics: &[String], ty: &syn::TypePath) -> winmd::Type {
    if ty.qself.is_none() {
        return syn_path(namespace, generics, &ty.path);
    }

    unimplemented!()
}

fn syn_path(namespace: &str, generics: &[String], path: &syn::Path) -> winmd::Type {
    if let Some(segment) = path.segments.first() {
        if path.segments.len() == 1 && segment.arguments.is_empty() {
            let name = segment.ident.to_string();

            if let Some(number) = generics.iter().position(|generic| generic == &name) {
                return winmd::Type::GenericParam(number as u16);
            }

            match name.as_str() {
                "void" => return winmd::Type::Void,
                "bool" => return winmd::Type::Bool,
                "char" => return winmd::Type::Char,
                "i8" => return winmd::Type::I8,
                "u8" => return winmd::Type::U8,
                "i16" => return winmd::Type::I16,
                "u16" => return winmd::Type::U16,
                "i32" => return winmd::Type::I32,
                "u32" => return winmd::Type::U32,
                "i64" => return winmd::Type::I64,
                "u64" => return winmd::Type::U64,
                "f32" => return winmd::Type::F32,
                "f64" => return winmd::Type::F64,
                "isize" => return winmd::Type::ISize,
                "usize" => return winmd::Type::USize,
                "HSTRING" => return winmd::Type::String,
                "GUID" => return winmd::Type::GUID,
                "IUnknown" => return winmd::Type::IUnknown,
                "IInspectable" => return winmd::Type::IInspectable,
                "HRESULT" => return winmd::Type::HRESULT,
                "PSTR" => return winmd::Type::PSTR,
                "PWSTR" => return winmd::Type::PWSTR,
                "PCSTR" => return winmd::Type::PCSTR,
                "PCWSTR" => return winmd::Type::PCWSTR,
                "BSTR" => return winmd::Type::BSTR,
                _ => {}
            };
        }
    }

    // TODO: Here we assume that paths are absolute since there's no way to disambiguate between nested and absolute paths
    // The canonicalize function (should maybe) preprocesses the IDL to make this work

    let mut builder = vec![];

    for segment in &path.segments {
        let segment = segment.ident.to_string();

        if segment == "super" {
            if builder.is_empty() {
                for segment in namespace.split('.') {
                    builder.push(segment.to_string());
                }
            }
            builder.pop();
        } else {
            builder.push(segment);
        }
    }

    // Unwrapping is fine as there should always be at least one segment.
    let (name, type_namespace) = builder.split_last().unwrap();
    let type_namespace = if type_namespace.is_empty() { namespace.to_string() } else { type_namespace.join(".") };
    let mut type_generics = vec![];

    if let Some(segment) = path.segments.last() {
        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
            for arg in &args.args {
                match arg {
                    syn::GenericArgument::Type(ty) => type_generics.push(syn_type(namespace, generics, ty)),
                    rest => unimplemented!("{rest:?}"),
                }
            }
        }
    }

    winmd::Type::TypeRef(winmd::TypeName { namespace: type_namespace, name: name.to_string(), generics: type_generics })
}
