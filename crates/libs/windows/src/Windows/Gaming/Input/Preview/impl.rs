#[cfg(feature = "implement_exclusive")]
pub trait IGameControllerProviderInfoStaticsImpl: Sized {
    fn GetParentProviderId(&self, provider: &::core::option::Option<super::Custom::IGameControllerProvider>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetProviderId(&self, provider: &::core::option::Option<super::Custom::IGameControllerProvider>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameControllerProviderInfoStatics {
    const NAME: &'static str = "Windows.Gaming.Input.Preview.IGameControllerProviderInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGameControllerProviderInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerProviderInfoStaticsImpl, const OFFSET: isize>() -> IGameControllerProviderInfoStaticsVtbl {
        unsafe extern "system" fn GetParentProviderId<Impl: IGameControllerProviderInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentProviderId(&*(&provider as *const <super::Custom::IGameControllerProvider as ::windows::core::Abi>::Abi as *const <super::Custom::IGameControllerProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderId<Impl: IGameControllerProviderInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderId(&*(&provider as *const <super::Custom::IGameControllerProvider as ::windows::core::Abi>::Abi as *const <super::Custom::IGameControllerProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameControllerProviderInfoStatics>, ::windows::core::GetTrustLevel, GetParentProviderId::<Impl, OFFSET>, GetProviderId::<Impl, OFFSET>)
    }
}
