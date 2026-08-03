use windows_metadata::reader::{Index, ParamDirection, TypeCategory, TypeDef};
use windows_metadata::{HasAttributes, Type, TypeAttributes, Value};

/// A GUID decoded from a `GuidAttribute`, used as an interface IID.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Guid(
    pub u32,
    pub u16,
    pub u16,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
);

impl Guid {
    /// Emits the `new Guid(0x..., ...)` constructor call used by the projection.
    pub fn to_cs(self) -> String {
        format!(
            "new Guid(0x{:08x}, 0x{:04x}, 0x{:04x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x})",
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10
        )
    }

    pub fn to_guid_string(self) -> String {
        format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10
        )
    }
}

/// The subset of ABI parameter and return types the projection supports today.
#[derive(Clone, PartialEq, Eq)]
pub enum CsType {
    /// Native `void`, used only as a pointer target.
    Void,
    /// A blittable scalar whose C# surface, ABI-input, and ABI-output spellings share one name
    /// (`int`, `uint`, `float`, ...). The out-pointer form is `{name}*`.
    Scalar(&'static str),
    /// A WinRT `Boolean`. The C# surface is `bool`, while the ABI is one byte.
    Boolean,
    /// Win32 `BOOL`: idiomatic `bool` on the surface and a four-byte integer at the ABI.
    Win32Bool,
    /// A native `HRESULT`. Public wrappers translate failures and otherwise return no value.
    HResult,
    /// A WinRT `HSTRING`. The surface type is `string`; the ABI type is `nint` (the `HSTRING`
    /// handle). Conversion uses the combase string helpers.
    String,
    /// A WinRT enum. `name` is the namespace-qualified C# type (`Bench.Color`); `underlying` is the
    /// blittable backing scalar (`int`, `uint`, ...) crossed on the ABI. The surface is the enum
    /// type while the ABI is the scalar, so calls cast across the boundary but add no copying.
    Enum {
        name: String,
        underlying: &'static str,
    },
    /// A WinRT or native struct. `name` is the namespace-qualified C# surface type (`Sample.Rect`);
    /// `abi_name` names a generated ABI companion when one or more fields need conversion.
    /// `owns_abi` is true when that companion contains owned `HSTRING` handles.
    Struct {
        name: String,
        abi_name: Option<String>,
        owns_abi: bool,
    },
    /// A genuine Win32 opaque native handle (`Windows.Win32.HWND`, `Windows.Win32.HANDLE`, ...):
    /// a `NativeTypedefAttribute` type whose only field (`Value`) is an opaque `void*` (see
    /// [`native_handle_value`]). `name` is the namespace-qualified C# type. Like [`Self::Struct`],
    /// the emitted type (an explicit single-`nint`-field `readonly struct`) is blittable, so its
    /// surface and ABI spellings are the same type and it crosses the ABI by value with no copying
    /// and no boundary cast - unlike a scalar identifier alias (`COLORREF`, `ATOM`, ...) or a
    /// pointer-to-named-type alias (`PWSTR`, `LPRECT`, ...), both of which stay collapsed to their
    /// existing scalar/pointer representation rather than becoming a distinct nominal type.
    Handle { name: String },
    /// A native pointer. `depth` is the number of pointer indirections.
    Pointer {
        element: Box<Self>,
        mutable: bool,
        depth: usize,
    },
    /// A native callback typedef. C# exposes the literal unmanaged function-pointer signature so
    /// callback storage, context, and lifetime remain explicit and allocation-free.
    Callback {
        params: Vec<Self>,
        return_type: Box<Self>,
        convention: CallingConvention,
    },
    /// The one required interface out-parameter selected as an owning return on an
    /// HRESULT-returning Win32 function or native COM method.
    ComOut { name: String },
    /// A projected reference type: a runtime class, interface, or delegate, `name` being the
    /// namespace-qualified C# type (`Sample.IWidget`). It crosses the ABI as a single interface
    /// pointer (`nint`). The projected sealed class owns the returned `+1` reference through its
    /// `SafeHandle` base; an `[in]` parameter takes a lease so disposal cannot release the borrowed
    /// pointer during the call.
    Object { name: String },
    /// Metadata `Object` (`IInspectable`). The surface accepts any projected `ComObject`; returned
    /// pointers use the concrete projected `Windows.Foundation.IInspectable` owner.
    Inspectable,
    /// A closed `IReference<T>` over an unmanaged value. The C# surface is `T?`; inputs use a
    /// temporary native COM box and outputs unbox and release the returned interface pointer.
    Reference { value: Box<Self>, iid: Guid },
    /// A closed `IAsyncOperation<T>` over an unmanaged result.
    Async { value: Box<Self>, iid: Guid },
    /// A WinRT array. Input arrays cross as `(length, pointer)`; output arrays and array returns
    /// cross as `(length*, pointer**)` and are freed with `CoTaskMemFree` after copying.
    Array { element: Box<Self>, output: bool },
    /// A reference to an open generic's type parameter (`T`) inside the generic interface's own
    /// definition. It crosses the ABI by value as `T` under a `where T : unmanaged` constraint, so
    /// its surface, ABI-input, and ABI-output spellings are all `T` / `T*` and conversions are
    /// identity.
    TypeParam(String),
}

impl CsType {
    /// Maps a metadata type to a `CsType`, returning `None` for shapes the projection does not yet
    /// handle. `index` resolves named value types (enums and blittable structs) - a struct projects
    /// only when every field has a supported ABI representation, so a signature never names a
    /// struct the generator dropped.
    pub fn map(index: &Index, ty: &Type) -> Option<Self> {
        if matches!(ty, Type::Bool) {
            return Some(Self::Boolean);
        }
        if let Some(scalar) = scalar_name(ty) {
            return Some(Self::Scalar(scalar));
        }
        Some(match ty {
            Type::Void => Self::Void,
            Type::String => Self::String,
            Type::Object => Self::Inspectable,
            Type::Array(element) => {
                let element = Self::map(index, element)?;
                if !element.is_array_element() {
                    return None;
                }
                Self::Array {
                    element: Box::new(element),
                    output: false,
                }
            }
            Type::RefMut(inner) => {
                if let Type::Array(element) = inner.as_ref() {
                    let element = Self::map(index, element)?;
                    if !element.is_array_element() {
                        return None;
                    }
                    Self::Array {
                        element: Box::new(element),
                        output: true,
                    }
                } else {
                    Self::Pointer {
                        element: Box::new(Self::map(index, inner)?),
                        mutable: true,
                        depth: 1,
                    }
                }
            }
            Type::RefConst(inner) => Self::Pointer {
                element: Box::new(Self::map(index, inner)?),
                mutable: false,
                depth: 1,
            },
            Type::PtrMut(inner, depth) => Self::Pointer {
                element: Box::new(Self::map(index, inner)?),
                mutable: true,
                depth: *depth,
            },
            Type::PtrConst(inner, depth) => Self::Pointer {
                element: Box::new(Self::map(index, inner)?),
                mutable: false,
                depth: *depth,
            },
            Type::Generic(name, _) => Self::TypeParam(name.clone()),
            Type::ValueName(tn) => {
                if tn.name == "BOOL"
                    && (tn.namespace == "Windows.Win32" || is_native_i32_typedef(index, tn))
                {
                    return Some(Self::Win32Bool);
                }
                if tn.name == "HRESULT"
                    && (tn.namespace == "Windows.Win32" || is_native_i32_typedef(index, tn))
                {
                    return Some(Self::HResult);
                }
                if tn.namespace == "Windows.Foundation" && tn.name == "HResult" {
                    return Some(Self::HResult);
                }
                let def = index.get(&tn.namespace, &tn.name).next()?;
                if native_handle_value(def).is_some() {
                    return Some(Self::Handle {
                        name: format!("{}.{}", tn.namespace, tn.name),
                    });
                }
                if let Some(underlying) = native_typedef_underlying(def) {
                    return Self::map(index, &underlying);
                }
                let name = format!("{}.{}", tn.namespace, tn.name);
                match def.category() {
                    TypeCategory::Enum => Self::Enum {
                        name,
                        underlying: enum_underlying(def)?,
                    },
                    TypeCategory::Struct => {
                        if def.has_attribute("AlignmentAttribute") {
                            return None;
                        }
                        if !struct_fields_are_native(index, def) {
                            return None;
                        }
                        let owns_abi = struct_owns_abi(index, def);
                        let needs_abi = owns_abi || struct_needs_abi(index, def);
                        if needs_abi && def.flags().contains(TypeAttributes::ExplicitLayout) {
                            return None;
                        }
                        let abi_name = needs_abi.then(|| format!("{name}Abi"));
                        Self::Struct {
                            name,
                            abi_name,
                            owns_abi,
                        }
                    }
                    TypeCategory::Delegate => native_callback(index, def)?,
                    _ => return None,
                }
            }
            Type::ClassName(tn) if !tn.generics.is_empty() => {
                if tn.namespace == "Windows.Foundation"
                    && tn.name == "IReference`1"
                    && tn.generics.len() == 1
                {
                    let value = Self::map(index, &tn.generics[0])?;
                    if !value.is_unmanaged() {
                        return None;
                    }
                    let piid = index
                        .get("Windows.Foundation", "IReference")
                        .next()
                        .and_then(crate::guid_attribute)?;
                    let iid = crate::guid::generic_iid(index, piid, &tn.generics)?;
                    return Some(Self::Reference {
                        value: Box::new(value),
                        iid,
                    });
                }
                if tn.namespace == "Windows.Foundation"
                    && tn.name == "IAsyncOperation`1"
                    && tn.generics.len() == 1
                {
                    let value = Self::map(index, &tn.generics[0])?;
                    if !value.is_unmanaged() && !value.is_object() && !matches!(value, Self::String)
                    {
                        return None;
                    }
                    let piid = index
                        .get("Windows.Foundation", "IAsyncOperation")
                        .next()
                        .and_then(crate::guid_attribute)?;
                    let iid = crate::guid::generic_iid(index, piid, &tn.generics)?;
                    return Some(Self::Async {
                        value: Box::new(value),
                        iid,
                    });
                }
                // A closed generic instantiation (for example `IVector<Int32>`). It crosses the ABI
                // as one interface pointer like any other object; its surface is the C# generic name
                // and its IID is derived at generation time (see `crate::guid`). Only the generic
                // shapes the projection emits (and only with `unmanaged` type arguments) are
                // supported.
                Self::Object {
                    name: generic_name(index, tn)?,
                }
            }
            Type::ClassName(tn) => {
                let def = index.get(&tn.namespace, &tn.name).next()?;
                let name = format!("{}.{}", tn.namespace, tn.name);
                match def.category() {
                    TypeCategory::Class if def.flags().contains(TypeAttributes::WindowsRuntime) => {
                        Self::Object { name }
                    }
                    TypeCategory::Interface => Self::Object { name },
                    // A delegate crosses the ABI as an interface pointer just like a class or
                    // interface, but it projects only when its `Invoke` signature is itself
                    // projectable (so the generator emits the delegate type the signature names).
                    TypeCategory::Delegate
                        if def.flags().contains(TypeAttributes::WindowsRuntime) =>
                    {
                        delegate_invoke(index, def)?;
                        Self::Object { name }
                    }
                    TypeCategory::Delegate => native_callback(index, def)?,
                    _ => return None,
                }
            }
            _ => return None,
        })
    }

    /// The idiomatic C# surface type (`int`, `string`, `Bench.Color`, `Sample.Rect`,
    /// `Sample.IWidget`, ...).
    pub fn surface(&self) -> String {
        match self {
            Self::Void => "void".to_string(),
            Self::Scalar(name) => name.to_string(),
            Self::Boolean => "bool".to_string(),
            Self::Win32Bool => "bool".to_string(),
            Self::HResult => "int".to_string(),
            Self::String => "string".to_string(),
            Self::Enum { name, .. } => name.clone(),
            Self::Struct { name, .. } => name.clone(),
            Self::Handle { name } => name.clone(),
            Self::Pointer { .. } => self.abi_in(),
            Self::Callback { .. } => self.abi_in(),
            Self::ComOut { name } => format!("{name}?"),
            Self::Object { name } => format!("{name}?"),
            Self::Inspectable => "WindowsCsharp.ComObject?".to_string(),
            Self::Reference { value, .. } => format!("{}?", value.surface()),
            Self::Async { value, .. } => {
                format!("Windows.Foundation.IAsyncOperation<{}>?", value.surface())
            }
            Self::Array { element, .. } if matches!(element.as_ref(), Self::Inspectable) => {
                "Windows.Foundation.IInspectable?[]".to_string()
            }
            Self::Array { element, .. } => format!("{}[]", element.surface()),
            Self::TypeParam(name) => name.clone(),
        }
    }

    /// The non-nullable spelling used when a projected reference type is a generic collection
    /// element. C# does not permit nullable reference annotations in `typeof`, and WinRT generic
    /// identity likewise depends on the runtime type rather than its nullable annotation.
    pub fn collection_surface(&self) -> String {
        match self {
            Self::Object { name } => name.clone(),
            Self::Inspectable => "Windows.Foundation.IInspectable".to_string(),
            Self::Async { value, .. } => {
                format!("Windows.Foundation.IAsyncOperation<{}>", value.surface())
            }
            Self::Array { .. } => self.surface(),
            _ => self.surface(),
        }
    }

    /// The nullable-aware generic argument spelling used on the public collection surface.
    pub fn collection_generic_surface(&self) -> String {
        match self {
            Self::Inspectable => "Windows.Foundation.IInspectable?".to_string(),
            _ => self.surface(),
        }
    }

    /// The surface type used for a native-to-managed delegate parameter. Strings are copied from
    /// the borrowed HSTRING buffer. Object pointers use the projected callback-confined view rather
    /// than allocating an owning wrapper and issuing an AddRef on every invocation.
    pub fn callback_surface(&self) -> String {
        match self {
            Self::Object { name } => format!("{name}.Borrowed"),
            Self::Inspectable => "Windows.Foundation.IInspectable.Borrowed".to_string(),
            Self::Reference { .. } => self.surface(),
            Self::Async { .. } => self.surface(),
            Self::Array { .. } => self.surface(),
            _ => self.surface(),
        }
    }

    /// The ABI type when passed by value as an input (`int`, `nint` for a string handle or an
    /// interface pointer, the underlying scalar for an enum, the blittable struct itself for a
    /// struct).
    pub fn abi_in(&self) -> String {
        match self {
            Self::Void => "void".to_string(),
            Self::Scalar(name) => name.to_string(),
            Self::Boolean => "byte".to_string(),
            Self::Win32Bool | Self::HResult => "int".to_string(),
            Self::String => "nint".to_string(),
            Self::Enum { underlying, .. } => underlying.to_string(),
            Self::Struct { name, abi_name, .. } => abi_name.as_ref().unwrap_or(name).clone(),
            Self::Handle { name } => name.clone(),
            Self::Pointer { element, depth, .. } => {
                format!("{}{}", element.abi_in(), "*".repeat(*depth))
            }
            Self::Callback {
                params,
                return_type,
                convention,
            } => {
                let mut types: Vec<_> = params.iter().map(Self::abi_in).collect();
                types.push(return_type.abi_in());
                format!(
                    "delegate* unmanaged[{}]<{}>",
                    convention.cs_name(),
                    types.join(", ")
                )
            }
            Self::ComOut { .. } => "nint".to_string(),
            Self::Object { .. }
            | Self::Inspectable
            | Self::Reference { .. }
            | Self::Async { .. } => "nint".to_string(),
            Self::Array { .. } => unreachable!("arrays expand to multiple ABI parameters"),
            Self::TypeParam(name) => name.clone(),
        }
    }

    /// The ABI out-pointer type used for returns (`int*`, `nint*`, `Sample.Rect*`).
    pub fn abi_out(&self) -> String {
        format!("{}*", self.abi_in())
    }

    /// Whether the surface type crosses the ABI by value with no conversion.
    pub fn is_blittable(&self) -> bool {
        matches!(
            self,
            Self::Scalar(_)
                | Self::Enum { .. }
                | Self::Struct { abi_name: None, .. }
                | Self::Handle { .. }
                | Self::Callback { .. }
        )
    }

    pub fn is_native_abi(&self) -> bool {
        self.is_blittable()
            || matches!(self, Self::Win32Bool | Self::HResult | Self::Pointer { .. })
    }

    /// Whether a native COM method returns this value through the ABI result pointer immediately
    /// after `this`. This is the Microsoft C++ member-function convention used by COM vtables, not
    /// the CLR's ordinary unmanaged aggregate-return convention. Enums, opaque handles, callbacks,
    /// scalars, and pointers remain direct returns.
    pub fn is_native_com_record_return(&self) -> bool {
        matches!(self, Self::Struct { abi_name: None, .. })
    }

    pub(crate) fn is_struct_field_abi(&self) -> bool {
        self.is_native_abi()
            || matches!(
                self,
                Self::Boolean
                    | Self::String
                    | Self::Struct {
                        abi_name: Some(_),
                        ..
                    }
            )
    }

    /// Whether the type satisfies a C# `unmanaged` constraint, so it can be a generic element type
    /// (`IVector<T> where T : unmanaged`). Scalars, enums, and blittable structs qualify. Managed
    /// strings and projected reference classes require per-element ABI adapters not yet emitted.
    pub fn is_unmanaged(&self) -> bool {
        !matches!(
            self,
            Self::String
                | Self::Boolean
                | Self::Win32Bool
                | Self::HResult
                | Self::Struct { owns_abi: true, .. }
                | Self::Object { .. }
                | Self::Inspectable
                | Self::Reference { .. }
                | Self::Async { .. }
                | Self::Array { .. }
                | Self::ComOut { .. }
        )
    }

    /// Wraps an ABI-typed expression to produce the surface value. An enum casts from its
    /// underlying scalar; a returned or `[out]` object takes ownership of the `+1` interface pointer
    /// by wrapping it in the projected struct; scalars, strings, and structs are identity.
    pub fn abi_to_surface(&self, expr: &str) -> String {
        match self {
            Self::String => format!("WindowsCsharp.Interop.FromHstring({expr})"),
            Self::Boolean => format!("{expr} != 0"),
            Self::Win32Bool => format!("{expr} != 0"),
            Self::Enum { name, .. } => format!("({name}){expr}"),
            Self::Struct {
                abi_name: Some(_), ..
            } => format!("{expr}.ToSurface()"),
            Self::Object { name } => format!("WindowsCsharp.Com.Wrap<{name}>({expr})"),
            Self::Inspectable => {
                format!("WindowsCsharp.Com.Wrap<Windows.Foundation.IInspectable>({expr})")
            }
            Self::Reference { value, .. } => {
                format!(
                    "WindowsCsharp.ReferenceBox<{}>.Unbox({expr})",
                    value.surface()
                )
            }
            Self::Async { value, .. } => format!(
                "WindowsCsharp.Com.Wrap<Windows.Foundation.IAsyncOperation<{}>>({expr})",
                value.surface()
            ),
            Self::Array { .. } => unreachable!("array conversion is emitted by the writer"),
            Self::ComOut { name } => format!("WindowsCsharp.Com.Wrap<{name}>({expr})"),
            _ => expr.to_string(),
        }
    }

    /// Converts a delegate input from its borrowed ABI representation to the callback surface.
    pub fn abi_to_callback_surface(&self, expr: &str) -> String {
        match self {
            Self::String => format!("WindowsCsharp.Interop.FromHstringBorrowed({expr})"),
            Self::Object { name } => format!("new {name}.Borrowed({expr})"),
            Self::Inspectable => {
                format!("new Windows.Foundation.IInspectable.Borrowed({expr})")
            }
            Self::Struct { owns_abi: true, .. } => format!("{expr}.FromAbi()"),
            _ => self.abi_to_surface(expr),
        }
    }

    /// Wraps a surface expression to produce the ABI value. An enum casts to its underlying scalar;
    /// scalars and structs are identity. Object inputs require a call-scoped lease and are handled
    /// by the writer rather than by an expression-only conversion.
    pub fn surface_to_abi(&self, expr: &str) -> String {
        match self {
            Self::Boolean => format!("({expr} ? (byte)1 : (byte)0)"),
            Self::Win32Bool => format!("({expr} ? 1 : 0)"),
            Self::Enum { underlying, .. } => format!("({underlying}){expr}"),
            Self::Struct {
                abi_name: Some(abi_name),
                owns_abi: false,
                ..
            } => format!("{abi_name}.FromSurface({expr})"),
            Self::Struct { owns_abi: true, .. } => {
                unreachable!("owned struct inputs require a call-scoped ABI local")
            }
            Self::Object { .. }
            | Self::Inspectable
            | Self::Reference { .. }
            | Self::Async { .. }
            | Self::Array { .. } => {
                unreachable!("projected object inputs require a call-scoped lease")
            }
            Self::ComOut { .. } => {
                unreachable!("COM out parameters are emitted by the function writer")
            }
            _ => expr.to_string(),
        }
    }

    pub fn is_object(&self) -> bool {
        matches!(
            self,
            Self::Object { .. } | Self::Inspectable | Self::Async { .. }
        )
    }

    pub fn owned_struct_abi(&self) -> Option<&str> {
        match self {
            Self::Struct {
                abi_name: Some(name),
                owns_abi: true,
                ..
            } => Some(name),
            _ => None,
        }
    }

    pub fn surface_to_owned_abi(&self, expr: &str) -> String {
        let abi = self
            .owned_struct_abi()
            .expect("owned ABI conversion requires an owning struct");
        format!("{abi}.FromSurface({expr})")
    }

    pub fn array(&self) -> Option<(&Self, bool)> {
        match self {
            Self::Array { element, output } => Some((element, *output)),
            _ => None,
        }
    }

    fn is_array_element(&self) -> bool {
        matches!(
            self,
            Self::Scalar(_)
                | Self::Boolean
                | Self::Enum { .. }
                | Self::Struct { abi_name: None, .. }
                | Self::Handle { .. }
                | Self::String
        ) || self.is_object()
    }

    pub fn parameter(&self, name: &str) -> String {
        match self {
            Self::Array { output: true, .. } => format!("out {} {name}", self.surface()),
            _ => format!("{} {name}", self.surface()),
        }
    }

    pub fn reference(&self) -> Option<(&Self, Guid)> {
        match self {
            Self::Reference { value, iid } => Some((value, *iid)),
            _ => None,
        }
    }
}

fn struct_fields_are_native(index: &Index, def: TypeDef) -> bool {
    if def.has_attribute("AlignmentAttribute") {
        return false;
    }
    def.fields()
        .filter(|field| {
            !field
                .flags()
                .contains(windows_metadata::FieldAttributes::Static)
        })
        .all(|field| {
            let ty = field.ty();
            if let Type::ValueName(name) = &ty
                && name.namespace.is_empty()
            {
                index
                    .nested(def)
                    .find(|nested| nested.name() == name.name)
                    .is_some_and(|nested| struct_fields_are_native(index, nested))
            } else {
                CsType::map(index, &ty).is_some_and(|ty| ty.is_struct_field_abi())
            }
        })
}

pub(crate) fn struct_needs_abi(index: &Index, def: TypeDef) -> bool {
    def.fields()
        .filter(|field| {
            !field
                .flags()
                .contains(windows_metadata::FieldAttributes::Static)
        })
        .any(|field| {
            let ty = field.ty();
            if let Type::ValueName(name) = &ty
                && name.namespace.is_empty()
            {
                index
                    .nested(def)
                    .find(|nested| nested.name() == name.name)
                    .is_some_and(|nested| struct_needs_abi(index, nested))
            } else {
                matches!(
                    CsType::map(index, &ty),
                    Some(
                        CsType::Boolean
                            | CsType::Struct {
                                abi_name: Some(_),
                                ..
                            }
                    )
                )
            }
        })
}

pub(crate) fn struct_owns_abi(index: &Index, def: TypeDef) -> bool {
    def.fields()
        .filter(|field| {
            !field
                .flags()
                .contains(windows_metadata::FieldAttributes::Static)
        })
        .any(|field| {
            let ty = field.ty();
            if let Type::ValueName(name) = &ty
                && name.namespace.is_empty()
            {
                index
                    .nested(def)
                    .find(|nested| nested.name() == name.name)
                    .is_some_and(|nested| struct_owns_abi(index, nested))
            } else {
                matches!(
                    CsType::map(index, &ty),
                    Some(CsType::String | CsType::Struct { owns_abi: true, .. })
                )
            }
        })
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    Stdcall,
    Cdecl,
    Thiscall,
    Fastcall,
}

impl CallingConvention {
    fn cs_name(self) -> &'static str {
        match self {
            Self::Stdcall => "Stdcall",
            Self::Cdecl => "Cdecl",
            Self::Thiscall => "Thiscall",
            Self::Fastcall => "Fastcall",
        }
    }
}

pub(crate) fn native_callback(index: &Index, def: TypeDef) -> Option<CsType> {
    if def.flags().contains(TypeAttributes::WindowsRuntime)
        || def.category() != TypeCategory::Delegate
    {
        return None;
    }

    let invoke = def.methods().find(|method| method.name() == "Invoke")?;
    let signature = invoke.signature(&[]);
    let params = signature
        .types
        .iter()
        .map(|ty| CsType::map(index, ty))
        .collect::<Option<Vec<_>>>()?;
    let return_type = Box::new(CsType::map(index, &signature.return_type)?);
    let convention = match def
        .find_attribute("UnmanagedFunctionPointerAttribute")
        .and_then(|attribute| attribute.value().into_iter().next())
        .and_then(|(_, value)| match value {
            Value::EnumValue(_, value) => match *value {
                Value::I32(value) => Some(value),
                _ => None,
            },
            _ => None,
        }) {
        None | Some(1 | 3) => CallingConvention::Stdcall,
        Some(2) => CallingConvention::Cdecl,
        Some(4) => CallingConvention::Thiscall,
        Some(5) => CallingConvention::Fastcall,
        _ => return None,
    };

    Some(CsType::Callback {
        params,
        return_type,
        convention,
    })
}

/// Maps a blittable scalar metadata type to its C# name, or `None` for any non-scalar. Shared by
/// [`CsType::map`] and enum underlying-type resolution.
pub fn scalar_name(ty: &Type) -> Option<&'static str> {
    Some(match ty {
        Type::I8 => "sbyte",
        Type::U8 => "byte",
        Type::I16 => "short",
        Type::U16 => "ushort",
        Type::I32 => "int",
        Type::U32 => "uint",
        Type::I64 => "long",
        Type::U64 => "ulong",
        Type::ISize => "nint",
        Type::USize => "nuint",
        Type::F32 => "float",
        Type::F64 => "double",
        Type::Char => "ushort",
        _ => return None,
    })
}

/// The set of closed generic instantiations the projection supports, mapping a metadata type name
/// to the C# generic surface name (for example `Windows.Foundation.Collections.IVector<int>` or
/// `Windows.Foundation.Collections.IMap<int, int>`). Returns `None` for an unsupported generic or a
/// unsupported type argument. Vectors support unmanaged values and projected object elements;
/// maps retain their narrower value support. The projected shapes are the mutable collections
/// `IVector<T>`/`IMap<K,V>` and their read-only views
/// `IVectorView<T>`/`IMapView<K,V>` (arity one for the vector shapes, arity two for the map shapes).
pub fn generic_name(index: &Index, tn: &windows_metadata::TypeName) -> Option<String> {
    if tn.namespace != "Windows.Foundation.Collections" {
        return None;
    }
    let arity1 = match tn.name.as_str() {
        "IVector`1" => Some("IVector"),
        "IVectorView`1" => Some("IVectorView"),
        _ => None,
    };
    if let Some(surface) = arity1 {
        if tn.generics.len() != 1 {
            return None;
        }
        let arg = CsType::map(index, &tn.generics[0])?;
        if !arg.is_unmanaged() && !arg.is_object() && !matches!(arg, CsType::String) {
            return None;
        }
        return Some(format!(
            "Windows.Foundation.Collections.{surface}<{}>",
            arg.collection_generic_surface()
        ));
    }
    let arity2 = match tn.name.as_str() {
        "IMap`2" => Some("IMap"),
        "IMapView`2" => Some("IMapView"),
        _ => None,
    };
    if let Some(surface) = arity2 {
        if tn.generics.len() != 2 {
            return None;
        }
        let key = CsType::map(index, &tn.generics[0])?;
        let value = CsType::map(index, &tn.generics[1])?;
        if (!key.is_unmanaged() && !key.is_object() && !matches!(key, CsType::String))
            || (!value.is_unmanaged() && !value.is_object() && !matches!(value, CsType::String))
        {
            return None;
        }
        return Some(format!(
            "Windows.Foundation.Collections.{surface}<{}, {}>",
            key.collection_generic_surface(),
            value.collection_generic_surface()
        ));
    }
    None
}

fn is_native_i32_typedef(index: &Index, name: &windows_metadata::TypeName) -> bool {
    let Some(def) = index.get(&name.namespace, &name.name).next() else {
        return false;
    };
    matches!(native_typedef_underlying(def), Some(Type::I32))
}

pub(crate) fn native_typedef_underlying(def: TypeDef) -> Option<Type> {
    if !def.has_attribute("NativeTypedefAttribute") {
        return None;
    }
    let mut fields = def.fields().filter(|field| {
        !field
            .flags()
            .contains(windows_metadata::FieldAttributes::Static)
    });
    let ty = fields.next()?.ty();
    fields.next().is_none().then_some(ty)
}

/// The structural rule that recognizes a genuine Win32 opaque handle: a `NativeTypedefAttribute`
/// type whose single field resolves to an opaque `void*` (exactly one pointer indirection to
/// `void`), returning that field type for `def` such a handle, or `None` otherwise. This is
/// narrower on purpose than `windows-bindgen`'s `CppStruct::is_handle` (which treats any
/// single-`Value`-field typedef backed by a primitive - including a plain scalar or a pointer to a
/// named type - as a "handle"; see `docs/crates/windows-csharp.md`'s cross-crate review): a scalar
/// identifier alias (`COLORREF: Value: u32`, `ATOM: Value: u16`, `HFILE`/`NTSTATUS: Value: i32`)
/// and a pointer-to-named-type alias (`PWSTR: Value: *mut u16`, `LPRECT: Value: *mut RECT`) both
/// stay collapsed to their existing scalar/pointer representation instead of becoming a wrapper
/// type, while an opaque-pointer typedef (`HWND`, `HANDLE`, `HDC`, `HKEY`, `SC_HANDLE`, ...) - the
/// shape every real Win32 handle in `Windows.Win32.winmd` actually uses - becomes a distinct
/// blittable `readonly struct`. A typedef chained to another handle (`HGLOBAL: Value: HANDLE`) is
/// not itself matched here (its field type is a named type, not a raw pointer); `CsType::map`'s
/// existing recursive collapse instead resolves it straight through to the chained handle's own
/// wrapper, so `HGLOBAL` and `HANDLE` project as the same C# type rather than minting a second one.
pub fn native_handle_value(def: TypeDef) -> Option<Type> {
    let underlying = native_typedef_underlying(def)?;
    matches!(
        &underlying,
        Type::PtrMut(inner, 1) | Type::PtrConst(inner, 1) if matches!(inner.as_ref(), Type::Void)
    )
    .then_some(underlying)
}

/// Resolves an enum's underlying scalar from its `value__` storage field (always the first field),
/// mapped to a C# scalar name. The reader's `underlying_type()` only handles single-field typedefs,
/// so it returns `None` for an enum, which also carries a literal field per variant.
pub fn enum_underlying(def: TypeDef) -> Option<&'static str> {
    let field = def.fields().next()?;
    let ty = if let Some(constant) = field.constant() {
        constant.ty()
    } else {
        field.ty()
    };
    scalar_name(&ty)
}

/// Returns the mapped `Invoke` parameter and return types of a WinRT delegate, or `None` if any is
/// unsupported. Strings are copied without consuming the caller's `HSTRING`; input objects are
/// callback-confined borrowed views. String and projected-object returns transfer a newly owned ABI
/// value to the native caller.
pub fn delegate_invoke(index: &Index, def: TypeDef) -> Option<(Vec<CsType>, Option<CsType>)> {
    let invoke = def.methods().find(|method| method.name() == "Invoke")?;
    let signature = invoke.signature(&[]);

    let ret = match &signature.return_type {
        Type::Void => None,
        ty => {
            let ret = CsType::map(index, ty)?;
            if !ret.is_blittable() && !matches!(ret, CsType::String) && !ret.is_object() {
                return None;
            }
            Some(ret)
        }
    };

    let mut params = Vec::with_capacity(signature.types.len());
    for ty in &signature.types {
        let param = CsType::map(index, ty)?;
        if !param.is_blittable()
            && !matches!(
                param,
                CsType::String | CsType::Object { .. } | CsType::Inspectable
            )
        {
            return None;
        }
        params.push(param);
    }

    Some((params, ret))
}

/// A parameter's direction after applying the C# projection's input fallback to an unspecified
/// metadata direction.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Input,
    Output,
    InputOutput,
}

impl From<ParamDirection> for Direction {
    fn from(direction: ParamDirection) -> Self {
        match direction {
            ParamDirection::Unspecified | ParamDirection::Input => Self::Input,
            ParamDirection::Output => Self::Output,
            ParamDirection::InputOutput => Self::InputOutput,
        }
    }
}

/// A pointer parameter whose element or byte count is carried by another parameter.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BufferLength {
    Elements(usize),
    Bytes(usize),
}

impl BufferLength {
    pub fn param(self) -> usize {
        match self {
            Self::Elements(param) | Self::Bytes(param) => param,
        }
    }
}

/// Mutually exclusive public-surface shaping retained from a parameter's original metadata.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParamProjection {
    Value,
    Utf16String,
    Buffer(BufferLength),
    Utf16Buffer(BufferLength),
}

/// One parameter (or constructor/delegate argument) shared by every reader that reads a metadata
/// parameter list: its C# name, mapped type, metadata direction, optionality, and projection hints
/// that depend on the original metadata spelling rather than only its collapsed ABI type.
pub struct Param {
    pub name: String,
    pub ty: CsType,
    pub direction: Direction,
    pub optional: bool,
    pub projection: ParamProjection,
}

impl Param {
    pub fn is_utf16_string(&self) -> bool {
        matches!(self.projection, ParamProjection::Utf16String)
    }

    pub fn buffer_length(&self) -> Option<BufferLength> {
        match self.projection {
            ParamProjection::Buffer(length) | ParamProjection::Utf16Buffer(length) => Some(length),
            _ => None,
        }
    }

    /// The pointee type when this parameter is safe to project as a C# `out`/`ref` parameter
    /// instead of its raw ABI pointer: exactly one pointer indirection to a native scalar, enum, or
    /// blittable struct, with a metadata direction that resolves unambiguously to `Output` or
    /// `InputOutput`. Multi-level indirection, `void*`, optional pointers, arrays/strings/objects,
    /// and a mutable pointer whose metadata never sets the `Out` flag are all left as the raw
    /// pointer ABI shape rather than guessed at.
    pub fn scalar_pointer_target(&self) -> Option<&CsType> {
        if self.optional
            || self.buffer_length().is_some()
            || !matches!(self.direction, Direction::Output | Direction::InputOutput)
        {
            return None;
        }
        match &self.ty {
            CsType::Pointer {
                element,
                mutable: true,
                depth: 1,
            } if element.is_blittable() => Some(element),
            _ => None,
        }
    }

    /// The element type and count-parameter position for a validated counted buffer.
    pub fn buffer_target(&self) -> Option<(BufferElement<'_>, usize)> {
        let length = self.buffer_length()?;
        let CsType::Pointer {
            element,
            mutable,
            depth: 1,
        } = &self.ty
        else {
            return None;
        };
        if !matches!(self.direction, Direction::Input) && !mutable {
            return None;
        }
        let element = if matches!(self.projection, ParamProjection::Utf16Buffer(_)) {
            if !matches!(element.as_ref(), CsType::Scalar("ushort")) {
                return None;
            }
            BufferElement::Utf16
        } else if matches!(length, BufferLength::Bytes(_))
            && matches!(element.as_ref(), CsType::Void)
        {
            BufferElement::ByteVoid
        } else {
            if !element.is_blittable()
                || matches!(length, BufferLength::Bytes(_))
                    && !matches!(element.as_ref(), CsType::Scalar("byte" | "sbyte"))
            {
                return None;
            }
            BufferElement::Value(element)
        };
        Some((element, length.param()))
    }

    pub fn is_buffer_count(&self) -> bool {
        matches!(
            &self.ty,
            CsType::Scalar(
                "byte"
                    | "sbyte"
                    | "short"
                    | "ushort"
                    | "int"
                    | "uint"
                    | "long"
                    | "ulong"
                    | "nint"
                    | "nuint"
            )
        ) && matches!(self.direction, Direction::Input)
            && !self.optional
    }
}

#[derive(Clone, Copy)]
pub enum BufferElement<'a> {
    Value(&'a CsType),
    Utf16,
    ByteVoid,
}

impl BufferElement<'_> {
    pub fn surface(self) -> String {
        match self {
            Self::Value(value) => value.surface(),
            Self::Utf16 => "char".to_string(),
            Self::ByteVoid => "byte".to_string(),
        }
    }

    pub fn abi(self) -> String {
        match self {
            Self::Value(value) => value.abi_in(),
            Self::Utf16 => "ushort".to_string(),
            Self::ByteVoid => "void".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum ParamRole<'a> {
    Value,
    ScalarPointer {
        target: &'a CsType,
        is_ref: bool,
    },
    Utf16String,
    Buffer {
        element: BufferElement<'a>,
        count: usize,
    },
    BufferCount {
        buffer: usize,
    },
}

/// Resolves both sides of each parameter relationship once for any call emitter. The reader has
/// already guaranteed that a counted buffer has one valid scalar count and that no count is shared.
pub fn param_roles(params: &[Param]) -> Vec<ParamRole<'_>> {
    let mut roles = params
        .iter()
        .map(|param| {
            if param.is_utf16_string() {
                ParamRole::Utf16String
            } else if let Some((element, count)) = param.buffer_target() {
                ParamRole::Buffer { element, count }
            } else if let Some(target) = param.scalar_pointer_target() {
                ParamRole::ScalarPointer {
                    target,
                    is_ref: matches!(param.direction, Direction::InputOutput),
                }
            } else {
                ParamRole::Value
            }
        })
        .collect::<Vec<_>>();

    for buffer in 0..roles.len() {
        let role = roles[buffer];
        if let ParamRole::Buffer { count, .. } = role {
            roles[count] = ParamRole::BufferCount { buffer };
        }
    }
    roles
}

/// A projected member of a runtime class or interface: a property, a method, or an event.
pub enum Member {
    Property {
        name: String,
        ty: CsType,
        get_slot: Option<usize>,
        put_slot: Option<usize>,
    },
    Method {
        name: String,
        params: Vec<Param>,
        ret: Option<CsType>,
        slot: usize,
        abi: MethodAbi,
    },
    /// A WinRT event: paired `add_`/`remove_` accessors projected as `Add{name}(handler) -> long`
    /// and `Remove{name}(long token)`. `delegate` is the handler's projected type (an `Object`
    /// carrying one interface pointer). The `EventRegistrationToken` the ABI returns is a blittable
    /// `i64`, so the token surfaces as a plain `long`.
    Event {
        name: String,
        delegate: CsType,
        add_slot: usize,
        remove_slot: usize,
    },
}

#[derive(Clone, Copy)]
pub enum MethodAbi {
    /// WinRT methods return HRESULT and place their logical return in a trailing out parameter.
    WinRt,
    /// Win32 COM methods use the metadata signature directly, including its native return type.
    Direct,
}

/// A projected WinRT runtime class: one interface pointer wrapping the default interface, its
/// default-interface members, non-default interface forwarders, and activation.
///
/// Activation is modeled from the class's metadata attributes rather than a single boolean:
/// - `default_activation` records an `ActivatableAttribute` with no factory type (a parameterless
///   `RoActivateInstance`-style constructor).
/// - `factories` records `ActivatableAttribute`/`ComposableAttribute` entries that name a factory
///   interface; each factory method becomes a public constructor.
/// - `statics` records `StaticAttribute` entries; each static interface's members become static
///   members on the class.
pub struct Class {
    pub namespace: String,
    pub name: String,
    pub default_activation: bool,
    pub factories: Vec<Factory>,
    pub statics: Vec<StaticInterface>,
    pub default_iid: Guid,
    pub members: Vec<Member>,
    pub forwarders: Vec<Forwarder>,
    /// Runtime classes and interfaces this class can safely supply to an ABI parameter. The writer
    /// emits zero-state generic marker interfaces for the subset present in the projection.
    pub compatible: Vec<String>,
}

impl Class {
    /// The activatable class id (`Namespace.Name`) passed to the activation factory.
    pub fn class_id(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

/// A WinRT activation or composition factory interface named by an `ActivatableAttribute` or
/// `ComposableAttribute`. Each of its creation methods projects to a public constructor on the
/// runtime class that calls the factory slot and takes ownership of the returned instance pointer.
pub struct Factory {
    pub iid: Guid,
    /// A composable factory (`ComposableAttribute`): its creation methods carry two trailing ABI
    /// parameters (`outer`, `inner`) that are omitted from the public constructor and passed as
    /// null for non-aggregating construction.
    pub composable: bool,
    pub constructors: Vec<Constructor>,
}

/// One factory creation method projected as a public constructor: the user-visible parameters
/// (composable `outer`/`inner` already dropped) and the factory-interface vtable slot to call.
pub struct Constructor {
    pub params: Vec<Param>,
    pub slot: usize,
}

/// A WinRT static interface named by a `StaticAttribute`. Its members project as `static` members
/// on the runtime class, reached through the class's activation factory rather than an instance
/// pointer.
pub struct StaticInterface {
    pub iid: Guid,
    pub members: Vec<Member>,
}

/// Methods from one non-default interface implemented by a runtime class. The class forwards these
/// directly through a temporary QI so common `object.Method()` calls avoid a projected owner.
pub struct Forwarder {
    pub iid: Guid,
    pub members: Vec<Member>,
}

/// A projected WinRT interface, emitted as its own `readonly unsafe struct`. Every in-scope
/// interface that is not the default interface of a projected class becomes one of these, reachable
/// from any projected type through the generic `As<T>()` cast (a `QueryInterface` for
/// [`iid`](Self::iid)).
pub struct Interface {
    pub namespace: String,
    pub name: String,
    pub iid: Guid,
    pub members: Vec<Member>,
    pub compatible: Vec<String>,
    pub native_base: Option<String>,
    pub native_own_members: Option<Vec<Member>>,
}

/// A projected WinRT enum, emitted as a C# `enum` over its blittable underlying scalar. Enums are
/// blittable, so a member typed as an enum reuses the scalar ABI path with a cast at the boundary
/// and no copying.
pub struct Enum {
    pub namespace: String,
    pub name: String,
    pub underlying: &'static str,
    pub fields: Vec<(String, String)>,
}

/// A projected WinRT or Win32 record. Sequential structs retain normal field order; native unions
/// use explicit layout with every field at offset zero. Each field has a supported ABI
/// representation.
pub struct Struct {
    pub namespace: String,
    pub name: String,
    pub abi_name: Option<String>,
    pub owns_abi: bool,
    pub explicit: bool,
    pub packing_size: Option<u16>,
    pub class_size: Option<u32>,
    pub fields: Vec<(String, CsType)>,
    pub nested: Vec<Self>,
}

/// A genuine Win32 opaque native handle (see [`native_handle_value`]), emitted as an explicit
/// blittable `readonly struct` wrapping a single `nint` field: strongly typed and nominally
/// distinct from every other handle (an `HWND` cannot be passed where a `HANDLE` is expected
/// without an explicit cast), equatable, default/null capable (`default(HWND)` is the zero
/// handle), and exposes its raw value with no unsafe pointer field and no ownership/close
/// semantics.
pub struct Handle {
    pub namespace: String,
    pub name: String,
}

/// A Win32 exported function projected as a static method on the namespace-local `Apis` class.
/// The first slice supports direct blittable parameters and returns; pointer, string, callback,
/// and HRESULT-sugar shapes are added on top of this model rather than through a separate writer.
pub struct Function {
    pub namespace: String,
    pub name: String,
    pub library: String,
    pub import_name: String,
    pub cdecl: bool,
    pub params: Vec<Param>,
    pub ret: Option<CsType>,
    pub hresult: bool,
}

/// A Win32 metadata constant projected on the namespace-local `Apis` class.
pub struct ApiConstant {
    pub namespace: String,
    pub name: String,
    pub ty: CsType,
    pub value: String,
}

/// A projected WinRT delegate, emitted as a `readonly unsafe struct` that both invokes a delegate
/// pointer received from native (a slot-3 call) and, through `Create`, allocates a native COM
/// object backed by a managed callback (the reverse vtable used to subscribe to events). `Invoke`
/// sits at vtable slot 3, after the `IUnknown` slots a delegate derives from (`QueryInterface` 0,
/// `AddRef` 1, `Release` 2).
pub struct Delegate {
    pub namespace: String,
    pub name: String,
    pub iid: Guid,
    pub params: Vec<Param>,
    pub ret: Option<CsType>,
}

/// The projected generic collection `Windows.Foundation.Collections.IVector<T>`, emitted once when
/// any in-scope member's signature names an `IVector<...>`. The struct body is a single real C#
/// generic. Each entry carries the concrete element type and its generation-time parameterized
/// IID. Value elements cross by value; projected object elements cross as interface pointers.
pub struct Vector {
    pub instantiations: Vec<VectorInstantiation>,
}

pub struct VectorInstantiation {
    pub element: CsType,
    pub iid: Guid,
}

/// The projected `Windows.Foundation.IAsyncOperation<T>` and its matching
/// `AsyncOperationCompletedHandler<T>` delegate. Both closed generic IIDs are derived from the
/// same result type at generation time.
pub struct AsyncOperation {
    pub instantiations: Vec<AsyncOperationInstantiation>,
}

pub struct AsyncOperationInstantiation {
    pub element: CsType,
    pub iid: Guid,
    pub completed_iid: Guid,
}

/// The projected generic collection `Windows.Foundation.Collections.IMap<K, V>`, emitted once when
/// any in-scope member's signature names an `IMap<...>`. The struct body is a single real C# generic
/// type. Each entry carries the concrete key/value types plus the map, iterable, and iterator IIDs.
/// Small projections select them with closed-generic `typeof` checks. Broad projections also use
/// the entry to initialize per-pair managed function pointers once, keeping hot map calls compact.
pub struct Map {
    pub instantiations: Vec<MapInstantiation>,
}

pub struct MapInstantiation {
    pub key: CsType,
    pub value: CsType,
    pub iid: Guid,
    pub iterable_iid: Guid,
    pub iterator_iid: Guid,
}

/// The projected generic collections emitted into the shared `Windows.Foundation.Collections`
/// namespace block. Each is `Some` only when an in-scope member's signature names that
/// instantiation, so an input using none of them emits no collections block. The read-only views
/// (`vector_view`/`map_view`) reuse the [`Vector`]/[`Map`] models - same per-instantiation IID
/// table, different emitted surface.
pub struct Collections {
    pub inspectable: bool,
    pub async_operation: Option<AsyncOperation>,
    pub vector: Option<Vector>,
    pub map: Option<Map>,
    pub vector_view: Option<Vector>,
    pub map_view: Option<Map>,
}
