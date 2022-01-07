#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationDataManager {
    const NAME: &'static str = "Windows.Management.Core.IApplicationDataManager";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationDataManagerVtbl {
    pub const fn new<Impl: IApplicationDataManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationDataManagerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplicationDataManager>, base.5)
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
    pub const fn new<Impl: IApplicationDataManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationDataManagerStaticsVtbl {
        unsafe extern "system" fn CreateForPackageFamily<Impl: IApplicationDataManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForPackageFamily(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplicationDataManagerStatics>, base.5, CreateForPackageFamily::<Impl, OFFSET>)
    }
}
