use super::*;
use std::path::{Path, PathBuf};

mod remap;
pub use remap::Remapper;

type MethodKey = (usize, usize);

#[derive(Default)]
struct CopyContext {
    methods: HashMap<MethodKey, writer::MethodDef>,
    semantics: Vec<(u16, MethodKey, writer::HasSemantics, String)>,
}

impl CopyContext {
    fn method_key(method: reader::MethodDef) -> MethodKey {
        (method.file() as *const reader::File as usize, method.pos())
    }

    fn method(&mut self, source: reader::MethodDef, target: writer::MethodDef) {
        self.methods.insert(Self::method_key(source), target);
    }

    fn semantics(
        &mut self,
        semantics: u16,
        method: reader::MethodDef,
        association: writer::HasSemantics,
        description: String,
    ) {
        self.semantics.push((
            semantics,
            Self::method_key(method),
            association,
            description,
        ));
    }

    fn finish(self, file: &mut writer::File) -> Result<(), Error> {
        for (semantics, method, association, description) in self.semantics {
            let method =
                self.methods.get(&method).copied().ok_or_else(|| {
                    Error::new(format!("missing accessor method for {description}"))
                })?;
            file.MethodSemantics(semantics, method, association);
        }
        Ok(())
    }
}

pub struct Error(String);

impl Error {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl std::error::Error for Error {}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nerror: {}", self.0)
    }
}

#[derive(Default)]
pub struct Merger {
    input: Vec<PathBuf>,
    /// `(path, arch_bits)` where bits are 1=X86, 2=X64, 4=Arm64.
    arch_inputs: Vec<(PathBuf, i32)>,
    output: PathBuf,
    union_enums: bool,
}

impl Merger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.input.push(input.as_ref().to_path_buf());
        self
    }

    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        for input in inputs {
            self.input(input);
        }
        self
    }

    /// Adds an architecture-tagged input winmd file.
    pub fn arch_input(&mut self, path: impl AsRef<Path>, arch: i32) -> &mut Self {
        self.arch_inputs.push((path.as_ref().to_path_buf(), arch));
        self
    }

    /// Unions same-named enums across inputs into a single enum, deduplicating members.
    ///
    /// Without this, two inputs that each define an enum with the same namespace and name
    /// produce two `TypeDef` rows. `tool_win32` uses this to reconcile a value type an `um`
    /// header truncates (for example `FILE_INFORMATION_CLASS`) with the complete `km`
    /// definition, yielding one enum carrying every member.
    pub fn union_enums(&mut self) -> &mut Self {
        self.union_enums = true;
        self
    }

    pub fn output(&mut self, output: impl AsRef<Path>) -> &mut Self {
        self.output = output.as_ref().to_path_buf();
        self
    }

    pub fn merge(&self) -> Result<(), Error> {
        if self.output.as_os_str().is_empty() {
            return Err(Error::new("output is required"));
        }

        let name = self
            .output
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| {
                Error::new(format!("invalid output path `{}`", self.output.display()))
            })?;

        let files = read_inputs(&self.input)?;
        let index = reader::Index::new(files);

        let mut file = writer::File::new(name);
        let mut context = CopyContext::default();

        if self.union_enums {
            let mut groups: BTreeMap<(String, String), Vec<reader::TypeDef<'_>>> = BTreeMap::new();
            for ty in index.types() {
                groups
                    .entry((ty.namespace().to_string(), ty.name().to_string()))
                    .or_default()
                    .push(ty);
            }

            for copies in groups.values() {
                // The per-namespace `Apis` container is defined by both the `um` and `km` inputs;
                // union its fields and methods so both function/constant surfaces survive. Each
                // member keeps its own arch tag, so no arch sub-grouping applies here.
                if copies
                    .iter()
                    .all(|ty| ty.category() == reader::TypeCategory::Class)
                {
                    write_class_union(&mut file, &mut context, &index, copies);
                    continue;
                }

                // Sub-group by architecture so arch-specific variants (an enum whose members
                // differ per arch, like `INTERLOCKED_RESULT`) are never unioned across arches;
                // each arch keeps its own copy, tagged as the inputs had it.
                let mut by_arch: BTreeMap<i32, Vec<reader::TypeDef<'_>>> = BTreeMap::new();
                for copy in copies {
                    by_arch
                        .entry(type_arch_bits(*copy))
                        .or_default()
                        .push(*copy);
                }

                for arch_copies in by_arch.values() {
                    if arch_copies
                        .iter()
                        .all(|ty| ty.category() == reader::TypeCategory::Enum)
                    {
                        write_enum_union(&mut file, arch_copies)?;
                    } else {
                        // Remaining non-enum collisions are not expected within one arch (the `km`
                        // scrape excludes reference types other than extended enums); keep the
                        // first deterministically.
                        write_type(&mut file, &mut context, &index, arch_copies[0], None, None);
                    }
                }
            }
        } else {
            let mut types: Vec<reader::TypeDef<'_>> = index.types().collect();
            types.sort_by(|a, b| (a.namespace(), a.name()).cmp(&(b.namespace(), b.name())));

            for ty in types {
                write_type(&mut file, &mut context, &index, ty, None, None);
            }
        }

        if !self.arch_inputs.is_empty() {
            let all_arches_mask: i32 = self.arch_inputs.iter().fold(0, |acc, (_, arch)| acc | arch);

            let mut arch_groups: Vec<(reader::Index, i32)> =
                Vec::with_capacity(self.arch_inputs.len());
            for (path, arch_bits) in &self.arch_inputs {
                let files = read_inputs(std::slice::from_ref(path))?;
                arch_groups.push((reader::Index::new(files), *arch_bits));
            }

            let mut groups: BTreeMap<
                (String, String),
                Vec<(&reader::Index, reader::TypeDef<'_>, i32)>,
            > = BTreeMap::new();
            for (idx, arch_bits) in &arch_groups {
                for ty in idx.types() {
                    groups
                        .entry((ty.namespace().to_string(), ty.name().to_string()))
                        .or_default()
                        .push((idx, ty, *arch_bits));
                }
            }

            for copies in groups.values() {
                let (idx, ty, _) = copies[0];
                if ty.category() == reader::TypeCategory::Class {
                    // Apis members can diverge by arch; union them instead of taking one copy.
                    write_type_arch_merged(
                        &mut file,
                        &mut context,
                        idx,
                        ty,
                        copies,
                        all_arches_mask,
                    );
                } else if let Some(signature) = merge_native_sized_callback(copies) {
                    let bits = copies.iter().fold(0, |acc, (_, _, bits)| acc | bits);
                    let arch = if bits == all_arches_mask { 0 } else { bits };
                    write_type_with_signature(
                        &mut file,
                        &mut context,
                        idx,
                        ty,
                        None,
                        Some(arch),
                        Some(&signature),
                    );
                } else {
                    // Split value types by shape so arch-specific layouts are not lost.
                    let mut by_sig: Vec<(String, &reader::Index, reader::TypeDef, i32)> = vec![];
                    for (cidx, c, bits) in copies {
                        let sig = type_sig(cidx, *c);
                        if let Some(entry) = by_sig.iter_mut().find(|(s, ..)| *s == sig) {
                            entry.3 |= *bits;
                        } else {
                            by_sig.push((sig, cidx, *c, *bits));
                        }
                    }
                    for (_, cidx, c, bits) in &by_sig {
                        let arch = if *bits == all_arches_mask { 0 } else { *bits };
                        write_type(&mut file, &mut context, cidx, *c, None, Some(arch));
                    }
                }
            }
        }

        context.finish(&mut file)?;
        let bytes = file.into_stream();
        std::fs::write(&self.output, bytes)
            .map_err(|e| Error::new(format!("failed to write `{}`: {e}", self.output.display())))
    }
}

fn read_inputs(inputs: &[PathBuf]) -> Result<Vec<reader::File>, Error> {
    let mut result = vec![];

    for input in inputs {
        if input.is_dir() {
            let prev_len = result.len();

            let entries = std::fs::read_dir(input).map_err(|e| {
                Error::new(format!(
                    "failed to read directory `{}`: {e}",
                    input.display()
                ))
            })?;

            for entry in entries.flatten() {
                let entry_path = entry.path();

                if entry_path.is_file()
                    && entry_path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("winmd"))
                {
                    let file = reader::File::read(&entry_path).ok_or_else(|| {
                        Error::new(format!("failed to read `{}`", entry_path.display()))
                    })?;
                    result.push(file);
                }
            }

            if result.len() == prev_len {
                return Err(Error::new(format!(
                    "no .winmd files found in directory `{}`",
                    input.display()
                )));
            }
        } else {
            let file = reader::File::read(input)
                .ok_or_else(|| Error::new(format!("failed to read `{}`", input.display())))?;
            result.push(file);
        }
    }

    Ok(result)
}

/// Writes a `TypeDef`, using `arch_override` to replace any existing arch attribute.
fn write_type(
    file: &mut writer::File,
    context: &mut CopyContext,
    index: &reader::Index,
    def: reader::TypeDef,
    outer: Option<writer::TypeDef>,
    arch_override: Option<i32>,
) {
    write_type_with_signature(file, context, index, def, outer, arch_override, None);
}

fn write_type_with_signature(
    file: &mut writer::File,
    context: &mut CopyContext,
    index: &reader::Index,
    def: reader::TypeDef,
    outer: Option<writer::TypeDef>,
    arch_override: Option<i32>,
    signature_override: Option<&Signature>,
) {
    let extends = def
        .extends()
        .map(|extends| {
            writer::TypeDefOrRef::TypeRef(file.TypeRef(extends.namespace(), extends.name()))
        })
        .unwrap_or_default();

    debug_assert!(
        !def.flags().is_nested() || def.namespace().is_empty(),
        "nested type should have empty namespace"
    );
    debug_assert!(
        def.flags().is_nested() || !def.namespace().is_empty(),
        "non-nested type should have non-empty namespace"
    );

    let type_def = file.TypeDef(def.namespace(), def.name(), extends, def.flags());

    if let Some(outer) = outer {
        file.NestedClass(type_def, outer);
    }

    for field in def.fields() {
        write_field(file, field, None);
    }

    let generics: Vec<_> = def
        .generic_params()
        .map(|param| Type::Generic(param.name().to_string(), param.sequence()))
        .collect();

    write_attributes_with_arch(
        file,
        writer::HasAttribute::TypeDef(type_def),
        def,
        arch_override,
    );

    for map in def.interface_impls() {
        let interface_impl = file.InterfaceImpl(type_def, &map.interface(&generics));
        write_attributes(
            file,
            writer::HasAttribute::InterfaceImpl(interface_impl),
            map,
        );
    }

    for generic in def.generic_params() {
        file.GenericParam(
            generic.name(),
            writer::TypeOrMethodDef::TypeDef(type_def),
            generic.sequence(),
            generic.flags(),
        );
    }

    for method in def.methods() {
        write_method_with_signature(
            file,
            context,
            method,
            &generics,
            None,
            signature_override.filter(|_| method.name() == "Invoke"),
        );
    }
    write_semantics(file, context, type_def, def, &generics);

    if let Some(class_layout) = def.class_layout() {
        file.ClassLayout(
            type_def,
            class_layout.packing_size(),
            class_layout.class_size(),
        );
    }

    for inner_def in index.nested(def) {
        debug_assert!(inner_def.namespace().is_empty());
        debug_assert!(inner_def.flags().is_nested());
        write_type(
            file,
            context,
            index,
            inner_def,
            Some(type_def),
            arch_override,
        );
    }
}

fn write_field(file: &mut writer::File, field: reader::Field, arch_override: Option<i32>) {
    let field_def = file.Field(field.name(), &field.ty(), field.flags());
    if let Some(layout) = field.layout() {
        file.FieldLayout(field_def, layout.offset());
    }
    if let Some(constant) = field.constant() {
        file.Constant(writer::HasConstant::Field(field_def), &constant.value());
    }
    write_attributes_with_arch(
        file,
        writer::HasAttribute::Field(field_def),
        field,
        arch_override,
    );
}

/// Returns the `SupportedArchitectureAttribute` bits on a type, or 0 (arch-neutral) if absent.
fn type_arch_bits(def: reader::TypeDef) -> i32 {
    for attribute in def.attributes() {
        let ty = attribute.ctor().parent();
        if ty.namespace() == "Windows.Win32.Metadata"
            && ty.name() == "SupportedArchitectureAttribute"
            && let Some((_, Value::I32(bits))) = attribute.value().first()
        {
            return *bits;
        }
    }
    0
}

/// Extracts the integer value of an enum member for comparison, or `None` for non-integer values.
fn enum_member_i64(value: &Value) -> Option<i64> {
    match value {
        Value::U8(v) => Some(*v as i64),
        Value::I8(v) => Some(*v as i64),
        Value::U16(v) => Some(*v as i64),
        Value::I16(v) => Some(*v as i64),
        Value::U32(v) => Some(*v as i64),
        Value::I32(v) => Some(*v as i64),
        Value::U64(v) => Some(*v as i64),
        Value::I64(v) => Some(*v),
        _ => None,
    }
}

/// Unions same-named class copies (the per-namespace `Apis` container) into one, combining every
/// copy's fields and methods. Each member keeps its own attributes, including any arch tag the
/// input winmd already applied, so the `um` and `km` function/constant surfaces both survive.
fn write_class_union(
    file: &mut writer::File,
    context: &mut CopyContext,
    index: &reader::Index,
    copies: &[reader::TypeDef],
) {
    let def = copies[0];

    let extends = def
        .extends()
        .map(|extends| {
            writer::TypeDefOrRef::TypeRef(file.TypeRef(extends.namespace(), extends.name()))
        })
        .unwrap_or_default();
    let type_def = file.TypeDef(def.namespace(), def.name(), extends, def.flags());

    write_attributes_with_arch(file, writer::HasAttribute::TypeDef(type_def), def, None);

    let generics: Vec<_> = def
        .generic_params()
        .map(|param| Type::Generic(param.name().to_string(), param.sequence()))
        .collect();

    let mut seen_fields: HashSet<String> = HashSet::new();
    for copy in copies {
        for field in copy.fields() {
            let value = field
                .constant()
                .map(|c| format!("{:?}", c.value()))
                .unwrap_or_default();
            let key = format!("{}|{:?}|{value}", field.name(), field.ty());
            if seen_fields.insert(key) {
                write_field(file, field, None);
            }
        }
    }

    let mut seen_methods: HashSet<String> = HashSet::new();
    for copy in copies {
        for method in copy.methods() {
            let key = format!("{}|{:?}", method.name(), method.signature(&generics));
            if seen_methods.insert(key) {
                write_method(file, context, method, &generics, None);
            }
        }
    }
    write_semantics(file, context, type_def, def, &generics);

    for inner_def in index.nested(def) {
        write_type(file, context, index, inner_def, Some(type_def), None);
    }
}

/// Returns `true` if the member name marks a trailing count sentinel in the NT naming style.
///
/// These enums terminate with a member whose value equals the member count, not a real value.
/// A truncated projection carries a smaller sentinel; the fuller definition carries a larger
/// one. The sentinel is the only member allowed to disagree across copies of the same enum.
///
/// The match is limited to the NT sentinel spellings - a `Max` prefix (`MaxKeySetInfoClass`,
/// `MaximumInterfaceType`) or a PascalCase `Maximum` suffix (`PowerSystemMaximum`,
/// `FileMaximumInformation`). A broader `contains("Max")` would also treat the many real enum
/// values that merely contain `MAX`/`_MAX` (`IPPROTO_MAX`, `WBEM_MAX_PATH`, `MaxPayload128Bytes`)
/// as tolerable, silently masking a genuine value conflict.
fn is_max_sentinel(name: &str) -> bool {
    name.starts_with("Max") || name.ends_with("Maximum") || name.ends_with("MaximumInformation")
}

/// Unions same-named enum copies into one enum carrying every member.
///
/// A `um` header often projects a value type in truncated or partial form while the `km` scrape
/// emits more of it; neither is guaranteed to be a superset (for example `THREADINFOCLASS`, where
/// `um` contributes `ThreadNameInformation` that `km` omits). The fullest copy sets the member
/// order, the type flags, and the attributes; members other copies add are appended. A member
/// shared by two copies must agree on its value, except for the trailing `Max*` count sentinel,
/// whose value legitimately grows with the member count. Any other disagreement is a real
/// metadata conflict and is rejected.
fn write_enum_union(file: &mut writer::File, copies: &[reader::TypeDef]) -> Result<(), Error> {
    let base = *copies
        .iter()
        .max_by_key(|copy| copy.fields().count())
        .unwrap();

    let base_members: HashMap<String, Value> = base
        .fields()
        .filter_map(|field| {
            field
                .constant()
                .map(|c| (field.name().to_string(), c.value()))
        })
        .collect();

    // Members present in some copy but not the fullest one, kept in first-appearance order.
    let mut extra_order: Vec<String> = Vec::new();
    let mut extras: HashMap<String, reader::Field> = HashMap::new();

    let conflict = |field: reader::Field| {
        Error::new(format!(
            "enum `{}.{}` member `{}` has conflicting values across inputs",
            base.namespace(),
            base.name(),
            field.name()
        ))
    };

    for copy in copies {
        for field in copy.fields() {
            let Some(constant) = field.constant() else {
                continue;
            };
            let value = constant.value();

            if let Some(base_value) = base_members.get(field.name()) {
                if *base_value == value {
                    continue;
                }
                // The fullest copy's sentinel is authoritative (its count is the largest); a
                // smaller sentinel from a truncated copy is discarded.
                let tolerated = is_max_sentinel(field.name())
                    && matches!(
                        (enum_member_i64(base_value), enum_member_i64(&value)),
                        (Some(b), Some(v)) if b >= v
                    );
                if !tolerated {
                    return Err(conflict(field));
                }
                continue;
            }

            match extras.get(field.name()) {
                None => {
                    extra_order.push(field.name().to_string());
                    extras.insert(field.name().to_string(), field);
                }
                Some(existing) => {
                    let existing_value = existing.constant().unwrap().value();
                    if existing_value == value {
                        continue;
                    }
                    let keep_larger = is_max_sentinel(field.name())
                        && matches!(
                            (enum_member_i64(&existing_value), enum_member_i64(&value)),
                            (Some(a), Some(b)) if a != b
                        );
                    if !keep_larger {
                        return Err(conflict(field));
                    }
                    if enum_member_i64(&value) > enum_member_i64(&existing_value) {
                        extras.insert(field.name().to_string(), field);
                    }
                }
            }
        }
    }

    let extends = base
        .extends()
        .map(|extends| {
            writer::TypeDefOrRef::TypeRef(file.TypeRef(extends.namespace(), extends.name()))
        })
        .unwrap_or_default();

    let type_def = file.TypeDef(base.namespace(), base.name(), extends, base.flags());
    write_attributes_with_arch(file, writer::HasAttribute::TypeDef(type_def), base, None);

    for field in base.fields() {
        write_field(file, field, None);
    }
    for name in &extra_order {
        write_field(file, extras[name], None);
    }

    Ok(())
}

fn write_method(
    file: &mut writer::File,
    context: &mut CopyContext,
    method: reader::MethodDef,
    generics: &[Type],
    arch_override: Option<i32>,
) -> writer::MethodDef {
    write_method_with_signature(file, context, method, generics, arch_override, None)
}

fn write_method_with_signature(
    file: &mut writer::File,
    context: &mut CopyContext,
    method: reader::MethodDef,
    generics: &[Type],
    arch_override: Option<i32>,
    signature_override: Option<&Signature>,
) -> writer::MethodDef {
    let signature;
    let signature = if let Some(signature) = signature_override {
        signature
    } else {
        signature = method.signature(generics);
        &signature
    };
    let method_def = file.MethodDef(
        method.name(),
        signature,
        method.flags(),
        method.impl_flags(),
    );
    context.method(method, method_def);
    for param_def in method.params() {
        let param = file.Param(param_def.name(), param_def.sequence(), param_def.flags());
        write_attributes(file, writer::HasAttribute::Param(param), param_def);
    }
    write_attributes_with_arch(
        file,
        writer::HasAttribute::MethodDef(method_def),
        method,
        arch_override,
    );
    if let Some(impl_map) = method.impl_map() {
        file.ImplMap(
            method_def,
            impl_map.flags(),
            impl_map.import_name(),
            impl_map.import_scope().name(),
        );
    }
    for generic in method.generic_params() {
        file.GenericParam(
            generic.name(),
            writer::TypeOrMethodDef::MethodDef(method_def),
            generic.sequence(),
            generic.flags(),
        );
    }
    method_def
}

fn write_semantics(
    file: &mut writer::File,
    context: &mut CopyContext,
    type_def: writer::TypeDef,
    def: reader::TypeDef,
    generics: &[Type],
) {
    let mut first_property = None;
    for property in def.properties() {
        let property_def = file.PropertyWithSignature(
            property.name(),
            &property.signature(generics),
            property.flags(),
        );
        first_property.get_or_insert(property_def);
        write_attributes(file, writer::HasAttribute::Property(property_def), property);
        if let Some(constant) = property.constant() {
            file.Constant(
                writer::HasConstant::Property(property_def),
                &constant.value(),
            );
        }
        for semantics in property.semantics() {
            context.semantics(
                semantics.semantics(),
                semantics.method(),
                writer::HasSemantics::Property(property_def),
                format!(
                    "property {}.{}.{}",
                    def.namespace(),
                    def.name(),
                    property.name()
                ),
            );
        }
    }
    if let Some(first_property) = first_property {
        file.PropertyMap(type_def, first_property);
    }

    let mut first_event = None;
    for event in def.events() {
        let event_def = file.EventWithFlags(event.name(), &event.ty(generics), event.flags());
        first_event.get_or_insert(event_def);
        write_attributes(file, writer::HasAttribute::Event(event_def), event);
        for semantics in event.semantics() {
            context.semantics(
                semantics.semantics(),
                semantics.method(),
                writer::HasSemantics::Event(event_def),
                format!("event {}.{}.{}", def.namespace(), def.name(), event.name()),
            );
        }
    }
    if let Some(first_event) = first_event {
        file.EventMap(type_def, first_event);
    }
}

/// Reconciles an unmanaged callback whose SDK signature explicitly uses a native-sized integer on
/// at least one architecture and the same-width fixed integer on the others.
///
/// This is intentionally not a general `i32`/`i64` merge heuristic. The `isize`/`usize` spelling
/// supplies the semantic evidence, and each fixed integer must match that input's pointer width.
fn merge_native_sized_callback(
    copies: &[(&reader::Index, reader::TypeDef, i32)],
) -> Option<Signature> {
    if copies.len() < 2
        || copies
            .iter()
            .any(|(_, def, _)| !is_unmanaged_callback(*def))
    {
        return None;
    }

    let first_def = copies[0].1;
    if copies.iter().any(|(_, def, _)| {
        def.flags() != first_def.flags()
            || callback_attributes(*def) != callback_attributes(first_def)
    }) {
        return None;
    }

    let methods: Vec<_> = copies
        .iter()
        .map(|(_, def, bits)| {
            let mut methods = def.methods();
            let method = methods.next()?;
            (method.name() == "Invoke" && methods.next().is_none()).then_some((method, *bits))
        })
        .collect::<Option<_>>()?;

    let first = methods[0].0;
    if methods.iter().any(|(method, _)| {
        method.flags() != first.flags()
            || method.impl_flags() != first.impl_flags()
            || callback_attributes(*method) != callback_attributes(first)
            || callback_params(*method) != callback_params(first)
    }) {
        return None;
    }

    let signatures: Vec<_> = methods
        .iter()
        .map(|(method, bits)| (method.signature(&[]), *bits))
        .collect();
    let flags = signatures[0].0.flags;
    if signatures.iter().any(|(signature, _)| {
        signature.flags != flags || signature.types.len() != signatures[0].0.types.len()
    }) {
        return None;
    }

    let (return_type, mut changed) = merge_native_sized_type(
        &signatures
            .iter()
            .map(|(signature, bits)| (&signature.return_type, *bits))
            .collect::<Vec<_>>(),
    )?;

    let mut types = Vec::with_capacity(signatures[0].0.types.len());
    for index in 0..signatures[0].0.types.len() {
        let (ty, position_changed) = merge_native_sized_type(
            &signatures
                .iter()
                .map(|(signature, bits)| (&signature.types[index], *bits))
                .collect::<Vec<_>>(),
        )?;
        changed |= position_changed;
        types.push(ty);
    }

    changed.then_some(Signature {
        flags,
        return_type,
        types,
    })
}

fn is_unmanaged_callback(def: reader::TypeDef) -> bool {
    def.category() == reader::TypeCategory::Delegate
        && def.attributes().any(|attribute| {
            let ty = attribute.ctor().parent();
            ty.namespace() == "System.Runtime.InteropServices"
                && ty.name() == "UnmanagedFunctionPointerAttribute"
        })
}

fn callback_params(
    method: reader::MethodDef,
) -> Vec<(
    String,
    u16,
    ParamAttributes,
    Vec<(String, String, Vec<(String, Value)>)>,
)> {
    method
        .params()
        .map(|param| {
            (
                param.name().to_string(),
                param.sequence(),
                param.flags(),
                callback_attributes(param),
            )
        })
        .collect()
}

fn callback_attributes<'a, R: HasAttributes<'a>>(
    row: R,
) -> Vec<(String, String, Vec<(String, Value)>)> {
    row.attributes()
        .filter_map(|attribute| {
            let ty = attribute.ctor().parent();
            (!(ty.namespace() == "Windows.Win32.Metadata"
                && ty.name() == "SupportedArchitectureAttribute"))
                .then(|| {
                    (
                        ty.namespace().to_string(),
                        ty.name().to_string(),
                        attribute.value(),
                    )
                })
        })
        .collect()
}

fn merge_native_sized_type(copies: &[(&Type, i32)]) -> Option<(Type, bool)> {
    let first = copies.first()?.0;
    if copies.iter().all(|(ty, _)| *ty == first) {
        return Some((first.clone(), false));
    }

    if copies.iter().any(|(ty, _)| **ty == Type::ISize)
        && copies
            .iter()
            .all(|(ty, bits)| native_signed_compatible(ty, *bits))
    {
        return Some((Type::ISize, true));
    }
    if copies.iter().any(|(ty, _)| **ty == Type::USize)
        && copies
            .iter()
            .all(|(ty, bits)| native_unsigned_compatible(ty, *bits))
    {
        return Some((Type::USize, true));
    }
    None
}

fn native_signed_compatible(ty: &Type, bits: i32) -> bool {
    matches!(ty, Type::ISize)
        || matches!(
            (ty, pointer_width(bits)),
            (Type::I32, Some(32)) | (Type::I64, Some(64))
        )
}

fn native_unsigned_compatible(ty: &Type, bits: i32) -> bool {
    matches!(ty, Type::USize)
        || matches!(
            (ty, pointer_width(bits)),
            (Type::U32, Some(32)) | (Type::U64, Some(64))
        )
}

fn pointer_width(bits: i32) -> Option<u8> {
    match bits {
        1 => Some(32),
        2 | 4 => Some(64),
        _ => None,
    }
}

/// Unions arch-specific Apis members and tags members absent from some arches.
fn write_type_arch_merged(
    file: &mut writer::File,
    context: &mut CopyContext,
    index: &reader::Index,
    def: reader::TypeDef,
    copies: &[(&reader::Index, reader::TypeDef, i32)],
    all_mask: i32,
) {
    let extends = def
        .extends()
        .map(|e| writer::TypeDefOrRef::TypeRef(file.TypeRef(e.namespace(), e.name())))
        .unwrap_or_default();
    let type_def = file.TypeDef(def.namespace(), def.name(), extends, def.flags());

    let generics: Vec<_> = def
        .generic_params()
        .map(|p| Type::Generic(p.name().to_string(), p.sequence()))
        .collect();

    write_attributes_with_arch(file, writer::HasAttribute::TypeDef(type_def), def, Some(0));
    for map in def.interface_impls() {
        let interface_impl = file.InterfaceImpl(type_def, &map.interface(&generics));
        write_attributes(
            file,
            writer::HasAttribute::InterfaceImpl(interface_impl),
            map,
        );
    }
    for generic in def.generic_params() {
        file.GenericParam(
            generic.name(),
            writer::TypeOrMethodDef::TypeDef(type_def),
            generic.sequence(),
            generic.flags(),
        );
    }

    // Include constant values in the key so divergent constants survive.
    let mut fields: BTreeMap<String, (reader::Field, i32)> = BTreeMap::new();
    for (_, ty, bits) in copies {
        for field in ty.fields() {
            let val = field
                .constant()
                .map(|c| format!("{:?}", c.value()))
                .unwrap_or_default();
            let key = format!("{}|{:?}|{val}", field.name(), field.ty());
            fields.entry(key).or_insert((field, 0)).1 |= bits;
        }
    }
    for (field, bits) in fields.into_values() {
        write_field(file, field, Some(if bits == all_mask { 0 } else { bits }));
    }

    let mut method_copies: BTreeMap<String, (reader::MethodDef, i32)> = BTreeMap::new();
    for (_, ty, bits) in copies {
        for method in ty.methods() {
            let key = format!("{}|{:?}", method.name(), method.signature(&generics));
            method_copies.entry(key).or_insert((method, 0)).1 |= bits;
        }
    }
    for (method, bits) in method_copies.into_values() {
        write_method(
            file,
            context,
            method,
            &generics,
            Some(if bits == all_mask { 0 } else { bits }),
        );
    }
    write_semantics(file, context, type_def, def, &generics);

    if let Some(class_layout) = def.class_layout() {
        file.ClassLayout(
            type_def,
            class_layout.packing_size(),
            class_layout.class_size(),
        );
    }

    for inner_def in index.nested(def) {
        write_type(file, context, index, inner_def, Some(type_def), Some(0));
    }
}

/// Signatures include layout, methods, constants, alignment, and nested shapes so arch-specific
/// value types do not collapse into one neutral definition.
fn type_sig(index: &reader::Index, def: reader::TypeDef) -> String {
    let fields: Vec<String> = def
        .fields()
        .map(|f| {
            let val = f
                .constant()
                .map(|c| format!("{:?}", c.value()))
                .unwrap_or_default();
            format!("{}:{:?}={val}", f.name(), f.ty())
        })
        .collect();
    let methods: Vec<String> = def
        .methods()
        .map(|m| format!("{}:{:?}", m.name(), m.signature(&[])))
        .collect();
    let layout = def
        .class_layout()
        .map(|l| (l.packing_size(), l.class_size()));
    let align = def
        .find_attribute("AlignmentAttribute")
        .map(|a| format!("{:?}", a.value()));
    // Recurse into nested shapes; outer fields reference only invariant nested leaf names.
    let nested: Vec<String> = index
        .nested(def)
        .map(|inner| format!("{}={}", inner.name(), type_sig(index, inner)))
        .collect();
    format!(
        "{fields:?}|{methods:?}|{layout:?}|{align:?}|{:?}|{nested:?}",
        def.flags()
    )
}

fn write_attributes<'a, R: HasAttributes<'a>>(
    file: &mut writer::File,
    parent: writer::HasAttribute,
    row: R,
) {
    write_attributes_with_arch(file, parent, row, None);
}

/// Copies attributes, optionally replacing `SupportedArchitectureAttribute`.
fn write_attributes_with_arch<'a, R: HasAttributes<'a>>(
    file: &mut writer::File,
    parent: writer::HasAttribute,
    row: R,
    arch_override: Option<i32>,
) {
    for attribute in row.attributes() {
        let ctor = attribute.ctor();
        let ty = ctor.parent();

        if arch_override.is_some()
            && ty.namespace() == "Windows.Win32.Metadata"
            && ty.name() == "SupportedArchitectureAttribute"
        {
            continue;
        }

        let attribute_ref =
            writer::MemberRefParent::TypeRef(file.TypeRef(ty.namespace(), ty.name()));

        let ctor_ref = file.MemberRef(".ctor", &ctor.signature(&[]), attribute_ref);

        file.Attribute(
            parent,
            writer::AttributeType::MemberRef(ctor_ref),
            &attribute.value(),
        );
    }

    if let Some(arch_bits) = arch_override
        && arch_bits != 0
    {
        write_supported_architecture_attr(file, parent, arch_bits);
    }
}

fn write_supported_architecture_attr(
    file: &mut writer::File,
    parent: writer::HasAttribute,
    arch_bits: i32,
) {
    let ns = "Windows.Win32.Metadata";
    let name = "SupportedArchitectureAttribute";

    let type_ref = writer::MemberRefParent::TypeRef(file.TypeRef(ns, name));

    let sig = Signature {
        flags: MethodCallAttributes::HASTHIS,
        return_type: Type::Void,
        types: vec![Type::I32],
    };

    let ctor_ref = file.MemberRef(".ctor", &sig, type_ref);

    file.Attribute(
        parent,
        writer::AttributeType::MemberRef(ctor_ref),
        &[(String::new(), Value::I32(arch_bits))],
    );
}
