#![doc = include_str!("../readme.md")]

mod guid;
mod model;
mod writer;

use model::*;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use windows_metadata::reader::*;
use windows_metadata::{
    FieldAttributes, HasAttributes, MethodAttributes, MethodCallAttributes, Type, TypeAttributes,
    Value,
};

/// Creates a new [`Builder`] for generating a C# projection.
pub fn builder() -> Builder {
    Builder::default()
}

/// Target architecture used to select architecture-specific Win32 metadata rows.
#[derive(Clone, Copy)]
pub enum Architecture {
    X86,
    X64,
    Arm64,
}

impl Architecture {
    fn bit(self) -> i32 {
        match self {
            Self::X86 => 1,
            Self::X64 => 2,
            Self::Arm64 => 4,
        }
    }

    fn host() -> Option<Self> {
        if cfg!(target_arch = "x86") {
            Some(Self::X86)
        } else if cfg!(target_arch = "x86_64") {
            Some(Self::X64)
        } else if cfg!(target_arch = "aarch64") {
            Some(Self::Arm64)
        } else {
            None
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::X86 => "x86",
            Self::X64 => "x64",
            Self::Arm64 => "arm64",
        }
    }
}

/// Tracks whether an exact-selection root (a class or interface named through [`Builder::select`]
/// or [`Builder::member`]) projects every member or only a caller-chosen subset.
#[derive(Clone)]
enum Selection {
    /// Every projectable member: the default-interface members plus collision-free forwarders for
    /// a class, or every member for an interface.
    All,
    /// Only the named projected surface members (property/method/event names).
    Members(HashSet<String>),
}

/// Builder for generating a C# projection from Windows metadata.
///
/// ```rust,no_run
/// windows_csharp::builder()
///     .input("component.winmd")
///     .filter("Component")
///     .output("Component.cs")
///     .write()
///     .unwrap();
/// ```
pub struct Builder {
    input: Vec<String>,
    filter: Vec<String>,
    output: String,
    fragment: bool,
    synchronized: bool,
    selected: HashMap<String, Selection>,
    selected_functions: HashSet<String>,
    selected_constants: HashSet<String>,
    architecture: Option<Architecture>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            input: Vec::new(),
            filter: Vec::new(),
            output: String::new(),
            fragment: false,
            synchronized: false,
            selected: HashMap::new(),
            selected_functions: HashSet::new(),
            selected_constants: HashSet::new(),
            architecture: Architecture::host(),
        }
    }
}

impl Builder {
    /// Adds a metadata input. The path may be a single `.winmd` file or a directory of them.
    pub fn input<S: Into<String>>(mut self, input: S) -> Self {
        self.input.push(input.into());
        self
    }

    /// Restricts generation to types whose namespace matches (equals, or is nested under) the
    /// given namespace. Ignored once an exact type, member, function, or constant selection has
    /// been added: exact selection replaces namespace filtering entirely for that build.
    pub fn filter<S: Into<String>>(mut self, filter: S) -> Self {
        self.filter.push(filter.into());
        self
    }

    /// Selects an exact class, interface, delegate, enum, struct, or opaque handle by its fully
    /// qualified metadata name (for example `"Windows.Foundation.Uri"`). A class projects its
    /// default-interface members plus collision-free forwarders from its other implemented
    /// interfaces; an interface projects every declared member; a value or delegate type projects
    /// its complete definition. Every class/interface/delegate/enum/struct transitively reachable
    /// from the selected signatures is included automatically (a referenced class or interface is
    /// projected as a marker/castable owner unless it is itself selected).
    ///
    /// Calling [`Builder::select`] or [`Builder::member`] at all switches the build into exact-
    /// selection mode, and [`Builder::filter`] is then ignored. Selecting the same type more than
    /// once is fine; the type is still projected once with the union of everything requested for
    /// it. Calling [`Builder::member`] for a type that was given `select`ed narrows it to only the
    /// named members - `select` alone is only a starting point, not a guarantee every member stays.
    pub fn select<S: Into<String>>(mut self, type_name: S) -> Self {
        self.selected
            .entry(type_name.into())
            .or_insert(Selection::All);
        self
    }

    /// Selects one projected member (a property, method, or event's projected surface name) on an
    /// exact WinRT class, WinRT interface, or native interface named by its fully qualified
    /// metadata name. Implicitly adds the type as a selection root, the same as
    /// [`Builder::select`], but narrowed to exactly the members requested through this method -
    /// calling it more than once for the same type accumulates the member names rather than
    /// replacing them. If [`Builder::select`] was also called for this type, the narrowed member set
    /// from this method wins.
    pub fn member<S: Into<String>>(mut self, type_name: S, member_name: S) -> Self {
        let type_name = type_name.into();
        let member_name = member_name.into();
        let selection = self
            .selected
            .entry(type_name)
            .or_insert_with(|| Selection::Members(HashSet::new()));
        match selection {
            Selection::All => {
                let mut members = HashSet::new();
                members.insert(member_name);
                *selection = Selection::Members(members);
            }
            Selection::Members(members) => {
                members.insert(member_name);
            }
        }
        self
    }

    /// Selects one Win32 exported function by its fully qualified metadata name.
    pub fn function<S: Into<String>>(mut self, function_name: S) -> Self {
        self.selected_functions.insert(function_name.into());
        self
    }

    /// Selects one Win32 constant by its fully qualified metadata name.
    pub fn constant<S: Into<String>>(mut self, constant_name: S) -> Self {
        self.selected_constants.insert(constant_name.into());
        self
    }

    /// Selects the target architecture for architecture-specific Win32 metadata. The default is
    /// the architecture running the generator.
    pub fn architecture(mut self, architecture: Architecture) -> Self {
        self.architecture = Some(architecture);
        self
    }

    /// Sets the output `.cs` file path.
    pub fn output<S: Into<String>>(mut self, output: S) -> Self {
        self.output = output.into();
        self
    }

    /// Emits only the projected namespace blocks, omitting the file header and the shared runtime
    /// support. Use this to compose several inputs into one compilation unit alongside a single
    /// copy of [`runtime_support`], or [`synchronized_runtime_support`] when combined with
    /// [`Builder::synchronized`], or to keep golden-test output focused on the projected types.
    pub fn fragment(mut self) -> Self {
        self.fragment = true;
        self
    }

    /// Enables call/dispose synchronization, agility probing, apartment checks, context-aware
    /// release, and finalizer recovery. The default raw `IDisposable` owner avoids this overhead
    /// and requires deterministic same-apartment disposal without concurrent calls.
    pub fn synchronized(mut self) -> Self {
        self.synchronized = true;
        self
    }

    /// Reads the inputs, builds the projection model, and writes the C# source.
    pub fn write(self) -> std::io::Result<()> {
        if self.output.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "windows-csharp: output path is required",
            ));
        }
        let files = read_files(&self.input)?;
        if files.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "windows-csharp: at least one metadata input is required",
            ));
        }
        if !self.selected.is_empty()
            || !self.selected_functions.is_empty()
            || !self.selected_constants.is_empty()
        {
            let all_index = Index::new(read_files(&self.input)?);
            let index =
                Index::new_for_architecture(files, self.architecture.map_or(0, Architecture::bit));
            let source = self
                .write_selected(&index, &all_index)
                .map_err(invalid_input)?;
            return std::fs::write(&self.output, source);
        }
        let index =
            Index::new_for_architecture(files, self.architecture.map_or(0, Architecture::bit));

        // First pass: runtime classes. Record each class's default interface so the second pass can
        // skip projecting it standalone (its members are inlined into the class).
        let mut classes = Vec::new();
        let mut default_interfaces: HashSet<(String, String)> = HashSet::new();
        for (namespace, _name, def) in index.iter() {
            if !self.includes_namespace(namespace) {
                continue;
            }
            if let Some((class, default)) = read_class(&index, def) {
                default_interfaces.insert(default);
                for consumed in consumed_interfaces(&index, def) {
                    default_interfaces.insert(consumed);
                }
                classes.push(class);
            }
        }
        classes
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));

        // Second pass: every in-scope WinRT interface that is not a projected class's default
        // interface becomes a standalone struct, reachable through the generic `As<T>()` cast.
        let mut interfaces = Vec::new();
        for (namespace, name, def) in index.iter() {
            if !self.includes_namespace(namespace) {
                continue;
            }
            if default_interfaces.contains(&(namespace.to_string(), name.to_string())) {
                continue;
            }
            if let Some(interface) = read_interface(&index, def) {
                interfaces.push(interface);
            }
        }
        interfaces
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));

        // Third pass: every in-scope WinRT enum becomes a C# enum over its underlying scalar.
        let mut enums = Vec::new();
        for (namespace, _name, def) in index.iter() {
            if !self.includes_namespace(namespace) {
                continue;
            }
            if let Some(item) = read_enum(def) {
                enums.push(item);
            }
        }
        enums.sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));

        // Fourth pass: every in-scope Win32 opaque handle (see `native_handle_value`) becomes a
        // distinct blittable wrapper struct instead of collapsing to a bare `nint`.
        let mut handles = Vec::new();
        for (namespace, _name, def) in index.iter() {
            if !self.includes_namespace(namespace) {
                continue;
            }
            if let Some(item) = read_handle(def) {
                handles.push(item);
            }
        }
        handles
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));

        // Fifth pass: every in-scope WinRT struct becomes a blittable C# struct.
        let mut structs = Vec::new();
        for (namespace, _name, def) in index.iter() {
            if !self.includes_namespace(namespace) {
                continue;
            }
            if let Some(item) = read_struct(&index, def) {
                structs.push(item);
            }
        }
        structs
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));

        // Sixth pass: every in-scope WinRT delegate becomes a struct that can invoke a native
        // delegate pointer and, through `Create`, allocate a native callback object for events.
        let mut delegates = Vec::new();
        for (namespace, _name, def) in index.iter() {
            if !self.includes_namespace(namespace) {
                continue;
            }
            if let Some(item) = read_delegate(&index, def) {
                delegates.push(item);
            }
        }
        delegates
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));

        // Seventh pass: Win32 exports and constants become members of a namespace-local static
        // `Apis` class. The initial ABI slice intentionally accepts only direct blittable
        // parameters and returns; later slices add pointers, strings, callbacks, and HRESULT
        // shaping without changing the public container.
        let mut functions = Vec::new();
        let mut constants = Vec::new();
        for (namespace, _name, item) in index.iter_items() {
            if !self.includes_namespace(namespace) {
                continue;
            }
            match item {
                Item::Fn(method) => {
                    if let Some(function) = read_function(&index, namespace, method) {
                        functions.push(function);
                    }
                }
                Item::Const(field) => {
                    if let Some(constant) = read_constant(&index, namespace, field) {
                        constants.push(constant);
                    }
                }
                Item::Type(_) => {}
            }
        }
        functions
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));
        constants
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));

        // Eighth pass: collect every `IVector<...>`, `IMap<...>`, and their read-only views named by
        // an in-scope member so each generic definition can be emitted once with a `typeof`-to-IID
        // switch covering the concrete instantiations that appear. The views reuse the arity-1 and
        // arity-2 collectors, keyed by their own metadata name and open-generic PIID.
        let collections = Collections {
            inspectable: self.uses_object(&index),
            async_operation: self.collect_async(&index),
            vector: self.collect_arity1(&index, "IVector`1", "IVector"),
            map: self.collect_arity2(&index, "IMap`2", "IMap"),
            vector_view: self.collect_arity1(&index, "IVectorView`1", "IVectorView"),
            map_view: self.collect_arity2(&index, "IMapView`2", "IMapView"),
        };

        let source = writer::write(
            &classes,
            &interfaces,
            &enums,
            &handles,
            &structs,
            &delegates,
            &functions,
            &constants,
            &collections,
            !self.fragment,
            !self.synchronized,
        );
        std::fs::write(&self.output, source)
    }

    /// Implements exact-selection generation: resolves every [`Builder::select`]/[`Builder::member`]
    /// root to its `TypeDef`, projects it (narrowed by member name if requested), then walks the
    /// transitive dependency closure of every kept member's signature - adding referenced classes
    /// and interfaces as markers, and referenced delegates/enums/structs in full - before handing
    /// the assembled model to [`writer::write`]. Every unresolved or unprojectable root, member, or
    /// dependency is a hard error rather than a silently incomplete projection.
    fn write_selected(&self, index: &Index, all_index: &Index) -> Result<String, String> {
        let mut class_roots: HashMap<TypeDef, Option<HashSet<String>>> = HashMap::new();
        let mut interface_roots: HashMap<TypeDef, Option<HashSet<String>>> = HashMap::new();
        let mut delegate_roots = HashSet::new();
        let mut enum_roots = HashSet::new();
        let mut struct_roots = HashSet::new();
        let mut handle_roots = HashSet::new();

        let mut selected_types = self.selected.iter().collect::<Vec<_>>();
        selected_types.sort_by_key(|(name, _)| name.as_str());
        for (type_name, selection) in selected_types {
            let (_, _, item) = resolve_selected_item(
                index,
                all_index,
                type_name,
                SelectedItemKind::Type,
                self.architecture,
            )?;
            let Item::Type(def) = item else {
                unreachable!()
            };
            let members = match selection {
                Selection::All => None,
                Selection::Members(names) => Some(names.clone()),
            };
            match def.category() {
                TypeCategory::Class => {
                    if !def.flags().contains(TypeAttributes::WindowsRuntime) {
                        return Err(format!(
                            "windows-csharp: selected type `{type_name}` is not a WinRT class"
                        ));
                    }
                    class_roots.insert(def, members);
                }
                TypeCategory::Interface => {
                    interface_roots.insert(def, members);
                }
                TypeCategory::Delegate => {
                    if members.is_some() {
                        return Err(wrong_member_root(type_name, def));
                    }
                    if !def.flags().contains(TypeAttributes::WindowsRuntime) {
                        return Err(format!(
                            "windows-csharp: selected type `{type_name}` is a native callback \
                             typedef, which has no standalone projection"
                        ));
                    }
                    diagnose_delegate(index, Some(all_index), self.architecture, def, "selected")?;
                    delegate_roots.insert(def);
                }
                TypeCategory::Enum => {
                    if members.is_some() {
                        return Err(wrong_member_root(type_name, def));
                    }
                    diagnose_enum(def, "selected")?;
                    enum_roots.insert(def);
                }
                TypeCategory::Struct => {
                    if members.is_some() {
                        return Err(wrong_member_root(type_name, def));
                    }
                    if native_handle_value(def).is_some() {
                        handle_roots.insert(def);
                    } else {
                        diagnose_struct(
                            index,
                            Some(all_index),
                            self.architecture,
                            def,
                            "selected",
                        )?;
                        struct_roots.insert(def);
                    }
                }
                TypeCategory::Attribute => {
                    return Err(format!(
                        "windows-csharp: selected type `{type_name}` has metadata kind `attribute`, \
                         which has no projection"
                    ));
                }
            }
        }

        let mut classes = Vec::new();
        let mut interfaces = Vec::new();
        let mut functions = Vec::new();
        let mut constants = Vec::new();
        let mut signature_types: Vec<Type> = Vec::new();
        let mut work: VecDeque<Type> = VecDeque::new();

        let mut selected_functions = self.selected_functions.iter().collect::<Vec<_>>();
        selected_functions.sort();
        for function_name in selected_functions {
            let (namespace, _, item) = resolve_selected_item(
                index,
                all_index,
                function_name,
                SelectedItemKind::Function,
                self.architecture,
            )?;
            let Item::Fn(method) = item else {
                unreachable!()
            };
            let signature = method.signature(&[]);
            signature_types.push(signature.return_type.clone());
            work.push_back(signature.return_type);
            for ty in signature.types {
                signature_types.push(ty.clone());
                work.push_back(ty);
            }
            functions.push(read_function(index, namespace, method).ok_or_else(|| {
                diagnose_function(
                    index,
                    Some(all_index),
                    self.architecture,
                    function_name,
                    method,
                )
            })?);
        }

        let mut selected_constants = self.selected_constants.iter().collect::<Vec<_>>();
        selected_constants.sort();
        for constant_name in selected_constants {
            let (namespace, _, item) = resolve_selected_item(
                index,
                all_index,
                constant_name,
                SelectedItemKind::Constant,
                self.architecture,
            )?;
            let Item::Const(field) = item else {
                unreachable!()
            };
            let ty = field.ty();
            signature_types.push(ty.clone());
            work.push_back(ty);
            constants.push(read_constant(index, namespace, field).ok_or_else(|| {
                diagnose_constant(
                    index,
                    Some(all_index),
                    self.architecture,
                    constant_name,
                    field,
                )
            })?);
        }

        let mut sorted_classes = class_roots.iter().collect::<Vec<_>>();
        sorted_classes.sort_by_key(|(def, _)| qualified_name(**def));
        for (&def, selection) in sorted_classes {
            let (class, dep_types) =
                select_class(index, all_index, self.architecture, def, selection)?;
            for ty in &dep_types {
                work.push_back(ty.clone());
            }
            signature_types.extend(dep_types);
            classes.push(class);
        }
        let mut sorted_interfaces = interface_roots.iter().collect::<Vec<_>>();
        sorted_interfaces.sort_by_key(|(def, _)| qualified_name(**def));
        for (&def, selection) in sorted_interfaces {
            let (interface, dep_types) =
                select_interface(index, all_index, self.architecture, def, selection)?;
            for ty in &dep_types {
                work.push_back(ty.clone());
            }
            signature_types.extend(dep_types);
            interfaces.push(interface);
        }

        let mut sorted_delegate_roots = delegate_roots.iter().copied().collect::<Vec<_>>();
        sorted_delegate_roots.sort_by_key(|def| qualified_name(*def));
        for def in sorted_delegate_roots {
            let invoke = def
                .methods()
                .find(|method| method.name() == "Invoke")
                .unwrap();
            let signature = invoke.signature(&[]);
            signature_types.push(signature.return_type.clone());
            work.push_back(signature.return_type);
            for ty in signature.types {
                signature_types.push(ty.clone());
                work.push_back(ty);
            }
        }
        let mut sorted_struct_roots = struct_roots.iter().copied().collect::<Vec<_>>();
        sorted_struct_roots.sort_by_key(|def| qualified_name(*def));
        for def in sorted_struct_roots {
            enqueue_struct_field_types(index, def, &mut work);
        }

        let mut closure = Closure {
            index,
            all_index,
            architecture: self.architecture,
            class_roots: &class_roots,
            interface_roots: &interface_roots,
            class_markers: HashSet::new(),
            interface_markers: HashSet::new(),
            delegates: delegate_roots,
            native_callbacks: HashSet::new(),
            enums: enum_roots,
            structs: struct_roots,
            handles: handle_roots,
        };
        while let Some(ty) = work.pop_front() {
            closure.discover(&ty, &mut work, &mut signature_types)?;
        }

        for def in closure.class_markers {
            classes.push(marker_class(index, def)?);
        }
        for def in closure.interface_markers {
            interfaces.push(marker_interface(index, def)?);
        }
        let mut delegates = Vec::new();
        for def in closure.delegates {
            delegates.push(read_delegate(index, def).ok_or_else(|| {
                format!(
                    "windows-csharp: dependency delegate `{}.{}` could not be projected",
                    def.namespace(),
                    def.name()
                )
            })?);
        }
        let mut enums = Vec::new();
        for def in closure.enums {
            enums.push(read_enum(def).ok_or_else(|| {
                format!(
                    "windows-csharp: dependency enum `{}.{}` could not be projected",
                    def.namespace(),
                    def.name()
                )
            })?);
        }
        let mut structs = Vec::new();
        for def in closure.structs {
            structs.push(read_struct(index, def).ok_or_else(|| {
                format!(
                    "windows-csharp: dependency struct `{}.{}` could not be projected",
                    def.namespace(),
                    def.name()
                )
            })?);
        }
        let mut handles = Vec::new();
        for def in closure.handles {
            handles.push(read_handle(def).ok_or_else(|| {
                format!(
                    "windows-csharp: dependency handle `{}.{}` could not be projected",
                    def.namespace(),
                    def.name()
                )
            })?);
        }

        classes
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));
        interfaces
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));
        delegates
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));
        enums.sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));
        structs
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));
        handles
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));
        functions
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));
        constants
            .sort_by(|a, b| (a.namespace.as_str(), a.name.as_str()).cmp(&(&b.namespace, &b.name)));

        let collections = Collections {
            inspectable: object_used(&signature_types),
            async_operation: async_from_types(index, &signature_types)?,
            vector: vector_from_types(index, &signature_types, "IVector`1", "IVector")?,
            map: map_from_types(index, &signature_types, "IMap`2", "IMap")?,
            vector_view: vector_from_types(
                index,
                &signature_types,
                "IVectorView`1",
                "IVectorView",
            )?,
            map_view: map_from_types(index, &signature_types, "IMapView`2", "IMapView")?,
        };

        Ok(writer::write(
            &classes,
            &interfaces,
            &enums,
            &handles,
            &structs,
            &delegates,
            &functions,
            &constants,
            &collections,
            !self.fragment,
            !self.synchronized,
        ))
    }

    fn uses_object(&self, index: &Index) -> bool {
        for (namespace, _name, def) in index.iter() {
            if !self.includes_namespace(namespace)
                || !def.flags().contains(TypeAttributes::WindowsRuntime)
            {
                continue;
            }

            for method in def.methods() {
                let signature = method.signature(&[]);
                if type_uses_object(&signature.return_type)
                    || signature.types.iter().any(type_uses_object)
                {
                    return true;
                }
            }
        }
        false
    }

    fn collect_async(&self, index: &Index) -> Option<AsyncOperation> {
        let piid = index
            .get("Windows.Foundation", "IAsyncOperation")
            .next()
            .and_then(guid_attribute)?;
        let completed_piid = index
            .get("Windows.Foundation", "AsyncOperationCompletedHandler")
            .next()
            .and_then(guid_attribute)?;
        let mut seen = HashSet::new();
        let mut instantiations = Vec::new();

        for (namespace, _name, def) in index.iter() {
            if !self.includes_namespace(namespace)
                || !def.flags().contains(TypeAttributes::WindowsRuntime)
            {
                continue;
            }
            let iface = match def.category() {
                TypeCategory::Interface => def,
                TypeCategory::Class => match default_interface(index, def) {
                    Some(iface) => iface,
                    None => continue,
                },
                _ => continue,
            };
            for method in iface.methods() {
                let signature = method.signature(&[]);
                for ty in std::iter::once(&signature.return_type).chain(signature.types.iter()) {
                    let Type::ClassName(tn) = ty else { continue };
                    if tn.namespace != "Windows.Foundation"
                        || tn.name != "IAsyncOperation`1"
                        || tn.generics.len() != 1
                    {
                        continue;
                    }
                    let Some(value) = CsType::map(index, &tn.generics[0]) else {
                        continue;
                    };
                    if (!value.is_unmanaged()
                        && !value.is_object()
                        && !matches!(value, CsType::String))
                        || !seen.insert(value.surface())
                    {
                        continue;
                    }
                    let Some(iid) = guid::generic_iid(index, piid, &tn.generics) else {
                        continue;
                    };
                    let Some(completed_iid) =
                        guid::generic_iid(index, completed_piid, &tn.generics)
                    else {
                        continue;
                    };
                    instantiations.push(AsyncOperationInstantiation {
                        element: value,
                        iid,
                        completed_iid,
                    });
                }
            }
        }

        if instantiations.is_empty() {
            None
        } else {
            instantiations.sort_by_key(|value| value.element.surface());
            Some(AsyncOperation { instantiations })
        }
    }

    fn includes_namespace(&self, namespace: &str) -> bool {
        self.filter.iter().any(|filter| {
            namespace == filter
                || (namespace.len() > filter.len()
                    && namespace.starts_with(filter.as_str())
                    && namespace.as_bytes()[filter.len()] == b'.')
        })
    }

    /// Scans every in-scope interface (standalone or a class's default interface) for method
    /// signatures naming a supported arity-one generic instantiation (`IVector<...>` or its view
    /// `IVectorView<...>`), and returns the generic model carrying each element type's C# surface
    /// spelling and its generation-time IID, or `None` when no in-scope member uses one. `meta_name`
    /// is the arity-suffixed metadata name (`IVector`1`); `open_name` is the arity-trimmed key the
    /// Index stores (`IVector`), whose `GuidAttribute` seeds the parameterized-IID computation.
    fn collect_arity1(&self, index: &Index, meta_name: &str, open_name: &str) -> Option<Vector> {
        // The Index stores type names with the generic-arity suffix trimmed, so `IVector`1` is keyed
        // as `IVector`.
        let piid = index
            .get("Windows.Foundation.Collections", open_name)
            .next()
            .and_then(guid_attribute)?;

        // Dedup by element surface spelling, preserving first-seen order for deterministic output.
        let mut seen: HashSet<String> = HashSet::new();
        let mut instantiations: Vec<VectorInstantiation> = Vec::new();

        for (namespace, _name, def) in index.iter() {
            if !self.includes_namespace(namespace) {
                continue;
            }
            if !def.flags().contains(TypeAttributes::WindowsRuntime) {
                continue;
            }
            let iface = match def.category() {
                TypeCategory::Interface => def,
                TypeCategory::Class => match default_interface(index, def) {
                    Some(iface) => iface,
                    None => continue,
                },
                _ => continue,
            };
            for method in iface.methods() {
                let signature = method.signature(&[]);
                for ty in std::iter::once(&signature.return_type).chain(signature.types.iter()) {
                    let Type::ClassName(tn) = ty else { continue };
                    // `generic_name` gates that the element is unmanaged and accepts every generic
                    // shape; the name check selects only this collector's shape.
                    if tn.name != meta_name || generic_name(index, tn).is_none() {
                        continue;
                    }
                    let element = CsType::map(index, &tn.generics[0])?;
                    if !seen.insert(element.collection_surface()) {
                        continue;
                    }
                    let iid = guid::generic_iid(index, piid, &tn.generics)?;
                    instantiations.push(VectorInstantiation { element, iid });
                }
            }
        }

        if instantiations.is_empty() {
            return None;
        }
        instantiations.sort_by_key(|value| value.element.collection_surface());
        Some(Vector { instantiations })
    }

    /// The arity-two analogue of [`Self::collect_arity1`]: scans every in-scope interface for a
    /// signature naming a supported `IMap<...>` or its view `IMapView<...>` and returns the generic
    /// model carrying each key/value pair's C# surface spellings and generation-time IID, or `None`
    /// when no in-scope member uses one. `meta_name`/`open_name` select the metadata shape and the
    /// open-generic PIID.
    fn collect_arity2(&self, index: &Index, meta_name: &str, open_name: &str) -> Option<Map> {
        // The Index keys generic types by their arity-trimmed name, so `IMap`2` is keyed as `IMap`.
        let piid = index
            .get("Windows.Foundation.Collections", open_name)
            .next()
            .and_then(guid_attribute)?;

        // Dedup by (key, value) surface spelling, preserving first-seen order for determinism.
        let mut seen: HashSet<(String, String)> = HashSet::new();
        let mut instantiations: Vec<MapInstantiation> = Vec::new();
        let iterable_piid = index
            .get("Windows.Foundation.Collections", "IIterable")
            .next()
            .and_then(guid_attribute)?;
        let iterator_piid = index
            .get("Windows.Foundation.Collections", "IIterator")
            .next()
            .and_then(guid_attribute)?;

        for (namespace, _name, def) in index.iter() {
            if !self.includes_namespace(namespace) {
                continue;
            }
            if !def.flags().contains(TypeAttributes::WindowsRuntime) {
                continue;
            }
            let iface = match def.category() {
                TypeCategory::Interface => def,
                TypeCategory::Class => match default_interface(index, def) {
                    Some(iface) => iface,
                    None => continue,
                },
                _ => continue,
            };
            for method in iface.methods() {
                let signature = method.signature(&[]);
                for ty in std::iter::once(&signature.return_type).chain(signature.types.iter()) {
                    let Type::ClassName(tn) = ty else { continue };
                    if tn.name != meta_name || generic_name(index, tn).is_none() {
                        continue;
                    }
                    let key = CsType::map(index, &tn.generics[0])?;
                    let value = CsType::map(index, &tn.generics[1])?;
                    if !seen.insert((key.surface(), value.surface())) {
                        continue;
                    }
                    let iid = guid::generic_iid(index, piid, &tn.generics)?;
                    let pair = Type::ClassName(windows_metadata::TypeName {
                        namespace: "Windows.Foundation.Collections".to_string(),
                        name: "IKeyValuePair`2".to_string(),
                        generics: tn.generics.clone(),
                    });
                    let iterable_iid =
                        guid::generic_iid(index, iterable_piid, std::slice::from_ref(&pair))?;
                    let iterator_iid =
                        guid::generic_iid(index, iterator_piid, std::slice::from_ref(&pair))?;
                    instantiations.push(MapInstantiation {
                        key,
                        value,
                        iid,
                        iterable_iid,
                        iterator_iid,
                    });
                }
            }
        }

        if instantiations.is_empty() {
            return None;
        }
        instantiations.sort_by(|a, b| {
            (a.key.surface(), a.value.surface()).cmp(&(b.key.surface(), b.value.surface()))
        });
        Some(Map { instantiations })
    }
}

/// Returns the shared runtime support as a standalone C# compilation unit: the registration-free
/// activation helper and the combase/kernel32 P/Invokes that projection fragments (see
/// [`Builder::fragment`]) call into.
pub fn runtime_support() -> String {
    writer::support(true)
}

/// Returns the opt-in synchronized runtime support used by [`Builder::synchronized`] as a
/// standalone C# compilation unit.
pub fn synchronized_runtime_support() -> String {
    writer::support(false)
}

/// Reads every `.winmd` file referenced by the inputs (files or directories).
fn read_files(inputs: &[String]) -> std::io::Result<Vec<File>> {
    let mut files = Vec::new();
    for input in inputs {
        let path = std::path::Path::new(input);
        if path.is_dir() {
            let mut entries = std::fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;
            entries.sort_by_key(std::fs::DirEntry::path);
            for entry in entries {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "winmd") {
                    files.push(read_winmd(&path)?);
                }
            }
        } else {
            files.push(read_winmd(path)?);
        }
    }
    Ok(files)
}

/// Reads one metadata file, distinguishing a missing path from an invalid winmd instead of silently
/// omitting either from the projection.
fn read_winmd(path: &std::path::Path) -> std::io::Result<File> {
    if !path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "windows-csharp: metadata input not found: {}",
                path.display()
            ),
        ));
    }
    File::read(path).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "windows-csharp: failed to read metadata input: {}",
                path.display()
            ),
        )
    })
}

/// Resolves a class's default interface and reads its full (unfiltered) member list plus every
/// directly implemented and inherited interface's collision-free forwarder candidates, preserving
/// each member's real ABI vtable slot. Returns `None` when the type has no default interface. Shared
/// by [`read_class`] (namespace-filtered generation, which drops an empty result) and
/// [`select_class`] (exact selection, which narrows the result by member name instead of discarding
/// it).
fn class_parts<'a>(
    index: &'a Index,
    def: TypeDef<'a>,
) -> Option<(TypeDef<'a>, Vec<Member>, Vec<(TypeDef<'a>, Vec<Member>)>)> {
    fn add_interface<'a>(
        index: &'a Index,
        default: TypeDef<'a>,
        interface: TypeDef<'a>,
        names: &mut HashSet<String>,
        interfaces: &mut HashSet<(String, String)>,
        sources: &mut Vec<(TypeDef<'a>, Vec<Member>)>,
    ) {
        if interface.namespace() == default.namespace() && interface.name() == default.name() {
            return;
        }
        if !interfaces.insert((
            interface.namespace().to_string(),
            interface.name().to_string(),
        )) {
            return;
        }
        let mut forwarded = read_members(index, interface, 6, MethodAbi::WinRt);
        let mut claimed = HashSet::new();
        forwarded.retain(|member| {
            let name = member_name(member);
            !matches!(member, Member::Event { .. }) && !names.contains(name) && {
                claimed.insert(name.to_string());
                true
            }
        });
        names.extend(claimed);
        if !forwarded.is_empty() {
            sources.push((interface, forwarded));
        }
    }

    fn add_implemented<'a>(
        index: &'a Index,
        default: TypeDef<'a>,
        class: TypeDef<'a>,
        names: &mut HashSet<String>,
        interfaces: &mut HashSet<(String, String)>,
        sources: &mut Vec<(TypeDef<'a>, Vec<Member>)>,
    ) {
        for imp in class.interface_impls() {
            let Type::ClassName(name) = imp.interface(&type_generics(class)) else {
                continue;
            };
            let Some(interface) = index.get(&name.namespace, &name.name).next() else {
                continue;
            };
            add_interface(index, default, interface, names, interfaces, sources);
        }
    }

    let default = default_interface(index, def)?;
    let members = read_members(index, default, 6, MethodAbi::WinRt);
    let mut names: HashSet<String> = members
        .iter()
        .map(member_name)
        .map(str::to_string)
        .collect();
    let mut forwarder_sources = Vec::new();
    let mut interfaces = HashSet::new();

    add_implemented(
        index,
        default,
        def,
        &mut names,
        &mut interfaces,
        &mut forwarder_sources,
    );

    let mut class = def;
    while let Some(extends) = class.extends() {
        if extends.namespace() == "System" && extends.name() == "Object" {
            break;
        }
        let Some(base) = index.get(extends.namespace(), extends.name()).next() else {
            break;
        };
        if let Some(base_default) = default_interface(index, base) {
            add_interface(
                index,
                default,
                base_default,
                &mut names,
                &mut interfaces,
                &mut forwarder_sources,
            );
        }
        add_implemented(
            index,
            default,
            base,
            &mut names,
            &mut interfaces,
            &mut forwarder_sources,
        );
        class = base;
    }
    Some((default, members, forwarder_sources))
}

/// Whether the class declares a parameterless (`RoActivateInstance`-style) constructor: an
/// `ActivatableAttribute` that names no factory type. Mirrors windows-bindgen's
/// `Class::has_default_constructor`.
fn has_default_activation(def: TypeDef) -> bool {
    def.attributes()
        .filter(|attribute| attribute.name() == "ActivatableAttribute")
        .any(|attribute| {
            !attribute
                .value()
                .iter()
                .any(|(_, arg)| matches!(arg, Value::TypeName(_)))
        })
}

/// Resolves the factory/static/composition interface an activation attribute names, if any. An
/// `ActivatableAttribute` with no `System.Type` argument is default activation and yields `None`.
fn factory_interface<'a>(index: &'a Index, attribute: &Attribute<'_>) -> Option<TypeDef<'a>> {
    let name = factory_interface_name(attribute)?;
    index.get(&name.namespace, &name.name).next()
}

fn factory_interface_name(attribute: &Attribute<'_>) -> Option<windows_metadata::TypeName> {
    attribute.value().into_iter().find_map(|(_, arg)| {
        if let Value::TypeName(name) = arg {
            Some(name)
        } else {
            None
        }
    })
}

/// Reads one factory (`ActivatableAttribute`/`ComposableAttribute`) interface into a [`Factory`]:
/// every creation method becomes a public constructor calling that interface's vtable slot (from 6,
/// after the `IInspectable` base). A composable factory's trailing `outer`/`inner` ABI parameters
/// are dropped from the public constructor. Returns `None` when the interface has no `GuidAttribute`
/// or projects no usable creation method.
fn read_factory(index: &Index, iface: TypeDef, composable: bool) -> Option<Factory> {
    let iid = guid_attribute(iface)?;
    let mut constructors = Vec::new();
    for (slot, method) in (6usize..).zip(iface.methods()) {
        if method.flags().contains(MethodAttributes::SpecialName) {
            continue;
        }
        let signature = method.signature(&[]);
        let count = if composable {
            match signature.types.len().checked_sub(2) {
                Some(count) => count,
                None => continue,
            }
        } else {
            signature.types.len()
        };
        if let Some(params) = read_params(index, method, &signature, count, |_, ty| {
            CsType::map(index, ty)
        }) {
            constructors.push(Constructor { params, slot });
        }
    }
    if constructors.is_empty() {
        None
    } else {
        Some(Factory {
            iid,
            composable,
            constructors,
        })
    }
}

/// Reads one static (`StaticAttribute`) interface into a [`StaticInterface`], reusing
/// [`read_members`] so its members project exactly like instance members. Returns `None` when the
/// interface has no `GuidAttribute` or projects no members.
fn read_static(index: &Index, iface: TypeDef) -> Option<StaticInterface> {
    let iid = guid_attribute(iface)?;
    let members = read_members(index, iface, 6, MethodAbi::WinRt);
    if members.is_empty() {
        return None;
    }
    Some(StaticInterface { iid, members })
}

/// Reads every factory/composition interface a class names through `ActivatableAttribute` and
/// `ComposableAttribute`, in attribute order. A default-activation `ActivatableAttribute` (no
/// factory type) is skipped here; see [`has_default_activation`].
fn read_factories(index: &Index, def: TypeDef) -> Vec<Factory> {
    let mut factories = Vec::new();
    for attribute in def.attributes() {
        let composable = match attribute.name() {
            "ComposableAttribute" => true,
            "ActivatableAttribute" => false,
            _ => continue,
        };
        let Some(iface) = factory_interface(index, &attribute) else {
            continue;
        };
        if let Some(factory) = read_factory(index, iface, composable) {
            factories.push(factory);
        }
    }
    factories
}

/// Reads every static interface a class names through `StaticAttribute`, in attribute order.
fn read_statics(index: &Index, def: TypeDef) -> Vec<StaticInterface> {
    let mut statics = Vec::new();
    for attribute in def.attributes() {
        if attribute.name() != "StaticAttribute" {
            continue;
        }
        let Some(iface) = factory_interface(index, &attribute) else {
            continue;
        };
        if let Some(item) = read_static(index, iface) {
            statics.push(item);
        }
    }
    statics
}

/// The `(namespace, name)` identities of the factory, static, and composition interfaces a class
/// consumes through its activation attributes. These are inlined into the class (as constructors
/// and static members) and must not also be projected as standalone interfaces, matching
/// windows-bindgen (which projects them only as part of the class).
fn consumed_interfaces(index: &Index, def: TypeDef) -> Vec<(String, String)> {
    let mut consumed = Vec::new();
    for attribute in def.attributes() {
        match attribute.name() {
            "ActivatableAttribute" | "ComposableAttribute" | "StaticAttribute" => {}
            _ => continue,
        }
        if let Some(iface) = factory_interface(index, &attribute) {
            consumed.push((iface.namespace().to_string(), iface.name().to_string()));
        }
    }
    consumed
}

/// Reads a runtime-class `TypeDef` into a [`Class`] plus the `(namespace, name)` of its default
/// interface, or returns `None` when the type is not a projectable WinRT runtime class (for example
/// a static class with no default interface). The default interface's members are inlined into the
/// class; its identity is returned so the caller can skip projecting it a second time as a
/// standalone struct.
fn read_class(index: &Index, def: TypeDef) -> Option<(Class, (String, String))> {
    if !def.flags().contains(TypeAttributes::WindowsRuntime)
        || def.category() != TypeCategory::Class
    {
        return None;
    }

    let (default, members, forwarder_sources) = class_parts(index, def)?;
    if members.is_empty() {
        return None;
    }

    let mut forwarders = Vec::new();
    for (interface, forwarded) in forwarder_sources {
        forwarders.push(Forwarder {
            iid: guid_attribute(interface)?,
            members: forwarded,
        });
    }

    let class = Class {
        namespace: def.namespace().to_string(),
        name: def.name().to_string(),
        default_activation: has_default_activation(def),
        factories: read_factories(index, def),
        statics: read_statics(index, def),
        default_iid: guid_attribute(default)?,
        members,
        forwarders,
        compatible: class_compatible_types(index, def),
    };
    let identity = (default.namespace().to_string(), default.name().to_string());
    Some((class, identity))
}

fn member_name(member: &Member) -> &str {
    match member {
        Member::Property { name, .. }
        | Member::Method { name, .. }
        | Member::Event { name, .. } => name,
    }
}

fn metadata_member_name(name: &str) -> &str {
    for prefix in ["get_", "put_", "add_", "remove_"] {
        if let Some(name) = name.strip_prefix(prefix) {
            return name;
        }
    }
    name
}

fn native_interface_chain<'a>(index: &'a Index, def: TypeDef<'a>) -> Vec<TypeDef<'a>> {
    fn visit<'a>(
        index: &'a Index,
        def: TypeDef<'a>,
        seen: &mut HashSet<(String, String)>,
        chain: &mut Vec<TypeDef<'a>>,
    ) {
        let identity = (def.namespace().to_string(), def.name().to_string());
        if !seen.insert(identity) {
            return;
        }
        if let Some(extends) = def.extends() {
            if extends.name() != "IUnknown"
                && let Some(base) = index.get(extends.namespace(), extends.name()).next()
            {
                visit(index, base, seen, chain);
            }
        }
        for imp in def.interface_impls() {
            let Type::ClassName(name) = imp.interface(&type_generics(def)) else {
                continue;
            };
            if name.name == "IUnknown" {
                continue;
            }
            if let Some(base) = index.get(&name.namespace, &name.name).next() {
                visit(index, base, seen, chain);
            }
        }
        chain.push(def);
    }

    let mut chain = Vec::new();
    visit(index, def, &mut HashSet::new(), &mut chain);
    chain
}

fn read_native_interface_members(
    index: &Index,
    def: TypeDef,
) -> (Vec<Member>, Option<String>, Option<Vec<Member>>) {
    let mut members = Vec::new();
    let mut slot = 3;
    for current in native_interface_chain(index, def) {
        members.extend(read_members(index, current, slot, MethodAbi::Direct));
        slot += current.methods().count();
    }
    let mut bases = Vec::new();
    if let Some(extends) = def.extends()
        && extends.name() != "IUnknown"
    {
        bases.push(format!("{}.{}", extends.namespace(), extends.name()));
    }
    for imp in def.interface_impls() {
        let Type::ClassName(name) = imp.interface(&type_generics(def)) else {
            continue;
        };
        if name.name != "IUnknown" {
            bases.push(format!("{}.{}", name.namespace, name.name));
        }
    }
    bases.sort();
    bases.dedup();
    let base = (bases.len() == 1).then(|| bases.remove(0));
    let own = read_members(index, def, 3, MethodAbi::Direct);
    let own = (bases.is_empty() && own.len() == def.methods().count()).then_some(own);
    (members, base, own)
}

fn native_interface_methods<'a>(
    index: &'a Index,
    def: TypeDef<'a>,
) -> HashMap<usize, MethodDef<'a>> {
    let mut result = HashMap::new();
    let mut slot = 3;
    for current in native_interface_chain(index, def) {
        for method in current.methods() {
            result.insert(slot, method);
            slot += 1;
        }
    }
    result
}

/// Reads an interface `TypeDef` into a standalone [`Interface`], or returns `None` when the type is
/// not projectable or projects no members (an empty marker interface adds no cast surface).
fn read_interface(index: &Index, def: TypeDef) -> Option<Interface> {
    if def.category() != TypeCategory::Interface {
        return None;
    }

    let winrt = def.flags().contains(TypeAttributes::WindowsRuntime);
    let (members, native_base, native_own_members) = if winrt {
        (read_members(index, def, 6, MethodAbi::WinRt), None, None)
    } else {
        let (members, base, own) = read_native_interface_members(index, def);
        (members, base, own)
    };
    if members.is_empty() {
        return None;
    }

    Some(Interface {
        namespace: def.namespace().to_string(),
        name: def.name().to_string(),
        iid: guid_attribute(def)?,
        members,
        compatible: interface_compatible_types(index, def),
        native_base,
        native_own_members,
    })
}

/// Reads a WinRT enum `TypeDef` into an [`Enum`], or returns `None` when the type is not a
/// projectable WinRT enum. The enumerator fields carry literal constants; the lone storage field
/// (`value__`) has no constant and is skipped.
fn read_enum(def: TypeDef) -> Option<Enum> {
    if def.category() != TypeCategory::Enum {
        return None;
    }

    let underlying = enum_underlying(def)?;
    let mut fields = Vec::new();
    for field in def.fields() {
        if !field.flags().contains(FieldAttributes::Literal) {
            continue;
        }
        let Some(constant) = field.constant() else {
            continue;
        };
        let Some(value) = enum_value(&constant.value()) else {
            continue;
        };
        fields.push((field.name().to_string(), value));
    }

    Some(Enum {
        namespace: def.namespace().to_string(),
        name: def.name().to_string(),
        underlying,
        fields,
    })
}

/// Reads a WinRT or Win32 record `TypeDef` into a [`Struct`], or returns `None` when any field has
/// no supported ABI representation. Explicit-layout metadata is projected as a native union with
/// every field at offset zero.
fn read_struct(index: &Index, def: TypeDef) -> Option<Struct> {
    let qualified_name = format!("{}.{}", def.namespace(), def.name());
    read_struct_with_name(index, def, &qualified_name)
}

fn read_struct_with_name(index: &Index, def: TypeDef, qualified_name: &str) -> Option<Struct> {
    if def.category() != TypeCategory::Struct
        || def.has_attribute("NativeTypedefAttribute")
        || def.has_attribute("AlignmentAttribute")
    {
        return None;
    }

    let nested_defs: Vec<_> = index.nested(def).collect();
    let mut fields = Vec::new();
    for field in def.fields() {
        if field.flags().contains(FieldAttributes::Static) {
            continue;
        }
        let field_ty = field.ty();
        let ty = if let Type::ValueName(name) = &field_ty
            && name.namespace.is_empty()
            && let Some(nested) = nested_defs.iter().find(|nested| nested.name() == name.name)
        {
            let owns_abi = struct_owns_abi(index, *nested);
            CsType::Struct {
                name: format!("{qualified_name}.{}", nested.name()),
                abi_name: (owns_abi || struct_needs_abi(index, *nested))
                    .then(|| format!("{qualified_name}.{}Abi", nested.name())),
                owns_abi,
            }
        } else {
            CsType::map(index, &field_ty)?
        };
        if !ty.is_struct_field_abi() {
            return None;
        }
        fields.push((field.name().to_string(), ty));
    }
    if fields.is_empty() {
        return None;
    }

    let owns_abi = struct_owns_abi(index, def);
    let needs_abi = owns_abi || struct_needs_abi(index, def);
    if needs_abi && def.flags().contains(TypeAttributes::ExplicitLayout) {
        return None;
    }
    let layout = def.class_layout();
    Some(Struct {
        namespace: def.namespace().to_string(),
        name: def.name().to_string(),
        explicit: def.flags().contains(TypeAttributes::ExplicitLayout),
        abi_name: needs_abi.then(|| format!("{}Abi", def.name())),
        owns_abi,
        packing_size: layout
            .filter(|layout| layout.packing_size() != 0)
            .map(|layout| layout.packing_size()),
        class_size: layout
            .filter(|layout| layout.class_size() != 0)
            .map(|layout| layout.class_size()),
        fields,
        nested: nested_defs
            .into_iter()
            .map(|nested| {
                let name = format!("{qualified_name}.{}", nested.name());
                read_struct_with_name(index, nested, &name)
            })
            .collect::<Option<Vec<_>>>()?,
    })
}

/// Reads a genuine Win32 opaque-handle `TypeDef` into a [`Handle`], or `None` when `def` does not
/// meet the structural handle rule in [`native_handle_value`] (a scalar identifier alias, a
/// pointer-to-named-type alias, a typedef chained to another handle, or an ordinary WinRT/Win32
/// struct all return `None` here).
fn read_handle(def: TypeDef) -> Option<Handle> {
    native_handle_value(def)?;
    Some(Handle {
        namespace: def.namespace().to_string(),
        name: def.name().to_string(),
    })
}

/// Reads a Win32 export whose ABI is already idiomatic C#: direct blittable parameters and an
/// optional direct blittable return. This deliberately excludes WinRT `String`/`Boolean`, arrays,
/// objects, and every pointer-like shape until their Win32-specific ownership/nullability model is
/// represented explicitly.
fn read_function(index: &Index, namespace: &str, method: MethodDef) -> Option<Function> {
    let map = method.impl_map()?;
    let signature = method.signature(&[]);
    if signature.flags.contains(MethodCallAttributes::VARARG) {
        return None;
    }
    let ret = match &signature.return_type {
        Type::Void => None,
        ty => {
            let ty = CsType::map(index, ty)?;
            if !ty.is_native_abi() {
                return None;
            }
            Some(ty)
        }
    };
    let hresult = matches!(ret, Some(CsType::HResult));
    let com_out = hresult
        .then(|| function_com_out(index, method, &signature))
        .flatten();

    let params = read_params(
        index,
        method,
        &signature,
        signature.types.len(),
        |position, ty| {
            let ty = if com_out
                .as_ref()
                .is_some_and(|(candidate, _)| *candidate == position)
            {
                com_out.as_ref().unwrap().1.clone()
            } else {
                CsType::map(index, ty)?
            };
            (ty.is_native_abi() || matches!(ty, CsType::ComOut { .. })).then_some(ty)
        },
    )?;

    Some(Function {
        namespace: namespace.to_string(),
        name: method.name().to_string(),
        library: map.import_scope().name().to_string(),
        import_name: map.import_name().to_string(),
        cdecl: method.calling_convention() == "C",
        params,
        ret,
        hresult,
    })
}

/// Selects one interface double-pointer that can safely become an owning projected return.
///
/// An explicit trailing `RetValAttribute` resolves the selection. Without it, this follows the same
/// conservative shape as windows-bindgen's retval heuristic: the candidate must be the last
/// parameter and every preceding parameter must be input-only. Invalid or ambiguous candidates
/// remain literal raw pointer parameters.
fn function_com_out(
    index: &Index,
    method: MethodDef,
    signature: &windows_metadata::Signature,
) -> Option<(usize, CsType)> {
    let params = method.params_by_sequence(signature.types.len()).ok()?;
    let mut candidates = Vec::new();

    for (position, ty) in signature.types.iter().enumerate() {
        let Some(def) = params.params()[position].as_ref() else {
            continue;
        };
        let Some(mapped) = com_out_type(index, ty) else {
            continue;
        };
        if def.direction() != ParamDirection::Output
            || def.is_optional()
            || def.is_reserved()
            || def.attributes().any(|attribute| {
                matches!(
                    attribute.name(),
                    "NativeArrayInfoAttribute" | "MemorySizeAttribute"
                )
            })
        {
            continue;
        }
        candidates.push((position, mapped, def.is_retval_attribute()));
    }

    let mut explicit = candidates.iter().filter(|candidate| candidate.2);
    if let Some(candidate) = explicit.next() {
        if candidate.0 + 1 == signature.types.len() && explicit.next().is_none() {
            return Some((candidate.0, candidate.1.clone()));
        }
        return None;
    }

    let [candidate] = candidates.as_slice() else {
        return None;
    };
    if candidate.0 + 1 != signature.types.len()
        || params.params()[..candidate.0]
            .iter()
            .flatten()
            .any(|param| {
                matches!(
                    param.direction(),
                    ParamDirection::Output | ParamDirection::InputOutput
                )
            })
    {
        return None;
    }

    Some((candidate.0, candidate.1.clone()))
}

fn com_out_type(index: &Index, ty: &Type) -> Option<CsType> {
    // An interface ClassName already represents one ABI pointer; the outer pointer is the out slot.
    let Type::PtrMut(inner, 1) = ty else {
        return None;
    };
    let Type::ClassName(name) = inner.as_ref() else {
        return None;
    };
    let def = index.get(&name.namespace, &name.name).next()?;
    if def.category() != TypeCategory::Interface {
        return None;
    }
    Some(CsType::ComOut {
        name: format!("{}.{}", name.namespace, name.name),
    })
}

/// Reads an integer Win32 metadata constant. Other literal families remain unsupported until their
/// C# spelling and metadata semantics are covered by fixtures.
fn read_constant(index: &Index, namespace: &str, field: Field) -> Option<ApiConstant> {
    let constant = field.constant()?;
    let ty = CsType::map(index, &constant.ty())?;
    if !ty.is_native_abi() {
        return None;
    }
    Some(ApiConstant {
        namespace: namespace.to_string(),
        name: field.name().to_string(),
        ty,
        value: constant_literal(&constant.value())?,
    })
}

fn constant_literal(value: &Value) -> Option<String> {
    Some(match value {
        Value::I8(value) => format!("(sbyte){value}"),
        Value::U8(value) => format!("(byte){value}"),
        Value::I16(value) => format!("(short){value}"),
        Value::U16(value) => format!("(ushort){value}"),
        Value::I32(value) => value.to_string(),
        Value::U32(value) => format!("{value}u"),
        Value::I64(value) => format!("{value}L"),
        Value::U64(value) => format!("{value}UL"),
        Value::ISize(value) => format!("(nint){value}"),
        Value::USize(value) => format!("(nuint){value}"),
        Value::EnumValue(_, value) => constant_literal(value)?,
        _ => return None,
    })
}

/// Reads a WinRT delegate `TypeDef` into a [`Delegate`], or returns `None` when the type is not a
/// projectable WinRT delegate or its `Invoke` signature names an unsupported type.
fn read_delegate(index: &Index, def: TypeDef) -> Option<Delegate> {
    if !def.flags().contains(TypeAttributes::WindowsRuntime)
        || def.category() != TypeCategory::Delegate
    {
        return None;
    }

    let (types, ret) = delegate_invoke(index, def)?;
    let invoke = def.methods().find(|method| method.name() == "Invoke")?;
    let defs = invoke.params_by_sequence(types.len()).ok()?;

    let mut params = Vec::with_capacity(types.len());
    for (i, ty) in types.into_iter().enumerate() {
        let (name, direction, optional) =
            param_meta(defs.params().get(i).and_then(Option::as_ref), i);
        params.push(Param {
            name,
            ty,
            direction,
            optional,
            projection: ParamProjection::Value,
        });
    }

    Some(Delegate {
        namespace: def.namespace().to_string(),
        name: def.name().to_string(),
        iid: guid_attribute(def)?,
        params,
        ret,
    })
}

/// Renders an enum constant as its C# integer literal, or `None` for a non-integer constant.
fn enum_value(value: &Value) -> Option<String> {
    Some(match value {
        Value::I8(n) => n.to_string(),
        Value::U8(n) => n.to_string(),
        Value::I16(n) => n.to_string(),
        Value::U16(n) => n.to_string(),
        Value::I32(n) => n.to_string(),
        Value::U32(n) => n.to_string(),
        Value::I64(n) => n.to_string(),
        Value::U64(n) => n.to_string(),
        _ => return None,
    })
}

/// Resolves a runtime class's default interface: the `InterfaceImpl` marked `DefaultAttribute`, or
/// the first interface when none is marked.
pub(crate) fn default_interface<'a>(index: &'a Index, def: TypeDef<'a>) -> Option<TypeDef<'a>> {
    let mut resolved = Vec::new();
    let mut default_index = 0;
    for imp in def.interface_impls() {
        let Type::ClassName(name) = imp.interface(&type_generics(def)) else {
            continue;
        };
        let Some(typedef) = index.get(&name.namespace, &name.name).next() else {
            continue;
        };
        if imp.has_attribute("DefaultAttribute") {
            default_index = resolved.len();
        }
        resolved.push(typedef);
    }

    if resolved.is_empty() {
        let extends = def.extends()?;
        if extends.namespace() == "System" && extends.name() == "Object" {
            return None;
        }
        let base = index.get(extends.namespace(), extends.name()).next()?;
        return default_interface(index, base);
    }

    Some(resolved.remove(default_index))
}

/// Returns a runtime class's default-interface type without discarding closed generic arguments.
fn default_interface_type(index: &Index, def: TypeDef) -> Option<Type> {
    let mut resolved = Vec::new();
    let mut default_index = 0;
    for imp in def.interface_impls() {
        let interface = imp.interface(&type_generics(def));
        if !matches!(interface, Type::ClassName(_)) {
            continue;
        }
        if imp.has_attribute("DefaultAttribute") {
            default_index = resolved.len();
        }
        resolved.push(interface);
    }

    if resolved.is_empty() {
        let extends = def.extends()?;
        if extends.namespace() == "System" && extends.name() == "Object" {
            return None;
        }
        let base = index.get(extends.namespace(), extends.name()).next()?;
        return default_interface_type(index, base);
    }

    Some(resolved.remove(default_index))
}

/// Computes the IID of a runtime class's default interface, including a parameterized IID when the
/// class defaults directly to a closed generic interface such as `IVector<RowDefinition>`.
pub(crate) fn default_interface_iid(index: &Index, def: TypeDef) -> Option<Guid> {
    let Type::ClassName(name) = default_interface_type(index, def)? else {
        return None;
    };
    let interface = index
        .get(&name.namespace, windows_metadata::trim_tick(&name.name))
        .next()?;
    let iid = guid_attribute(interface)?;
    if name.generics.is_empty() {
        Some(iid)
    } else {
        guid::generic_iid(index, iid, &name.generics)
    }
}

/// Projects an interface's methods into ordered members, assigning vtable slots from 6 (the first
/// slot after the `IInspectable` base) and folding `get_`/`put_` pairs into properties.
fn read_members(
    index: &Index,
    iface: TypeDef,
    slot_base: usize,
    method_abi: MethodAbi,
) -> Vec<Member> {
    let mut members: Vec<Member> = Vec::new();
    let mut properties: HashMap<String, usize> = HashMap::new();
    let mut events: HashMap<String, usize> = HashMap::new();

    for (this_slot, method) in (slot_base..).zip(iface.methods()) {
        let name = method.name();
        let signature = method.signature(&[]);
        if method.params_by_sequence(signature.types.len()).is_err() {
            continue;
        }
        let special = method.flags().contains(MethodAttributes::SpecialName);

        if matches!(method_abi, MethodAbi::Direct) {
            if let Some(member) = read_direct_method(index, &method, &signature, name, this_slot) {
                members.push(member);
            }
            continue;
        }

        if special && (name.starts_with("get_") || name.starts_with("put_")) {
            let is_get = name.starts_with("get_");
            let property = &name[4..];
            let meta = if is_get {
                &signature.return_type
            } else {
                match signature.types.first() {
                    Some(ty) => ty,
                    None => continue,
                }
            };
            let Some(ty) = CsType::map(index, meta) else {
                continue;
            };

            if let Some(&idx) = properties.get(property) {
                if let Member::Property {
                    get_slot, put_slot, ..
                } = &mut members[idx]
                {
                    if is_get {
                        *get_slot = Some(this_slot);
                    } else {
                        *put_slot = Some(this_slot);
                    }
                }
            } else {
                properties.insert(property.to_string(), members.len());
                members.push(Member::Property {
                    name: property.to_string(),
                    ty,
                    get_slot: is_get.then_some(this_slot),
                    put_slot: (!is_get).then_some(this_slot),
                });
            }
        } else if special && name.starts_with("add_") {
            // The `add_` accessor takes the delegate handler and returns the registration token.
            let event = &name[4..];
            let Some(handler) = signature.types.first() else {
                continue;
            };
            let Some(delegate) = CsType::map(index, handler) else {
                continue;
            };
            if let Some(&idx) = events.get(event) {
                if let Member::Event { add_slot, .. } = &mut members[idx] {
                    *add_slot = this_slot;
                }
            } else {
                events.insert(event.to_string(), members.len());
                members.push(Member::Event {
                    name: event.to_string(),
                    delegate,
                    add_slot: this_slot,
                    remove_slot: 0,
                });
            }
        } else if special && name.starts_with("remove_") {
            // The `remove_` accessor takes the registration token and returns void.
            let event = &name[7..];
            if let Some(&idx) = events.get(event) {
                if let Member::Event { remove_slot, .. } = &mut members[idx] {
                    *remove_slot = this_slot;
                }
            } else {
                // A `remove_` seen before its `add_` cannot resolve the delegate type; record the
                // slot with a placeholder that `add_` fills in.
                events.insert(event.to_string(), members.len());
                members.push(Member::Event {
                    name: event.to_string(),
                    delegate: CsType::Scalar("void"),
                    add_slot: 0,
                    remove_slot: this_slot,
                });
            }
        } else if special {
            // Other special-name methods are not projected.
            continue;
        } else if let Some(member) = read_method(index, &method, &signature, name, this_slot) {
            members.push(member);
        }
    }

    // Drop any event whose add_/remove_ pair did not fully resolve.
    members.retain(|member| match member {
        Member::Event {
            delegate,
            add_slot,
            remove_slot,
            ..
        } => *add_slot != 0 && *remove_slot != 0 && !matches!(delegate, CsType::Scalar("void")),
        _ => true,
    });

    members
}

/// Returns `raw` verbatim, unless it is empty or a C# keyword, in which case it falls back to the
/// positional placeholder `p{i}` used throughout the projection for an unusable metadata name.
fn param_name(raw: &str, i: usize) -> String {
    if !raw.is_empty() && !is_reserved(raw) {
        raw.to_string()
    } else {
        format!("p{i}")
    }
}

/// Reads one parameter's name/direction/optional metadata at position `i`, or the defaults a
/// missing `MethodParam` row (name `p{i}`, `Direction::Input`, not optional) - the same fallback
/// every prior per-site reader used for a parameter whose row cannot be found.
fn param_meta(def: Option<&MethodParam<'_>>, i: usize) -> (String, Direction, bool) {
    match def {
        Some(def) => (
            param_name(def.name(), i),
            Direction::from(def.direction()),
            def.is_optional(),
        ),
        None => (format!("p{i}"), Direction::Input, false),
    }
}

/// Reads the first `count` of `signature.types` into the shared [`Param`] model: `map_ty` maps
/// (and may reject) the type at each index, and the resulting name/direction/optional metadata is
/// read from the matching `MethodParam` row. Returns `None` at the first type `map_ty` rejects.
/// Centralizes the name-collection-then-type-mapping pattern every method/function/constructor
/// reader previously duplicated.
fn read_params(
    index: &Index,
    method: MethodDef,
    signature: &windows_metadata::Signature,
    count: usize,
    mut map_ty: impl FnMut(usize, &Type) -> Option<CsType>,
) -> Option<Vec<Param>> {
    let defs = method.params_by_sequence(signature.types.len()).ok()?;
    let mut params = Vec::with_capacity(count);
    for (i, ty) in signature.types.iter().take(count).enumerate() {
        let mapped = map_ty(i, ty)?;
        let def = defs.params().get(i).and_then(Option::as_ref);
        let (name, direction, optional) = param_meta(def, i);
        let length = param_buffer_length(def);
        let projection = if let Some(length) = length {
            if is_utf16_buffer_alias(index, ty) {
                ParamProjection::Utf16Buffer(length)
            } else {
                ParamProjection::Buffer(length)
            }
        } else if matches!(direction, Direction::Input) && is_utf16_string_alias(index, ty) {
            ParamProjection::Utf16String
        } else {
            ParamProjection::Value
        };
        params.push(Param {
            name,
            ty: mapped,
            direction,
            optional,
            projection,
        });
    }
    normalize_buffer_lengths(&mut params);
    Some(params)
}

fn param_buffer_length(def: Option<&MethodParam<'_>>) -> Option<BufferLength> {
    let def = def?;
    for attribute in def.attributes() {
        let kind = match attribute.name() {
            "NativeArrayInfoAttribute" => BufferLength::Elements,
            "MemorySizeAttribute" => BufferLength::Bytes,
            _ => continue,
        };
        for (_, value) in attribute.value() {
            if let Value::I16(value) = value
                && value >= 0
            {
                return Some(kind(value as usize));
            }
        }
    }
    None
}

fn normalize_buffer_lengths(params: &mut [Param]) {
    let mut uses = vec![0usize; params.len()];
    for (position, param) in params.iter().enumerate() {
        let Some(length) = param.buffer_length() else {
            continue;
        };
        let count = length.param();
        if count != position && params.get(count).is_some_and(Param::is_buffer_count) {
            uses[count] += 1;
        }
    }
    for param in params {
        let Some(length) = param.buffer_length() else {
            continue;
        };
        if uses.get(length.param()) != Some(&1) || param.buffer_target().is_none() {
            param.projection = ParamProjection::Value;
        }
    }
}

fn is_utf16_string_alias(index: &Index, ty: &Type) -> bool {
    let Type::ValueName(name) = ty else {
        return false;
    };
    if !matches!(name.name.as_str(), "PCWSTR" | "LPCWSTR") {
        return false;
    }

    let Some(def) = index.get(&name.namespace, &name.name).next() else {
        return false;
    };
    matches!(
        native_typedef_underlying(def),
        Some(Type::PtrConst(element, 1)) if matches!(*element, Type::U16)
    )
}

fn is_utf16_buffer_alias(index: &Index, ty: &Type) -> bool {
    let Type::ValueName(name) = ty else {
        return false;
    };
    if !matches!(
        name.name.as_str(),
        "PCWSTR" | "LPCWSTR" | "PWSTR" | "LPWSTR"
    ) {
        return false;
    }
    let Some(def) = index.get(&name.namespace, &name.name).next() else {
        return false;
    };
    matches!(
        native_typedef_underlying(def),
        Some(Type::PtrConst(element, 1) | Type::PtrMut(element, 1))
            if matches!(*element, Type::U16)
    )
}

/// Projects a regular method, returning `None` if any parameter or the return uses an unsupported
/// type.
fn read_method(
    index: &Index,
    method: &MethodDef,
    signature: &windows_metadata::Signature,
    name: &str,
    slot: usize,
) -> Option<Member> {
    let ret = match &signature.return_type {
        Type::Void => None,
        ty => Some(CsType::map(index, ty)?),
    };

    let params = read_params(index, *method, signature, signature.types.len(), |_, ty| {
        CsType::map(index, ty)
    })?;

    Some(Member::Method {
        name: name.to_string(),
        params,
        ret,
        slot,
        abi: MethodAbi::WinRt,
    })
}

fn read_direct_method(
    index: &Index,
    method: &MethodDef,
    signature: &windows_metadata::Signature,
    name: &str,
    slot: usize,
) -> Option<Member> {
    let ret = match &signature.return_type {
        Type::Void => None,
        ty => {
            let ty = CsType::map(index, ty)?;
            if !ty.is_native_abi() {
                return None;
            }
            Some(ty)
        }
    };
    let hresult = matches!(ret, Some(CsType::HResult));
    let com_out = hresult
        .then(|| function_com_out(index, *method, signature))
        .flatten();
    let params = read_params(
        index,
        *method,
        signature,
        signature.types.len(),
        |position, ty| {
            let ty = if com_out
                .as_ref()
                .is_some_and(|(candidate, _)| *candidate == position)
            {
                com_out.as_ref().unwrap().1.clone()
            } else {
                CsType::map(index, ty)?
            };
            (ty.is_native_abi() || matches!(ty, CsType::ComOut { .. })).then_some(ty)
        },
    )?;
    Some(Member::Method {
        name: name.to_string(),
        params,
        ret,
        slot,
        abi: MethodAbi::Direct,
    })
}

/// Returns whether `name` is a C# keyword that cannot be used bare as a parameter name.
fn is_reserved(name: &str) -> bool {
    matches!(
        name,
        "abstract"
            | "as"
            | "base"
            | "bool"
            | "break"
            | "byte"
            | "case"
            | "catch"
            | "char"
            | "checked"
            | "class"
            | "const"
            | "continue"
            | "decimal"
            | "default"
            | "delegate"
            | "do"
            | "double"
            | "else"
            | "enum"
            | "event"
            | "explicit"
            | "extern"
            | "false"
            | "finally"
            | "fixed"
            | "float"
            | "for"
            | "foreach"
            | "goto"
            | "if"
            | "implicit"
            | "in"
            | "int"
            | "interface"
            | "internal"
            | "is"
            | "lock"
            | "long"
            | "namespace"
            | "new"
            | "null"
            | "object"
            | "operator"
            | "out"
            | "override"
            | "params"
            | "private"
            | "protected"
            | "public"
            | "readonly"
            | "ref"
            | "return"
            | "sbyte"
            | "sealed"
            | "short"
            | "sizeof"
            | "stackalloc"
            | "static"
            | "string"
            | "struct"
            | "switch"
            | "this"
            | "throw"
            | "true"
            | "try"
            | "typeof"
            | "uint"
            | "ulong"
            | "unchecked"
            | "unsafe"
            | "ushort"
            | "using"
            | "virtual"
            | "void"
            | "volatile"
            | "while"
    )
}

/// Decodes a `GuidAttribute` into a [`Guid`]. The attribute carries 11 fixed arguments
/// (`u32, u16, u16, 8 x u8`) per ECMA-335.
pub(crate) fn guid_attribute<'a>(row: impl HasAttributes<'a>) -> Option<Guid> {
    let attribute = row.find_attribute("GuidAttribute")?;
    let args = attribute.value();

    fn u32(value: &Value) -> u32 {
        match value {
            Value::U32(value) => *value,
            _ => panic!("expected u32 GUID argument"),
        }
    }
    fn u16(value: &Value) -> u16 {
        match value {
            Value::U16(value) => *value,
            _ => panic!("expected u16 GUID argument"),
        }
    }
    fn u8(value: &Value) -> u8 {
        match value {
            Value::U8(value) => *value,
            _ => panic!("expected u8 GUID argument"),
        }
    }

    Some(Guid(
        u32(&args[0].1),
        u16(&args[1].1),
        u16(&args[2].1),
        u8(&args[3].1),
        u8(&args[4].1),
        u8(&args[5].1),
        u8(&args[6].1),
        u8(&args[7].1),
        u8(&args[8].1),
        u8(&args[9].1),
        u8(&args[10].1),
    ))
}

/// Builds an `InvalidInput` I/O error from a plain message, used to surface an exact-selection
/// failure (a missing or unprojectable selected root, member, or dependency) instead of silently
/// dropping it.
fn invalid_input(message: String) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, message)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SelectedItemKind {
    Type,
    Function,
    Constant,
}

impl SelectedItemKind {
    fn name(self) -> &'static str {
        match self {
            Self::Type => "type",
            Self::Function => "function",
            Self::Constant => "constant",
        }
    }

    fn matches(self, item: Item<'_>) -> bool {
        matches!(
            (self, item),
            (Self::Type, Item::Type(_))
                | (Self::Function, Item::Fn(_))
                | (Self::Constant, Item::Const(_))
        )
    }
}

fn item_kind(item: Item<'_>) -> &'static str {
    match item {
        Item::Type(_) => "type",
        Item::Fn(_) => "function",
        Item::Const(_) => "constant",
    }
}

fn resolve_selected_item<'a, 'q>(
    index: &'a Index,
    all_index: &Index,
    qualified: &'q str,
    expected: SelectedItemKind,
    architecture: Option<Architecture>,
) -> Result<(&'q str, &'q str, Item<'a>), String> {
    let (namespace, name) = qualified.rsplit_once('.').ok_or_else(|| {
        format!(
            "windows-csharp: selected {} `{qualified}` must be a fully qualified metadata name",
            expected.name()
        )
    })?;

    let items = index.get_item(namespace, name).collect::<Vec<_>>();
    if let Some(item) = items.iter().copied().find(|item| expected.matches(*item)) {
        return Ok((namespace, name, item));
    }

    let all_items = all_index.get_item(namespace, name).collect::<Vec<_>>();
    if all_items.iter().copied().any(|item| expected.matches(item)) {
        return Err(format!(
            "windows-csharp: selected {} `{qualified}` is unavailable on the selected {} \
             architecture",
            expected.name(),
            architecture.map_or("target", Architecture::name)
        ));
    }

    let mut kinds = items
        .iter()
        .chain(all_items.iter())
        .map(|item| item_kind(*item))
        .collect::<BTreeSet<_>>();
    if let Some(actual) = kinds.pop_first() {
        return Err(format!(
            "windows-csharp: selected {} `{qualified}` has metadata kind `{actual}`, expected `{}`",
            expected.name(),
            expected.name()
        ));
    }

    Err(format!(
        "windows-csharp: selected {} `{qualified}` was not found in the metadata",
        expected.name()
    ))
}

fn qualified_name(def: TypeDef) -> String {
    format!("{}.{}", def.namespace(), def.name())
}

fn category_name(def: TypeDef) -> &'static str {
    match def.category() {
        TypeCategory::Class => "class",
        TypeCategory::Interface => "interface",
        TypeCategory::Delegate => "delegate",
        TypeCategory::Enum => "enum",
        TypeCategory::Struct => "struct",
        TypeCategory::Attribute => "attribute",
    }
}

fn wrong_member_root(type_name: &str, def: TypeDef) -> String {
    format!(
        "windows-csharp: selected member root `{type_name}` has metadata kind `{}`, expected \
         `class` or `interface`",
        category_name(def)
    )
}

fn type_debug(ty: &Type) -> String {
    format!("{ty:?}")
}

fn mapped_type_error(
    index: &Index,
    all_index: Option<&Index>,
    architecture: Option<Architecture>,
    ty: &Type,
) -> String {
    let named = match ty {
        Type::ClassName(name) | Type::ValueName(name) => Some(name),
        Type::Array(inner)
        | Type::ArrayFixed(inner, _)
        | Type::RefMut(inner)
        | Type::RefConst(inner)
        | Type::PtrMut(inner, _)
        | Type::PtrConst(inner, _) => {
            return mapped_type_error(index, all_index, architecture, inner);
        }
        _ => None,
    };
    if let Some(name) = named {
        if index.get(&name.namespace, &name.name).next().is_none()
            && all_index.is_some_and(|all| all.get(&name.namespace, &name.name).next().is_some())
        {
            return format!(
                "metadata type `{}.{}` is unavailable on the selected {} architecture",
                name.namespace,
                name.name,
                architecture.map_or("target", Architecture::name)
            );
        }
        for generic in &name.generics {
            let error = mapped_type_error(index, all_index, architecture, generic);
            if error.contains("unavailable on the selected") {
                return error;
            }
        }
    }
    if let Type::ClassName(name) = ty
        && name.generics.is_empty()
        && let Some(def) = index.get(&name.namespace, &name.name).next()
        && def.category() == TypeCategory::Delegate
        && def.flags().contains(TypeAttributes::WindowsRuntime)
        && let Err(error) = diagnose_delegate(index, all_index, architecture, def, "referenced")
    {
        return error;
    }
    format!("metadata type `{}` is not supported", type_debug(ty))
}

fn diagnose_function(
    index: &Index,
    all_index: Option<&Index>,
    architecture: Option<Architecture>,
    name: &str,
    method: MethodDef,
) -> String {
    if method.impl_map().is_none() {
        return format!("windows-csharp: selected function `{name}` has no native import mapping");
    }
    let signature = method.signature(&[]);
    if signature.flags.contains(MethodCallAttributes::VARARG) {
        return format!(
            "windows-csharp: selected function `{name}` is variadic, and variadic functions are \
             unsupported"
        );
    }
    if let Err(error) = method.params_by_sequence(signature.types.len()) {
        return format!(
            "windows-csharp: selected function `{name}` has invalid parameter metadata: {error}"
        );
    }
    let ret = match &signature.return_type {
        Type::Void => None,
        ty => match CsType::map(index, ty) {
            Some(ty) if ty.is_native_abi() => Some(ty),
            Some(_) => {
                return format!(
                    "windows-csharp: selected function `{name}` has unsupported return shape `{}`",
                    type_debug(ty)
                );
            }
            None => {
                return format!(
                    "windows-csharp: selected function `{name}` has unsupported return type: {}",
                    mapped_type_error(index, all_index, architecture, ty)
                );
            }
        },
    };
    let hresult = matches!(ret, Some(CsType::HResult));
    let com_out = hresult
        .then(|| function_com_out(index, method, &signature))
        .flatten();
    for (position, ty) in signature.types.iter().enumerate() {
        let mapped = if com_out
            .as_ref()
            .is_some_and(|(candidate, _)| *candidate == position)
        {
            com_out.as_ref().map(|(_, ty)| ty.clone())
        } else {
            CsType::map(index, ty)
        };
        match mapped {
            Some(CsType::ComOut { .. }) => {}
            Some(mapped) if mapped.is_native_abi() => {}
            Some(_) => {
                return format!(
                    "windows-csharp: selected function `{name}` parameter {position} has \
                     unsupported shape `{}`",
                    type_debug(ty)
                );
            }
            None => {
                return format!(
                    "windows-csharp: selected function `{name}` parameter {position} has \
                     unsupported type: {}",
                    mapped_type_error(index, all_index, architecture, ty)
                );
            }
        }
    }
    format!("windows-csharp: selected function `{name}` has an unsupported signature")
}

fn diagnose_constant(
    index: &Index,
    all_index: Option<&Index>,
    architecture: Option<Architecture>,
    name: &str,
    field: Field,
) -> String {
    let Some(constant) = field.constant() else {
        return format!("windows-csharp: selected constant `{name}` has no literal value");
    };
    let ty = constant.ty();
    match CsType::map(index, &ty) {
        Some(ty) if ty.is_native_abi() => {}
        Some(_) => {
            return format!(
                "windows-csharp: selected constant `{name}` has unsupported type shape `{}`",
                type_debug(&constant.ty())
            );
        }
        None => {
            return format!(
                "windows-csharp: selected constant `{name}` has unsupported type: {}",
                mapped_type_error(index, all_index, architecture, &ty)
            );
        }
    }
    if constant_literal(&constant.value()).is_none() {
        return format!(
            "windows-csharp: selected constant `{name}` has unsupported literal kind `{:?}`",
            constant.value()
        );
    }
    format!("windows-csharp: selected constant `{name}` is not supported")
}

fn diagnose_delegate(
    index: &Index,
    all_index: Option<&Index>,
    architecture: Option<Architecture>,
    def: TypeDef,
    role: &str,
) -> Result<(), String> {
    let name = qualified_name(def);
    if !def.flags().contains(TypeAttributes::WindowsRuntime) {
        return Err(format!(
            "windows-csharp: {role} delegate `{name}` is not a WinRT delegate"
        ));
    }
    let Some(invoke) = def.methods().find(|method| method.name() == "Invoke") else {
        return Err(format!(
            "windows-csharp: {role} delegate `{name}` has no Invoke method"
        ));
    };
    let signature = invoke.signature(&[]);
    if let Err(error) = invoke.params_by_sequence(signature.types.len()) {
        return Err(format!(
            "windows-csharp: {role} delegate `{name}` has invalid parameter metadata: {error}"
        ));
    }
    if !matches!(signature.return_type, Type::Void) {
        match CsType::map(index, &signature.return_type) {
            Some(ty) if ty.is_blittable() || matches!(ty, CsType::String) || ty.is_object() => {}
            Some(_) => {
                return Err(format!(
                    "windows-csharp: {role} delegate `{name}` has unsupported reverse-delegate \
                     return shape `{}`",
                    type_debug(&signature.return_type)
                ));
            }
            None => {
                return Err(format!(
                    "windows-csharp: {role} delegate `{name}` has unsupported return type: {}",
                    mapped_type_error(index, all_index, architecture, &signature.return_type)
                ));
            }
        }
    }
    for (position, ty) in signature.types.iter().enumerate() {
        match CsType::map(index, ty) {
            Some(mapped)
                if mapped.is_blittable()
                    || matches!(
                        mapped,
                        CsType::String | CsType::Object { .. } | CsType::Inspectable
                    ) => {}
            Some(_) => {
                return Err(format!(
                    "windows-csharp: {role} delegate `{name}` parameter {position} has unsupported \
                     reverse-delegate shape `{}`",
                    type_debug(ty)
                ));
            }
            None => {
                return Err(format!(
                    "windows-csharp: {role} delegate `{name}` parameter {position} has unsupported \
                     type: {}",
                    mapped_type_error(index, all_index, architecture, ty)
                ));
            }
        }
    }
    if guid_attribute(def).is_none() {
        return Err(format!(
            "windows-csharp: {role} delegate `{name}` has no GuidAttribute"
        ));
    }
    if read_delegate(index, def).is_none() {
        return Err(format!(
            "windows-csharp: {role} delegate `{name}` has an unsupported shape"
        ));
    }
    Ok(())
}

fn diagnose_enum(def: TypeDef, role: &str) -> Result<(), String> {
    let name = qualified_name(def);
    if enum_underlying(def).is_none() {
        return Err(format!(
            "windows-csharp: {role} enum `{name}` has an unsupported underlying type"
        ));
    }
    for field in def
        .fields()
        .filter(|field| field.flags().contains(FieldAttributes::Literal))
    {
        let Some(constant) = field.constant() else {
            return Err(format!(
                "windows-csharp: {role} enum `{name}` field `{}` has no literal value",
                field.name()
            ));
        };
        if enum_value(&constant.value()).is_none() {
            return Err(format!(
                "windows-csharp: {role} enum `{name}` field `{}` has an unsupported literal",
                field.name()
            ));
        }
    }
    Ok(())
}

fn diagnose_struct(
    index: &Index,
    all_index: Option<&Index>,
    architecture: Option<Architecture>,
    def: TypeDef,
    role: &str,
) -> Result<(), String> {
    let name = qualified_name(def);
    if def.has_attribute("NativeTypedefAttribute") {
        return Err(format!(
            "windows-csharp: {role} struct `{name}` is a native typedef with no standalone \
             projection"
        ));
    }
    if def.has_attribute("AlignmentAttribute") {
        return Err(format!(
            "windows-csharp: {role} struct `{name}` has unsupported explicit alignment metadata"
        ));
    }
    if def
        .fields()
        .all(|field| field.flags().contains(FieldAttributes::Static))
    {
        return Err(format!(
            "windows-csharp: {role} struct `{name}` has no instance fields"
        ));
    }
    for field in def
        .fields()
        .filter(|field| !field.flags().contains(FieldAttributes::Static))
    {
        let ty = field.ty();
        if let Type::ValueName(nested_name) = &ty
            && nested_name.namespace.is_empty()
            && let Some(nested) = index
                .nested(def)
                .find(|nested| nested.name() == nested_name.name)
        {
            diagnose_struct(index, all_index, architecture, nested, role)?;
            continue;
        }
        match CsType::map(index, &ty) {
            Some(mapped) if mapped.is_struct_field_abi() => {}
            Some(_) => {
                return Err(format!(
                    "windows-csharp: {role} struct `{name}` field `{}` has unsupported shape `{}`",
                    field.name(),
                    type_debug(&ty)
                ));
            }
            None => {
                return Err(format!(
                    "windows-csharp: {role} struct `{name}` field `{}` has unsupported type: {}",
                    field.name(),
                    mapped_type_error(index, all_index, architecture, &ty)
                ));
            }
        }
    }
    if struct_needs_abi(index, def) && def.flags().contains(TypeAttributes::ExplicitLayout) {
        return Err(format!(
            "windows-csharp: {role} struct `{name}` combines explicit layout with fields requiring \
             ABI conversion"
        ));
    }
    if read_struct(index, def).is_none() {
        return Err(format!(
            "windows-csharp: {role} struct `{name}` has an unsupported shape"
        ));
    }
    Ok(())
}

fn enqueue_struct_field_types(index: &Index, def: TypeDef, work: &mut VecDeque<Type>) {
    for field in def
        .fields()
        .filter(|field| !field.flags().contains(FieldAttributes::Static))
    {
        let ty = field.ty();
        if let Type::ValueName(name) = &ty
            && name.namespace.is_empty()
            && let Some(nested) = index.nested(def).find(|nested| nested.name() == name.name)
        {
            enqueue_struct_field_types(index, nested, work);
        } else {
            work.push_back(ty);
        }
    }
}

/// Keeps only the members named in `selection`, or every member when `selection` is `None` (an
/// unnarrowed [`Selection::All`] root). Never renumbers a kept member's vtable slot.
fn filter_members(members: Vec<Member>, selection: &Option<HashSet<String>>) -> Vec<Member> {
    match selection {
        None => members,
        Some(names) => members
            .into_iter()
            .filter(|member| names.contains(member_name(member)))
            .collect(),
    }
}

/// Returns the raw metadata parameter/return types a projected member's ABI calls actually touch,
/// resolved by mapping the member's recorded vtable slot back to its `MethodDef` (slot 6 is the
/// first method after `iface`'s `IInspectable` base, matching the numbering [`read_members`]
/// assigned). Used to seed the exact-selection dependency closure from only the members that
/// survived filtering. An event's `remove_` accessor carries no member-relevant type (just the
/// opaque registration token), so only its `add_` handler type is collected - mirroring what
/// [`read_members`] itself maps for an event.
fn member_signature_types(iface: TypeDef, member: &Member) -> Vec<Type> {
    let base = if iface.flags().contains(TypeAttributes::WindowsRuntime) {
        6
    } else {
        3
    };
    member_signature_types_with(member, |slot| iface.methods().nth(slot - base))
}

fn member_signature_types_with<'a>(
    member: &Member,
    mut method_at: impl FnMut(usize) -> Option<MethodDef<'a>>,
) -> Vec<Type> {
    match member {
        Member::Property {
            get_slot, put_slot, ..
        } => {
            let mut types = Vec::new();
            if let Some(method) = get_slot.and_then(&mut method_at) {
                types.push(method.signature(&[]).return_type);
            }
            if let Some(method) = put_slot.and_then(&mut method_at) {
                if let Some(ty) = method.signature(&[]).types.into_iter().next() {
                    types.push(ty);
                }
            }
            types
        }
        Member::Method { slot, .. } => match method_at(*slot) {
            Some(method) => {
                let signature = method.signature(&[]);
                let mut types = vec![signature.return_type];
                types.extend(signature.types);
                types
            }
            None => Vec::new(),
        },
        Member::Event { add_slot, .. } => method_at(*add_slot)
            .and_then(|method| method.signature(&[]).types.into_iter().next())
            .into_iter()
            .collect(),
    }
}

fn declared_member_names(interfaces: &[TypeDef<'_>]) -> BTreeSet<String> {
    interfaces
        .iter()
        .flat_map(|interface| interface.methods())
        .map(|method| metadata_member_name(method.name()).to_string())
        .collect()
}

fn type_generics(def: TypeDef) -> Vec<Type> {
    def.generic_params()
        .enumerate()
        .map(|(index, param)| Type::Generic(param.name().to_string(), index as u16))
        .collect()
}

fn metadata_method_supported(
    index: &Index,
    owner: TypeDef,
    method: MethodDef,
    abi: MethodAbi,
) -> bool {
    let signature = method.signature(&type_generics(owner));
    if method.params_by_sequence(signature.types.len()).is_err() {
        return false;
    }
    if matches!(abi, MethodAbi::Direct) {
        return read_direct_method(index, &method, &signature, method.name(), 3).is_some();
    }

    let name = method.name();
    if method.flags().contains(MethodAttributes::SpecialName) {
        if name.starts_with("get_") {
            return signature.types.is_empty()
                && !matches!(signature.return_type, Type::Void)
                && CsType::map(index, &signature.return_type).is_some();
        }
        if name.starts_with("put_") {
            return matches!(signature.return_type, Type::Void)
                && signature.types.len() == 1
                && CsType::map(index, &signature.types[0]).is_some();
        }
        if name.starts_with("add_") {
            return signature.types.len() == 1 && CsType::map(index, &signature.types[0]).is_some();
        }
        if name.starts_with("remove_") {
            return matches!(signature.return_type, Type::Void) && signature.types.len() == 1;
        }
        return false;
    }
    read_method(index, &method, &signature, name, 6).is_some()
}

fn diagnose_member_signature(
    index: &Index,
    all_index: Option<&Index>,
    architecture: Option<Architecture>,
    owner: TypeDef,
    member_name: &str,
    abi: MethodAbi,
) -> String {
    let owner_name = qualified_name(owner);
    for method in owner
        .methods()
        .filter(|method| metadata_member_name(method.name()) == member_name)
    {
        let method_name = method.name();
        let signature = method.signature(&type_generics(owner));
        if let Err(error) = method.params_by_sequence(signature.types.len()) {
            return format!(
                "windows-csharp: member `{member_name}` on `{owner_name}` method `{method_name}` \
                 has invalid parameter metadata: {error}"
            );
        }
        if !matches!(signature.return_type, Type::Void) {
            match CsType::map(index, &signature.return_type) {
                Some(ty) if !matches!(abi, MethodAbi::Direct) || ty.is_native_abi() => {}
                Some(_) => {
                    return format!(
                        "windows-csharp: member `{member_name}` on `{owner_name}` has unsupported \
                         return shape `{}` in `{method_name}`",
                        type_debug(&signature.return_type)
                    );
                }
                None => {
                    return format!(
                        "windows-csharp: member `{member_name}` on `{owner_name}` has unsupported \
                         return type in `{method_name}`: {}",
                        mapped_type_error(index, all_index, architecture, &signature.return_type,)
                    );
                }
            }
        }
        for (position, ty) in signature.types.iter().enumerate() {
            match CsType::map(index, ty) {
                Some(mapped) if !matches!(abi, MethodAbi::Direct) || mapped.is_native_abi() => {}
                Some(_) => {
                    return format!(
                        "windows-csharp: member `{member_name}` on `{owner_name}` parameter \
                         {position} has unsupported shape `{}` in `{method_name}`",
                        type_debug(ty)
                    );
                }
                None => {
                    return format!(
                        "windows-csharp: member `{member_name}` on `{owner_name}` parameter \
                         {position} has unsupported type in `{method_name}`: {}",
                        mapped_type_error(index, all_index, architecture, ty)
                    );
                }
            }
        }
    }
    format!(
        "windows-csharp: member `{member_name}` on `{owner_name}` has an unsupported accessor or \
         signature shape"
    )
}

fn diagnose_selected_member(
    index: &Index,
    all_index: Option<&Index>,
    architecture: Option<Architecture>,
    selected_type: TypeDef,
    interfaces: &[TypeDef<'_>],
    member_name: &str,
    abi: MethodAbi,
) -> String {
    let mut declaring = interfaces
        .iter()
        .copied()
        .filter(|interface| {
            interface
                .methods()
                .any(|method| metadata_member_name(method.name()) == member_name)
        })
        .collect::<Vec<_>>();
    declaring.sort_by_key(|def| qualified_name(*def));
    if let Some(interface) = declaring.first() {
        let detail =
            diagnose_member_signature(index, all_index, architecture, *interface, member_name, abi);
        format!(
            "windows-csharp: member `{member_name}` on selected type `{}` is not supported: {}",
            qualified_name(selected_type),
            detail
                .strip_prefix("windows-csharp: ")
                .unwrap_or(detail.as_str())
        )
    } else {
        format!(
            "windows-csharp: member `{member_name}` was not found on selected type `{}`",
            qualified_name(selected_type)
        )
    }
}

fn class_declared_interfaces<'a>(index: &'a Index, def: TypeDef<'a>) -> Vec<TypeDef<'a>> {
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    let mut class = Some(def);
    while let Some(current) = class {
        if let Some(default) = default_interface(index, current) {
            let key = qualified_name(default);
            if seen.insert(key) {
                result.push(default);
            }
        }
        for implementation in current.interface_impls() {
            let Type::ClassName(name) = implementation.interface(&type_generics(current)) else {
                continue;
            };
            let Some(interface) = index.get(&name.namespace, &name.name).next() else {
                continue;
            };
            let key = qualified_name(interface);
            if seen.insert(key) {
                result.push(interface);
            }
        }
        class = current.extends().and_then(|extends| {
            if extends.namespace() == "System" && extends.name() == "Object" {
                None
            } else {
                index.get(extends.namespace(), extends.name()).next()
            }
        });
    }
    result
}

fn required_type_def<'a>(
    index: &'a Index,
    all_index: &Index,
    architecture: Option<Architecture>,
    namespace: &str,
    name: &str,
    context: &str,
) -> Result<TypeDef<'a>, String> {
    let lookup = name.split_once('`').map_or(name, |(name, _)| name);
    if let Some(def) = index.get(namespace, lookup).next() {
        return Ok(def);
    }
    if all_index.get(namespace, lookup).next().is_some() {
        return Err(format!(
            "windows-csharp: {context} requires type `{namespace}.{name}`, which is unavailable on \
             the selected {} architecture",
            architecture.map_or("target", Architecture::name)
        ));
    }
    Err(format!(
        "windows-csharp: {context} requires type `{namespace}.{name}`, which was not found in the \
         metadata"
    ))
}

fn validate_interface_bases(
    index: &Index,
    all_index: &Index,
    architecture: Option<Architecture>,
    def: TypeDef,
    seen: &mut HashSet<String>,
) -> Result<(), String> {
    let identity = qualified_name(def);
    if !seen.insert(identity.clone()) {
        return Ok(());
    }
    if let Some(extends) = def.extends()
        && extends.name() != "IUnknown"
        && !(extends.namespace() == "System" && extends.name() == "Object")
    {
        let base = required_type_def(
            index,
            all_index,
            architecture,
            extends.namespace(),
            extends.name(),
            &format!("interface `{identity}`"),
        )?;
        validate_interface_bases(index, all_index, architecture, base, seen)?;
    }
    for implementation in def.interface_impls() {
        let Type::ClassName(name) = implementation.interface(&type_generics(def)) else {
            return Err(format!(
                "windows-csharp: interface `{identity}` has an unsupported non-class interface \
                 implementation"
            ));
        };
        if name.name == "IUnknown" {
            continue;
        }
        let base = required_type_def(
            index,
            all_index,
            architecture,
            &name.namespace,
            &name.name,
            &format!("interface `{identity}`"),
        )?;
        validate_interface_bases(index, all_index, architecture, base, seen)?;
    }
    Ok(())
}

fn validate_class_hierarchy(
    index: &Index,
    all_index: &Index,
    architecture: Option<Architecture>,
    def: TypeDef,
) -> Result<(), String> {
    let mut class = Some(def);
    let mut seen_classes = HashSet::new();
    let mut seen_interfaces = HashSet::new();
    while let Some(current) = class {
        let identity = qualified_name(current);
        if !seen_classes.insert(identity.clone()) {
            break;
        }
        for implementation in current.interface_impls() {
            let Type::ClassName(name) = implementation.interface(&type_generics(current)) else {
                return Err(format!(
                    "windows-csharp: class `{identity}` has an unsupported non-class interface \
                     implementation"
                ));
            };
            let interface = required_type_def(
                index,
                all_index,
                architecture,
                &name.namespace,
                &name.name,
                &format!("class `{identity}`"),
            )?;
            validate_interface_bases(
                index,
                all_index,
                architecture,
                interface,
                &mut seen_interfaces,
            )?;
        }
        class = match current.extends() {
            Some(extends) if !(extends.namespace() == "System" && extends.name() == "Object") => {
                Some(required_type_def(
                    index,
                    all_index,
                    architecture,
                    extends.namespace(),
                    extends.name(),
                    &format!("class `{identity}`"),
                )?)
            }
            _ => None,
        };
    }
    Ok(())
}

fn diagnose_factory(
    index: &Index,
    all_index: Option<&Index>,
    architecture: Option<Architecture>,
    class: TypeDef,
    iface: TypeDef,
    composable: bool,
) -> String {
    let class_name = qualified_name(class);
    let interface_name = qualified_name(iface);
    if guid_attribute(iface).is_none() {
        return format!(
            "windows-csharp: selected class `{class_name}` activation interface \
             `{interface_name}` has no GuidAttribute"
        );
    }
    for method in iface
        .methods()
        .filter(|method| !method.flags().contains(MethodAttributes::SpecialName))
    {
        let signature = method.signature(&[]);
        if let Err(error) = method.params_by_sequence(signature.types.len()) {
            return format!(
                "windows-csharp: selected class `{class_name}` constructor `{}` has invalid \
                 parameter metadata: {error}",
                method.name()
            );
        }
        match CsType::map(index, &signature.return_type) {
            Some(ty) if ty.is_object() => {}
            Some(_) => {
                return format!(
                    "windows-csharp: selected class `{class_name}` constructor `{}` has \
                     unsupported return shape `{}`",
                    method.name(),
                    type_debug(&signature.return_type)
                );
            }
            None => {
                return format!(
                    "windows-csharp: selected class `{class_name}` constructor `{}` has \
                     unsupported return type: {}",
                    method.name(),
                    mapped_type_error(index, all_index, architecture, &signature.return_type)
                );
            }
        }
        let Some(count) = (if composable {
            signature.types.len().checked_sub(2)
        } else {
            Some(signature.types.len())
        }) else {
            return format!(
                "windows-csharp: selected class `{class_name}` composable constructor `{}` has \
                 fewer than two composition parameters",
                method.name()
            );
        };
        for (position, ty) in signature.types.iter().take(count).enumerate() {
            if CsType::map(index, ty).is_none() {
                return format!(
                    "windows-csharp: selected class `{class_name}` constructor `{}` parameter \
                     {position} has unsupported type: {}",
                    method.name(),
                    mapped_type_error(index, all_index, architecture, ty)
                );
            }
        }
    }
    format!(
        "windows-csharp: selected class `{class_name}` activation interface `{interface_name}` has \
         no projectable constructors"
    )
}

fn validate_factory_returns(
    index: &Index,
    all_index: &Index,
    architecture: Option<Architecture>,
    class: TypeDef,
    iface: TypeDef,
) -> Result<(), String> {
    let class_name = qualified_name(class);
    for method in iface
        .methods()
        .filter(|method| !method.flags().contains(MethodAttributes::SpecialName))
    {
        let ty = method.signature(&[]).return_type;
        match CsType::map(index, &ty) {
            Some(mapped) if mapped.is_object() => {}
            Some(_) => {
                return Err(format!(
                    "windows-csharp: selected class `{class_name}` constructor `{}` has \
                     unsupported return shape `{}`",
                    method.name(),
                    type_debug(&ty)
                ));
            }
            None => {
                return Err(format!(
                    "windows-csharp: selected class `{class_name}` constructor `{}` has \
                     unsupported return type: {}",
                    method.name(),
                    mapped_type_error(index, Some(all_index), architecture, &ty)
                ));
            }
        }
    }
    Ok(())
}

/// Reads an exact-selection class root: the full (unfiltered) default-interface member list and
/// forwarder candidates from [`class_parts`], validated and narrowed by `selection`, plus the raw
/// signature types the kept members touch (for the dependency closure). Returns an error naming a
/// requested member that does not exist on the type rather than silently ignoring it.
fn select_class(
    index: &Index,
    all_index: &Index,
    architecture: Option<Architecture>,
    def: TypeDef,
    selection: &Option<HashSet<String>>,
) -> Result<(Class, Vec<Type>), String> {
    validate_class_hierarchy(index, all_index, architecture, def)?;
    let (default, members, forwarder_sources) = class_parts(index, def).ok_or_else(|| {
        format!(
            "windows-csharp: selected type `{}.{}` has no projectable default interface",
            def.namespace(),
            def.name()
        )
    })?;

    // Decode activation up front (unfiltered): constructors are emitted whenever metadata provides
    // them (they are never selected by a member name), while static members join the selectable
    // surface and are narrowed by `selection` like instance and forwarder members.
    let declared_interfaces = class_declared_interfaces(index, def);
    let mut static_interfaces = Vec::new();
    let default_activation = has_default_activation(def);
    let mut raw_factories: Vec<(TypeDef, Factory)> = Vec::new();
    for attribute in def.attributes() {
        let composable = match attribute.name() {
            "ComposableAttribute" => true,
            "ActivatableAttribute" => false,
            _ => continue,
        };
        let Some(name) = factory_interface_name(&attribute) else {
            continue;
        };
        let iface = required_type_def(
            index,
            all_index,
            architecture,
            &name.namespace,
            &name.name,
            &format!(
                "selected class `{}` activation metadata",
                qualified_name(def)
            ),
        )?;
        let declared = iface
            .methods()
            .filter(|method| !method.flags().contains(MethodAttributes::SpecialName))
            .count();
        if declared == 0 {
            continue;
        }
        let Some(factory) = read_factory(index, iface, composable) else {
            return Err(diagnose_factory(
                index,
                Some(all_index),
                architecture,
                def,
                iface,
                composable,
            ));
        };
        validate_factory_returns(index, all_index, architecture, def, iface)?;
        if factory.constructors.len() != declared {
            return Err(diagnose_factory(
                index,
                Some(all_index),
                architecture,
                def,
                iface,
                composable,
            ));
        }
        raw_factories.push((iface, factory));
    }
    let mut raw_statics: Vec<(TypeDef, StaticInterface)> = Vec::new();
    for attribute in def.attributes() {
        if attribute.name() != "StaticAttribute" {
            continue;
        }
        let name = factory_interface_name(&attribute).ok_or_else(|| {
            format!(
                "windows-csharp: selected class `{}` has a StaticAttribute without an interface \
                 type",
                qualified_name(def)
            )
        })?;
        let iface = required_type_def(
            index,
            all_index,
            architecture,
            &name.namespace,
            &name.name,
            &format!("selected class `{}` static metadata", qualified_name(def)),
        )?;
        static_interfaces.push(iface);
        if let Some(item) = read_static(index, iface) {
            raw_statics.push((iface, item));
        }
    }

    let declared = declared_member_names(
        &declared_interfaces
            .iter()
            .chain(static_interfaces.iter())
            .copied()
            .collect::<Vec<_>>(),
    );
    if let Some(wanted) = selection {
        let mut available: HashSet<&str> = members.iter().map(member_name).collect();
        for (_, forwarded) in &forwarder_sources {
            available.extend(forwarded.iter().map(member_name));
        }
        for (_, item) in &raw_statics {
            available.extend(item.members.iter().map(member_name));
        }
        let mut wanted = wanted.iter().collect::<Vec<_>>();
        wanted.sort();
        for name in wanted {
            if !available.contains(name.as_str()) {
                if declared.contains(name) {
                    return Err(diagnose_selected_member(
                        index,
                        Some(all_index),
                        architecture,
                        def,
                        &declared_interfaces
                            .iter()
                            .chain(static_interfaces.iter())
                            .copied()
                            .collect::<Vec<_>>(),
                        name,
                        MethodAbi::WinRt,
                    ));
                }
                return Err(format!(
                    "windows-csharp: member `{name}` was not found on selected type `{}`",
                    qualified_name(def)
                ));
            }
        }
    } else {
        let mut available: HashSet<&str> = members.iter().map(member_name).collect();
        for (_, forwarded) in &forwarder_sources {
            available.extend(forwarded.iter().map(member_name));
        }
        for (_, item) in &raw_statics {
            available.extend(item.members.iter().map(member_name));
        }
        if let Some(name) = declared
            .iter()
            .find(|name| !available.contains(name.as_str()))
        {
            return Err(diagnose_selected_member(
                index,
                Some(all_index),
                architecture,
                def,
                &declared_interfaces
                    .iter()
                    .chain(static_interfaces.iter())
                    .copied()
                    .collect::<Vec<_>>(),
                name,
                MethodAbi::WinRt,
            ));
        }
    }

    let selected_names = match selection {
        Some(names) => names.iter().map(String::as_str).collect::<HashSet<_>>(),
        None => declared.iter().map(String::as_str).collect::<HashSet<_>>(),
    };
    let mut unsupported = Vec::new();
    for method in default.methods() {
        let name = metadata_member_name(method.name());
        if selected_names.contains(name)
            && !metadata_method_supported(index, default, method, MethodAbi::WinRt)
        {
            unsupported.push((name.to_string(), default));
        }
    }
    for (interface, forwarded) in &forwarder_sources {
        let forwarded = forwarded.iter().map(member_name).collect::<HashSet<_>>();
        for method in interface.methods() {
            let name = metadata_member_name(method.name());
            if selected_names.contains(name)
                && forwarded.contains(name)
                && !metadata_method_supported(index, *interface, method, MethodAbi::WinRt)
            {
                unsupported.push((name.to_string(), *interface));
            }
        }
    }
    for (interface, item) in &raw_statics {
        let emitted = item.members.iter().map(member_name).collect::<HashSet<_>>();
        for method in interface.methods() {
            let name = metadata_member_name(method.name());
            if selected_names.contains(name)
                && emitted.contains(name)
                && !metadata_method_supported(index, *interface, method, MethodAbi::WinRt)
            {
                unsupported.push((name.to_string(), *interface));
            }
        }
    }
    unsupported.sort_by_key(|(name, interface)| (name.clone(), qualified_name(*interface)));
    if let Some((name, interface)) = unsupported.first() {
        return Err(diagnose_selected_member(
            index,
            Some(all_index),
            architecture,
            def,
            &[*interface],
            name,
            MethodAbi::WinRt,
        ));
    }

    let filtered = filter_members(members, selection);
    let mut dep_types = Vec::new();
    for member in &filtered {
        dep_types.extend(member_signature_types(default, member));
    }

    let mut forwarders = Vec::new();
    for (interface, forwarded) in forwarder_sources {
        let filtered_forwarded = filter_members(forwarded, selection);
        if filtered_forwarded.is_empty() {
            continue;
        }
        for member in &filtered_forwarded {
            dep_types.extend(member_signature_types(interface, member));
        }
        let iid = guid_attribute(interface).ok_or_else(|| {
            format!(
                "windows-csharp: interface `{}.{}` has no GuidAttribute",
                interface.namespace(),
                interface.name()
            )
        })?;
        forwarders.push(Forwarder {
            iid,
            members: filtered_forwarded,
        });
    }

    // Constructors are always emitted, so every factory constructor's parameter types participate
    // in the dependency closure regardless of member selection.
    let mut factories = Vec::with_capacity(raw_factories.len());
    for (iface, factory) in raw_factories {
        for ctor in &factory.constructors {
            if let Some(method) = iface.methods().nth(ctor.slot - 6) {
                let signature = method.signature(&[]);
                let count = if factory.composable {
                    signature.types.len().saturating_sub(2)
                } else {
                    signature.types.len()
                };
                dep_types.extend(signature.types.into_iter().take(count));
            }
        }
        factories.push(factory);
    }

    // Static members are narrowed by `selection`; each surviving member's signature types join the
    // closure so a referenced delegate/struct/enum/class is projected.
    let mut statics = Vec::new();
    for (iface, mut item) in raw_statics {
        item.members = filter_members(item.members, selection);
        if item.members.is_empty() {
            continue;
        }
        for member in &item.members {
            dep_types.extend(member_signature_types(iface, member));
        }
        statics.push(item);
    }

    let class = Class {
        namespace: def.namespace().to_string(),
        name: def.name().to_string(),
        default_activation,
        factories,
        statics,
        default_iid: guid_attribute(default).ok_or_else(|| {
            format!(
                "windows-csharp: default interface `{}.{}` has no GuidAttribute",
                default.namespace(),
                default.name()
            )
        })?,
        members: filtered,
        forwarders,
        compatible: class_compatible_types(index, def),
    };
    Ok((class, dep_types))
}

/// Reads an exact-selection interface root, validated and narrowed by `selection`, plus the raw
/// signature types the kept members touch (for the dependency closure). Returns an error naming a
/// requested member that does not exist on the type rather than silently ignoring it.
fn select_interface(
    index: &Index,
    all_index: &Index,
    architecture: Option<Architecture>,
    def: TypeDef,
    selection: &Option<HashSet<String>>,
) -> Result<(Interface, Vec<Type>), String> {
    validate_interface_bases(index, all_index, architecture, def, &mut HashSet::new())?;
    let winrt = def.flags().contains(TypeAttributes::WindowsRuntime);
    let (members, native_base, native_own_members) = if winrt {
        (read_members(index, def, 6, MethodAbi::WinRt), None, None)
    } else {
        read_native_interface_members(index, def)
    };

    let available: HashSet<&str> = members.iter().map(member_name).collect();
    let declared_interfaces = if winrt {
        vec![def]
    } else {
        native_interface_chain(index, def)
    };
    let declared = declared_member_names(&declared_interfaces);
    if let Some(wanted) = selection {
        let mut wanted = wanted.iter().collect::<Vec<_>>();
        wanted.sort();
        for name in wanted {
            if !available.contains(name.as_str()) {
                if declared.contains(name) {
                    return Err(diagnose_selected_member(
                        index,
                        Some(all_index),
                        architecture,
                        def,
                        &declared_interfaces,
                        name,
                        if winrt {
                            MethodAbi::WinRt
                        } else {
                            MethodAbi::Direct
                        },
                    ));
                } else {
                    return Err(format!(
                        "windows-csharp: member `{name}` was not found on selected type `{}`",
                        qualified_name(def)
                    ));
                }
            }
        }
    } else {
        let unsupported = declared
            .iter()
            .filter(|name| !available.contains(name.as_str()))
            .collect::<Vec<_>>();
        if let Some(name) = unsupported.first() {
            return Err(diagnose_selected_member(
                index,
                Some(all_index),
                architecture,
                def,
                &declared_interfaces,
                name,
                if winrt {
                    MethodAbi::WinRt
                } else {
                    MethodAbi::Direct
                },
            ));
        }
    }

    let selected_names = match selection {
        Some(names) => names.iter().map(String::as_str).collect::<HashSet<_>>(),
        None => declared.iter().map(String::as_str).collect::<HashSet<_>>(),
    };
    let abi = if winrt {
        MethodAbi::WinRt
    } else {
        MethodAbi::Direct
    };
    let mut unsupported = Vec::new();
    for interface in &declared_interfaces {
        for method in interface.methods() {
            let name = metadata_member_name(method.name());
            if selected_names.contains(name)
                && !metadata_method_supported(index, *interface, method, abi)
            {
                unsupported.push((name.to_string(), *interface));
            }
        }
    }
    unsupported.sort_by_key(|(name, interface)| (name.clone(), qualified_name(*interface)));
    if let Some((name, interface)) = unsupported.first() {
        return Err(diagnose_selected_member(
            index,
            Some(all_index),
            architecture,
            def,
            &[*interface],
            name,
            abi,
        ));
    }

    let filtered = filter_members(members, selection);
    let mut dep_types = Vec::new();
    if winrt {
        for member in &filtered {
            dep_types.extend(member_signature_types(def, member));
        }
    } else {
        let methods = native_interface_methods(index, def);
        for member in &filtered {
            dep_types.extend(member_signature_types_with(member, |slot| {
                methods.get(&slot).copied()
            }));
        }
    }

    let iid = guid_attribute(def).ok_or_else(|| {
        format!(
            "windows-csharp: interface `{}.{}` has no GuidAttribute",
            def.namespace(),
            def.name()
        )
    })?;
    Ok((
        Interface {
            namespace: def.namespace().to_string(),
            name: def.name().to_string(),
            iid,
            members: filtered,
            compatible: interface_compatible_types(index, def),
            native_base,
            native_own_members: selection.is_none().then_some(native_own_members).flatten(),
        },
        dep_types,
    ))
}

/// Builds a marker projection for a class pulled in purely as a dependency (referenced by a
/// selected member's signature, but not itself a selection root): an `Iid`, activation-free
/// constructors, and `As<T>()`, but no members and no non-default-interface forwarders. Requirement
/// 3's dependency closure only needs the type to be nameable and castable, not to expose its API.
fn marker_class(index: &Index, def: TypeDef) -> Result<Class, String> {
    let default_iid = default_interface_iid(index, def).ok_or_else(|| {
        format!(
            "windows-csharp: dependency class `{}.{}` has no projectable default interface",
            def.namespace(),
            def.name()
        )
    })?;
    Ok(Class {
        namespace: def.namespace().to_string(),
        name: def.name().to_string(),
        default_activation: false,
        factories: Vec::new(),
        statics: Vec::new(),
        default_iid,
        members: Vec::new(),
        forwarders: Vec::new(),
        compatible: class_compatible_types(index, def),
    })
}

fn class_compatible_types(index: &Index, def: TypeDef) -> Vec<String> {
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    let mut class = Some(def);

    while let Some(current) = class {
        let class_name = format!("{}.{}", current.namespace(), current.name());
        if seen.insert(class_name.clone()) {
            result.push(class_name);
        }
        for imp in current.interface_impls() {
            let Type::ClassName(name) = imp.interface(&type_generics(current)) else {
                continue;
            };
            let Some(interface) = index.get(&name.namespace, &name.name).next() else {
                continue;
            };
            for interface_name in interface_compatible_types(index, interface) {
                if seen.insert(interface_name.clone()) {
                    result.push(interface_name);
                }
            }
        }
        class = current.extends().and_then(|extends| {
            if extends.namespace() == "System" && extends.name() == "Object" {
                None
            } else {
                index.get(extends.namespace(), extends.name()).next()
            }
        });
    }

    result
}

/// Builds a marker projection for an interface pulled in purely as a dependency: an `Iid` and
/// `As<T>()`, but no members. See [`marker_class`].
fn marker_interface(index: &Index, def: TypeDef) -> Result<Interface, String> {
    let iid = guid_attribute(def).ok_or_else(|| {
        format!(
            "windows-csharp: interface `{}.{}` has no GuidAttribute",
            def.namespace(),
            def.name()
        )
    })?;
    Ok(Interface {
        namespace: def.namespace().to_string(),
        name: def.name().to_string(),
        iid,
        members: Vec::new(),
        compatible: interface_compatible_types(index, def),
        native_base: None,
        native_own_members: None,
    })
}

fn interface_compatible_types(index: &Index, def: TypeDef) -> Vec<String> {
    fn visit(index: &Index, def: TypeDef, seen: &mut HashSet<String>, result: &mut Vec<String>) {
        let name = format!("{}.{}", def.namespace(), def.name());
        if !seen.insert(name.clone()) {
            return;
        }
        result.push(name);
        if let Some(extends) = def.extends() {
            if extends.name() != "IUnknown"
                && let Some(base) = index.get(extends.namespace(), extends.name()).next()
            {
                visit(index, base, seen, result);
            }
        }
        for imp in def.interface_impls() {
            let Type::ClassName(name) = imp.interface(&type_generics(def)) else {
                continue;
            };
            if let Some(base) = index.get(&name.namespace, &name.name).next() {
                visit(index, base, seen, result);
            }
        }
    }

    let mut result = Vec::new();
    visit(index, def, &mut HashSet::new(), &mut result);
    result
}

/// Walks the exact-selection dependency closure: every raw metadata type reachable from a selected
/// member's signature. A referenced class or interface that is not itself a selection root is
/// recorded as a marker (see [`marker_class`]/[`marker_interface`]); a referenced delegate is
/// recorded and its `Invoke` signature is enqueued in turn; a referenced enum is recorded; a
/// referenced struct is recorded and its own (non-static) field types are enqueued, recursing
/// through nested structs/enums; a referenced opaque handle (see [`native_handle_value`]) is
/// recorded with no further field enqueuing (its one field is an opaque pointer, not a further
/// dependency). A referenced native typedef that is not itself a handle enqueues its underlying
/// type: scalar aliases terminate immediately, pointer aliases retain named pointee dependencies,
/// and a chained handle such as `HGLOBAL -> HANDLE` records the target handle wrapper. Every type
/// this walk cannot resolve or project is a hard error (requirement 5) rather than a silently
/// incomplete projection.
struct Closure<'a> {
    index: &'a Index,
    all_index: &'a Index,
    architecture: Option<Architecture>,
    class_roots: &'a HashMap<TypeDef<'a>, Option<HashSet<String>>>,
    interface_roots: &'a HashMap<TypeDef<'a>, Option<HashSet<String>>>,
    class_markers: HashSet<TypeDef<'a>>,
    interface_markers: HashSet<TypeDef<'a>>,
    delegates: HashSet<TypeDef<'a>>,
    native_callbacks: HashSet<TypeDef<'a>>,
    enums: HashSet<TypeDef<'a>>,
    structs: HashSet<TypeDef<'a>>,
    handles: HashSet<TypeDef<'a>>,
}

impl<'a> Closure<'a> {
    /// Discovers the dependencies of one type reached from a selected signature, enqueuing further
    /// work in `work` and, for a delegate's `Invoke` signature, recording its types in
    /// `signature_types` too (so the generic-collection/`Object` scan used to build
    /// [`Collections`] also covers delegate signatures, not just root members).
    fn discover(
        &mut self,
        ty: &Type,
        work: &mut VecDeque<Type>,
        signature_types: &mut Vec<Type>,
    ) -> Result<(), String> {
        match ty {
            Type::ValueName(tn) => {
                if tn.namespace == "Windows.Foundation" && tn.name == "HResult" {
                    return Ok(());
                }
                let def = self.resolve(tn)?;
                match def.category() {
                    TypeCategory::Enum => {
                        diagnose_enum(def, "dependency")?;
                        self.enums.insert(def);
                    }
                    TypeCategory::Interface => {
                        if !self.interface_roots.contains_key(&def) {
                            validate_interface_bases(
                                self.index,
                                self.all_index,
                                self.architecture,
                                def,
                                &mut HashSet::new(),
                            )?;
                            marker_interface(self.index, def)?;
                            self.interface_markers.insert(def);
                        }
                    }
                    TypeCategory::Struct if native_handle_value(def).is_some() => {
                        self.handles.insert(def);
                    }
                    TypeCategory::Struct if def.has_attribute("NativeTypedefAttribute") => {
                        if let Some(underlying) = native_typedef_underlying(def) {
                            signature_types.push(underlying.clone());
                            work.push_back(underlying);
                        } else {
                            return Err(format!(
                                "windows-csharp: dependency native typedef `{}.{}` has an \
                                 unsupported shape",
                                tn.namespace, tn.name
                            ));
                        }
                    }
                    TypeCategory::Struct if self.structs.insert(def) => {
                        diagnose_struct(
                            self.index,
                            Some(self.all_index),
                            self.architecture,
                            def,
                            "dependency",
                        )?;
                        self.enqueue_struct_fields(def, work);
                    }
                    TypeCategory::Struct => {}
                    TypeCategory::Delegate if self.native_callbacks.insert(def) => {
                        if native_callback(self.index, def).is_none() {
                            return Err(format!(
                                "windows-csharp: dependency callback `{}.{}` has an unsupported \
                                 signature or calling convention",
                                tn.namespace, tn.name
                            ));
                        }
                        let invoke = def
                            .methods()
                            .find(|method| method.name() == "Invoke")
                            .unwrap();
                        let signature = invoke.signature(&[]);
                        signature_types.push(signature.return_type.clone());
                        work.push_back(signature.return_type);
                        for param in signature.types {
                            signature_types.push(param.clone());
                            work.push_back(param);
                        }
                    }
                    _ => {
                        return Err(format!(
                            "windows-csharp: dependency value type `{}.{}` has unsupported metadata \
                             kind `{}`",
                            tn.namespace,
                            tn.name,
                            category_name(def)
                        ));
                    }
                }
            }
            Type::ClassName(tn) if tn.generics.is_empty() => {
                let def = self.resolve(tn)?;
                match def.category() {
                    TypeCategory::Class => {
                        if !def.flags().contains(TypeAttributes::WindowsRuntime) {
                            return Err(format!(
                                "windows-csharp: dependency class `{}.{}` is not a WinRT type",
                                tn.namespace, tn.name
                            ));
                        }
                        if !self.class_roots.contains_key(&def) {
                            validate_class_hierarchy(
                                self.index,
                                self.all_index,
                                self.architecture,
                                def,
                            )?;
                            marker_class(self.index, def)?;
                            self.class_markers.insert(def);
                        }
                        for implementation in def.interface_impls() {
                            if implementation.has_attribute("DefaultAttribute") {
                                let interface = implementation.interface(&type_generics(def));
                                if matches!(
                                    &interface,
                                    Type::ClassName(name) if !name.generics.is_empty()
                                ) {
                                    signature_types.push(interface.clone());
                                    work.push_back(interface);
                                }
                                break;
                            }
                        }
                    }
                    TypeCategory::Interface => {
                        if !self.interface_roots.contains_key(&def) {
                            validate_interface_bases(
                                self.index,
                                self.all_index,
                                self.architecture,
                                def,
                                &mut HashSet::new(),
                            )?;
                            marker_interface(self.index, def)?;
                            self.interface_markers.insert(def);
                        }
                    }
                    TypeCategory::Delegate => {
                        if def.flags().contains(TypeAttributes::WindowsRuntime)
                            && self.delegates.insert(def)
                        {
                            diagnose_delegate(
                                self.index,
                                Some(self.all_index),
                                self.architecture,
                                def,
                                "dependency",
                            )?;
                            let invoke = def
                                .methods()
                                .find(|method| method.name() == "Invoke")
                                .unwrap();
                            let signature = invoke.signature(&[]);
                            signature_types.push(signature.return_type.clone());
                            work.push_back(signature.return_type);
                            for param in signature.types {
                                signature_types.push(param.clone());
                                work.push_back(param);
                            }
                        } else if !def.flags().contains(TypeAttributes::WindowsRuntime)
                            && self.native_callbacks.insert(def)
                        {
                            if native_callback(self.index, def).is_none() {
                                return Err(format!(
                                    "windows-csharp: dependency callback `{}.{}` has an unsupported \
                                     signature or calling convention",
                                    tn.namespace, tn.name
                                ));
                            }
                            let invoke = def
                                .methods()
                                .find(|method| method.name() == "Invoke")
                                .unwrap();
                            let signature = invoke.signature(&[]);
                            signature_types.push(signature.return_type.clone());
                            work.push_back(signature.return_type);
                            for param in signature.types {
                                signature_types.push(param.clone());
                                work.push_back(param);
                            }
                        }
                    }
                    _ => {
                        return Err(format!(
                            "windows-csharp: dependency type `{}.{}` cannot be projected",
                            tn.namespace, tn.name
                        ));
                    }
                }
            }
            // A closed generic instantiation (`IVector<Widget>`, `IReference<Widget>`, ...): the
            // instantiation itself needs no marker (it is not a nameable metadata type), but each
            // type argument may still be a dependency.
            Type::ClassName(tn) => {
                if CsType::map(self.index, ty).is_none() {
                    return Err(format!(
                        "windows-csharp: dependency generic `{}` has unsupported type arguments or \
                         shape",
                        type_debug(ty)
                    ));
                }
                for generic in &tn.generics {
                    signature_types.push(generic.clone());
                    work.push_back(generic.clone());
                }
            }
            Type::Array(inner)
            | Type::ArrayFixed(inner, _)
            | Type::RefMut(inner)
            | Type::RefConst(inner)
            | Type::PtrMut(inner, _)
            | Type::PtrConst(inner, _) => {
                if CsType::map(self.index, ty).is_none() {
                    return Err(format!(
                        "windows-csharp: dependency signature contains unsupported type `{}`",
                        type_debug(ty)
                    ));
                }
                let inner = (**inner).clone();
                signature_types.push(inner.clone());
                work.push_back(inner);
            }
            _ => {
                if CsType::map(self.index, ty).is_none() {
                    return Err(format!(
                        "windows-csharp: dependency signature contains unsupported type `{}`",
                        type_debug(ty)
                    ));
                }
            }
        }
        Ok(())
    }

    fn enqueue_struct_fields(&self, def: TypeDef, work: &mut VecDeque<Type>) {
        for field in def.fields() {
            if field.flags().contains(FieldAttributes::Static) {
                continue;
            }
            let ty = field.ty();
            if let Type::ValueName(name) = &ty
                && name.namespace.is_empty()
                && let Some(nested) = self
                    .index
                    .nested(def)
                    .find(|nested| nested.name() == name.name)
            {
                self.enqueue_struct_fields(nested, work);
            } else {
                work.push_back(ty);
            }
        }
    }

    fn resolve(&self, tn: &windows_metadata::TypeName) -> Result<TypeDef<'a>, String> {
        if let Some(def) = self.index.get(&tn.namespace, &tn.name).next() {
            return Ok(def);
        }
        if self.all_index.get(&tn.namespace, &tn.name).next().is_some() {
            return Err(format!(
                "windows-csharp: dependency type `{}.{}` is unavailable on the selected {} \
                 architecture",
                tn.namespace,
                tn.name,
                self.architecture.map_or("target", Architecture::name)
            ));
        }
        Err(format!(
            "windows-csharp: dependency type `{}.{}` was not found in the metadata",
            tn.namespace, tn.name
        ))
    }
}

/// Returns whether any signature type is metadata `Object` (`IInspectable`), used by exact-selection
/// generation to decide whether the shared `Windows.Foundation.IInspectable` owner needs to be
/// emitted. The namespace-filtered pipeline uses [`Builder::uses_object`] instead, which scans a
/// broader set of methods (every in-scope type's own methods, not just selected member
/// signatures); the two are intentionally not unified so exact selection stays scoped to only what
/// was actually selected.
fn object_used(types: &[Type]) -> bool {
    types.iter().any(type_uses_object)
}

fn type_uses_object(ty: &Type) -> bool {
    match ty {
        Type::Object => true,
        Type::ClassName(name) | Type::ValueName(name) => name.generics.iter().any(type_uses_object),
        Type::Array(inner)
        | Type::ArrayFixed(inner, _)
        | Type::RefMut(inner)
        | Type::RefConst(inner)
        | Type::PtrMut(inner, _)
        | Type::PtrConst(inner, _) => type_uses_object(inner),
        _ => false,
    }
}

/// The exact-selection analogue of [`Builder::collect_async`]: scans a flat list of already-
/// collected signature types (rather than re-walking the metadata by namespace) for a supported
/// `IAsyncOperation<...>` instantiation.
fn async_from_types(index: &Index, types: &[Type]) -> Result<Option<AsyncOperation>, String> {
    let mut seen = HashSet::new();
    let mut instantiations = Vec::new();

    for ty in types {
        let Type::ClassName(tn) = ty else { continue };
        if tn.namespace != "Windows.Foundation"
            || tn.name != "IAsyncOperation`1"
            || tn.generics.len() != 1
        {
            continue;
        }
        let value = CsType::map(index, &tn.generics[0]).ok_or_else(|| {
            format!(
                "windows-csharp: required generic `{}` has an unsupported result type",
                type_debug(ty)
            )
        })?;
        if !value.is_unmanaged() && !value.is_object() && !matches!(value, CsType::String) {
            return Err(format!(
                "windows-csharp: required generic `{}` has an unsupported result shape",
                type_debug(ty)
            ));
        }
        if !seen.insert(value.surface()) {
            continue;
        }
        let piid = index
            .get("Windows.Foundation", "IAsyncOperation")
            .next()
            .and_then(guid_attribute)
            .ok_or_else(|| {
                "windows-csharp: required generic `Windows.Foundation.IAsyncOperation<T>` has no \
                 open-generic GuidAttribute"
                    .to_string()
            })?;
        let completed_piid = index
            .get("Windows.Foundation", "AsyncOperationCompletedHandler")
            .next()
            .and_then(guid_attribute)
            .ok_or_else(|| {
                "windows-csharp: required generic \
                 `Windows.Foundation.AsyncOperationCompletedHandler<T>` has no open-generic \
                 GuidAttribute"
                    .to_string()
            })?;
        let iid = guid::generic_iid(index, piid, &tn.generics).ok_or_else(|| {
            format!(
                "windows-csharp: required generic `{}` has unsupported parameterized IID \
                 dependencies",
                type_debug(ty)
            )
        })?;
        let completed_iid =
            guid::generic_iid(index, completed_piid, &tn.generics).ok_or_else(|| {
                format!(
                    "windows-csharp: required completion handler for `{}` has unsupported \
                     parameterized IID dependencies",
                    type_debug(ty)
                )
            })?;
        instantiations.push(AsyncOperationInstantiation {
            element: value,
            iid,
            completed_iid,
        });
    }

    if instantiations.is_empty() {
        Ok(None)
    } else {
        instantiations.sort_by_key(|value| value.element.surface());
        Ok(Some(AsyncOperation { instantiations }))
    }
}

/// The exact-selection analogue of [`Builder::collect_arity1`]: scans a flat list of already-
/// collected signature types for a supported arity-one generic instantiation (`IVector<...>` or its
/// view `IVectorView<...>`).
fn vector_from_types(
    index: &Index,
    types: &[Type],
    meta_name: &str,
    open_name: &str,
) -> Result<Option<Vector>, String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut instantiations: Vec<VectorInstantiation> = Vec::new();

    for ty in types {
        let Type::ClassName(tn) = ty else { continue };
        if tn.namespace != "Windows.Foundation.Collections" || tn.name != meta_name {
            continue;
        }
        if generic_name(index, tn).is_none() {
            return Err(format!(
                "windows-csharp: required generic `{}` has unsupported type arguments or shape",
                type_debug(ty)
            ));
        }
        let element = CsType::map(index, &tn.generics[0]).ok_or_else(|| {
            format!(
                "windows-csharp: required generic `{}` has an unsupported element type",
                type_debug(ty)
            )
        })?;
        if !seen.insert(element.collection_surface()) {
            continue;
        }
        let piid = index
            .get("Windows.Foundation.Collections", open_name)
            .next()
            .and_then(guid_attribute)
            .ok_or_else(|| {
                format!(
                    "windows-csharp: required generic \
                     `Windows.Foundation.Collections.{open_name}<T>` has no open-generic \
                     GuidAttribute"
                )
            })?;
        let iid = guid::generic_iid(index, piid, &tn.generics).ok_or_else(|| {
            format!(
                "windows-csharp: required generic `{}` has unsupported parameterized IID \
                 dependencies",
                type_debug(ty)
            )
        })?;
        instantiations.push(VectorInstantiation { element, iid });
    }

    if instantiations.is_empty() {
        return Ok(None);
    }
    instantiations.sort_by_key(|value| value.element.collection_surface());
    Ok(Some(Vector { instantiations }))
}

/// The exact-selection analogue of [`Builder::collect_arity2`]: scans a flat list of already-
/// collected signature types for a supported `IMap<...>` or its view `IMapView<...>`.
fn map_from_types(
    index: &Index,
    types: &[Type],
    meta_name: &str,
    open_name: &str,
) -> Result<Option<Map>, String> {
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let mut instantiations: Vec<MapInstantiation> = Vec::new();

    for ty in types {
        let Type::ClassName(tn) = ty else { continue };
        if tn.namespace != "Windows.Foundation.Collections" || tn.name != meta_name {
            continue;
        }
        if generic_name(index, tn).is_none() {
            return Err(format!(
                "windows-csharp: required generic `{}` has unsupported type arguments or shape",
                type_debug(ty)
            ));
        }
        let key = CsType::map(index, &tn.generics[0]).ok_or_else(|| {
            format!(
                "windows-csharp: required generic `{}` has an unsupported key type",
                type_debug(ty)
            )
        })?;
        let value = CsType::map(index, &tn.generics[1]).ok_or_else(|| {
            format!(
                "windows-csharp: required generic `{}` has an unsupported value type",
                type_debug(ty)
            )
        })?;
        if !seen.insert((key.surface(), value.surface())) {
            continue;
        }
        let open = |name| {
            index
                .get("Windows.Foundation.Collections", name)
                .next()
                .and_then(guid_attribute)
                .ok_or_else(|| {
                    format!(
                        "windows-csharp: required generic \
                         `Windows.Foundation.Collections.{name}` has no open-generic GuidAttribute"
                    )
                })
        };
        let piid = open(open_name)?;
        let iterable_piid = open("IIterable")?;
        let iterator_piid = open("IIterator")?;
        let iid = guid::generic_iid(index, piid, &tn.generics).ok_or_else(|| {
            format!(
                "windows-csharp: required generic `{}` has unsupported parameterized IID \
                 dependencies",
                type_debug(ty)
            )
        })?;
        let pair = Type::ClassName(windows_metadata::TypeName {
            namespace: "Windows.Foundation.Collections".to_string(),
            name: "IKeyValuePair`2".to_string(),
            generics: tn.generics.clone(),
        });
        let iterable_iid = guid::generic_iid(index, iterable_piid, std::slice::from_ref(&pair))
            .ok_or_else(|| {
                format!(
                    "windows-csharp: required iterable for `{}` has unsupported parameterized IID \
                     dependencies",
                    type_debug(ty)
                )
            })?;
        let iterator_iid = guid::generic_iid(index, iterator_piid, std::slice::from_ref(&pair))
            .ok_or_else(|| {
                format!(
                    "windows-csharp: required iterator for `{}` has unsupported parameterized IID \
                     dependencies",
                    type_debug(ty)
                )
            })?;
        instantiations.push(MapInstantiation {
            key,
            value,
            iid,
            iterable_iid,
            iterator_iid,
        });
    }

    if instantiations.is_empty() {
        return Ok(None);
    }
    instantiations.sort_by(|a, b| {
        (a.key.surface(), a.value.surface()).cmp(&(b.key.surface(), b.value.surface()))
    });
    Ok(Some(Map { instantiations }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_metadata::ParamAttributes;

    fn parameter_index(rows: &[(&str, u16, ParamAttributes)]) -> Index {
        let mut file = windows_metadata::writer::File::new("test");
        file.TypeDef(
            "Test",
            "Apis",
            windows_metadata::writer::TypeDefOrRef::default(),
            TypeAttributes::Public,
        );
        let signature = windows_metadata::Signature {
            return_type: Type::Void,
            types: vec![Type::I32, Type::U32, Type::I64],
            ..Default::default()
        };
        let method = file.MethodDef(
            "Method",
            &signature,
            MethodAttributes::Public | MethodAttributes::Static | MethodAttributes::PInvokeImpl,
            Default::default(),
        );
        for (name, sequence, flags) in rows {
            file.Param(name, *sequence, *flags);
        }
        file.ImplMap(
            method,
            windows_metadata::PInvokeAttributes::CallConvPlatformapi,
            "Method",
            "test.dll",
        );
        Index::new(vec![File::new(file.into_stream()).unwrap()])
    }

    #[test]
    fn sparse_out_of_order_params_follow_sequence() {
        let index = parameter_index(&[
            ("third", 3, ParamAttributes::Out | ParamAttributes::Optional),
            ("return", 0, ParamAttributes::Out),
            ("first", 1, ParamAttributes::In | ParamAttributes::Out),
        ]);
        let method = index.expect("Test", "Apis").methods().next().unwrap();
        let signature = method.signature(&[]);
        let params = read_params(
            &index,
            method,
            &signature,
            signature.types.len(),
            |_, ty| CsType::map(&index, ty),
        )
        .unwrap();

        assert_eq!(
            params
                .iter()
                .map(|param| param.name.as_str())
                .collect::<Vec<_>>(),
            ["first", "p1", "third"]
        );
        assert!(params.iter().map(|param| param.direction).eq([
            Direction::InputOutput,
            Direction::Input,
            Direction::Output
        ]));
        assert_eq!(
            params
                .iter()
                .map(|param| param.optional)
                .collect::<Vec<_>>(),
            [false, false, true]
        );
    }

    #[test]
    fn unspecified_direction_uses_csharp_input_fallback() {
        let index = parameter_index(&[("first", 1, ParamAttributes::default())]);
        let method = index.expect("Test", "Apis").methods().next().unwrap();
        let signature = method.signature(&[]);
        let params = read_params(&index, method, &signature, 1, |_, ty| {
            CsType::map(&index, ty)
        })
        .unwrap();

        assert!(matches!(params[0].direction, Direction::Input));
    }

    #[test]
    fn malformed_param_sequence_has_exact_selection_diagnostic() {
        let index = parameter_index(&[
            ("first", 1, ParamAttributes::In),
            ("duplicate", 1, ParamAttributes::Out),
        ]);
        let method = index.expect("Test", "Apis").methods().next().unwrap();
        let diagnostic = diagnose_function(&index, None, None, "Test.Method", method);

        assert_eq!(
            diagnostic,
            "windows-csharp: selected function `Test.Method` has invalid parameter metadata: \
             duplicate Param.Sequence 1"
        );
    }

    #[test]
    fn real_win32_istream_shapes() {
        let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../bindgen/default/Windows.Win32.winmd");
        let files = read_files(&[input.to_string_lossy().into_owned()]).unwrap();
        let index = Index::new(files);

        let (namespace, method) = index
            .iter_items()
            .find_map(|(namespace, _, item)| match item {
                Item::Fn(method) if method.name() == "CreateStreamOnHGlobal" => {
                    Some((namespace, method))
                }
                _ => None,
            })
            .unwrap();
        let signature = method.signature(&[]);
        let function = read_function(&index, namespace, method).unwrap_or_else(|| {
            panic!(
                "unsupported real signature: return={:?}, params={:?}",
                signature.return_type, signature.types
            )
        });
        assert_eq!(function.namespace, "Windows.Win32");
        assert_eq!(function.library, "ole32.dll");
        assert!(function.hresult);
        assert!(matches!(function.ret, Some(CsType::HResult)));
        assert!(matches!(
            function.params.as_slice(),
            [
                Param {
                    ty: CsType::Handle { name: hglobal },
                    ..
                },
                Param {
                    ty: CsType::Win32Bool,
                    ..
                },
                Param {
                    ty: CsType::ComOut { name: stream },
                    ..
                }
            ] if hglobal == "Windows.Win32.HANDLE" && stream == "Windows.Win32.IStream"
        ));

        let def = index.get("Windows.Win32", "IStream").next().unwrap();
        let stream = read_interface(&index, def).unwrap();
        assert!(stream.native_own_members.is_none());
        for (name, slot) in [("Read", 3), ("Write", 4), ("Seek", 5), ("SetSize", 6)] {
            assert!(stream.members.iter().any(|member| {
                matches!(
                    member,
                    Member::Method {
                        name: actual,
                        slot: actual_slot,
                        abi: MethodAbi::Direct,
                        ..
                    } if actual == name && *actual_slot == slot
                )
            }));
        }

        let def = index
            .get("Windows.Win32", "ISequentialStream")
            .next()
            .unwrap();
        let sequential = read_interface(&index, def).unwrap();
        assert_eq!(sequential.native_own_members.as_ref().unwrap().len(), 2);
        assert!(sequential.native_base.is_none());
        let read = sequential
            .native_own_members
            .as_ref()
            .unwrap()
            .iter()
            .find_map(|member| match member {
                Member::Method { name, params, .. } if name == "Read" => Some(params),
                _ => None,
            })
            .unwrap();
        assert!(matches!(
            read[0].buffer_length(),
            Some(BufferLength::Bytes(1))
        ));
        assert!(!read[0].optional);
        assert!(matches!(read[0].direction, Direction::Output));
        assert!(read[0].buffer_target().is_some());
        assert!(read[2].optional);
        assert!(matches!(
            param_roles(read).as_slice(),
            [
                ParamRole::Buffer {
                    element: BufferElement::Value(CsType::Scalar("byte")) | BufferElement::ByteVoid,
                    count: 1
                },
                ParamRole::BufferCount { buffer: 0 },
                ParamRole::Value
            ]
        ));
    }

    #[test]
    fn real_win32_callback_shape() {
        let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../bindgen/default/Windows.Win32.winmd");
        let files = read_files(&[input.to_string_lossy().into_owned()]).unwrap();
        let index = Index::new(files);

        let (namespace, method) = index
            .iter_items()
            .find_map(|(namespace, _, item)| match item {
                Item::Fn(method) if method.name() == "EnumWindows" => Some((namespace, method)),
                _ => None,
            })
            .unwrap();
        let function = read_function(&index, namespace, method).unwrap();
        assert!(matches!(
            function.params.as_slice(),
            [
                Param {
                    ty: CsType::Callback {
                        params,
                        return_type,
                        convention: CallingConvention::Stdcall,
                    },
                    ..
                },
                Param {
                    ty: CsType::Scalar("nint"),
                    ..
                }
            ] if matches!(
                params.as_slice(),
                [
                    CsType::Handle { name },
                    CsType::Scalar("nint")
                ] if name == "Windows.Win32.HWND"
            ) && matches!(return_type.as_ref(), CsType::Win32Bool)
        ));
    }

    #[test]
    fn real_win32_architecture_layouts() {
        let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../bindgen/default/Windows.Win32.winmd");
        let input = input.to_string_lossy().into_owned();

        let all = Index::new(read_files(std::slice::from_ref(&input)).unwrap());
        assert_eq!(all.get("Windows.Win32", "PACKAGE_ID").count(), 2);

        for (architecture, packing_size) in [(1, None), (2, Some(4)), (4, Some(4))] {
            let index = Index::new_for_architecture(
                read_files(std::slice::from_ref(&input)).unwrap(),
                architecture,
            );
            let mut defs = index.get("Windows.Win32", "PACKAGE_ID");
            let def = defs.next().unwrap();
            assert!(defs.next().is_none());
            let value = read_struct(&index, def).unwrap();
            assert_eq!(value.packing_size, packing_size);
            assert_eq!(value.fields.len(), 7);
        }

        let index = Index::new_for_architecture(read_files(&[input]).unwrap(), 2);
        let aligned = index.get("Windows.Win32", "M128A").next().unwrap();
        assert!(read_struct(&index, aligned).is_none());
        assert!(CsType::map(&index, &Type::value_named("Windows.Win32", "M128A")).is_none());
    }

    /// Validates output and input/output direction analysis against real Win32 exports.
    #[test]
    fn real_win32_direction_analysis() {
        let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../bindgen/default/Windows.Win32.winmd");
        let files = read_files(&[input.to_string_lossy().into_owned()]).unwrap();
        let index = Index::new(files);

        let find = |name: &str| {
            index
                .iter_items()
                .find_map(|(namespace, _, item)| match item {
                    Item::Fn(method) if method.name() == name => Some((namespace, method)),
                    _ => None,
                })
                .unwrap()
        };

        let (namespace, method) = find("GetWindowRect");
        let function = read_function(&index, namespace, method).unwrap();
        assert!(matches!(function.ret, Some(CsType::Win32Bool)));
        assert!(!function.hresult);
        let lp_rect = &function.params[1];
        assert_eq!(lp_rect.name, "lpRect");
        assert!(matches!(lp_rect.direction, Direction::Output));
        assert!(!lp_rect.optional);
        assert!(matches!(
            lp_rect.scalar_pointer_target(),
            Some(CsType::Struct { name, .. }) if name == "Windows.Win32.RECT"
        ));

        let (namespace, method) = find("QueryPerformanceCounter");
        let function = read_function(&index, namespace, method).unwrap();
        assert!(matches!(function.ret, Some(CsType::Win32Bool)));
        let lp_count = &function.params[0];
        assert_eq!(lp_count.name, "lpPerformanceCount");
        assert!(matches!(lp_count.direction, Direction::Output));
        assert!(matches!(
            lp_count.scalar_pointer_target(),
            Some(CsType::Scalar("long"))
        ));

        let (namespace, method) = find("CryptBinaryToStringW");
        let function = read_function(&index, namespace, method).unwrap();
        let string_length = &function.params[4];
        assert_eq!(string_length.name, "pcchString");
        assert!(matches!(string_length.direction, Direction::InputOutput));
        assert!(matches!(
            string_length.scalar_pointer_target(),
            Some(CsType::Scalar("uint"))
        ));

        let (namespace, method) = find("CryptProtectMemory");
        let function = read_function(&index, namespace, method).unwrap();
        let data = &function.params[0];
        assert_eq!(data.name, "pDataIn");
        assert!(matches!(data.direction, Direction::InputOutput));
        assert!(data.scalar_pointer_target().is_none());
    }

    #[test]
    fn real_win32_buffer_analysis() {
        let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../bindgen/default/Windows.Win32.winmd");
        let files = read_files(&[input.to_string_lossy().into_owned()]).unwrap();
        let index = Index::new(files);

        let (namespace, method) = index
            .iter_items()
            .find_map(|(namespace, _, item)| match item {
                Item::Fn(method) if method.name() == "Polyline" => Some((namespace, method)),
                _ => None,
            })
            .unwrap();
        let function = read_function(&index, namespace, method).unwrap();
        let points = &function.params[1];
        assert_eq!(points.name, "apt");
        assert!(matches!(
            points.buffer_length(),
            Some(BufferLength::Elements(2))
        ));
        assert!(matches!(
            points.buffer_target(),
            Some((BufferElement::Value(CsType::Struct { name, .. }), 2))
                if name == "Windows.Win32.POINT"
        ));
        assert!(function.params[2].is_buffer_count());

        let (namespace, method) = index
            .iter_items()
            .find_map(|(namespace, _, item)| match item {
                Item::Fn(method) if method.name() == "GetTempPathW" => Some((namespace, method)),
                _ => None,
            })
            .unwrap();
        let function = read_function(&index, namespace, method).unwrap();
        let buffer = &function.params[1];
        assert_eq!(buffer.name, "lpBuffer");
        assert!(matches!(
            buffer.buffer_target(),
            Some((BufferElement::Utf16, 0))
        ));
        assert!(function.params[0].is_buffer_count());
    }

    /// Validates the handle rule (`native_handle_value`) against real `Windows.Win32.winmd`
    /// shapes: `GetWindowRect`'s first parameter stays a named `HWND` handle rather than
    /// collapsing to a bare pointer/`nint`, `HANDLE` itself is the opaque-`void*` shape the rule
    /// matches, and a scalar identifier alias (`COLORREF`) and a pointer-to-named-type alias
    /// (`PWSTR`) both correctly fail the rule and keep their existing collapsed representation -
    /// confirming the handle/non-handle boundary the projection draws (see `native_handle_value`'s
    /// docs and the cross-crate review in `docs/crates/windows-csharp.md`).
    #[test]
    fn real_win32_handle_shapes() {
        let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../bindgen/default/Windows.Win32.winmd");
        let files = read_files(&[input.to_string_lossy().into_owned()]).unwrap();
        let index = Index::new(files);

        let find_ty = |name: &str| index.get("Windows.Win32", name).next().unwrap();

        // `GetWindowRect(HWND hWnd, LPRECT lpRect)`: the first parameter is a named handle passed
        // by value, not a pointer - `CsType::map` must resolve it to `CsType::Handle`, not collapse
        // it to `nint`/`void*` the way a scalar or pointer-alias typedef would.
        let (namespace, method) = index
            .iter_items()
            .find_map(|(namespace, _, item)| match item {
                Item::Fn(method) if method.name() == "GetWindowRect" => Some((namespace, method)),
                _ => None,
            })
            .unwrap();
        let function = read_function(&index, namespace, method).unwrap();
        let h_wnd = &function.params[0];
        assert_eq!(h_wnd.name, "hWnd");
        assert!(matches!(
            &h_wnd.ty,
            CsType::Handle { name } if name == "Windows.Win32.HWND"
        ));

        let (namespace, method) = index
            .iter_items()
            .find_map(|(namespace, _, item)| match item {
                Item::Fn(method) if method.name() == "GetModuleHandleW" => {
                    Some((namespace, method))
                }
                _ => None,
            })
            .unwrap();
        let function = read_function(&index, namespace, method).unwrap();
        let module_name = &function.params[0];
        assert_eq!(module_name.name, "lpModuleName");
        assert!(module_name.optional);
        assert!(module_name.is_utf16_string());

        // `HANDLE` itself: a `NativeTypedefAttribute` struct whose sole field (`Value`) is exactly
        // one indirection to `void` - the structural shape every genuine Win32 handle shares, and
        // the shape `native_handle_value` requires.
        let handle_def = find_ty("HANDLE");
        assert!(matches!(
            native_handle_value(handle_def),
            Some(Type::PtrMut(inner, 1)) if matches!(*inner, Type::Void)
        ));
        assert!(matches!(
            CsType::map(&index, &Type::ValueName(windows_metadata::TypeName {
                namespace: "Windows.Win32".to_string(),
                name: "HANDLE".to_string(),
                generics: vec![],
            })),
            Some(CsType::Handle { name }) if name == "Windows.Win32.HANDLE"
        ));

        // `HGLOBAL` is a native typedef whose field is `HANDLE`, so it collapses to the target
        // wrapper. Exact-selection closure must still discover and emit that target definition.
        let hglobal = Type::ValueName(windows_metadata::TypeName {
            namespace: "Windows.Win32".to_string(),
            name: "HGLOBAL".to_string(),
            generics: vec![],
        });
        assert!(matches!(
            CsType::map(&index, &hglobal),
            Some(CsType::Handle { name }) if name == "Windows.Win32.HANDLE"
        ));
        let class_roots = HashMap::new();
        let interface_roots = HashMap::new();
        let mut closure = Closure {
            index: &index,
            all_index: &index,
            architecture: None,
            class_roots: &class_roots,
            interface_roots: &interface_roots,
            class_markers: HashSet::new(),
            interface_markers: HashSet::new(),
            delegates: HashSet::new(),
            native_callbacks: HashSet::new(),
            enums: HashSet::new(),
            structs: HashSet::new(),
            handles: HashSet::new(),
        };
        let mut work = VecDeque::from([hglobal]);
        let mut signature_types = Vec::new();
        while let Some(ty) = work.pop_front() {
            closure
                .discover(&ty, &mut work, &mut signature_types)
                .unwrap();
        }
        assert!(closure.handles.contains(&handle_def));

        // A scalar identifier alias (`COLORREF: Value: u32`) is not a handle by the structural
        // rule - it stays collapsed to its underlying scalar, matching the existing DWORD-style
        // typedef behavior rather than becoming a distinct wrapper type.
        let colorref_def = find_ty("COLORREF");
        assert!(native_handle_value(colorref_def).is_none());
        assert!(matches!(
            CsType::map(
                &index,
                &Type::ValueName(windows_metadata::TypeName {
                    namespace: "Windows.Win32".to_string(),
                    name: "COLORREF".to_string(),
                    generics: vec![],
                })
            ),
            Some(CsType::Scalar("uint"))
        ));

        // A pointer-to-named-type alias (`PWSTR: Value: *mut u16`) is not a handle either - the
        // pointee is a concrete scalar, not `void`, so it stays the raw pointer shape.
        let pwstr_def = find_ty("PWSTR");
        assert!(native_handle_value(pwstr_def).is_none());
        assert!(matches!(
            CsType::map(
                &index,
                &Type::ValueName(windows_metadata::TypeName {
                    namespace: "Windows.Win32".to_string(),
                    name: "PWSTR".to_string(),
                    generics: vec![],
                })
            ),
            Some(CsType::Pointer {
                mutable: true,
                depth: 1,
                ..
            })
        ));
    }
}
