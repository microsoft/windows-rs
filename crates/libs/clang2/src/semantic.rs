use super::*;
use metadata::HasAttributes;
use metadata::reader::{Index, Item, MethodDef, TypeCategory, TypeDef};

impl Snapshot {
    /// Checks the frozen pilot's named natural-storage output against the original native graph.
    pub fn verify_winmd(&self, namespace: &str, index: &Index) -> Result<()> {
        let check = Inspector {
            snapshot: self,
            namespace,
            index,
        };
        let mut types = BTreeSet::new();
        let mut functions = BTreeSet::new();
        for declaration in &self.declarations {
            let name = check.name(declaration.evidence.id)?;
            match &declaration.kind {
                DeclarationKind::Function(observations) => {
                    let Item::Fn(method) = check.item(&name)? else {
                        return invalid("native function has the wrong metadata kind");
                    };
                    functions.insert(key(&name));
                    for observation in observations {
                        check.function(method, observation, false)?;
                    }
                }
                kind => {
                    let Item::Type(ty) = check.item(&name)? else {
                        return invalid("native type has the wrong metadata kind");
                    };
                    types.insert(key(&name));
                    match kind {
                        DeclarationKind::Alias { written, .. } => {
                            if let NativeKind::Record(target) | NativeKind::Enum(target) =
                                written.kind
                                && written.qualifiers == Qualifiers::default()
                                && check.name(target)? == check.name(declaration.evidence.id)?
                            {
                                continue;
                            }
                            if ty.category() != TypeCategory::Struct
                                || ty.methods().next().is_some()
                                || ty.find_attribute("NativeTypedefAttribute").is_none()
                            {
                                return invalid("written typedef identity was erased");
                            }
                            let fields = ty.fields().collect::<Vec<_>>();
                            if fields.len() != 1
                                || fields[0].name() != "Value"
                                || fields[0].constant().is_some()
                                || fields[0]
                                    .flags()
                                    .contains(metadata::FieldAttributes::Static)
                            {
                                return invalid("typedef has the wrong metadata shape");
                            }
                            check.ty(written, &fields[0].ty())?;
                        }
                        DeclarationKind::Enum {
                            underlying,
                            scoped,
                            members,
                        } => {
                            if ty.category() != TypeCategory::Enum
                                || ty.methods().next().is_some()
                                || ty.fields().len() != members.len() + 1
                                || ty.find_attribute("ScopedEnumAttribute").is_some() != *scoped
                            {
                                return invalid("enum kind or scopedness changed");
                            }
                            check.ty(
                                underlying,
                                &ty.underlying_type()
                                    .ok_or_else(|| failure("enum backing type missing"))?,
                            )?;
                            let fields = ty
                                .fields()
                                .filter(|field| field.constant().is_some())
                                .collect::<Vec<_>>();
                            if fields.len() != members.len() {
                                return invalid("enum member count changed");
                            }
                            for (field, member) in fields.iter().zip(members) {
                                if field.name() != member.name
                                    || field.constant().unwrap().value() != member.value
                                {
                                    return invalid("enum member name/order/value changed");
                                }
                            }
                        }
                        DeclarationKind::Record {
                            union,
                            fields,
                            contract,
                        } => {
                            if ty.category() != TypeCategory::Struct
                                || ty.methods().next().is_some()
                                || ty
                                    .flags()
                                    .contains(metadata::TypeAttributes::ExplicitLayout)
                                    != *union
                            {
                                return invalid("record kind changed");
                            }
                            let observed = ty.fields().collect::<Vec<_>>();
                            if observed.len() != fields.len() {
                                return invalid("record field count changed");
                            }
                            let mut size = 0;
                            let mut alignment = 1;
                            let pack = ty.class_layout().map_or(0, |layout| layout.packing_size());
                            if pack != contract.and_then(|contract| contract.packing).unwrap_or(0)
                                || ty
                                    .class_layout()
                                    .is_some_and(|layout| layout.class_size() != 0)
                                || contract.is_some_and(|contract| contract.alignment.is_some())
                                || ty.find_attribute("AlignmentAttribute").is_some()
                            {
                                return invalid(
                                    "record packing/size request changed or adjusted alignment is outside this gate",
                                );
                            }
                            for (field, native) in observed.iter().zip(fields) {
                                if native.evidence.anonymous
                                    || field.name() != native.evidence.name
                                    || field.constant().is_some()
                                    || field.flags().contains(metadata::FieldAttributes::Static)
                                {
                                    return invalid("record field identity/order changed");
                                }
                                check.ty(&native.written, &field.ty())?;
                                let layout = check.layout(&field.ty(), &mut BTreeSet::new())?;
                                let align = if pack == 0 {
                                    layout.alignment
                                } else {
                                    layout.alignment.min(u64::from(pack))
                                };
                                alignment = alignment.max(align);
                                let offset = if *union { 0 } else { round(size, align)? };
                                if native.evidence.offset != offset {
                                    return invalid(
                                        "compiled record field offset differs from compiler",
                                    );
                                }
                                size = if *union {
                                    size.max(layout.size)
                                } else {
                                    offset
                                        .checked_add(layout.size)
                                        .ok_or_else(|| failure("record size overflow"))?
                                };
                            }
                            let observed = Layout {
                                size: round(size.max(1), alignment)?,
                                alignment,
                            };
                            if declaration.evidence.layout != Some(observed) {
                                return invalid(&format!(
                                    "{}: compiled storage {:?} differs from compiler {:?}",
                                    declaration.evidence.qualified_name,
                                    observed,
                                    declaration.evidence.layout
                                ));
                            }
                        }
                        DeclarationKind::Opaque { union } => {
                            if ty.category() != TypeCategory::Struct
                                || ty.methods().next().is_some()
                                || ty.find_attribute("NativeOpaqueAttribute").is_none()
                                || ty.fields().next().is_some()
                                || ty
                                    .flags()
                                    .contains(metadata::TypeAttributes::ExplicitLayout)
                                    != *union
                            {
                                return invalid("opaque nominal identity changed");
                            }
                        }
                        DeclarationKind::Interface {
                            evidence,
                            base,
                            methods,
                            ..
                        } => {
                            if ty.category() != TypeCategory::Interface {
                                return invalid("interface kind changed");
                            }
                            let table = ty
                                .native_vtable()
                                .map_err(|error| failure(&error.to_string()))?
                                .ok_or_else(|| failure("native vtable missing"))?;
                            if (table.size, table.inherited)
                                != (evidence.vtable_size, evidence.inherited)
                            {
                                return invalid("native vtable extent changed");
                            }
                            let guid = ty
                                .find_attribute("GuidAttribute")
                                .ok_or_else(|| failure("interface GUID missing"))?;
                            let values = guid
                                .value()
                                .into_iter()
                                .map(|(_, value)| value)
                                .collect::<Vec<_>>();
                            if values != guid_values(&evidence.guid)? {
                                return invalid("compiler interface GUID changed");
                            }
                            let bases = ty
                                .interface_impls()
                                .map(|base| base.interface(&[]))
                                .collect::<Vec<_>>();
                            let expected = base.map(|id| check.name(id)).transpose()?;
                            if bases
                                != expected
                                    .into_iter()
                                    .map(metadata::Type::ClassName)
                                    .collect::<Vec<_>>()
                            {
                                return invalid("native interface base changed");
                            }
                            let observed = ty.methods().collect::<Vec<_>>();
                            if observed.len() != methods.len() {
                                return invalid("native interface method count changed");
                            }
                            for (method, native) in observed.iter().zip(methods) {
                                let slot = method
                                    .native_slot()
                                    .map_err(|error| failure(&error.to_string()))?
                                    .ok_or_else(|| failure("native method slot missing"))?;
                                if method.name() != native.evidence.name
                                    || slot.index != native.evidence.slot
                                    || slot.reuse.is_some()
                                {
                                    return invalid("native method identity/order/slot changed");
                                }
                                check.function(*method, &native.function, true)?;
                            }
                        }
                        DeclarationKind::Function(_) => unreachable!(),
                    }
                }
            }
        }
        let mut constants = BTreeSet::new();
        for evidence in &self.macros {
            if let MacroKind::Integer { value, .. } = &evidence.kind {
                let name = metadata::TypeName::named(namespace, &evidence.name);
                let Item::Const(field) = check.item(&name)? else {
                    return invalid("macro value has the wrong metadata kind");
                };
                if field.ty() != value.ty()
                    || field.constant().map(|constant| constant.value()) != Some(value.clone())
                {
                    return invalid("compiler macro type or value changed");
                }
                constants.insert(key(&name));
            }
        }
        for (ns, name, ty) in index.iter() {
            if name == "Apis" {
                for method in ty.methods() {
                    if !functions.contains(&(ns.to_string(), method.name().to_string())) {
                        return invalid("unexpected compiled function");
                    }
                }
                for field in ty.fields() {
                    if !constants.contains(&(ns.to_string(), field.name().to_string())) {
                        return invalid("unexpected compiled constant");
                    }
                }
            } else if !types.contains(&(ns.to_string(), name.to_string())) {
                return invalid("unexpected compiled type");
            }
        }
        Ok(())
    }

    /// Checks architecture-neutral written contracts, allowing only the pinned ULONG_PTR width gate.
    pub fn agree_architecture(&self, other: &Self) -> Result<()> {
        if self.compiler != other.compiler || self.target == other.target {
            return invalid(
                "architecture agreement requires distinct targets and the same compiler",
            );
        }
        self.check()?;
        other.check()?;
        if self.architecture_contracts()? != other.architecture_contracts()? {
            return invalid(
                "cross-target written source contracts differ outside the ULONG_PTR/ABI policy",
            );
        }
        Ok(())
    }

    /// Returns checked architecture-neutral contracts for the frozen Windows stdcall pilot.
    pub fn architecture_contracts(&self) -> Result<BTreeMap<String, String>> {
        fn ty(snapshot: &Snapshot, native: &NativeType) -> String {
            let shape = match &native.kind {
                NativeKind::Void => "void".into(),
                NativeKind::Scalar(kind) => format!("scalar:{kind}"),
                NativeKind::Alias(id) | NativeKind::Record(id) | NativeKind::Enum(id) => format!(
                    "named:{}",
                    snapshot.declarations[*id].evidence.qualified_name
                ),
                NativeKind::Pointer(inner) => format!("pointer:{}", ty(snapshot, inner)),
                NativeKind::Reference(inner) => format!("reference:{}", ty(snapshot, inner)),
                NativeKind::Array(inner, count) => format!("array:{count}:{}", ty(snapshot, inner)),
            };
            format!("{:?}:{shape}", native.qualifiers)
        }
        fn function(snapshot: &Snapshot, native: &NativeFunction) -> String {
            // The frozen source prototypes use stdcall, normalized to Win64 on x64/ARM64.
            let convention = match native.evidence.convention {
                CallingConvention::Stdcall | CallingConvention::Win64 => "windows",
                CallingConvention::C if snapshot.target == Target::X86 => "C",
                CallingConvention::C => "windows",
                CallingConvention::Fastcall => "fastcall",
            };
            format!(
                "{convention}:{}:{}:{:?}:{}:{:?}",
                native.evidence.variadic,
                native.evidence.dll_import,
                native.evidence.annotations,
                ty(snapshot, &native.result),
                native
                    .params
                    .iter()
                    .map(|param| (ty(snapshot, &param.written), &param.evidence.contract))
                    .collect::<Vec<_>>()
            )
        }
        let mut shapes = BTreeMap::new();
        for declaration in &self.declarations {
            let shape = match &declaration.kind {
                DeclarationKind::Alias { written, .. }
                    if declaration.evidence.qualified_name == "ULONG_PTR" =>
                {
                    if !matches!(
                        written.kind,
                        NativeKind::Scalar(clang_sys::CXType_ULong | clang_sys::CXType_ULongLong)
                    ) || written.layout
                        != Some(Layout {
                            size: u64::from(self.target.pointer_bits() / 8),
                            alignment: u64::from(self.target.pointer_bits() / 8),
                        })
                    {
                        return invalid(
                            "ULONG_PTR is not the compiler's unsigned pointer-width contract",
                        );
                    }
                    "unsigned-pointer-width".into()
                }
                DeclarationKind::Alias { written, .. } => format!("alias:{}", ty(self, written)),
                DeclarationKind::Opaque { union } => format!("opaque:{union}"),
                DeclarationKind::Record {
                    union,
                    fields,
                    contract,
                } => format!(
                    "record:{union}:{:?}:{:?}",
                    contract.map(|contract| (
                        contract.packing,
                        contract.alignment,
                        contract.type_visibility
                    )),
                    fields
                        .iter()
                        .map(|field| (&field.evidence.name, ty(self, &field.written)))
                        .collect::<Vec<_>>()
                ),
                DeclarationKind::Enum {
                    underlying,
                    scoped,
                    members,
                } => format!(
                    "enum:{}:{scoped}:{:?}",
                    ty(self, underlying),
                    members
                        .iter()
                        .map(|member| (&member.name, &member.value))
                        .collect::<Vec<_>>()
                ),
                DeclarationKind::Function(observations) => format!(
                    "functions:{:?}",
                    observations
                        .iter()
                        .map(|native| function(self, native))
                        .collect::<Vec<_>>()
                ),
                DeclarationKind::Interface {
                    evidence,
                    base,
                    methods,
                    ..
                } => format!(
                    "interface:{evidence:?}:{:?}:{:?}",
                    base.map(|id| &self.declarations[id].evidence.qualified_name),
                    methods
                        .iter()
                        .map(|method| (
                            &method.evidence.name,
                            method.evidence.slot,
                            function(self, &method.function)
                        ))
                        .collect::<Vec<_>>()
                ),
            };
            if shapes
                .insert(
                    format!(
                        "{}:{}",
                        declaration.evidence.kind, declaration.evidence.qualified_name
                    ),
                    format!(
                        "{}:{:?}:{shape}",
                        declaration.evidence.kind, declaration.evidence.annotations
                    ),
                )
                .is_some()
            {
                return invalid("ambiguous architecture declaration identity");
            }
        }
        for evidence in &self.macros {
            let shape = match &evidence.kind {
                MacroKind::Integer { value, .. } => format!("value:{value:?}"),
                MacroKind::Alias {
                    qualified_name,
                    kind,
                    ..
                } => format!("alias:{kind}:{qualified_name}"),
            };
            shapes.insert(format!("macro:{}", evidence.name), shape);
        }
        Ok(shapes)
    }
}

struct Inspector<'a> {
    snapshot: &'a Snapshot,
    namespace: &'a str,
    index: &'a Index,
}

impl<'a> Inspector<'a> {
    fn name(&self, id: Id) -> Result<metadata::TypeName> {
        let declaration = &self.snapshot.declarations[id];
        if declaration.anonymous || declaration.owner.is_some() {
            return invalid(
                "semantic readback currently requires named top-level native declarations",
            );
        }
        let mut parts = declaration.names.clone();
        let name = parts.pop().ok_or_else(|| failure("missing native name"))?;
        let namespace = std::iter::once(self.namespace.to_string())
            .chain(parts)
            .collect::<Vec<_>>()
            .join(".");
        Ok(metadata::TypeName::named(&namespace, &name))
    }

    fn item(&self, name: &metadata::TypeName) -> Result<Item<'a>> {
        let items = self
            .index
            .get_item(&name.namespace, &name.name)
            .collect::<Vec<_>>();
        if items.len() != 1 {
            return invalid(&format!("missing or duplicate compiled item: {name:?}"));
        }
        Ok(items[0])
    }

    fn function(&self, method: MethodDef<'_>, native: &NativeFunction, member: bool) -> Result<()> {
        let signature = method.signature(&[]);
        if signature.types.len() != native.params.len()
            || signature
                .flags
                .contains(metadata::MethodCallAttributes::VARARG)
                != native.evidence.variadic
        {
            return invalid("callable arity or variadic contract changed");
        }
        if signature
            .flags
            .contains(metadata::MethodCallAttributes::HASTHIS)
            != member
            || member && method.impl_map().is_some()
        {
            return invalid("callable instance/static contract changed");
        }
        self.ty(&native.result, &signature.return_type)?;
        for (ty, param) in signature.types.iter().zip(&native.params) {
            self.ty(&param.written, ty)?;
        }
        Ok(())
    }

    fn ty(&self, native: &NativeType, observed: &metadata::Type) -> Result<()> {
        use metadata::Type as T;
        let mut observed = observed.clone();
        if native.qualifiers.restrict {
            return invalid("unsupported restrict qualifier in readback");
        }
        if native.qualifiers.constant {
            let T::Const(inner) = observed else {
                return invalid("const qualifier erased");
            };
            observed = *inner;
        }
        if native.qualifiers.volatile {
            let T::Volatile(inner) = observed else {
                return invalid("volatile qualifier erased");
            };
            observed = *inner;
        }
        let matched = match &native.kind {
            NativeKind::Void => observed == T::Void,
            NativeKind::Scalar(kind) => scalar(*kind) == Some(observed.clone()),
            NativeKind::Alias(id) | NativeKind::Record(id) | NativeKind::Enum(id) => {
                observed == T::ValueName(self.name(*id)?)
            }
            NativeKind::Pointer(inner) | NativeKind::Reference(inner) => {
                let (element, constant) = match (&native.kind, observed) {
                    (NativeKind::Pointer(_), T::PtrMut(element, depth)) if depth > 0 => (
                        if depth == 1 {
                            element
                        } else {
                            Box::new(T::PtrMut(element, depth - 1))
                        },
                        false,
                    ),
                    (NativeKind::Pointer(_), T::PtrConst(element, depth)) if depth > 0 => (
                        if depth == 1 {
                            element
                        } else {
                            Box::new(T::PtrConst(element, depth - 1))
                        },
                        true,
                    ),
                    (NativeKind::Reference(_), T::RefMut(element)) => (element, false),
                    (NativeKind::Reference(_), T::RefConst(element)) => (element, true),
                    _ => return invalid("pointer/reference depth or kind changed"),
                };
                if inner.qualifiers.constant != constant {
                    return invalid("pointee constness changed");
                }
                let mut native = inner.as_ref().clone();
                native.qualifiers.constant = false;
                self.ty(&native, &element)?;
                true
            }
            NativeKind::Array(inner, count) => {
                let T::ArrayFixed(element, observed_count) = observed else {
                    return invalid("fixed array shape changed");
                };
                if *count != observed_count {
                    return invalid("fixed array count changed");
                }
                self.ty(inner, &element)?;
                true
            }
        };
        if !matched {
            return invalid(&format!(
                "compiled type differs from native graph: {native:?}"
            ));
        }
        Ok(())
    }

    fn layout(
        &self,
        ty: &metadata::Type,
        visiting: &mut BTreeSet<(String, String)>,
    ) -> Result<Layout> {
        use metadata::Type as T;
        let size = match ty {
            T::Bool | T::I8 | T::U8 => 1,
            T::Char | T::I16 | T::U16 => 2,
            T::I32 | T::U32 | T::F32 => 4,
            T::I64 | T::U64 | T::F64 => 8,
            T::PtrMut(_, _)
            | T::PtrConst(_, _)
            | T::RefMut(_)
            | T::RefConst(_)
            | T::ISize
            | T::USize => u64::from(self.snapshot.target.pointer_bits() / 8),
            T::Const(inner) | T::Volatile(inner) => return self.layout(inner, visiting),
            T::ArrayFixed(inner, count) => {
                let inner = self.layout(inner, visiting)?;
                return Ok(Layout {
                    size: inner
                        .size
                        .checked_mul(*count as u64)
                        .ok_or_else(|| failure("array size overflow"))?,
                    alignment: inner.alignment,
                });
            }
            T::ValueName(name) => {
                if !visiting.insert(key(name)) {
                    return invalid("cyclic compiled storage");
                }
                let Item::Type(definition) = self.item(name)? else {
                    return invalid("storage reference is not a type");
                };
                let result = self.definition_layout(definition, visiting);
                visiting.remove(&key(name));
                return result;
            }
            _ => return invalid("unsupported compiled storage"),
        };
        Ok(Layout {
            size,
            alignment: size,
        })
    }

    fn definition_layout(
        &self,
        ty: TypeDef<'_>,
        visiting: &mut BTreeSet<(String, String)>,
    ) -> Result<Layout> {
        if ty.find_attribute("NativeTypedefAttribute").is_some()
            || ty.category() == TypeCategory::Enum
        {
            return self.layout(
                &ty.underlying_type()
                    .ok_or_else(|| failure("compiled alias/enum backing type missing"))?,
                visiting,
            );
        }
        if ty.category() != TypeCategory::Struct {
            return invalid("non-record compiled storage");
        }
        let union = ty
            .flags()
            .contains(metadata::TypeAttributes::ExplicitLayout);
        let pack = ty.class_layout().map_or(0, |layout| layout.packing_size());
        let mut size = 0;
        let mut alignment = 1;
        for field in ty.fields() {
            let layout = self.layout(&field.ty(), visiting)?;
            let align = if pack == 0 {
                layout.alignment
            } else {
                layout.alignment.min(u64::from(pack))
            };
            alignment = alignment.max(align);
            size = if union {
                size.max(layout.size)
            } else {
                round(size, align)?
                    .checked_add(layout.size)
                    .ok_or_else(|| failure("record size overflow"))?
            };
        }
        Ok(Layout {
            size: round(size.max(1), alignment)?,
            alignment,
        })
    }
}

#[allow(non_upper_case_globals)]
fn scalar(kind: i32) -> Option<metadata::Type> {
    use clang_sys::*;
    use metadata::Type as T;
    Some(match kind {
        CXType_Bool => T::Bool,
        CXType_Char_S | CXType_SChar => T::I8,
        CXType_Char_U | CXType_UChar => T::U8,
        CXType_Short => T::I16,
        CXType_UShort | CXType_WChar => T::U16,
        CXType_Int | CXType_Long => T::I32,
        CXType_UInt | CXType_ULong => T::U32,
        CXType_LongLong => T::I64,
        CXType_ULongLong => T::U64,
        CXType_Float => T::F32,
        CXType_Double => T::F64,
        _ => return None,
    })
}

fn guid_values(guid: &str) -> Result<Vec<metadata::Value>> {
    let digits = guid.replace('-', "");
    if digits.len() != 32 || !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return invalid("invalid native GUID");
    }
    let mut values = vec![
        metadata::Value::U32(
            u32::from_str_radix(&digits[..8], 16).map_err(|error| failure(&error.to_string()))?,
        ),
        metadata::Value::U16(
            u16::from_str_radix(&digits[8..12], 16).map_err(|error| failure(&error.to_string()))?,
        ),
        metadata::Value::U16(
            u16::from_str_radix(&digits[12..16], 16)
                .map_err(|error| failure(&error.to_string()))?,
        ),
    ];
    for start in (16..32).step_by(2) {
        values.push(metadata::Value::U8(
            u8::from_str_radix(&digits[start..start + 2], 16)
                .map_err(|error| failure(&error.to_string()))?,
        ));
    }
    Ok(values)
}

fn round(size: u64, alignment: u64) -> Result<u64> {
    if alignment == 0 || !alignment.is_power_of_two() {
        return invalid("invalid compiled alignment");
    }
    size.checked_add(alignment - 1)
        .map(|size| size & !(alignment - 1))
        .ok_or_else(|| failure("compiled layout overflow"))
}

fn failure(message: &str) -> Error {
    Error::new(Stage::Representation, message)
}
fn invalid<T>(message: &str) -> Result<T> {
    Err(failure(message))
}
fn key(name: &metadata::TypeName) -> (String, String) {
    (name.namespace.clone(), name.name.clone())
}
