#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationDataManager {
    const NAME: &'static str = "Windows.Management.Core.IApplicationDataManager";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationDataManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataManagerImpl, const OFFSET: isize>() -> IApplicationDataManagerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IApplicationDataManager>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataManagerStaticsImpl: Sized {
    fn CreateForPackageFamily(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Storage::ApplicationData>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationDataManagerStatics {
    const NAME: &'static str = "Windows.Management.Core.IApplicationDataManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationDataManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataManagerStaticsImpl, const OFFSET: isize>() -> IApplicationDataManagerStaticsVtbl {
        unsafe extern "system" fn CreateForPackageFamily<Impl: IApplicationDataManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForPackageFamily(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IApplicationDataManagerStatics>, ::windows::core::GetTrustLevel, CreateForPackageFamily::<Impl, OFFSET>)
    }
}
