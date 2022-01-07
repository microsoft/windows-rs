#[cfg(feature = "Win32_System_Com")]
pub trait ICatalogImpl: Sized + IDispatchImpl {
    fn GetCollection();
    fn Connect();
    fn MajorVersion();
    fn MinorVersion();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICatalog {
    const NAME: &'static str = "Windows.Win32.System.TransactionServer.ICatalog";
}
#[cfg(feature = "Win32_System_Com")]
impl ICatalogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatalogImpl, const OFFSET: isize>() -> ICatalogVtbl {
        unsafe extern "system" fn GetCollection<Impl: ICatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollection(&*(&bstrcollname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcatalogcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: ICatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect(&*(&bstrconnectstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcatalogcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: ICatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion(retval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: ICatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion(retval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICatalog>, ::windows::core::GetTrustLevel, GetCollection::<Impl, OFFSET>, Connect::<Impl, OFFSET>, MajorVersion::<Impl, OFFSET>, MinorVersion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComponentUtilImpl: Sized + IDispatchImpl {
    fn InstallComponent();
    fn ImportComponent();
    fn ImportComponentByName();
    fn GetCLSIDs();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IComponentUtil {
    const NAME: &'static str = "Windows.Win32.System.TransactionServer.IComponentUtil";
}
#[cfg(feature = "Win32_System_Com")]
impl IComponentUtilVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentUtilImpl, const OFFSET: isize>() -> IComponentUtilVtbl {
        unsafe extern "system" fn InstallComponent<Impl: IComponentUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrproxystubdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallComponent(
                &*(&bstrdllfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrtypelibfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrproxystubdllfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportComponent<Impl: IComponentUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportComponent(&*(&bstrclsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportComponentByName<Impl: IComponentUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportComponentByName(&*(&bstrprogid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCLSIDs<Impl: IComponentUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCLSIDs(
                &*(&bstrdllfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrtypelibfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&aclsids as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComponentUtil>, ::windows::core::GetTrustLevel, InstallComponent::<Impl, OFFSET>, ImportComponent::<Impl, OFFSET>, ImportComponentByName::<Impl, OFFSET>, GetCLSIDs::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPackageUtilImpl: Sized + IDispatchImpl {
    fn InstallPackage();
    fn ExportPackage();
    fn ShutdownPackage();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPackageUtil {
    const NAME: &'static str = "Windows.Win32.System.TransactionServer.IPackageUtil";
}
#[cfg(feature = "Win32_System_Com")]
impl IPackageUtilVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageUtilImpl, const OFFSET: isize>() -> IPackageUtilVtbl {
        unsafe extern "system" fn InstallPackage<Impl: IPackageUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackagefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinstallpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallPackage(&*(&bstrpackagefile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrinstallpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), loptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPackage<Impl: IPackageUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportPackage(&*(&bstrpackageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrpackagefile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), loptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownPackage<Impl: IPackageUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShutdownPackage(&*(&bstrpackageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageUtil>, ::windows::core::GetTrustLevel, InstallPackage::<Impl, OFFSET>, ExportPackage::<Impl, OFFSET>, ShutdownPackage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteComponentUtilImpl: Sized + IDispatchImpl {
    fn InstallRemoteComponent();
    fn InstallRemoteComponentByName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRemoteComponentUtil {
    const NAME: &'static str = "Windows.Win32.System.TransactionServer.IRemoteComponentUtil";
}
#[cfg(feature = "Win32_System_Com")]
impl IRemoteComponentUtilVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteComponentUtilImpl, const OFFSET: isize>() -> IRemoteComponentUtilVtbl {
        unsafe extern "system" fn InstallRemoteComponent<Impl: IRemoteComponentUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallRemoteComponent(
                &*(&bstrserver as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpackageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrclsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallRemoteComponentByName<Impl: IRemoteComponentUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallRemoteComponentByName(
                &*(&bstrserver as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpackagename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrprogid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteComponentUtil>, ::windows::core::GetTrustLevel, InstallRemoteComponent::<Impl, OFFSET>, InstallRemoteComponentByName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRoleAssociationUtilImpl: Sized + IDispatchImpl {
    fn AssociateRole();
    fn AssociateRoleByName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRoleAssociationUtil {
    const NAME: &'static str = "Windows.Win32.System.TransactionServer.IRoleAssociationUtil";
}
#[cfg(feature = "Win32_System_Com")]
impl IRoleAssociationUtilVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRoleAssociationUtilImpl, const OFFSET: isize>() -> IRoleAssociationUtilVtbl {
        unsafe extern "system" fn AssociateRole<Impl: IRoleAssociationUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateRole(&*(&bstrroleid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociateRoleByName<Impl: IRoleAssociationUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateRoleByName(&*(&bstrrolename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRoleAssociationUtil>, ::windows::core::GetTrustLevel, AssociateRole::<Impl, OFFSET>, AssociateRoleByName::<Impl, OFFSET>)
    }
}
