use crate::model::*;
use std::collections::HashSet;

/// A minimal indenting writer for emitting C# source.
struct Writer {
    buf: String,
    indent: usize,
}

/// Where a projected member reaches its COM pointer from. This controls both the member's C#
/// visibility and how the `self` pointer is acquired, so the same marshalling code (`write_method`,
/// `write_property`, `write_event`) serves instance members, the callback-confined `Borrowed` view,
/// and static members reached through the class activation factory.
#[derive(Clone, Copy)]
enum Receiver<'a> {
    /// An instance member on an owned object: `public`, `self` leased from the object.
    Owned,
    /// A member inside the callback-confined `Borrowed` view: `public`, `self` is the borrowed
    /// pointer with a disposed-object guard.
    Borrowed,
    /// A static member reached through the class activation factory: `public static`, `self` is a
    /// factory-interface pointer from a `FactoryLease`.
    Factory(&'a FactoryAccess),
}

/// The class-level state a static member uses to acquire its factory-interface pointer: the shared
/// activation module cache field, this static interface's own agile factory cache field, the
/// runtime class id, and the static interface IID expression.
struct FactoryAccess {
    module: String,
    cache: String,
    class_id: String,
    iid: String,
}

impl Receiver<'_> {
    fn is_borrowed(&self) -> bool {
        matches!(self, Receiver::Borrowed)
    }

    /// The C# visibility/binding prefix: `public` for instance members, `public static` for static
    /// members reached through the factory.
    fn vis(&self) -> &'static str {
        match self {
            Receiver::Factory(_) => "public static",
            _ => "public",
        }
    }
}

/// Emits the `using` declaration that owns the source pointer for the duration of a call: a
/// `ComLease` over the owned object, or a `FactoryLease` over the activation factory. The borrowed
/// view owns nothing (its pointer is confined to the callback), so it emits nothing here.
fn write_source_lease(w: &mut Writer, receiver: Receiver, name: &str) {
    match receiver {
        Receiver::Owned => w.line(&format!("using WindowsCsharp.ComLease {name} = Acquire();")),
        Receiver::Factory(access) => w.line(&format!(
            "using WindowsCsharp.FactoryLease {name} = WindowsCsharp.WinRT.GetActivationFactory(ref {}, ref {}, \"{}\", {});",
            access.module, access.cache, access.class_id, access.iid
        )),
        Receiver::Borrowed => {}
    }
}

/// Binds `self` from the source acquired by [`write_source_lease`]: the borrowed pointer (with a
/// disposed-object guard) or the lease handle.
fn write_source_self(w: &mut Writer, receiver: Receiver, lease: &str) {
    match receiver {
        Receiver::Borrowed => {
            w.line("nint self = _this;");
            w.open("if (self == 0)");
            w.line("throw new ObjectDisposedException(\"borrowed COM interface\");");
            w.close();
        }
        _ => w.line(&format!("nint self = {lease}.Handle;")),
    }
}

fn write_forwarders(w: &mut Writer, forwarders: &[Forwarder], receiver: Receiver) {
    for (index, forwarder) in forwarders.iter().enumerate() {
        for member in &forwarder.members {
            w.line("");
            match member {
                Member::Property {
                    name,
                    ty,
                    get_slot,
                    put_slot,
                } => write_property(
                    w,
                    name,
                    ty,
                    *get_slot,
                    *put_slot,
                    receiver,
                    Some(&format!("s_forward{index}")),
                ),
                Member::Method {
                    name,
                    params,
                    ret,
                    slot,
                    abi,
                } => write_method(
                    w,
                    name,
                    params,
                    ret.as_ref(),
                    *slot,
                    *abi,
                    receiver,
                    Some(&format!("s_forward{index}")),
                ),
                Member::Event { .. } => unreachable!(),
            }
        }
    }
}

impl Writer {
    fn new() -> Self {
        Self {
            buf: String::new(),
            indent: 0,
        }
    }

    /// Writes one line at the current indent. An empty string emits a blank line with no indent.
    fn line(&mut self, text: &str) {
        if !text.is_empty() {
            for _ in 0..self.indent {
                self.buf.push_str("    ");
            }
            self.buf.push_str(text);
        }
        self.buf.push('\n');
    }

    fn open(&mut self, text: &str) {
        self.line(text);
        self.line("{");
        self.indent += 1;
    }

    fn close(&mut self) {
        self.indent -= 1;
        self.line("}");
    }

    fn close_with(&mut self, suffix: &str) {
        self.indent -= 1;
        self.line(&format!("}}{suffix}"));
    }

    fn finish(mut self) -> String {
        while self.buf.ends_with('\n') {
            self.buf.pop();
        }
        if !self.buf.is_empty() {
            self.buf.push('\n');
        }
        self.buf
    }
}

/// The `IUnknown` vtable slots shared by every projected COM interface. Default
/// (`IInspectable`-derived) interface members start at slot 6, after the three `IUnknown` and three
/// `IInspectable` slots.
const QUERY_INTERFACE_SLOT: usize = 0;
const ADD_REF_SLOT: usize = 1;
const RELEASE_SLOT: usize = 2;
// The scale harness finds a .NET JIT cliff between 251 and 257 closed map pairs.
const GENERIC_SPECIALIZATION_THRESHOLD: usize = 256;

/// Emits the full C# source for the given classes.
///
/// When `full` is set, the output is a standalone file: a header (usings and the module attribute),
/// the projected types, and the shared runtime support. When clear, only the projected namespace
/// blocks are emitted - a fragment suitable for golden tests or for composing several inputs into
/// one compilation unit (see [`support`]).
#[allow(clippy::too_many_arguments)]
pub fn write(
    classes: &[Class],
    interfaces: &[Interface],
    enums: &[Enum],
    handles: &[Handle],
    structs: &[Struct],
    delegates: &[Delegate],
    functions: &[Function],
    constants: &[ApiConstant],
    collections: &Collections,
    full: bool,
    raw: bool,
) -> String {
    let mut w = Writer::new();

    if full {
        write_header(&mut w);
    }

    write_projection(
        &mut w,
        classes,
        interfaces,
        enums,
        handles,
        structs,
        delegates,
        functions,
        constants,
        collections,
    );

    if full {
        write_support(&mut w, raw, collections.async_operation.is_some());
    }

    w.finish()
}

/// Emits just the shared runtime support (the `WindowsCsharp` namespace), including the header so
/// the result is a standalone compilation unit. Used to compile several projection fragments
/// together against a single copy of the support.
pub fn support(raw: bool) -> String {
    let mut w = Writer::new();
    write_header(&mut w);
    write_support(&mut w, raw, true);
    w.finish()
}

fn write_header(w: &mut Writer) {
    w.line("// Generated by windows-csharp. Do not edit.");
    w.line("");
    w.line("using System;");
    w.line("using System.Runtime.CompilerServices;");
    w.line("using System.Runtime.InteropServices;");
    w.line("using System.Threading;");
    w.line("");
    w.line("[module: SkipLocalsInit]");
    w.line("");
}

#[allow(clippy::too_many_arguments)]
fn write_projection(
    w: &mut Writer,
    classes: &[Class],
    interfaces: &[Interface],
    enums: &[Enum],
    handles: &[Handle],
    structs: &[Struct],
    delegates: &[Delegate],
    functions: &[Function],
    constants: &[ApiConstant],
    collections: &Collections,
) {
    let projected_objects: HashSet<String> = classes
        .iter()
        .map(Class::class_id)
        .chain(
            interfaces
                .iter()
                .map(|item| format!("{}.{}", item.namespace, item.name)),
        )
        .collect();
    let borrowable_objects: HashSet<String> = classes
        .iter()
        .filter(|item| {
            !item.members.is_empty()
                || item
                    .forwarders
                    .iter()
                    .any(|forwarder| !forwarder.members.is_empty())
        })
        .map(Class::class_id)
        .chain(
            interfaces
                .iter()
                .filter(|item| !item.members.is_empty())
                .map(|item| format!("{}.{}", item.namespace, item.name)),
        )
        .collect();
    let generated_com_candidates: HashSet<String> = interfaces
        .iter()
        .filter(|item| item.native_own_members.is_some())
        .map(|item| format!("{}.{}", item.namespace, item.name))
        .collect();
    let mut generated_com_objects = HashSet::new();
    loop {
        let before = generated_com_objects.len();
        for item in interfaces {
            let name = format!("{}.{}", item.namespace, item.name);
            if generated_com_candidates.contains(&name)
                && item
                    .native_base
                    .as_ref()
                    .is_none_or(|base| generated_com_objects.contains(base))
            {
                generated_com_objects.insert(name);
            }
        }
        if generated_com_objects.len() == before {
            break;
        }
    }
    let vector = collections.vector.as_ref();
    let map = collections.map.as_ref();
    let vector_view = collections.vector_view.as_ref();
    let map_view = collections.map_view.as_ref();
    let async_operation = collections.async_operation.as_ref();

    // Group classes, interfaces, enums, structs, and delegates by namespace so each maps to a C#
    // namespace block. A projected generic collection or view (`IVector<T>`, `IMap<K,V>`,
    // `IVectorView<T>`, `IMapView<K,V>`), if any in-scope member uses one, is emitted into its own
    // `Windows.Foundation.Collections` block.
    let mut namespaces: Vec<&str> = classes
        .iter()
        .map(|c| c.namespace.as_str())
        .chain(interfaces.iter().map(|i| i.namespace.as_str()))
        .chain(enums.iter().map(|e| e.namespace.as_str()))
        .chain(handles.iter().map(|h| h.namespace.as_str()))
        .chain(structs.iter().map(|s| s.namespace.as_str()))
        .chain(delegates.iter().map(|d| d.namespace.as_str()))
        .chain(functions.iter().map(|f| f.namespace.as_str()))
        .chain(constants.iter().map(|c| c.namespace.as_str()))
        .chain(
            (collections.inspectable || async_operation.is_some()).then_some("Windows.Foundation"),
        )
        .chain(vector.map(|_| "Windows.Foundation.Collections"))
        .chain(map.map(|_| "Windows.Foundation.Collections"))
        .chain(vector_view.map(|_| "Windows.Foundation.Collections"))
        .chain(map_view.map(|_| "Windows.Foundation.Collections"))
        .collect();
    namespaces.sort_unstable();
    namespaces.dedup();

    for namespace in namespaces {
        w.open(&format!("namespace {namespace}"));
        let mut first = true;

        if namespace == "Windows.Foundation.Collections" {
            if let Some(vector) = vector {
                first = false;
                write_vector(w, vector, "IVector", 16, true);
            }
            if let Some(vector_view) = vector_view {
                if !first {
                    w.line("");
                }
                first = false;
                write_vector(w, vector_view, "IVectorView", 9, false);
            }
            if let Some(map) = map {
                if !first {
                    w.line("");
                }
                first = false;
                write_map(w, map, "IMap", true);
            }
            if let Some(map_view) = map_view {
                if !first {
                    w.line("");
                }
                first = false;
                write_map(w, map_view, "IMapView", false);
            }
        }

        if namespace == "Windows.Foundation" && collections.inspectable {
            first = false;
            write_inspectable(w);
        }
        if namespace == "Windows.Foundation" {
            if let Some(async_operation) = async_operation {
                if !first {
                    w.line("");
                }
                first = false;
                write_async_operation(w, async_operation);
            }
        }

        for e in enums.iter().filter(|e| e.namespace == namespace) {
            if !first {
                w.line("");
            }
            first = false;
            write_enum(w, e);
        }

        for h in handles.iter().filter(|h| h.namespace == namespace) {
            if !first {
                w.line("");
            }
            first = false;
            write_handle(w, h);
        }

        for s in structs.iter().filter(|s| s.namespace == namespace) {
            if !first {
                w.line("");
            }
            first = false;
            write_struct(w, s);
        }

        for d in delegates.iter().filter(|d| d.namespace == namespace) {
            if !first {
                w.line("");
            }
            first = false;
            write_delegate(w, d);
        }

        let namespace_functions: Vec<_> = functions
            .iter()
            .filter(|function| function.namespace == namespace)
            .collect();
        let namespace_constants: Vec<_> = constants
            .iter()
            .filter(|constant| constant.namespace == namespace)
            .collect();
        if !namespace_functions.is_empty() || !namespace_constants.is_empty() {
            if !first {
                w.line("");
            }
            first = false;
            write_apis(w, &namespace_functions, &namespace_constants);
        }

        for class in classes.iter().filter(|c| c.namespace == namespace) {
            if !first {
                w.line("");
            }
            first = false;
            write_class(w, class, &projected_objects, &borrowable_objects);
        }

        for interface in interfaces.iter().filter(|i| i.namespace == namespace) {
            if !first {
                w.line("");
            }
            first = false;
            write_interface(
                w,
                interface,
                &projected_objects,
                &borrowable_objects,
                &generated_com_objects,
            );
        }

        w.close();
        w.line("");
    }
}

fn write_apis(w: &mut Writer, functions: &[&Function], constants: &[&ApiConstant]) {
    w.open("public static unsafe partial class Apis");
    let mut first = true;
    for constant in constants {
        if !first {
            w.line("");
        }
        first = false;
        w.line(&format!(
            "public const {} {} = {};",
            constant.ty.surface(),
            constant.name,
            constant.value
        ));
    }
    for function in functions {
        if !first {
            w.line("");
        }
        first = false;
        let roles = param_roles(&function.params);
        let has_projection = roles.iter().any(|role| !matches!(role, ParamRole::Value));
        let wrapped = function.hresult
            || matches!(function.ret, Some(CsType::Win32Bool))
            || function.params.iter().any(|param| {
                matches!(param.ty, CsType::Win32Bool | CsType::ComOut { .. })
                    || param.ty.owned_struct_abi().is_some()
            })
            || function
                .ret
                .as_ref()
                .is_some_and(|ret| ret.owned_struct_abi().is_some())
            || has_projection;
        write_import(w, function, wrapped);
        if wrapped {
            let public_params = function
                .params
                .iter()
                .enumerate()
                .filter(|(i, param)| {
                    !matches!(param.ty, CsType::ComOut { .. })
                        && !matches!(roles[*i], ParamRole::BufferCount { .. })
                })
                .map(|(i, param)| match roles[i] {
                    ParamRole::ScalarPointer { target, is_ref } => {
                        let keyword = if is_ref { "ref" } else { "out" };
                        format!("{keyword} {} {}", target.surface(), param.name)
                    }
                    ParamRole::Utf16String => {
                        let nullable = if param.optional { "?" } else { "" };
                        format!("string{nullable} {}", param.name)
                    }
                    ParamRole::Buffer { element, .. } => {
                        let span = if matches!(param.direction, Direction::Input) {
                            "ReadOnlySpan"
                        } else {
                            "Span"
                        };
                        format!("{span}<{}> {}", element.surface(), param.name)
                    }
                    ParamRole::Value => format!("{} {}", param.ty.surface(), param.name),
                    ParamRole::BufferCount { .. } => unreachable!(),
                })
                .collect::<Vec<_>>()
                .join(", ");
            let out_object = function.params.iter().find_map(|param| match &param.ty {
                CsType::ComOut { name: object } => Some((&param.name, object)),
                _ => None,
            });
            let public_ret = out_object
                .map(|(_, object)| object.clone())
                .or_else(|| {
                    (!function.hresult)
                        .then(|| function.ret.as_ref().map(CsType::surface))
                        .flatten()
                })
                .unwrap_or_else(|| "void".to_string());
            w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
            w.open(&format!(
                "public static {public_ret} {}({public_params})",
                function.name
            ));
            if let Some((name, _)) = out_object {
                w.line(&format!("nint {name} = 0;"));
            }
            // One ABI-typed local per sugared pointer parameter, initialized from the surface
            // value for a `ref` (`InputOutput`) parameter. Taking a local variable's address never
            // needs a `fixed` statement (it is never a moveable GC reference), so the private
            // import keeps its raw pointer parameter while the public surface stays a plain
            // `out`/`ref` value with no pinning or extra allocation.
            for (i, param) in function.params.iter().enumerate() {
                if let ParamRole::ScalarPointer { target, is_ref } = roles[i] {
                    if is_ref {
                        w.line(&format!(
                            "{} _abi{i} = {};",
                            target.abi_in(),
                            target.surface_to_abi(&param.name)
                        ));
                    } else {
                        w.line(&format!("{} _abi{i};", target.abi_in()));
                    }
                }
            }
            let owned_struct_params: Vec<_> = function
                .params
                .iter()
                .enumerate()
                .filter(|(i, param)| {
                    matches!(roles[*i], ParamRole::Value) && param.ty.owned_struct_abi().is_some()
                })
                .collect();
            for (i, param) in &owned_struct_params {
                w.line(&format!("{} _owned{i} = default;", param.ty.abi_in()));
            }
            if !owned_struct_params.is_empty() {
                w.open("try");
                for (i, param) in &owned_struct_params {
                    w.line(&format!(
                        "_owned{i} = {};",
                        param.ty.surface_to_owned_abi(&param.name)
                    ));
                }
            }
            for (i, param) in function.params.iter().enumerate() {
                match roles[i] {
                    ParamRole::Utf16String => {
                        w.open(&format!("fixed (char* _abi{i} = {})", param.name));
                    }
                    ParamRole::Buffer { element, .. } => {
                        w.open(&format!(
                            "fixed ({}* _abi{i} = {})",
                            element.surface(),
                            param.name
                        ));
                    }
                    _ => {}
                }
            }
            let args = function
                .params
                .iter()
                .enumerate()
                .map(|(i, param)| match (&param.ty, roles[i]) {
                    (CsType::ComOut { .. }, _) => format!("&{}", param.name),
                    (_, ParamRole::ScalarPointer { .. }) => format!("&_abi{i}"),
                    (_, ParamRole::Utf16String) => format!("(ushort*)_abi{i}"),
                    (_, ParamRole::Buffer { element, .. }) => {
                        format!("({}*)_abi{i}", element.abi())
                    }
                    (_, ParamRole::BufferCount { buffer }) => format!(
                        "checked(({}){}.Length)",
                        param.ty.abi_in(),
                        function.params[buffer].name
                    ),
                    (_, ParamRole::Value) if param.ty.owned_struct_abi().is_some() => {
                        format!("_owned{i}")
                    }
                    _ => param.ty.surface_to_abi(&param.name),
                })
                .collect::<Vec<_>>()
                .join(", ");
            let owned_result = function
                .ret
                .as_ref()
                .is_some_and(|ret| ret.owned_struct_abi().is_some());
            if owned_result {
                w.line(&format!(
                    "{} result = default;",
                    function.ret.as_ref().unwrap().abi_in()
                ));
                w.open("try");
                w.line(&format!("result = {}Abi({args});", function.name));
            } else if function.hresult && out_object.is_none() {
                w.line(&format!(
                    "WindowsCsharp.Com.Check({}Abi({args}));",
                    function.name
                ));
            } else if function.hresult {
                let (name, _) = out_object.unwrap();
                w.line(&format!("int _comOutHr = {}Abi({args});", function.name));
                write_com_out_result_check(w, name);
            } else if let Some(ret) = &function.ret {
                w.line(&format!(
                    "{} result = {}Abi({args});",
                    ret.abi_in(),
                    function.name
                ));
            } else {
                w.line(&format!("{}Abi({args});", function.name));
            }
            for (i, param) in function.params.iter().enumerate() {
                if let ParamRole::ScalarPointer { target, .. } = roles[i] {
                    w.line(&format!(
                        "{} = {};",
                        param.name,
                        target.abi_to_surface(&format!("_abi{i}"))
                    ));
                }
            }
            if let Some((name, object)) = out_object {
                w.line(&format!(
                    "return WindowsCsharp.Com.Wrap<{object}>({name})!;"
                ));
            } else if let Some(ret) = &function.ret {
                if !function.hresult {
                    w.line(&format!("return {};", ret.abi_to_surface("result")));
                }
            }
            if owned_result {
                w.close();
                w.open("finally");
                w.line("result.Dispose();");
                w.close();
            }
            for role in roles.iter().rev() {
                if matches!(role, ParamRole::Utf16String | ParamRole::Buffer { .. }) {
                    w.close();
                }
            }
            if !owned_struct_params.is_empty() {
                w.close();
                w.open("finally");
                for (i, _) in owned_struct_params.iter().rev() {
                    w.line(&format!("_owned{i}.Dispose();"));
                }
                w.close();
            }
            w.close();
        }
    }
    w.close();

    fn write_import(w: &mut Writer, function: &Function, wrapped: bool) {
        w.line(&format!(
            "[LibraryImport(\"{}\", EntryPoint = \"{}\")]",
            function.library, function.import_name
        ));
        if function.cdecl {
            w.line("[UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]");
        }
        let params = function
            .params
            .iter()
            .map(|param| match &param.ty {
                CsType::ComOut { .. } => format!("nint* {}", param.name),
                _ if wrapped => format!("{} {}", param.ty.abi_in(), param.name),
                _ => format!("{} {}", param.ty.surface(), param.name),
            })
            .collect::<Vec<_>>()
            .join(", ");
        let ret = function.ret.as_ref().map_or_else(
            || "void".to_string(),
            |ty| {
                if wrapped { ty.abi_in() } else { ty.surface() }
            },
        );
        let name = if wrapped {
            format!("{}Abi", function.name)
        } else {
            function.name.clone()
        };
        let visibility = if wrapped { "private" } else { "public" };
        w.line(&format!(
            "{visibility} static partial {ret} {name}({params});"
        ));
    }
}

fn write_com_out_result_check(w: &mut Writer, name: &str) {
    w.open("if (_comOutHr < 0)");
    w.open(&format!("if ({name} != 0)"));
    w.line(&format!("_ = WindowsCsharp.Com.Release({name});"));
    w.close();
    w.line("WindowsCsharp.Com.Check(_comOutHr);");
    w.close();
    w.open(&format!("if ({name} == 0)"));
    w.line("WindowsCsharp.Com.Check(unchecked((int)0x80004003));");
    w.close();
}

fn write_inspectable(w: &mut Writer) {
    w.open("public sealed unsafe class IInspectable : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IInspectable>");
    w.line("public static Guid Iid { get; } = new Guid(0xaf86e2e0, 0xb12d, 0x4c6a, 0x9c, 0x5a, 0xd7, 0xaa, 0x65, 0x10, 0x1e, 0x90);");
    w.line("");
    w.line("internal IInspectable(nint self) : base(self, Iid) {}");
    w.line("internal IInspectable(nint self, bool trustedAgile) : base(self, trustedAgile) {}");
    w.line("static IInspectable WindowsCsharp.IComInterface<IInspectable>.FromAbi(nint self) => new IInspectable(self);");
    w.line("static IInspectable WindowsCsharp.IComInterface<IInspectable>.FromAgileAbi(nint self) => new IInspectable(self, true);");
    w.line("");
    write_generic_cast(w, Receiver::Owned);
    w.line("");
    write_borrowed(w, &[], &[]);
    w.close();
}

fn write_async_operation(w: &mut Writer, operation: &AsyncOperation) {
    let reference_values: Vec<_> = operation
        .instantiations
        .iter()
        .filter(|value| !value.element.is_unmanaged())
        .collect();
    let supports_adapters = operation
        .instantiations
        .iter()
        .any(|value| !value.element.is_unmanaged());
    let constraint = if supports_adapters {
        ""
    } else {
        " where T : unmanaged"
    };
    w.open(&format!("public sealed unsafe class IAsyncOperation<T> : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IAsyncOperation<T>>{constraint}"));
    w.line("private static readonly Guid s_asyncInfo = new Guid(0x00000036, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);");
    if supports_adapters {
        w.line("private static readonly int s_referenceType = ComputeReferenceType();");
    }
    w.line("public static Guid Iid { get; } = ComputeIid();");
    w.line("private static readonly Guid* s_completedIid = WindowsCsharp.Callback.PinIid(ComputeCompletedIid());");
    w.line("");
    if supports_adapters {
        w.open("private static int ComputeReferenceType()");
        w.open("if (!RuntimeHelpers.IsReferenceOrContainsReferences<T>())");
        w.line("return -1;");
        w.close();
        for (index, value) in reference_values.iter().enumerate() {
            w.line(&format!(
                "if (typeof(T) == typeof({})) return {index};",
                value.element.collection_surface()
            ));
        }
        w.line("throw new NotSupportedException();");
        w.close();
        w.line("");
    }
    w.open("private static Guid ComputeIid()");
    if supports_adapters {
        w.open("if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())");
        w.open("switch (s_referenceType)");
        for (index, value) in reference_values.iter().enumerate() {
            w.line(&format!("case {index}: return {};", value.iid.to_cs()));
        }
        w.close();
        w.close();
    }
    for value in operation
        .instantiations
        .iter()
        .filter(|value| value.element.is_unmanaged())
    {
        w.line(&format!(
            "if (typeof(T) == typeof({})) return {};",
            value.element.collection_surface(),
            value.iid.to_cs()
        ));
    }
    w.line("throw new NotSupportedException();");
    w.close();
    w.line("");
    w.open("private static Guid ComputeCompletedIid()");
    if supports_adapters {
        w.open("if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())");
        w.open("switch (s_referenceType)");
        for (index, value) in reference_values.iter().enumerate() {
            w.line(&format!(
                "case {index}: return {};",
                value.completed_iid.to_cs()
            ));
        }
        w.close();
        w.close();
    }
    for value in operation
        .instantiations
        .iter()
        .filter(|value| value.element.is_unmanaged())
    {
        w.line(&format!(
            "if (typeof(T) == typeof({})) return {};",
            value.element.collection_surface(),
            value.completed_iid.to_cs()
        ));
    }
    w.line("throw new NotSupportedException();");
    w.close();
    w.line("");
    w.line("internal IAsyncOperation(nint self) : base(self, Iid) {}");
    w.line("internal IAsyncOperation(nint self, bool trustedAgile) : base(self, trustedAgile) {}");
    w.line("static IAsyncOperation<T> WindowsCsharp.IComInterface<IAsyncOperation<T>>.FromAbi(nint self) => new IAsyncOperation<T>(self);");
    w.line("static IAsyncOperation<T> WindowsCsharp.IComInterface<IAsyncOperation<T>>.FromAgileAbi(nint self) => new IAsyncOperation<T>(self, true);");
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public T GetResults()");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    if supports_adapters {
        w.line("return GetResultsAbi(self);");
    } else {
        w.line("T value;");
        w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, T*, int>)(*(void***)self)[8])(self, &value));");
        w.line("return value;");
    }
    w.close();
    w.line("");
    if supports_adapters {
        w.open("private static T GetResultsAbi(nint self)");
        w.open("if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())");
        w.open("switch (s_referenceType)");
        for (index, value) in reference_values.iter().enumerate() {
            let element = &value.element;
            let surface = element.collection_surface();
            w.open(&format!("case {index}:"));
            w.line(&format!("{} result;", element.abi_in()));
            w.line(&format!(
                "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {}*, int>)(*(void***)self)[8])(self, &result));",
                element.abi_in()
            ));
            w.line(&format!(
                "{surface} value = {}!;",
                element.abi_to_surface("result")
            ));
            if element.is_object() || matches!(element, CsType::String) {
                w.line("return (T)(object)value;");
            } else {
                w.line(&format!("return Unsafe.As<{surface}, T>(ref value);"));
            }
            w.close();
        }
        w.close();
        w.close();
        for value in operation
            .instantiations
            .iter()
            .filter(|value| value.element.is_unmanaged())
        {
            let element = &value.element;
            let surface = element.collection_surface();
            w.open(&format!("if (typeof(T) == typeof({surface}))"));
            w.line(&format!("{} result;", element.abi_in()));
            w.line(&format!(
                "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {}*, int>)(*(void***)self)[8])(self, &result));",
                element.abi_in()
            ));
            w.line(&format!(
                "{surface} value = {}!;",
                element.abi_to_surface("result")
            ));
            w.line(&format!("return Unsafe.As<{surface}, T>(ref value);"));
            w.close();
        }
        w.line("throw new NotSupportedException();");
        w.close();
        w.line("");
    }
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("private bool IsCompleted()");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    w.line("Guid iid = s_asyncInfo;");
    w.line("nint info;");
    w.line("WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(self, &iid, &info));");
    w.open("try");
    w.line("int status;");
    w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)info)[7])(info, &status));");
    w.line("return status != 0;");
    w.close();
    w.open("finally");
    w.line("_ = WindowsCsharp.Com.Release(info);");
    w.close();
    w.close();
    w.line("");
    // The local callback reference covers an inline invocation. Native put_Completed takes its own
    // reference when it stores the handler. The extra operation reference protects the raw owner
    // policy from an inline continuation that disposes its owner before put_Completed returns.
    w.open("private void RegisterContinuation(Action continuation)");
    w.line("nint handler = WindowsCsharp.Callback.AllocCompleted(s_completedIid, continuation);");
    w.open("try");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    w.line("_ = WindowsCsharp.Com.AddRef(self);");
    w.open("try");
    w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[6])(self, handler));");
    w.close();
    w.open("finally");
    w.line("_ = WindowsCsharp.Com.Release(self);");
    w.close();
    w.close();
    w.open("finally");
    w.line("_ = WindowsCsharp.Com.Release(handler);");
    w.close();
    w.close();
    w.line("");
    w.line("public Awaiter GetAwaiter() => new Awaiter(this);");
    w.line("");
    w.open("public readonly struct Awaiter : ICriticalNotifyCompletion");
    w.line("private readonly IAsyncOperation<T> _operation;");
    w.line("internal Awaiter(IAsyncOperation<T> operation) => _operation = operation;");
    w.line("public bool IsCompleted => _operation.IsCompleted();");
    w.line("");
    w.line("public T GetResult() => _operation.GetResults();");
    w.line("");
    w.line("public void OnCompleted(Action continuation) => UnsafeOnCompleted(continuation);");
    w.line("");
    w.open("public void UnsafeOnCompleted(Action continuation)");
    w.open("if (continuation is null)");
    w.line("throw new ArgumentNullException(nameof(continuation));");
    w.close();
    w.line("_operation.RegisterContinuation(continuation);");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface>");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("return WindowsCsharp.Com.As<TInterface>(lease.Handle, lease.TrustedAgile);");
    w.close();
    w.close();
}

/// Emits a projected WinRT enum as a C# `enum` over its blittable underlying scalar.
fn write_enum(w: &mut Writer, e: &Enum) {
    w.open(&format!("public enum {} : {}", e.name, e.underlying));
    for (name, value) in &e.fields {
        w.line(&format!("{name} = {value},"));
    }
    w.close();
}

/// Emits a genuine Win32 opaque handle (see `native_handle_value`) as an explicit blittable
/// `readonly struct` wrapping a single `nint` field: sequential layout gives it the same size and
/// ABI shape as the raw pointer it wraps, so it crosses `LibraryImport`/vtable signatures directly
/// with no copying and no separate ABI decomposition (the same treatment as a blittable struct).
/// `default(T)` is already the zero/null handle, so no explicit `Default` is needed (unlike the
/// Rust projection, which must implement `Default` by hand for a struct wrapping a raw pointer).
/// The only conversions are to/from `nint`: implicit to `nint` (always lossless) and explicit from
/// `nint` (so a plain integer is never mistaken for a specific handle type without an explicit
/// cast). No `Close`/`Dispose` or "invalid handle" constant is emitted - ownership and the
/// invalid-handle sentinel (`null`, `-1`, ...) are API-specific and are left to the caller.
fn write_handle(w: &mut Writer, h: &Handle) {
    w.line("[StructLayout(LayoutKind.Sequential)]");
    w.open(&format!(
        "public readonly struct {} : IEquatable<{}>",
        h.name, h.name
    ));
    w.line("public readonly nint Value;");
    w.line("");
    w.open(&format!("public {}(nint value)", h.name));
    w.line("Value = value;");
    w.close();
    w.line("");
    w.line(&format!(
        "public static implicit operator nint({} value) => value.Value;",
        h.name
    ));
    w.line(&format!(
        "public static explicit operator {}(nint value) => new(value);",
        h.name
    ));
    w.line("");
    w.line(&format!(
        "public static bool operator ==({0} left, {0} right) => left.Value == right.Value;",
        h.name
    ));
    w.line(&format!(
        "public static bool operator !=({0} left, {0} right) => !(left == right);",
        h.name
    ));
    w.line("");
    w.line(&format!(
        "public bool Equals({} other) => Value == other.Value;",
        h.name
    ));
    w.line(&format!(
        "public override bool Equals(object? obj) => obj is {} other && Equals(other);",
        h.name
    ));
    w.line("public override int GetHashCode() => Value.GetHashCode();");
    w.close();
}

/// Emits a projected record with sequential struct layout or explicit native-union layout.
fn write_struct(w: &mut Writer, s: &Struct) {
    let layout = if s.explicit { "Explicit" } else { "Sequential" };
    let mut options = Vec::new();
    if let Some(pack) = s.packing_size {
        options.push(format!("Pack = {pack}"));
    }
    if let Some(size) = s.class_size {
        options.push(format!("Size = {size}"));
    }
    let options = if options.is_empty() {
        String::new()
    } else {
        format!(", {}", options.join(", "))
    };
    w.line(&format!("[StructLayout(LayoutKind.{layout}{options})]"));
    let unsafe_modifier = if s
        .fields
        .iter()
        .any(|(_, ty)| matches!(ty, CsType::Pointer { .. } | CsType::Callback { .. }))
    {
        " unsafe"
    } else {
        ""
    };
    w.open(&format!("public{unsafe_modifier} struct {}", s.name));
    for (name, ty) in &s.fields {
        if s.explicit {
            w.line("[FieldOffset(0)]");
        }
        let field_type = match ty {
            CsType::Win32Bool
            | CsType::HResult
            | CsType::Pointer { .. }
            | CsType::Callback { .. } => ty.abi_in(),
            _ => ty.surface(),
        };
        w.line(&format!("public {field_type} {name};"));
    }
    for nested in &s.nested {
        w.line("");
        write_struct(w, nested);
    }
    w.close();

    if let Some(abi_name) = &s.abi_name {
        w.line("");
        w.line(&format!("[StructLayout(LayoutKind.{layout}{options})]"));
        let unsafe_modifier = if s
            .fields
            .iter()
            .any(|(_, ty)| matches!(ty, CsType::Pointer { .. } | CsType::Callback { .. }))
        {
            " unsafe"
        } else {
            ""
        };
        w.open(&format!("internal{unsafe_modifier} struct {abi_name}"));
        for (name, ty) in &s.fields {
            if s.explicit {
                w.line("[FieldOffset(0)]");
            }
            w.line(&format!("public {} {name};", ty.abi_in()));
        }
        if s.owns_abi {
            w.line("");
            w.open(&format!(
                "internal static {abi_name} FromSurface({} value)",
                s.name
            ));
            w.line(&format!("{abi_name} result = default;"));
            w.open("try");
            for (name, ty) in &s.fields {
                w.line(&format!(
                    "result.{name} = {};",
                    struct_field_to_owned_abi(ty, &format!("value.{name}"))
                ));
            }
            w.line("return result;");
            w.close();
            w.open("catch");
            w.line("result.Dispose();");
            w.line("throw;");
            w.close();
            w.close();
            w.line("");
            w.open(&format!("internal readonly {} FromAbi() => new()", s.name));
            for (name, ty) in &s.fields {
                w.line(&format!(
                    "{name} = {},",
                    struct_field_from_borrowed_abi(ty, name)
                ));
            }
            w.close_with(";");
            w.line("");
            w.open(&format!("internal {} ToSurface()", s.name));
            w.line(&format!("{} result = default;", s.name));
            w.open("try");
            for (name, ty) in &s.fields {
                w.line(&format!(
                    "result.{name} = {};",
                    struct_field_from_owned_abi(ty, name)
                ));
            }
            w.line("return result;");
            w.close();
            w.open("finally");
            w.line("Dispose();");
            w.close();
            w.close();
            w.line("");
            w.open("internal void Dispose()");
            for (name, ty) in s.fields.iter().rev() {
                if let Some(cleanup) = struct_field_cleanup(ty, name) {
                    w.line(&cleanup);
                }
            }
            w.close();
        } else {
            w.line("");
            w.open(&format!(
                "internal static {abi_name} FromSurface({} value) => new()",
                s.name
            ));
            for (name, ty) in &s.fields {
                w.line(&format!(
                    "{name} = {},",
                    ty.surface_to_abi(&format!("value.{name}"))
                ));
            }
            w.close_with(";");
            w.line("");
            w.open(&format!(
                "internal readonly {} ToSurface() => new()",
                s.name
            ));
            for (name, ty) in &s.fields {
                w.line(&format!("{name} = {},", ty.abi_to_surface(name)));
            }
            w.close_with(";");
        }
        w.close();
    }
}

fn struct_field_to_owned_abi(ty: &CsType, expr: &str) -> String {
    match ty {
        CsType::String => format!("WindowsCsharp.Interop.CreateString({expr})"),
        CsType::Struct { owns_abi: true, .. } => ty.surface_to_owned_abi(expr),
        _ => ty.surface_to_abi(expr),
    }
}

fn struct_field_from_borrowed_abi(ty: &CsType, expr: &str) -> String {
    match ty {
        CsType::String => format!("WindowsCsharp.Interop.FromHstringBorrowed({expr})"),
        CsType::Struct { owns_abi: true, .. } => format!("{expr}.FromAbi()"),
        _ => ty.abi_to_surface(expr),
    }
}

fn struct_field_from_owned_abi(ty: &CsType, expr: &str) -> String {
    match ty {
        CsType::String => format!("WindowsCsharp.Interop.TakeHstring(ref {expr})"),
        CsType::Struct { owns_abi: true, .. } => format!("{expr}.ToSurface()"),
        _ => ty.abi_to_surface(expr),
    }
}

fn struct_field_cleanup(ty: &CsType, expr: &str) -> Option<String> {
    match ty {
        CsType::String => Some(format!("WindowsCsharp.Interop.DeleteHstring(ref {expr});")),
        CsType::Struct { owns_abi: true, .. } => Some(format!("{expr}.Dispose();")),
        _ => None,
    }
}

fn write_class(
    w: &mut Writer,
    class: &Class,
    projected_objects: &HashSet<String>,
    borrowable_objects: &HashSet<String>,
) {
    let mut declaration = format!(
        "public sealed unsafe class {0} : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<{0}>",
        class.name
    );
    for compatible in &class.compatible {
        if projected_objects.contains(compatible) {
            declaration.push_str(&format!(
                ", WindowsCsharp.IObjectParameter<{compatible}._Parameter>"
            ));
        }
    }
    w.open(&declaration);
    w.line("public readonly struct _Parameter {}");

    // A shared activation-module cache is needed whenever the class is constructed or exposes
    // static members. The default-activation factory cache (`s_factory`) is emitted only for
    // parameterless activation, keeping plain-activatable output unchanged.
    let needs_module =
        class.default_activation || !class.factories.is_empty() || !class.statics.is_empty();
    if needs_module {
        w.line("private static nint s_module;");
    }
    if class.default_activation {
        w.line("private static nint s_factory;");
    }
    w.line(&format!(
        "public static Guid Iid {{ get; }} = {};",
        class.default_iid.to_cs()
    ));
    for (index, forwarder) in class.forwarders.iter().enumerate() {
        w.line(&format!(
            "private static readonly Guid s_forward{index} = {};",
            forwarder.iid.to_cs()
        ));
    }
    for (index, factory) in class.factories.iter().enumerate() {
        w.line(&format!("private static nint s_factory{index};"));
        w.line(&format!(
            "private static readonly Guid s_factory{index}_iid = {};",
            factory.iid.to_cs()
        ));
    }
    for (index, static_iface) in class.statics.iter().enumerate() {
        w.line(&format!("private static nint s_static{index};"));
        w.line(&format!(
            "private static readonly Guid s_static{index}_iid = {};",
            static_iface.iid.to_cs()
        ));
    }
    w.line("");

    w.line(&format!(
        "internal {}(nint self) : base(self, Iid) {{}}",
        class.name
    ));
    w.line(&format!(
        "internal {}(nint self, bool trustedAgile) : base(self, trustedAgile) {{}}",
        class.name
    ));
    w.line(&format!(
        "static {0} WindowsCsharp.IComInterface<{0}>.FromAbi(nint self) => new {0}(self);",
        class.name
    ));
    w.line(&format!(
        "static {0} WindowsCsharp.IComInterface<{0}>.FromAgileAbi(nint self) => new {0}(self, true);",
        class.name
    ));

    // Track emitted constructor parameter signatures so a parameterless composable constructor does
    // not collide with default activation, and two factory methods with the same surface parameter
    // list do not emit duplicate C# constructors.
    let mut signatures: HashSet<String> = HashSet::new();
    if class.default_activation {
        w.line("");
        write_activating_ctor(w, class);
        signatures.insert(String::new());
    }

    let mut helper_index = 0usize;
    for (factory_index, factory) in class.factories.iter().enumerate() {
        for ctor in &factory.constructors {
            let signature = ctor
                .params
                .iter()
                .map(|param| param.ty.surface())
                .collect::<Vec<_>>()
                .join(", ");
            if !signatures.insert(signature) {
                continue;
            }
            write_factory_constructor(
                w,
                class,
                factory_index,
                factory.composable,
                ctor,
                helper_index,
            );
            helper_index += 1;
        }
    }

    write_members(w, &class.members, Receiver::Owned);
    write_forwarders(w, &class.forwarders, Receiver::Owned);

    w.line("");
    write_generic_cast(w, Receiver::Owned);

    // Static members reach their factory interface through the shared module cache and this static
    // interface's own agile factory cache, marshalling exactly like instance members.
    for (index, static_iface) in class.statics.iter().enumerate() {
        let access = FactoryAccess {
            module: "s_module".to_string(),
            cache: format!("s_static{index}"),
            class_id: class.class_id(),
            iid: format!("s_static{index}_iid"),
        };
        write_members(w, &static_iface.members, Receiver::Factory(&access));
    }

    w.line("");
    write_borrowed(w, &class.members, &class.forwarders);
    write_borrow_as(w, &class.class_id(), &class.compatible, borrowable_objects);

    w.close();
}

/// Emits a WinRT interface as its own projected class: one interface pointer, an internal `nint`
/// constructor, the `IComInterface` support surface (IID + `FromAbi`), its members, the generic
/// `As<T>()` cast, and inherited ownership support.
fn write_interface(
    w: &mut Writer,
    interface: &Interface,
    projected_objects: &HashSet<String>,
    borrowable_objects: &HashSet<String>,
    generated_com_objects: &HashSet<String>,
) {
    let mut declaration = format!(
        "public sealed unsafe class {0} : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<{0}>",
        interface.name
    );
    for compatible in &interface.compatible {
        if projected_objects.contains(compatible) {
            declaration.push_str(&format!(
                ", WindowsCsharp.IObjectParameter<{compatible}._Parameter>"
            ));
        }
    }
    w.open(&declaration);
    w.line("public readonly struct _Parameter {}");

    w.line(&format!(
        "public static Guid Iid {{ get; }} = {};",
        interface.iid.to_cs()
    ));
    w.line("");
    w.line(&format!(
        "internal {}(nint self) : base(self, Iid) {{}}",
        interface.name
    ));
    w.line(&format!(
        "internal {}(nint self, bool trustedAgile) : base(self, trustedAgile) {{}}",
        interface.name
    ));
    w.line(&format!(
        "static {0} WindowsCsharp.IComInterface<{0}>.FromAbi(nint self) => new {0}(self);",
        interface.name
    ));
    w.line(&format!(
        "static {0} WindowsCsharp.IComInterface<{0}>.FromAgileAbi(nint self) => new {0}(self, true);",
        interface.name
    ));
    write_members(w, &interface.members, Receiver::Owned);

    w.line("");
    write_generic_cast(w, Receiver::Owned);

    w.line("");
    write_borrowed(w, &interface.members, &[]);
    write_borrow_as(
        w,
        &format!("{}.{}", interface.namespace, interface.name),
        &interface.compatible,
        borrowable_objects,
    );

    w.close();

    if generated_com_objects.contains(&format!("{}.{}", interface.namespace, interface.name)) {
        w.line("");
        write_generated_com_interface(w, interface);
    }
}

fn write_generated_com_interface(w: &mut Writer, interface: &Interface) {
    w.line("[System.Runtime.InteropServices.Marshalling.GeneratedComInterface]");
    w.line(&format!("[Guid(\"{}\")]", interface.iid.to_guid_string()));
    let base = interface
        .native_base
        .as_ref()
        .map(|base| format!(" : {base}Abi"))
        .unwrap_or_default();
    w.open(&format!(
        "public unsafe partial interface {}Abi{base}",
        interface.name
    ));
    for member in interface.native_own_members.as_ref().unwrap() {
        let Member::Method {
            name, params, ret, ..
        } = member
        else {
            unreachable!();
        };
        w.line("[PreserveSig]");
        let indirect_ret = ret.as_ref().filter(|ret| ret.is_native_com_record_return());
        let ret = if indirect_ret.is_some() {
            "void".to_string()
        } else {
            ret.as_ref()
                .map_or_else(|| "void".to_string(), CsType::abi_in)
        };
        let mut params = params
            .iter()
            .map(|param| {
                let ty = if matches!(param.ty, CsType::ComOut { .. }) {
                    "nint*".to_string()
                } else {
                    param.ty.abi_in()
                };
                format!("{ty} {}", param.name)
            })
            .collect::<Vec<_>>();
        if let Some(indirect_ret) = indirect_ret {
            params.insert(0, format!("{}* result__", indirect_ret.abi_in()));
        }
        let params = params.join(", ");
        w.line(&format!("{ret} {name}({params});"));
    }
    w.close();
}

/// Emits a projected WinRT delegate as a sealed class that both invokes a delegate pointer received
/// from native (a slot-3 call through a leased pointer) and, through `Create`, builds a
/// native callback object backed by a managed `Callback`. The reverse vtable shares its
/// `IUnknown` thunks (`QueryInterface`/`AddRef`/`Release`) with every other delegate through
/// `WindowsCsharp.Callback`; only the slot-3 `Invoke` thunk is emitted per delegate.
fn write_delegate(w: &mut Writer, d: &Delegate) {
    w.open(&format!(
        "public sealed unsafe class {0} : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<{0}>, WindowsCsharp.IObjectParameter<{1}.{0}._Parameter>",
        d.name, d.namespace
    ));
    w.line("public readonly struct _Parameter {}");

    w.line(&format!(
        "public static Guid Iid {{ get; }} = {};",
        d.iid.to_cs()
    ));
    w.line("");
    w.line(&format!(
        "internal {}(nint self) : base(self, Iid) {{}}",
        d.name
    ));
    w.line(&format!(
        "internal {}(nint self, bool trustedAgile) : base(self, trustedAgile) {{}}",
        d.name
    ));
    w.line(&format!(
        "static {0} WindowsCsharp.IComInterface<{0}>.FromAbi(nint self) => new {0}(self);",
        d.name
    ));
    w.line(&format!(
        "static {0} WindowsCsharp.IComInterface<{0}>.FromAgileAbi(nint self) => new {0}(self, true);",
        d.name
    ));
    w.line("");

    // The managed callback shape, spelled in surface types.
    let surface_params = d
        .params
        .iter()
        .map(|param| format!("{} {}", param.ty.callback_surface(), param.name))
        .collect::<Vec<_>>()
        .join(", ");
    let ret_surface = d
        .ret
        .as_ref()
        .map_or_else(|| "void".to_string(), CsType::surface);
    w.line(&format!(
        "public delegate {ret_surface} Callback({surface_params});"
    ));
    w.line("");

    // Per-delegate pinned IID and reverse vtable, built once on first use.
    w.line("private static readonly Guid* s_iid = WindowsCsharp.Callback.PinIid(Iid);");
    w.line("private static readonly nint* s_vtable = BuildVtable();");
    w.line("");

    // The slot-3 `Invoke` thunk's unmanaged signature, matching the ABI: `this`, the blittable
    // parameters, an out-pointer for a blittable return, and the `HRESULT`.
    let mut invoke_generics = vec!["nint".to_string()];
    for param in &d.params {
        invoke_generics.push(param.ty.abi_in());
    }
    if let Some(ret) = &d.ret {
        invoke_generics.push(ret.abi_out());
    }
    invoke_generics.push("int".to_string());

    w.open("private static nint* BuildVtable()");
    w.line("nint* vtable = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));");
    w.line("vtable[0] = WindowsCsharp.Callback.QueryInterfacePtr;");
    w.line("vtable[1] = WindowsCsharp.Callback.AddRefPtr;");
    w.line("vtable[2] = WindowsCsharp.Callback.ReleasePtr;");
    w.line(&format!(
        "vtable[3] = (nint)(delegate* unmanaged<{}>)&NativeInvoke;",
        invoke_generics.join(", ")
    ));
    w.line("return vtable;");
    w.close();
    w.line("");

    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line(&format!(
        "public static {0} Create(Callback handler) => WindowsCsharp.Com.WrapAgile<{0}>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;",
        d.name
    ));
    w.line("");

    // The reverse thunk: recover the managed callback, invoke it, and translate an exception into
    // an `HRESULT` so a native caller never sees a managed exception cross the ABI.
    let mut thunk_params = vec!["nint self".to_string()];
    for param in &d.params {
        thunk_params.push(format!("{} {}", param.ty.abi_in(), param.name));
    }
    if let Some(ret) = &d.ret {
        thunk_params.push(format!("{}* result", ret.abi_in()));
    }
    let call_args = d
        .params
        .iter()
        .map(|param| param.ty.abi_to_callback_surface(&param.name))
        .collect::<Vec<_>>()
        .join(", ");

    w.line("[UnmanagedCallersOnly]");
    w.open(&format!(
        "private static int NativeInvoke({})",
        thunk_params.join(", ")
    ));
    if d.ret.is_some() {
        w.open("if (result == null)");
        w.line("return unchecked((int)0x80004003);");
        w.close();
        w.line("*result = default;");
    }
    let owned_result = d
        .ret
        .as_ref()
        .is_some_and(|ret| matches!(ret, CsType::String) || ret.is_object());
    if owned_result {
        w.line("nint ownedResult = 0;");
    }
    w.open("try");
    w.line("Callback callback = (Callback)WindowsCsharp.Callback.Target(self);");
    if let Some(ret) = &d.ret {
        if matches!(ret, CsType::String) {
            w.line(&format!(
                "ownedResult = WindowsCsharp.Interop.CreateString(callback({call_args}));"
            ));
            w.line("*result = ownedResult;");
            w.line("ownedResult = 0;");
        } else if ret.is_object() {
            w.line(&format!(
                "using WindowsCsharp.ComLease resultLease = WindowsCsharp.ComLease.From(callback({call_args}));"
            ));
            w.line("nint resultValue = resultLease.Handle;");
            w.open("if (resultValue != 0)");
            w.line("_ = WindowsCsharp.Com.AddRef(resultValue);");
            w.line("ownedResult = resultValue;");
            w.close();
            w.line("*result = ownedResult;");
            w.line("ownedResult = 0;");
        } else {
            let converted = if ret.owned_struct_abi().is_some() {
                ret.surface_to_owned_abi(&format!("callback({call_args})"))
            } else {
                ret.surface_to_abi(&format!("callback({call_args})"))
            };
            w.line(&format!("*result = {converted};"));
        }
    } else {
        w.line(&format!("callback({call_args});"));
    }
    w.line("return 0;");
    w.close();
    w.open("catch (Exception error)");
    if matches!(d.ret.as_ref(), Some(CsType::String)) {
        w.line("WindowsCsharp.Interop.DeleteHstring(ref ownedResult);");
    } else if owned_result {
        w.open("if (ownedResult != 0)");
        w.line("_ = WindowsCsharp.Com.Release(ownedResult);");
        w.close();
    }
    w.line("return Marshal.GetHRForException(error);");
    w.close();
    w.close();

    // The forward `Invoke`: the leased native delegate is called at slot 3.
    w.line("");
    write_method(
        w,
        "Invoke",
        &d.params,
        d.ret.as_ref(),
        3,
        MethodAbi::WinRt,
        Receiver::Owned,
        None,
    );

    w.line("");
    write_generic_cast(w, Receiver::Owned);

    w.line("");
    write_borrowed(w, &[], &[]);

    w.close();
}

/// Emits a projected arity-one generic collection. Value-only projections retain the
/// `where T : unmanaged` fast path. A projection containing object elements emits type-specialized
/// ABI adapters selected by `typeof(T)`: value elements cross by value and object elements cross as
/// owned or borrowed interface pointers.
///
/// The surface is the zero-allocation read primitive set: `Count` (`get_Size`, slot 7), `GetAt`
/// (slot 6), a `Span<T>`-based `GetMany` for batched reads, and a `GetEnumerator` returning a
/// struct enumerator so `foreach` batches through `GetMany` with no per-element vtable call and no
/// heap allocation - matching how the Rust and C++/WinRT consumers iterate.
fn write_vector(
    w: &mut Writer,
    vector: &Vector,
    base_name: &str,
    getmany_slot: usize,
    mutable: bool,
) {
    let reference_values: Vec<_> = vector
        .instantiations
        .iter()
        .filter(|value| !value.element.is_unmanaged())
        .collect();
    let supports_objects = vector
        .instantiations
        .iter()
        .any(|value| value.element.is_object());
    let supports_adapters = vector
        .instantiations
        .iter()
        .any(|value| !value.element.is_unmanaged());
    let constraint = if supports_adapters {
        ""
    } else {
        " where T : unmanaged"
    };
    w.open(&format!(
        "public sealed unsafe class {base_name}<T> : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<{base_name}<T>>{constraint}"
    ));

    if supports_adapters {
        w.line("private static readonly int s_referenceType = ComputeReferenceType();");
    }
    w.line("public static Guid Iid { get; } = ComputeIid();");
    w.line("");

    if supports_adapters {
        w.open("private static int ComputeReferenceType()");
        w.open("if (!RuntimeHelpers.IsReferenceOrContainsReferences<T>())");
        w.line("return -1;");
        w.close();
        for (index, value) in reference_values.iter().enumerate() {
            w.line(&format!(
                "if (typeof(T) == typeof({})) return {index};",
                value.element.collection_surface()
            ));
        }
        w.line("throw new NotSupportedException();");
        w.close();
        w.line("");
    }

    // The per-element IID switch, resolved once into the static `Iid` property. Each arm is a
    // generation-time parameterized GUID literal, so no SHA1 runs at run time.
    w.open("private static Guid ComputeIid()");
    if supports_adapters {
        w.open("if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())");
        w.open("switch (s_referenceType)");
        for (index, value) in reference_values.iter().enumerate() {
            w.line(&format!("case {index}: return {};", value.iid.to_cs()));
        }
        w.close();
        w.close();
    }
    for value in vector
        .instantiations
        .iter()
        .filter(|value| value.element.is_unmanaged())
    {
        w.line(&format!(
            "if (typeof(T) == typeof({})) return {};",
            value.element.collection_surface(),
            value.iid.to_cs()
        ));
    }
    w.line("throw new NotSupportedException();");
    w.close();
    w.line("");

    w.line(&format!(
        "internal {base_name}(nint self) : base(self, Iid) {{}}"
    ));
    w.line(&format!(
        "internal {base_name}(nint self, bool trustedAgile) : base(self, trustedAgile) {{}}"
    ));
    w.line(&format!("static {base_name}<T> WindowsCsharp.IComInterface<{base_name}<T>>.FromAbi(nint self) => new {base_name}<T>(self);"));
    w.line(&format!("static {base_name}<T> WindowsCsharp.IComInterface<{base_name}<T>>.FromAgileAbi(nint self) => new {base_name}<T>(self, true);"));
    w.line("");

    // Count: get_Size at slot 7.
    w.open("public uint Count");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("get");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    w.line("uint value;");
    w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int>)(*(void***)self)[7])(self, &value));");
    w.line("return value;");
    w.close();
    w.close();
    w.line("");

    // GetAt at slot 6.
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public T GetAt(uint index)");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    if supports_adapters {
        w.line("return GetAtAbi(self, index);");
    } else {
        w.line("T result;");
        w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, T*, int>)(*(void***)self)[6])(self, index, &result));");
        w.line("return result;");
    }
    w.close();
    w.line("");

    // GetMany at slot `getmany_slot`: fill the caller's span and return the count copied.
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public uint GetMany(uint startIndex, Span<T> items)");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    if supports_adapters {
        w.line("return GetManyAbi(self, startIndex, items);");
    } else {
        w.line("uint actual;");
        w.open("fixed (T* p = items)");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, T*, uint*, int>)(*(void***)self)[{getmany_slot}])(self, startIndex, (uint)items.Length, p, &actual));"));
        w.close();
        w.line("return actual;");
    }
    w.close();
    w.line("");

    if mutable {
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("public void Append(T value)");
        w.line("using WindowsCsharp.ComLease lease = Acquire();");
        if supports_adapters {
            w.line("AppendAbi(lease.Handle, value);");
        } else {
            w.line("nint self = lease.Handle;");
            w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, T, int>)(*(void***)self)[13])(self, value));");
        }
        w.close();
        w.line("");
        if supports_objects {
            w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
            w.open("internal void AppendObject(WindowsCsharp.ComObject? value, Guid iid)");
            w.line("using WindowsCsharp.ComLease lease = Acquire();");
            w.line("using WindowsCsharp.InterfaceLease itemLease = WindowsCsharp.InterfaceLease.From(value, iid);");
            w.line("nint self = lease.Handle;");
            w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, itemLease.Handle));");
            w.close();
            w.line("");
        }
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("public void RemoveAtEnd()");
        w.line("using WindowsCsharp.ComLease lease = Acquire();");
        w.line("nint self = lease.Handle;");
        w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)self)[14])(self));");
        w.close();
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("public void Clear()");
        w.line("using WindowsCsharp.ComLease lease = Acquire();");
        w.line("nint self = lease.Handle;");
        w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)self)[15])(self));");
        w.close();
        w.line("");
    }

    if supports_adapters {
        write_vector_adapters(w, vector, mutable, getmany_slot);
    }

    // Zero-allocation `foreach`: `GetEnumerator` returns a mutable struct enumerator, so the C#
    // compiler duck-types the loop onto it with no interface dispatch and no heap object. The
    // enumerator batches through `GetMany` into a stack-resident `[InlineArray]` buffer - one
    // vtable call per block instead of one per element - mirroring the windows-rs `BufferedIterator`.
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line("public Enumerator GetEnumerator() => new Enumerator(this);");
    w.line("");

    if supports_objects {
        w.open("public struct Enumerator : IDisposable");
    } else {
        w.open("public struct Enumerator");
    }
    w.line("private const int BufferLength = 64;");
    w.line(&format!("private readonly {base_name}<T> _vector;"));
    w.line("private Buffer _buffer;");
    w.line("private uint _start;");
    w.line("private int _index;");
    w.line("private int _length;");
    w.line("private T _current;");
    w.line("");

    w.open(&format!("internal Enumerator({base_name}<T> vector)"));
    w.line("_vector = vector;");
    w.line("_buffer = default;");
    w.line("_start = 0;");
    w.line("_index = 0;");
    w.line("_length = 0;");
    w.line("_current = default!;");
    w.close();
    w.line("");

    w.line("public readonly T Current => _current;");
    w.line("");

    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public bool MoveNext()");
    w.open("if (_index >= _length)");
    w.line("_index = 0;");
    w.line("_length = (int)_vector.GetMany(_start, MemoryMarshal.CreateSpan(ref Unsafe.As<Buffer, T>(ref _buffer), BufferLength));");
    w.line("_start += (uint)_length;");
    w.open("if (_length == 0)");
    w.line("return false;");
    w.close();
    w.close();
    w.line("_current = Unsafe.Add(ref Unsafe.As<Buffer, T>(ref _buffer), _index);");
    w.line("_index++;");
    w.line("return true;");
    w.close();
    w.line("");
    if supports_objects {
        w.open("public void Dispose()");
        w.open("for (int i = _index; i < _length; i++)");
        w.line("ref T value = ref Unsafe.Add(ref Unsafe.As<Buffer, T>(ref _buffer), i);");
        w.open("if (value is WindowsCsharp.ComObject item)");
        w.line("item.Dispose();");
        w.close();
        w.line("value = default!;");
        w.close();
        w.line("_index = _length;");
        w.close();
        w.line("");
    }

    w.line("[InlineArray(BufferLength)]");
    w.open("private struct Buffer");
    w.line("private T _element0;");
    w.close();

    w.close();
    w.line("");

    // The generic cast uses its own type parameter name to avoid shadowing the element `T`.
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface>");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("return WindowsCsharp.Com.As<TInterface>(lease.Handle, lease.TrustedAgile);");
    w.close();
    w.line("");
    write_vector_borrowed(w, getmany_slot, supports_adapters, mutable);

    w.close();

    if mutable && supports_objects {
        w.line("");
        w.open("public static class IVectorObjectExtensions");
        for value in &vector.instantiations {
            let CsType::Object { name } = &value.element else {
                continue;
            };
            let surface = value.element.collection_generic_surface();
            w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
            w.open(&format!(
                "public static void Append<TValue>(this IVector<{surface}> vector, TValue? value) where TValue : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TValue>, WindowsCsharp.IObjectParameter<{name}._Parameter>"
            ));
            w.line(&format!("vector.AppendObject(value, {name}.Iid);"));
            w.close();
            w.line("");
        }
        w.close();
    }
}

fn write_vector_adapters(w: &mut Writer, vector: &Vector, mutable: bool, getmany_slot: usize) {
    let reference_values: Vec<_> = vector
        .instantiations
        .iter()
        .filter(|value| !value.element.is_unmanaged())
        .collect();

    w.open("private static T GetAtAbi(nint self, uint index)");
    w.open("if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())");
    w.open("switch (s_referenceType)");
    for (index, value) in reference_values.iter().enumerate() {
        let element = &value.element;
        let surface = element.collection_surface();
        w.open(&format!("case {index}:"));
        w.line(&format!("{} result;", element.abi_in()));
        w.line(&format!(
            "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, {}*, int>)(*(void***)self)[6])(self, index, &result));",
            element.abi_in()
        ));
        if element.is_object() || matches!(element, CsType::String) {
            w.line(&format!(
                "{surface} value = {}!;",
                element.abi_to_surface("result")
            ));
            w.line("return (T)(object)value;");
        } else {
            w.line(&format!(
                "{surface} value = {};",
                element.abi_to_surface("result")
            ));
            w.line(&format!("return Unsafe.As<{surface}, T>(ref value);"));
        }
        w.close();
    }
    w.close();
    w.close();
    for value in vector
        .instantiations
        .iter()
        .filter(|value| value.element.is_unmanaged())
    {
        let element = &value.element;
        let surface = element.collection_surface();
        w.open(&format!("if (typeof(T) == typeof({surface}))"));
        w.line(&format!("{} result;", element.abi_in()));
        w.line(&format!(
            "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, {}*, int>)(*(void***)self)[6])(self, index, &result));",
            element.abi_in()
        ));
        w.line(&format!(
            "{surface} value = {};",
            element.abi_to_surface("result")
        ));
        w.line(&format!("return Unsafe.As<{surface}, T>(ref value);"));
        w.close();
    }
    w.line("throw new NotSupportedException();");
    w.close();
    w.line("");

    w.open("private static uint GetManyAbi(nint self, uint startIndex, Span<T> items)");
    w.open("if (items.IsEmpty)");
    w.line("return 0;");
    w.close();
    for value in vector
        .instantiations
        .iter()
        .filter(|value| value.element.is_unmanaged())
    {
        let element = &value.element;
        if !element.is_blittable() {
            continue;
        }
        let surface = element.collection_surface();
        let abi = element.abi_in();
        w.open(&format!("if (typeof(T) == typeof({surface}))"));
        w.line("uint actual;");
        w.line("ref T first = ref MemoryMarshal.GetReference(items);");
        w.open(&format!(
            "fixed ({abi}* p = &Unsafe.As<T, {abi}>(ref first))"
        ));
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, {abi}*, uint*, int>)(*(void***)self)[{getmany_slot}])(self, startIndex, (uint)items.Length, p, &actual));"));
        w.close();
        w.line("return actual;");
        w.close();
    }
    w.open("if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())");
    w.open("switch (s_referenceType)");
    for (index, value) in reference_values.iter().enumerate() {
        let element = &value.element;
        let surface = element.collection_surface();
        w.open(&format!("case {index}:"));
        w.line("const uint Capacity = 64;");
        w.line("nint* values = stackalloc nint[(int)Capacity];");
        w.line("uint total = 0;");
        w.open("while (total < (uint)items.Length)");
        w.line("uint requested = Math.Min(Capacity, (uint)items.Length - total);");
        w.open("for (uint i = 0; i < requested; i++)");
        w.line("values[i] = 0;");
        w.close();
        w.line("uint actual;");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, nint*, uint*, int>)(*(void***)self)[{getmany_slot}])(self, startIndex + total, requested, values, &actual));"));
        w.open("try");
        w.open("for (uint i = 0; i < actual; i++)");
        w.line("nint abi = values[i];");
        w.line("values[i] = 0;");
        if element.is_object() {
            w.line(&format!(
                "{surface}? value = {};",
                element.abi_to_surface("abi")
            ));
            w.line("items[(int)(total + i)] = (T)(object?)value!;");
        } else {
            w.line(&format!(
                "{surface} value = {};",
                element.abi_to_surface("abi")
            ));
            w.line("items[(int)(total + i)] = (T)(object)value;");
        }
        w.close();
        w.close();
        w.open("finally");
        w.open("for (uint i = 0; i < actual; i++)");
        w.open("if (values[i] != 0)");
        if element.is_object() {
            w.line("_ = WindowsCsharp.Com.Release(values[i]);");
        } else {
            w.line("_ = WindowsCsharp.Interop.WindowsDeleteString(values[i]);");
        }
        w.close();
        w.close();
        w.close();
        w.line("total += actual;");
        w.open("if (actual < requested)");
        w.line("break;");
        w.close();
        w.close();
        w.line("return total;");
        w.close();
    }
    w.close();
    w.close();
    w.line("uint size;");
    w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int>)(*(void***)self)[7])(self, &size));");
    w.open("if (startIndex >= size)");
    w.line("return 0;");
    w.close();
    w.line("uint count = Math.Min((uint)items.Length, size - startIndex);");
    w.open("for (uint i = 0; i < count; i++)");
    w.line("items[(int)i] = GetAtAbi(self, startIndex + i);");
    w.close();
    w.line("return count;");
    w.close();
    w.line("");

    if mutable {
        w.open("private static void AppendAbi(nint self, T value)");
        w.open("if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())");
        w.open("switch (s_referenceType)");
        for (index, value) in reference_values.iter().enumerate() {
            let element = &value.element;
            let surface = element.collection_surface();
            w.open(&format!("case {index}:"));
            if element.is_object() {
                w.line("object? boxed = value;");
                w.open("if (boxed is null)");
                w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, 0));");
                w.line("return;");
                w.close();
                w.line(&format!("{surface} item = ({surface})boxed;"));
                w.line("using WindowsCsharp.ComLease itemLease = item.Acquire();");
                w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, itemLease.Handle));");
            } else if matches!(element, CsType::String) {
                w.line("string? item = (string?)(object?)value;");
                w.open("fixed (char* buffer = item)");
                w.line("WindowsCsharp.Interop.HstringHeader header;");
                w.line("nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)buffer, (uint)(item?.Length ?? 0), &header);");
                w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, hstring));");
                w.close();
            } else {
                w.line(&format!(
                    "{surface} item = Unsafe.As<T, {surface}>(ref value);"
                ));
                w.line(&format!(
                    "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {}, int>)(*(void***)self)[13])(self, {}));",
                    element.abi_in(),
                    element.surface_to_abi("item")
                ));
            }
            w.line("return;");
            w.close();
        }
        w.close();
        w.close();
        for value in vector
            .instantiations
            .iter()
            .filter(|value| value.element.is_unmanaged())
        {
            let element = &value.element;
            let surface = element.collection_surface();
            w.open(&format!("if (typeof(T) == typeof({surface}))"));
            w.line(&format!(
                "{surface} item = Unsafe.As<T, {surface}>(ref value);"
            ));
            w.line(&format!(
                "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {}, int>)(*(void***)self)[13])(self, {}));",
                element.abi_in(),
                element.surface_to_abi("item")
            ));
            w.line("return;");
            w.close();
        }
        w.line("throw new NotSupportedException();");
        w.close();
        w.line("");
    }
}

fn write_vector_borrowed(
    w: &mut Writer,
    getmany_slot: usize,
    supports_adapters: bool,
    mutable: bool,
) {
    w.line("public delegate void BorrowAction(Borrowed value);");
    w.line("public delegate TResult BorrowFunc<TResult>(Borrowed value);");
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public void Borrow(BorrowAction action)");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("action(new Borrowed(lease.Handle));");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public TResult Borrow<TResult>(BorrowFunc<TResult> action)");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("return action(new Borrowed(lease.Handle));");
    w.close();
    w.line("");
    w.open("public readonly ref struct Borrowed");
    w.line("private readonly nint _this;");
    w.line("internal Borrowed(nint self) => _this = self;");
    w.line("");
    w.open("public uint Count");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("get");
    w.line("uint value;");
    w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int>)(*(void***)_this)[7])(_this, &value));");
    w.line("return value;");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public T GetAt(uint index)");
    if supports_adapters {
        w.line("return GetAtAbi(_this, index);");
    } else {
        w.line("T result;");
        w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, T*, int>)(*(void***)_this)[6])(_this, index, &result));");
        w.line("return result;");
    }
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public uint GetMany(uint startIndex, Span<T> items)");
    if supports_adapters {
        w.line("return GetManyAbi(_this, startIndex, items);");
    } else {
        w.line("uint actual;");
        w.open("fixed (T* p = items)");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, T*, uint*, int>)(*(void***)_this)[{getmany_slot}])(_this, startIndex, (uint)items.Length, p, &actual));"));
        w.close();
        w.line("return actual;");
    }
    w.close();
    w.line("");
    if mutable {
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        if supports_adapters {
            w.line("public void Append(T value) => AppendAbi(_this, value);");
        } else {
            w.line("public void Append(T value) => WindowsCsharp.Com.Check(((delegate* unmanaged<nint, T, int>)(*(void***)_this)[13])(_this, value));");
        }
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.line("public void RemoveAtEnd() => WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)_this)[14])(_this));");
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.line("public void Clear() => WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)_this)[15])(_this));");
        w.line("");
    }
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line("public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface> => WindowsCsharp.Com.As<TInterface>(_this, false);");
    w.close();
}

/// Emits a projected arity-two generic collection as a single real C# generic
/// sealed class with unmanaged keys and values, mirroring [`write_vector`].
/// `base_name` is the class name (`IMap` for the mutable collection, `IMapView` for the read-only
/// view); `mutable` gates the mutators. Small projections use closed-generic `typeof` dispatch.
/// Broad projections select one set of per-pair managed function pointers in the static
/// constructor, avoiding oversized hot methods and repeated type tests. The read surface is the
/// dictionary primitive set: `Count` (`get_Size`, slot 7), `Lookup` (slot 6), and `HasKey` (slot 8);
/// when `mutable`, the mutators `Insert` (slot 10), `Remove` (slot 11), and `Clear` (slot 12) are
/// also emitted. WinRT `Boolean` results cross the ABI as a single byte.
fn write_map(w: &mut Writer, map: &Map, base_name: &str, mutable: bool) {
    let specialized = map.instantiations.len() >= GENERIC_SPECIALIZATION_THRESHOLD;
    let supports_value_adapters = map
        .instantiations
        .iter()
        .any(|item| !item.value.is_unmanaged());
    let constraint = if supports_value_adapters {
        ""
    } else {
        " where V : unmanaged"
    };
    w.open(&format!("public sealed unsafe class {base_name}<K, V> : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<{base_name}<K, V>>{constraint}"));

    if specialized {
        w.line("private static readonly Guid s_iterableIid;");
        w.line("private static readonly Guid s_iteratorIid;");
        w.line("private static readonly delegate* managed<nint, K, V> s_lookup;");
        w.line("private static readonly delegate* managed<nint, K, bool> s_hasKey;");
        if mutable {
            w.line("private static readonly delegate* managed<nint, K, V, bool> s_insert;");
            w.line("private static readonly delegate* managed<nint, K, void> s_remove;");
        }
        w.line("private static readonly delegate* managed<nint, Entry> s_entry;");
        w.line("public static Guid Iid { get; }");
        w.line("");
        w.open(&format!("static {base_name}()"));
        for (index, item) in map.instantiations.iter().enumerate() {
            w.open(&format!(
                "if (typeof(K) == typeof({}) && typeof(V) == typeof({}))",
                item.key.collection_surface(),
                item.value.collection_surface()
            ));
            w.line(&format!("Iid = {};", item.iid.to_cs()));
            w.line(&format!("s_iterableIid = {};", item.iterable_iid.to_cs()));
            w.line(&format!("s_iteratorIid = {};", item.iterator_iid.to_cs()));
            w.line(&format!("s_lookup = &Lookup{index};"));
            w.line(&format!("s_hasKey = &HasKey{index};"));
            if mutable {
                w.line(&format!("s_insert = &Insert{index};"));
                w.line(&format!("s_remove = &Remove{index};"));
            }
            w.line(&format!("s_entry = &ReadEntry{index};"));
            w.line("return;");
            w.close();
        }
        w.line("throw new NotSupportedException();");
        w.close();
        w.line("");
    } else {
        w.line("public static Guid Iid { get; } = ComputeIid();");
        w.line("");

        w.open("private static Guid ComputeIid()");
        for item in &map.instantiations {
            w.line(&format!(
                "if (typeof(K) == typeof({key}) && typeof(V) == typeof({value})) return {};",
                item.iid.to_cs(),
                key = item.key.collection_surface(),
                value = item.value.collection_surface()
            ));
        }
        w.line("throw new NotSupportedException();");
        w.close();
        w.line("");

        w.open("private static Guid ComputeIterableIid()");
        for item in &map.instantiations {
            w.line(&format!(
                "if (typeof(K) == typeof({key}) && typeof(V) == typeof({value})) return {};",
                item.iterable_iid.to_cs(),
                key = item.key.collection_surface(),
                value = item.value.collection_surface()
            ));
        }
        w.line("throw new NotSupportedException();");
        w.close();
        w.line("");

        w.open("private static Guid ComputeIteratorIid()");
        for item in &map.instantiations {
            w.line(&format!(
                "if (typeof(K) == typeof({key}) && typeof(V) == typeof({value})) return {};",
                item.iterator_iid.to_cs(),
                key = item.key.collection_surface(),
                value = item.value.collection_surface()
            ));
        }
        w.line("throw new NotSupportedException();");
        w.close();
        w.line("");
    }

    w.line(&format!(
        "internal {base_name}(nint self) : base(self, Iid) {{}}"
    ));
    w.line(&format!(
        "internal {base_name}(nint self, bool trustedAgile) : base(self, trustedAgile) {{}}"
    ));
    w.line(&format!("static {base_name}<K, V> WindowsCsharp.IComInterface<{base_name}<K, V>>.FromAbi(nint self) => new {base_name}<K, V>(self);"));
    w.line(&format!("static {base_name}<K, V> WindowsCsharp.IComInterface<{base_name}<K, V>>.FromAgileAbi(nint self) => new {base_name}<K, V>(self, true);"));
    w.line("");

    // Count: get_Size at slot 7.
    w.open("public uint Count");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("get");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    w.line("uint value;");
    w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int>)(*(void***)self)[7])(self, &value));");
    w.line("return value;");
    w.close();
    w.close();
    w.line("");

    // Lookup at slot 6.
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public V Lookup(K key)");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    if specialized {
        w.line("return s_lookup(self, key);");
    } else {
        write_map_lookup(w, map, "self");
    }
    w.close();
    w.line("");

    // HasKey at slot 8: the WinRT Boolean result is a single ABI byte.
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public bool HasKey(K key)");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    if specialized {
        w.line("return s_hasKey(self, key);");
    } else {
        write_map_has_key(w, map, "self");
    }
    w.close();
    w.line("");

    // Insert at slot 10: returns whether an existing key's value was replaced. Emitted only for the
    // mutable `IMap`; the read-only `IMapView` has no mutators.
    if mutable {
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("public bool Insert(K key, V value)");
        w.line("using WindowsCsharp.ComLease lease = Acquire();");
        w.line("nint self = lease.Handle;");
        if specialized {
            w.line("return s_insert(self, key, value);");
        } else {
            write_map_insert(w, map, "self");
        }
        w.close();
        w.line("");

        // Remove at slot 11.
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("public void Remove(K key)");
        w.line("using WindowsCsharp.ComLease lease = Acquire();");
        w.line("nint self = lease.Handle;");
        if specialized {
            w.line("s_remove(self, key);");
        } else {
            write_map_remove(w, map, "self");
        }
        w.close();
        w.line("");

        // Clear at slot 12.
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("public void Clear()");
        w.line("using WindowsCsharp.ComLease lease = Acquire();");
        w.line("nint self = lease.Handle;");
        w.line(
            "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)self)[12])(self));",
        );
        w.close();
        w.line("");
    }

    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public Enumerator GetEnumerator()");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    if specialized {
        w.line("Guid iid = s_iterableIid;");
    } else {
        w.line("Guid iid = ComputeIterableIid();");
    }
    w.line("nint iterable;");
    w.line("WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(self, &iid, &iterable));");
    w.open("try");
    w.line("nint iterator;");
    w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)iterable)[6])(iterable, &iterator));");
    w.line("return new Enumerator(iterator);");
    w.close();
    w.open("finally");
    w.line("_ = WindowsCsharp.Com.Release(iterable);");
    w.close();
    w.close();
    w.line("");

    w.open("public sealed class Enumerator : WindowsCsharp.ComObject");
    w.line("private bool _started;");
    if specialized {
        w.line("internal Enumerator(nint self) : base(self, s_iteratorIid) {}");
    } else {
        w.line("internal Enumerator(nint self) : base(self, ComputeIteratorIid()) {}");
    }
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public bool MoveNext()");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    w.open("if (!_started)");
    w.line("_started = true;");
    w.line("byte current;");
    w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, int>)(*(void***)self)[7])(self, &current));");
    w.line("return current != 0;");
    w.close();
    w.line("byte moved;");
    w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, int>)(*(void***)self)[8])(self, &moved));");
    w.line("return moved != 0;");
    w.close();
    w.line("");
    w.open("public Entry Current");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("get");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("nint self = lease.Handle;");
    w.line("nint pair;");
    w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[6])(self, &pair));");
    w.open("try");
    if specialized {
        w.line("return s_entry(pair);");
    } else {
        write_map_entry(w, map, "pair");
    }
    w.close();
    w.open("finally");
    w.line("_ = WindowsCsharp.Com.Release(pair);");
    w.close();
    w.close();
    w.close();
    w.close();
    w.line("");

    w.open("public readonly struct Entry : IDisposable");
    w.line("public K Key { get; }");
    w.line("public V Value { get; }");
    w.line("");
    w.open("internal Entry(K key, V value)");
    w.line("Key = key;");
    w.line("Value = value;");
    w.close();
    w.line("");
    w.open("public void Dispose()");
    w.line("object? keyObject = Key;");
    w.open("if (keyObject is WindowsCsharp.ComObject key)");
    w.line("key.Dispose();");
    w.close();
    w.line("object? valueObject = Value;");
    w.open("if (valueObject is WindowsCsharp.ComObject value)");
    w.line("value.Dispose();");
    w.close();
    w.close();
    w.close();
    w.line("");

    // The generic cast uses its own type parameter name to avoid shadowing `K`/`V`.
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface>");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("return WindowsCsharp.Com.As<TInterface>(lease.Handle, lease.TrustedAgile);");
    w.close();
    w.line("");
    write_map_borrowed(w, map, mutable, specialized);

    if specialized {
        write_map_specialized_helpers(w, map, mutable);
    }

    w.close();
}

fn write_map_borrowed(w: &mut Writer, map: &Map, mutable: bool, specialized: bool) {
    w.line("public delegate void BorrowAction(Borrowed value);");
    w.line("public delegate TResult BorrowFunc<TResult>(Borrowed value);");
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public void Borrow(BorrowAction action)");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("action(new Borrowed(lease.Handle));");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public TResult Borrow<TResult>(BorrowFunc<TResult> action)");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("return action(new Borrowed(lease.Handle));");
    w.close();
    w.line("");
    w.open("public readonly ref struct Borrowed");
    w.line("private readonly nint _this;");
    w.line("internal Borrowed(nint self) => _this = self;");
    w.line("");
    w.open("public uint Count");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("get");
    w.line("uint value;");
    w.line("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int>)(*(void***)_this)[7])(_this, &value));");
    w.line("return value;");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public V Lookup(K key)");
    if specialized {
        w.line("return s_lookup(_this, key);");
    } else {
        write_map_lookup(w, map, "_this");
    }
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public bool HasKey(K key)");
    if specialized {
        w.line("return s_hasKey(_this, key);");
    } else {
        write_map_has_key(w, map, "_this");
    }
    w.close();
    if mutable {
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("public bool Insert(K key, V value)");
        if specialized {
            w.line("return s_insert(_this, key, value);");
        } else {
            write_map_insert(w, map, "_this");
        }
        w.close();
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("public void Remove(K key)");
        if specialized {
            w.line("s_remove(_this, key);");
        } else {
            write_map_remove(w, map, "_this");
        }
        w.close();
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.line("public void Clear() => WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)_this)[12])(_this));");
    }
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line("public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface> => WindowsCsharp.Com.As<TInterface>(_this, false);");
    w.close();
}

fn write_map_specialized_helpers(w: &mut Writer, map: &Map, mutable: bool) {
    for (index, item) in map.instantiations.iter().enumerate() {
        w.line("");
        w.open(&format!("private static V Lookup{index}(nint self, K key)"));
        write_map_specialized_lookup(w, item, "self");
        w.close();
        w.line("");
        w.open(&format!(
            "private static bool HasKey{index}(nint self, K key)"
        ));
        write_map_specialized_has_key(w, &item.key, "self");
        w.close();
        if mutable {
            w.line("");
            w.open(&format!(
                "private static bool Insert{index}(nint self, K key, V value)"
            ));
            write_map_specialized_insert(w, item, "self");
            w.close();
            w.line("");
            w.open(&format!(
                "private static void Remove{index}(nint self, K key)"
            ));
            write_map_specialized_remove(w, &item.key, "self");
            w.close();
        }
        w.line("");
        w.open(&format!("private static Entry ReadEntry{index}(nint pair)"));
        write_map_specialized_entry(w, item, "pair");
        w.close();
    }
}

fn write_map_specialized_lookup(w: &mut Writer, item: &MapInstantiation, self_: &str) {
    let key = &item.key;
    let surface = key.collection_surface();
    if key.is_object() {
        w.line("object? boxedKey = key;");
        w.open("if (boxedKey is null)");
        write_map_specialized_lookup_result(w, &item.value, self_, "nint", "0", "Null");
        w.close();
        w.line(&format!("{surface} objectKey = ({surface})boxedKey;"));
        w.line("using WindowsCsharp.ComLease keyLease = objectKey.Acquire();");
        write_map_specialized_lookup_result(w, &item.value, self_, "nint", "keyLease.Handle", "");
    } else if matches!(key, CsType::String) {
        w.line("string? text = Unsafe.As<K, string?>(ref key);");
        w.open("fixed (char* chars = text)");
        w.line("WindowsCsharp.Interop.HstringHeader header;");
        w.line("nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);");
        write_map_specialized_lookup_result(w, &item.value, self_, "nint", "hstring", "");
        w.close();
    } else {
        let abi = key.abi_in();
        let converted = key.surface_to_abi(&format!("Unsafe.As<K, {surface}>(ref key)"));
        w.line(&format!("{abi} abiKey = {converted};"));
        write_map_specialized_lookup_result(w, &item.value, self_, &abi, "abiKey", "");
    }
}

fn write_map_specialized_lookup_result(
    w: &mut Writer,
    value: &CsType,
    self_: &str,
    key_abi: &str,
    key_arg: &str,
    suffix: &str,
) {
    let surface = value.collection_surface();
    let abi = value.abi_in();
    w.line(&format!("{abi} result{suffix};"));
    w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {key_abi}, {abi}*, int>)(*(void***){self_})[6])({self_}, {key_arg}, &result{suffix}));"));
    w.line(&format!(
        "{surface} value{suffix} = {};",
        value.abi_to_surface(&format!("result{suffix}"))
    ));
    if value.is_object() {
        w.line(&format!("return (V)(object?)value{suffix}!;"));
    } else if matches!(value, CsType::String) {
        w.line(&format!("return (V)(object)value{suffix};"));
    } else {
        w.line(&format!(
            "return Unsafe.As<{surface}, V>(ref value{suffix});"
        ));
    }
}

fn write_map_specialized_has_key(w: &mut Writer, key: &CsType, self_: &str) {
    let surface = key.collection_surface();
    if key.is_object() {
        w.line("object? boxedKey = key;");
        w.line("using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);");
        w.line("byte result;");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, byte*, int>)(*(void***){self_})[8])({self_}, keyLease.Handle, &result));"));
        w.line("return result != 0;");
    } else if matches!(key, CsType::String) {
        w.line("string? text = Unsafe.As<K, string?>(ref key);");
        w.open("fixed (char* chars = text)");
        w.line("WindowsCsharp.Interop.HstringHeader header;");
        w.line("nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);");
        w.line("byte result;");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, byte*, int>)(*(void***){self_})[8])({self_}, hstring, &result));"));
        w.line("return result != 0;");
        w.close();
    } else {
        let abi = key.abi_in();
        let converted = key.surface_to_abi(&format!("Unsafe.As<K, {surface}>(ref key)"));
        w.line(&format!("{abi} abiKey = {converted};"));
        w.line("byte result;");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {abi}, byte*, int>)(*(void***){self_})[8])({self_}, abiKey, &result));"));
        w.line("return result != 0;");
    }
}

fn write_map_specialized_insert(w: &mut Writer, item: &MapInstantiation, self_: &str) {
    let key = &item.key;
    let surface = key.collection_surface();
    if key.is_object() {
        w.line("object? boxedKey = key;");
        w.line("using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);");
        write_map_specialized_insert_value(w, &item.value, self_, "nint", "keyLease.Handle");
    } else if matches!(key, CsType::String) {
        w.line("string? text = Unsafe.As<K, string?>(ref key);");
        w.open("fixed (char* chars = text)");
        w.line("WindowsCsharp.Interop.HstringHeader header;");
        w.line("nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);");
        write_map_specialized_insert_value(w, &item.value, self_, "nint", "hstring");
        w.close();
    } else {
        let abi = key.abi_in();
        let converted = key.surface_to_abi(&format!("Unsafe.As<K, {surface}>(ref key)"));
        w.line(&format!("{abi} abiKey = {converted};"));
        write_map_specialized_insert_value(w, &item.value, self_, &abi, "abiKey");
    }
}

fn write_map_specialized_insert_value(
    w: &mut Writer,
    value: &CsType,
    self_: &str,
    key_abi: &str,
    key_arg: &str,
) {
    let surface = value.collection_surface();
    if value.is_object() {
        w.line("object? boxedValue = value;");
        w.open("if (boxedValue is null)");
        w.line("byte replaced;");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {key_abi}, nint, byte*, int>)(*(void***){self_})[10])({self_}, {key_arg}, 0, &replaced));"));
        w.line("return replaced != 0;");
        w.close();
        w.line(&format!("{surface} objectValue = ({surface})boxedValue;"));
        w.line("using WindowsCsharp.ComLease valueLease = objectValue.Acquire();");
        w.line("byte replacedValue;");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {key_abi}, nint, byte*, int>)(*(void***){self_})[10])({self_}, {key_arg}, valueLease.Handle, &replacedValue));"));
        w.line("return replacedValue != 0;");
    } else if matches!(value, CsType::String) {
        w.line("string? textValue = (string?)(object?)value;");
        w.open("fixed (char* valueChars = textValue)");
        w.line("WindowsCsharp.Interop.HstringHeader valueHeader;");
        w.line("nint abiValue = WindowsCsharp.Interop.CreateStringReference((ushort*)valueChars, (uint)(textValue?.Length ?? 0), &valueHeader);");
        w.line("byte replaced;");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {key_abi}, nint, byte*, int>)(*(void***){self_})[10])({self_}, {key_arg}, abiValue, &replaced));"));
        w.line("return replaced != 0;");
        w.close();
    } else {
        let abi = value.abi_in();
        let converted = value.surface_to_abi(&format!("Unsafe.As<V, {surface}>(ref value)"));
        w.line(&format!("{abi} abiValue = {converted};"));
        w.line("byte replaced;");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {key_abi}, {abi}, byte*, int>)(*(void***){self_})[10])({self_}, {key_arg}, abiValue, &replaced));"));
        w.line("return replaced != 0;");
    }
}

fn write_map_specialized_remove(w: &mut Writer, key: &CsType, self_: &str) {
    let surface = key.collection_surface();
    if key.is_object() {
        w.line("object? boxedKey = key;");
        w.line("using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***){self_})[11])({self_}, keyLease.Handle));"));
    } else if matches!(key, CsType::String) {
        w.line("string? text = Unsafe.As<K, string?>(ref key);");
        w.open("fixed (char* chars = text)");
        w.line("WindowsCsharp.Interop.HstringHeader header;");
        w.line("nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);");
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***){self_})[11])({self_}, hstring));"));
        w.close();
    } else {
        let abi = key.abi_in();
        let converted = key.surface_to_abi(&format!("Unsafe.As<K, {surface}>(ref key)"));
        w.line(&format!("{abi} abiKey = {converted};"));
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {abi}, int>)(*(void***){self_})[11])({self_}, abiKey));"));
    }
}

fn write_map_specialized_entry(w: &mut Writer, item: &MapInstantiation, pair: &str) {
    let key = &item.key;
    let value = &item.value;
    let surface = key.collection_surface();
    let abi = key.abi_in();
    let value_surface = value.collection_surface();
    let value_abi = value.abi_in();
    w.line(&format!("{abi} abiKey;"));
    w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {abi}*, int>)(*(void***){pair})[6])({pair}, &abiKey));"));
    w.line(&format!(
        "{surface} keyValue = {};",
        key.abi_to_surface("abiKey")
    ));
    if key.is_object() {
        w.line("K key = (K)(object?)keyValue!;");
        w.open("try");
    } else if matches!(key, CsType::String) {
        w.line("K key = (K)(object)keyValue;");
    } else {
        w.line(&format!("K key = Unsafe.As<{surface}, K>(ref keyValue);"));
    }
    w.line(&format!("{value_abi} abiValue;"));
    w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {value_abi}*, int>)(*(void***){pair})[7])({pair}, &abiValue));"));
    w.line(&format!(
        "{value_surface} valueResult = {};",
        value.abi_to_surface("abiValue")
    ));
    if value.is_object() {
        w.line("V value = (V)(object?)valueResult!;");
    } else if matches!(value, CsType::String) {
        w.line("V value = (V)(object)valueResult;");
    } else {
        w.line(&format!(
            "V value = Unsafe.As<{value_surface}, V>(ref valueResult);"
        ));
    }
    w.line("return new Entry(key, value);");
    if key.is_object() {
        w.close();
        w.open("catch");
        w.line("object? keyObject = key;");
        w.open("if (keyObject is WindowsCsharp.ComObject owner)");
        w.line("owner.Dispose();");
        w.close();
        w.line("throw;");
        w.close();
    }
}

fn map_key_types(map: &Map) -> Vec<&CsType> {
    let mut result = Vec::new();
    for item in &map.instantiations {
        let key = &item.key;
        if !result
            .iter()
            .any(|existing: &&CsType| existing.collection_surface() == key.collection_surface())
        {
            result.push(key);
        }
    }
    result
}

fn map_value_types<'a>(map: &'a Map, key: &CsType) -> Vec<&'a CsType> {
    let mut result = Vec::new();
    for item in &map.instantiations {
        if item.key.collection_surface() != key.collection_surface() {
            continue;
        }
        let value = &item.value;
        if !result
            .iter()
            .any(|existing: &&CsType| existing.collection_surface() == value.collection_surface())
        {
            result.push(value);
        }
    }
    result
}

fn write_map_lookup(w: &mut Writer, map: &Map, self_: &str) {
    for key in map_key_types(map) {
        let surface = key.collection_surface();
        w.open(&format!("if (typeof(K) == typeof({surface}))"));
        if key.is_object() {
            w.line("object? boxedKey = key;");
            w.open("if (boxedKey is null)");
            write_map_lookup_value(w, map, key, self_, "nint", "0");
            w.close();
            w.line(&format!("{surface} objectKey = ({surface})boxedKey;"));
            w.line("using WindowsCsharp.ComLease keyLease = objectKey.Acquire();");
            write_map_lookup_value(w, map, key, self_, "nint", "keyLease.Handle");
        } else if matches!(key, CsType::String) {
            w.line("string? text = Unsafe.As<K, string?>(ref key);");
            w.open("fixed (char* chars = text)");
            w.line("WindowsCsharp.Interop.HstringHeader header;");
            w.line("nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);");
            write_map_lookup_value(w, map, key, self_, "nint", "hstring");
            w.close();
        } else {
            let abi = key.abi_in();
            let converted = key.surface_to_abi(&format!("Unsafe.As<K, {surface}>(ref key)"));
            w.line(&format!("{abi} abiKey = {converted};"));
            write_map_lookup_value(w, map, key, self_, &abi, "abiKey");
        }
        w.close();
    }
    w.line("throw new NotSupportedException();");
}

fn write_map_lookup_value(
    w: &mut Writer,
    map: &Map,
    key: &CsType,
    self_: &str,
    key_abi: &str,
    key_arg: &str,
) {
    for value in map_value_types(map, key) {
        let surface = value.collection_surface();
        let abi = value.abi_in();
        w.open(&format!("if (typeof(V) == typeof({surface}))"));
        w.line(&format!("{abi} result;"));
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {key_abi}, {abi}*, int>)(*(void***){self_})[6])({self_}, {key_arg}, &result));"));
        w.line(&format!(
            "{surface} value = {};",
            value.abi_to_surface("result")
        ));
        if value.is_object() {
            w.line("return (V)(object?)value!;");
        } else if matches!(value, CsType::String) {
            w.line("return (V)(object)value;");
        } else {
            w.line(&format!("return Unsafe.As<{surface}, V>(ref value);"));
        }
        w.close();
    }
    w.line("throw new NotSupportedException();");
}

fn write_map_has_key(w: &mut Writer, map: &Map, self_: &str) {
    for key in map_key_types(map) {
        let surface = key.collection_surface();
        w.open(&format!("if (typeof(K) == typeof({surface}))"));
        if key.is_object() {
            w.line("object? boxedKey = key;");
            w.line("using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);");
            w.line("byte result;");
            w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, byte*, int>)(*(void***){self_})[8])({self_}, keyLease.Handle, &result));"));
            w.line("return result != 0;");
        } else if matches!(key, CsType::String) {
            w.line("string? text = Unsafe.As<K, string?>(ref key);");
            w.open("fixed (char* chars = text)");
            w.line("WindowsCsharp.Interop.HstringHeader header;");
            w.line("nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);");
            w.line("byte result;");
            w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, byte*, int>)(*(void***){self_})[8])({self_}, hstring, &result));"));
            w.line("return result != 0;");
            w.close();
        } else {
            let abi = key.abi_in();
            let converted = key.surface_to_abi(&format!("Unsafe.As<K, {surface}>(ref key)"));
            w.line(&format!("{abi} abiKey = {converted};"));
            w.line("byte result;");
            w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {abi}, byte*, int>)(*(void***){self_})[8])({self_}, abiKey, &result));"));
            w.line("return result != 0;");
        }
        w.close();
    }
    w.line("throw new NotSupportedException();");
}

fn write_map_insert(w: &mut Writer, map: &Map, self_: &str) {
    for key in map_key_types(map) {
        let surface = key.collection_surface();
        w.open(&format!("if (typeof(K) == typeof({surface}))"));
        if key.is_object() {
            w.line("object? boxedKey = key;");
            w.line("using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);");
            write_map_insert_value(w, map, key, self_, "nint", "keyLease.Handle");
        } else if matches!(key, CsType::String) {
            w.line("string? text = Unsafe.As<K, string?>(ref key);");
            w.open("fixed (char* chars = text)");
            w.line("WindowsCsharp.Interop.HstringHeader header;");
            w.line("nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);");
            write_map_insert_value(w, map, key, self_, "nint", "hstring");
            w.close();
        } else {
            let abi = key.abi_in();
            let converted = key.surface_to_abi(&format!("Unsafe.As<K, {surface}>(ref key)"));
            w.line(&format!("{abi} abiKey = {converted};"));
            write_map_insert_value(w, map, key, self_, &abi, "abiKey");
        }
        w.close();
    }
    w.line("throw new NotSupportedException();");
}

fn write_map_insert_value(
    w: &mut Writer,
    map: &Map,
    key: &CsType,
    self_: &str,
    key_abi: &str,
    key_arg: &str,
) {
    for value in map_value_types(map, key) {
        let surface = value.collection_surface();
        w.open(&format!("if (typeof(V) == typeof({surface}))"));
        if value.is_object() {
            w.line("object? boxedValue = value;");
            w.open("if (boxedValue is null)");
            w.line("byte replaced;");
            w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {key_abi}, nint, byte*, int>)(*(void***){self_})[10])({self_}, {key_arg}, 0, &replaced));"));
            w.line("return replaced != 0;");
            w.close();
            w.line(&format!("{surface} objectValue = ({surface})boxedValue;"));
            w.line("using WindowsCsharp.ComLease valueLease = objectValue.Acquire();");
            w.line("byte replacedValue;");
            w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {key_abi}, nint, byte*, int>)(*(void***){self_})[10])({self_}, {key_arg}, valueLease.Handle, &replacedValue));"));
            w.line("return replacedValue != 0;");
        } else if matches!(value, CsType::String) {
            w.line("string? textValue = (string?)(object?)value;");
            w.open("fixed (char* valueChars = textValue)");
            w.line("WindowsCsharp.Interop.HstringHeader valueHeader;");
            w.line("nint abiValue = WindowsCsharp.Interop.CreateStringReference((ushort*)valueChars, (uint)(textValue?.Length ?? 0), &valueHeader);");
            w.line("byte replaced;");
            w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {key_abi}, nint, byte*, int>)(*(void***){self_})[10])({self_}, {key_arg}, abiValue, &replaced));"));
            w.line("return replaced != 0;");
            w.close();
        } else {
            let abi = value.abi_in();
            let converted = value.surface_to_abi(&format!("Unsafe.As<V, {surface}>(ref value)"));
            w.line(&format!("{abi} abiValue = {converted};"));
            w.line("byte replaced;");
            w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {key_abi}, {abi}, byte*, int>)(*(void***){self_})[10])({self_}, {key_arg}, abiValue, &replaced));"));
            w.line("return replaced != 0;");
        }
        w.close();
    }
    w.line("throw new NotSupportedException();");
}

fn write_map_remove(w: &mut Writer, map: &Map, self_: &str) {
    for key in map_key_types(map) {
        let surface = key.collection_surface();
        w.open(&format!("if (typeof(K) == typeof({surface}))"));
        if key.is_object() {
            w.line("object? boxedKey = key;");
            w.line("using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);");
            w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***){self_})[11])({self_}, keyLease.Handle));"));
            w.line("return;");
        } else if matches!(key, CsType::String) {
            w.line("string? text = Unsafe.As<K, string?>(ref key);");
            w.open("fixed (char* chars = text)");
            w.line("WindowsCsharp.Interop.HstringHeader header;");
            w.line("nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);");
            w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***){self_})[11])({self_}, hstring));"));
            w.line("return;");
            w.close();
        } else {
            let abi = key.abi_in();
            let converted = key.surface_to_abi(&format!("Unsafe.As<K, {surface}>(ref key)"));
            w.line(&format!("{abi} abiKey = {converted};"));
            w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {abi}, int>)(*(void***){self_})[11])({self_}, abiKey));"));
            w.line("return;");
        }
        w.close();
    }
    w.line("throw new NotSupportedException();");
}

fn write_map_entry(w: &mut Writer, map: &Map, pair: &str) {
    for item in &map.instantiations {
        let key = &item.key;
        let value = &item.value;
        let surface = key.collection_surface();
        let abi = key.abi_in();
        let value_surface = value.collection_surface();
        let value_abi = value.abi_in();
        w.open(&format!(
            "if (typeof(K) == typeof({surface}) && typeof(V) == typeof({value_surface}))"
        ));
        w.line(&format!("{abi} abiKey;"));
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {abi}*, int>)(*(void***){pair})[6])({pair}, &abiKey));"));
        w.line(&format!(
            "{surface} keyValue = {};",
            key.abi_to_surface("abiKey")
        ));
        if key.is_object() {
            w.line("K key = (K)(object?)keyValue!;");
            w.open("try");
        } else if matches!(key, CsType::String) {
            w.line("K key = (K)(object)keyValue;");
        } else {
            w.line(&format!("K key = Unsafe.As<{surface}, K>(ref keyValue);"));
        }
        w.line(&format!("{value_abi} abiValue;"));
        w.line(&format!("WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {value_abi}*, int>)(*(void***){pair})[7])({pair}, &abiValue));"));
        w.line(&format!(
            "{value_surface} valueResult = {};",
            value.abi_to_surface("abiValue")
        ));
        if value.is_object() {
            w.line("V value = (V)(object?)valueResult!;");
        } else if matches!(value, CsType::String) {
            w.line("V value = (V)(object)valueResult;");
        } else {
            w.line(&format!(
                "V value = Unsafe.As<{value_surface}, V>(ref valueResult);"
            ));
        }
        w.line("return new Entry(key, value);");
        if key.is_object() {
            w.close();
            w.open("catch");
            w.line("object? keyObject = key;");
            w.open("if (keyObject is WindowsCsharp.ComObject owner)");
            w.line("owner.Dispose();");
            w.close();
            w.line("throw;");
            w.close();
        }
        w.close();
    }
    w.line("throw new NotSupportedException();");
}

/// Emits the generic `As<T>()` cast: a `QueryInterface` for `T.Iid` that returns the projected `T`.
/// This is the C# analogue of Rust's `cast::<T>()` and CsWinRT's `As<T>()` - it works for any
/// projected interface (or class) and issues a fresh QueryInterface on every call. The caller owns
/// the returned reference and disposes it.
fn write_generic_cast(w: &mut Writer, receiver: Receiver) {
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public T As<T>() where T : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T>");
    write_self(w, receiver);
    match receiver {
        Receiver::Owned => w.line("return WindowsCsharp.Com.As<T>(self, lease.TrustedAgile);"),
        _ => w.line("return WindowsCsharp.Com.As<T>(self, false);"),
    }
    w.close();
}

/// Emits each projected member (property, method, or event) through `receiver`, which selects the
/// member's visibility and how `self` is acquired.
fn write_members(w: &mut Writer, members: &[Member], receiver: Receiver) {
    for member in members {
        w.line("");
        match member {
            Member::Property {
                name,
                ty,
                get_slot,
                put_slot,
            } => write_property(w, name, ty, *get_slot, *put_slot, receiver, None),
            Member::Method {
                name,
                params,
                ret,
                slot,
                abi,
            } => write_method(w, name, params, ret.as_ref(), *slot, *abi, receiver, None),
            Member::Event {
                name,
                delegate,
                add_slot,
                remove_slot,
            } => write_event(w, name, delegate, *add_slot, *remove_slot, receiver),
        }
    }
}

fn write_borrowed(w: &mut Writer, members: &[Member], forwarders: &[Forwarder]) {
    w.line("public delegate void BorrowAction(Borrowed value);");
    w.line("public delegate TResult BorrowFunc<TResult>(Borrowed value);");
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public void Borrow(BorrowAction action)");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("action(new Borrowed(lease.Handle));");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public TResult Borrow<TResult>(BorrowFunc<TResult> action)");
    w.line("using WindowsCsharp.ComLease lease = Acquire();");
    w.line("return action(new Borrowed(lease.Handle));");
    w.close();
    w.line("");
    w.open("public readonly ref struct Borrowed");
    w.line("private readonly nint _this;");
    w.line("internal Borrowed(nint self) => _this = self;");
    w.line("public bool IsNull => _this == 0;");
    write_members(w, members, Receiver::Borrowed);
    write_forwarders(w, forwarders, Receiver::Borrowed);
    w.line("");
    write_generic_cast(w, Receiver::Borrowed);
    w.close();
}

fn write_borrow_as(
    w: &mut Writer,
    current: &str,
    compatible: &[String],
    projected_objects: &HashSet<String>,
) {
    for target in compatible {
        if target == current || !projected_objects.contains(target) {
            continue;
        }
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open(&format!(
            "public void BorrowAs({target}.BorrowAction action)"
        ));
        w.line("using WindowsCsharp.ComLease source = Acquire();");
        w.line(&format!(
            "using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, {target}.Iid);"
        ));
        w.line(&format!("action(new {target}.Borrowed(lease.Handle));"));
        w.close();
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open(&format!(
            "public TResult BorrowAs<TResult>({target}.BorrowFunc<TResult> action)"
        ));
        w.line("using WindowsCsharp.ComLease source = Acquire();");
        w.line(&format!(
            "using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, {target}.Iid);"
        ));
        w.line(&format!(
            "return action(new {target}.Borrowed(lease.Handle));"
        ));
        w.close();
    }
}

fn write_self(w: &mut Writer, receiver: Receiver) {
    write_source_lease(w, receiver, "lease");
    write_source_self(w, receiver, "lease");
}

/// Emits a WinRT event as an explicit `Add{name}(handler) -> long` / `Remove{name}(long token)`
/// pair, plus a `{name}(handler) -> WindowsCsharp.EventRevoker` RAII form beside them. The raw
/// accessors expose the registration token directly, so subscribing and unsubscribing add no
/// per-object bookkeeping or allocation - the zero-alloc fast path. The revoker form matches
/// windows-rs's `EventRevoker` and C++/WinRT's `auto_revoke`: it AddRefs the source and captures the
/// token and the remove slot, so `using var r = widget.Changed(handler);` unsubscribes and releases
/// the source at scope exit. The handler is a projected delegate class over one interface pointer;
/// the token is the blittable `i64` the ABI returns.
fn write_event(
    w: &mut Writer,
    name: &str,
    delegate: &CsType,
    add_slot: usize,
    remove_slot: usize,
    receiver: Receiver,
) {
    let vis = receiver.vis();
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open(&format!(
        "{vis} long Add{name}({} handler)",
        delegate.surface()
    ));
    write_source_lease(w, receiver, "sourceLease");
    w.line("using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);");
    write_source_self(w, receiver, "sourceLease");
    w.line("long token;");
    w.line(&format!(
        "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[{add_slot}])(self, handlerLease.Handle, &token));"
    ));
    w.line("return token;");
    w.close();

    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open(&format!("{vis} void Remove{name}(long token)"));
    write_self(w, receiver);
    w.line(&format!(
        "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long, int>)(*(void***)self)[{remove_slot}])(self, token));"
    ));
    w.close();

    // Allocate the revoker before registering so an allocation failure cannot strand a token. Then
    // register through the raw `Add{name}`, AddRef the source so the revoker owns a reference
    // independent of this projected struct, and attach the token and remove thunk. If `Add{name}`
    // throws, the source is never AddRef'd and the unattached revoker owns nothing.
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open(&format!(
        "{vis} WindowsCsharp.EventRevoker {name}({} handler)",
        delegate.surface()
    ));
    w.line("WindowsCsharp.EventRevoker revoker = new WindowsCsharp.EventRevoker();");
    write_source_lease(w, receiver, "sourceLease");
    w.line("using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);");
    write_source_self(w, receiver, "sourceLease");
    w.line("long token;");
    w.line(&format!(
        "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[{add_slot}])(self, handlerLease.Handle, &token));"
    ));
    w.line("_ = WindowsCsharp.Com.AddRef(self);");
    w.open("try");
    let trusted_agile = if receiver.is_borrowed() {
        "false"
    } else {
        "sourceLease.TrustedAgile"
    };
    w.line(&format!(
        "revoker.Attach(self, {trusted_agile}, token, (delegate* unmanaged<nint, long, int>)(*(void***)self)[{remove_slot}]);"
    ));
    w.close();
    w.open("catch");
    w.line(&format!(
        "_ = ((delegate* unmanaged<nint, long, int>)(*(void***)self)[{remove_slot}])(self, token);"
    ));
    w.line("_ = WindowsCsharp.Com.Release(self);");
    w.line("throw;");
    w.close();
    w.line("return revoker;");
    w.close();
}

fn write_activating_ctor(w: &mut Writer, class: &Class) {
    w.line(&format!(
        "public {}() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, \"{}\", Iid), Iid) {{}}",
        class.name,
        class.class_id()
    ));
}

fn write_property(
    w: &mut Writer,
    name: &str,
    ty: &CsType,
    get_slot: Option<usize>,
    put_slot: Option<usize>,
    receiver: Receiver,
    forward_iid: Option<&str>,
) {
    w.open(&format!("{} {} {name}", receiver.vis(), ty.surface()));

    if let Some(slot) = get_slot {
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("get");
        write_forward_self(w, receiver, forward_iid);
        if let CsType::String = ty {
            write_string_get(w, slot);
        } else if ty.owned_struct_abi().is_some() {
            let abi = ty.abi_in();
            w.line(&format!("{abi} value = default;"));
            w.open("try");
            w.line(&format!(
                "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {abi}*, int>)(*(void***)self)[{slot}])(self, &value));"
            ));
            w.line("return value.ToSurface();");
            w.close();
            w.open("finally");
            w.line("value.Dispose();");
            w.close();
        } else if let Some((value, _)) = ty.reference() {
            w.line("nint reference;");
            w.line(&format!(
                "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[{slot}])(self, &reference));"
            ));
            w.line(&format!(
                "return WindowsCsharp.ReferenceBox<{}>.Unbox(reference);",
                value.surface()
            ));
        } else {
            // Every non-string getter has the same shape: declare an ABI-typed local, fill it
            // through the out-pointer slot, and wrap it back to the surface type (identity for a
            // scalar or struct, a cast for an enum, ownership of the `+1` for an object).
            let abi = ty.abi_in();
            w.line(&format!("{abi} value;"));
            w.line(&format!(
                "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {abi}*, int>)(*(void***)self)[{slot}])(self, &value));"
            ));
            w.line(&format!("return {};", ty.abi_to_surface("value")));
        }
        close_forward_self(w, forward_iid);
        w.close();
    }

    if let Some(slot) = put_slot {
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("set");
        write_forward_self(w, receiver, forward_iid);
        if let CsType::String = ty {
            write_string_put(w, slot, "value");
        } else if ty.owned_struct_abi().is_some() {
            let abi = ty.abi_in();
            w.line(&format!(
                "{abi} abi = {};",
                ty.surface_to_owned_abi("value")
            ));
            w.open("try");
            w.line(&format!(
                "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {abi}, int>)(*(void***)self)[{slot}])(self, abi));"
            ));
            w.close();
            w.open("finally");
            w.line("abi.Dispose();");
            w.close();
        } else if let Some((reference, iid)) = ty.reference() {
            w.line(&format!(
                "using WindowsCsharp.ReferenceBox<{}> box = new WindowsCsharp.ReferenceBox<{}>(value, {});",
                reference.surface(),
                reference.surface(),
                iid.to_cs()
            ));
            w.line(&format!(
                "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[{slot}])(self, box.Handle));"
            ));
        } else if ty.is_object() {
            w.line("using WindowsCsharp.ComLease valueLease = WindowsCsharp.ComLease.From(value);");
            let abi = ty.abi_in();
            w.line(&format!(
                "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {abi}, int>)(*(void***)self)[{slot}])(self, valueLease.Handle));"
            ));
        } else {
            // Every non-string setter is the mirror: convert the surface value to the ABI (identity,
            // an enum cast, or a borrowed interface pointer) and pass it by value to the slot.
            let abi = ty.abi_in();
            w.line(&format!(
                "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, {abi}, int>)(*(void***)self)[{slot}])(self, {}));",
                ty.surface_to_abi("value")
            ));
        }
        close_forward_self(w, forward_iid);
        w.close();
    }

    fn write_forward_self(w: &mut Writer, receiver: Receiver, forward_iid: Option<&str>) {
        let Some(iid) = forward_iid else {
            write_self(w, receiver);
            return;
        };
        if receiver.is_borrowed() {
            write_self(w, Receiver::Borrowed);
            w.line("nint source = self;");
        } else {
            w.line("using WindowsCsharp.ComLease lease = Acquire();");
            w.line("nint source = lease.Handle;");
            w.line("nint self;");
        }
        w.line(&format!("Guid iid = {iid};"));
        w.line("WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));");
        w.open("try");
    }

    fn close_forward_self(w: &mut Writer, forward_iid: Option<&str>) {
        if forward_iid.is_some() {
            w.close();
            w.open("finally");
            w.line("_ = WindowsCsharp.Com.Release(self);");
            w.close();
        }
    }

    w.close();
}

#[allow(clippy::too_many_arguments)]
fn write_method(
    w: &mut Writer,
    name: &str,
    params: &[Param],
    ret: Option<&CsType>,
    slot: usize,
    abi: MethodAbi,
    receiver: Receiver,
    forward_iid: Option<&str>,
) {
    write_method_core(
        w,
        name,
        params,
        ret,
        slot,
        abi,
        receiver,
        forward_iid,
        false,
    );
    if matches!(abi, MethodAbi::WinRt)
        && params
            .iter()
            .any(|param| compatible_object_name(&param.ty).is_some())
    {
        w.line("");
        write_method_core(w, name, params, ret, slot, abi, receiver, forward_iid, true);
    }
}

fn compatible_object_name(ty: &CsType) -> Option<&str> {
    match ty {
        CsType::Object { name } if !name.contains('<') => Some(name),
        _ => None,
    }
}

#[allow(clippy::too_many_arguments)]
fn write_method_core(
    w: &mut Writer,
    name: &str,
    params: &[Param],
    ret: Option<&CsType>,
    slot: usize,
    abi: MethodAbi,
    receiver: Receiver,
    forward_iid: Option<&str>,
    compatible: bool,
) {
    let roles = if matches!(abi, MethodAbi::Direct) {
        param_roles(params)
    } else {
        vec![ParamRole::Value; params.len()]
    };
    let out_object = matches!(abi, MethodAbi::Direct)
        .then(|| {
            params.iter().enumerate().find_map(|(index, param)| {
                if let CsType::ComOut { name } = &param.ty {
                    Some((index, param.name.as_str(), name.as_str()))
                } else {
                    None
                }
            })
        })
        .flatten();
    let surface_params = params
        .iter()
        .enumerate()
        .filter(|(index, _)| {
            !matches!(roles[*index], ParamRole::BufferCount { .. })
                && out_object.is_none_or(|(out_index, _, _)| *index != out_index)
        })
        .map(|(index, param)| {
            if compatible && compatible_object_name(&param.ty).is_some() {
                format!("T{index}? {}", param.name)
            } else {
                match roles[index] {
                    ParamRole::ScalarPointer { target, is_ref } => {
                        let keyword = if is_ref { "ref" } else { "out" };
                        format!("{keyword} {} {}", target.surface(), param.name)
                    }
                    ParamRole::Utf16String => {
                        let nullable = if param.optional { "?" } else { "" };
                        format!("string{nullable} {}", param.name)
                    }
                    ParamRole::Buffer { element, .. } => {
                        let span = if matches!(param.direction, Direction::Input) {
                            "ReadOnlySpan"
                        } else {
                            "Span"
                        };
                        format!("{span}<{}> {}", element.surface(), param.name)
                    }
                    ParamRole::Value => param.ty.parameter(&param.name),
                    ParamRole::BufferCount { .. } => unreachable!(),
                }
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    let ret_surface = out_object.map_or_else(
        || match ret {
            Some(CsType::HResult) | None => "void".to_string(),
            Some(ret) => ret.surface(),
        },
        |(_, _, object)| object.to_string(),
    );
    let generic_args = if compatible {
        let names = params
            .iter()
            .enumerate()
            .filter_map(|(index, param)| {
                compatible_object_name(&param.ty).map(|_| format!("T{index}"))
            })
            .collect::<Vec<_>>();
        format!("<{}>", names.join(", "))
    } else {
        String::new()
    };
    let constraints = if compatible {
        params
            .iter()
            .enumerate()
            .filter_map(|(index, param)| {
                compatible_object_name(&param.ty).map(|target| {
                    format!(
                        " where T{index} : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T{index}>, WindowsCsharp.IObjectParameter<{target}._Parameter>"
                    )
                })
            })
            .collect::<String>()
    } else {
        String::new()
    };

    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open(&format!(
        "{} {ret_surface} {name}{generic_args}({surface_params}){constraints}",
        receiver.vis(),
    ));
    if let Some(iid) = forward_iid {
        if receiver.is_borrowed() {
            write_self(w, Receiver::Borrowed);
            w.line("nint source = self;");
        } else {
            w.line("using WindowsCsharp.ComLease lease = Acquire();");
            w.line("nint source = lease.Handle;");
            w.line("nint self;");
        }
        w.line(&format!("Guid iid = {iid};"));
        w.line("WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));");
        w.open("try");
    } else {
        write_self(w, receiver);
    }

    if matches!(abi, MethodAbi::Direct) {
        write_direct_method_call(w, params, &roles, ret, slot);
        if forward_iid.is_some() {
            w.close();
            w.open("finally");
            w.line("_ = WindowsCsharp.Com.Release(self);");
            w.close();
        }
        w.close();
        return;
    }

    let ret_is_string = matches!(ret, Some(CsType::String));
    let ret_array = ret.and_then(CsType::array);
    let output_arrays: Vec<_> = params
        .iter()
        .enumerate()
        .filter_map(|(i, param)| {
            param
                .ty
                .array()
                .filter(|(_, output)| *output)
                .map(|(element, _)| (i, element))
        })
        .collect();
    for (i, element) in &output_arrays {
        w.line(&format!("uint _asize{i} = 0;"));
        w.line(&format!("{}* _adata{i} = null;", element.abi_in()));
    }
    let scalar_result = ret.filter(|ret| ret.array().is_none());
    if let Some(ret) = scalar_result
        && ret.owned_struct_abi().is_some()
    {
        w.line(&format!("{} result = default;", ret.abi_in()));
    }
    let result = scalar_result.map(|ret| {
        let declaration = if ret.owned_struct_abi().is_some() {
            String::new()
        } else {
            format!("{} result;", ret.abi_in())
        };
        (declaration, ret.abi_out(), "&result".to_string())
    });
    if let Some((element, _)) = ret_array {
        w.line("uint resultSize = 0;");
        w.line(&format!("{}* result = null;", element.abi_in()));
    }
    let has_cleanup = ret_array.is_some()
        || !output_arrays.is_empty()
        || ret.is_some_and(|ret| ret.owned_struct_abi().is_some());
    if has_cleanup {
        w.open("try");
    }
    if let Some((element, _)) = ret_array {
        let trailing = vec![
            ("uint*".to_string(), "&resultSize".to_string()),
            (format!("{}**", element.abi_in()), "&result".to_string()),
        ];
        write_marshalled_call(w, params, slot, &trailing, result, compatible, true);
    } else {
        write_marshalled_call(w, params, slot, &[], result, compatible, true);
    }

    fn write_direct_method_call(
        w: &mut Writer,
        params: &[Param],
        roles: &[ParamRole<'_>],
        ret: Option<&CsType>,
        slot: usize,
    ) {
        let out_object = params.iter().find_map(|param| {
            if let CsType::ComOut { name } = &param.ty {
                Some((param.name.as_str(), name.as_str()))
            } else {
                None
            }
        });
        if let Some((name, _)) = out_object {
            w.line(&format!("nint {name} = 0;"));
        }
        let owned_struct_params: Vec<_> = params
            .iter()
            .enumerate()
            .filter(|(i, param)| {
                matches!(roles[*i], ParamRole::Value) && param.ty.owned_struct_abi().is_some()
            })
            .collect();
        for (i, param) in params.iter().enumerate() {
            if let ParamRole::ScalarPointer { target, is_ref } = roles[i] {
                if is_ref {
                    w.line(&format!(
                        "{} _abi{i} = {};",
                        target.abi_in(),
                        target.surface_to_abi(&param.name)
                    ));
                } else {
                    w.line(&format!("{} _abi{i};", target.abi_in()));
                }
            }
        }
        for (i, param) in &owned_struct_params {
            w.line(&format!("{} _owned{i} = default;", param.ty.abi_in()));
        }
        if !owned_struct_params.is_empty() {
            w.open("try");
            for (i, param) in &owned_struct_params {
                w.line(&format!(
                    "_owned{i} = {};",
                    param.ty.surface_to_owned_abi(&param.name)
                ));
            }
        }
        for (i, param) in params.iter().enumerate() {
            match roles[i] {
                ParamRole::Utf16String => {
                    w.open(&format!("fixed (char* _abi{i} = {})", param.name));
                }
                ParamRole::Buffer { element, .. } => {
                    w.open(&format!(
                        "fixed ({}* _abi{i} = {})",
                        element.surface(),
                        param.name
                    ));
                }
                _ => {}
            }
        }
        let indirect_result = ret.filter(|ret| ret.is_native_com_record_return());
        if let Some(ret) = indirect_result {
            w.line(&format!("{} result = default;", ret.abi_in()));
        }
        let mut generics = vec!["nint".to_string()];
        let mut args = vec!["self".to_string()];
        if let Some(ret) = indirect_result {
            generics.push(ret.abi_out());
            args.push("&result".to_string());
        }
        for (i, param) in params.iter().enumerate() {
            if matches!(param.ty, CsType::ComOut { .. }) {
                generics.push("nint*".to_string());
                args.push(format!("&{}", param.name));
            } else {
                generics.push(param.ty.abi_in());
                args.push(match roles[i] {
                    ParamRole::ScalarPointer { .. } => format!("&_abi{i}"),
                    ParamRole::Utf16String => format!("(ushort*)_abi{i}"),
                    ParamRole::Buffer { element, .. } => {
                        format!("({}*)_abi{i}", element.abi())
                    }
                    ParamRole::BufferCount { buffer } => format!(
                        "checked(({}){}.Length)",
                        param.ty.abi_in(),
                        params[buffer].name
                    ),
                    ParamRole::Value if param.ty.owned_struct_abi().is_some() => {
                        format!("_owned{i}")
                    }
                    ParamRole::Value => param.ty.surface_to_abi(&param.name),
                });
            }
        }
        let abi_ret = if indirect_result.is_some() {
            "void".to_string()
        } else {
            ret.map_or_else(|| "void".to_string(), CsType::abi_in)
        };
        generics.push(abi_ret);
        let call = format!(
            "((delegate* unmanaged<{}>)(*(void***)self)[{slot}])({})",
            generics.join(", "),
            args.join(", ")
        );
        let owned_result = ret.is_some_and(|ret| ret.owned_struct_abi().is_some());
        if indirect_result.is_some() {
            w.line(&format!("{call};"));
        } else if owned_result {
            w.line(&format!("{} result = default;", ret.unwrap().abi_in()));
            w.open("try");
            w.line(&format!("result = {call};"));
        } else if matches!(ret, Some(CsType::HResult)) && out_object.is_none() {
            w.line(&format!("WindowsCsharp.Com.Check({call});"));
        } else if matches!(ret, Some(CsType::HResult)) {
            let (name, _) = out_object.unwrap();
            w.line(&format!("int _comOutHr = {call};"));
            write_com_out_result_check(w, name);
        } else if let Some(ret) = ret {
            w.line(&format!("{} result = {call};", ret.abi_in()));
        } else {
            w.line(&format!("{call};"));
        }
        for (i, param) in params.iter().enumerate() {
            if let ParamRole::ScalarPointer { target, .. } = roles[i] {
                w.line(&format!(
                    "{} = {};",
                    param.name,
                    target.abi_to_surface(&format!("_abi{i}"))
                ));
            }
        }
        if let Some((name, object)) = out_object {
            w.line(&format!(
                "return WindowsCsharp.Com.Wrap<{object}>({name})!;"
            ));
        } else if let Some(ret) = ret
            && !matches!(ret, CsType::HResult)
        {
            w.line(&format!("return {};", ret.abi_to_surface("result")));
        }
        if owned_result {
            w.close();
            w.open("finally");
            w.line("result.Dispose();");
            w.close();
        }
        for role in roles.iter().rev() {
            if matches!(role, ParamRole::Utf16String | ParamRole::Buffer { .. }) {
                w.close();
            }
        }
        if !owned_struct_params.is_empty() {
            w.close();
            w.open("finally");
            for (i, _) in owned_struct_params.iter().rev() {
                w.line(&format!("_owned{i}.Dispose();"));
            }
            w.close();
        }
    }

    if let Some((element, _)) = ret_array {
        w.line(&format!(
            "return {};",
            array_from_abi(element, "resultSize", "result")
        ));
    } else if ret_is_string {
        w.line("return WindowsCsharp.Interop.FromHstring(result);");
    } else if let Some(ret) = ret {
        w.line(&format!("return {};", ret.abi_to_surface("result")));
    }
    if has_cleanup {
        w.close();
        w.open("finally");
        for (i, element) in &output_arrays {
            w.line(&array_cleanup(
                element,
                &format!("_asize{i}"),
                &format!("_adata{i}"),
            ));
        }
        if let Some((element, _)) = ret_array {
            w.line(&array_cleanup(element, "resultSize", "result"));
        }
        if ret.is_some_and(|ret| ret.owned_struct_abi().is_some()) {
            w.line("result.Dispose();");
        }
        w.close();
    }

    if forward_iid.is_some() {
        w.close();
        w.open("finally");
        w.line("_ = WindowsCsharp.Com.Release(self);");
        w.close();
    }
    w.close();
}

/// Emits the shared parameter-marshalling prologue and the vtable call for one slot invocation,
/// assuming `self` is already bound. Objects cross as call-scoped `ComLease` handles, sugared
/// `IReference<T>` inputs as `ReferenceBox` handles, and strings as pinned `HSTRING` references
/// built inside a `fixed` scope; scalars/enums/structs use the by-value ABI adapters. `trailing`
/// are extra ABI `(generic, argument)` pairs inserted after the user parameters and before the
/// result (the composable factory's `outer`/`inner` nulls). `result` is the return out-parameter as
/// `(local declaration, ABI generic, call argument)`. The caller predeclares owning struct results
/// so its cleanup can run from an outer `finally`.
fn write_marshalled_call(
    w: &mut Writer,
    params: &[Param],
    slot: usize,
    trailing: &[(String, String)],
    result: Option<(String, String, String)>,
    compatible: bool,
    arrays_predeclared: bool,
) {
    // A `String` crosses the ABI as an `HSTRING` handle (`nint`), so a string parameter is pinned
    // and wrapped in a fast-pass reference for the duration of the call. An object crosses as a
    // call-scoped lease, a sugared `IReference<T>` as a boxed value.
    let string_params: Vec<(usize, &String)> = params
        .iter()
        .enumerate()
        .filter(|(_, param)| matches!(param.ty, CsType::String))
        .map(|(i, param)| (i, &param.name))
        .collect();
    let object_params: Vec<(usize, &String)> = params
        .iter()
        .enumerate()
        .filter(|(_, param)| param.ty.is_object())
        .map(|(i, param)| (i, &param.name))
        .collect();
    let reference_params: Vec<(usize, &String, &CsType, Guid)> = params
        .iter()
        .enumerate()
        .filter_map(|(i, param)| {
            param
                .ty
                .reference()
                .map(|(value, iid)| (i, &param.name, value, iid))
        })
        .collect();
    let array_params: Vec<(usize, &String, &CsType, bool)> = params
        .iter()
        .enumerate()
        .filter_map(|(i, param)| {
            param
                .ty
                .array()
                .map(|(element, output)| (i, &param.name, element, output))
        })
        .collect();
    let owned_struct_params: Vec<(usize, &String, &CsType)> = params
        .iter()
        .enumerate()
        .filter(|(_, param)| param.ty.owned_struct_abi().is_some())
        .map(|(i, param)| (i, &param.name, &param.ty))
        .collect();

    for (i, pname) in &object_params {
        if compatible {
            if let Some(target) = compatible_object_name(&params[*i].ty) {
                w.line(&format!(
                    "using WindowsCsharp.InterfaceLease _olease{i} = WindowsCsharp.InterfaceLease.From({pname}, {target}.Iid);"
                ));
                continue;
            }
        }
        w.line(&format!(
            "using WindowsCsharp.ComLease _olease{i} = WindowsCsharp.ComLease.From({pname});"
        ));
    }
    for (i, pname, value, iid) in &reference_params {
        w.line(&format!(
            "using WindowsCsharp.ReferenceBox<{}> _rbox{i} = new WindowsCsharp.ReferenceBox<{}>({pname}, {});",
            value.surface(),
            value.surface(),
            iid.to_cs()
        ));
    }
    if !arrays_predeclared {
        for (i, _, element, output) in &array_params {
            if *output {
                w.line(&format!("uint _asize{i};"));
                w.line(&format!("{}* _adata{i};", element.abi_in()));
            }
        }
    }
    if let Some((declaration, _, _)) = &result
        && !declaration.is_empty()
    {
        w.line(declaration);
    }

    for (i, _, ty) in &owned_struct_params {
        w.line(&format!("{} _abi{i} = default;", ty.abi_in()));
    }
    if !owned_struct_params.is_empty() {
        w.open("try");
        for (i, pname, ty) in &owned_struct_params {
            w.line(&format!("_abi{i} = {};", ty.surface_to_owned_abi(pname)));
        }
    }

    // Build the ABI delegate generic list and the call-argument list. Enums cross the ABI as their
    // underlying scalar, so surface values are cast to the ABI on input and back on return; a
    // string parameter passes the `HSTRING` reference built below.
    let mut generics = vec!["nint".to_string()];
    let mut args = vec!["self".to_string()];
    for (i, param) in params.iter().enumerate() {
        let pname = &param.name;
        let ty = &param.ty;
        if let Some((element, output)) = ty.array() {
            if output {
                generics.push("uint*".to_string());
                args.push(format!("&_asize{i}"));
                generics.push(format!("{}**", element.abi_in()));
                args.push(format!("&_adata{i}"));
                continue;
            }
            generics.push("uint".to_string());
            args.push(format!("(uint){pname}.Length"));
            generics.push(format!("{}*", element.abi_in()));
            if matches!(element, CsType::String) || element.is_object() {
                args.push(format!("_alease{i}.Values"));
            } else if element.surface() == element.abi_in() {
                args.push(format!("_aptr{i}"));
            } else {
                args.push(format!("({}*)_aptr{i}", element.abi_in()));
            }
            continue;
        }
        generics.push(ty.abi_in());
        if ty.owned_struct_abi().is_some() {
            args.push(format!("_abi{i}"));
        } else if matches!(ty, CsType::String) {
            args.push(format!("_hstr{i}"));
        } else if ty.is_object() {
            args.push(format!("_olease{i}.Handle"));
        } else if ty.reference().is_some() {
            args.push(format!("_rbox{i}.Handle"));
        } else {
            args.push(ty.surface_to_abi(pname));
        }
    }
    for (generic, arg) in trailing {
        generics.push(generic.clone());
        args.push(arg.clone());
    }
    if let Some((_, generic, arg)) = &result {
        generics.push(generic.clone());
        args.push(arg.clone());
    }
    generics.push("int".to_string());

    let call = format!(
        "((delegate* unmanaged<{}>)(*(void***)self)[{slot}])({})",
        generics.join(", "),
        args.join(", ")
    );

    // The fast-pass reference points into the pinned buffer, so the call must run inside the `fixed`
    // scope. A `null` string pins to a null pointer and passes a zero-length (null) `HSTRING`.
    let input_arrays: Vec<_> = array_params
        .iter()
        .filter(|(_, _, element, output)| {
            !*output && !matches!(element, CsType::String) && !element.is_object()
        })
        .collect();
    for (i, pname, element, output) in &array_params {
        if *output {
            continue;
        }
        if matches!(element, CsType::String) {
            w.line(&format!(
                "using WindowsCsharp.StringArrayLease _alease{i} = WindowsCsharp.StringArrayLease.From({pname});"
            ));
        } else if element.is_object() {
            w.line(&format!(
                "using WindowsCsharp.ObjectArrayLease _alease{i} = WindowsCsharp.ObjectArrayLease.From({pname});"
            ));
        }
    }
    if string_params.is_empty() && input_arrays.is_empty() {
        w.line(&format!("WindowsCsharp.Com.Check({call});"));
    } else {
        for (i, pname, element, _) in &input_arrays {
            w.open(&format!(
                "fixed ({}* _aptr{i} = {pname})",
                element.surface()
            ));
        }
        let decls = string_params
            .iter()
            .enumerate()
            .map(|(n, (i, pname))| {
                if n == 0 {
                    format!("char* _hbuf{i} = {pname}")
                } else {
                    format!("_hbuf{i} = {pname}")
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        if !string_params.is_empty() {
            w.open(&format!("fixed ({decls})"));
        }
        for (i, pname) in &string_params {
            w.line(&format!("WindowsCsharp.Interop.HstringHeader _hhdr{i};"));
            w.line(&format!(
                "nint _hstr{i} = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf{i}, (uint)({pname}?.Length ?? 0), &_hhdr{i});"
            ));
        }
        w.line(&format!("WindowsCsharp.Com.Check({call});"));
        if !string_params.is_empty() {
            w.close();
        }
        for _ in &input_arrays {
            w.close();
        }
    }

    for (i, pname, element, output) in &array_params {
        if *output {
            w.line(&format!(
                "{pname} = {};",
                array_from_abi(element, &format!("_asize{i}"), &format!("_adata{i}"))
            ));
        }
    }
    if !owned_struct_params.is_empty() {
        w.close();
        w.open("finally");
        for (i, _, _) in owned_struct_params.iter().rev() {
            w.line(&format!("_abi{i}.Dispose();"));
        }
        w.close();
    }
}

fn array_from_abi(element: &CsType, length: &str, value: &str) -> String {
    if matches!(element, CsType::Boolean) {
        format!("WindowsCsharp.Interop.FromBooleanArray(ref {length}, ref {value})")
    } else if matches!(element, CsType::String) {
        format!("WindowsCsharp.Interop.FromStringArray(ref {length}, ref {value})")
    } else if element.is_object() {
        format!(
            "WindowsCsharp.Interop.FromObjectArray<{}>(ref {length}, ref {value})",
            element.collection_surface()
        )
    } else {
        let surface = element.surface();
        let abi = element.abi_in();
        format!("WindowsCsharp.Interop.FromArray<{surface}, {abi}>(ref {length}, ref {value})")
    }
}

fn array_cleanup(element: &CsType, length: &str, value: &str) -> String {
    if matches!(element, CsType::String) {
        format!("WindowsCsharp.Interop.FreeStringArray({length}, {value});")
    } else if element.is_object() {
        format!("WindowsCsharp.Interop.FreeObjectArray({length}, {value});")
    } else {
        format!("Marshal.FreeCoTaskMem((nint){value});")
    }
}

/// Emits one factory creation method as a public constructor plus its private static create helper.
/// The constructor delegates to the helper and hands the raw runtime-class pointer (already the
/// class default interface by ABI convention) to the safe `ComObject` base, which takes agile-safe
/// ownership - including non-agile objects. The helper leases the factory interface (through
/// `FactoryLease`, so a non-agile factory is released in `finally`), marshals the user parameters
/// with the same adapters instance members use, and calls the create slot. A composable factory
/// passes `outer`/`inner` as null for non-aggregating construction.
fn write_factory_constructor(
    w: &mut Writer,
    class: &Class,
    factory_index: usize,
    composable: bool,
    ctor: &Constructor,
    helper_index: usize,
) {
    let surface_params = ctor
        .params
        .iter()
        .map(|param| param.ty.parameter(&param.name))
        .collect::<Vec<_>>()
        .join(", ");
    let arg_names = ctor
        .params
        .iter()
        .map(|param| param.name.clone())
        .collect::<Vec<_>>()
        .join(", ");
    let helper = format!("FactoryCreate{helper_index}");
    let slot = ctor.slot;

    w.line("");
    w.line(&format!(
        "public {0}({surface_params}) : base({helper}({arg_names}), Iid) {{}}",
        class.name
    ));

    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open(&format!("private static nint {helper}({surface_params})"));
    w.line(&format!(
        "using WindowsCsharp.FactoryLease lease = WindowsCsharp.WinRT.GetActivationFactory(ref s_module, ref s_factory{factory_index}, \"{}\", s_factory{factory_index}_iid);",
        class.class_id()
    ));
    w.line("nint self = lease.Handle;");
    // A composable factory's `outer` and `inner` ABI parameters are passed as null (both
    // pointer-sized) for non-aggregating construction.
    let trailing: Vec<(String, String)> = if composable {
        vec![
            ("nint".to_string(), "0".to_string()),
            ("nint".to_string(), "0".to_string()),
        ]
    } else {
        Vec::new()
    };
    write_marshalled_call(
        w,
        &ctor.params,
        slot,
        &trailing,
        Some((
            "nint _instance;".to_string(),
            "nint*".to_string(),
            "&_instance".to_string(),
        )),
        false,
        false,
    );
    w.line("return _instance;");
    w.close();
}

/// Emits the `HSTRING` getter body: call the slot, copy the raw buffer into a managed string, and
/// release the returned handle.
fn write_string_get(w: &mut Writer, slot: usize) {
    w.line("nint hstring;");
    w.line(&format!(
        "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[{slot}])(self, &hstring));"
    ));
    w.line("return WindowsCsharp.Interop.FromHstring(hstring);");
}

/// Emits the `HSTRING` setter body: pin the UTF-16 buffer, build a fast-pass string reference, and
/// call the slot.
fn write_string_put(w: &mut Writer, slot: usize, value: &str) {
    w.open(&format!("fixed (char* c = {value})"));
    w.line("WindowsCsharp.Interop.HstringHeader header;");
    w.line(&format!(
        "nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)c, (uint)({value}?.Length ?? 0), &header);"
    ));
    w.line(&format!(
        "WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[{slot}])(self, hstring));"
    ));
    w.close();
}

fn write_reference_support(w: &mut Writer) {
    w.open("internal readonly unsafe ref struct ReferenceBox<T> where T : unmanaged");
    w.line("internal nint Handle { get; }");
    w.line("");
    w.open("internal ReferenceBox(T? value, Guid iid)");
    w.line("Handle = value.HasValue ? ReferenceBoxAbi.Create(value.GetValueOrDefault(), iid) : 0;");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line("public void Dispose() { if (Handle != 0) _ = Com.Release(Handle); }");
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("internal static T? Unbox(nint value)");
    w.open("if (value == 0)");
    w.line("return null;");
    w.close();
    w.open("try");
    w.line("T result;");
    w.line(
        "Com.Check(((delegate* unmanaged<nint, T*, int>)(*(void***)value)[6])(value, &result));",
    );
    w.line("return result;");
    w.close();
    w.open("finally");
    w.line("_ = Com.Release(value);");
    w.close();
    w.close();
    w.close();
    w.line("");

    w.open("internal static unsafe class ReferenceBoxAbi");
    w.line("private static readonly Guid s_iunknown = new Guid(0x00000000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);");
    w.line("private static readonly Guid s_iinspectable = new Guid(0xaf86e2e0, 0xb12d, 0x4c6a, 0x9c, 0x5a, 0xd7, 0xaa, 0x65, 0x10, 0x1e, 0x90);");
    w.line("private static readonly Guid s_iagile = new Guid(0x94ea2b94, 0xe9cc, 0x49e0, 0xc0, 0xff, 0xee, 0x64, 0xca, 0x8f, 0x5b, 0x90);");
    w.line("private static readonly nint* s_vtable = BuildVtable();");
    w.line("");
    w.line("[StructLayout(LayoutKind.Sequential)]");
    w.open("private struct Header");
    w.line("internal nint Vtable;");
    w.line("internal int References;");
    w.line("internal int Size;");
    w.line("internal Guid Iid;");
    w.close();
    w.line("");
    w.open("private static nint* BuildVtable()");
    w.line("nint* value = (nint*)NativeMemory.Alloc(7, (nuint)sizeof(nint));");
    w.line("value[0] = (nint)(delegate* unmanaged<nint, Guid*, nint*, int>)&QueryInterface;");
    w.line("value[1] = (nint)(delegate* unmanaged<nint, uint>)&AddRef;");
    w.line("value[2] = (nint)(delegate* unmanaged<nint, uint>)&Release;");
    w.line("value[3] = (nint)(delegate* unmanaged<nint, uint*, Guid**, int>)&GetIids;");
    w.line("value[4] = (nint)(delegate* unmanaged<nint, nint*, int>)&GetRuntimeClassName;");
    w.line("value[5] = (nint)(delegate* unmanaged<nint, int*, int>)&GetTrustLevel;");
    w.line("value[6] = (nint)(delegate* unmanaged<nint, void*, int>)&GetValue;");
    w.line("return value;");
    w.close();
    w.line("");
    w.open("internal static nint Create<T>(T value, Guid iid) where T : unmanaged");
    w.line(
        "Header* header = (Header*)NativeMemory.AllocZeroed((nuint)(sizeof(Header) + sizeof(T)));",
    );
    w.line("header->Vtable = (nint)s_vtable;");
    w.line("header->References = 1;");
    w.line("header->Size = sizeof(T);");
    w.line("header->Iid = iid;");
    w.line("*(T*)((byte*)header + sizeof(Header)) = value;");
    w.line("return (nint)header;");
    w.close();
    w.line("");
    w.line("[UnmanagedCallersOnly]");
    w.open("private static int QueryInterface(nint self, Guid* iid, nint* result)");
    w.open("if (result == null)");
    w.line("return unchecked((int)0x80004003);");
    w.close();
    w.line("Header* header = (Header*)self;");
    w.open("if (*iid == header->Iid || *iid == s_iunknown || *iid == s_iinspectable || *iid == s_iagile)");
    w.line("_ = Interlocked.Increment(ref header->References);");
    w.line("*result = self;");
    w.line("return 0;");
    w.close();
    w.line("*result = 0;");
    w.line("return unchecked((int)0x80004002);");
    w.close();
    w.line("");
    w.line("[UnmanagedCallersOnly]");
    w.line("private static uint AddRef(nint self) => (uint)Interlocked.Increment(ref ((Header*)self)->References);");
    w.line("");
    w.line("[UnmanagedCallersOnly]");
    w.open("private static uint Release(nint self)");
    w.line("int count = Interlocked.Decrement(ref ((Header*)self)->References);");
    w.open("if (count == 0)");
    w.line("NativeMemory.Free((void*)self);");
    w.close();
    w.line("return (uint)count;");
    w.close();
    w.line("");
    w.line("[UnmanagedCallersOnly]");
    w.open("private static int GetIids(nint self, uint* count, Guid** values)");
    w.open("if (count == null || values == null)");
    w.line("return unchecked((int)0x80004003);");
    w.close();
    w.line("*count = 0;");
    w.line("*values = null;");
    w.open("try");
    w.line("Guid* value = (Guid*)Marshal.AllocCoTaskMem(sizeof(Guid));");
    w.line("*value = ((Header*)self)->Iid;");
    w.line("*count = 1;");
    w.line("*values = value;");
    w.line("return 0;");
    w.close();
    w.open("catch (OutOfMemoryException)");
    w.line("return unchecked((int)0x8007000e);");
    w.close();
    w.close();
    w.line("");
    w.line("[UnmanagedCallersOnly]");
    w.open("private static int GetRuntimeClassName(nint self, nint* value)");
    w.open("if (value == null)");
    w.line("return unchecked((int)0x80004003);");
    w.close();
    w.line("*value = 0;");
    w.line("return 0;");
    w.close();
    w.line("");
    w.line("[UnmanagedCallersOnly]");
    w.open("private static int GetTrustLevel(nint self, int* value)");
    w.open("if (value == null)");
    w.line("return unchecked((int)0x80004003);");
    w.close();
    w.line("*value = 0;");
    w.line("return 0;");
    w.close();
    w.line("");
    w.line("[UnmanagedCallersOnly]");
    w.open("private static int GetValue(nint self, void* result)");
    w.open("if (result == null)");
    w.line("return unchecked((int)0x80004003);");
    w.close();
    w.line("Header* header = (Header*)self;");
    w.line(
        "Buffer.MemoryCopy((byte*)header + sizeof(Header), result, header->Size, header->Size);",
    );
    w.line("return 0;");
    w.close();
    w.close();
}

/// Emits the shared runtime support: registration-free activation and the combase/kernel32
/// P/Invokes. This mirrors windows-rs reg-free activation (`LoadLibrary("<Namespace>.dll")` +
/// `DllGetActivationFactory`), not `RoActivateInstance`.
fn write_raw_owner_support(w: &mut Writer) {
    w.open("public abstract unsafe class ComObject : IDisposable");
    w.line("private nint _this;");
    w.line("");
    w.line("protected ComObject(nint self, Guid iid) => Initialize(self);");
    w.line("protected ComObject(nint self, bool trustedAgile) => Initialize(self);");
    w.line("");
    w.open("private void Initialize(nint self)");
    w.open("if (self == 0)");
    w.line(
        "throw new ArgumentException(\"A COM interface pointer cannot be null.\", nameof(self));",
    );
    w.close();
    w.line("_this = self;");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("internal ComLease Acquire()");
    w.line("nint self = _this;");
    w.open("if (self == 0)");
    w.line("throw new ObjectDisposedException(GetType().FullName);");
    w.close();
    w.line("return new ComLease(self);");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public void Dispose()");
    w.line("Release();");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("private void Release()");
    w.line("nint self = _this;");
    w.line("_this = 0;");
    w.open("if (self != 0)");
    w.line("_ = Com.Release(self);");
    w.close();
    w.close();
    w.close();
    w.line("");

    w.open("internal readonly ref struct ComLease");
    w.line("internal nint Handle { get; }");
    w.line("internal bool TrustedAgile => Handle != 0;");
    w.line("");
    w.open("internal ComLease(nint handle)");
    w.line("Handle = handle;");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line(
        "internal static ComLease From(ComObject? owner) => owner is null ? default : owner.Acquire();",
    );
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line("public void Dispose() {}");
    w.close();
}

fn write_raw_winrt_support(w: &mut Writer) {
    w.open("internal static unsafe class WinRT");
    w.line("private static readonly Guid s_iagile = new Guid(0x94ea2b94, 0xe9cc, 0x49e0, 0xc0, 0xff, 0xee, 0x64, 0xca, 0x8f, 0x5b, 0x90);");
    w.line("[ThreadStatic]");
    w.line("private static MtaUsage? s_mta;");
    w.line("");
    w.open(
        "public static nint Activate(ref nint moduleCache, ref nint factoryCache, string classId, Guid iid)",
    );
    w.line("s_mta ??= new MtaUsage();");
    w.line("nint factory = factoryCache;");
    w.line("bool releaseFactory = false;");
    w.open("if (factory == 0)");
    w.line("factory = LoadActivationFactory(ref moduleCache, classId);");
    w.line("Guid iidAgile = s_iagile;");
    w.line("nint agile;");
    w.open("if (Com.QueryInterface(factory, &iidAgile, &agile) >= 0)");
    w.line("_ = Com.Release(agile);");
    w.line("factoryCache = factory;");
    w.close();
    w.open("else");
    w.line("releaseFactory = true;");
    w.close();
    w.close();
    w.open("try");
    w.line("nint inspectable;");
    w.line("Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)factory)[6])(factory, &inspectable));");
    w.open("try");
    w.line("nint self;");
    w.line("Com.Check(Com.QueryInterface(inspectable, &iid, &self));");
    w.line("return self;");
    w.close();
    w.open("finally");
    w.line("_ = Com.Release(inspectable);");
    w.close();
    w.close();
    w.open("finally");
    w.open("if (releaseFactory)");
    w.line("_ = Com.Release(factory);");
    w.close();
    w.close();
    w.close();
    w.line("");
    w.open("public static FactoryLease GetActivationFactory(ref nint moduleCache, ref nint factoryCache, string classId, Guid iid)");
    w.line("s_mta ??= new MtaUsage();");
    w.line("nint requested = factoryCache;");
    w.open("if (requested != 0)");
    w.line("return new FactoryLease(requested, 0);");
    w.close();
    w.line("nint factory = LoadActivationFactory(ref moduleCache, classId);");
    w.open("try");
    w.line("Com.Check(Com.QueryInterface(factory, &iid, &requested));");
    w.close();
    w.open("finally");
    w.line("_ = Com.Release(factory);");
    w.close();
    w.line("Guid iidAgile = s_iagile;");
    w.line("nint agile;");
    w.open("if (Com.QueryInterface(requested, &iidAgile, &agile) >= 0)");
    w.line("_ = Com.Release(agile);");
    w.line("factoryCache = requested;");
    w.line("return new FactoryLease(requested, 0);");
    w.close();
    w.line("return new FactoryLease(requested, requested);");
    w.close();
    w.line("");
    w.open("private static nint LoadActivationFactory(ref nint moduleCache, string classId)");
    w.line("nint module = moduleCache;");
    w.open("if (module == 0)");
    w.line("module = LoadModule(classId);");
    w.line("moduleCache = module;");
    w.close();
    w.line("nint factory = 0;");
    w.line("int hr = unchecked((int)0x80040111);");
    w.open("fixed (char* id = classId)");
    w.line("Interop.HstringHeader header;");
    w.line(
        "nint hstring = Interop.CreateStringReference((ushort*)id, (uint)classId.Length, &header);",
    );
    w.open("if (module != 0)");
    w.line("nint proc = Interop.GetProcAddress(module, \"DllGetActivationFactory\");");
    w.open("if (proc != 0)");
    w.line("hr = ((delegate* unmanaged<nint, nint*, int>)proc)(hstring, &factory);");
    w.close();
    w.close();
    w.open("if (hr < 0)");
    w.line("Guid iid = new Guid(0x00000035, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);");
    w.line("hr = Interop.RoGetActivationFactory(hstring, &iid, &factory);");
    w.close();
    w.close();
    w.line("Com.Check(hr);");
    w.line("return factory;");
    w.close();
    w.line("");
    w.open("private static nint LoadModule(string classId)");
    w.line("nint module = 0;");
    w.line("string name = classId;");
    w.open("while (true)");
    w.line("int dot = name.LastIndexOf('.');");
    w.open("if (dot < 0)");
    w.line("break;");
    w.close();
    w.line("name = name.Substring(0, dot);");
    w.line("module = Interop.LoadLibrary(name + \".dll\");");
    w.open("if (module != 0)");
    w.line("break;");
    w.close();
    w.close();
    w.line("return module;");
    w.close();
    w.line("");
    w.open("private sealed class MtaUsage");
    w.line("private nint _cookie;");
    w.line("");
    w.open("public MtaUsage()");
    w.line("nint cookie;");
    w.line("Com.Check(Interop.CoIncrementMTAUsage(&cookie));");
    w.line("_cookie = cookie;");
    w.close();
    w.line("");
    w.open("~MtaUsage()");
    w.open("if (_cookie != 0)");
    w.line("_ = Interop.CoDecrementMTAUsage(_cookie);");
    w.close();
    w.close();
    w.close();
    w.close();
}

fn write_array_lease_support(w: &mut Writer) {
    w.open("internal readonly unsafe ref struct StringArrayLease");
    w.line("private readonly nint* _values;");
    w.line("private readonly int _length;");
    w.line("internal nint* Values => _values;");
    w.line("");
    w.open("private StringArrayLease(nint* values, int length)");
    w.line("_values = values;");
    w.line("_length = length;");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("internal static StringArrayLease From(string[] values)");
    w.open("if (values.Length == 0)");
    w.line("return default;");
    w.close();
    w.line("nint* result = (nint*)NativeMemory.AllocZeroed((nuint)values.Length, (nuint)sizeof(nint));");
    w.open("try");
    w.open("for (int i = 0; i < values.Length; i++)");
    w.line("result[i] = Interop.CreateString(values[i]);");
    w.close();
    w.line("return new StringArrayLease(result, values.Length);");
    w.close();
    w.open("catch");
    w.line("Free(result, values.Length);");
    w.line("throw;");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line("public void Dispose() => Free(_values, _length);");
    w.line("");
    w.open("private static void Free(nint* values, int length)");
    w.open("if (values == null)");
    w.line("return;");
    w.close();
    w.open("for (int i = 0; i < length; i++)");
    w.open("if (values[i] != 0)");
    w.line("_ = Interop.WindowsDeleteString(values[i]);");
    w.close();
    w.close();
    w.line("NativeMemory.Free(values);");
    w.close();
    w.close();
    w.line("");

    w.open("internal readonly unsafe ref struct ObjectArrayLease");
    w.line("private readonly nint* _values;");
    w.line("private readonly int _length;");
    w.line("internal nint* Values => _values;");
    w.line("");
    w.open("private ObjectArrayLease(nint* values, int length)");
    w.line("_values = values;");
    w.line("_length = length;");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("internal static ObjectArrayLease From<T>(T?[] values) where T : ComObject");
    w.open("if (values.Length == 0)");
    w.line("return default;");
    w.close();
    w.line("nint* result = (nint*)NativeMemory.AllocZeroed((nuint)values.Length, (nuint)sizeof(nint));");
    w.open("try");
    w.open("for (int i = 0; i < values.Length; i++)");
    w.line("using ComLease lease = ComLease.From(values[i]);");
    w.line("nint value = lease.Handle;");
    w.open("if (value != 0)");
    w.line("_ = Com.AddRef(value);");
    w.close();
    w.line("result[i] = value;");
    w.close();
    w.line("return new ObjectArrayLease(result, values.Length);");
    w.close();
    w.open("catch");
    w.line("Free(result, values.Length);");
    w.line("throw;");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line("public void Dispose() => Free(_values, _length);");
    w.line("");
    w.open("private static void Free(nint* values, int length)");
    w.open("if (values == null)");
    w.line("return;");
    w.close();
    w.open("for (int i = 0; i < length; i++)");
    w.open("if (values[i] != 0)");
    w.line("_ = Com.Release(values[i]);");
    w.close();
    w.close();
    w.line("NativeMemory.Free(values);");
    w.close();
    w.close();
}

fn write_support(w: &mut Writer, raw: bool, async_operation: bool) {
    w.open("namespace WindowsCsharp");

    if raw {
        write_raw_owner_support(w);
    } else {
        // Every projected reference type is one managed allocation. Agile objects are stored directly.
        // Apartment-bound objects retain their original pointer and COM context in a tagged native
        // block. Calls remain apartment-affine; disposal from another context, including finalization,
        // switches back to the originating context before Release. The high state bit marks disposal
        // and the low bits count active ABI calls, so disposal cannot free either representation while
        // a call uses it.
        w.open("public abstract unsafe class ComObject : IDisposable");
        w.line("private const int Disposed = int.MinValue;");
        w.line("private static readonly Guid s_iagile = new Guid(0x94ea2b94, 0xe9cc, 0x49e0, 0xc0, 0xff, 0xee, 0x64, 0xca, 0x8f, 0x5b, 0x90);");
        w.line("private static readonly Guid s_contextCallback = new Guid(0x000001da, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);");
        w.line("private static readonly Guid s_noReentrancy = new Guid(0x0a299774, 0x3e4e, 0xfc42, 0x1d, 0x9d, 0x72, 0xce, 0xe1, 0x05, 0xca, 0x57);");
        w.line("private nint _this;");
        w.line("private int _state;");
        w.line("");
        w.line("[StructLayout(LayoutKind.Sequential)]");
        w.open("private struct ApartmentReference");
        w.line("public nint Value;");
        w.line("public nint Context;");
        w.line("public nint Token;");
        w.close();
        w.line("");
        w.line("[StructLayout(LayoutKind.Sequential)]");
        w.open("private struct ContextCallData");
        w.line("public int Dispid;");
        w.line("public int Reserved;");
        w.line("public nint UserDefined;");
        w.close();
        w.line("");
        w.line("protected ComObject(nint self, Guid iid) : this(self, iid, false) {}");
        w.line(
        "protected ComObject(nint self, bool trustedAgile) : this(self, default, trustedAgile) {}",
    );
        w.line("");
        w.open("protected ComObject(nint self, Guid iid, bool trustedAgile)");
        w.open("if (self == 0)");
        w.line(
        "throw new ArgumentException(\"A COM interface pointer cannot be null.\", nameof(self));",
    );
        w.close();
        w.open("if (trustedAgile)");
        w.line("_this = self;");
        w.line("return;");
        w.close();
        w.line("Guid agileIid = s_iagile;");
        w.line("nint agile;");
        w.open("if (Com.QueryInterface(self, &agileIid, &agile) >= 0)");
        w.line("_ = Com.Release(agile);");
        w.line("_this = self;");
        w.line("return;");
        w.close();
        w.line("ApartmentReference* reference = null;");
        w.open("try");
        w.line(
        "reference = (ApartmentReference*)NativeMemory.Alloc((nuint)sizeof(ApartmentReference));",
    );
        w.line("reference->Value = 0;");
        w.line("reference->Context = 0;");
        w.line("reference->Token = 0;");
        w.line("Guid contextIid = s_contextCallback;");
        w.line("Com.Check(Interop.CoGetObjectContext(&contextIid, &reference->Context));");
        w.line("Com.Check(Interop.CoGetContextToken(&reference->Token));");
        w.line("reference->Value = self;");
        w.close();
        w.open("catch");
        w.open("if (reference != null)");
        w.open("if (reference->Context != 0)");
        w.line("_ = Com.Release(reference->Context);");
        w.close();
        w.line("NativeMemory.Free(reference);");
        w.close();
        w.line("_ = Com.Release(self);");
        w.line("throw;");
        w.close();
        w.line("_this = (nint)reference | 1;");
        w.close();
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("internal ComLease Acquire()");
        w.line("int state = Volatile.Read(ref _state);");
        w.open("while (state >= 0)");
        w.open("if (state == int.MaxValue)");
        w.line("throw new InvalidOperationException(\"Too many concurrent COM calls.\");");
        w.close();
        w.line("int observed = Interlocked.CompareExchange(ref _state, state + 1, state);");
        w.open("if (observed == state)");
        w.line("nint identity = _this;");
        w.open("if ((identity & 1) == 0)");
        w.line("return new ComLease(this, identity, true);");
        w.close();
        w.line("ApartmentReference* reference = (ApartmentReference*)(identity & ~1);");
        w.line("nint currentToken;");
        w.line("int hr = Interop.CoGetContextToken(&currentToken);");
        w.open("if (hr < 0 || currentToken != reference->Token)");
        w.line("ReleaseLease();");
        w.open("if (hr < 0)");
        w.line("Com.Check(hr);");
        w.close();
        w.line("throw new COMException(\"The COM object belongs to a different apartment.\", unchecked((int)0x8001010e));");
        w.close();
        w.line("return new ComLease(this, reference->Value, false);");
        w.close();
        w.line("state = observed;");
        w.close();
        w.line("throw new ObjectDisposedException(GetType().FullName);");
        w.close();
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("internal void ReleaseLease()");
        w.open("if (Interlocked.Decrement(ref _state) == Disposed)");
        w.line("ReleaseReference();");
        w.close();
        w.close();
        w.line("");
        w.open("public void Dispose()");
        w.line("DisposeCore();");
        w.line("GC.SuppressFinalize(this);");
        w.close();
        w.line("");
        w.open("private void DisposeCore()");
        w.line("int state = Volatile.Read(ref _state);");
        w.open("while (state >= 0)");
        w.line("int observed = Interlocked.CompareExchange(ref _state, state | Disposed, state);");
        w.open("if (observed == state)");
        w.open("if (state == 0)");
        w.line("ReleaseReference();");
        w.close();
        w.line("return;");
        w.close();
        w.line("state = observed;");
        w.close();
        w.close();
        w.line("");
        w.open("~ComObject()");
        w.line("DisposeCore();");
        w.close();
        w.line("");
        w.open("private void ReleaseReference()");
        w.line("nint self = Interlocked.Exchange(ref _this, 0);");
        w.open("if (self != 0)");
        w.open("if ((self & 1) == 0)");
        w.line("_ = Com.Release(self);");
        w.close();
        w.open("else");
        w.line("ApartmentReference* reference = (ApartmentReference*)(self & ~1);");
        w.line("ReleaseApartmentReference(reference);");
        w.line("NativeMemory.Free(reference);");
        w.close();
        w.close();
        w.close();
        w.line("");
        w.open("private static void ReleaseApartmentReference(ApartmentReference* reference)");
        w.line("ContextCallData data = default;");
        w.line("data.UserDefined = (nint)reference;");
        w.line("nint currentToken;");
        w.open(
        "if (Interop.CoGetContextToken(&currentToken) >= 0 && currentToken == reference->Token)",
    );
        w.line("ReleaseValue(reference);");
        w.close();
        w.open("else");
        w.line("Guid iid = s_noReentrancy;");
        w.line("// If the originating context is gone, leaking is safer than releasing here.");
        w.line("_ = ((delegate* unmanaged<nint, nint, ContextCallData*, Guid*, int, nint, int>)(*(void***)reference->Context)[3])(reference->Context, (nint)(delegate* unmanaged<ContextCallData*, int>)&ReleaseInContext, &data, &iid, 5, 0);");
        w.close();
        w.line("_ = Com.Release(reference->Context);");
        w.close();
        w.line("");
        w.open("private static void ReleaseValue(ApartmentReference* reference)");
        w.line("nint value = Interlocked.Exchange(ref reference->Value, 0);");
        w.open("if (value != 0)");
        w.line("_ = Com.Release(value);");
        w.close();
        w.close();
        w.line("");
        w.line("[UnmanagedCallersOnly]");
        w.open("private static int ReleaseInContext(ContextCallData* data)");
        w.line("ApartmentReference* reference = (ApartmentReference*)data->UserDefined;");
        w.line("ReleaseValue(reference);");
        w.line("return 0;");
        w.close();
        w.close();

        w.line("");

        w.open("internal readonly ref struct ComLease");
        w.line("private readonly ComObject? _owner;");
        w.line("private readonly nint _handle;");
        w.line("internal nint Handle => _handle & ~1;");
        w.line("internal bool TrustedAgile => (_handle & 1) != 0;");
        w.line("");
        w.open("internal ComLease(ComObject owner, nint handle, bool trustedAgile)");
        w.line("_owner = owner;");
        w.line("_handle = trustedAgile ? handle | 1 : handle;");
        w.close();
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.line("internal static ComLease From(ComObject? owner) => owner is null ? default : owner.Acquire();");
        w.line("");
        w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
        w.open("public void Dispose()");
        w.line("_owner?.ReleaseLease();");
        w.close();
        w.close();
    }

    w.line("");

    w.open("internal readonly unsafe ref struct InterfaceLease");
    w.line("internal nint Handle { get; }");
    w.line("");
    w.open("private InterfaceLease(nint handle)");
    w.line("Handle = handle;");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("internal static InterfaceLease From(ComObject? owner, Guid iid)");
    w.open("if (owner is null)");
    w.line("return default;");
    w.close();
    w.line("using ComLease source = ComLease.From(owner);");
    w.line("return From(source.Handle, iid);");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("internal static InterfaceLease From(nint source, Guid iid)");
    w.line("nint result;");
    w.line("Com.Check(Com.QueryInterface(source, &iid, &result));");
    w.line("return new InterfaceLease(result);");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public void Dispose()");
    w.open("if (Handle != 0)");
    w.line("_ = Com.Release(Handle);");
    w.close();
    w.close();
    w.close();

    w.line("");

    // A call-scoped hold on an activation-factory interface pointer. An agile factory is cached and
    // shared, so its lease borrows the cached pointer and releases nothing (`_release == 0`). A
    // non-agile factory cannot be cached across apartments, so its lease owns a transient reference
    // and releases it on `Dispose`.
    w.open("internal readonly ref struct FactoryLease");
    w.line("private readonly nint _release;");
    w.line("internal nint Handle { get; }");
    w.line("");
    w.open("internal FactoryLease(nint handle, nint release)");
    w.line("Handle = handle;");
    w.line("_release = release;");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public void Dispose()");
    w.open("if (_release != 0)");
    w.line("_ = Com.Release(_release);");
    w.close();
    w.close();
    w.close();

    w.line("");

    write_reference_support(w);

    w.line("");

    // The support surface every projected type implements: its interface IID and a constructor from
    // a raw ABI pointer. The generic `As<T>()` cast reads these through the type parameter, so a
    // QueryInterface can target any projected interface without a per-interface cast method.
    w.open("public interface IComInterface<T> where T : ComObject, IComInterface<T>");
    w.line("static abstract Guid Iid { get; }");
    w.line("static abstract T FromAbi(nint self);");
    w.line("static abstract T FromAgileAbi(nint self);");
    w.close();

    w.line("");

    w.open("public interface IObjectParameter<T>");
    w.close();

    w.line("");

    write_array_lease_support(w);

    w.line("");

    w.open("internal static unsafe class Com");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static void Check(int hr)");
    w.open("if (hr < 0)");
    w.line("Throw(hr);");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.NoInlining)]");
    w.open("private static void Throw(int hr)");
    w.line("throw new COMException(Interop.TakeErrorMessage(), hr);");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line(&format!(
        "public static int QueryInterface(nint self, Guid* iid, nint* result) => ((delegate* unmanaged<nint, Guid*, nint*, int>)(*(void***)self)[{QUERY_INTERFACE_SLOT}])(self, iid, result);"
    ));
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line(&format!(
        "public static uint AddRef(nint self) => ((delegate* unmanaged<nint, uint>)(*(void***)self)[{ADD_REF_SLOT}])(self);"
    ));
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.line(&format!(
        "public static uint Release(nint self) => ((delegate* unmanaged<nint, uint>)(*(void***)self)[{RELEASE_SLOT}])(self);"
    ));
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open(
        "public static T As<T>(nint self, bool trustedAgile) where T : ComObject, IComInterface<T>",
    );
    w.line("Guid iid = T.Iid;");
    w.line("nint result;");
    w.line("Check(QueryInterface(self, &iid, &result));");
    w.line("return trustedAgile ? WrapAgile<T>(result)! : Wrap<T>(result)!;");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static T? Wrap<T>(nint self) where T : ComObject, IComInterface<T>");
    w.open("if (self == 0)");
    w.line("return null;");
    w.close();
    w.line("return T.FromAbi(self);");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static T? WrapAgile<T>(nint self) where T : ComObject, IComInterface<T>");
    w.open("if (self == 0)");
    w.line("return null;");
    w.close();
    w.line("return T.FromAgileAbi(self);");
    w.close();
    w.close();

    w.line("");

    // The RAII event revoker owns one AddRef on the event source plus the registration token and the
    // remove thunk captured at subscription. Aliases share one atomic dispose state. The raw
    // Add/Remove event accessors remain the allocation-free path.
    w.open("public sealed unsafe class EventRevoker : IDisposable");
    w.line("private static readonly Guid s_contextCallback = new Guid(0x000001da, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);");
    w.line("private static readonly Guid s_noReentrancy = new Guid(0x0a299774, 0x3e4e, 0xfc42, 0x1d, 0x9d, 0x72, 0xce, 0xe1, 0x05, 0xca, 0x57);");
    w.line("private nint _source;");
    w.line("private nint _context;");
    w.line("private nint _contextToken;");
    w.line("private long _token;");
    w.line("private delegate* unmanaged<nint, long, int> _remove;");
    w.line("");
    w.line("[StructLayout(LayoutKind.Sequential)]");
    w.open("private struct ContextCallData");
    w.line("public int Dispid;");
    w.line("public int Reserved;");
    w.line("public nint UserDefined;");
    w.close();
    w.line("");
    w.line("[StructLayout(LayoutKind.Sequential)]");
    w.open("private struct RevokeData");
    w.line("public nint Source;");
    w.line("public long Token;");
    w.line("public delegate* unmanaged<nint, long, int> Remove;");
    w.close();
    w.line("");
    w.open("internal void Attach(nint source, bool trustedAgile, long token, delegate* unmanaged<nint, long, int> remove)");
    w.open("if (!trustedAgile)");
    w.line("Guid iid = s_contextCallback;");
    w.line("nint context;");
    w.line("Com.Check(Interop.CoGetObjectContext(&iid, &context));");
    w.line("_context = context;");
    w.open("try");
    w.line("nint contextToken;");
    w.line("Com.Check(Interop.CoGetContextToken(&contextToken));");
    w.line("_contextToken = contextToken;");
    w.close();
    w.open("catch");
    w.line("_ = Com.Release(_context);");
    w.line("_context = 0;");
    w.line("throw;");
    w.close();
    w.close();
    w.line("_token = token;");
    w.line("_remove = remove;");
    w.line("Volatile.Write(ref _source, source);");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public void Dispose()");
    w.line("DisposeCore();");
    w.line("GC.SuppressFinalize(this);");
    w.close();
    w.line("");
    w.open("private void DisposeCore()");
    w.line("nint source = Interlocked.Exchange(ref _source, 0);");
    w.open("if (source == 0)");
    w.line("return;");
    w.close();
    w.line("nint context = Interlocked.Exchange(ref _context, 0);");
    w.open("if (context == 0)");
    w.line("_ = _remove(source, _token);");
    w.line("_ = Com.Release(source);");
    w.line("return;");
    w.close();
    w.line("nint currentToken;");
    w.open("if (Interop.CoGetContextToken(&currentToken) >= 0 && currentToken == _contextToken)");
    w.line("_ = _remove(source, _token);");
    w.line("_ = Com.Release(source);");
    w.close();
    w.open("else");
    w.line(
        "RevokeData revoke = new RevokeData { Source = source, Token = _token, Remove = _remove };",
    );
    w.line("ContextCallData data = new ContextCallData { UserDefined = (nint)(&revoke) };");
    w.line("Guid iid = s_noReentrancy;");
    w.line(
        "// If the originating context is gone, leaking is safer than calling or releasing here.",
    );
    w.line("_ = ((delegate* unmanaged<nint, nint, ContextCallData*, Guid*, int, nint, int>)(*(void***)context)[3])(context, (nint)(delegate* unmanaged<ContextCallData*, int>)&RevokeInContext, &data, &iid, 5, 0);");
    w.close();
    w.line("_ = Com.Release(context);");
    w.close();
    w.line("");
    w.line("[UnmanagedCallersOnly]");
    w.open("private static int RevokeInContext(ContextCallData* data)");
    w.line("RevokeData* revoke = (RevokeData*)data->UserDefined;");
    w.line("_ = revoke->Remove(revoke->Source, revoke->Token);");
    w.line("_ = Com.Release(revoke->Source);");
    w.line("return 0;");
    w.close();
    w.line("");
    w.open("~EventRevoker()");
    w.line("DisposeCore();");
    w.close();
    w.close();

    w.line("");

    write_callback_support(w, async_operation);

    w.line("");

    if raw {
        write_raw_winrt_support(w);
    } else {
        w.open("internal static unsafe class WinRT");
        w.line("private static readonly Guid s_iagile = new Guid(0x94ea2b94, 0xe9cc, 0x49e0, 0xc0, 0xff, 0xee, 0x64, 0xca, 0x8f, 0x5b, 0x90);");
        w.line("[ThreadStatic]");
        w.line("private static MtaUsage? s_mta;");
        w.line("");
        w.open(
        "public static nint Activate(ref nint moduleCache, ref nint factoryCache, string classId, Guid iid)",
    );
        w.line("s_mta ??= new MtaUsage();");
        w.line("nint factory = Volatile.Read(ref factoryCache);");
        w.line("bool releaseFactory = false;");
        w.open("if (factory == 0)");
        w.line("factory = LoadActivationFactory(ref moduleCache, classId);");
        w.line("Guid iidAgile = s_iagile;");
        w.line("nint agile;");
        w.open("if (Com.QueryInterface(factory, &iidAgile, &agile) >= 0)");
        w.line("_ = Com.Release(agile);");
        w.line("nint existing = Interlocked.CompareExchange(ref factoryCache, factory, 0);");
        w.open("if (existing != 0)");
        w.line("_ = Com.Release(factory);");
        w.line("factory = existing;");
        w.close();
        w.close();
        w.open("else");
        w.line("releaseFactory = true;");
        w.close();
        w.close();
        w.open("try");
        w.line("nint inspectable;");
        w.line("Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)factory)[6])(factory, &inspectable));");
        w.open("try");
        w.line("nint self;");
        w.line("Com.Check(Com.QueryInterface(inspectable, &iid, &self));");
        w.line("return self;");
        w.close();
        w.open("finally");
        w.line("_ = Com.Release(inspectable);");
        w.close();
        w.close();
        w.open("finally");
        w.open("if (releaseFactory)");
        w.line("_ = Com.Release(factory);");
        w.close();
        w.close();
        w.close();
        w.line("");
        // Acquire a specific activation-factory interface (a static or creation factory) by IID from the
        // registration-free `DllGetActivationFactory` path. An agile factory is cached and shared across
        // calls; a non-agile factory is returned as a transient lease the caller releases in `finally`.
        // Only function pointers and `ref`/by-value arguments cross the ABI, so no managed delegate is
        // allocated per call.
        w.open("public static FactoryLease GetActivationFactory(ref nint moduleCache, ref nint factoryCache, string classId, Guid iid)");
        w.line("s_mta ??= new MtaUsage();");
        w.line("nint cached = Volatile.Read(ref factoryCache);");
        w.open("if (cached != 0)");
        w.line("return new FactoryLease(cached, 0);");
        w.close();
        w.line("nint factory = LoadActivationFactory(ref moduleCache, classId);");
        w.line("nint requested;");
        w.open("try");
        w.line("Com.Check(Com.QueryInterface(factory, &iid, &requested));");
        w.close();
        w.open("finally");
        w.line("_ = Com.Release(factory);");
        w.close();
        w.line("Guid iidAgile = s_iagile;");
        w.line("nint agile;");
        w.open("if (Com.QueryInterface(requested, &iidAgile, &agile) >= 0)");
        w.line("_ = Com.Release(agile);");
        w.line("nint existing = Interlocked.CompareExchange(ref factoryCache, requested, 0);");
        w.open("if (existing != 0)");
        w.line("_ = Com.Release(requested);");
        w.line("return new FactoryLease(existing, 0);");
        w.close();
        w.line("return new FactoryLease(requested, 0);");
        w.close();
        w.line("return new FactoryLease(requested, requested);");
        w.close();
        w.line("");
        w.open("private static nint LoadActivationFactory(ref nint moduleCache, string classId)");
        w.line("nint module = Volatile.Read(ref moduleCache);");
        w.open("if (module == 0)");
        w.line("module = LoadModule(classId);");
        w.open("if (module != 0)");
        w.line("nint existing = Interlocked.CompareExchange(ref moduleCache, module, 0);");
        w.open("if (existing != 0)");
        w.line("_ = Interop.FreeLibrary(module);");
        w.line("module = existing;");
        w.close();
        w.close();
        w.close();
        w.line("nint factory = 0;");
        w.line("int hr = unchecked((int)0x80040111);");
        w.open("fixed (char* id = classId)");
        w.line("Interop.HstringHeader header;");
        w.line(
        "nint hstring = Interop.CreateStringReference((ushort*)id, (uint)classId.Length, &header);",
    );
        w.open("if (module != 0)");
        w.line("nint proc = Interop.GetProcAddress(module, \"DllGetActivationFactory\");");
        w.open("if (proc != 0)");
        w.line("hr = ((delegate* unmanaged<nint, nint*, int>)proc)(hstring, &factory);");
        w.close();
        w.close();
        w.open("if (hr < 0)");
        w.line("Guid iid = new Guid(0x00000035, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);");
        w.line("hr = Interop.RoGetActivationFactory(hstring, &iid, &factory);");
        w.close();
        w.close();
        w.line("Com.Check(hr);");
        w.line("return factory;");
        w.close();
        w.line("");
        w.open("private static nint LoadModule(string classId)");
        w.line("nint module = 0;");
        w.line("string name = classId;");
        w.open("while (true)");
        w.line("int dot = name.LastIndexOf('.');");
        w.open("if (dot < 0)");
        w.line("break;");
        w.close();
        w.line("name = name.Substring(0, dot);");
        w.line("module = Interop.LoadLibrary(name + \".dll\");");
        w.open("if (module != 0)");
        w.line("break;");
        w.close();
        w.close();
        w.line("return module;");
        w.close();
        w.line("");
        w.open("private sealed class MtaUsage");
        w.line("private nint _cookie;");
        w.line("");
        w.open("public MtaUsage()");
        w.line("nint cookie;");
        w.line("Com.Check(Interop.CoIncrementMTAUsage(&cookie));");
        w.line("_cookie = cookie;");
        w.close();
        w.line("");
        w.open("~MtaUsage()");
        w.open("if (_cookie != 0)");
        w.line("_ = Interop.CoDecrementMTAUsage(_cookie);");
        w.close();
        w.close();
        w.close();
        w.close();
    }

    w.line("");

    w.open("internal static unsafe partial class Interop");
    w.line("[LibraryImport(\"ole32.dll\")]");
    w.line("public static partial int CoIncrementMTAUsage(nint* cookie);");
    w.line("");
    w.line("[LibraryImport(\"ole32.dll\")]");
    w.line("public static partial int CoDecrementMTAUsage(nint cookie);");
    w.line("");
    w.line("[LibraryImport(\"ole32.dll\")]");
    w.line("public static partial int CoGetContextToken(nint* token);");
    w.line("");
    w.line("[LibraryImport(\"ole32.dll\")]");
    w.line("public static partial int CoGetObjectContext(Guid* iid, nint* context);");
    w.line("");
    w.line("[LibraryImport(\"combase.dll\")]");
    w.line(
        "public static partial int RoGetActivationFactory(nint classId, Guid* iid, nint* factory);",
    );
    w.line("");
    w.line("[LibraryImport(\"kernel32.dll\", EntryPoint = \"LoadLibraryW\", StringMarshalling = StringMarshalling.Utf16)]");
    w.line("public static partial nint LoadLibrary(string name);");
    w.line("");
    w.line("[LibraryImport(\"kernel32.dll\")]");
    w.line("[return: MarshalAs(UnmanagedType.Bool)]");
    w.line("public static partial bool FreeLibrary(nint module);");
    w.line("");
    w.line("[LibraryImport(\"kernel32.dll\", EntryPoint = \"GetProcAddress\", StringMarshalling = StringMarshalling.Utf8)]");
    w.line("public static partial nint GetProcAddress(nint module, string name);");
    w.line("");
    w.line("[LibraryImport(\"combase.dll\")]");
    w.line("public static partial int WindowsCreateString(ushort* sourceString, uint length, nint* str);");
    w.line("");
    w.line("[LibraryImport(\"combase.dll\")]");
    w.line("public static partial int WindowsCreateStringReference(ushort* sourceString, uint length, HstringHeader* header, nint* str);");
    w.line("");
    w.line("[LibraryImport(\"combase.dll\")]");
    w.line("public static partial ushort* WindowsGetStringRawBuffer(nint str, out uint length);");
    w.line("");
    w.line("[LibraryImport(\"combase.dll\")]");
    w.line("public static partial int WindowsDeleteString(nint str);");
    w.line("");
    w.line("[LibraryImport(\"oleaut32.dll\")]");
    w.line("private static partial int GetErrorInfo(uint reserved, nint* info);");
    w.line("");
    w.open("public static string? TakeErrorMessage()");
    w.line("nint info = 0;");
    w.line("_ = GetErrorInfo(0, &info);");
    w.open("if (info == 0)");
    w.line("return null;");
    w.close();
    w.open("try");
    w.line("string? message = GetRestrictedErrorMessage(info);");
    w.line("return string.IsNullOrEmpty(message) ? GetErrorDescription(info) : message;");
    w.close();
    w.open("finally");
    w.line("_ = Com.Release(info);");
    w.close();
    w.close();
    w.line("");
    w.open("private static string? GetRestrictedErrorMessage(nint info)");
    w.line("Guid iid = new Guid(0x82ba7092, 0x4c88, 0x427d, 0xa7, 0xbc, 0x16, 0xdd, 0x93, 0xfe, 0xb6, 0x7e);");
    w.line("nint restrictedInfo;");
    w.open("if (Com.QueryInterface(info, &iid, &restrictedInfo) < 0)");
    w.line("return null;");
    w.close();
    w.line("nint description = 0;");
    w.line("nint restricted = 0;");
    w.line("nint capability = 0;");
    w.open("try");
    w.line("int code;");
    w.line("int hr = ((delegate* unmanaged<nint, nint*, int*, nint*, nint*, int>)(*(void***)restrictedInfo)[3])(restrictedInfo, &description, &code, &restricted, &capability);");
    w.open("if (hr < 0)");
    w.line("return null;");
    w.close();
    w.line("string? message = restricted == 0 ? null : Marshal.PtrToStringBSTR(restricted);");
    w.line("return string.IsNullOrEmpty(message) && description != 0 ? Marshal.PtrToStringBSTR(description) : message;");
    w.close();
    w.open("finally");
    w.open("if (description != 0)");
    w.line("Marshal.FreeBSTR(description);");
    w.close();
    w.open("if (restricted != 0)");
    w.line("Marshal.FreeBSTR(restricted);");
    w.close();
    w.open("if (capability != 0)");
    w.line("Marshal.FreeBSTR(capability);");
    w.close();
    w.line("_ = Com.Release(restrictedInfo);");
    w.close();
    w.close();
    w.line("");
    w.open("private static string? GetErrorDescription(nint info)");
    w.line("nint description = 0;");
    w.open("try");
    w.line("int hr = ((delegate* unmanaged<nint, nint*, int>)(*(void***)info)[5])(info, &description);");
    w.line("return hr < 0 || description == 0 ? null : Marshal.PtrToStringBSTR(description);");
    w.close();
    w.open("finally");
    w.open("if (description != 0)");
    w.line("Marshal.FreeBSTR(description);");
    w.close();
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static nint CreateString(string? source)");
    w.open("if (source is null)");
    w.line("return 0;");
    w.close();
    w.open("fixed (char* buffer = source)");
    w.line("nint value = 0;");
    w.line("int hr = WindowsCreateString((ushort*)buffer, (uint)source.Length, &value);");
    w.open("if (hr < 0)");
    w.line("DeleteHstring(ref value);");
    w.line("Com.Check(hr);");
    w.close();
    w.line("return value;");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static nint CreateStringReference(ushort* source, uint length, HstringHeader* header)");
    w.line("nint value;");
    w.line("Com.Check(WindowsCreateStringReference(source, length, header, &value));");
    w.line("return value;");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static string FromHstring(nint value)");
    w.open("if (value == 0)");
    w.line("return string.Empty;");
    w.close();
    w.open("try");
    w.line("return FromHstringBorrowed(value);");
    w.close();
    w.open("finally");
    w.line("_ = WindowsDeleteString(value);");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static string FromHstringBorrowed(nint value)");
    w.open("if (value == 0)");
    w.line("return string.Empty;");
    w.close();
    w.line("ushort* buffer = WindowsGetStringRawBuffer(value, out uint length);");
    w.line("return new string((char*)buffer, 0, (int)length);");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static string TakeHstring(ref nint value)");
    w.line("nint current = value;");
    w.line("value = 0;");
    w.line("return FromHstring(current);");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static void DeleteHstring(ref nint value)");
    w.line("nint current = value;");
    w.line("value = 0;");
    w.open("if (current != 0)");
    w.line("_ = WindowsDeleteString(current);");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static bool[] FromBooleanArray(ref uint length, ref byte* value)");
    w.line("uint count = length;");
    w.line("byte* source = value;");
    w.line("length = 0;");
    w.line("value = null;");
    w.open("try");
    w.line("bool[] result = new bool[count];");
    w.open("fixed (bool* target = result)");
    w.line("new ReadOnlySpan<byte>(source, checked((int)count)).CopyTo(new Span<byte>((byte*)target, checked((int)count)));");
    w.close();
    w.line("return result;");
    w.close();
    w.open("finally");
    w.line("Marshal.FreeCoTaskMem((nint)source);");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static TSurface[] FromArray<TSurface, TAbi>(ref uint length, ref TAbi* value) where TSurface : unmanaged where TAbi : unmanaged");
    w.line("uint count = length;");
    w.line("TAbi* source = value;");
    w.line("length = 0;");
    w.line("value = null;");
    w.open("try");
    w.open("if (sizeof(TSurface) != sizeof(TAbi))");
    w.line("throw new InvalidOperationException(\"Array surface and ABI element sizes differ.\");");
    w.close();
    w.line("TSurface[] result = new TSurface[count];");
    w.open("if (count != 0)");
    w.open("fixed (TSurface* target = result)");
    w.line("long bytes = checked((long)count * sizeof(TSurface));");
    w.line("Buffer.MemoryCopy(source, target, bytes, bytes);");
    w.close();
    w.close();
    w.line("return result;");
    w.close();
    w.open("finally");
    w.line("Marshal.FreeCoTaskMem((nint)source);");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static string[] FromStringArray(ref uint length, ref nint* value)");
    w.line("uint count = length;");
    w.line("nint* source = value;");
    w.line("length = 0;");
    w.line("value = null;");
    w.open("try");
    w.line("string[] result = new string[count];");
    w.open("for (uint i = 0; i < count; i++)");
    w.line("nint current = source[i];");
    w.line("source[i] = 0;");
    w.line("result[i] = FromHstring(current);");
    w.close();
    w.line("return result;");
    w.close();
    w.open("finally");
    w.line("FreeStringArray(count, source);");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static T?[] FromObjectArray<T>(ref uint length, ref nint* value) where T : ComObject, IComInterface<T>");
    w.line("uint count = length;");
    w.line("nint* source = value;");
    w.line("length = 0;");
    w.line("value = null;");
    w.line("T?[]? result = null;");
    w.line("uint converted = 0;");
    w.open("try");
    w.line("result = new T?[count];");
    w.open("for (; converted < count; converted++)");
    w.line("nint current = source[converted];");
    w.line("source[converted] = 0;");
    w.line("result[converted] = Com.Wrap<T>(current);");
    w.close();
    w.line("return result;");
    w.close();
    w.open("catch");
    w.open("if (result is not null)");
    w.open("for (uint i = 0; i < converted; i++)");
    w.line("result[i]?.Dispose();");
    w.close();
    w.close();
    w.line("throw;");
    w.close();
    w.open("finally");
    w.line("FreeObjectArray(count, source);");
    w.close();
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static void FreeStringArray(uint length, nint* value)");
    w.open("if (value == null)");
    w.line("return;");
    w.close();
    w.open("for (uint i = 0; i < length; i++)");
    w.open("if (value[i] != 0)");
    w.line("_ = WindowsDeleteString(value[i]);");
    w.close();
    w.close();
    w.line("Marshal.FreeCoTaskMem((nint)value);");
    w.close();
    w.line("");
    w.line("[MethodImpl(MethodImplOptions.AggressiveInlining)]");
    w.open("public static void FreeObjectArray(uint length, nint* value)");
    w.open("if (value == null)");
    w.line("return;");
    w.close();
    w.open("for (uint i = 0; i < length; i++)");
    w.open("if (value[i] != 0)");
    w.line("_ = Com.Release(value[i]);");
    w.close();
    w.close();
    w.line("Marshal.FreeCoTaskMem((nint)value);");
    w.close();
    w.line("");
    w.line("[StructLayout(LayoutKind.Sequential, Size = 24)]");
    w.line("public struct HstringHeader { }");
    w.close();

    w.close();
}

/// Emits the shared reverse-vtable support (`WindowsCsharp.Callback`) that backs projected
/// delegates and async completion handlers. A callback object is a heap block of four native words
/// containing a vtable pointer, reference count, a `GCHandle` to the managed callback, and a pointer
/// to the delegate's IID. Its vtable uses these shared `IUnknown` thunks and an `Invoke` thunk at
/// slot 3. `QueryInterface` answers the delegate IID (read from the block), `IUnknown`, and
/// `IAgileObject`; `Release` frees the block and the `GCHandle` at zero.
fn write_callback_support(w: &mut Writer, async_operation: bool) {
    w.open("internal static unsafe class Callback");

    w.line("private static readonly Guid s_iunknown = new Guid(0x00000000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);");
    w.line("private static readonly Guid s_iagile = new Guid(0x94ea2b94, 0xe9cc, 0x49e0, 0xc0, 0xff, 0xee, 0x64, 0xca, 0x8f, 0x5b, 0x90);");
    w.line("");

    w.line("public static readonly nint QueryInterfacePtr = (nint)(delegate* unmanaged<nint, Guid*, nint*, int>)&QueryInterface;");
    w.line(
        "public static readonly nint AddRefPtr = (nint)(delegate* unmanaged<nint, uint>)&AddRef;",
    );
    w.line(
        "public static readonly nint ReleasePtr = (nint)(delegate* unmanaged<nint, uint>)&Release;",
    );
    if async_operation {
        w.line("private static readonly nint* s_completedVtable = BuildCompletedVtable();");
    }
    w.line("");

    if async_operation {
        w.open("private static nint* BuildCompletedVtable()");
        w.line("nint* vtable = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));");
        w.line("vtable[0] = QueryInterfacePtr;");
        w.line("vtable[1] = AddRefPtr;");
        w.line("vtable[2] = ReleasePtr;");
        w.line("vtable[3] = (nint)(delegate* unmanaged<nint, nint, int, int>)&CompletedInvoke;");
        w.line("return vtable;");
        w.close();
        w.line("");

        w.open("private sealed class Completion");
        w.line("private Action? _continuation;");
        w.line("internal Completion(Action continuation) => _continuation = continuation;");
        w.line("");
        w.open("internal void Invoke()");
        w.line("Interlocked.Exchange(ref _continuation, null)?.Invoke();");
        w.close();
        w.close();
        w.line("");

        w.open("public static nint AllocCompleted(Guid* iid, Action continuation)");
        w.line("return Alloc((nint)s_completedVtable, iid, new Completion(continuation));");
        w.close();
        w.line("");
    }

    // Pin a delegate's IID in native memory so the shared QueryInterface thunk can read it from the
    // callback block. The allocation lives for the process, like the vtable it accompanies.
    w.open("public static Guid* PinIid(Guid iid)");
    w.line("Guid* p = (Guid*)NativeMemory.Alloc((nuint)sizeof(Guid));");
    w.line("*p = iid;");
    w.line("return p;");
    w.close();
    w.line("");

    // Allocate a callback block: [0] vtable, [1] refcount = 1, [2] GCHandle to the callback, [3]
    // IID pointer. The returned pointer is the COM interface pointer handed to native code.
    w.open("public static nint Alloc(nint vtable, Guid* iid, object callback)");
    w.line("nint* block = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));");
    w.line("block[0] = vtable;");
    w.line("block[1] = 1;");
    w.line("block[2] = GCHandle.ToIntPtr(GCHandle.Alloc(callback));");
    w.line("block[3] = (nint)iid;");
    w.line("return (nint)block;");
    w.close();
    w.line("");

    w.open("public static object Target(nint self)");
    w.line("nint* block = (nint*)self;");
    w.line("return GCHandle.FromIntPtr(block[2]).Target!;");
    w.close();
    w.line("");

    if async_operation {
        w.line("[UnmanagedCallersOnly]");
        w.open("private static int CompletedInvoke(nint self, nint operation, int status)");
        w.open("try");
        w.line("((Completion)Target(self)).Invoke();");
        w.line("return 0;");
        w.close();
        w.open("catch (Exception error)");
        w.line("return Marshal.GetHRForException(error);");
        w.close();
        w.close();
        w.line("");
    }

    w.line("[UnmanagedCallersOnly]");
    w.open("private static int QueryInterface(nint self, Guid* iid, nint* ppv)");
    w.open("if (ppv == null)");
    w.line("return unchecked((int)0x80004003);");
    w.close();
    w.line("nint* block = (nint*)self;");
    w.line("Guid* typeIid = (Guid*)block[3];");
    w.open("if (*iid == *typeIid || *iid == s_iunknown || *iid == s_iagile)");
    w.line("_ = Interlocked.Increment(ref *(int*)(block + 1));");
    w.line("*ppv = self;");
    w.line("return 0;");
    w.close();
    w.line("*ppv = 0;");
    w.line("return unchecked((int)0x80004002);");
    w.close();
    w.line("");

    w.line("[UnmanagedCallersOnly]");
    w.open("private static uint AddRef(nint self)");
    w.line("nint* block = (nint*)self;");
    w.line("return (uint)Interlocked.Increment(ref *(int*)(block + 1));");
    w.close();
    w.line("");

    w.line("[UnmanagedCallersOnly]");
    w.open("private static uint Release(nint self)");
    w.line("nint* block = (nint*)self;");
    w.line("int count = Interlocked.Decrement(ref *(int*)(block + 1));");
    w.open("if (count == 0)");
    w.line("GCHandle.FromIntPtr(block[2]).Free();");
    w.line("NativeMemory.Free(block);");
    w.close();
    w.line("return (uint)count;");
    w.close();

    w.close();
}
