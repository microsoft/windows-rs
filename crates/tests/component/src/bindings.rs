#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IClass(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClass {
    type Vtable = IClass_Vtbl;
}
impl ::core::clone::Clone for IClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IClass {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8e698df5_82fa_5cf4_8fc9_06e4a38c7178);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClass_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Flags,
    ) -> ::windows_core::HRESULT,
    pub Int32Array: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a_array_size: u32,
        a: *const i32,
        b_array_size: u32,
        b: *mut i32,
        c_array_size: *mut u32,
        c: *mut *mut i32,
        result_size__: *mut u32,
        result__: *mut *mut i32,
    ) -> ::windows_core::HRESULT,
    pub StringArray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a_array_size: u32,
        a: *const ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        b_array_size: u32,
        b: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        c_array_size: *mut u32,
        c: *mut *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result_size__: *mut u32,
        result__: *mut *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Input: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut ::core::ffi::c_void,
        b: *mut ::core::ffi::c_void,
        c: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct Class(::windows_core::IUnknown);
impl Class {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            Class,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Property(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetProperty(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetProperty)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Flags(&self) -> ::windows_core::Result<Flags> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Flags)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Int32Array(
        &self,
        a: &[i32],
        b: &mut [i32],
        c: &mut ::windows_core::Array<i32>,
    ) -> ::windows_core::Result<::windows_core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).Int32Array)(
                ::windows_core::Interface::as_raw(this),
                a.len() as u32,
                a.as_ptr(),
                b.len() as u32,
                b.as_mut_ptr(),
                c.set_abi_len(),
                c as *mut _ as _,
                ::windows_core::Array::<i32>::set_abi_len(::std::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn StringArray(
        &self,
        a: &[::windows_core::HSTRING],
        b: &mut [::windows_core::HSTRING],
        c: &mut ::windows_core::Array<::windows_core::HSTRING>,
    ) -> ::windows_core::Result<::windows_core::Array<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).StringArray)(
                ::windows_core::Interface::as_raw(this),
                a.len() as u32,
                ::core::mem::transmute(a.as_ptr()),
                b.len() as u32,
                ::core::mem::transmute_copy(&b),
                c.set_abi_len(),
                c as *mut _ as _,
                ::windows_core::Array::<::windows_core::HSTRING>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn Input<P0, P1>(&self, a: P0, b: &Class, c: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<::windows::Foundation::IStringable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Input)(
                ::windows_core::Interface::as_raw(this),
                a.into_param().abi(),
                ::core::mem::transmute_copy(b),
                c.try_into_param()?.abi(),
            )
            .ok()
        }
    }
}
impl ::core::cmp::PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Class {}
impl ::core::fmt::Debug for Class {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Class").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Class {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"rc(test_component.Class;{8e698df5-82fa-5cf4-8fc9-06e4a38c7178})",
        );
}
impl ::core::clone::Clone for Class {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Class {
    type Vtable = IClass_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Class {
    const IID: ::windows_core::GUID = <IClass as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Class {
    const NAME: &'static str = "test_component.Class";
}
::windows_core::imp::interface_hierarchy!(
    Class,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for Class {}
unsafe impl ::core::marker::Sync for Class {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Flags(pub u32);
impl Flags {
    pub const Ok: Self = Self(0u32);
}
impl ::core::marker::Copy for Flags {}
impl ::core::clone::Clone for Flags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Flags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for Flags {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for Flags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Flags").field(&self.0).finish()
    }
}
impl Flags {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for Flags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for Flags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for Flags {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for Flags {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for Flags {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for Flags {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(b"enum(test_component.Flags;u4)");
}
pub trait IClass_Impl: Sized {
    fn Property(&self) -> ::windows_core::Result<i32>;
    fn SetProperty(&self, value: i32) -> ::windows_core::Result<()>;
    fn Flags(&self) -> ::windows_core::Result<Flags>;
    fn Int32Array(
        &self,
        a: &[i32],
        b: &mut [i32],
        c: &mut ::windows_core::Array<i32>,
    ) -> ::windows_core::Result<::windows_core::Array<i32>>;
    fn StringArray(
        &self,
        a: &[::windows_core::HSTRING],
        b: &mut [::windows_core::HSTRING],
        c: &mut ::windows_core::Array<::windows_core::HSTRING>,
    ) -> ::windows_core::Result<::windows_core::Array<::windows_core::HSTRING>>;
    fn Input(
        &self,
        a: ::core::option::Option<&::windows_core::IInspectable>,
        b: ::core::option::Option<&Class>,
        c: ::core::option::Option<&::windows::Foundation::IStringable>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IClass {
    const NAME: &'static str = "test_component.IClass";
}
impl IClass_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IClass_Impl,
        const OFFSET: isize,
    >() -> IClass_Vtbl {
        unsafe extern "system" fn Property<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Property() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(value).into()
        }
        unsafe extern "system" fn Flags<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut Flags,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Flags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Int32Array<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            a_array_size: u32,
            a: *const i32,
            b_array_size: u32,
            b: *mut i32,
            c_array_size: *mut u32,
            c: *mut *mut i32,
            result_size__: *mut u32,
            result__: *mut *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Int32Array(
                ::core::slice::from_raw_parts(::core::mem::transmute_copy(&a), a_array_size as _),
                ::core::slice::from_raw_parts_mut(
                    ::core::mem::transmute_copy(&b),
                    b_array_size as _,
                ),
                ::windows_core::ArrayProxy::from_raw_parts(
                    ::core::mem::transmute_copy(&c),
                    c_array_size,
                )
                .as_array(),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StringArray<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            a_array_size: u32,
            a: *const ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            b_array_size: u32,
            b: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            c_array_size: *mut u32,
            c: *mut *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result_size__: *mut u32,
            result__: *mut *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StringArray(
                ::core::slice::from_raw_parts(::core::mem::transmute_copy(&a), a_array_size as _),
                ::core::slice::from_raw_parts_mut(
                    ::core::mem::transmute_copy(&b),
                    b_array_size as _,
                ),
                ::windows_core::ArrayProxy::from_raw_parts(
                    ::core::mem::transmute_copy(&c),
                    c_array_size,
                )
                .as_array(),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Input<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            a: *mut ::core::ffi::c_void,
            b: *mut ::core::ffi::c_void,
            c: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Input(
                ::windows_core::from_raw_borrowed(&a),
                ::windows_core::from_raw_borrowed(&b),
                ::windows_core::from_raw_borrowed(&c),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IClass, OFFSET>(),
            Property: Property::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            Int32Array: Int32Array::<Identity, Impl, OFFSET>,
            StringArray: StringArray::<Identity, Impl, OFFSET>,
            Input: Input::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IClass as ::windows_core::ComInterface>::IID
    }
}
