use super::*;

// TODO: need to split win32 and winrt structs as their signatures are interpreted differently

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

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.0.fields().map(|f| f.definition()).flatten().collect()
    }

    pub fn definition(&self) -> Vec<tables::TypeDef> {
        vec![self.0]
    }

    pub fn is_blittable(&self) -> bool {
        self.0.fields().all(|f| f.is_blittable())
    }

    pub fn is_handle(&self) -> bool {
        self.0
            .has_attribute("Windows.Win32.Interop", "NativeTypedefAttribute")
    }

    pub fn gen_abi_name(&self, gen: Gen) -> TokenStream {
        if self.is_blittable() {
            self.0.gen_name(gen)
        } else {
            self.0.gen_abi_name(gen)
        }
    }

    pub fn gen(&self, gen: Gen) -> TokenStream {
        if let Some(replacement) = self.gen_replacement() {
            return replacement;
        }

        let name = self.0.gen_name(gen);

        if let Some(guid) = Guid::from_type_def(&self.0) {
            let guid = guid.gen();

            return quote! {
                pub const #name: ::windows::Guid = ::windows::Guid::from_values(#guid);
            };
        }

        if self.0.fields().next().is_none() {
            return quote! {
                #[repr(C)]
                #[allow(non_snake_case)]
                #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::marker::Copy)]
                pub struct #name(pub u8);
            };
        }

        let runtime_type = if self.0.is_winrt() {
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

        let is_handle = self.is_handle();
        let is_empty = self.0.fields().next().is_none();

        let copy = if is_handle {
            quote! {
                impl ::std::marker::Copy for #name {}
            }
        } else {
            quote! {}
        };

        let body = if is_handle {
            let fields = self.0.fields().map(|f| {
                let kind = f.signature().gen(gen);
                quote! {
                    pub #kind
                }
            });

            quote! {
                ( #(#fields),* );
            }
        } else {
            let fields = self.0.fields().map(|f| {
                let name = f.gen_name();
                let kind = f.signature().gen(gen);
                quote! {
                    pub #name: #kind
                }
            });

            quote! {
                { #(#fields),* }
            }
        };

        let abi = if self.is_blittable() {
            quote! {
                unsafe impl ::windows::Abi for #name {
                    type Abi = Self;
                }
            }
        } else {
            let abi_name = self.0.gen_abi_name(gen);
            let fields = self.0.fields().map(|f| f.signature().gen_abi(gen));

            quote! {
                #[repr(C)]
                #[doc(hidden)]
                pub struct #abi_name(#(#fields),*);
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

        let compare = if is_empty {
            quote! { true }
        } else {
            let fields = self.0.fields().enumerate().map(|(index, f)| {
                let name = f.gen_name();

                if let ElementType::Callback(_) = f.signature().kind {
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

            quote! { #(#fields)&&* }
        };

        let defaults = if is_handle {
            let defaults = self.0.fields().map(|f| f.signature().gen_default());

            quote! {
                Self( #(#defaults),* )
            }
        } else {
            let defaults = self.0.fields().map(|f| {
                let name = f.gen_name();
                let value = f.signature().gen_default();
                quote! {
                    #name: #value
                }
            });

            quote! {
                Self{ #(#defaults),* }
            }
        };

        let debug_name = self.0.name();

        let debug_fields = self.0.fields().enumerate().filter_map(|(index, field)| {
            // TODO: there must be a simpler way to implement Debug just to exclude this type.
            if let ElementType::Callback(_) = field.signature().kind {
                return None;
            }

            let name = to_snake(field.name());

            if is_handle {
                let index = Literal::u32_unsuffixed(index as u32);

                Some(quote! {
                    .field(#name, &format_args!("{:?}", self.#index))
                })
            } else {
                let field = to_ident(&name);

                Some(quote! {
                    .field(#name, &format_args!("{:?}", self.#field))
                })
            }
        });

        let extensions = self.gen_extensions();

        quote! {
            #[repr(C)]
            #[allow(non_snake_case)]
            #[derive(::std::clone::Clone)]
            pub struct #name #body
            impl #name {
                #(#constants)*
            }
            impl ::std::default::Default for #name {
                fn default() -> Self {
                    #defaults
                }
            }
            impl ::std::fmt::Debug for #name {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct(#debug_name)
                        #(#debug_fields)*
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    #compare
                }
            }
            impl ::std::cmp::Eq for #name {}
            #abi
            #copy
            #runtime_type
            #extensions
        }
    }

    fn gen_replacement(&self) -> Option<TokenStream> {
        match self.0.full_name() {
            ("Windows.Win32.SystemServices", "BOOL") => Some(quote! {
                #[repr(C)]
                #[allow(non_snake_case)]
                #[derive(::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::default::Default)]
                pub struct BOOL(pub i32);
                impl BOOL {
                    #[inline]
                    pub fn as_bool(self) -> bool {
                        if self.0 == 0 {
                            false
                        } else {
                            true
                        }
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
                #[allow(non_snake_case)]
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

                    fn drop_param<'a>(param: &mut ::windows::Param<'a, Self>) {
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
                #[allow(non_snake_case)]
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

                    fn drop_param<'a>(param: &mut ::windows::Param<'a, Self>) {
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
                #[allow(non_snake_case)]
                #[derive(:: std :: clone :: Clone, ::std::cmp::Eq)]
                pub struct BSTR(*mut u16);
                impl BSTR {
                    pub fn is_empty(&self) -> bool {
                        self.0.is_null()
                    }
                    fn from_wide(value: &[u16]) -> Self {
                        #[link(name = "oleaut32")]
                        extern "system" {
                            fn SysAllocStringLen(value: *const u16, len: u32) -> *mut u16;
                        }

                        if value.len() == 0 {
                            return Self(::std::ptr::null_mut());
                        }

                        unsafe { Self(SysAllocStringLen(value.as_ptr(), value.len() as u32)) }
                    }
                    fn as_wide(&self) -> &[u16] {
                        #[link(name = "oleaut32")]
                        extern "system" {
                            fn SysStringLen(bstr: *mut u16) -> u32;
                        }

                        if self.0.is_null() {
                            return &[];
                        }

                        unsafe { ::std::slice::from_raw_parts(self.0 as *const u16, SysStringLen(self.0) as usize) }
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
                        #[link(name = "oleaut32")]
                        extern "system" {
                            fn SysFreeString(bstr: *mut u16);
                        }

                        if !self.0.is_null() {
                            unsafe { SysFreeString(self.0); }
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
                            duration: (value.as_nanos() / 100) as i64,
                        }
                    }
                }
                impl ::std::convert::From<TimeSpan> for ::std::time::Duration {
                    fn from(value: TimeSpan) -> Self {
                        ::std::time::Duration::from_nanos((value.duration * 100) as u64)
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
                    pub fn zero() -> Self {
                        Self { x: 0f32, y: 0f32 }
                    }
                    pub fn one() -> Self {
                        Self { x: 1f32, y: 1f32 }
                    }
                    pub fn unit_x() -> Self {
                        Self { x: 1.0, y: 0.0 }
                    }
                    pub fn unit_y() -> Self {
                        Self { x: 0.0, y: 1.0 }
                    }
                    pub fn dot(&self, rhs: &Self) -> f32 {
                        self.x * rhs.x + self.y * rhs.y
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
                            x: self.x + rhs.x,
                            y: self.y + rhs.y,
                        }
                    }
                    fn impl_sub(&self, rhs: &Self) -> Self {
                        Self {
                            x: self.x - rhs.x,
                            y: self.y - rhs.y,
                        }
                    }
                    fn impl_div(&self, rhs: &Self) -> Self {
                        Self {
                            x: self.x / rhs.x,
                            y: self.y / rhs.y,
                        }
                    }
                    fn impl_div_f32(&self, rhs: f32) -> Self {
                        Self {
                            x: self.x / rhs,
                            y: self.y / rhs,
                        }
                    }
                    fn impl_mul(&self, rhs: &Self) -> Self {
                        Self {
                            x: self.x * rhs.x,
                            y: self.y * rhs.y,
                        }
                    }
                    fn impl_mul_f32(&self, rhs: f32) -> Self {
                        Self {
                            x: self.x * rhs,
                            y: self.y * rhs,
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
                    pub fn zero() -> Self {
                        Self {
                            x: 0f32,
                            y: 0f32,
                            z: 0f32,
                        }
                    }
                    pub fn one() -> Self {
                        Self {
                            x: 1f32,
                            y: 1f32,
                            z: 1f32,
                        }
                    }
                    pub fn unit_x() -> Self {
                        Self {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                        }
                    }
                    pub fn unit_y() -> Self {
                        Self {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        }
                    }
                    pub fn unit_z() -> Self {
                        Self {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        }
                    }
                    pub fn dot(&self, rhs: &Self) -> f32 {
                        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
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
                            x: self.x + rhs.x,
                            y: self.y + rhs.y,
                            z: self.z + rhs.z,
                        }
                    }
                    fn impl_sub(&self, rhs: &Self) -> Self {
                        Self {
                            x: self.x - rhs.x,
                            y: self.y - rhs.y,
                            z: self.z - rhs.z,
                        }
                    }
                    fn impl_div(&self, rhs: &Self) -> Self {
                        Self {
                            x: self.x / rhs.x,
                            y: self.y / rhs.y,
                            z: self.z / rhs.z,
                        }
                    }
                    fn impl_div_f32(&self, rhs: f32) -> Self {
                        Self {
                            x: self.x / rhs,
                            y: self.y / rhs,
                            z: self.z / rhs,
                        }
                    }
                    fn impl_mul(&self, rhs: &Self) -> Self {
                        Self {
                            x: self.x * rhs.x,
                            y: self.y * rhs.y,
                            z: self.z * rhs.z,
                        }
                    }
                    fn impl_mul_f32(&self, rhs: f32) -> Self {
                        Self {
                            x: self.x * rhs,
                            y: self.y * rhs,
                            z: self.z * rhs,
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
                    pub fn zero() -> Self {
                        Self {
                            x: 0f32,
                            y: 0f32,
                            z: 0f32,
                            w: 0f32,
                        }
                    }
                    pub fn one() -> Self {
                        Self {
                            x: 1f32,
                            y: 1f32,
                            z: 1f32,
                            w: 1f32,
                        }
                    }
                    pub fn unit_x() -> Self {
                        Self {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                            w: 0.0,
                        }
                    }
                    pub fn unit_y() -> Self {
                        Self {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                            w: 0.0,
                        }
                    }
                    pub fn unit_z() -> Self {
                        Self {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                            w: 0.0,
                        }
                    }
                    pub fn unit_w() -> Self {
                        Self {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                            w: 1.0,
                        }
                    }
                    pub fn dot(&self, rhs: &Self) -> f32 {
                        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
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
                            x: self.x + rhs.x,
                            y: self.y + rhs.y,
                            z: self.z + rhs.z,
                            w: self.w + rhs.w,
                        }
                    }
                    fn impl_sub(&self, rhs: &Self) -> Self {
                        Self {
                            x: self.x - rhs.x,
                            y: self.y - rhs.y,
                            z: self.z - rhs.z,
                            w: self.w - rhs.w,
                        }
                    }
                    fn impl_div(&self, rhs: &Self) -> Self {
                        Self {
                            x: self.x / rhs.x,
                            y: self.y / rhs.y,
                            z: self.z / rhs.z,
                            w: self.w / rhs.w,
                        }
                    }
                    fn impl_div_f32(&self, rhs: f32) -> Self {
                        Self {
                            x: self.x / rhs,
                            y: self.y / rhs,
                            z: self.z / rhs,
                            w: self.w / rhs,
                        }
                    }
                    fn impl_mul(&self, rhs: &Self) -> Self {
                        Self {
                            x: self.x * rhs.x,
                            y: self.y * rhs.y,
                            z: self.z * rhs.z,
                            w: self.w * rhs.w,
                        }
                    }
                    fn impl_mul_f32(&self, rhs: f32) -> Self {
                        Self {
                            x: self.x * rhs,
                            y: self.y * rhs,
                            z: self.z * rhs,
                            w: self.w * rhs,
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
                            m11: 1.0,
                            m12: 0.0,
                            m21: 0.0,
                            m22: 1.0,
                            m31: 0.0,
                            m32: 0.0,
                        }
                    }
                    pub fn translation(x: f32, y: f32) -> Self {
                        Self {
                            m11: 1.0,
                            m12: 0.0,
                            m21: 0.0,
                            m22: 1.0,
                            m31: x,
                            m32: y,
                        }
                    }
                    pub fn rotation(angle: f32, center_x: f32, center_y: f32) -> Self {
                        #[repr(C)]
                        pub struct Center(f32, f32);

                        #[link(name = "d2d1")]
                        extern "system" {
                            fn D2D1MakeRotateMatrix(angle: f32, center: Center, matrix: &mut Matrix3x2);
                        }

                        let mut matrix = Self::default();
                        unsafe {
                            D2D1MakeRotateMatrix(angle, Center(center_x, center_y), &mut matrix);
                        }
                        matrix
                    }
                    fn impl_add(&self, rhs: &Self) -> Self {
                        Self {
                            m11: self.m11 + rhs.m11,
                            m12: self.m12 + rhs.m12,
                            m21: self.m21 + rhs.m21,
                            m22: self.m22 + rhs.m22,
                            m31: self.m31 + rhs.m31,
                            m32: self.m32 + rhs.m32,
                        }
                    }
                    fn impl_sub(&self, rhs: &Self) -> Self {
                        Self {
                            m11: self.m11 - rhs.m11,
                            m12: self.m12 - rhs.m12,
                            m21: self.m21 - rhs.m21,
                            m22: self.m22 - rhs.m22,
                            m31: self.m31 - rhs.m31,
                            m32: self.m32 - rhs.m32,
                        }
                    }
                    fn impl_mul(&self, rhs: &Self) -> Self {
                        Self {
                            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
                            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
                            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
                            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
                            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + rhs.m31,
                            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + rhs.m32,
                        }
                    }
                    fn impl_mul_f32(&self, rhs: f32) -> Self {
                        Self {
                            m11: self.m11 * rhs,
                            m12: self.m12 * rhs,
                            m21: self.m21 * rhs,
                            m22: self.m22 * rhs,
                            m31: self.m31 * rhs,
                            m32: self.m32 * rhs,
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
                            m11: self.m11 + rhs.m11,
                            m12: self.m12 + rhs.m12,
                            m13: self.m13 + rhs.m13,
                            m14: self.m14 + rhs.m14,
                            m21: self.m21 + rhs.m21,
                            m22: self.m22 + rhs.m22,
                            m23: self.m23 + rhs.m23,
                            m24: self.m24 + rhs.m24,
                            m31: self.m31 + rhs.m31,
                            m32: self.m32 + rhs.m32,
                            m33: self.m33 + rhs.m33,
                            m34: self.m34 + rhs.m34,
                            m41: self.m41 + rhs.m41,
                            m42: self.m42 + rhs.m42,
                            m43: self.m43 + rhs.m43,
                            m44: self.m44 + rhs.m44,
                        }
                    }
                    fn impl_sub(&self, rhs: &Self) -> Self {
                        Self {
                            m11: self.m11 - rhs.m11,
                            m12: self.m12 - rhs.m12,
                            m13: self.m13 - rhs.m13,
                            m14: self.m14 - rhs.m14,
                            m21: self.m21 - rhs.m21,
                            m22: self.m22 - rhs.m22,
                            m23: self.m23 - rhs.m23,
                            m24: self.m24 - rhs.m24,
                            m31: self.m31 - rhs.m31,
                            m32: self.m32 - rhs.m32,
                            m33: self.m33 - rhs.m33,
                            m34: self.m34 - rhs.m34,
                            m41: self.m41 - rhs.m41,
                            m42: self.m42 - rhs.m42,
                            m43: self.m43 - rhs.m43,
                            m44: self.m44 - rhs.m44,
                        }
                    }
                    fn impl_mul(&self, rhs: &Self) -> Self {
                        Self {
                            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31 + self.m14 * rhs.m41,
                            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32 + self.m14 * rhs.m42,
                            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33 + self.m14 * rhs.m43,
                            m14: self.m11 * rhs.m14 + self.m12 * rhs.m24 + self.m13 * rhs.m34 + self.m14 * rhs.m44,
                            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31 + self.m24 * rhs.m41,
                            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32 + self.m24 * rhs.m42,
                            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33 + self.m24 * rhs.m43,
                            m24: self.m21 * rhs.m14 + self.m22 * rhs.m24 + self.m23 * rhs.m34 + self.m24 * rhs.m44,
                            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31 + self.m34 * rhs.m41,
                            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32 + self.m34 * rhs.m42,
                            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33 + self.m34 * rhs.m43,
                            m34: self.m31 * rhs.m14 + self.m32 * rhs.m24 + self.m33 * rhs.m34 + self.m34 * rhs.m44,
                            m41: self.m41 * rhs.m11 + self.m42 * rhs.m21 + self.m43 * rhs.m31 + self.m44 * rhs.m41,
                            m42: self.m41 * rhs.m12 + self.m42 * rhs.m22 + self.m43 * rhs.m32 + self.m44 * rhs.m42,
                            m43: self.m41 * rhs.m13 + self.m42 * rhs.m23 + self.m43 * rhs.m33 + self.m44 * rhs.m43,
                            m44: self.m41 * rhs.m14 + self.m42 * rhs.m24 + self.m43 * rhs.m34 + self.m44 * rhs.m44,
                        }
                    }
                    fn impl_mul_f32(&self, rhs: f32) -> Self {
                        Self {
                            m11: self.m11 * rhs,
                            m12: self.m12 * rhs,
                            m13: self.m13 * rhs,
                            m14: self.m14 * rhs,
                            m21: self.m21 * rhs,
                            m22: self.m22 * rhs,
                            m23: self.m23 * rhs,
                            m24: self.m24 * rhs,
                            m31: self.m31 * rhs,
                            m32: self.m32 * rhs,
                            m33: self.m33 * rhs,
                            m34: self.m34 * rhs,
                            m41: self.m41 * rhs,
                            m42: self.m42 * rhs,
                            m43: self.m43 * rhs,
                            m44: self.m44 * rhs,
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
