#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISoundLevelBrokerStatics_Impl: Sized {
    fn SoundLevel(&mut self) -> ::windows::core::Result<super::super::SoundLevel>;
    fn SoundLevelChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSoundLevelChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISoundLevelBrokerStatics {
    const NAME: &'static str = "Windows.Media.Core.Preview.ISoundLevelBrokerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISoundLevelBrokerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoundLevelBrokerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISoundLevelBrokerStatics_Vtbl {
        unsafe extern "system" fn SoundLevel<Impl: ISoundLevelBrokerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::SoundLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoundLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoundLevelChanged<Impl: ISoundLevelBrokerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoundLevelChanged(&*(&handler as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSoundLevelChanged<Impl: ISoundLevelBrokerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSoundLevelChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISoundLevelBrokerStatics, BASE_OFFSET>(),
            SoundLevel: SoundLevel::<Impl, IMPL_OFFSET>,
            SoundLevelChanged: SoundLevelChanged::<Impl, IMPL_OFFSET>,
            RemoveSoundLevelChanged: RemoveSoundLevelChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISoundLevelBrokerStatics as ::windows::core::Interface>::IID
    }
}
