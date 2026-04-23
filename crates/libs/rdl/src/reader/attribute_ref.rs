use super::guid;
use super::*;

/// A parsed and validated reference to a custom attribute defined in metadata or RDL.
///
/// Built-in RDL attributes (`win32`, `winrt`, `repr`, `link`, etc.) are
/// handled separately by the individual encode functions and are never represented here.
pub struct AttributeRef {
    pub type_name: metadata::TypeName,
    pub args: Vec<(String, metadata::Value)>,
}

/// Collected information about an attribute type: constructor overloads and named
/// instance-field properties (e.g. `version: u32`).
struct AttributeInfo {
    type_name: metadata::TypeName,
    constructors: Vec<Vec<metadata::Type>>,
    /// Named instance fields: `(field_name, field_type)`.
    properties: Vec<(String, metadata::Type)>,
}

/// Positional and named arguments split from a raw argument list.
struct SplitArgs<'a> {
    positional: Vec<&'a syn::Expr>,
    named: Vec<(String, &'a syn::Expr)>,
}

fn collect_bitor_variants(expr: &syn::Expr) -> Option<Vec<String>> {
    let mut names = Vec::new();
    collect_bitor_variants_inner(expr, &mut names)?;
    if names.len() >= 2 {
        Some(names)
    } else {
        None
    }
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

impl Encoder<'_> {
    fn find_attribute_type(&self, path: &syn::Path) -> Option<AttributeInfo> {
        let mut segments: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();

        let name = segments.pop()?;
        let attr_name = format!("{}Attribute", name);

        let explicit_namespace = if segments.is_empty() {
            None
        } else {
            Some(segments.join("."))
        };

        let namespaces: Vec<String> = if let Some(ns) = explicit_namespace {
            vec![ns]
        } else {
            let parts: Vec<&str> = self.namespace.split('.').collect();
            let mut nss: Vec<String> = (1..=parts.len())
                .rev()
                .map(|len| parts[..len].join("."))
                .collect();
            for use_item in &self.file.uses {
                if let Some(ns) = glob_use_namespace(use_item) {
                    if !nss.contains(&ns) {
                        nss.push(ns);
                    }
                }
            }
            nss
        };

        for ns in &namespaces {
            if let Some(info) = self
                .find_in_reference(ns, &attr_name)
                .or_else(|| self.find_in_index(ns, &attr_name))
            {
                return Some(info);
            }
        }

        None
    }

    fn find_in_reference(&self, namespace: &str, attr_name: &str) -> Option<AttributeInfo> {
        let mut constructors = vec![];
        let mut properties = vec![];

        if let Some(reference) = self.output.reference() {
            for typedef in reference.get(namespace, attr_name) {
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
                .map(|arg| self.encode_type_in_attr_ns(namespace, &arg.ty))
                .collect();
            if let Ok(types) = types {
                constructors.push(types);
            }
        }

        let mut properties = vec![];
        for (prop_name, prop_ty) in &attr_item.properties {
            if let Ok(ty) = self.encode_type_in_attr_ns(namespace, prop_ty) {
                properties.push((prop_name.to_string(), ty));
            }
        }

        Some(AttributeInfo {
            type_name: metadata::TypeName::named(namespace, attr_name),
            constructors,
            properties,
        })
    }

    fn split_args<'a>(&self, args: &'a [syn::Expr]) -> Result<SplitArgs<'a>, Error> {
        let mut positional: Vec<&syn::Expr> = vec![];
        let mut named: Vec<(String, &syn::Expr)> = vec![];

        for arg in args {
            if let syn::Expr::Assign(syn::ExprAssign { left, right, .. }) = arg {
                if let syn::Expr::Path(syn::ExprPath { path, .. }) = left.as_ref() {
                    if path.leading_colon.is_none() && path.segments.len() == 1 {
                        named.push((path.segments[0].ident.to_string(), right.as_ref()));
                        continue;
                    }
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

        let mut result = match ctor_values {
            Some(v) => v,
            None => {
                if let Some(err) = last_type_error {
                    return Err(err);
                } else {
                    return self.err(attr, "no matching attribute constructor found");
                }
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
                if let syn::Expr::Path(syn::ExprPath { path, .. }) = value {
                    if path.leading_colon.is_none() && path.segments.len() == 1 {
                        let variant_name = path.segments[0].ident.to_string();
                        let inner = self.find_enum_variant_value(tn, &variant_name, value)?;
                        return Ok(metadata::Value::EnumValue(tn.clone(), Box::new(inner)));
                    }
                }
                if self.enum_is_flags(tn) {
                    if let Some(names) = collect_bitor_variants(value) {
                        let mut combined: i32 = 0;
                        for name in &names {
                            let inner = self.find_enum_variant_value(tn, name, value)?;
                            let v = match inner {
                                metadata::Value::I32(v) => v,
                                _ => {
                                    return self.err(
                                        value,
                                        &format!("expected `{}` variant name", tn.name),
                                    )
                                }
                            };
                            combined |= v;
                        }
                        return Ok(metadata::Value::EnumValue(
                            tn.clone(),
                            Box::new(metadata::Value::I32(combined)),
                        ));
                    }
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Int(int),
                        ..
                    }) = value
                    {
                        if let Ok(v) = int.base10_parse::<i32>() {
                            return Ok(metadata::Value::EnumValue(
                                tn.clone(),
                                Box::new(metadata::Value::I32(v)),
                            ));
                        }
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
        if let Some(ns) = self.index.namespaces.get(&tn.namespace) {
            if let Some(variants) = ns.types.get(&tn.name) {
                if let Some((_, Item::Enum(enum_item))) = variants.first() {
                    if enum_item.attrs.iter().any(|a| a.path().is_ident("flags")) {
                        return true;
                    }
                }
            }
        }
        false
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
                        {
                            if let Some(constant) = field.constant() {
                                return Ok(match constant.value() {
                                    metadata::Value::I32(v) => metadata::Value::I32(v),
                                    metadata::Value::U32(v) => metadata::Value::I32(v as i32),
                                    other => {
                                        return self.err(
                                            spanned,
                                            &format!("unsupported enum constant type: {other:?}"),
                                        )
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }

        if let Some(ns) = self.index.namespaces.get(&tn.namespace) {
            if let Some(variants) = ns.types.get(&tn.name) {
                if let Some((_, Item::Enum(enum_item))) = variants.first() {
                    for variant in &enum_item.variants {
                        if variant.ident == variant_name {
                            if let Some((_, discriminant)) = &variant.discriminant {
                                let result =
                                    self.encode_value(&metadata::Type::I32, discriminant)
                                        .or_else(|_| {
                                            self.encode_value(&metadata::Type::U32, discriminant)
                                                .map(|v| match v {
                                                    metadata::Value::U32(n) => {
                                                        metadata::Value::I32(n as i32)
                                                    }
                                                    other => other,
                                                })
                                        });
                                return result;
                            }
                        }
                    }
                }
            }
        }

        self.err(spanned, "enum variant not found")
    }

    pub fn resolve_attribute_ref(&self, attr: &syn::Attribute) -> Result<AttributeRef, Error> {
        let path = attr.path();

        let info = self
            .find_attribute_type(path)
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

    /// Emits the `Windows.Win32.Foundation.Metadata.NativeTypedefAttribute` on `target`.
    pub fn encode_native_typedef_attribute(&mut self, target: metadata::writer::HasAttribute) {
        let attr_ref = AttributeRef {
            type_name: metadata::TypeName::named(
                "Windows.Win32.Foundation.Metadata",
                "NativeTypedefAttribute",
            ),
            args: vec![],
        };
        self.encode_named_attribute(target, &attr_ref);
    }

    pub fn is_guid_attribute(&self, attr: &syn::Attribute) -> bool {
        self.find_attribute_type(attr.path())
            .map(|info| &info.type_name == ("Windows.Foundation.Metadata", "GuidAttribute"))
            .unwrap_or(false)
    }

    pub fn is_exclusive_to_attribute(&self, attr: &syn::Attribute) -> bool {
        self.find_attribute_type(attr.path())
            .map(|info| &info.type_name == ("Windows.Foundation.Metadata", "ExclusiveToAttribute"))
            .unwrap_or(false)
    }

    /// Processes `#[guid(0x…)]` and `#[no_guid]` pseudo-attributes in `attrs`, emitting a
    /// `GuidAttribute` on `target` when an explicit GUID is supplied.  Returns `true` when the
    /// attribute list already carries GUID information (explicit `#[guid]`, `#[no_guid]`, or an
    /// existing `GuidAttribute`), so the caller can skip automatic GUID derivation.
    pub fn encode_guid_pseudo_attrs(
        &mut self,
        target: metadata::writer::HasAttribute,
        attrs: &[syn::Attribute],
    ) -> Result<bool, Error> {
        let already_has_guid = attrs.iter().any(|attr| {
            self.is_guid_attribute(attr)
                || attr.path().is_ident("guid")
                || attr.path().is_ident("no_guid")
        });

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

            if path.is_ident("win32") || path.is_ident("winrt") {
                continue;
            }

            if skip.iter().any(|s| path.is_ident(s)) {
                continue;
            }

            let attr_ref = self.resolve_attribute_ref(attr)?;
            self.encode_named_attribute(has_attribute, &attr_ref);
        }

        Ok(())
    }
}
