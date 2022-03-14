use super::*;

// TODO: need to use the generated version
// https://github.com/microsoft/win32metadata/issues/833

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IActivationFactory(IUnknown);

impl IActivationFactory {
    pub fn ActivateInstance<I: Interface>(&self) -> ::windows::core::Result<I> {
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(self).ActivateInstance)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IInspectable>(result__)?.cast()
        }
    }
}

#[repr(C)]
#[doc(hidden)]
pub struct IActivationFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ActivateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}

unsafe impl ::windows::core::Interface for IActivationFactory {
    type Vtable = IActivationFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000035_0000_0000_c000_000000000046);
}

impl core::fmt::Debug for IActivationFactory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IActivationFactory").field(&self.0).finish()
    }
}

pub trait IActivationFactory_Impl: Sized {
    fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}

impl ::windows::core::RuntimeName for IActivationFactory {
    const NAME: &'static str = "";
}

#[cfg(feature = "implement")]
impl IActivationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationFactory_Impl, const OFFSET: isize>() -> IActivationFactory_Vtbl {
        unsafe extern "system" fn ActivateInstance<Identity: ::windows::core::IUnknownImpl, Impl: IActivationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActivateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *instance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivationFactory, OFFSET>(),
            ActivateInstance: ActivateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationFactory as ::windows::core::Interface>::IID
    }
}
