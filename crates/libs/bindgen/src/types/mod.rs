use super::*;

mod class;
mod cpp_const;
mod cpp_delegate;
mod cpp_enum;
mod cpp_fn;
mod cpp_handle;
mod cpp_interface;
mod cpp_method;
mod cpp_struct;
mod delegate;
mod r#enum;
mod interface;
mod method;
mod r#struct;

pub use class::*;
pub use cpp_const::*;
pub use cpp_delegate::*;
pub use cpp_enum::*;
pub use cpp_fn::*;
pub use cpp_interface::*;
pub use cpp_method::*;
pub use cpp_struct::*;
pub use delegate::*;
pub use r#enum::*;
pub use interface::*;
pub use method::*;
pub use r#struct::*;

#[expect(clippy::upper_case_acronyms, clippy::enum_variant_names)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    CppFn(CppFn),
    Class(Class),
    Interface(Interface),
    CppInterface(CppInterface),
    Delegate(Delegate),
    CppDelegate(CppDelegate),
    Enum(Enum),
    CppEnum(CppEnum),
    Struct(Struct),
    CppStruct(CppStruct),
    CppConst(CppConst),

    Generic(GenericParam),
    PtrMut(Box<Self>, usize),
    PtrConst(Box<Self>, usize),
    ArrayFixed(Box<Self>, usize),
    Array(Box<Self>),
    ArrayRef(Box<Self>),
    ConstRef(Box<Self>),
    PrimitiveOrEnum(Box<Self>, Box<Self>),

    Void,
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    ISize,
    USize,
    String,
    Object,
    Type,

    PSTR,
    PCSTR,
    PWSTR,
    PCWSTR,
    GUID,
    HRESULT,
    IUnknown,
    BSTR,
    BOOL,
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key().cmp(&(other.sort_key()))
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq)]
pub enum Remap {
    Type(Type),
    Name(TypeName),
    None,
}

impl Type {
    fn sort_key(&self) -> (bool, TypeName, i32, i32) {
        // This sorts types as follows:
        // 1. functions are placed first
        // 2. type name
        // 3. type namespace
        // 4. architecture
        // 5. overloaded types

        let kind = match self {
            Self::CppFn(..) => 0,
            Self::Class(..) => 1,
            Self::Interface(..) => 2,
            Self::CppInterface(..) => 3,
            Self::Delegate(..) => 4,
            Self::CppDelegate(..) => 5,
            Self::Enum(..) => 6,
            Self::CppEnum(..) => 7,
            Self::Struct(..) => 8,
            Self::CppStruct(..) => 9,
            Self::CppConst(..) => 10,
            _ => -1,
        };

        let arches = match self {
            Self::CppFn(ty) => ty.method.arches(),
            Self::CppStruct(ty) => ty.def.arches(),
            Self::CppDelegate(ty) => ty.def.arches(),
            Self::CppConst(ty) => ty.field.arches(),
            _ => 0,
        };

        (kind != 0, self.type_name(), arches, kind)
    }

    /// A stable secondary key that breaks ties between two distinct types sharing
    /// the same `sort_key`. Such collisions only arise in `--flat` layouts, where
    /// collapsing namespaces lands two same-name types in one bucket (e.g.
    /// `D3DFMT_P8` defined as both the `D3DFORMAT` enum and a raw `u32`). Ordering
    /// by the underlying metadata row — whose `(file, pos)` is stable across runs
    /// for identical inputs — makes the resulting collapse deterministic.
    fn row_key(&self) -> (usize, usize) {
        let row = match self {
            Self::CppFn(ty) => Some(ty.method.to_row()),
            Self::CppStruct(ty) => Some(ty.def.to_row()),
            Self::CppEnum(ty) => Some(ty.def.to_row()),
            Self::CppInterface(ty) => Some(ty.def.to_row()),
            Self::CppDelegate(ty) => Some(ty.def.to_row()),
            Self::CppConst(ty) => Some(ty.field.to_row()),
            Self::Class(ty) => Some(ty.def.to_row()),
            Self::Interface(ty) => Some(ty.def.to_row()),
            Self::Enum(ty) => Some(ty.def.to_row()),
            Self::Struct(ty) => Some(ty.def.to_row()),
            Self::Delegate(ty) => Some(ty.def.to_row()),
            _ => None,
        };
        row.map(|r| (r.file, r.pos)).unwrap_or_default()
    }

    /// Total ordering used when inserting into the codegen `BTreeSet`. Extends
    /// `sort_key` with `row_key` so that when two distinct types collapse to one
    /// (equal `sort_key`), the surviving element is chosen deterministically.
    pub(crate) fn dedup_cmp(&self, other: &Self) -> Ordering {
        self.sort_key()
            .cmp(&other.sort_key())
            .then_with(|| self.row_key().cmp(&other.row_key()))
    }

    pub(crate) fn is_intrinsic(&self) -> bool {
        matches!(
            self,
            Self::Generic(..)
                | Self::Void
                | Self::Bool
                | Self::Char
                | Self::I8
                | Self::U8
                | Self::I16
                | Self::U16
                | Self::I32
                | Self::U32
                | Self::I64
                | Self::U64
                | Self::F32
                | Self::F64
                | Self::ISize
                | Self::USize
        )
    }

    pub fn remap(namespace: &str, name: &str, sys: bool) -> Remap {
        // WinRT / .NET system projections keep full-name matching: their names
        // (`Guid`, `HResult`, `Type`) differ from the C spellings and would be
        // ambiguous if matched by name alone.
        match (namespace, name) {
            ("System", "Guid") => return Remap::Type(Self::GUID),
            ("System", "Type") => return Remap::Type(Self::Type),
            ("Windows.Foundation", "HResult") => return Remap::Type(Self::HRESULT),
            ("Windows.Foundation", "EventRegistrationToken") => return Remap::Type(Self::I64),
            _ => {}
        }

        // The Win32 core types are matched by name alone so that the win32metadata
        // namespaces (e.g. `Windows.Win32.Foundation.HRESULT`), the in-house faithful
        // metadata's flat namespace (`Windows.Win32.HRESULT`), and the published
        // package's per-header namespaces (`Windows.guiddef.GUID`, produced by the
        // package remapper) all resolve to the same hard-coded core type. These C
        // spellings are unambiguous, so the namespace they live in does not matter.
        // WinRT namespaces always have PascalCase leaves, so a lowercase leaf directly
        // under `Windows.` uniquely identifies a flat Win32/WDK header stem.
        let win32_meta = namespace == "Windows.Win32" || namespace.starts_with("Windows.Win32.");
        let is_flat_win32_stem = namespace
            .strip_prefix("Windows.")
            .is_some_and(|leaf| leaf.starts_with(|c: char| c.is_ascii_lowercase()));

        if win32_meta || is_flat_win32_stem {
            match name {
                "GUID" => return Remap::Type(Self::GUID),
                "HRESULT" => return Remap::Type(Self::HRESULT),
                "PSTR" => return Remap::Type(Self::PSTR),
                "PWSTR" => return Remap::Type(Self::PWSTR),
                "PCSTR" => return Remap::Type(Self::PCSTR),
                "PCWSTR" => return Remap::Type(Self::PCWSTR),
                "BSTR" => return Remap::Type(Self::BSTR),
                "HSTRING" => return Remap::Type(Self::String),
                "IInspectable" => return Remap::Type(Self::Object),
                "IUnknown" => return Remap::Type(Self::IUnknown),
                "CHAR" => return Remap::Type(Self::I8),
                "BOOLEAN" => return Remap::Type(Self::Bool),
                "BOOL" => return Remap::Type(Self::BOOL),
                "EventRegistrationToken" => return Remap::Type(Self::I64),

                // `LARGE_INTEGER` / `ULARGE_INTEGER` are faithfully scraped as unions
                // (their `QuadPart` / `LowPart`+`HighPart` overlay), but the reference
                // winmd and every published `windows` / `windows-sys` binding collapse
                // them to their 64-bit scalar. Remap at gen time — like `CHAR` / `BOOLEAN`
                // above — so the in-house union metadata stays faithful while consumers see
                // the ergonomic `i64` / `u64`.
                "LARGE_INTEGER" => return Remap::Type(Self::I64),
                "ULARGE_INTEGER" => return Remap::Type(Self::U64),
                _ => {}
            }
        }

        // Numerics substitutions swap a faithful Win32 struct for its
        // layout-identical `Windows.Foundation.Numerics` projection (an ergonomic
        // gen-time choice — the winmd keeps the D2D/D3D struct). These must be
        // matched by name, not shape: the same `{ f32; f32 }` layout is reused
        // under many names that map to *different* Numerics types.
        //
        // The projection is applied for the `windows` crate (whose input carries
        // the WinRT `Windows.Foundation.Numerics` types, re-exported from the
        // `windows-numerics` crate) but NOT for `windows-sys`: the sys package is
        // generated from the Win32/WDK winmd alone, which has no Numerics types to
        // resolve against, so `windows-sys` keeps the raw D2D/D3D structs.
        if (win32_meta || is_flat_win32_stem) && !sys {
            match name {
                "D2D_MATRIX_3X2_F" => {
                    return Remap::Name(TypeName("Windows.Foundation.Numerics", "Matrix3x2"));
                }
                "D3DMATRIX" | "D2D_MATRIX_4X4_F" => {
                    return Remap::Name(TypeName("Windows.Foundation.Numerics", "Matrix4x4"));
                }
                "D2D_POINT_2F" | "D2D_VECTOR_2F" => {
                    return Remap::Name(TypeName("Windows.Foundation.Numerics", "Vector2"));
                }
                "D2D_VECTOR_4F" => {
                    return Remap::Name(TypeName("Windows.Foundation.Numerics", "Vector4"));
                }
                _ => {}
            }
        }

        Remap::None
    }

    pub fn generic_placeholders(count: usize) -> Vec<windows_metadata::Type> {
        (0..count)
            .map(|i| windows_metadata::Type::Generic(String::new(), i as u16))
            .collect()
    }

    #[track_caller]
    pub fn from_ref(
        code: TypeDefOrRef,
        enclosing: Option<&CppStruct>,
        generics: &[Self],
        reader: &Reader,
    ) -> Self {
        if let TypeDefOrRef::TypeSpec(def) = code {
            let mut blob = def.blob(0);
            let metadata_type = blob.read_type_code(&Self::generic_placeholders(generics.len()));
            return Self::from_metadata_type(&metadata_type, None, generics, reader);
        }

        let mut code_name = code.type_name();

        match Self::remap(code_name.namespace(), code_name.name(), reader.sys) {
            Remap::Type(ty) => return ty,
            Remap::Name(type_name) => {
                code_name = type_name;
            }
            Remap::None => {}
        }

        if let Some(outer) = enclosing {
            if code_name.namespace().is_empty() {
                return Self::CppStruct(outer.nested[code_name.name()].clone());
            }
        }

        reader.unwrap_full_name(code_name.namespace(), code_name.name())
    }

    #[track_caller]
    pub fn from_metadata_type(
        ty: &windows_metadata::Type,
        enclosing: Option<&CppStruct>,
        generics: &[Self],
        reader: &Reader,
    ) -> Self {
        match ty {
            windows_metadata::Type::Void => Self::Void,
            windows_metadata::Type::Bool => Self::Bool,
            windows_metadata::Type::Char => Self::Char,
            windows_metadata::Type::I8 => Self::I8,
            windows_metadata::Type::U8 => Self::U8,
            windows_metadata::Type::I16 => Self::I16,
            windows_metadata::Type::U16 => Self::U16,
            windows_metadata::Type::I32 => Self::I32,
            windows_metadata::Type::U32 => Self::U32,
            windows_metadata::Type::I64 => Self::I64,
            windows_metadata::Type::U64 => Self::U64,
            windows_metadata::Type::F32 => Self::F32,
            windows_metadata::Type::F64 => Self::F64,
            windows_metadata::Type::ISize => Self::ISize,
            windows_metadata::Type::USize => Self::USize,
            windows_metadata::Type::String => Self::String,
            windows_metadata::Type::Object => Self::Object,
            windows_metadata::Type::ClassName(tn) | windows_metadata::Type::ValueName(tn) => {
                let ns: &str = &tn.namespace;
                let n: &str = &tn.name;

                let (ns, n) = match Self::remap(ns, n, reader.sys) {
                    Remap::Type(ty) => return ty,
                    Remap::Name(type_name) => (type_name.namespace(), type_name.name()),
                    Remap::None => (ns, n),
                };

                if let Some(outer) = enclosing {
                    if ns.is_empty() {
                        return Self::CppStruct(outer.nested[n].clone());
                    }
                }
                let mut bindgen_ty = reader.unwrap_full_name(ns, n);
                if !tn.generics.is_empty() {
                    let item_generics: Vec<Self> = tn
                        .generics
                        .iter()
                        .map(|g| Self::from_metadata_type(g, None, generics, reader))
                        .collect();
                    bindgen_ty.set_generics(item_generics);
                }
                bindgen_ty
            }
            // Generic type parameter (ELEMENT_TYPE_VAR)
            windows_metadata::Type::Generic(_, index) => {
                generics.get(*index as usize).cloned().unwrap_or(Self::Void)
            }
            windows_metadata::Type::Array(inner) => Self::Array(Box::new(
                Self::from_metadata_type(inner, enclosing, generics, reader),
            )),
            windows_metadata::Type::ArrayFixed(inner, size) => Self::ArrayFixed(
                Box::new(Self::from_metadata_type(inner, enclosing, generics, reader)),
                *size,
            ),
            windows_metadata::Type::PtrMut(inner, pointers) => Self::PtrMut(
                Box::new(Self::from_metadata_type(inner, enclosing, generics, reader)),
                *pointers,
            ),
            windows_metadata::Type::PtrConst(inner, pointers) => Self::PtrConst(
                Box::new(Self::from_metadata_type(inner, enclosing, generics, reader)),
                *pointers,
            ),
            // IsConst modifier (ELEMENT_TYPE_CMOD_REQD IsConst)
            windows_metadata::Type::RefConst(inner) => Self::ConstRef(Box::new(
                Self::from_metadata_type(inner, enclosing, generics, reader),
            )),
            // ELEMENT_TYPE_BYREF - strip wrapper; byref is tracked by param flags.
            // BYREF + SZARRAY = ArrayRef.
            windows_metadata::Type::RefMut(inner) => match inner.as_ref() {
                windows_metadata::Type::Array(arr_inner) => Self::ArrayRef(Box::new(
                    Self::from_metadata_type(arr_inner, enclosing, generics, reader),
                )),
                _ => Self::from_metadata_type(inner, enclosing, generics, reader),
            },
        }
    }

    pub fn to_const_type(&self) -> Self {
        match self {
            Self::PtrMut(ty, pointers) => Self::PtrMut(Box::new(ty.to_const_type()), *pointers),
            Self::PtrConst(ty, pointers) => Self::PtrConst(Box::new(ty.to_const_type()), *pointers),
            Self::PSTR => Self::PCSTR,
            Self::PWSTR => Self::PCWSTR,
            _ => self.clone(),
        }
    }

    pub fn to_const_ptr(&self) -> Self {
        match self {
            Self::PtrMut(ty, pointers) => Self::PtrConst(ty.clone(), *pointers),
            _ => self.clone(),
        }
    }

    pub fn deref(&self) -> Self {
        match self {
            Self::PtrConst(ty, 1) | Self::PtrMut(ty, 1) => {
                if **ty == Self::Void {
                    Self::U8
                } else {
                    *ty.clone()
                }
            }
            Self::PtrConst(ty, pointers) => Self::PtrConst(ty.clone(), pointers - 1),
            Self::PtrMut(ty, pointers) => Self::PtrMut(ty.clone(), pointers - 1),
            Self::PSTR | Self::PCSTR => Self::U8,
            Self::PWSTR | Self::PCWSTR => Self::U16,
            _ => self.clone(),
        }
    }

    pub fn has_cpp_delegate(&self, reader: &Reader) -> bool {
        match self {
            Self::CppDelegate(..) => true,
            Self::CppStruct(ty) => ty.has_cpp_delegate(reader),
            _ => false,
        }
    }

    pub fn is_interface(&self) -> bool {
        matches!(
            self,
            Self::Class(_)
                | Self::Interface(_)
                | Self::Delegate(_)
                | Self::CppInterface(_)
                | Self::IUnknown
                | Self::Object
        )
    }

    pub fn write_name(&self, config: &Config) -> TokenStream {
        if config.bindgen.style.is_sys() && self.is_interface() {
            return quote! { *mut core::ffi::c_void };
        }

        match self {
            Self::Void => quote! { core::ffi::c_void },
            Self::Bool => quote! { bool },
            Self::I8 => quote! { i8 },
            Self::U8 => quote! { u8 },
            Self::I16 => quote! { i16 },
            Self::U16 | Self::Char => quote! { u16 },
            Self::I32 => quote! { i32 },
            Self::U32 => quote! { u32 },
            Self::I64 => quote! { i64 },
            Self::U64 => quote! { u64 },
            Self::F32 => quote! { f32 },
            Self::F64 => quote! { f64 },
            Self::ISize => quote! { isize },
            Self::USize => quote! { usize },
            Self::BSTR => {
                let core = config.write_core();
                quote! { #core BSTR }
            }
            Self::IUnknown => {
                let core = config.write_core();
                quote! { #core IUnknown }
            }
            Self::GUID => {
                let core = config.write_core();
                quote! { #core GUID }
            }
            Self::HRESULT => {
                let core = config.write_core();
                quote! { #core HRESULT }
            }
            Self::BOOL => {
                let core = config.write_core();
                quote! { #core BOOL }
            }
            Self::String => {
                let core = config.write_core();
                quote! { #core HSTRING }
            }
            Self::Object => {
                let core = config.write_core();
                quote! { #core IInspectable }
            }
            Self::PSTR => {
                let core = config.write_core();
                quote! { #core PSTR }
            }
            Self::PCSTR => {
                let core = config.write_core();
                quote! { #core PCSTR }
            }
            Self::PWSTR => {
                let core = config.write_core();
                quote! { #core PWSTR }
            }
            Self::PCWSTR => {
                let core = config.write_core();
                quote! { #core PCWSTR }
            }
            Self::CppInterface(ty) => ty.write_name(config),
            Self::Struct(ty) => ty.write_name(config),
            Self::Enum(ty) => ty.write_name(config),
            Self::Interface(ty) => ty.write_name(config),
            Self::CppStruct(ty) => ty.write_name(config),
            Self::CppEnum(ty) => ty.write_name(config),
            Self::CppFn(ty) => ty.write_name(config),
            Self::CppConst(ty) => ty.write_name(config),
            Self::CppDelegate(ty) => ty.write_name(config),
            Self::Delegate(ty) => ty.write_name(config),
            Self::Class(ty) => ty.write_name(config),
            Self::Generic(param) => to_ident(param.name()),
            Self::PtrMut(ty, pointers) => {
                let pointers = write_ptr_mut(*pointers);
                let ty = ty.write_default(config);
                quote! { #pointers #ty }
            }
            Self::PtrConst(ty, pointers) => {
                let pointers = write_ptr_const(*pointers);
                let ty = ty.write_default(config);
                quote! { #pointers #ty }
            }
            Self::ArrayFixed(ty, len) => {
                let name = ty.write_default(config);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Self::Array(ty) | Self::ArrayRef(ty) | Self::ConstRef(ty) => ty.write_name(config),
            Self::PrimitiveOrEnum(primitive, ty) => {
                if config.bindgen.style.is_sys() {
                    primitive.write_name(config)
                } else {
                    ty.write_name(config)
                }
            }
            rest => panic!("{rest:?}"),
        }
    }

    pub fn write_default(&self, config: &Config) -> TokenStream {
        if config.bindgen.style.is_sys() {
            return self.write_name(config);
        }

        if let Self::Array(ty) = self {
            ty.write_default(config)
        } else {
            let tokens = self.write_name(config);

            if matches!(self, Self::Generic(_)) {
                quote! { <#tokens as windows_core::Type<#tokens>>::Default }
            } else if self.is_interface() {
                quote! { Option<#tokens> }
            } else {
                tokens
            }
        }
    }

    pub fn write_impl_name(&self, config: &Config) -> TokenStream {
        match self {
            Self::IUnknown | Self::Object => {
                let core = config.write_core();
                quote! { #core IUnknownImpl }
            }
            Self::CppInterface(ty) => ty.write_impl_name(config),
            Self::Interface(ty) => ty.write_impl_name(config),
            rest => panic!("{rest:?}"),
        }
    }

    pub fn write_abi(&self, config: &Config) -> TokenStream {
        if config.bindgen.style.is_sys() {
            return self.write_name(config);
        }

        match self {
            Self::IUnknown
            | Self::Object
            | Self::Delegate(_)
            | Self::Class(_)
            | Self::CppInterface(_)
            | Self::Interface(_)
            | Self::String
            | Self::BSTR => quote! { *mut core::ffi::c_void },
            Self::ArrayFixed(ty, len) => {
                let name = ty.write_abi(config);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Self::Generic(name) => {
                let name = to_ident(name.name());
                quote! { windows_core::AbiType<#name> }
            }
            Self::Array(ty) | Self::ArrayRef(ty) if matches!(ty.as_ref(), Self::Generic(_)) => {
                ty.write_abi(config)
            }
            Self::Struct(ty) => {
                let name = self.write_name(config);
                if ty.is_copyable(config.reader) {
                    name
                } else {
                    quote! { core::mem::MaybeUninit<#name> }
                }
            }
            Self::PtrMut(ty, pointers) => {
                let ty = ty.write_abi(config);
                let pointers = write_ptr_mut(*pointers);
                quote! { #pointers #ty }
            }
            Self::PtrConst(ty, pointers) => {
                let ty = ty.write_abi(config);
                let pointers = write_ptr_const(*pointers);
                quote! { #pointers #ty }
            }
            Self::PrimitiveOrEnum(ty, _) => ty.write_name(config),
            ty => ty.write_name(config),
        }
    }

    pub fn runtime_signature(&self, reader: &Reader) -> String {
        match self {
            Self::Bool => "b1".to_string(),
            Self::Char => "c2".to_string(),
            Self::I8 => "i1".to_string(),
            Self::U8 => "u1".to_string(),
            Self::I16 => "i2".to_string(),
            Self::U16 => "u2".to_string(),
            Self::I32 => "i4".to_string(),
            Self::U32 => "u4".to_string(),
            Self::I64 => "i8".to_string(),
            Self::U64 => "u8".to_string(),
            Self::F32 => "f4".to_string(),
            Self::F64 => "f8".to_string(),
            Self::ISize => "is".to_string(),
            Self::USize => "us".to_string(),
            Self::String => "string".to_string(),
            Self::Object => "cinterface(IInspectable)".to_string(),
            Self::GUID => "g16".to_string(),
            Self::HRESULT => "struct(Windows.Foundation.HResult;i4)".to_string(),
            Self::Class(ty) => ty.runtime_signature(reader),
            Self::Delegate(ty) => ty.runtime_signature(reader),
            Self::Enum(ty) => ty.runtime_signature(reader),
            Self::Interface(ty) => ty.runtime_signature(reader),
            Self::Struct(ty) => ty.runtime_signature(reader),
            rest => panic!("{rest:?}"),
        }
    }

    pub fn split_generic(&self, reader: &Reader) -> (Self, Vec<Self>) {
        match self {
            Self::Interface(ty) if !ty.generics.is_empty() => {
                let base = reader.unwrap_full_name(ty.def.namespace(), ty.def.name());
                (base, ty.generics.clone())
            }
            Self::Delegate(ty) if !ty.generics.is_empty() => {
                let base = reader.unwrap_full_name(ty.def.namespace(), ty.def.name());
                (base, ty.generics.clone())
            }
            _ => (self.clone(), vec![]),
        }
    }

    pub fn decay(&self) -> &Self {
        match self {
            Self::PtrMut(ty, _)
            | Self::PtrConst(ty, _)
            | Self::Array(ty)
            | Self::ArrayRef(ty)
            | Self::ConstRef(ty)
            | Self::PrimitiveOrEnum(_, ty) => ty,
            Self::ArrayFixed(ty, _) => ty.decay(),
            _ => self,
        }
    }

    pub fn is_exclusive(&self) -> bool {
        match self {
            Self::Interface(ty) => ty.def.has_attribute("ExclusiveToAttribute"),
            _ => false,
        }
    }

    pub fn is_winrt_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_winrt_array_ref(&self) -> bool {
        matches!(self, Self::ArrayRef(_))
    }

    /// Returns `true` when `write_abi` emits `*mut core::ffi::c_void` for this
    /// type — i.e. the ABI representation is a raw COM/HSTRING pointer rather
    /// than the Rust type itself or `MaybeUninit<Self>`.
    pub fn has_pointer_abi(&self) -> bool {
        matches!(
            self,
            Self::IUnknown
                | Self::Object
                | Self::Delegate(_)
                | Self::Class(_)
                | Self::CppInterface(_)
                | Self::Interface(_)
                | Self::String
                | Self::BSTR
        )
    }

    pub fn is_async(&self) -> bool {
        match self {
            Self::Interface(ty) => ty.def.is_async(),
            _ => false,
        }
    }

    pub fn is_copyable(&self, reader: &Reader) -> bool {
        match self {
            Self::Struct(ty) => ty.is_copyable(reader),
            Self::CppStruct(ty) => ty.is_copyable(reader),
            Self::Interface(..) | Self::CppInterface(..) | Self::Class(..) | Self::Delegate(..) => {
                false
            }
            Self::String | Self::BSTR | Self::Object | Self::IUnknown | Self::Generic(_) => false,
            Self::ArrayFixed(ty, _) | Self::Array(ty) => ty.is_copyable(reader),
            _ => true,
        }
    }

    /// Returns `true` when the type implements `Eq` (no floating-point fields).
    pub fn is_eq(&self, reader: &Reader) -> bool {
        !matches!(self, Self::F32 | Self::F64)
            && match self {
                Self::Struct(ty) => ty
                    .def
                    .fields()
                    .all(|field| field.field_type(None, reader).is_eq(reader)),
                Self::ArrayFixed(ty, _) => ty.is_eq(reader),
                _ => true,
            }
    }

    pub fn is_dropped(&self, reader: &Reader) -> bool {
        match self {
            Self::Struct(ty) => !ty.is_copyable(reader),
            Self::CppInterface(..) => true,
            Self::String | Self::BSTR | Self::Object | Self::IUnknown => true,
            Self::ArrayFixed(ty, _) => ty.is_dropped(reader),
            _ => false,
        }
    }

    pub fn is_convertible(&self) -> bool {
        matches!(
            self,
            Self::Delegate(..)
                | Self::Interface(..)
                | Self::Class(..)
                | Self::CppInterface(..)
                | Self::PCSTR
                | Self::PCWSTR
                | Self::Object
                | Self::IUnknown
                | Self::Generic(_)
        )
    }

    pub fn is_const_ref(&self) -> bool {
        matches!(self, Self::ConstRef(_))
    }

    /// If this is `Windows.Foundation.IReference<T>` returns `Some(&T)`.
    pub fn as_ireference_inner(&self) -> Option<&Self> {
        if let Self::Interface(iface) = self {
            if iface.generics.len() == 1
                && iface.type_name() == TypeName("Windows.Foundation", "IReference")
            {
                return Some(&iface.generics[0]);
            }
        }
        None
    }

    /// If this type is `Windows.Foundation.IReference<T>` whose inner `T` is
    /// sugar-eligible (primitives, enums, copyable structs, GUID, or `HSTRING`),
    /// returns `Some(&T)`. Used by both input-parameter and return-position
    /// `Option`/unwrap sugar in bindgen.
    pub fn ireference_inner_for_sugar(&self, reader: &Reader) -> Option<&Self> {
        let inner = self.as_ireference_inner()?;
        match inner {
            // `HSTRING` is sugar-eligible: we accept `&str`/`String`/`HSTRING`
            // for input via `Into<IReference<HSTRING>>`, and unwrap to `HSTRING`
            // for return position.
            Self::String => Some(inner),
            Self::Generic(_)
            | Self::BSTR
            | Self::Object
            | Self::IUnknown
            | Self::Interface(_)
            | Self::CppInterface(_)
            | Self::Class(_)
            | Self::Delegate(_)
            | Self::CppDelegate(_) => None,
            _ if inner.is_copyable(reader) => Some(inner),
            _ => None,
        }
    }

    pub fn is_primitive(&self, reader: &Reader) -> bool {
        match self {
            Self::Enum(_) | Self::CppEnum(_) | Self::CppDelegate(_) => true,
            Self::CppStruct(ty) => ty.is_handle(reader),
            Self::Bool
            | Self::Char
            | Self::I8
            | Self::U8
            | Self::I16
            | Self::U16
            | Self::I32
            | Self::U32
            | Self::I64
            | Self::U64
            | Self::F32
            | Self::F64
            | Self::ISize
            | Self::USize
            | Self::HRESULT
            | Self::BOOL
            | Self::PtrConst(_, _)
            | Self::PtrMut(_, _) => true,
            _ => false,
        }
    }

    pub fn is_unsigned(&self) -> bool {
        matches!(
            self,
            Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::USize
        )
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, Self::PtrConst(_, _) | Self::PtrMut(_, _))
    }

    pub fn is_byte_size(&self) -> bool {
        match self {
            Self::PtrConst(ty, _) | Self::PtrMut(ty, _) => ty.is_byte_size(),
            Self::I8 | Self::U8 | Self::PSTR | Self::PCSTR => true,
            _ => false,
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Self::PtrConst(ty, _) | Self::PtrMut(ty, _) => ty.is_void(),
            Self::Void => true,
            _ => false,
        }
    }

    pub fn size(&self, reader: &Reader) -> usize {
        match self {
            Self::I8 | Self::U8 => 1,
            Self::I16 | Self::U16 => 2,
            Self::I64 | Self::U64 | Self::F64 => 8,
            Self::GUID => 16,
            Self::ArrayFixed(ty, len) => ty.size(reader) * len,
            Self::PrimitiveOrEnum(ty, _) => ty.size(reader),
            Self::CppStruct(ty) => ty.size(reader),
            Self::Struct(ty) => ty.size(reader),
            Self::CppEnum(ty) => ty.size(reader),
            _ => 4,
        }
    }

    pub fn align(&self, reader: &Reader) -> usize {
        match self {
            Self::I8 | Self::U8 => 1,
            Self::I16 | Self::U16 => 2,
            Self::I64 | Self::U64 | Self::F64 => 8,
            Self::ArrayFixed(ty, len) => ty.align(reader) * len,
            Self::CppStruct(ty) => ty.align(reader),
            Self::Struct(ty) => ty.align(reader),
            Self::CppEnum(ty) => ty.align(reader),
            _ => 4,
        }
    }

    pub fn underlying_type(&self, reader: &Reader) -> Self {
        match self {
            Self::Struct(ty) => ty.def.underlying_type_ext(reader),
            Self::CppEnum(ty) => ty.def.underlying_type_ext(reader),
            Self::Enum(ty) => ty.def.underlying_type_ext(reader),
            Self::CppStruct(ty) => ty.def.underlying_type_ext(reader),
            Self::HRESULT => Self::I32,
            Self::BOOL => Self::I32,
            _ => self.clone(),
        }
    }

    fn write_no_deps(&self, config: &Config) -> TokenStream {
        if !config.bindgen.uses_inline_core_types() {
            return quote! {};
        }

        match self {
            Self::HRESULT => quote! { pub type HRESULT = i32; },
            Self::BOOL => quote! { pub type BOOL = i32; },

            Self::PWSTR => quote! { pub type PWSTR = *mut u16; },
            Self::PCSTR => quote! { pub type PCSTR = *const u8; },
            Self::PSTR => quote! { pub type PSTR = *mut u8; },
            Self::PCWSTR => quote! { pub type PCWSTR = *const u16; },
            Self::BSTR => quote! { pub type BSTR = *const u16; },
            Self::String => {
                quote! { pub type HSTRING = *mut core::ffi::c_void; }
            }
            Self::GUID => quote! {
                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct GUID {
                    pub data1: u32,
                    pub data2: u16,
                    pub data3: u16,
                    pub data4: [u8; 8],
                }
            },
            Self::IUnknown => quote! {
                pub const IID_IUnknown: GUID = GUID { data1: 0x00000000, data2: 0x0000, data3: 0x0000, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
                #[repr(C)]
                pub struct IUnknown_Vtbl {
                    pub QueryInterface: unsafe extern "system" fn(this: *mut core::ffi::c_void, iid: *const GUID, interface: *mut *mut core::ffi::c_void) -> HRESULT,
                    pub AddRef: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
                    pub Release: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
                }
            },
            Self::Object => quote! {
                pub const IID_IInspectable: GUID = GUID { data1: 0xaf86e2e0, data2: 0xb12d, data3: 0x4c6a, data4: [156, 90, 215, 170, 101, 16, 30, 144] };
                #[repr(C)]
                pub struct IInspectable_Vtbl {
                    pub base: IUnknown_Vtbl,
                    pub GetIids: unsafe extern "system" fn(this: *mut core::ffi::c_void, count: *mut u32, values: *mut *mut GUID) -> HRESULT,
                    pub GetRuntimeClassName: unsafe extern "system" fn(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> HRESULT,
                    pub GetTrustLevel: unsafe extern "system" fn(this: *mut core::ffi::c_void, value: *mut i32) -> HRESULT,
                }
            },

            _ => quote! {},
        }
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        match self {
            Self::Struct(ty) => ty.write(config),
            Self::Enum(ty) => ty.write(config),
            Self::Interface(ty) => ty.write(config),
            Self::CppStruct(ty) => ty.write(config),
            Self::CppEnum(ty) => ty.write(config),
            Self::CppFn(ty) => ty.write(config),
            Self::CppConst(ty) => ty.write(config),
            Self::CppDelegate(ty) => ty.write(config),
            Self::Delegate(ty) => ty.write(config),
            Self::Class(ty) => ty.write(config),
            Self::CppInterface(ty) => ty.write(config),

            _ => self.write_no_deps(config),
        }
    }

    pub fn set_generics(&mut self, generics: Vec<Self>) {
        match self {
            Self::Interface(ty) => ty.generics = generics,
            Self::Delegate(ty) => ty.generics = generics,
            rest => panic!("{rest:?}"),
        }
    }

    pub fn type_name(&self) -> TypeName {
        match self {
            Self::Class(ty) => ty.type_name(),
            Self::Delegate(ty) => ty.type_name(),
            Self::Enum(ty) => ty.type_name(),
            Self::Interface(ty) => ty.type_name(),
            Self::Struct(ty) => ty.type_name(),
            Self::CppDelegate(ty) => ty.type_name(),
            Self::CppEnum(ty) => ty.type_name(),
            Self::CppInterface(ty) => ty.type_name(),
            Self::CppStruct(ty) => ty.type_name(),
            Self::CppConst(ty) => ty.type_name(),
            Self::CppFn(ty) => ty.type_name(),

            Self::PSTR => TypeName("", "PSTR"),
            Self::PCSTR => TypeName("", "PCSTR"),
            Self::PWSTR => TypeName("", "PWSTR"),
            Self::PCWSTR => TypeName("", "PCWSTR"),
            Self::GUID => TypeName("", "GUID"),
            Self::HRESULT => TypeName("", "HRESULT"),
            Self::BOOL => TypeName("", "BOOL"),
            Self::IUnknown => TypeName("", "IUnknown"),
            Self::BSTR => TypeName("", "BSTR"),
            Self::String => TypeName("", "String"),
            Self::Object => TypeName("", "Object"),

            _ => TypeName("", ""),
        }
    }

    pub fn is_core(&self) -> bool {
        matches!(
            self,
            Self::PSTR
                | Self::PCSTR
                | Self::PWSTR
                | Self::PCWSTR
                | Self::GUID
                | Self::HRESULT
                | Self::BOOL
                | Self::IUnknown
                | Self::Object
                | Self::BSTR
                | Self::String
        )
    }

    pub fn write_result_map(&self, reader: &Reader) -> TokenStream {
        if self.is_copyable(reader) {
            quote! { map(|| result__) }
        } else if self.is_convertible() {
            quote! { and_then(||windows_core::Type::from_abi(result__)) }
        } else {
            quote! { map(|| core::mem::transmute(result__)) }
        }
    }
}

impl Dependencies for Type {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        let ty = self.decay();

        if ty.is_intrinsic() {
            return;
        }

        let mut nested = false;

        if let Self::CppStruct(ty) = ty {
            if ty.def.namespace().is_empty() {
                nested = true;
            }
        }

        let (ty, generics) = ty.split_generic(reader);

        for ty in generics {
            ty.combine(dependencies, reader);
        }

        if !nested && !dependencies.insert(ty.clone()) {
            return;
        }

        if let Some(multi) = match &ty {
            Self::CppStruct(ty) => Some(reader.with_full_name(ty.def.namespace(), ty.def.name())),
            Self::CppFn(ty) => Some(reader.with_full_name(ty.namespace, ty.method.name())),
            Self::CppDelegate(ty) => Some(reader.with_full_name(ty.def.namespace(), ty.def.name())),
            Self::CppConst(ty) => Some(reader.with_full_name(ty.namespace, ty.field.name())),
            _ => None,
        } {
            multi.for_each(|multi| {
                if ty != multi {
                    multi.combine(dependencies, reader);
                }
            });
        }

        match &ty {
            Self::Class(ty) => ty.combine(dependencies, reader),
            Self::Delegate(ty) => ty.combine(dependencies, reader),
            Self::Enum(..) => {}
            Self::Interface(ty) => ty.combine(dependencies, reader),
            Self::Struct(ty) => ty.combine(dependencies, reader),
            Self::CppConst(ty) => ty.combine(dependencies, reader),
            Self::CppDelegate(ty) => ty.combine(dependencies, reader),
            Self::CppFn(ty) => ty.combine(dependencies, reader),
            Self::CppInterface(ty) => ty.combine(dependencies, reader),
            Self::CppStruct(ty) => ty.combine(dependencies, reader),
            Self::CppEnum(ty) => ty.combine(dependencies, reader),

            Self::IUnknown => {
                Self::GUID.combine(dependencies, reader);
                Self::HRESULT.combine(dependencies, reader);
            }

            Self::Object => {
                Self::IUnknown.combine(dependencies, reader);
            }

            _ => {}
        }
    }
}

pub fn interface_signature(def: TypeDef, generics: &[Type], reader: &Reader) -> String {
    if generics.is_empty() {
        let guid = def.guid_attribute().unwrap();
        format!("{{{guid}}}")
    } else {
        let guid = def.guid_attribute().unwrap();
        let mut signature = format!("pinterface({{{guid}}}");

        for generic in generics {
            signature.push(';');
            signature.push_str(&generic.runtime_signature(reader));
        }

        signature.push(')');
        signature
    }
}

fn write_ptr_mut(pointers: usize) -> TokenStream {
    "*mut ".repeat(pointers).parse().unwrap()
}

fn write_ptr_const(pointers: usize) -> TokenStream {
    "*const ".repeat(pointers).parse().unwrap()
}

/// Helper for types whose `write_cfg` only needs their own dependencies.
/// Returns an empty token stream when packaging is disabled.
fn write_simple_cfg(ty: &impl Dependencies, config: &Config) -> TokenStream {
    if !config.bindgen.layout.is_package() {
        return quote! {};
    }
    Cfg::new(&ty.dependencies(config.reader), config).write(config, false)
}

/// Helper for types whose `write_cfg` needs to return both the `Cfg` value and its token form.
/// Returns default/empty values when packaging is disabled.
fn write_full_cfg(ty: &impl Dependencies, config: &Config) -> (Cfg, TokenStream) {
    if !config.bindgen.layout.is_package() {
        return (Cfg::default(), quote! {});
    }
    let cfg = Cfg::new(&ty.dependencies(config.reader), config);
    let tokens = cfg.write(config, false);
    (cfg, tokens)
}

/// Emit a `#[cfg(target_arch = ...)]` attribute when a metadata row carries
/// a `[SupportedArchitectureAttribute]`. Independent of `--package` /
/// `--flat` layout — the generated arch gate is always meaningful.
pub fn write_arches<R: HasAttributes<'static>>(row: R) -> TokenStream {
    let mut tokens = quote! {};

    if let Some(attribute) = row.find_attribute("SupportedArchitectureAttribute") {
        let arch_value = match attribute.value().first() {
            Some((_, Value::I32(v))) => Some(*v),
            Some((_, Value::EnumValue(_, inner))) => {
                if let Value::I32(v) = inner.as_ref() {
                    Some(*v)
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(value) = arch_value {
            let mut arches = BTreeSet::new();

            if value & 1 == 1 {
                arches.insert("x86");
            }

            if value & 2 == 2 {
                arches.insert("x86_64");
                arches.insert("arm64ec");
            }

            if value & 4 == 4 {
                arches.insert("aarch64");
            }

            match arches.len() {
                0 => {}
                1 => tokens.combine(quote! { #[cfg(#(target_arch = #arches),*)] }),
                _ => tokens.combine(quote! { #[cfg(any(#(target_arch = #arches),*))] }),
            }
        }
    }

    tokens
}

/// Wraps a primitive `value` through the nested newtype layers of `ty` so it can
/// stand in for a full-mode handle / scalar-typedef value. The new metadata
/// preserves scalar typedefs (e.g. `JET_UINT32`, `DBLENGTH`) as named types; in
/// full mode these render as newtype structs, so a bare integer (a constant, or an
/// array length via `len().try_into()`) must be wrapped in one constructor per
/// layer, bottoming out at the primitive or pointer. In `--sys`/`--minimal` mode
/// these typedefs collapse to bare aliases, so no wrapping is applied and output is
/// unchanged (which is also why the published bindings are byte-identical).
pub(crate) fn write_newtype_wrap(ty: &Type, value: &TokenStream, config: &Config) -> TokenStream {
    if !config.bindgen.style.emit_bare_typedef() {
        if let Type::CppStruct(s) = ty {
            if s.is_handle(config.reader) {
                let name = ty.write_name(config);
                let inner = ty.underlying_type(config.reader);
                let arg = write_newtype_wrap(&inner, value, config);
                return quote! { #name(#arg) };
            }
        }
    }
    value.clone()
}
