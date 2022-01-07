#[cfg(feature = "implement_exclusive")]
pub trait ISoundLevelBrokerStaticsImpl: Sized {
    fn SoundLevel(&self) -> ::windows::core::Result<super::super::SoundLevel>;
    fn SoundLevelChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSoundLevelChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISoundLevelBrokerStatics {
    const NAME: &'static str = "Windows.Media.Core.Preview.ISoundLevelBrokerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISoundLevelBrokerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoundLevelBrokerStaticsImpl, const OFFSET: isize>() -> ISoundLevelBrokerStaticsVtbl {
        unsafe extern "system" fn SoundLevel<Impl: ISoundLevelBrokerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::SoundLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SoundLevelChanged<Impl: ISoundLevelBrokerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSoundLevelChanged<Impl: ISoundLevelBrokerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSoundLevelChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISoundLevelBrokerStatics>, ::windows::core::GetTrustLevel, SoundLevel::<Impl, OFFSET>, SoundLevelChanged::<Impl, OFFSET>, RemoveSoundLevelChanged::<Impl, OFFSET>)
    }
}
