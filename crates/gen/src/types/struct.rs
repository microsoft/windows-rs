use super::*;

// TODO: need to split win32 and winrt structs as their signatures are different and win32 structs also include unions and they are
// radically different.

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Struct(pub tables::TypeDef);

impl Struct {
    pub fn type_signature(&self) -> String {
        let mut result = format!("struct({}.{}", self.0.namespace(), self.0.name());

        for field in self.0.fields() {
            result.push(';');
            result.push_str(&field.signature().kind.type_signature());
        }

        result.push(')');
        result
    }

    pub fn dependencies(&self) -> Vec<ElementType> {
        let reader = TypeReader::get();

        // TODO: add tests for each
        match self.0.full_name() {
            ("Windows.Win32.Automation", "BSTR") => vec![
                reader.resolve_type("Windows.Win32.Automation", "SysAllocStringLen"),
                reader.resolve_type("Windows.Win32.Automation", "SysStringLen"),
                reader.resolve_type("Windows.Win32.Automation", "SysFreeString"),
            ],
            ("Windows.Foundation.Numerics", "Matrix3x2") => {
                vec![reader.resolve_type("Windows.Win32.Direct2D", "D2D1MakeRotateMatrix")]
            }
            _ => {
                let mut dependencies: Vec<ElementType> =
                    self.0.fields().map(|f| f.definition()).flatten().collect();

                if let Some(dependency) = self.0.is_convertible() {
                    dependencies.push(dependency);
                }

                dependencies
            }
        }
    }

    pub fn definition(&self) -> Vec<ElementType> {
        vec![ElementType::Struct(self.clone())]
    }

    pub fn is_blittable(&self) -> bool {
        // TODO: should this only be applied to types where we actually take advantage of the RAII attribute?
        if self.0.full_name() == ("Windows.Win32.Automation", "BSTR") {
            false
        } else {
            self.0.fields().all(|f| f.is_blittable())
        }
    }

    pub fn is_handle(&self) -> bool {
        self.0.has_attribute("NativeTypedefAttribute")
    }

    pub fn gen_abi_name(&self, gen: &Gen) -> TokenStream {
        if self.is_blittable() {
            self.0.gen_name(gen)
        } else {
            self.0.gen_abi_name(gen)
        }
    }

    pub fn gen(&self, gen: &Gen) -> TokenStream {
        self.gen_struct(self.0.name(), gen)
    }

    fn gen_struct(&self, struct_name: &str, gen: &Gen) -> TokenStream {
        if let Some(replacement) = self.gen_replacement() {
            return replacement;
        }

        let name = to_ident(struct_name);

        if let Some(guid) = Guid::from_attributes(self.0.attributes()) {
            let guid = guid.gen();

            return quote! {
                pub const #name: ::windows::Guid = ::windows::Guid::from_values(#guid);
            };
        }

        let fields: Vec<(tables::Field, Signature, Ident)> = self
            .0
            .fields()
            .filter_map(|f| {
                if f.flags().literal() {
                    None
                } else {
                    Some((f, f.signature(), to_ident(f.name())))
                }
            })
            .collect();

        if fields.is_empty() {
            return quote! {
                #[repr(C)]
                #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::marker::Copy)]
                pub struct #name(pub u8);
            };
        }

        let is_winrt = self.0.is_winrt();
        let is_handle = self.is_handle();
        let is_union = self.0.flags().explicit();
        let layout = self.0.class_layout();

        let repr = if let Some(layout) = layout {
            let packing = Literal::u32_unsuffixed(layout.packing_size());
            quote! { #[repr(C, packed(#packing))] }
        } else {
            quote! { #[repr(C)] }
        };

        // TODO: add test for Windows.Win32.Security.TRUSTEE_A
        let has_union = fields
            .iter()
            .any(|(_, signature, _)| signature.is_explicit());

        // TODO: workaround for getting windows-docs building
        let has_complex_array = fields
            .iter()
            .any(|(_, signature, _)| match &signature.kind {
                ElementType::Array((signature, _)) => {
                    !signature.is_blittable() || signature.kind.is_nullable()
                }
                _ => false,
            });

        let runtime_type = if is_winrt {
            let signature = Literal::byte_string(&self.type_signature().as_bytes());

            quote! {
                unsafe impl ::windows::RuntimeType for #name {
                    type DefaultType = Self;
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(#signature);
                }
            }
        } else {
            quote! {}
        };

        let clone_or_copy = if self.is_blittable() {
            quote! {
                #[derive(::std::clone::Clone, ::std::marker::Copy)]
            }
        } else if is_union || has_union || layout.is_some() {
            quote! {}
        } else {
            quote! {
                #[derive(::std::clone::Clone)]
            }
        };

        let body = if is_handle {
            let fields = fields.iter().map(|(_, signature, _)| {
                let kind = if is_winrt {
                    signature.gen_winrt(gen)
                } else {
                    signature.gen_win32(gen)
                };

                quote! {
                    pub #kind
                }
            });

            quote! {
                ( #(#fields),* );
            }
        } else {
            let fields = fields.iter().map(|(_, signature, name)| {
                let kind = if is_winrt {
                    signature.gen_winrt(gen)
                } else if is_union {
                    signature.gen_win32_abi(gen)
                } else {
                    signature.gen_win32(gen)
                };

                quote! {
                    pub #name: #kind
                }
            });

            quote! {
                { #(#fields),* }
            }
        };

        let struct_or_union = if is_union {
            quote! { union }
        } else {
            quote! { struct }
        };

        let abi = if self.is_blittable() {
            quote! {
                unsafe impl ::windows::Abi for #name {
                    type Abi = Self;
                }
            }
        } else {
            let abi_name = self.0.gen_abi_name(gen);

            let fields = if is_winrt {
                let fields = fields.iter().map(|(_, signature, name)| {
                    let kind = signature.gen_winrt_abi(gen);
                    quote! { pub #name: #kind }
                });
                quote! { #(#fields),* }
            } else {
                let fields = fields.iter().map(|(_, signature, name)| {
                    let kind = signature.gen_win32_abi(gen);
                    quote! { pub #name: #kind }
                });
                quote! { #(#fields),* }
            };

            quote! {
                #repr
                #[doc(hidden)]
                #[derive(::std::clone::Clone, ::std::marker::Copy)]
                pub #struct_or_union #abi_name{ #fields }
                unsafe impl ::windows::Abi for #name {
                    type Abi = #abi_name;
                }
            }
        };

        let constants = self.0.fields().filter_map(|f| {
            if f.flags().literal() {
                if let Some(constant) = f.constant() {
                    let name = to_ident(f.name());
                    let value = constant.value().gen();

                    return Some(quote! {
                        pub const #name: #value;
                    });
                }
            }

            None
        });

        let compare = if is_union | has_union | has_complex_array {
            quote! {}
        } else {
            let compare = fields
                .iter()
                .enumerate()
                .map(|(index, (_, signature, name))| {
                    let is_callback = matches!(signature.kind, ElementType::Callback(_));

                    if is_callback && signature.pointers == 0 {
                        quote! {
                            self.#name.map(|f| f as usize) == other.#name.map(|f| f as usize)
                        }
                    } else if is_handle {
                        let index = Literal::u32_unsuffixed(index as u32);

                        quote! {
                            self.#index == other.#index
                        }
                    } else {
                        quote! {
                            self.#name == other.#name
                        }
                    }
                });

            if layout.is_some() {
                quote! {
                    impl ::std::cmp::PartialEq for #name {
                        fn eq(&self, other: &Self) -> bool {
                            unsafe { #(#compare)&&* }
                        }
                    }
                    impl ::std::cmp::Eq for #name {}
                }
            } else {
                quote! {
                    impl ::std::cmp::PartialEq for #name {
                        fn eq(&self, other: &Self) -> bool {
                            #(#compare)&&*
                        }
                    }
                    impl ::std::cmp::Eq for #name {}
                }
            }
        };

        let default = if is_union || has_union || has_complex_array {
            quote! {}
        } else {
            let defaults = if is_handle {
                if is_winrt {
                    let defaults = fields
                        .iter()
                        .map(|(_, signature, _)| signature.gen_winrt_default());
                    quote! {
                        Self( #(#defaults),* )
                    }
                } else {
                    let defaults = fields
                        .iter()
                        .map(|(_, signature, _)| signature.gen_win32_default());
                    quote! {
                        Self( #(#defaults),* )
                    }
                }
            } else {
                let defaults = fields.iter().map(|(_, signature, name)| {
                    let value = if is_winrt {
                        signature.gen_winrt_default()
                    } else {
                        signature.gen_win32_default()
                    };
                    quote! {
                        #name: #value
                    }
                });

                quote! {
                    Self{ #(#defaults),* }
                }
            };

            quote! {
                impl ::std::default::Default for #name {
                    fn default() -> Self {
                        #defaults
                    }
                }
            }
        };

        let debug = if is_union || has_union || has_complex_array {
            quote! {}
        } else {
            let debug_name = self.0.name();

            let debug_fields =
                fields
                    .iter()
                    .enumerate()
                    .filter_map(|(index, (_, signature, name))| {
                        // TODO: there must be a simpler way to implement Debug just to exclude this type.
                        match &signature.kind {
                            ElementType::Callback(_) => return None,
                            ElementType::Array((kind, _)) => {
                                if let ElementType::Callback(_) = kind.kind {
                                    return None;
                                }
                            }
                            _ => {}
                        }

                        let field = name.as_str();

                        if is_handle {
                            let index = Literal::u32_unsuffixed(index as u32);

                            Some(quote! {
                                .field(#field, &format_args!("{:?}", self.#index))
                            })
                        } else {
                            Some(quote! {
                                .field(#field, &format_args!("{:?}", self.#name))
                            })
                        }
                    });

            if layout.is_some() {
                quote! {
                    impl ::std::fmt::Debug for #name {
                        fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            unsafe {
                                fmt.debug_struct(#debug_name)
                                    #(#debug_fields)*
                                    .finish()
                            }
                        }
                    }
                }
            } else {
                quote! {
                    impl ::std::fmt::Debug for #name {
                        fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            fmt.debug_struct(#debug_name)
                                #(#debug_fields)*
                                .finish()
                        }
                    }
                }
            }
        };

        let extensions = self.gen_extensions();
        let nested_types = gen_nested_types(struct_name, &self.0, gen);

        let convertible = if let Some(dependency) = self.0.is_convertible() {
            let dependency = dependency.gen_name(gen);

            quote! {
                impl<'a> ::windows::IntoParam<'a, #dependency> for #name {
                    fn into_param(self) -> ::windows::Param<'a, #dependency> {
                        ::windows::Param::Owned(#dependency(self.0))
                    }
                }
            }
        } else {
            quote! {}
        };

        quote! {
            #repr
            #clone_or_copy
            pub #struct_or_union #name #body
            impl #name {
                #(#constants)*
            }
            #default
            #debug
            #compare
            #abi
            #runtime_type
            #extensions
            #nested_types
            #convertible
        }
    }

    fn gen_replacement(&self) -> Option<TokenStream> {
        match self.0.full_name() {
            ("Windows.Win32.SystemServices", "BOOL") => Some(quote! {
                #[repr(C)]
                #[derive(::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::default::Default)]
                pub struct BOOL(pub i32);
                impl BOOL {
                    #[inline]
                    pub fn as_bool(self) -> bool {
                        !(self.0 == 0)
                    }
                    #[inline]
                    pub fn ok(self) -> ::windows::Result<()> {
                        if self.as_bool() {
                            Ok(())
                        } else {
                            Err(::windows::ErrorCode::from_thread().into())
                        }
                    }
                    #[inline]
                    pub fn unwrap(self) {
                        self.ok().unwrap();
                    }
                    #[inline]
                    pub fn expect(self, msg: &str) {
                        self.ok().expect(msg);
                    }
                }
                impl ::std::fmt::Debug for BOOL {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        let msg = if self.as_bool() { "true" } else { "false" };
                        fmt.write_str(msg)
                    }
                }
                unsafe impl ::windows::Abi for BOOL {
                    type Abi = Self;
                }
                impl ::std::convert::From<BOOL> for bool {
                    fn from(value: BOOL) -> Self {
                        value.as_bool()
                    }
                }

                impl ::std::convert::From<&BOOL> for bool {
                    fn from(value: &BOOL) -> Self {
                        value.as_bool()
                    }
                }

                impl ::std::convert::From<bool> for BOOL {
                    fn from(value: bool) -> Self {
                        if value {
                            BOOL(1)
                        } else {
                            BOOL(0)
                        }
                    }
                }

                impl ::std::convert::From<&bool> for BOOL {
                    fn from(value: &bool) -> Self {
                        (*value).into()
                    }
                }



                impl ::std::cmp::PartialEq<bool> for BOOL {
                    fn eq(&self, other: &bool) -> bool {
                        self.as_bool() == *other
                    }
                }

                impl ::std::cmp::PartialEq<BOOL> for bool {
                    fn eq(&self, other: &BOOL) -> bool {
                        *self == other.as_bool()
                    }
                }

                impl std::ops::Not for BOOL {
                    type Output = Self;
                    fn not(self) -> Self::Output {
                        if self.as_bool() {
                            BOOL(0)
                        } else {
                            BOOL(1)
                        }
                    }
                }
                impl<'a> ::windows::IntoParam<'a, BOOL> for bool {
                    fn into_param(self) -> ::windows::Param<'a, BOOL> {
                        ::windows::Param::Owned(self.into())
                    }
                }
            }),

            ("Windows.Win32.SystemServices", "PWSTR") => Some(quote! {
                #[repr(C)]
                #[derive(::std::clone::Clone, ::std::marker::Copy, ::std::cmp::Eq, ::std::fmt::Debug)]
                pub struct PWSTR(pub *mut u16);
                impl ::std::default::Default for PWSTR {
                    fn default() -> Self {
                        Self(::std::ptr::null_mut())
                    }
                }
                // TODO: impl Debug and Display to display value and PartialEq etc
                impl ::std::cmp::PartialEq for PWSTR {
                    fn eq(&self, other: &Self) -> bool {
                        // TODO: do value compare
                        self.0 == other.0
                    }
                }
                unsafe impl ::windows::Abi for PWSTR {
                    type Abi = Self;

                    fn drop_param(param: &mut ::windows::Param<Self>) {
                        if let ::windows::Param::Boxed(value) = param {
                            if !value.0.is_null() {
                                unsafe { ::std::boxed::Box::from_raw(value.0); }
                            }
                        }
                    }
                }
                impl<'a> ::windows::IntoParam<'a, PWSTR> for &'a str {
                    fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                        ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_utf16().chain(::std::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, PWSTR> for String {
                    fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                        // TODO: call variant above
                        ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_utf16().chain(::std::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
                    }
                }
            }),
            ("Windows.Win32.SystemServices", "PSTR") => Some(quote! {
                #[repr(C)]
                #[derive(::std::clone::Clone, ::std::marker::Copy, ::std::cmp::Eq, ::std::fmt::Debug)]
                pub struct PSTR(pub *mut u8);
                impl ::std::default::Default for PSTR {
                    fn default() -> Self {
                        Self(::std::ptr::null_mut())
                    }
                }
                // TODO: impl Debug and Display to display value and PartialEq etc
                impl ::std::cmp::PartialEq for PSTR {
                    fn eq(&self, other: &Self) -> bool {
                        // TODO: do value compare
                        self.0 == other.0
                    }
                }
                unsafe impl ::windows::Abi for PSTR {
                    type Abi = Self;

                    fn drop_param(param: &mut ::windows::Param<Self>) {
                        if let ::windows::Param::Boxed(value) = param {
                            if !value.0.is_null() {
                                unsafe { ::std::boxed::Box::from_raw(value.0); }
                            }
                        }
                    }
                }
                impl<'a> ::windows::IntoParam<'a, PSTR> for &'a str {
                    fn into_param(self) -> ::windows::Param<'a, PSTR> {
                        ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(self.bytes().chain(::std::iter::once(0)).collect::<std::vec::Vec<u8>>().into_boxed_slice()) as _))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, PSTR> for String {
                    fn into_param(self) -> ::windows::Param<'a, PSTR> {
                        // TODO: call variant above
                        ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(self.bytes().chain(::std::iter::once(0)).collect::<std::vec::Vec<u8>>().into_boxed_slice()) as _))
                    }
                }
            }),
            // TODO: This can be an extension rather than replacement?
            ("Windows.Win32.Automation", "BSTR") => Some(quote! {
                #[repr(C)]
                #[derive(::std::clone::Clone, ::std::cmp::Eq)]
                pub struct BSTR(*mut u16);
                impl BSTR {
                    pub fn is_empty(&self) -> bool {
                        self.0.is_null()
                    }
                    fn from_wide(value: &[u16]) -> Self {
                        if value.len() == 0 {
                            return Self(::std::ptr::null_mut());
                        }

                        unsafe { SysAllocStringLen(super::SystemServices::PWSTR(value.as_ptr() as _), value.len() as u32) }
                    }
                    fn as_wide(&self) -> &[u16] {
                        if self.0.is_null() {
                            return &[];
                        }

                        unsafe { ::std::slice::from_raw_parts(self.0 as *const u16, SysStringLen(self) as usize) }
                    }
                }
                impl  std::convert::From<&str> for BSTR {
                    fn from(value: &str) -> Self {
                        let value: ::std::vec::Vec<u16> = value.encode_utf16().collect();
                        Self::from_wide(&value)
                    }
                }

                impl  std::convert::From<::std::string::String> for BSTR {
                    fn from(value: ::std::string::String) -> Self {
                        value.as_str().into()
                    }
                }

                impl  ::std::convert::From<&::std::string::String> for BSTR {
                    fn from(value: &::std::string::String) -> Self {
                        value.as_str().into()
                    }
                }
                impl<'a> ::std::convert::TryFrom<&'a BSTR> for ::std::string::String {
                    type Error = ::std::string::FromUtf16Error;

                    fn try_from(value: &BSTR) -> ::std::result::Result<Self, Self::Error> {
                        ::std::string::String::from_utf16(value.as_wide())
                    }
                }

                impl ::std::convert::TryFrom<BSTR> for ::std::string::String {
                    type Error = ::std::string::FromUtf16Error;

                    fn try_from(value: BSTR) -> ::std::result::Result<Self, Self::Error> {
                        ::std::string::String::try_from(&value)
                    }
                }
                impl ::std::default::Default for BSTR {
                    fn default() -> Self {
                        Self(::std::ptr::null_mut())
                    }
                }
                impl ::std::fmt::Display for BSTR {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        use ::std::fmt::Write;
                        for c in ::std::char::decode_utf16(self.as_wide().iter().cloned()) {
                            f.write_char(c.map_err(|_| ::std::fmt::Error)?)?
                        }
                        Ok(())
                    }
                }
                impl ::std::fmt::Debug for BSTR {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        ::std::write!(f, "{}", self)
                    }
                }
                impl ::std::cmp::PartialEq for BSTR {
                    fn eq(&self, other: &Self) -> bool {
                        self.as_wide() == other.as_wide()
                    }
                }
                impl ::std::cmp::PartialEq<::std::string::String> for BSTR {
                    fn eq(&self, other: &::std::string::String) -> bool {
                        self == other.as_str()
                    }
                }
                impl ::std::cmp::PartialEq<str> for BSTR {
                    fn eq(&self, other: &str) -> bool {
                        self == other
                    }
                }
                impl ::std::cmp::PartialEq<&str> for BSTR {
                    fn eq(&self, other: &&str) -> bool {
                        self.as_wide().iter().copied().eq(other.encode_utf16())
                    }
                }

                impl ::std::cmp::PartialEq<BSTR> for &str {
                    fn eq(&self, other: &BSTR) -> bool {
                        other == self
                    }
                }
                impl ::std::ops::Drop for BSTR {
                    fn drop(&mut self) {
                        if !self.0.is_null() {
                            unsafe { SysFreeString(self as &Self); }
                        }
                    }
                }
                unsafe impl ::windows::Abi for BSTR {
                    type Abi = *mut u16;

                    fn set_abi(&mut self) -> *mut *mut u16 {
                        debug_assert!(self.0.is_null());
                        &mut self.0 as *mut _ as _
                    }
                }
                pub type BSTR_abi = *mut u16;
            }),
            _ => None,
        }
    }

    fn gen_extensions(&self) -> TokenStream {
        match self.0.full_name() {
            ("Windows.Foundation", "TimeSpan") => quote! {
                impl ::std::convert::From<::std::time::Duration> for TimeSpan {
                    fn from(value: ::std::time::Duration) -> Self {
                        Self {
                            Duration: (value.as_nanos() / 100) as i64,
                        }
                    }
                }
                impl ::std::convert::From<TimeSpan> for ::std::time::Duration {
                    fn from(value: TimeSpan) -> Self {
                        ::std::time::Duration::from_nanos((value.Duration * 100) as u64)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, TimeSpan> for ::std::time::Duration {
                    fn into_param(self) -> ::windows::Param<'a, TimeSpan> {
                        ::windows::Param::Owned(self.into())
                    }
                }
            },
            ("Windows.Foundation.Numerics", "Vector2") => quote! {
                impl Vector2 {
                    pub fn new(X: f32, Y: f32) -> Self {
                        Self { X, Y }
                    }
                    pub fn zero() -> Self {
                        Self { X: 0f32, Y: 0f32 }
                    }
                    pub fn one() -> Self {
                        Self { X: 1f32, Y: 1f32 }
                    }
                    pub fn unit_x() -> Self {
                        Self { X: 1.0, Y: 0.0 }
                    }
                    pub fn unit_y() -> Self {
                        Self { X: 0.0, Y: 1.0 }
                    }
                    pub fn dot(&self, rhs: &Self) -> f32 {
                        self.X * rhs.X + self.Y * rhs.Y
                    }
                    pub fn length_squared(&self) -> f32 {
                        self.dot(self)
                    }
                    pub fn length(&self) -> f32 {
                        self.length_squared().sqrt()
                    }
                    pub fn distance(&self, value: &Self) -> f32 {
                        (self - value).length()
                    }
                    pub fn distance_squared(&self, value: &Self) -> f32 {
                        (self - value).length_squared()
                    }
                    pub fn normalize(&self) -> Self {
                        self / self.length()
                    }

                    fn impl_add(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X + rhs.X,
                            Y: self.Y + rhs.Y,
                        }
                    }
                    fn impl_sub(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X - rhs.X,
                            Y: self.Y - rhs.Y,
                        }
                    }
                    fn impl_div(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X / rhs.X,
                            Y: self.Y / rhs.Y,
                        }
                    }
                    fn impl_div_f32(&self, rhs: f32) -> Self {
                        Self {
                            X: self.X / rhs,
                            Y: self.Y / rhs,
                        }
                    }
                    fn impl_mul(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X * rhs.X,
                            Y: self.Y * rhs.Y,
                        }
                    }
                    fn impl_mul_f32(&self, rhs: f32) -> Self {
                        Self {
                            X: self.X * rhs,
                            Y: self.Y * rhs,
                        }
                    }
                }

                impl ::std::ops::Add<Vector2> for Vector2 {
                    type Output = Vector2;
                    fn add(self, rhs: Vector2) -> Vector2 {
                        self.impl_add(&rhs)
                    }
                }
                impl ::std::ops::Add<&Vector2> for Vector2 {
                    type Output = Vector2;
                    fn add(self, rhs: &Vector2) -> Vector2 {
                        self.impl_add(rhs)
                    }
                }
                impl ::std::ops::Add<Vector2> for &Vector2 {
                    type Output = Vector2;
                    fn add(self, rhs: Vector2) -> Vector2 {
                        self.impl_add(&rhs)
                    }
                }
                impl ::std::ops::Add<&Vector2> for &Vector2 {
                    type Output = Vector2;
                    fn add(self, rhs: &Vector2) -> Vector2 {
                        self.impl_add(rhs)
                    }
                }
                impl ::std::ops::Sub<Vector2> for Vector2 {
                    type Output = Vector2;
                    fn sub(self, rhs: Vector2) -> Vector2 {
                        self.impl_sub(&rhs)
                    }
                }
                impl ::std::ops::Sub<&Vector2> for Vector2 {
                    type Output = Vector2;
                    fn sub(self, rhs: &Vector2) -> Vector2 {
                        self.impl_sub(rhs)
                    }
                }
                impl ::std::ops::Sub<Vector2> for &Vector2 {
                    type Output = Vector2;
                    fn sub(self, rhs: Vector2) -> Vector2 {
                        self.impl_sub(&rhs)
                    }
                }
                impl ::std::ops::Sub<&Vector2> for &Vector2 {
                    type Output = Vector2;
                    fn sub(self, rhs: &Vector2) -> Vector2 {
                        self.impl_sub(rhs)
                    }
                }
                impl ::std::ops::Div<Vector2> for Vector2 {
                    type Output = Vector2;
                    fn div(self, rhs: Vector2) -> Vector2 {
                        self.impl_div(&rhs)
                    }
                }
                impl ::std::ops::Div<&Vector2> for Vector2 {
                    type Output = Vector2;
                    fn div(self, rhs: &Vector2) -> Vector2 {
                        self.impl_div(rhs)
                    }
                }
                impl ::std::ops::Div<Vector2> for &Vector2 {
                    type Output = Vector2;
                    fn div(self, rhs: Vector2) -> Vector2 {
                        self.impl_div(&rhs)
                    }
                }
                impl ::std::ops::Div<&Vector2> for &Vector2 {
                    type Output = Vector2;
                    fn div(self, rhs: &Vector2) -> Vector2 {
                        self.impl_div(rhs)
                    }
                }
                impl ::std::ops::Div<f32> for Vector2 {
                    type Output = Vector2;
                    fn div(self, rhs: f32) -> Vector2 {
                        self.impl_div_f32(rhs)
                    }
                }
                impl ::std::ops::Div<f32> for &Vector2 {
                    type Output = Vector2;
                    fn div(self, rhs: f32) -> Vector2 {
                        self.impl_div_f32(rhs)
                    }
                }
                impl ::std::ops::Mul<Vector2> for Vector2 {
                    type Output = Vector2;
                    fn mul(self, rhs: Vector2) -> Vector2 {
                        self.impl_mul(&rhs)
                    }
                }
                impl ::std::ops::Mul<&Vector2> for Vector2 {
                    type Output = Vector2;
                    fn mul(self, rhs: &Vector2) -> Vector2 {
                        self.impl_mul(rhs)
                    }
                }
                impl ::std::ops::Mul<Vector2> for &Vector2 {
                    type Output = Vector2;
                    fn mul(self, rhs: Vector2) -> Vector2 {
                        self.impl_mul(&rhs)
                    }
                }
                impl ::std::ops::Mul<&Vector2> for &Vector2 {
                    type Output = Vector2;
                    fn mul(self, rhs: &Vector2) -> Vector2 {
                        self.impl_mul(rhs)
                    }
                }
                impl ::std::ops::Mul<f32> for Vector2 {
                    type Output = Vector2;
                    fn mul(self, rhs: f32) -> Vector2 {
                        self.impl_mul_f32(rhs)
                    }
                }
                impl ::std::ops::Mul<f32> for &Vector2 {
                    type Output = Vector2;
                    fn mul(self, rhs: f32) -> Vector2 {
                        self.impl_mul_f32(rhs)
                    }
                }
            },
            ("Windows.Foundation.Numerics", "Vector3") => quote! {
                impl Vector3 {
                    pub fn new(X: f32, Y: f32, Z: f32) -> Self {
                        Self { X, Y, Z }
                    }
                    pub fn zero() -> Self {
                        Self {
                            X: 0f32,
                            Y: 0f32,
                            Z: 0f32,
                        }
                    }
                    pub fn one() -> Self {
                        Self {
                            X: 1f32,
                            Y: 1f32,
                            Z: 1f32,
                        }
                    }
                    pub fn unit_x() -> Self {
                        Self {
                            X: 1.0,
                            Y: 0.0,
                            Z: 0.0,
                        }
                    }
                    pub fn unit_y() -> Self {
                        Self {
                            X: 0.0,
                            Y: 1.0,
                            Z: 0.0,
                        }
                    }
                    pub fn unit_z() -> Self {
                        Self {
                            X: 0.0,
                            Y: 0.0,
                            Z: 1.0,
                        }
                    }
                    pub fn dot(&self, rhs: &Self) -> f32 {
                        self.X * rhs.X + self.Y * rhs.Y + self.Z * rhs.Z
                    }
                    pub fn length_squared(&self) -> f32 {
                        self.dot(self)
                    }
                    pub fn length(&self) -> f32 {
                        self.length_squared().sqrt()
                    }
                    pub fn distance(&self, value: &Self) -> f32 {
                        (self - value).length()
                    }
                    pub fn distance_squared(&self, value: &Self) -> f32 {
                        (self - value).length_squared()
                    }
                    pub fn normalize(&self) -> Self {
                        self / self.length()
                    }

                    fn impl_add(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X + rhs.X,
                            Y: self.Y + rhs.Y,
                            Z: self.Z + rhs.Z,
                        }
                    }
                    fn impl_sub(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X - rhs.X,
                            Y: self.Y - rhs.Y,
                            Z: self.Z - rhs.Z,
                        }
                    }
                    fn impl_div(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X / rhs.X,
                            Y: self.Y / rhs.Y,
                            Z: self.Z / rhs.Z,
                        }
                    }
                    fn impl_div_f32(&self, rhs: f32) -> Self {
                        Self {
                            X: self.X / rhs,
                            Y: self.Y / rhs,
                            Z: self.Z / rhs,
                        }
                    }
                    fn impl_mul(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X * rhs.X,
                            Y: self.Y * rhs.Y,
                            Z: self.Z * rhs.Z,
                        }
                    }
                    fn impl_mul_f32(&self, rhs: f32) -> Self {
                        Self {
                            X: self.X * rhs,
                            Y: self.Y * rhs,
                            Z: self.Z * rhs,
                        }
                    }
                }

                impl ::std::ops::Add<Vector3> for Vector3 {
                    type Output = Vector3;
                    fn add(self, rhs: Vector3) -> Vector3 {
                        self.impl_add(&rhs)
                    }
                }
                impl ::std::ops::Add<&Vector3> for Vector3 {
                    type Output = Vector3;
                    fn add(self, rhs: &Vector3) -> Vector3 {
                        self.impl_add(rhs)
                    }
                }
                impl ::std::ops::Add<Vector3> for &Vector3 {
                    type Output = Vector3;
                    fn add(self, rhs: Vector3) -> Vector3 {
                        self.impl_add(&rhs)
                    }
                }
                impl ::std::ops::Add<&Vector3> for &Vector3 {
                    type Output = Vector3;
                    fn add(self, rhs: &Vector3) -> Vector3 {
                        self.impl_add(rhs)
                    }
                }
                impl ::std::ops::Sub<Vector3> for Vector3 {
                    type Output = Vector3;
                    fn sub(self, rhs: Vector3) -> Vector3 {
                        self.impl_sub(&rhs)
                    }
                }
                impl ::std::ops::Sub<&Vector3> for Vector3 {
                    type Output = Vector3;
                    fn sub(self, rhs: &Vector3) -> Vector3 {
                        self.impl_sub(rhs)
                    }
                }
                impl ::std::ops::Sub<Vector3> for &Vector3 {
                    type Output = Vector3;
                    fn sub(self, rhs: Vector3) -> Vector3 {
                        self.impl_sub(&rhs)
                    }
                }
                impl ::std::ops::Sub<&Vector3> for &Vector3 {
                    type Output = Vector3;
                    fn sub(self, rhs: &Vector3) -> Vector3 {
                        self.impl_sub(rhs)
                    }
                }
                impl ::std::ops::Div<Vector3> for Vector3 {
                    type Output = Vector3;
                    fn div(self, rhs: Vector3) -> Vector3 {
                        self.impl_div(&rhs)
                    }
                }
                impl ::std::ops::Div<&Vector3> for Vector3 {
                    type Output = Vector3;
                    fn div(self, rhs: &Vector3) -> Vector3 {
                        self.impl_div(rhs)
                    }
                }
                impl ::std::ops::Div<Vector3> for &Vector3 {
                    type Output = Vector3;
                    fn div(self, rhs: Vector3) -> Vector3 {
                        self.impl_div(&rhs)
                    }
                }
                impl ::std::ops::Div<&Vector3> for &Vector3 {
                    type Output = Vector3;
                    fn div(self, rhs: &Vector3) -> Vector3 {
                        self.impl_div(rhs)
                    }
                }
                impl ::std::ops::Div<f32> for Vector3 {
                    type Output = Vector3;
                    fn div(self, rhs: f32) -> Vector3 {
                        self.impl_div_f32(rhs)
                    }
                }
                impl ::std::ops::Div<f32> for &Vector3 {
                    type Output = Vector3;
                    fn div(self, rhs: f32) -> Vector3 {
                        self.impl_div_f32(rhs)
                    }
                }
                impl ::std::ops::Mul<Vector3> for Vector3 {
                    type Output = Vector3;
                    fn mul(self, rhs: Vector3) -> Vector3 {
                        self.impl_mul(&rhs)
                    }
                }
                impl ::std::ops::Mul<&Vector3> for Vector3 {
                    type Output = Vector3;
                    fn mul(self, rhs: &Vector3) -> Vector3 {
                        self.impl_mul(rhs)
                    }
                }
                impl ::std::ops::Mul<Vector3> for &Vector3 {
                    type Output = Vector3;
                    fn mul(self, rhs: Vector3) -> Vector3 {
                        self.impl_mul(&rhs)
                    }
                }
                impl ::std::ops::Mul<&Vector3> for &Vector3 {
                    type Output = Vector3;
                    fn mul(self, rhs: &Vector3) -> Vector3 {
                        self.impl_mul(rhs)
                    }
                }
                impl ::std::ops::Mul<f32> for Vector3 {
                    type Output = Vector3;
                    fn mul(self, rhs: f32) -> Vector3 {
                        self.impl_mul_f32(rhs)
                    }
                }
                impl ::std::ops::Mul<f32> for &Vector3 {
                    type Output = Vector3;
                    fn mul(self, rhs: f32) -> Vector3 {
                        self.impl_mul_f32(rhs)
                    }
                }
            },
            ("Windows.Foundation.Numerics", "Vector4") => quote! {
                impl Vector4 {
                    pub fn new(X: f32, Y: f32, Z: f32, W: f32) -> Self {
                        Self { X, Y, Z, W }
                    }
                    pub fn zero() -> Self {
                        Self {
                            X: 0f32,
                            Y: 0f32,
                            Z: 0f32,
                            W: 0f32,
                        }
                    }
                    pub fn one() -> Self {
                        Self {
                            X: 1f32,
                            Y: 1f32,
                            Z: 1f32,
                            W: 1f32,
                        }
                    }
                    pub fn unit_x() -> Self {
                        Self {
                            X: 1.0,
                            Y: 0.0,
                            Z: 0.0,
                            W: 0.0,
                        }
                    }
                    pub fn unit_y() -> Self {
                        Self {
                            X: 0.0,
                            Y: 1.0,
                            Z: 0.0,
                            W: 0.0,
                        }
                    }
                    pub fn unit_z() -> Self {
                        Self {
                            X: 0.0,
                            Y: 0.0,
                            Z: 1.0,
                            W: 0.0,
                        }
                    }
                    pub fn unit_w() -> Self {
                        Self {
                            X: 0.0,
                            Y: 0.0,
                            Z: 0.0,
                            W: 1.0,
                        }
                    }
                    pub fn dot(&self, rhs: &Self) -> f32 {
                        self.X * rhs.X + self.Y * rhs.Y + self.Z * rhs.Z + self.W * rhs.W
                    }
                    pub fn length_squared(&self) -> f32 {
                        self.dot(self)
                    }
                    pub fn length(&self) -> f32 {
                        self.length_squared().sqrt()
                    }
                    pub fn distance(&self, value: &Self) -> f32 {
                        (self - value).length()
                    }
                    pub fn distance_squared(&self, value: &Self) -> f32 {
                        (self - value).length_squared()
                    }
                    pub fn normalize(&self) -> Self {
                        self / self.length()
                    }

                    fn impl_add(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X + rhs.X,
                            Y: self.Y + rhs.Y,
                            Z: self.Z + rhs.Z,
                            W: self.W + rhs.W,
                        }
                    }
                    fn impl_sub(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X - rhs.X,
                            Y: self.Y - rhs.Y,
                            Z: self.Z - rhs.Z,
                            W: self.W - rhs.W,
                        }
                    }
                    fn impl_div(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X / rhs.X,
                            Y: self.Y / rhs.Y,
                            Z: self.Z / rhs.Z,
                            W: self.W / rhs.W,
                        }
                    }
                    fn impl_div_f32(&self, rhs: f32) -> Self {
                        Self {
                            X: self.X / rhs,
                            Y: self.Y / rhs,
                            Z: self.Z / rhs,
                            W: self.W / rhs,
                        }
                    }
                    fn impl_mul(&self, rhs: &Self) -> Self {
                        Self {
                            X: self.X * rhs.X,
                            Y: self.Y * rhs.Y,
                            Z: self.Z * rhs.Z,
                            W: self.W * rhs.W,
                        }
                    }
                    fn impl_mul_f32(&self, rhs: f32) -> Self {
                        Self {
                            X: self.X * rhs,
                            Y: self.Y * rhs,
                            Z: self.Z * rhs,
                            W: self.W * rhs,
                        }
                    }
                }

                impl ::std::ops::Add<Vector4> for Vector4 {
                    type Output = Vector4;
                    fn add(self, rhs: Vector4) -> Vector4 {
                        self.impl_add(&rhs)
                    }
                }
                impl ::std::ops::Add<&Vector4> for Vector4 {
                    type Output = Vector4;
                    fn add(self, rhs: &Vector4) -> Vector4 {
                        self.impl_add(rhs)
                    }
                }
                impl ::std::ops::Add<Vector4> for &Vector4 {
                    type Output = Vector4;
                    fn add(self, rhs: Vector4) -> Vector4 {
                        self.impl_add(&rhs)
                    }
                }
                impl ::std::ops::Add<&Vector4> for &Vector4 {
                    type Output = Vector4;
                    fn add(self, rhs: &Vector4) -> Vector4 {
                        self.impl_add(rhs)
                    }
                }
                impl ::std::ops::Sub<Vector4> for Vector4 {
                    type Output = Vector4;
                    fn sub(self, rhs: Vector4) -> Vector4 {
                        self.impl_sub(&rhs)
                    }
                }
                impl ::std::ops::Sub<&Vector4> for Vector4 {
                    type Output = Vector4;
                    fn sub(self, rhs: &Vector4) -> Vector4 {
                        self.impl_sub(rhs)
                    }
                }
                impl ::std::ops::Sub<Vector4> for &Vector4 {
                    type Output = Vector4;
                    fn sub(self, rhs: Vector4) -> Vector4 {
                        self.impl_sub(&rhs)
                    }
                }
                impl ::std::ops::Sub<&Vector4> for &Vector4 {
                    type Output = Vector4;
                    fn sub(self, rhs: &Vector4) -> Vector4 {
                        self.impl_sub(rhs)
                    }
                }
                impl ::std::ops::Div<Vector4> for Vector4 {
                    type Output = Vector4;
                    fn div(self, rhs: Vector4) -> Vector4 {
                        self.impl_div(&rhs)
                    }
                }
                impl ::std::ops::Div<&Vector4> for Vector4 {
                    type Output = Vector4;
                    fn div(self, rhs: &Vector4) -> Vector4 {
                        self.impl_div(rhs)
                    }
                }
                impl ::std::ops::Div<Vector4> for &Vector4 {
                    type Output = Vector4;
                    fn div(self, rhs: Vector4) -> Vector4 {
                        self.impl_div(&rhs)
                    }
                }
                impl ::std::ops::Div<&Vector4> for &Vector4 {
                    type Output = Vector4;
                    fn div(self, rhs: &Vector4) -> Vector4 {
                        self.impl_div(rhs)
                    }
                }
                impl ::std::ops::Div<f32> for Vector4 {
                    type Output = Vector4;
                    fn div(self, rhs: f32) -> Vector4 {
                        self.impl_div_f32(rhs)
                    }
                }
                impl ::std::ops::Div<f32> for &Vector4 {
                    type Output = Vector4;
                    fn div(self, rhs: f32) -> Vector4 {
                        self.impl_div_f32(rhs)
                    }
                }
                impl ::std::ops::Mul<Vector4> for Vector4 {
                    type Output = Vector4;
                    fn mul(self, rhs: Vector4) -> Vector4 {
                        self.impl_mul(&rhs)
                    }
                }
                impl ::std::ops::Mul<&Vector4> for Vector4 {
                    type Output = Vector4;
                    fn mul(self, rhs: &Vector4) -> Vector4 {
                        self.impl_mul(rhs)
                    }
                }
                impl ::std::ops::Mul<Vector4> for &Vector4 {
                    type Output = Vector4;
                    fn mul(self, rhs: Vector4) -> Vector4 {
                        self.impl_mul(&rhs)
                    }
                }
                impl ::std::ops::Mul<&Vector4> for &Vector4 {
                    type Output = Vector4;
                    fn mul(self, rhs: &Vector4) -> Vector4 {
                        self.impl_mul(rhs)
                    }
                }
                impl ::std::ops::Mul<f32> for Vector4 {
                    type Output = Vector4;
                    fn mul(self, rhs: f32) -> Vector4 {
                        self.impl_mul_f32(rhs)
                    }
                }
                impl ::std::ops::Mul<f32> for &Vector4 {
                    type Output = Vector4;
                    fn mul(self, rhs: f32) -> Vector4 {
                        self.impl_mul_f32(rhs)
                    }
                }
            },
            ("Windows.Foundation.Numerics", "Matrix3x2") => quote! {
                impl Matrix3x2 {
                    pub fn identity() -> Self {
                        Self {
                            M11: 1.0,
                            M12: 0.0,
                            M21: 0.0,
                            M22: 1.0,
                            M31: 0.0,
                            M32: 0.0,
                        }
                    }
                    pub fn translation(x: f32, y: f32) -> Self {
                        Self {
                            M11: 1.0,
                            M12: 0.0,
                            M21: 0.0,
                            M22: 1.0,
                            M31: x,
                            M32: y,
                        }
                    }
                    pub fn rotation(angle: f32, x: f32, y: f32) -> Self {
                        let mut matrix = Self::default();
                        unsafe {
                            super::super::Win32::Direct2D::D2D1MakeRotateMatrix(angle, super::super::Win32::Direct2D::D2D_POINT_2F{x, y}, &mut matrix);
                        }
                        matrix
                    }
                    fn impl_add(&self, rhs: &Self) -> Self {
                        Self {
                            M11: self.M11 + rhs.M11,
                            M12: self.M12 + rhs.M12,
                            M21: self.M21 + rhs.M21,
                            M22: self.M22 + rhs.M22,
                            M31: self.M31 + rhs.M31,
                            M32: self.M32 + rhs.M32,
                        }
                    }
                    fn impl_sub(&self, rhs: &Self) -> Self {
                        Self {
                            M11: self.M11 - rhs.M11,
                            M12: self.M12 - rhs.M12,
                            M21: self.M21 - rhs.M21,
                            M22: self.M22 - rhs.M22,
                            M31: self.M31 - rhs.M31,
                            M32: self.M32 - rhs.M32,
                        }
                    }
                    fn impl_mul(&self, rhs: &Self) -> Self {
                        Self {
                            M11: self.M11 * rhs.M11 + self.M12 * rhs.M21,
                            M12: self.M11 * rhs.M12 + self.M12 * rhs.M22,
                            M21: self.M21 * rhs.M11 + self.M22 * rhs.M21,
                            M22: self.M21 * rhs.M12 + self.M22 * rhs.M22,
                            M31: self.M31 * rhs.M11 + self.M32 * rhs.M21 + rhs.M31,
                            M32: self.M31 * rhs.M12 + self.M32 * rhs.M22 + rhs.M32,
                        }
                    }
                    fn impl_mul_f32(&self, rhs: f32) -> Self {
                        Self {
                            M11: self.M11 * rhs,
                            M12: self.M12 * rhs,
                            M21: self.M21 * rhs,
                            M22: self.M22 * rhs,
                            M31: self.M31 * rhs,
                            M32: self.M32 * rhs,
                        }
                    }
                }

                impl ::std::ops::Add<Matrix3x2> for Matrix3x2 {
                    type Output = Matrix3x2;
                    fn add(self, rhs: Matrix3x2) -> Matrix3x2 {
                        self.impl_add(&rhs)
                    }
                }
                impl ::std::ops::Add<&Matrix3x2> for Matrix3x2 {
                    type Output = Matrix3x2;
                    fn add(self, rhs: &Matrix3x2) -> Matrix3x2 {
                        self.impl_add(rhs)
                    }
                }
                impl ::std::ops::Add<Matrix3x2> for &Matrix3x2 {
                    type Output = Matrix3x2;
                    fn add(self, rhs: Matrix3x2) -> Matrix3x2 {
                        self.impl_add(&rhs)
                    }
                }
                impl ::std::ops::Add<&Matrix3x2> for &Matrix3x2 {
                    type Output = Matrix3x2;
                    fn add(self, rhs: &Matrix3x2) -> Matrix3x2 {
                        self.impl_add(rhs)
                    }
                }
                impl ::std::ops::Sub<Matrix3x2> for Matrix3x2 {
                    type Output = Matrix3x2;
                    fn sub(self, rhs: Matrix3x2) -> Matrix3x2 {
                        self.impl_sub(&rhs)
                    }
                }
                impl ::std::ops::Sub<&Matrix3x2> for Matrix3x2 {
                    type Output = Matrix3x2;
                    fn sub(self, rhs: &Matrix3x2) -> Matrix3x2 {
                        self.impl_sub(rhs)
                    }
                }
                impl ::std::ops::Sub<Matrix3x2> for &Matrix3x2 {
                    type Output = Matrix3x2;
                    fn sub(self, rhs: Matrix3x2) -> Matrix3x2 {
                        self.impl_sub(&rhs)
                    }
                }
                impl ::std::ops::Sub<&Matrix3x2> for &Matrix3x2 {
                    type Output = Matrix3x2;
                    fn sub(self, rhs: &Matrix3x2) -> Matrix3x2 {
                        self.impl_sub(rhs)
                    }
                }
                impl ::std::ops::Mul<Matrix3x2> for Matrix3x2 {
                    type Output = Matrix3x2;
                    fn mul(self, rhs: Matrix3x2) -> Matrix3x2 {
                        self.impl_mul(&rhs)
                    }
                }
                impl ::std::ops::Mul<&Matrix3x2> for Matrix3x2 {
                    type Output = Matrix3x2;
                    fn mul(self, rhs: &Matrix3x2) -> Matrix3x2 {
                        self.impl_mul(rhs)
                    }
                }
                impl ::std::ops::Mul<Matrix3x2> for &Matrix3x2 {
                    type Output = Matrix3x2;
                    fn mul(self, rhs: Matrix3x2) -> Matrix3x2 {
                        self.impl_mul(&rhs)
                    }
                }
                impl ::std::ops::Mul<&Matrix3x2> for &Matrix3x2 {
                    type Output = Matrix3x2;
                    fn mul(self, rhs: &Matrix3x2) -> Matrix3x2 {
                        self.impl_mul(rhs)
                    }
                }
                impl ::std::ops::Mul<f32> for Matrix3x2 {
                    type Output = Matrix3x2;
                    fn mul(self, rhs: f32) -> Matrix3x2 {
                        self.impl_mul_f32(rhs)
                    }
                }
                impl ::std::ops::Mul<f32> for &Matrix3x2 {
                    type Output = Matrix3x2;
                    fn mul(self, rhs: f32) -> Matrix3x2 {
                        self.impl_mul_f32(rhs)
                    }
                }
            },
            ("Windows.Foundation.Numerics", "Matrix4x4") => quote! {
                impl Matrix4x4 {
                    fn impl_add(&self, rhs: &Self) -> Self {
                        Self {
                            M11: self.M11 + rhs.M11,
                            M12: self.M12 + rhs.M12,
                            M13: self.M13 + rhs.M13,
                            M14: self.M14 + rhs.M14,
                            M21: self.M21 + rhs.M21,
                            M22: self.M22 + rhs.M22,
                            M23: self.M23 + rhs.M23,
                            M24: self.M24 + rhs.M24,
                            M31: self.M31 + rhs.M31,
                            M32: self.M32 + rhs.M32,
                            M33: self.M33 + rhs.M33,
                            M34: self.M34 + rhs.M34,
                            M41: self.M41 + rhs.M41,
                            M42: self.M42 + rhs.M42,
                            M43: self.M43 + rhs.M43,
                            M44: self.M44 + rhs.M44,
                        }
                    }
                    fn impl_sub(&self, rhs: &Self) -> Self {
                        Self {
                            M11: self.M11 - rhs.M11,
                            M12: self.M12 - rhs.M12,
                            M13: self.M13 - rhs.M13,
                            M14: self.M14 - rhs.M14,
                            M21: self.M21 - rhs.M21,
                            M22: self.M22 - rhs.M22,
                            M23: self.M23 - rhs.M23,
                            M24: self.M24 - rhs.M24,
                            M31: self.M31 - rhs.M31,
                            M32: self.M32 - rhs.M32,
                            M33: self.M33 - rhs.M33,
                            M34: self.M34 - rhs.M34,
                            M41: self.M41 - rhs.M41,
                            M42: self.M42 - rhs.M42,
                            M43: self.M43 - rhs.M43,
                            M44: self.M44 - rhs.M44,
                        }
                    }
                    fn impl_mul(&self, rhs: &Self) -> Self {
                        Self {
                            M11: self.M11 * rhs.M11 + self.M12 * rhs.M21 + self.M13 * rhs.M31 + self.M14 * rhs.M41,
                            M12: self.M11 * rhs.M12 + self.M12 * rhs.M22 + self.M13 * rhs.M32 + self.M14 * rhs.M42,
                            M13: self.M11 * rhs.M13 + self.M12 * rhs.M23 + self.M13 * rhs.M33 + self.M14 * rhs.M43,
                            M14: self.M11 * rhs.M14 + self.M12 * rhs.M24 + self.M13 * rhs.M34 + self.M14 * rhs.M44,
                            M21: self.M21 * rhs.M11 + self.M22 * rhs.M21 + self.M23 * rhs.M31 + self.M24 * rhs.M41,
                            M22: self.M21 * rhs.M12 + self.M22 * rhs.M22 + self.M23 * rhs.M32 + self.M24 * rhs.M42,
                            M23: self.M21 * rhs.M13 + self.M22 * rhs.M23 + self.M23 * rhs.M33 + self.M24 * rhs.M43,
                            M24: self.M21 * rhs.M14 + self.M22 * rhs.M24 + self.M23 * rhs.M34 + self.M24 * rhs.M44,
                            M31: self.M31 * rhs.M11 + self.M32 * rhs.M21 + self.M33 * rhs.M31 + self.M34 * rhs.M41,
                            M32: self.M31 * rhs.M12 + self.M32 * rhs.M22 + self.M33 * rhs.M32 + self.M34 * rhs.M42,
                            M33: self.M31 * rhs.M13 + self.M32 * rhs.M23 + self.M33 * rhs.M33 + self.M34 * rhs.M43,
                            M34: self.M31 * rhs.M14 + self.M32 * rhs.M24 + self.M33 * rhs.M34 + self.M34 * rhs.M44,
                            M41: self.M41 * rhs.M11 + self.M42 * rhs.M21 + self.M43 * rhs.M31 + self.M44 * rhs.M41,
                            M42: self.M41 * rhs.M12 + self.M42 * rhs.M22 + self.M43 * rhs.M32 + self.M44 * rhs.M42,
                            M43: self.M41 * rhs.M13 + self.M42 * rhs.M23 + self.M43 * rhs.M33 + self.M44 * rhs.M43,
                            M44: self.M41 * rhs.M14 + self.M42 * rhs.M24 + self.M43 * rhs.M34 + self.M44 * rhs.M44,
                        }
                    }
                    fn impl_mul_f32(&self, rhs: f32) -> Self {
                        Self {
                            M11: self.M11 * rhs,
                            M12: self.M12 * rhs,
                            M13: self.M13 * rhs,
                            M14: self.M14 * rhs,
                            M21: self.M21 * rhs,
                            M22: self.M22 * rhs,
                            M23: self.M23 * rhs,
                            M24: self.M24 * rhs,
                            M31: self.M31 * rhs,
                            M32: self.M32 * rhs,
                            M33: self.M33 * rhs,
                            M34: self.M34 * rhs,
                            M41: self.M41 * rhs,
                            M42: self.M42 * rhs,
                            M43: self.M43 * rhs,
                            M44: self.M44 * rhs,
                        }
                    }
                }

                impl ::std::ops::Add<Matrix4x4> for Matrix4x4 {
                    type Output = Matrix4x4;
                    fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
                        self.impl_add(&rhs)
                    }
                }
                impl ::std::ops::Add<&Matrix4x4> for Matrix4x4 {
                    type Output = Matrix4x4;
                    fn add(self, rhs: &Matrix4x4) -> Matrix4x4 {
                        self.impl_add(rhs)
                    }
                }
                impl ::std::ops::Add<Matrix4x4> for &Matrix4x4 {
                    type Output = Matrix4x4;
                    fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
                        self.impl_add(&rhs)
                    }
                }
                impl ::std::ops::Add<&Matrix4x4> for &Matrix4x4 {
                    type Output = Matrix4x4;
                    fn add(self, rhs: &Matrix4x4) -> Matrix4x4 {
                        self.impl_add(rhs)
                    }
                }
                impl ::std::ops::Sub<Matrix4x4> for Matrix4x4 {
                    type Output = Matrix4x4;
                    fn sub(self, rhs: Matrix4x4) -> Matrix4x4 {
                        self.impl_sub(&rhs)
                    }
                }
                impl ::std::ops::Sub<&Matrix4x4> for Matrix4x4 {
                    type Output = Matrix4x4;
                    fn sub(self, rhs: &Matrix4x4) -> Matrix4x4 {
                        self.impl_sub(rhs)
                    }
                }
                impl ::std::ops::Sub<Matrix4x4> for &Matrix4x4 {
                    type Output = Matrix4x4;
                    fn sub(self, rhs: Matrix4x4) -> Matrix4x4 {
                        self.impl_sub(&rhs)
                    }
                }
                impl ::std::ops::Sub<&Matrix4x4> for &Matrix4x4 {
                    type Output = Matrix4x4;
                    fn sub(self, rhs: &Matrix4x4) -> Matrix4x4 {
                        self.impl_sub(rhs)
                    }
                }
                impl ::std::ops::Mul<Matrix4x4> for Matrix4x4 {
                    type Output = Matrix4x4;
                    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
                        self.impl_mul(&rhs)
                    }
                }
                impl ::std::ops::Mul<&Matrix4x4> for Matrix4x4 {
                    type Output = Matrix4x4;
                    fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
                        self.impl_mul(rhs)
                    }
                }
                impl ::std::ops::Mul<Matrix4x4> for &Matrix4x4 {
                    type Output = Matrix4x4;
                    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
                        self.impl_mul(&rhs)
                    }
                }
                impl ::std::ops::Mul<&Matrix4x4> for &Matrix4x4 {
                    type Output = Matrix4x4;
                    fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
                        self.impl_mul(rhs)
                    }
                }
                impl ::std::ops::Mul<f32> for Matrix4x4 {
                    type Output = Matrix4x4;
                    fn mul(self, rhs: f32) -> Matrix4x4 {
                        self.impl_mul_f32(rhs)
                    }
                }
                impl ::std::ops::Mul<f32> for &Matrix4x4 {
                    type Output = Matrix4x4;
                    fn mul(self, rhs: f32) -> Matrix4x4 {
                        self.impl_mul_f32(rhs)
                    }
                }
            },
            _ => TokenStream::new(),
        }
    }
}

fn gen_nested_types<'a>(
    enclosing_name: &'a str,
    enclosing_type: &'a tables::TypeDef,
    gen: &Gen,
) -> TokenStream {
    enclosing_type
        .nested_types()
        .iter()
        .enumerate()
        .map(|(index, nested_type)| {
            let nested_name = format!("{}_{}", enclosing_name, index);
            Struct(*nested_type).gen_struct(&nested_name, gen)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let t = TypeReader::get_struct("Windows.Foundation", "Point");
        assert_eq!(t.type_signature(), "struct(Windows.Foundation.Point;f4;f4)");
    }

    #[test]
    fn test_fields() {
        let t = TypeReader::get_struct("Windows.Win32.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA");
        let f: Vec<tables::Field> = t.0.fields().collect();
        assert_eq!(f.len(), 7);

        assert_eq!(f[0].name(), "PresentCount");
        assert_eq!(f[1].name(), "PresentRefreshCount");
        assert_eq!(f[2].name(), "SyncRefreshCount");
        assert_eq!(f[3].name(), "SyncQPCTime");
        assert_eq!(f[4].name(), "SyncGPUTime");
        assert_eq!(f[5].name(), "CompositionMode");
        assert_eq!(f[6].name(), "ApprovedPresentDuration");

        assert_eq!(f[0].signature().kind, ElementType::U32);
        assert_eq!(f[1].signature().kind, ElementType::U32);
        assert_eq!(f[2].signature().kind, ElementType::U32);
        assert_eq!(f[3].signature().kind, ElementType::I64);
        assert_eq!(f[4].signature().kind, ElementType::I64);
        assert_eq!(
            f[5].signature().kind,
            ElementType::Enum(TypeReader::get_enum(
                "Windows.Win32.Dxgi",
                "DXGI_FRAME_PRESENTATION_MODE"
            ))
        );
        assert_eq!(f[6].signature().kind, ElementType::U32);
    }

    #[test]
    fn test_dependencies() {
        let t = TypeReader::get_struct("Windows.Foundation", "Point");
        assert_eq!(t.dependencies().len(), 0);

        let t = TypeReader::get_struct("Windows.Win32.Dxgi", "DXGI_FRAME_STATISTICS");
        assert_eq!(t.dependencies().len(), 0);

        let t = TypeReader::get_struct("Windows.Win32.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA");
        let deps = t.dependencies();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name(), "DXGI_FRAME_PRESENTATION_MODE");
    }

    #[test]
    fn test_blittable() {
        assert_eq!(
            TypeReader::get_struct("Windows.Foundation", "Point").is_blittable(),
            true
        );
        assert_eq!(
            TypeReader::get_struct("Windows.UI.Xaml.Interop", "TypeName").is_blittable(),
            false
        );
    }
}
