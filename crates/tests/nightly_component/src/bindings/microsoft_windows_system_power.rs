#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPowerManager {
    type Vtable = IPowerManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fb54144_8cdf_5e0d_a781_f475737013fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManager_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct PowerManager(::windows::core::IUnknown);
impl PowerManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PowerManager, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::from_library(b"test_nightly_component.dll\0");
        unsafe { SHARED.call(callback) }
    }
    pub fn Property(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetProperty(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProperty)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for PowerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PowerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PowerManager {}
impl ::core::fmt::Debug for PowerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Microsoft.Windows.System.Power.PowerManager;{4fb54144-8cdf-5e0d-a781-f475737013fe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PowerManager {
    type Vtable = IPowerManager_Vtbl;
    const IID: ::windows::core::GUID = <IPowerManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PowerManager {
    const NAME: &'static str = "Microsoft.Windows.System.Power.PowerManager";
}
impl ::core::convert::From<PowerManager> for ::windows::core::IUnknown {
    fn from(value: PowerManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PowerManager> for ::windows::core::IUnknown {
    fn from(value: &PowerManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PowerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PowerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PowerManager> for ::windows::core::IInspectable {
    fn from(value: PowerManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PowerManager> for ::windows::core::IInspectable {
    fn from(value: &PowerManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PowerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PowerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PowerManager {}
unsafe impl ::core::marker::Sync for PowerManager {}
pub trait IPowerManager_Impl: Sized {
    fn Property(&self) -> ::windows::core::Result<i32>;
    fn SetProperty(&self, value: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPowerManager {
    const NAME: &'static str = "Microsoft.Windows.System.Power.IPowerManager";
}
impl IPowerManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPowerManager_Impl, const OFFSET: isize>() -> IPowerManager_Vtbl {
        unsafe extern "system" fn Property<Identity: ::windows::core::IUnknownImpl, Impl: IPowerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IPowerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(value).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IPowerManager, OFFSET>(),
            Property: Property::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPowerManager as ::windows::core::Interface>::IID
    }
}
