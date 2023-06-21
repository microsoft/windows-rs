use crate::{idl, winmd, Result};

// TODO: store span in winmd so that errors resolving type references can be traced back to file/line/column
use std::collections::HashMap;
//use syn::spanned::Spanned;

// TODO: this creates a temporary in-memory winmd used to treat the IDL content uniformly as metadata.
// The winmd_to_winmd does the harder job of validating and producing canonical winmd for public consumption.

pub fn idl_to_winmd(file: &idl::File) -> Result<Vec<u8>> {
    // Local-to-qualified type names found in use declaration - e.g. "IStringable" -> "Windows.Foundation.IStringable"
    // This is just a convenience for the developer to shorten common references like this but would not support globs or renames.
    // Note that none of these are verified to be real until much later when the winmd is validated since we don't
    // know what other metadata may be combined
    let mut _use_map = HashMap::<String, String>::new();

    // TODO: read file and populate use_map

    // Types are collected here in two passes - this allows us to figure out whether a local name points to a relative type
    // or a type from a use declaration...?
    let mut collector = HashMap::<String, HashMap<&str, idl::ModuleMember>>::new();

    file.modules
        .iter()
        .for_each(|module| collect_module(&mut collector, module, &module.name));

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
    collector: &mut HashMap<String, HashMap<&'a str, idl::ModuleMember>>,
    module: &'a idl::Module,
    namespace: &str,
) {
    module
        .members
        .iter()
        .for_each(|member| collect_member(collector, member, namespace));
}

fn collect_member<'a>(
    collector: &mut HashMap<String, HashMap<&'a str, idl::ModuleMember>>,
    member: &'a idl::ModuleMember,
    namespace: &str,
) {
    match member {
        idl::ModuleMember::Module(module) => {
            collect_module(collector, module, &format!("{namespace}.{}", module.name))
        }
        _ => {
            _ = collector
                .entry(namespace.to_string())
                .or_default()
                .entry(member.name())
                .or_insert(member.clone())
        }
    }
}

fn write_member(
    writer: &mut winmd::Writer,
    namespace: &str,
    name: &str,
    member: &idl::ModuleMember,
) {
    match member {
        idl::ModuleMember::Interface(member) => write_interface(writer, namespace, name, member),
        idl::ModuleMember::Struct(member) => write_struct(writer, namespace, name, member),
        idl::ModuleMember::Enum(member) => write_enum(writer, namespace, name, member),
        idl::ModuleMember::Class(member) => write_class(writer, namespace, name, member),
        idl::ModuleMember::Module(_) => {} // modules have already been flattened but rustc doesn't know this
    }
}

fn write_interface(
    writer: &mut winmd::Writer,
    namespace: &str,
    name: &str,
    _member: &idl::Interface,
) {
    let flags = metadata::TypeAttributes::Public
        | metadata::TypeAttributes::Interface
        | metadata::TypeAttributes::WindowsRuntime
        | metadata::TypeAttributes::Abstract;

    writer.tables.TypeDef.push(winmd::TypeDef {
        Extends: 0,
        FieldList: 0,
        Flags: flags.0,
        MethodList: writer.tables.MethodDef.len() as _,
        TypeName: writer.strings.insert(name),
        TypeNamespace: writer.strings.insert(namespace),
    });
}

fn write_struct(writer: &mut winmd::Writer, namespace: &str, name: &str, member: &idl::Struct) {
    let flags = metadata::TypeAttributes::Public
        | metadata::TypeAttributes::WindowsRuntime
        | metadata::TypeAttributes::Sealed
        | metadata::TypeAttributes::Import
        | metadata::TypeAttributes::SequentialLayout;

    let extends = writer.insert_type_ref("System", "ValueType");

    writer.tables.TypeDef.push(winmd::TypeDef {
        Extends: extends,
        FieldList: writer.tables.Field.len() as _,
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

fn write_enum(_writer: &mut winmd::Writer, _namespace: &str, _name: &str, _member: &idl::Enum) {}

fn write_class(_writer: &mut winmd::Writer, _namespace: &str, _name: &str, _member: &idl::Class) {}

// fn idl_interface(writer:  &mut winmd::Writer, _file: &idl::File, ty: &idl::Interface, namespace: &str, phase: ReadPhase) -> Result<()> {
//     let ident = ty.ident.to_string();

//         match phase {
//             ReadPhase::Index => {
//                 self.insert(namespace, 0).types.entry(ident).or_default();
//             }
//             ReadPhase::Define => {
//                 let flags = TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::WindowsRuntime | TypeAttributes::Abstract;
//                 let mut def = TypeDef { flags, extends: None, ..Default::default() };

//                 for method in &ty.methods {
//                     let name = method.sig.ident.to_string();
//                     let mut params = vec![];

//                     for input in &method.sig.inputs {
//                         let syn::FnArg::Typed(pat_type) = input else {
//                     unimplemented!();
//                 };

//                         let syn::Pat::Ident(ref pat_ident) = *pat_type.pat else {
//                     unimplemented!();
//                 };

//                         let name = pat_ident.ident.to_string();
//                         let ty = self.read_ty(namespace, &pat_type.ty)?;
//                         params.push(Param { name, ty, ..Default::default() });
//                     }

//                     let ty = if let syn::ReturnType::Type(_, ty) = &method.sig.output { self.read_ty(namespace, ty)? } else { Type::Void };
//                     let return_type = Param { ty, ..Default::default() };
//                     let flags = MethodAttributes::Public;

//                     def.methods.push(Method { flags, name, params, return_type, ..Default::default() });
//                 }

//                 self.insert(namespace, 0).types.entry(ident).or_default().push(def);
//             }
//         }

//     Ok(())
// }

// fn idl_struct(writer:  &mut winmd::Writer, _file: &idl::File, ty: &idl::Struct, namespace: &str, phase: ReadPhase) -> Result<()> {
//     let ident = ty.item.ident.to_string();

//         match phase {
//             ReadPhase::Index => {
//                 self.insert(namespace, 0).types.entry(ident).or_default();
//             }
//             ReadPhase::Define => {
//                 let flags = TypeAttributes::Public | TypeAttributes::WindowsRuntime | TypeAttributes::Sealed | TypeAttributes::Import | TypeAttributes::SequentialLayout;
//                 let mut def = TypeDef { flags, extends: Some(TypeRef { namespace: "System".to_string(), name: "ValueType".to_string(), ..Default::default() }), ..Default::default() };

//                 let syn::Fields::Named(fields) = &ty.item.fields else {
//                     unimplemented!();
//                 };

//                 for field in &fields.named {
//                     let Some(ref ident) = field.ident else {
//                        unimplemented!();
//                     };

//                     let flags = FieldAttributes::Public;
//                     let name = ident.to_string();
//                     let ty = self.read_ty(namespace, &field.ty)?;
//                     def.fields.push(Field { flags, name, ty, ..Default::default() });
//                 }

//                 self.insert(namespace, 0).types.entry(ident).or_default().push(def);
//             }
//         }

//     Ok(())
// }

// fn idl_enum(writer:  &mut winmd::Writer, _file: &idl::File, ty: &idl::Enum, namespace: &str, phase: ReadPhase) -> Result<()> {
//     let ident = ty.item.ident.to_string();

//         match phase {
//             ReadPhase::Index => {
//                 self.insert(namespace, 0).types.entry(ident).or_default();
//             }
//             ReadPhase::Define => {
//                 let mut def = TypeDef { extends: Some(TypeRef { namespace: "System".to_string(), name: "Enum".to_string(), ..Default::default() }), ..Default::default() };
//                 let enum_type = Type::TypeRef(TypeRef { namespace: namespace.to_string(), name: ident.clone(), ..Default::default() });

//                 for variant in &ty.item.variants {
//                     if let Some((_, expr)) = &variant.discriminant {
//                         let flags = FieldAttributes::Public;
//                         let name = variant.ident.to_string();
//                         let value = self.read_expr(expr, false)?;

//                         def.fields.push(Field { flags, name, ty: enum_type.clone(), value: Some(value) });
//                     }
//                 }

//                 self.insert(namespace, 0).types.entry(ident).or_default().push(def);
//             }
//         }

//     Ok(())
// }

// fn idl_class(writer:  &mut winmd::Writer, _file: &idl::File, ty: &idl::Class, namespace: &str, _phase: ReadPhase) -> Result<()> {
//     let ident = ty.ident.to_string();
//     self.insert(namespace, 0).types.entry(ident).or_default();
//     Ok(())
// }

// fn syn_expr(writer:  &mut winmd::Writer, expr: &syn::Expr, neg: bool) -> Result<Value> {
//     match expr {
//         syn::Expr::Lit(lit) => self.read_expr_lit(lit, neg),
//         syn::Expr::Unary(unary) => self.read_expr_unary(unary),
//         rest => unimplemented!("{rest:?}"),
//     }
// }

// fn syn_expr_unary(writer:  &mut winmd::Writer, unary: &syn::ExprUnary) -> Result<Value> {
//     self.read_expr(&unary.expr, true)
// }

// fn syn_expr_lit(writer:  &mut winmd::Writer, expr: &syn::ExprLit, neg: bool) -> Result<Value> {
//     self.read_lit(&expr.lit, neg)
// }

// fn syn_lit(writer:  &mut winmd::Writer, lit: &syn::Lit, neg: bool) -> Result<Value> {
//     match lit {
//         syn::Lit::Int(lit) => self.read_lit_int(lit, neg),
//         syn::Lit::Str(lit) => self.read_lit_str(lit),
//         rest => unimplemented!("{rest:?}"),
//     }
// }

// fn syn_lit_str(writer:  &mut winmd::Writer, lit: &syn::LitStr) -> Result<Value> {
//     Ok(Value::String(lit.value()))
// }

// fn syn_lit_int(writer:  &mut winmd::Writer, lit: &syn::LitInt, neg: bool) -> Result<Value> {
//     fn parse<T: std::str::FromStr>(lit: &syn::LitInt, neg: bool) -> Result<T> {
//         let raw = if neg { format!("-{}", lit.base10_digits()) } else { lit.base10_digits().to_string() };
//         raw.parse().map_err(|_| Error::new("failed to parse literal").with_span(lit.span()))
//     }

//     match lit.suffix() {
//         "i8" => Ok(Value::I8(parse(lit, neg)?)),
//         "u8" => Ok(Value::U8(parse(lit, neg)?)),
//         "i16" => Ok(Value::I16(parse(lit, neg)?)),
//         "u16" => Ok(Value::U16(parse(lit, neg)?)),
//         "i32" => Ok(Value::I32(parse(lit, neg)?)),
//         "u32" => Ok(Value::U32(parse(lit, neg)?)),
//         "i64" => Ok(Value::I64(parse(lit, neg)?)),
//         "u64" => Ok(Value::U64(parse(lit, neg)?)),
//         suffix => unimplemented!("suffix {:?}", suffix),
//     }
// }

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
