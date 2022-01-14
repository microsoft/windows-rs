#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataManager_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationDataManager {
    const NAME: &'static str = "Windows.Management.Core.IApplicationDataManager";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationDataManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationDataManager_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationDataManager, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationDataManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
pub trait IApplicationDataManagerStatics_Impl: Sized {
    fn CreateForPackageFamily(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Storage::ApplicationData>;
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationDataManagerStatics {
    const NAME: &'static str = "Windows.Management.Core.IApplicationDataManagerStatics";
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl IApplicationDataManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationDataManagerStatics_Vtbl {
        unsafe extern "system" fn CreateForPackageFamily<Impl: IApplicationDataManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationDataManagerStatics, BASE_OFFSET>(),
            CreateForPackageFamily: CreateForPackageFamily::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationDataManagerStatics as ::windows::core::Interface>::IID
    }
}
