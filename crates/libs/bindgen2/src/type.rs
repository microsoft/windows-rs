use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Type {
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

    Item(Item),
    Param(&'static str),
    PtrMut(Box<Self>, usize),
    PtrConst(Box<Self>, usize),
    ArrayFixed(Box<Self>, usize),
    Array(Box<Self>),
    ArrayRef(Box<Self>),
    ConstRef(Box<Self>),
    PrimitiveOrEnum(Box<Self>, Item),
}

#[derive(PartialEq)]
pub enum Remap {
    Type(Type),
    Name(TypeName<'static>),
    None,
}

impl Type {
    pub fn remap(namespace: &'static str, name: &'static str) -> Remap {
        match TypeName(namespace, name) {
            TypeName::GUID => Remap::Type(Self::GUID),
            TypeName::HResult => Remap::Type(Self::HRESULT),
            TypeName::HRESULT => Remap::Type(Self::HRESULT),
            TypeName::PSTR => Remap::Type(Self::PSTR),
            TypeName::PWSTR => Remap::Type(Self::PWSTR),
            TypeName::HSTRING => Remap::Type(Self::String),
            TypeName::BSTR => Remap::Type(Self::BSTR),
            TypeName::IInspectable => Remap::Type(Self::Object),
            TypeName::CHAR => Remap::Type(Self::I8),
            TypeName::IUnknown => Remap::Type(Self::IUnknown),
            TypeName::Type => Remap::Type(Self::Type),

            TypeName::D2D_MATRIX_3X2_F => Remap::Name(TypeName::Matrix3x2),
            TypeName::D3DMATRIX => Remap::Name(TypeName::Matrix4x4),

            _ => Remap::None,
        }
    }

    pub fn from_element_type(code: usize) -> Option<Self> {
        match code as u8 {
            ELEMENT_TYPE_VOID => Some(Self::Void),
            ELEMENT_TYPE_BOOLEAN => Some(Self::Bool),
            ELEMENT_TYPE_CHAR => Some(Self::Char),
            ELEMENT_TYPE_I1 => Some(Self::I8),
            ELEMENT_TYPE_U1 => Some(Self::U8),
            ELEMENT_TYPE_I2 => Some(Self::I16),
            ELEMENT_TYPE_U2 => Some(Self::U16),
            ELEMENT_TYPE_I4 => Some(Self::I32),
            ELEMENT_TYPE_U4 => Some(Self::U32),
            ELEMENT_TYPE_I8 => Some(Self::I64),
            ELEMENT_TYPE_U8 => Some(Self::U64),
            ELEMENT_TYPE_R4 => Some(Self::F32),
            ELEMENT_TYPE_R8 => Some(Self::F64),
            ELEMENT_TYPE_I => Some(Self::ISize),
            ELEMENT_TYPE_U => Some(Self::USize),
            ELEMENT_TYPE_STRING => Some(Self::String),
            ELEMENT_TYPE_OBJECT => Some(Self::Object),
            _ => None,
        }
    }

    pub fn from_ref(code: TypeDefOrRef, enclosing: Option<&CppStruct>, generics: &[Self]) -> Self {
        if let TypeDefOrRef::TypeSpec(def) = code {
            let mut blob = def.blob(0);
            return Self::from_blob_impl(&mut blob, None, generics);
        }

        let mut namespace = code.namespace();
        let mut name = code.name();

        match Self::remap(namespace, name) {
            Remap::Type(ty) => return ty,
            Remap::Name(type_name) => {
                namespace = type_name.0;
                name = type_name.1;
            }
            Remap::None => {}
        }

        // TODO: this needs to be deferred via a TypeName's optional nested type name?
        if let Some(outer) = enclosing {
            if namespace.is_empty() {
                return Self::Item(outer.nested[name].clone());
            }
        }

        if let Some(item) = code.reader().with_full_name(namespace, name).next() {
            Self::Item(item)
        } else {
            panic!("windows-bindgen: type not found: {namespace}.{name}")
        }
    }

    pub fn from_blob(blob: &mut Blob, enclosing: Option<&CppStruct>, generics: &[Self]) -> Self {
        // Used by WinRT to indicate that a struct input parameter is passed by reference rather than by value on the ABI.
        let is_const = blob.read_modifiers().iter().any(|def| {
            let type_name = TypeName(def.namespace(), def.name());
            type_name == TypeName::IsConst
        });

        // Used by WinRT to indicate an output parameter, but there are other ways to determine this direction so here
        // it is only used to distinguish between slices and heap-allocated arrays.
        let is_ref = blob.try_read(ELEMENT_TYPE_BYREF as usize);

        if blob.try_read(ELEMENT_TYPE_VOID as usize) {
            return Self::Void;
        }

        let is_array = blob.try_read(ELEMENT_TYPE_SZARRAY as usize); // Used by WinRT to indicate an array

        let mut pointers = 0;

        while blob.try_read(ELEMENT_TYPE_PTR as usize) {
            pointers += 1;
        }

        let kind = Self::from_blob_impl(blob, enclosing, generics);

        if pointers > 0 {
            Self::PtrMut(Box::new(kind), pointers)
        } else if is_const {
            Self::ConstRef(Box::new(kind))
        } else if is_array {
            if is_ref {
                Self::ArrayRef(Box::new(kind))
            } else {
                Self::Array(Box::new(kind))
            }
        } else {
            kind
        }
    }

    fn from_blob_impl(blob: &mut Blob, enclosing: Option<&CppStruct>, generics: &[Self]) -> Self {
        let code = blob.read_usize();

        if let Some(code) = Self::from_element_type(code) {
            return code;
        }

        match code as u8 {
            ELEMENT_TYPE_VALUETYPE | ELEMENT_TYPE_CLASS => {
                Self::from_ref(blob.decode(), enclosing, generics)
            }
            ELEMENT_TYPE_VAR => generics
                .get(blob.read_usize())
                .unwrap_or(&Self::Void)
                .clone(),
            ELEMENT_TYPE_ARRAY => {
                let kind = Self::from_blob(blob, enclosing, generics);
                let _rank = blob.read_usize();
                let _count = blob.read_usize();
                let bounds = blob.read_usize();
                Self::ArrayFixed(Box::new(kind), bounds)
            }
            ELEMENT_TYPE_GENERICINST => {
                blob.read_usize(); // ELEMENT_TYPE_VALUETYPE or ELEMENT_TYPE_CLASS

                let code = blob.decode::<TypeDefOrRef>();
                let namespace = code.namespace();
                let name = code.name();

                let mut item = blob
                    .reader()
                    .with_full_name(namespace, name)
                    .next()
                    .unwrap_or_else(|| {
                        panic!("windows-bindgen: type not found: {namespace}.{name}")
                    });

                let mut item_generics = vec![];

                for _ in 0..blob.read_usize() {
                    item_generics.push(Self::from_blob_impl(blob, enclosing, generics));
                }

                item.set_generics(item_generics);
                Self::Item(item)
            }
            rest => panic!("windows-bindgen: {rest:?}"),
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
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

    pub fn is_nullable(&self) -> bool {
        match self {
            Self::Item(item) => item.is_nullable(),
            Self::IUnknown | Self::Object => true,
            _ => false,
        }
    }

    pub fn write(&self, writer: &Writer) -> TokenStream {
        match self {
            Self::Void => quote! { core::ffi::c_void },
            Self::Bool => quote! { bool },
            Self::Char => quote! { u16 },
            Self::I8 => quote! { i8 },
            Self::U8 => quote! { u8 },
            Self::I16 => quote! { i16 },
            Self::U16 => quote! { u16 },
            Self::I32 => quote! { i32 },
            Self::U32 => quote! { u32 },
            Self::I64 => quote! { i64 },
            Self::U64 => quote! { u64 },
            Self::F32 => quote! { f32 },
            Self::F64 => quote! { f64 },
            Self::ISize => quote! { isize },
            Self::USize => quote! { usize },
            Self::BSTR => {
                let name = writer.write_core();
                quote! { #name BSTR }
            }
            Self::IUnknown => {
                if writer.config.sys {
                    quote! { *mut core::ffi::c_void }
                } else {
                    let name = writer.write_core();
                    quote! { #name IUnknown }
                }
            }
            Self::GUID => {
                let name = writer.write_core();
                quote! { #name GUID }
            }
            Self::HRESULT => {
                let name = writer.write_core();
                quote! { #name HRESULT }
            }
            Self::String => {
                let name = writer.write_core();
                quote! { #name HSTRING }
            }
            Self::Object => {
                if writer.config.sys {
                    quote! { *mut core::ffi::c_void }
                } else {
                    let name = writer.write_core();
                    quote! { #name IInspectable }
                }
            }
            Self::PSTR => {
                let name = writer.write_core();
                quote! { #name PSTR }
            }
            Self::PCSTR => {
                let name = writer.write_core();
                quote! { #name PCSTR }
            }
            Self::PWSTR => {
                let name = writer.write_core();
                quote! { #name PWSTR }
            }
            Self::PCWSTR => {
                let name = writer.write_core();
                quote! { #name PCWSTR }
            }
            Self::Item(item) => item.write_name(writer),
            Self::Param(param) => to_ident(param),
            Self::PtrMut(ty, pointers) => {
                let pointers = write_ptr_mut(*pointers);
                let ty = ty.write_default(writer);
                quote! { #pointers #ty }
            }
            Self::PtrConst(ty, pointers) => {
                let pointers = write_ptr_const(*pointers);
                let ty = ty.write_default(writer);
                quote! { #pointers #ty }
            }
            Self::ArrayFixed(ty, len) => {
                let name = ty.write_default(writer);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Self::Array(ty) => ty.write(writer),
            Self::ArrayRef(ty) => ty.write(writer),
            Self::ConstRef(ty) => ty.write(writer),
            Self::PrimitiveOrEnum(ty, item) => {
                if writer.config.sys {
                    ty.write(writer)
                } else {
                    item.write_name(writer)
                }
            }
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

    pub fn write_default(&self, writer: &Writer) -> TokenStream {
        if let Self::Array(ty) = self {
            ty.write_default(writer)
        } else {
            let tokens = self.write(writer);

            if matches!(self, Self::Param(_)) {
                quote! { <#tokens as windows_core::Type<#tokens>>::Default }
            } else if self.is_nullable() && !writer.config.sys {
                quote! { Option<#tokens> }
            } else {
                tokens
            }
        }
    }

    pub fn write_impl_name(&self, writer: &Writer) -> TokenStream {
        match self {
            Self::IUnknown => {
                let name = writer.write_core();
                quote! { #name IUnknownImpl }
            }
            Self::Object => {
                let name = writer.write_core();
                // TODO: ideally we can only require RuntimeName tough IInspectable_Impl
                quote! { #name IUnknownImpl }
            }
            Self::Item(Item::CppInterface(item)) => item.write_impl_name(writer),
            Self::Item(Item::Interface(item)) => item.write_impl_name(writer),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

    pub fn write_abi(&self, writer: &Writer) -> TokenStream {
        match self {
            Self::IUnknown
            | Self::Object
            | Self::Item(Item::Delegate(_))
            | Self::Item(Item::Class(_))
            | Self::Item(Item::CppInterface(_))
            | Self::Item(Item::Interface(_))
            | Self::String
            | Self::BSTR => quote! { *mut core::ffi::c_void },
            Self::ArrayFixed(ty, len) => {
                let name = ty.write_abi(writer);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Self::Param(name) => {
                let name = to_ident(name);
                quote! { windows_core::AbiType<#name> }
            }
            Self::Item(Item::Struct(item)) => {
                let name = self.write(writer);
                if item.is_copyable() {
                    name
                } else {
                    quote! { core::mem::MaybeUninit<#name> }
                }
            }
            Self::PtrMut(ty, pointers) => {
                let ty = ty.write_abi(writer);
                let pointers = write_ptr_mut(*pointers);
                quote! { #pointers #ty }
            }
            Self::PtrConst(ty, pointers) => {
                let ty = ty.write_abi(writer);
                let pointers = write_ptr_const(*pointers);
                quote! { #pointers #ty }
            }
            Self::PrimitiveOrEnum(ty, _) => ty.write(writer),
            ty => ty.write(writer),
        }
    }

    pub fn runtime_signature(&self) -> String {
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
            Self::Item(item) => item.runtime_signature(),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        match self {
            Self::PtrMut(ty, _) => ty.dependencies(dependencies),
            Self::PtrConst(ty, _) => ty.dependencies(dependencies),
            Self::ArrayFixed(ty, _) => ty.dependencies(dependencies),
            Self::Array(ty) => ty.dependencies(dependencies),
            Self::ArrayRef(ty) => ty.dependencies(dependencies),
            Self::ConstRef(ty) => ty.dependencies(dependencies),
            Self::PrimitiveOrEnum(_, ty) => ty.dependencies(dependencies),
            Self::String => {
                dependencies.insert("", "String");
            }
            Self::BSTR => {
                dependencies.insert("", "BSTR");
            }
            Self::Object => {
                dependencies.insert("", "Object");
            }
            Self::IUnknown => {
                dependencies.insert("", "IUnknown");
            }
            Self::PSTR => {
                dependencies.insert("", "PSTR");
            }
            Self::PCSTR => {
                dependencies.insert("", "PCSTR");
            }
            Self::PWSTR => {
                dependencies.insert("", "PWSTR");
            }
            Self::PCWSTR => {
                dependencies.insert("", "PCWSTR");
            }
            Self::GUID => {
                dependencies.insert("", "GUID");
            }
            Self::HRESULT => {
                dependencies.insert("", "HRESULT");
            }
            Self::Item(item) => {
                item.dependencies(dependencies);
            }
            _ => {}
        }
    }

    pub fn is_exclusive(&self) -> bool {
        match self {
            Self::Item(Item::Interface(item)) => item.def.has_attribute("ExclusiveToAttribute"),
            _ => false,
        }
    }

    pub fn is_winrt_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_winrt_array_ref(&self) -> bool {
        matches!(self, Self::ArrayRef(_))
    }

    pub fn is_async(&self) -> bool {
        match self {
            Self::Item(Item::Interface(item)) => item.def.is_async(),
            _ => false,
        }
    }

    pub fn is_copyable(&self) -> bool {
        match self {
            Self::Item(item) => item.is_copyable(),
            Self::String | Self::BSTR | Self::Object | Self::IUnknown | Self::Param(_) => false,
            Self::ArrayFixed(ty, _) => ty.is_copyable(),
            Self::Array(ty) => ty.is_copyable(),
            _ => true,
        }
    }

    pub fn is_dropped(&self) -> bool {
        match self {
            Self::Item(item) => item.is_dropped(),
            Self::String | Self::BSTR | Self::Object | Self::IUnknown => true,
            Self::ArrayFixed(ty, _) => ty.is_dropped(),
            _ => false,
        }
    }

    pub fn is_convertible(&self) -> bool {
        match self {
            Self::Item(item) => item.is_convertible(),
            Self::PCSTR | Self::PCWSTR | Self::Object | Self::IUnknown | Self::Param(_) => true,
            _ => false,
        }
    }

    pub fn is_const_ref(&self) -> bool {
        matches!(self, Self::ConstRef(_))
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Item(item) => item.is_primitive(),
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

    pub fn has_explicit_layout(&self) -> bool {
        match self {
            Self::Item(Item::CppStruct(item)) => item.has_explicit_layout(),
            Self::ArrayFixed(ty, _) => ty.has_explicit_layout(),
            _ => false,
        }
    }

    pub fn has_packing(&self) -> bool {
        match self {
            Self::Item(Item::CppStruct(item)) => item.has_packing(),
            Self::ArrayFixed(ty, _) => ty.has_packing(),
            _ => false,
        }
    }

    pub fn is_byte_size(&self) -> bool {
        match self {
            Self::PtrConst(ty, _) | Self::PtrMut(ty, _) => ty.is_byte_size(),
            Self::I8 | Self::U8 | Self::PSTR | Self::PCSTR => true,
            _ => false,
        }
    }

    pub fn is_handle(&self) -> bool {
        if let Self::Item(Item::CppStruct(item)) = self {
            item.is_handle()
        } else {
            false
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Type::PtrConst(ty, _) | Type::PtrMut(ty, _) => ty.is_void(),
            Type::Void => true,
            _ => false,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::GUID => 16,
            Type::Item(item) => item.size(),
            Type::ArrayFixed(ty, len) => ty.size() * len,
            Type::PrimitiveOrEnum(ty, _) => ty.size(),
            _ => 4,
        }
    }

    pub fn align(&self) -> usize {
        match self {
            Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::GUID => 4,
            Type::Item(item) => item.align(),
            Type::ArrayFixed(ty, len) => ty.align() * len,
            _ => 4,
        }
    }

    pub fn underlying_type(&self) -> Self {
        match self {
            Type::Item(item) => item.underlying_type(),
            Type::HRESULT => Type::I32,
            _ => self.clone(),
        }
    }
}

fn write_ptr_mut(pointers: usize) -> TokenStream {
    "*mut ".repeat(pointers).into()
}

fn write_ptr_const(pointers: usize) -> TokenStream {
    "*const ".repeat(pointers).into()
}
