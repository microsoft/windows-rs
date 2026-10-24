use super::guid;
use super::*;
use buffer_extent::pointee as extent_pointee;
use syn::ext::IdentExt;

#[derive(PartialEq)]
pub(super) enum GuidMode {
    None,
    Explicit,
    Auto,
}

pub struct AttributeRef {
    pub type_name: metadata::TypeName,
    pub args: Vec<(String, metadata::Value)>,
}

pub(super) struct ExtentTypes<'a> {
    pub counts: &'a [metadata::Type],
    pub subject: &'a metadata::Type,
    pub result: Option<&'a metadata::Type>,
}

struct AttributeInfo {
    type_name: metadata::TypeName,
    constructors: Vec<Vec<metadata::Type>>,
    properties: Vec<(String, metadata::Type)>,
}

struct SplitArgs<'a> {
    positional: Vec<&'a syn::Expr>,
    named: Vec<(String, &'a syn::Expr)>,
}

fn extent_integer(ty: &metadata::Type) -> bool {
    matches!(
        ty,
        metadata::Type::I8
            | metadata::Type::U8
            | metadata::Type::I16
            | metadata::Type::U16
            | metadata::Type::I32
            | metadata::Type::U32
            | metadata::Type::I64
            | metadata::Type::U64
            | metadata::Type::ISize
            | metadata::Type::USize
    )
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

impl Encoder<'_> {
    fn find_attribute_type(&self, path: &syn::Path) -> Option<AttributeInfo> {
        let mut segments: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();

        let name = segments.pop()?;
        let attr_name = format!("{name}Attribute");

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
                if let Some(ns) = glob_use_namespace(use_item)
                    && !nss.contains(&ns)
                {
                    nss.push(ns);
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
                syn::Expr::Path(syn::ExprPath { path, .. }) => self
                    .encode_path(path)?
                    .type_name()
                    .map(metadata::Value::TypeName)
                    .ok_or_else(|| {
                        self.error(value, "expected a primitive or nongeneric type name")
                    }),
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
                if self.enum_is_flags(tn)
                    && let Some(names) = collect_bitor_variants(value)
                {
                    let mut combined: i32 = 0;
                    for name in &names {
                        let inner = self.find_enum_variant_value(tn, name, value)?;
                        let metadata::Value::I32(v) = inner else {
                            return self
                                .err(value, &format!("expected `{}` variant name", tn.name));
                        };
                        combined |= v;
                    }
                    return Ok(metadata::Value::EnumValue(
                        tn.clone(),
                        Box::new(metadata::Value::I32(combined)),
                    ));
                }
                if matches!(
                    value,
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Int(_),
                        ..
                    }) | syn::Expr::Unary(syn::ExprUnary {
                        op: syn::UnOp::Neg(_),
                        ..
                    })
                ) {
                    return Ok(metadata::Value::EnumValue(
                        tn.clone(),
                        Box::new(self.encode_value(ty, value)?),
                    ));
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
                            return Ok(match constant.value() {
                                metadata::Value::I32(v) => metadata::Value::I32(v),
                                metadata::Value::U32(v) => metadata::Value::I32(v as i32),
                                other => {
                                    return self.err(
                                        spanned,
                                        &format!("unsupported enum constant type: {other:?}"),
                                    );
                                }
                            });
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
                    let result = self
                        .encode_value(&metadata::Type::I32, discriminant)
                        .or_else(|_| {
                            self.encode_value(&metadata::Type::U32, discriminant)
                                .map(|v| match v {
                                    metadata::Value::U32(n) => metadata::Value::I32(n as i32),
                                    other => other,
                                })
                        });
                    return result;
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
        let info = self
            .find_in_reference(METADATA_NAMESPACE, pseudo.metadata)
            .or_else(|| self.find_in_index(METADATA_NAMESPACE, pseudo.metadata))
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

    pub fn emit_bitfield_attribute(
        &mut self,
        target: metadata::writer::HasAttribute,
        name: &str,
        offset: u32,
        width: u32,
        member_type: Option<metadata::TypeName>,
    ) {
        let mut attr_ref = AttributeRef {
            type_name: metadata::TypeName::named(METADATA_NAMESPACE, "NativeBitfieldAttribute"),
            args: vec![
                (String::new(), metadata::Value::Utf8(name.to_string())),
                (String::new(), metadata::Value::I64(offset as i64)),
                (String::new(), metadata::Value::I64(width as i64)),
            ],
        };
        if let Some(member_type) = member_type {
            attr_ref
                .args
                .push(("MemberType".into(), metadata::Value::TypeName(member_type)));
        }
        self.encode_named_attribute(target, &attr_ref);
    }

    pub fn is_guid_attribute(&self, attr: &syn::Attribute) -> bool {
        self.find_attribute_type(attr.path())
            .is_some_and(|info| &info.type_name == ("Windows.Foundation.Metadata", "GuidAttribute"))
    }

    pub fn is_exclusive_to_attribute(&self, attr: &syn::Attribute) -> bool {
        self.find_attribute_type(attr.path()).is_some_and(|info| {
            &info.type_name == ("Windows.Foundation.Metadata", "ExclusiveToAttribute")
        })
    }

    pub fn is_native_override_slot_attribute(&self, attr: &syn::Attribute) -> bool {
        attr.path().is_ident("override_slot")
            || self.find_attribute_type(attr.path()).is_some_and(|info| {
                &info.type_name == (METADATA_NAMESPACE, "NativeOverrideSlotAttribute")
            })
    }

    pub fn encode_guid_pseudo_attrs(
        &mut self,
        target: metadata::writer::HasAttribute,
        attrs: &[syn::Attribute],
    ) -> Result<GuidMode, Error> {
        let mut mode = GuidMode::None;
        for attr in attrs {
            if attr.path().is_ident("no_guid") {
                return self.err(
                    attr,
                    "`#[no_guid]` is not supported; omit the GUID attribute on native interfaces",
                );
            }
            if !attr.path().is_ident("guid") && !self.is_guid_attribute(attr) {
                continue;
            }
            if mode != GuidMode::None {
                return self.err(attr, "duplicate GUID attribute");
            }
            mode = GuidMode::Explicit;
            if attr.path().is_ident("guid") {
                if attr
                    .parse_args_with(syn::Ident::parse_any)
                    .is_ok_and(|value| value == "auto")
                {
                    if !matches!(target, metadata::writer::HasAttribute::TypeDef(_)) {
                        return self.err(attr, "`#[guid(auto)]` requires an interface or delegate");
                    }
                    mode = GuidMode::Auto;
                    continue;
                }
                let lit: syn::LitInt = attr
                    .parse_args()
                    .map_err(|_| self.error(attr, "`#[guid]` requires a u128 literal or `auto`"))?;
                let v = parse_guid_u128(&lit)
                    .map_err(|_| self.error(attr, "invalid u128 literal in `#[guid]`"))?;
                let (d1, d2, d3, d4) = guid::u128_to_guid(v);
                guid::emit_guid_attribute(self.output, target, d1, d2, d3, d4);
            }
        }

        Ok(mode)
    }

    pub fn encode_buffer_extents(
        &mut self,
        target: metadata::writer::HasAttribute,
        attrs: &[syn::Attribute],
        names: &[&str],
        field: bool,
        types: ExtentTypes<'_>,
    ) -> Result<(), Error> {
        for attr in attrs {
            if attr.path().is_ident("range") {
                let range = integer_range::parse_integer_range(attr, names, field)
                    .map_err(|error| self.error(error.span(), &error.to_string()))?;
                self.encode_integer_range(target, attr, names, &types, &range)?;
                continue;
            }
            if attr.path().is_ident("null_terminated") {
                let (_, _, indirect) = self.pointer_contract_flags(attr, true)?;
                if field {
                    return self.err(attr, "termination phases require a parameter owner");
                }
                self.validate_buffer_subject(attr, types.subject, indirect)?;
                continue;
            }
            if attr.path().is_ident("nullable") {
                if field {
                    self.emit_value_nullable(target, attr, types.subject)?;
                    continue;
                }
                let (_, _, indirect) = self.pointer_contract_flags(attr, false)?;
                self.validate_nullable_subject(attr, types.subject, indirect)?;
                continue;
            }
            if !attr.path().is_ident("readable") && !attr.path().is_ident("writable") {
                continue;
            }
            let extent = buffer_extent::parse_buffer_extent(attr, names, field)
                .map_err(|error| self.error(error.span(), &error.to_string()))?;
            let count_type = match &extent.count {
                metadata::BufferCount::Parameter(index)
                | metadata::BufferCount::Dereference(index) => {
                    Some(types.counts[*index as usize].clone())
                }
                metadata::BufferCount::Field(name) => names
                    .iter()
                    .position(|candidate| *candidate == name)
                    .map(|index| types.counts[index].clone()),
                metadata::BufferCount::Return => types.result.cloned(),
                metadata::BufferCount::Constant(_) => None,
                metadata::BufferCount::Expression(expression) => {
                    self.validate_integer_expression(attr, expression, names, &types)?;
                    None
                }
            };
            if !matches!(
                extent.count,
                metadata::BufferCount::Constant(_) | metadata::BufferCount::Expression(_)
            ) {
                let mut ty = count_type.and_then(|ty| self.extent_underlying(ty));
                if matches!(extent.count, metadata::BufferCount::Dereference(_)) {
                    ty = ty
                        .and_then(extent_pointee)
                        .and_then(|ty| self.extent_underlying(ty));
                }
                if !ty.as_ref().is_some_and(extent_integer) {
                    return self.err(
                        attr,
                        "buffer count requires an integer value at the specified indirection",
                    );
                }
            }
            if let Some(index) = extent.null_fallback
                && !self
                    .extent_underlying(types.counts[index as usize].clone())
                    .as_ref()
                    .is_some_and(extent_integer)
            {
                return self.err(attr, "buffer count fallback requires a by-value integer");
            }
            self.validate_buffer_subject(attr, types.subject, extent.indirect)?;
            self.encode_named_attribute(
                target,
                &AttributeRef {
                    type_name: metadata::TypeName::named(
                        METADATA_NAMESPACE,
                        "BufferExtentAttribute",
                    ),
                    args: extent.to_values(),
                },
            );
        }
        Ok(())
    }

    pub(super) fn encode_integer_range(
        &mut self,
        target: metadata::writer::HasAttribute,
        attr: &syn::Attribute,
        names: &[&str],
        types: &ExtentTypes<'_>,
        range: &metadata::IntegerRange,
    ) -> Result<(), Error> {
        let mut subject = self.extent_underlying(types.subject.clone());
        if range.indirect {
            subject = subject
                .and_then(extent_pointee)
                .and_then(|ty| self.extent_underlying(ty));
        }
        if !subject.as_ref().is_some_and(extent_integer) {
            return self.err(
                attr,
                "integer range requires an integer subject at the specified indirection",
            );
        }
        for bound in range.lower.iter().chain(&range.upper) {
            self.validate_integer_expression(attr, &bound.expression, names, types)?;
        }
        self.encode_named_attribute(
            target,
            &AttributeRef {
                type_name: metadata::TypeName::named(METADATA_NAMESPACE, "IntegerRangeAttribute"),
                args: range.to_values(),
            },
        );
        Ok(())
    }

    fn validate_integer_expression(
        &self,
        attr: &syn::Attribute,
        expression: &metadata::IntegerExpression,
        names: &[&str],
        types: &ExtentTypes<'_>,
    ) -> Result<(), Error> {
        let references = expression
            .references()
            .ok_or_else(|| self.error(attr, "invalid integer expression"))?;
        for reference in references {
            use metadata::IntegerReference::*;
            let ty = match reference {
                Parameter(index) | Dereference(index) | OptionalDereference(index) => {
                    types.counts.get(*index as usize)
                }
                Field(name) => names
                    .iter()
                    .position(|candidate| *candidate == name)
                    .and_then(|index| types.counts.get(index)),
                Return => types.result,
            };
            let mut ty = ty.cloned().and_then(|ty| self.extent_underlying(ty));
            if matches!(reference, Dereference(_) | OptionalDereference(_)) {
                ty = ty
                    .and_then(extent_pointee)
                    .and_then(|ty| self.extent_underlying(ty));
            }
            if !ty.as_ref().is_some_and(extent_integer) {
                return self.err(
                    attr,
                    "expression reference requires an integer at the specified indirection",
                );
            }
        }
        Ok(())
    }

    pub(super) fn emit_value_nullable(
        &mut self,
        target: metadata::writer::HasAttribute,
        attr: &syn::Attribute,
        ty: &metadata::Type,
    ) -> Result<(), Error> {
        if !matches!(attr.meta, syn::Meta::Path(_)) {
            return self.err(
                attr,
                "field and return nullability do not accept phase flags",
            );
        }
        self.validate_nullable_subject(attr, ty, false)?;
        let expanded = syn::parse_quote! { #[Windows::Win32::Metadata::Nullable] };
        self.encode_attrs(target, &[expanded], &[])
    }

    pub(super) fn validate_nullable_subject(
        &self,
        attr: &syn::Attribute,
        ty: &metadata::Type,
        indirect: bool,
    ) -> Result<(), Error> {
        let mut subject = self.extent_underlying(ty.clone());
        if indirect {
            subject = subject
                .and_then(extent_pointee)
                .and_then(|ty| self.extent_underlying(ty));
        }
        if !subject.is_some_and(|ty| {
            matches!(
                ty,
                metadata::Type::PtrMut(..)
                    | metadata::Type::PtrConst(..)
                    | metadata::Type::RefMut(_)
                    | metadata::Type::RefConst(_)
                    | metadata::Type::ClassName(_)
                    | metadata::Type::FnPtr(_)
            )
        }) {
            return self.err(
                attr,
                "nullable requires a pointer at the specified indirection",
            );
        }
        Ok(())
    }

    fn validate_buffer_subject(
        &self,
        attr: &syn::Attribute,
        ty: &metadata::Type,
        indirect: bool,
    ) -> Result<(), Error> {
        if !buffer_extent::is_buffer_storage(ty.clone(), indirect, |ty| self.extent_underlying(ty))
        {
            return self.err(
                attr,
                "buffer contract requires pointer or array storage at the specified indirection",
            );
        }
        Ok(())
    }

    fn extent_underlying(&self, mut ty: metadata::Type) -> Option<metadata::Type> {
        let mut seen = HashSet::new();
        while let metadata::Type::ValueName(name) = &ty {
            let name = name.clone();
            if !seen.insert((name.namespace.clone(), name.name.clone())) {
                return None;
            }
            if let Some(Item::Typedef(alias)) = self.index.get(&name.namespace, &name.name).next() {
                ty = self
                    .encode_type_in_attr_ns(&name.namespace, &alias.ty)
                    .ok()?;
            } else if let Some(def) = self
                .output
                .reference()
                .and_then(|index| index.get(&name.namespace, &name.name).next())
                && metadata::HasAttributes::has_attribute(&def, "NativeTypedefAttribute")
            {
                ty = def.underlying_type()?;
            } else {
                return Some(ty);
            }
        }
        Some(ty)
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

            if path.is_ident("direction") {
                return self.err(
                    attr,
                    "use explicit `in`/`out` flags, or omit both for unspecified direction",
                );
            }
            if path.is_ident("readable") || path.is_ident("writable") {
                return self.err(attr, "buffer extents require a parameter or field owner");
            }
            if path.is_ident("range") {
                return self.err(attr, "integer ranges require a parameter or field owner");
            }
            if path.is_ident("out_ptr") {
                self.emit_returned_pointer_attribute(has_attribute, attr)?;
                continue;
            }
            if path.is_ident("success") {
                if !matches!(
                    has_attribute,
                    metadata::writer::HasAttribute::TypeDef(_)
                        | metadata::writer::HasAttribute::MethodDef(_)
                ) {
                    return self.err(attr, "success requires a callable or return-type owner");
                }
                let expression: TokenStream = attr
                    .parse_args()
                    .map_err(|_| self.error(attr, "success requires a return comparison"))?;
                let condition = SuccessCondition::from_expression(&expression)
                    .ok_or_else(|| self.error(attr, "unsupported success predicate"))?;
                self.encode_named_attribute(
                    has_attribute,
                    &AttributeRef {
                        type_name: metadata::TypeName::named(
                            METADATA_NAMESPACE,
                            "SuccessConditionAttribute",
                        ),
                        args: vec![(String::new(), condition.to_value())],
                    },
                );
                continue;
            }
            if path.is_ident("null_terminated") || path.is_ident("nullable") {
                self.emit_pointer_contract(has_attribute, attr, path.is_ident("null_terminated"))?;
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

    fn emit_returned_pointer_attribute(
        &mut self,
        target: metadata::writer::HasAttribute,
        attr: &syn::Attribute,
    ) -> Result<(), Error> {
        let flags = match &attr.meta {
            syn::Meta::Path(_) => syn::punctuated::Punctuated::new(),
            syn::Meta::List(_) => attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated,
                )
                .map_err(|error| self.error(attr, &error.to_string()))?,
            syn::Meta::NameValue(_) => {
                return self.err(attr, "out_ptr requires optional flag names");
            }
        };
        let (mut nullable, mut null_on_failure) = (false, false);
        for flag in flags {
            match flag.to_string().as_str() {
                "nullable" if !nullable => nullable = true,
                "null_on_failure" if !null_on_failure => null_on_failure = true,
                _ => return self.err(flag, "unknown or duplicate out_ptr flag"),
            }
        }
        let expanded = syn::parse_quote! {
            #[Windows::Win32::Metadata::NativeReturnedPointer(
                MayBeNull = #nullable, NullOnFailure = #null_on_failure
            )]
        };
        let attr_ref = self.resolve_attribute_ref(&expanded)?;
        self.encode_named_attribute(target, &attr_ref);
        Ok(())
    }

    fn emit_pointer_contract(
        &mut self,
        target: metadata::writer::HasAttribute,
        attr: &syn::Attribute,
        termination: bool,
    ) -> Result<(), Error> {
        let (entry, success, indirect) = self.pointer_contract_flags(attr, termination)?;
        if !matches!(target, metadata::writer::HasAttribute::Param(_)) {
            return self.err(attr, "pointer contract requires a parameter owner");
        }
        let on_entry = entry.is_some();
        let on_success = success.is_some();
        let double_entry = (entry == Some(true)).then(|| quote! { , DoubleNullOnEntry = true });
        let double_success =
            (success == Some(true)).then(|| quote! { , DoubleNullOnSuccess = true });
        let indirect = indirect.then(|| quote! { , Indirect = true });
        let name = if termination {
            quote! { Windows::Win32::Metadata::NativeNullTerminated }
        } else {
            quote! { Windows::Win32::Metadata::Nullable }
        };
        let expanded = syn::parse_quote! {
            #[#name(
                OnEntry = #on_entry, OnSuccess = #on_success #double_entry #double_success #indirect
            )]
        };
        let attr_ref = self.resolve_attribute_ref(&expanded)?;
        self.encode_named_attribute(target, &attr_ref);
        Ok(())
    }

    pub(super) fn pointer_contract_flags(
        &self,
        attr: &syn::Attribute,
        termination: bool,
    ) -> Result<(Option<bool>, Option<bool>, bool), Error> {
        let flags = attr
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated,
            )
            .map_err(|error| self.error(attr, &error.to_string()))?;
        let (mut entry, mut success) = (None, None);
        let mut indirect = false;
        for flag in flags {
            match flag.to_string().as_str() {
                "entry" if entry.is_none() => entry = Some(false),
                "double_entry" if termination && entry.is_none() => entry = Some(true),
                "success" if success.is_none() => success = Some(false),
                "double_success" if termination && success.is_none() => success = Some(true),
                "indirect" if !indirect => indirect = true,
                _ => return self.err(flag, "unknown or duplicate pointer contract flag"),
            }
        }
        if entry.is_none() && success.is_none() {
            return self.err(attr, "pointer contract requires an entry or success phase");
        }
        Ok((entry, success, indirect))
    }
}
