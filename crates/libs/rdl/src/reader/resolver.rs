use super::*;

pub struct Resolver<'a, 'input> {
    pub index: &'a Index<'input>,
    pub reference: &'a metadata::reader::Index,
    pub file: &'a File,
    pub namespace: &'a str,
    pub generics: &'a [String],
}

impl Resolver<'_, '_> {
    fn error<S: Spanned>(&self, spanned: S, message: &str) -> Error {
        let start = spanned.span().start();
        Error::new(message, &self.file.source, start.line, start.column)
    }

    pub(super) fn err<T, S: Spanned>(&self, spanned: S, message: &str) -> Result<T, Error> {
        Err(self.error(spanned, message))
    }

    pub fn resolve_type(&self, ty: &syn::Type) -> Result<metadata::Type, Error> {
        match ty {
            syn::Type::Path(ty) => self.resolve_path(&ty.path),
            syn::Type::Ptr(ty) => self.resolve_type_ptr(ty),
            syn::Type::Reference(ty) => self.resolve_type_reference(ty),
            syn::Type::Slice(ty) => Ok(metadata::Type::Array(Box::new(
                self.resolve_type(&ty.elem)?,
            ))),
            syn::Type::Array(ty) => Ok(metadata::Type::ArrayFixed(
                Box::new(self.resolve_type(&ty.elem)?),
                self.resolve_array_len(&ty.len)?,
            )),
            rest => self.err(rest, "type not supported"),
        }
    }

    pub fn resolve_type_in_attr_ns(
        &self,
        attr_ns: &str,
        ty: &syn::Type,
    ) -> Result<metadata::Type, Error> {
        if attr_ns == self.namespace {
            return self.resolve_type(ty);
        }

        if let syn::Type::Path(type_path) = ty
            && type_path.qself.is_none()
            && type_path.path.leading_colon.is_none()
        {
            let segments: Vec<String> = type_path
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect();

            if !segments.is_empty() && !segments.iter().any(|segment| segment == "super") {
                let name = segments.last().unwrap();
                let namespace = if segments.len() == 1 {
                    attr_ns.to_string()
                } else {
                    format!("{}.{}", attr_ns, segments[..segments.len() - 1].join("."))
                };

                if let Some(ty) = self.make_type(&namespace, name, vec![]) {
                    return Ok(ty);
                }
            }
        }

        self.resolve_type(ty)
    }

    pub fn resolve_path(&self, ty: &syn::Path) -> Result<metadata::Type, Error> {
        let mut path = vec![];

        for segment in &ty.segments {
            if segment.ident == "super" {
                if path.is_empty() {
                    path.extend(self.namespace.split('.').map(str::to_string));
                }

                if path.pop().is_none() {
                    return self.err(ty, "too many leading `super` keywords");
                }
            } else {
                path.push(segment.ident.to_string());
            }
        }

        let mut generics = vec![];
        if let Some(last) = ty.segments.last()
            && let syn::PathArguments::AngleBracketed(arguments) = &last.arguments
        {
            for argument in &arguments.args {
                if let syn::GenericArgument::Type(ty) = argument {
                    generics.push(self.resolve_type(ty)?);
                }
            }
        }

        if path.len() == 1 {
            if let Some(number) = self.generics.iter().position(|generic| *generic == path[0]) {
                return Ok(metadata::Type::Generic(
                    path[0].clone(),
                    number.try_into().unwrap(),
                ));
            }

            match path[0].as_str() {
                "bool" => return Ok(metadata::Type::Bool),
                "i8" => return Ok(metadata::Type::I8),
                "u8" => return Ok(metadata::Type::U8),
                "i16" => return Ok(metadata::Type::I16),
                "u16" => return Ok(metadata::Type::U16),
                "i32" => return Ok(metadata::Type::I32),
                "u32" => return Ok(metadata::Type::U32),
                "i64" => return Ok(metadata::Type::I64),
                "u64" => return Ok(metadata::Type::U64),
                "f32" => return Ok(metadata::Type::F32),
                "f64" => return Ok(metadata::Type::F64),
                "isize" => return Ok(metadata::Type::ISize),
                "usize" => return Ok(metadata::Type::USize),
                "void" => return Ok(metadata::Type::Void),
                "String" => return Ok(metadata::Type::String),
                "Object" => return Ok(metadata::Type::Object),
                "Char16" => return Ok(metadata::Type::Char),
                _ => {}
            }
        }

        let (name, namespace) = path.split_last().unwrap();
        let namespace = if namespace.is_empty() {
            self.namespace.to_string()
        } else {
            namespace.join(".")
        };

        if let Some(resolved) = self.make_type(&namespace, name, generics.clone()) {
            return self.finish_path_type(ty, resolved);
        }

        let namespace = format!("{}.{}", self.namespace, namespace);
        if let Some(resolved) = self.make_type(&namespace, name, generics.clone()) {
            return self.finish_path_type(ty, resolved);
        }

        let mut imported = vec![];
        if path.len() == 1 {
            for import in &self.file.imports {
                if import.glob || import.local.as_deref() != Some(name) {
                    continue;
                }
                let (target_name, target_namespace) = import.path.split_last().unwrap();
                if let Some(candidate) =
                    self.make_type(&target_namespace.join("."), target_name, generics.clone())
                {
                    push_import_candidate(&mut imported, import, candidate);
                }
            }
        } else if !ty.segments.iter().any(|segment| segment.ident == "super") {
            for import in &self.file.imports {
                if import.glob || import.local.as_deref() != Some(&path[0]) {
                    continue;
                }
                let target = import.path.join(".");
                if !self.namespace_exists(&target) {
                    continue;
                }
                let mut namespace = import.path.clone();
                namespace.extend_from_slice(&path[1..path.len() - 1]);
                if let Some(candidate) =
                    self.make_type(&namespace.join("."), name, generics.clone())
                {
                    push_import_candidate(&mut imported, import, candidate);
                }
            }
        }
        if let Some(resolved) = self.one_imported_type(ty, name, imported)? {
            return self.finish_path_type(ty, resolved);
        }

        let mut globbed = vec![];
        for import in &self.file.imports {
            if import.glob
                && let Some(candidate) =
                    self.make_type(&import.path.join("."), name, generics.clone())
            {
                push_import_candidate(&mut globbed, import, candidate);
            }
        }
        if let Some(resolved) = self.one_imported_type(ty, name, globbed)? {
            return self.finish_path_type(ty, resolved);
        }

        if ty.segments.len() == 1 {
            match name.as_str() {
                "Type" => {
                    return self
                        .finish_path_type(ty, metadata::Type::class_named("System", "Type"));
                }
                "GUID" => {
                    return self
                        .finish_path_type(ty, metadata::Type::value_named("System", "Guid"));
                }
                "HRESULT" => {
                    return self.finish_path_type(
                        ty,
                        metadata::Type::value_named("Windows.Foundation", "HResult"),
                    );
                }
                _ => {}
            }
        }

        Err(self.error(ty, "type not found"))
    }

    pub fn namespace_exists(&self, namespace: &str) -> bool {
        self.index.namespaces.contains_key(namespace)
            || self.reference.contains_namespace(namespace)
    }

    fn make_type(
        &self,
        namespace: &str,
        name: &str,
        generics: Vec<metadata::Type>,
    ) -> Option<metadata::Type> {
        if !self.index.contains(namespace, name) && !self.reference.contains(namespace, name) {
            return None;
        }

        let type_name = metadata::TypeName {
            namespace: namespace.to_string(),
            name: name.to_string(),
            generics,
        };
        if self.type_is_value(namespace, name) {
            Some(metadata::Type::ValueName(type_name))
        } else {
            Some(metadata::Type::ClassName(type_name))
        }
    }

    fn finish_path_type(
        &self,
        path: &syn::Path,
        ty: metadata::Type,
    ) -> Result<metadata::Type, Error> {
        let (metadata::Type::ValueName(type_name) | metadata::Type::ClassName(type_name)) = &ty
        else {
            return Ok(ty);
        };
        let expected = self
            .index
            .generic_arity(&type_name.namespace, &type_name.name)
            .or_else(|| {
                self.reference
                    .get(&type_name.namespace, &type_name.name)
                    .next()
                    .map(|def| def.generic_params().count())
            })
            .unwrap_or(0);
        let actual = type_name.generics.len();
        if expected == actual {
            return Ok(ty);
        }

        let start = path.span().start();
        let end = path.span().end();
        Err(Error::new(
            &format!(
                "type `{}.{}` expects {expected} generic arguments but {actual} were provided",
                type_name.namespace, type_name.name
            ),
            &self.file.source,
            start.line,
            start.column,
        )
        .with_code("RDL0005")
        .with_primary_label(
            Label::primary(&self.file.source, start.line, start.column)
                .with_end(end.line, end.column)
                .with_message("wrong number of generic arguments"),
        ))
    }

    fn type_is_value(&self, namespace: &str, name: &str) -> bool {
        self.index.is_value_type(namespace, name)
            || self
                .reference
                .get(namespace, name)
                .next()
                .is_some_and(|def| {
                    matches!(
                        def.category(),
                        metadata::reader::TypeCategory::Struct
                            | metadata::reader::TypeCategory::Enum
                    )
                })
    }

    fn resolve_type_reference(&self, ty: &syn::TypeReference) -> Result<metadata::Type, Error> {
        let is_mut = ty.mutability.is_some();
        let resolved = self.resolve_type(&ty.elem)?;
        if is_mut {
            Ok(metadata::Type::RefMut(Box::new(resolved)))
        } else {
            Ok(metadata::Type::RefConst(Box::new(resolved)))
        }
    }

    fn resolve_type_ptr(&self, ty: &syn::TypePtr) -> Result<metadata::Type, Error> {
        let is_mut = ty.mutability.is_some();
        let encoded = self.resolve_type(&ty.elem)?;

        match encoded {
            metadata::Type::PtrMut(inner, pointers) if is_mut => {
                Ok(metadata::Type::PtrMut(inner, pointers + 1))
            }
            metadata::Type::PtrConst(inner, pointers) if !is_mut => {
                Ok(metadata::Type::PtrConst(inner, pointers + 1))
            }
            metadata::Type::PtrMut(..) | metadata::Type::PtrConst(..) => self.err(
                ty.elem.as_ref(),
                "mixed `*mut` and `*const` pointer chains are not representable",
            ),
            _ if is_mut => Ok(metadata::Type::PtrMut(Box::new(encoded), 1)),
            _ => Ok(metadata::Type::PtrConst(Box::new(encoded), 1)),
        }
    }

    fn resolve_array_len(&self, expr: &syn::Expr) -> Result<usize, Error> {
        let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(value),
            ..
        }) = expr
        else {
            return self.err(expr, "value not valid");
        };
        value
            .base10_parse()
            .map_err(|_| self.error(expr, "value not valid"))
    }

    fn one_imported_type(
        &self,
        path: &syn::Path,
        name: &str,
        candidates: Vec<(&Import, metadata::Type)>,
    ) -> Result<Option<metadata::Type>, Error> {
        match candidates.as_slice() {
            [] => Ok(None),
            [(_, ty)] => Ok(Some(ty.clone())),
            _ => {
                let start = path.span().start();
                let end = path.span().end();
                let mut error = Error::new(
                    &format!("type name `{name}` is ambiguous"),
                    &self.file.source,
                    start.line,
                    start.column,
                )
                .with_code("RDL0004")
                .with_primary_label(
                    Label::primary(&self.file.source, start.line, start.column)
                        .with_end(end.line, end.column)
                        .with_message("ambiguous type name"),
                )
                .with_help("use a qualified path or add an explicit named import");
                for (import, ty) in candidates {
                    let import_start = import.span.start();
                    let import_end = import.span.end();
                    error = error.with_label(
                        Label::secondary(
                            &self.file.source,
                            import_start.line,
                            import_start.column,
                            &format!("candidate `{}`", display_named_type(&ty)),
                        )
                        .with_end(import_end.line, import_end.column),
                    );
                }
                Err(error)
            }
        }
    }
}
