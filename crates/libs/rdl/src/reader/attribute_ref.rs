use super::guid;
use super::*;

pub struct AttributeRef {
    pub type_name: metadata::TypeName,
    pub args: Vec<(String, metadata::Value)>,
}

#[derive(Clone)]
struct AttributeInfo {
    type_name: metadata::TypeName,
    constructors: Vec<Vec<metadata::Type>>,
    properties: Vec<(String, metadata::Type)>,
}

struct SplitArgs<'a> {
    positional: Vec<&'a syn::Expr>,
    named: Vec<(String, &'a syn::Expr)>,
}

fn push_attribute_candidate<'a>(
    candidates: &mut Vec<(&'a Import, AttributeInfo)>,
    import: &'a Import,
    info: AttributeInfo,
) {
    if !candidates
        .iter()
        .any(|(_, existing)| existing.type_name == info.type_name)
    {
        candidates.push((import, info));
    }
}

fn collect_bitor_variants(expr: &syn::Expr) -> Option<Vec<String>> {
    let mut names = Vec::new();
    collect_bitor_variants_inner(expr, &mut names)?;
    if names.len() >= 2 { Some(names) } else { None }
}

fn collect_bitor_variants_inner(expr: &syn::Expr, names: &mut Vec<String>) -> Option<()> {
    match expr {
        syn::Expr::Binary(syn::ExprBinary {
            left,
            op: syn::BinOp::BitOr(_),
            right,
            ..
        }) => {
            collect_bitor_variants_inner(left, names)?;
            collect_bitor_variants_inner(right, names)?;
            Some(())
        }
        syn::Expr::Path(syn::ExprPath { path, .. })
            if path.leading_colon.is_none() && path.segments.len() == 1 =>
        {
            names.push(path.segments[0].ident.to_string());
            Some(())
        }
        _ => None,
    }
}

impl Resolver<'_, '_> {
    fn find_attribute_type(&self, path: &syn::Path) -> Result<Option<AttributeInfo>, Error> {
        let mut segments: Vec<String> = path
            .segments
            .iter()
            .map(|segment| segment.ident.unraw_to_string())
            .collect();
        let Some(name) = segments.pop() else {
            return Ok(None);
        };

        if !segments.is_empty() {
            if segments.iter().any(|segment| segment == "super") {
                let mut namespace = vec![];
                for segment in &segments {
                    if segment == "super" {
                        if namespace.is_empty() {
                            namespace.extend(self.namespace.split('.').map(str::to_string));
                        }
                        if namespace.pop().is_none() {
                            return self.err(path, "too many leading `super` keywords");
                        }
                    } else {
                        namespace.push(segment.clone());
                    }
                }
                return Ok(self.find_attribute_in_namespace(&namespace.join("."), &name));
            }

            let namespace = segments.join(".");
            if let Some(info) = self.find_attribute_in_namespace(&namespace, &name) {
                return Ok(Some(info));
            }

            let relative = format!("{}.{}", self.namespace, namespace);
            if let Some(info) = self.find_attribute_in_namespace(&relative, &name) {
                return Ok(Some(info));
            }

            let mut imported = vec![];
            for import in &self.file.imports {
                if import.glob || import.local.as_deref() != Some(&segments[0]) {
                    continue;
                }
                let target = import.path.join(".");
                if !self.namespace_exists(&target) {
                    continue;
                }
                let mut namespace = import.path.clone();
                namespace.extend_from_slice(&segments[1..]);
                if let Some(info) = self.find_attribute_in_namespace(&namespace.join("."), &name) {
                    push_attribute_candidate(&mut imported, import, info);
                }
            }
            return self.one_imported_attribute(path, &name, imported);
        }

        let parts: Vec<&str> = self.namespace.split('.').collect();
        for namespace in (1..=parts.len()).rev().map(|len| parts[..len].join(".")) {
            if let Some(info) = self.find_attribute_in_namespace(&namespace, &name) {
                return Ok(Some(info));
            }
        }

        let mut imported = vec![];
        for import in &self.file.imports {
            if import.glob || import.local.as_deref() != Some(&name) {
                continue;
            }
            let (target_name, target_namespace) = import.path.split_last().unwrap();
            if let Some(info) =
                self.find_attribute_in_namespace(&target_namespace.join("."), target_name)
            {
                push_attribute_candidate(&mut imported, import, info);
            }
        }
        if let Some(info) = self.one_imported_attribute(path, &name, imported)? {
            return Ok(Some(info));
        }

        let mut globbed = vec![];
        for import in &self.file.imports {
            if import.glob
                && let Some(info) = self.find_attribute_in_namespace(&import.path.join("."), &name)
            {
                push_attribute_candidate(&mut globbed, import, info);
            }
        }
        self.one_imported_attribute(path, &name, globbed)
    }

    fn find_attribute_in_namespace(&self, namespace: &str, name: &str) -> Option<AttributeInfo> {
        let physical_name = if name.ends_with("Attribute") {
            name.to_string()
        } else {
            format!("{name}Attribute")
        };
        self.find_in_index(namespace, &physical_name)
            .or_else(|| self.find_in_reference(namespace, &physical_name))
    }

    fn one_imported_attribute(
        &self,
        path: &syn::Path,
        name: &str,
        candidates: Vec<(&Import, AttributeInfo)>,
    ) -> Result<Option<AttributeInfo>, Error> {
        match candidates.as_slice() {
            [] => Ok(None),
            [(_, info)] => Ok(Some(info.clone())),
            _ => {
                let start = path.span().start();
                let end = path.span().end();
                let mut error = Error::new(
                    &format!("attribute name `{name}` is ambiguous"),
                    &self.file.source,
                    start.line,
                    start.column,
                )
                .with_code("RDL0004")
                .with_primary_label(
                    Label::primary(&self.file.source, start.line, start.column)
                        .with_end(end.line, end.column)
                        .with_message("ambiguous attribute name"),
                )
                .with_help("use a qualified path or add an explicit named import");
                for (import, info) in candidates {
                    let import_start = import.span.start();
                    let import_end = import.span.end();
                    error = error.with_label(
                        Label::secondary(
                            &self.file.source,
                            import_start.line,
                            import_start.column,
                            &format!(
                                "candidate `{}.{}`",
                                info.type_name.namespace, info.type_name.name
                            ),
                        )
                        .with_end(import_end.line, import_end.column),
                    );
                }
                Err(error)
            }
        }
    }

    fn find_in_reference(&self, namespace: &str, attr_name: &str) -> Option<AttributeInfo> {
        let mut constructors = vec![];
        let mut properties = vec![];

        for typedef in self.reference.get(namespace, attr_name) {
            if typedef.category() == metadata::reader::TypeCategory::Attribute {
                for method in typedef.methods() {
                    if method.name() == ".ctor" {
                        let sig = method.signature(&[]);
                        constructors.push(sig.types);
                    }
                }
                for field in typedef.fields() {
                    let flags = field.flags();
                    if flags.contains(metadata::FieldAttributes::Public)
                        && !flags.contains(metadata::FieldAttributes::Static)
                        && !flags.contains(metadata::FieldAttributes::Literal)
                        && !flags.contains(metadata::FieldAttributes::SpecialName)
                    {
                        properties.push((field.name().to_string(), field.ty()));
                    }
                }
            }
        }

        if constructors.is_empty() && properties.is_empty() {
            None
        } else {
            Some(AttributeInfo {
                type_name: metadata::TypeName::named(namespace, attr_name),
                constructors,
                properties,
            })
        }
    }

    fn find_in_index(&self, namespace: &str, attr_name: &str) -> Option<AttributeInfo> {
        let (_, item) = *self
            .index
            .namespaces
            .get(namespace)?
            .types
            .get(attr_name)?
            .first()?;
        let Item::Attribute(attr_item) = item else {
            return None;
        };

        let mut constructors = vec![];
        for method in &attr_item.methods {
            let types: Result<Vec<_>, _> = method
                .inputs
                .iter()
                .map(|arg| self.resolve_type_in_attr_ns(namespace, &arg.ty))
                .collect();
            if let Ok(types) = types {
                constructors.push(types);
            }
        }

        let mut properties = vec![];
        for (prop_name, prop_ty) in &attr_item.properties {
            if let Ok(ty) = self.resolve_type_in_attr_ns(namespace, prop_ty) {
                properties.push((prop_name.to_string(), ty));
            }
        }

        Some(AttributeInfo {
            type_name: metadata::TypeName::named(namespace, attr_name),
            constructors,
            properties,
        })
    }
}

impl Encoder<'_> {
    fn find_attribute_type(&self, path: &syn::Path) -> Result<Option<AttributeInfo>, Error> {
        self.resolver().find_attribute_type(path)
    }

    fn split_args<'a>(&self, args: &'a [syn::Expr]) -> Result<SplitArgs<'a>, Error> {
        let mut positional: Vec<&syn::Expr> = vec![];
        let mut named: Vec<(String, &syn::Expr)> = vec![];

        for arg in args {
            if let syn::Expr::Assign(syn::ExprAssign { left, right, .. }) = arg {
                if let syn::Expr::Path(syn::ExprPath { path, .. }) = left.as_ref()
                    && path.leading_colon.is_none()
                    && path.segments.len() == 1
                {
                    named.push((path.segments[0].ident.to_string(), right.as_ref()));
                    continue;
                }
                return self.err(arg, "expected `name = value` for named attribute argument");
            }
            if !named.is_empty() {
                return self.err(
                    arg,
                    "positional attribute arguments must come before named arguments",
                );
            }
            positional.push(arg);
        }

        Ok(SplitArgs { positional, named })
    }

    fn resolve_attribute_args(
        &self,
        attr: &syn::Attribute,
        info: &AttributeInfo,
        positional: &[&syn::Expr],
        named: &[(String, &syn::Expr)],
    ) -> Result<Vec<(String, metadata::Value)>, Error> {
        let mut last_type_error: Option<Error> = None;
        let mut ctor_values: Option<Vec<(String, metadata::Value)>> = None;

        for types in &info.constructors {
            if types.len() != positional.len() {
                continue;
            }

            let mut values = vec![];
            let mut type_error: Option<Error> = None;

            for (ty, arg) in types.iter().zip(positional.iter()) {
                match self.encode_attr_value(ty, arg) {
                    Ok(v) => values.push((String::new(), v)),
                    Err(e) => {
                        type_error = Some(e);
                        break;
                    }
                }
            }

            match type_error {
                None => {
                    ctor_values = Some(values);
                    break;
                }
                Some(e) => last_type_error = Some(e),
            }
        }

        let Some(mut result) = ctor_values else {
            if let Some(err) = last_type_error {
                return Err(err);
            } else {
                return self.err(attr, "no matching attribute constructor found");
            }
        };

        for (name, value_expr) in named {
            let prop_ty = info
                .properties
                .iter()
                .find(|(pname, _)| pname == name)
                .map(|(_, ty)| ty)
                .ok_or_else(|| self.error(attr, &format!("attribute has no property `{name}`")))?;
            let value = self.encode_attr_value(prop_ty, value_expr)?;
            result.push((name.clone(), value));
        }

        Ok(result)
    }

    fn encode_attr_value(
        &self,
        ty: &metadata::Type,
        value: &syn::Expr,
    ) -> Result<metadata::Value, Error> {
        match ty {
            metadata::Type::String => match value {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) => Ok(metadata::Value::Utf8(s.value())),
                _ => self.err(value, "expected string literal"),
            },
            metadata::Type::ClassName(tn) if tn == ("System", "Type") => match value {
                syn::Expr::Path(syn::ExprPath { path, .. }) => match self.encode_path(path)? {
                    metadata::Type::ClassName(tn) | metadata::Type::ValueName(tn) => {
                        Ok(metadata::Value::TypeName(tn))
                    }
                    _ => self.err(value, "expected type name"),
                },
                _ => self.err(value, "expected type path"),
            },
            metadata::Type::ValueName(tn) | metadata::Type::ClassName(tn) => {
                if let syn::Expr::Path(syn::ExprPath { path, .. }) = value
                    && path.leading_colon.is_none()
                    && path.segments.len() == 1
                {
                    let variant_name = path.segments[0].ident.to_string();
                    let inner = self.find_enum_variant_value(tn, &variant_name, value)?;
                    return Ok(metadata::Value::EnumValue(tn.clone(), Box::new(inner)));
                }
                if self.enum_is_flags(tn) {
                    if let Some(names) = collect_bitor_variants(value) {
                        let Some(underlying) = self.enum_underlying_type(tn) else {
                            return self.err(value, "enum backing type not found");
                        };
                        let mut combined = 0;
                        for name in &names {
                            let inner = self.find_enum_variant_value(tn, name, value)?;
                            let Some(bits) = inner.integer_bits() else {
                                return self
                                    .err(value, &format!("expected `{}` variant name", tn.name));
                            };
                            combined |= bits;
                        }
                        let Some(combined) = metadata::Value::from_integer(&underlying, combined)
                        else {
                            return self.err(value, "invalid enum backing type");
                        };
                        return Ok(metadata::Value::EnumValue(tn.clone(), Box::new(combined)));
                    }
                    if let Some(underlying) = self.enum_underlying_type(tn)
                        && let Ok(value) = self.encode_value(&underlying, value)
                    {
                        return Ok(metadata::Value::EnumValue(tn.clone(), Box::new(value)));
                    }
                }
                self.err(value, &format!("expected `{}` variant name", tn.name))
            }
            _ => self.encode_value(ty, value),
        }
    }

    fn enum_is_flags(&self, tn: &metadata::TypeName) -> bool {
        if let Some(reference) = self.output.reference() {
            for typedef in reference.get(&tn.namespace, &tn.name) {
                if typedef.category() == metadata::reader::TypeCategory::Enum
                    && metadata::HasAttributes::attributes(&typedef).any(|attr| {
                        attr.name() == "FlagsAttribute"
                            && attr.ctor().parent().namespace() == "System"
                    })
                {
                    return true;
                }
            }
        }
        if let Some(ns) = self.index.namespaces.get(&tn.namespace)
            && let Some(variants) = ns.types.get(&tn.name)
            && let Some((_, Item::Enum(enum_item))) = variants.first()
            && enum_item.attrs.iter().any(|a| a.path().is_ident("flags"))
        {
            return true;
        }
        false
    }

    fn enum_underlying_type(&self, tn: &metadata::TypeName) -> Option<metadata::Type> {
        self.output
            .reference()
            .and_then(|reference| {
                let mut definitions = reference.get(&tn.namespace, &tn.name);
                let definition = definitions.next()?;
                definitions
                    .next()
                    .is_none()
                    .then(|| definition.underlying_type())?
            })
            .or_else(|| self.rdl_underlying_type(&tn.namespace, &tn.name))
    }

    fn find_enum_variant_value(
        &self,
        tn: &metadata::TypeName,
        variant_name: &str,
        spanned: &syn::Expr,
    ) -> Result<metadata::Value, Error> {
        if let Some(reference) = self.output.reference() {
            for typedef in reference.get(&tn.namespace, &tn.name) {
                if typedef.category() == metadata::reader::TypeCategory::Enum {
                    for field in typedef.fields() {
                        if field.flags().contains(metadata::FieldAttributes::Literal)
                            && field.name() == variant_name
                            && let Some(constant) = field.constant()
                        {
                            let value = constant.value();
                            if value.integer_bits().is_some() {
                                return Ok(value);
                            }
                            return self.err(
                                spanned,
                                &format!("unsupported enum constant type: {value:?}"),
                            );
                        }
                    }
                }
            }
        }

        if let Some(ns) = self.index.namespaces.get(&tn.namespace)
            && let Some(variants) = ns.types.get(&tn.name)
            && let Some((_, Item::Enum(enum_item))) = variants.first()
        {
            for variant in &enum_item.variants {
                if variant.ident == variant_name
                    && let Some((_, discriminant)) = &variant.discriminant
                {
                    let Some(underlying) = self.enum_underlying_type(tn) else {
                        return self.err(spanned, "enum backing type not found");
                    };
                    return self.encode_value(&underlying, discriminant);
                }
            }
        }

        self.err(spanned, "enum variant not found")
    }

    pub fn resolve_attribute_ref(&self, attr: &syn::Attribute) -> Result<AttributeRef, Error> {
        let path = attr.path();

        let info = self
            .find_attribute_type(path)?
            .ok_or_else(|| self.error(attr, "attribute type not found"))?;

        let raw_args: Vec<syn::Expr> = match &attr.meta {
            syn::Meta::Path(_) => vec![],
            syn::Meta::List(_) => attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated,
                )
                .map_err(|e| {
                    let start = e.span().start();
                    Error::new(&e.to_string(), &self.file.source, start.line, start.column)
                })?
                .into_iter()
                .collect(),
            syn::Meta::NameValue(_) => {
                return self.err(attr, "attribute cannot use top-level `name = value` syntax");
            }
        };

        let split = self.split_args(&raw_args)?;

        let args = self.resolve_attribute_args(attr, &info, &split.positional, &split.named)?;

        Ok(AttributeRef {
            type_name: info.type_name,
            args,
        })
    }

    pub fn encode_named_attribute(
        &mut self,
        has_attribute: metadata::writer::HasAttribute,
        attr_ref: &AttributeRef,
    ) {
        let attribute_typeref = self
            .output
            .TypeRef(&attr_ref.type_name.namespace, &attr_ref.type_name.name);

        let signature = metadata::Signature {
            flags: metadata::MethodCallAttributes::HASTHIS,
            return_type: metadata::Type::Void,
            types: attr_ref
                .args
                .iter()
                .filter(|(name, _)| name.is_empty())
                .map(|(_, v)| v.ty())
                .collect(),
        };

        let ctor = self.output.MemberRef(
            ".ctor",
            &signature,
            metadata::writer::MemberRefParent::TypeRef(attribute_typeref),
        );

        self.output.Attribute(
            has_attribute,
            metadata::writer::AttributeType::MemberRef(ctor),
            &attr_ref.args,
        );
    }

    pub fn encode_native_typedef_attribute(&mut self, target: metadata::writer::HasAttribute) {
        let attr_ref = AttributeRef {
            type_name: metadata::TypeName::named(METADATA_NAMESPACE, "NativeTypedefAttribute"),
            args: vec![],
        };
        self.encode_named_attribute(target, &attr_ref);
    }

    pub fn emit_pseudo_attribute(
        &mut self,
        target: metadata::writer::HasAttribute,
        pseudo: &PseudoAttr,
        attr: &syn::Attribute,
    ) -> Result<(), Error> {
        let attr_ref = if matches!(attr.meta, syn::Meta::Path(_)) {
            AttributeRef {
                type_name: metadata::TypeName::named(METADATA_NAMESPACE, pseudo.metadata),
                args: vec![],
            }
        } else {
            self.resolve_pseudo_attr_ref(attr, pseudo)?
        };
        self.encode_named_attribute(target, &attr_ref);
        Ok(())
    }

    fn resolve_pseudo_attr_ref(
        &self,
        attr: &syn::Attribute,
        pseudo: &PseudoAttr,
    ) -> Result<AttributeRef, Error> {
        let resolver = self.resolver();
        let info = resolver
            .find_in_reference(METADATA_NAMESPACE, pseudo.metadata)
            .or_else(|| resolver.find_in_index(METADATA_NAMESPACE, pseudo.metadata))
            .ok_or_else(|| self.error(attr, "pseudo-attribute type not found"))?;

        let raw_args: Vec<syn::Expr> = match &attr.meta {
            syn::Meta::Path(_) => vec![],
            syn::Meta::List(_) => attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated,
                )
                .map_err(|e| {
                    let start = e.span().start();
                    Error::new(&e.to_string(), &self.file.source, start.line, start.column)
                })?
                .into_iter()
                .collect(),
            syn::Meta::NameValue(_) => {
                return self.err(attr, "attribute cannot use top-level `name = value` syntax");
            }
        };

        let split = self.split_args(&raw_args)?;
        let args = if let Some(prop) = pseudo.prop {
            if split.positional.len() != 1 || !split.named.is_empty() {
                return self.err(attr, &format!("`{}` takes a single value", pseudo.short));
            }
            let named = [(prop.to_string(), split.positional[0])];
            self.resolve_attribute_args(attr, &info, &[], &named)?
        } else {
            self.resolve_attribute_args(attr, &info, &split.positional, &split.named)?
        };

        Ok(AttributeRef {
            type_name: info.type_name,
            args,
        })
    }

    pub fn emit_arch_attribute(&mut self, target: metadata::writer::HasAttribute, arch_bits: i32) {
        let attr_ref = AttributeRef {
            type_name: metadata::TypeName::named(
                METADATA_NAMESPACE,
                "SupportedArchitectureAttribute",
            ),
            args: vec![(String::new(), metadata::Value::I32(arch_bits))],
        };
        self.encode_named_attribute(target, &attr_ref);
    }

    pub fn emit_align_attribute(&mut self, target: metadata::writer::HasAttribute, alignment: u16) {
        let attr_ref = AttributeRef {
            type_name: metadata::TypeName::named(METADATA_NAMESPACE, "AlignmentAttribute"),
            args: vec![(String::new(), metadata::Value::I32(alignment as i32))],
        };
        self.encode_named_attribute(target, &attr_ref);
    }

    pub fn emit_overload_attribute(&mut self, target: metadata::writer::HasAttribute, name: &str) {
        let attr_ref = AttributeRef {
            type_name: metadata::TypeName::named(WINRT_METADATA_NAMESPACE, "OverloadAttribute"),
            args: vec![(String::new(), metadata::Value::Utf8(name.to_string()))],
        };
        self.encode_named_attribute(target, &attr_ref);
    }

    pub fn emit_default_overload_attribute(&mut self, target: metadata::writer::HasAttribute) {
        let attr_ref = AttributeRef {
            type_name: metadata::TypeName::named(
                WINRT_METADATA_NAMESPACE,
                "DefaultOverloadAttribute",
            ),
            args: vec![],
        };
        self.encode_named_attribute(target, &attr_ref);
    }

    pub fn emit_bitfield_attribute(
        &mut self,
        target: metadata::writer::HasAttribute,
        name: &str,
        offset: u32,
        width: u32,
    ) {
        let attr_ref = AttributeRef {
            type_name: metadata::TypeName::named(METADATA_NAMESPACE, "NativeBitfieldAttribute"),
            args: vec![
                (String::new(), metadata::Value::Utf8(name.to_string())),
                (String::new(), metadata::Value::I64(offset as i64)),
                (String::new(), metadata::Value::I64(width as i64)),
            ],
        };
        self.encode_named_attribute(target, &attr_ref);
    }

    pub fn is_guid_attribute(&self, attr: &syn::Attribute) -> Result<bool, Error> {
        Ok(self.find_attribute_type(attr.path())?.is_some_and(|info| {
            &info.type_name == ("Windows.Foundation.Metadata", "GuidAttribute")
        }))
    }

    pub fn is_exclusive_to_attribute(&self, attr: &syn::Attribute) -> Result<bool, Error> {
        Ok(self.find_attribute_type(attr.path())?.is_some_and(|info| {
            &info.type_name == ("Windows.Foundation.Metadata", "ExclusiveToAttribute")
        }))
    }

    pub fn encode_guid_pseudo_attrs(
        &mut self,
        target: metadata::writer::HasAttribute,
        attrs: &[syn::Attribute],
    ) -> Result<bool, Error> {
        let mut already_has_guid = false;
        for attr in attrs {
            already_has_guid |= self.is_guid_attribute(attr)?
                || attr.path().is_ident("guid")
                || attr.path().is_ident("no_guid");
        }

        for attr in attrs {
            if attr.path().is_ident("guid") {
                let lit: syn::LitInt = attr
                    .parse_args()
                    .map_err(|_| self.error(attr, "`#[guid]` requires a single u128 literal"))?;
                let v = parse_guid_u128(&lit)
                    .map_err(|_| self.error(attr, "invalid u128 literal in `#[guid]`"))?;
                let (d1, d2, d3, d4) = guid::u128_to_guid(v);
                guid::emit_guid_attribute(self.output, target, d1, d2, d3, d4);
            } else if attr.path().is_ident("no_guid") && !matches!(attr.meta, syn::Meta::Path(_)) {
                return self.err(attr, "`#[no_guid]` attribute does not accept arguments");
            }
        }

        Ok(already_has_guid)
    }

    pub fn encode_attrs(
        &mut self,
        has_attribute: metadata::writer::HasAttribute,
        attrs: &[syn::Attribute],
        skip: &[&str],
    ) -> Result<(), Error> {
        for attr in attrs {
            let path = attr.path();

            if path.is_ident("win32") || path.is_ident("winrt") || path.is_ident("arch") {
                continue;
            }

            if skip.iter().any(|s| path.is_ident(s)) {
                continue;
            }

            // A naturalized pseudo-attribute (short SAL/IDL spelling) maps to its metadata
            // attribute via the shared table; the fully-qualified spelling still resolves
            // generically below.
            if let Some(pseudo) = path
                .get_ident()
                .and_then(|i| pseudo_by_short(&i.to_string()))
            {
                self.emit_pseudo_attribute(has_attribute, pseudo, attr)?;
                continue;
            }

            let attr_ref = self.resolve_attribute_ref(attr)?;
            self.encode_named_attribute(has_attribute, &attr_ref);
        }

        Ok(())
    }

    pub fn encode_wrapped_attrs(
        &mut self,
        has_attribute: metadata::writer::HasAttribute,
        attrs: &[syn::Attribute],
        wrapper: &str,
    ) -> Result<(), Error> {
        for attr in attrs.iter().filter(|attr| attr.path().is_ident(wrapper)) {
            let meta: syn::Meta = attr
                .parse_args()
                .map_err(|_| self.error(attr, "invalid wrapped attribute"))?;
            let nested: syn::Attribute = syn::parse_quote_spanned!(attr.span()=> #[#meta]);
            self.encode_attrs(has_attribute, &[nested], &[])?;
        }
        Ok(())
    }
}
