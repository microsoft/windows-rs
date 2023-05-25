use super::*;
use syn::spanned::Spanned;

pub fn read_idl(tree: &mut Module, paths: &[String], filter: &Filter) -> Result<()> {
    let mut files = vec![];

    for path in paths {
        if extension(path).1 == "idl" {
            files.push((path.as_str(), IdlFile::parse_str(&read_to_string(path)?).map_err(|error| error.with_path(path))?));
        }
    }

    for (path, file) in &files {
        read_file(tree, file, filter).map_err(|error| error.with_path(path))?;
    }

    Ok(())
}

fn read_file(tree: &mut Module, file: &IdlFile, filter: &Filter) -> Result<()> {
    for module in &file.modules {
        read_module(tree, file, module, &module.ident.to_string(), filter)?;
    }

    Ok(())
}

fn read_module(tree: &mut Module, file: &IdlFile, module: &IdlModule, namespace: &str, filter: &Filter) -> Result<()> {
    if filter.includes_namespace(namespace) {
        for member in &module.members {
            read_member(tree, file, member, namespace, filter)?;
        }
    }

    Ok(())
}

fn read_member(tree: &mut Module, file: &IdlFile, member: &IdlModuleMember, namespace: &str, filter: &Filter) -> Result<()> {
    match member {
        IdlModuleMember::Module(member) => read_module(tree, file, member, &format!("{namespace}.{}", member.ident), filter),
        IdlModuleMember::Interface(member) => read_interface(tree, file, member, namespace, filter),
        IdlModuleMember::Struct(member) => read_struct(tree, file, member, namespace, filter),
        IdlModuleMember::Enum(member) => read_enum(tree, file, member, namespace, filter),
        IdlModuleMember::Class(member) => read_class(tree, file, member, namespace, filter),
    }
}

fn read_interface(tree: &mut Module, _file: &IdlFile, ty: &IdlInterface, namespace: &str, filter: &Filter) -> Result<()> {
    let ident = ty.ident.to_string();

    if filter.includes_type_name(reader::TypeName::new(namespace, &ident)) {
        let mut def = TypeDef { extends: None, ..Default::default() };

        for method in &ty.methods {
            let name = method.sig.ident.to_string();
            let mut params = vec![];

            for input in &method.sig.inputs {
                let syn::FnArg::Typed(pat_type) = input else {
                        todo!();
                    };

                let syn::Pat::Ident(ref pat_ident) = *pat_type.pat else {
                        todo!();
                    };

                let name = pat_ident.ident.to_string();
                let ty = read_ty(namespace, &pat_type.ty)?;
                params.push(Param { name, ty, ..Default::default() });
            }

            let ty = if let syn::ReturnType::Type(_, ty) = &method.sig.output { read_ty(namespace, ty)? } else { Type::Void };
            let return_type = Param { ty, ..Default::default() };

            def.methods.push(Method { name, params, return_type, ..Default::default() });
        }

        tree.insert(namespace, 0).types.entry(ident).or_default().push(def);
    }

    Ok(())
}

fn read_struct(tree: &mut Module, _file: &IdlFile, ty: &IdlStruct, namespace: &str, filter: &Filter) -> Result<()> {
    let ident = ty.item.ident.to_string();

    if filter.includes_type_name(reader::TypeName::new(namespace, &ident)) {
        let mut def = TypeDef { extends: Some(TypeRef { namespace: "System".to_string(), name: "ValueType".to_string(), ..Default::default() }), ..Default::default() };

        let syn::Fields::Named(fields) = &ty.item.fields else {
                unimplemented!();
            };

        for field in &fields.named {
            let Some(ref ident) = field.ident else {
                    unimplemented!();
                };

            let flags = FieldAttributes::PUBLIC;
            let name = ident.to_string();
            let ty = read_ty(namespace, &field.ty)?;
            def.fields.push(Field { flags, name, ty, ..Default::default() });
        }

        tree.insert(namespace, 0).types.entry(ident).or_default().push(def);
    }

    Ok(())
}

fn read_enum(tree: &mut Module, _file: &IdlFile, ty: &IdlEnum, namespace: &str, filter: &Filter) -> Result<()> {
    let ident = ty.item.ident.to_string();

    if filter.includes_type_name(reader::TypeName::new(namespace, &ident)) {
        let mut def = TypeDef { extends: Some(TypeRef { namespace: "System".to_string(), name: "Enum".to_string(), ..Default::default() }), ..Default::default() };
        let enum_type = Type::TypeRef(TypeRef { namespace: namespace.to_string(), name: ident.clone(), ..Default::default() });

        for variant in &ty.item.variants {
            if let Some((_, expr)) = &variant.discriminant {
                let flags = FieldAttributes::PUBLIC;
                let name = variant.ident.to_string();
                let value = read_expr(expr, false)?;

                def.fields.push(Field { flags, name, ty: enum_type.clone(), value: Some(value) });
            }
        }

        tree.insert(namespace, 0).types.entry(ident).or_default().push(def);
    }

    Ok(())
}

fn read_class(tree: &mut Module, _file: &IdlFile, ty: &IdlClass, namespace: &str, filter: &Filter) -> Result<()> {
    let ident = ty.ident.to_string();

    if filter.includes_type_name(reader::TypeName::new(namespace, &ident)) {
        tree.insert(namespace, 0).types.entry(ident).or_default();
    }

    Ok(())
}

fn read_expr(expr: &syn::Expr, neg: bool) -> Result<Value> {
    match expr {
        syn::Expr::Lit(lit) => read_expr_lit(lit, neg),
        syn::Expr::Unary(unary) => read_expr_unary(unary),
        _ => todo!("{:?}", expr),
    }
}

fn read_expr_unary(unary: &syn::ExprUnary) -> Result<Value> {
    read_expr(&unary.expr, true)
}

fn read_expr_lit(expr: &syn::ExprLit, neg: bool) -> Result<Value> {
    read_lit(&expr.lit, neg)
}

fn read_lit(lit: &syn::Lit, neg: bool) -> Result<Value> {
    match lit {
        syn::Lit::Int(lit) => read_lit_int(lit, neg),
        syn::Lit::Str(lit) => read_lit_str(lit),
        _ => todo!("{:?}", lit),
    }
}

fn read_lit_str(lit: &syn::LitStr) -> Result<Value> {
    Ok(Value::String(lit.value()))
}

fn read_lit_int(lit: &syn::LitInt, neg: bool) -> Result<Value> {
    fn parse<T: std::str::FromStr>(lit: &syn::LitInt, neg: bool) -> Result<T> {
        let raw = if neg { format!("-{}", lit.base10_digits()) } else { lit.base10_digits().to_string() };
        raw.parse().map_err(|_| Error::new("failed to parse literal").with_span(lit.span()))
    }

    match lit.suffix() {
        "i8" => Ok(Value::I8(parse(lit, neg)?)),
        "u8" => Ok(Value::U8(parse(lit, neg)?)),
        "i16" => Ok(Value::I16(parse(lit, neg)?)),
        "u16" => Ok(Value::U16(parse(lit, neg)?)),
        "i32" => Ok(Value::I32(parse(lit, neg)?)),
        "u32" => Ok(Value::U32(parse(lit, neg)?)),
        "i64" => Ok(Value::I64(parse(lit, neg)?)),
        "u64" => Ok(Value::U64(parse(lit, neg)?)),
        suffix => todo!("suffix {:?}", suffix),
    }
}

fn read_ty(namespace: &str, ty: &syn::Type) -> Result<Type> {
    match ty {
        syn::Type::Path(ty) => read_type_path(namespace, ty),
        syn::Type::Ptr(ptr) => read_type_ptr(namespace, ptr),
        syn::Type::Array(array) => read_type_array(namespace, array),
        _ => unimplemented!(),
    }
}

fn read_type_array(namespace: &str, array: &syn::TypeArray) -> Result<Type> {
    let ty = read_ty(namespace, &array.elem)?;

    if let syn::Expr::Lit(lit) = &array.len {
        if let syn::Lit::Int(lit) = &lit.lit {
            return Ok(ty.into_array(lit.base10_parse()?));
        }
    }

    todo!()
}

fn read_type_ptr(namespace: &str, ptr: &syn::TypePtr) -> Result<Type> {
    let ty = read_ty(namespace, &ptr.elem)?;
    if ptr.mutability.is_some() {
        Ok(ty.into_mut_ptr())
    } else {
        Ok(ty.into_const_ptr())
    }
}

fn read_type_path(namespace: &str, ty: &syn::TypePath) -> Result<Type> {
    if ty.qself.is_some() {
        unimplemented!();
    }

    read_path(namespace, &ty.path)
}

fn read_path(current: &str, path: &syn::Path) -> Result<Type> {
    if let Some(segment) = path.segments.first() {
        if path.segments.len() == 1 {
            let name = segment.ident.to_string();

            return match name.as_str() {
                "void" => Ok(Type::Void),
                "bool" => Ok(Type::Bool),
                "char" => Ok(Type::Char),
                "i8" => Ok(Type::I8),
                "u8" => Ok(Type::U8),
                "i16" => Ok(Type::I16),
                "u16" => Ok(Type::U16),
                "i32" => Ok(Type::I32),
                "u32" => Ok(Type::U32),
                "i64" => Ok(Type::I64),
                "u64" => Ok(Type::U64),
                "f32" => Ok(Type::F32),
                "f64" => Ok(Type::F64),
                "isize" => Ok(Type::ISize),
                "usize" => Ok(Type::USize),
                "HSTRING" => Ok(Type::String),
                "GUID" => Ok(Type::GUID),
                "IUnknown" => Ok(Type::IUnknown),
                "IInspectable" => Ok(Type::IInspectable),
                "HRESULT" => Ok(Type::HRESULT),
                "PSTR" => Ok(Type::PSTR),
                "PWSTR" => Ok(Type::PWSTR),
                "PCSTR" => Ok(Type::PCSTR),
                "PCWSTR" => Ok(Type::PCWSTR),
                "BSTR" => Ok(Type::BSTR),
                _ => Ok(Type::TypeRef(TypeRef { namespace: current.to_string(), name, ..Default::default() })),
            };
        }
    }

    let mut current: Vec<String> = current.split('.').map(|segment| segment.to_string()).collect();
    let mut name = vec![];

    for segment in &path.segments {
        let segment = segment.ident.to_string();
        if segment == "super" {
            current.pop().ok_or_else(|| syn::Error::new(path.span(), "no parent module"))?;
        } else {
            name.append(&mut current);
            name.push(segment);
        }
    }

    let (last, rest) = name.split_last().ok_or_else(|| syn::Error::new(path.span(), "no type name"))?;

    Ok(Type::TypeRef(TypeRef { namespace: rest.join("."), name: last.to_string(), ..Default::default() }))
}
