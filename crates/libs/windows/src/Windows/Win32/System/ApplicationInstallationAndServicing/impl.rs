pub trait IAssemblyCacheImpl: Sized {
    fn UninstallAssembly();
    fn QueryAssemblyInfo();
    fn CreateAssemblyCacheItem();
    fn Reserved();
    fn InstallAssembly();
}
impl ::windows::core::RuntimeName for IAssemblyCache {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IAssemblyCache";
}
impl IAssemblyCacheVtbl {
    pub const fn new<Impl: IAssemblyCacheImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAssemblyCacheVtbl {
        unsafe extern "system" fn UninstallAssembly<Impl: IAssemblyCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszassemblyname: super::super::Foundation::PWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UninstallAssembly(dwflags, &*(&pszassemblyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&prefdata as *const <FUSION_INSTALL_REFERENCE as ::windows::core::Abi>::Abi as *const <FUSION_INSTALL_REFERENCE as ::windows::core::DefaultType>::DefaultType), puldisposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssemblyInfo<Impl: IAssemblyCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: super::super::Foundation::PWSTR, pasminfo: *mut ASSEMBLY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryAssemblyInfo(dwflags, &*(&pszassemblyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pasminfo as *const <ASSEMBLY_INFO as ::windows::core::Abi>::Abi as *const <ASSEMBLY_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAssemblyCacheItem<Impl: IAssemblyCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut ::windows::core::RawPtr, pszassemblyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAssemblyCacheItem(dwflags, &*(&pvreserved as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppasmitem), &*(&pszassemblyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserved<Impl: IAssemblyCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reserved(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallAssembly<Impl: IAssemblyCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszmanifestfilepath: super::super::Foundation::PWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstallAssembly(dwflags, &*(&pszmanifestfilepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&prefdata as *const <FUSION_INSTALL_REFERENCE as ::windows::core::Abi>::Abi as *const <FUSION_INSTALL_REFERENCE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAssemblyCache>, base.5, UninstallAssembly::<Impl, OFFSET>, QueryAssemblyInfo::<Impl, OFFSET>, CreateAssemblyCacheItem::<Impl, OFFSET>, Reserved::<Impl, OFFSET>, InstallAssembly::<Impl, OFFSET>)
    }
}
pub trait IAssemblyCacheItemImpl: Sized {
    fn CreateStream();
    fn Commit();
    fn AbortItem();
}
impl ::windows::core::RuntimeName for IAssemblyCacheItem {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IAssemblyCacheItem";
}
impl IAssemblyCacheItemVtbl {
    pub const fn new<Impl: IAssemblyCacheItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAssemblyCacheItemVtbl {
        unsafe extern "system" fn CreateStream<Impl: IAssemblyCacheItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszstreamname: super::super::Foundation::PWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut ::windows::core::RawPtr, pulimaxsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStream(dwflags, &*(&pszstreamname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwformat, dwformatflags, ::core::mem::transmute_copy(&ppistream), pulimaxsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IAssemblyCacheItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, puldisposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Commit(dwflags, puldisposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortItem<Impl: IAssemblyCacheItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbortItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAssemblyCacheItem>, base.5, CreateStream::<Impl, OFFSET>, Commit::<Impl, OFFSET>, AbortItem::<Impl, OFFSET>)
    }
}
pub trait IAssemblyNameImpl: Sized {
    fn SetProperty();
    fn GetProperty();
    fn Finalize();
    fn GetDisplayName();
    fn Reserved();
    fn GetName();
    fn GetVersion();
    fn IsEqual();
    fn Clone();
}
impl ::windows::core::RuntimeName for IAssemblyName {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IAssemblyName";
}
impl IAssemblyNameVtbl {
    pub const fn new<Impl: IAssemblyNameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAssemblyNameVtbl {
        unsafe extern "system" fn SetProperty<Impl: IAssemblyNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProperty(propertyid, &*(&pvproperty as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbproperty) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAssemblyNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperty(propertyid, &*(&pvproperty as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), pcbproperty) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finalize<Impl: IAssemblyNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finalize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Impl: IAssemblyNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szdisplayname: super::super::Foundation::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayName(::core::mem::transmute_copy(&szdisplayname), pccdisplayname, dwdisplayflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserved<Impl: IAssemblyNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, punkreserved1: *mut ::core::ffi::c_void, punkreserved2: *mut ::core::ffi::c_void, szreserved: super::super::Foundation::PWSTR, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reserved(
                &*(&refiid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&punkreserved1 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&punkreserved2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&szreserved as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                llreserved,
                &*(&pvreserved as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbreserved,
                &*(&ppreserved as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IAssemblyNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpcwbuffer: *mut u32, pwzname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetName(lpcwbuffer, ::core::mem::transmute_copy(&pwzname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: IAssemblyNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVersion(pdwversionhi, pdwversionlow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IAssemblyNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::RawPtr, dwcmpflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&pname as *const <IAssemblyName as ::windows::core::Abi>::Abi as *const <IAssemblyName as ::windows::core::DefaultType>::DefaultType), dwcmpflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IAssemblyNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAssemblyName>, base.5, SetProperty::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>, Finalize::<Impl, OFFSET>, GetDisplayName::<Impl, OFFSET>, Reserved::<Impl, OFFSET>, GetName::<Impl, OFFSET>, GetVersion::<Impl, OFFSET>, IsEqual::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumMsmDependencyImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumMsmDependency {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IEnumMsmDependency";
}
impl IEnumMsmDependencyVtbl {
    pub const fn new<Impl: IEnumMsmDependencyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumMsmDependencyVtbl {
        unsafe extern "system" fn Next<Impl: IEnumMsmDependencyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmdependencies: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cfetch, ::core::mem::transmute_copy(&rgmsmdependencies), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumMsmDependencyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(cskip) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumMsmDependencyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumMsmDependencyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pemsmdependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&pemsmdependencies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumMsmDependency>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumMsmErrorImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumMsmError {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IEnumMsmError";
}
impl IEnumMsmErrorVtbl {
    pub const fn new<Impl: IEnumMsmErrorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumMsmErrorVtbl {
        unsafe extern "system" fn Next<Impl: IEnumMsmErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmerrors: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cfetch, ::core::mem::transmute_copy(&rgmsmerrors), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumMsmErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(cskip) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumMsmErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumMsmErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pemsmerrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&pemsmerrors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumMsmError>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumMsmStringImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumMsmString {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IEnumMsmString";
}
impl IEnumMsmStringVtbl {
    pub const fn new<Impl: IEnumMsmStringImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumMsmStringVtbl {
        unsafe extern "system" fn Next<Impl: IEnumMsmStringImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgbstrstrings: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cfetch, &*(&rgbstrstrings as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumMsmStringImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(cskip) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumMsmStringImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumMsmStringImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pemsmstrings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&pemsmstrings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumMsmString>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmDependenciesImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMsmDependencies {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IMsmDependencies";
}
#[cfg(feature = "Win32_System_Com")]
impl IMsmDependenciesVtbl {
    pub const fn new<Impl: IMsmDependenciesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMsmDependenciesVtbl {
        unsafe extern "system" fn Item<Impl: IMsmDependenciesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(item, ::core::mem::transmute_copy(&r#return)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMsmDependenciesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IMsmDependenciesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMsmDependencies>, base.5, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmDependencyImpl: Sized + IDispatchImpl {
    fn Module();
    fn Language();
    fn Version();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMsmDependency {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IMsmDependency";
}
#[cfg(feature = "Win32_System_Com")]
impl IMsmDependencyVtbl {
    pub const fn new<Impl: IMsmDependencyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMsmDependencyVtbl {
        unsafe extern "system" fn Module<Impl: IMsmDependencyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, module: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Module(&*(&module as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IMsmDependencyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language(language) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: IMsmDependencyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Version(&*(&version as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMsmDependency>, base.5, Module::<Impl, OFFSET>, Language::<Impl, OFFSET>, Version::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmErrorImpl: Sized + IDispatchImpl {
    fn Type();
    fn Path();
    fn Language();
    fn DatabaseTable();
    fn DatabaseKeys();
    fn ModuleTable();
    fn ModuleKeys();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMsmError {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IMsmError";
}
#[cfg(feature = "Win32_System_Com")]
impl IMsmErrorVtbl {
    pub const fn new<Impl: IMsmErrorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMsmErrorVtbl {
        unsafe extern "system" fn Type<Impl: IMsmErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errortype: *mut msmErrorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type(errortype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IMsmErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Path(&*(&errorpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IMsmErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorlanguage: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language(errorlanguage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DatabaseTable<Impl: IMsmErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DatabaseTable(&*(&errortable as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DatabaseKeys<Impl: IMsmErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DatabaseKeys(::core::mem::transmute_copy(&errorkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModuleTable<Impl: IMsmErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModuleTable(&*(&errortable as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModuleKeys<Impl: IMsmErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModuleKeys(::core::mem::transmute_copy(&errorkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMsmError>, base.5, Type::<Impl, OFFSET>, Path::<Impl, OFFSET>, Language::<Impl, OFFSET>, DatabaseTable::<Impl, OFFSET>, DatabaseKeys::<Impl, OFFSET>, ModuleTable::<Impl, OFFSET>, ModuleKeys::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmErrorsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMsmErrors {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IMsmErrors";
}
#[cfg(feature = "Win32_System_Com")]
impl IMsmErrorsVtbl {
    pub const fn new<Impl: IMsmErrorsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMsmErrorsVtbl {
        unsafe extern "system" fn Item<Impl: IMsmErrorsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(item, ::core::mem::transmute_copy(&r#return)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMsmErrorsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IMsmErrorsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMsmErrors>, base.5, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmGetFilesImpl: Sized + IDispatchImpl {
    fn ModuleFiles();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMsmGetFiles {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IMsmGetFiles";
}
#[cfg(feature = "Win32_System_Com")]
impl IMsmGetFilesVtbl {
    pub const fn new<Impl: IMsmGetFilesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMsmGetFilesVtbl {
        unsafe extern "system" fn ModuleFiles<Impl: IMsmGetFilesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, files: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModuleFiles(::core::mem::transmute_copy(&files)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMsmGetFiles>, base.5, ModuleFiles::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmMergeImpl: Sized + IDispatchImpl {
    fn OpenDatabase();
    fn OpenModule();
    fn CloseDatabase();
    fn CloseModule();
    fn OpenLog();
    fn CloseLog();
    fn Log();
    fn Errors();
    fn Dependencies();
    fn Merge();
    fn Connect();
    fn ExtractCAB();
    fn ExtractFiles();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMsmMerge {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IMsmMerge";
}
#[cfg(feature = "Win32_System_Com")]
impl IMsmMergeVtbl {
    pub const fn new<Impl: IMsmMergeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMsmMergeVtbl {
        unsafe extern "system" fn OpenDatabase<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenDatabase(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenModule<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, language: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenModule(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), language) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseDatabase<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commit: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseDatabase(commit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseModule<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseModule() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenLog<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenLog(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseLog<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseLog() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Log<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Log(&*(&message as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Errors<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Errors(::core::mem::transmute_copy(&errors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependencies<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Dependencies(::core::mem::transmute_copy(&dependencies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, redirectdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Merge(&*(&feature as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&redirectdir as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connect(&*(&feature as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtractCAB<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtractCAB(&*(&filename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtractFiles<Impl: IMsmMergeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtractFiles(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMsmMerge>, base.5, OpenDatabase::<Impl, OFFSET>, OpenModule::<Impl, OFFSET>, CloseDatabase::<Impl, OFFSET>, CloseModule::<Impl, OFFSET>, OpenLog::<Impl, OFFSET>, CloseLog::<Impl, OFFSET>, Log::<Impl, OFFSET>, Errors::<Impl, OFFSET>, Dependencies::<Impl, OFFSET>, Merge::<Impl, OFFSET>, Connect::<Impl, OFFSET>, ExtractCAB::<Impl, OFFSET>, ExtractFiles::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmStringsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMsmStrings {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IMsmStrings";
}
#[cfg(feature = "Win32_System_Com")]
impl IMsmStringsVtbl {
    pub const fn new<Impl: IMsmStringsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMsmStringsVtbl {
        unsafe extern "system" fn Item<Impl: IMsmStringsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(item, &*(&r#return as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMsmStringsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IMsmStringsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMsmStrings>, base.5, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
pub trait IPMApplicationInfoImpl: Sized {
    fn ProductID();
    fn InstanceID();
    fn OfferID();
    fn DefaultTask();
    fn AppTitle();
    fn IconPath();
    fn NotificationState();
    fn AppInstallType();
    fn State();
    fn IsRevoked();
    fn UpdateAvailable();
    fn InstallDate();
    fn IsUninstallable();
    fn IsThemable();
    fn IsTrial();
    fn InstallPath();
    fn DataRoot();
    fn Genre();
    fn Publisher();
    fn Author();
    fn Description();
    fn Version();
    fn InvocationInfo();
    fn AppPlatMajorVersion();
    fn AppPlatMinorVersion();
    fn PublisherID();
    fn IsMultiCore();
    fn SID();
    fn AppPlatMajorVersionLightUp();
    fn AppPlatMinorVersionLightUp();
    fn set_UpdateAvailable();
    fn set_NotificationState();
    fn set_IconPath();
    fn set_UninstallableState();
    fn IsPinableOnKidZone();
    fn IsOriginallyPreInstalled();
    fn IsInstallOnSD();
    fn IsOptoutOnSD();
    fn IsOptoutBackupRestore();
    fn set_EnterpriseDisabled();
    fn set_EnterpriseUninstallable();
    fn EnterpriseDisabled();
    fn EnterpriseUninstallable();
    fn IsVisibleOnAppList();
    fn IsInboxApp();
    fn StorageID();
    fn StartAppBlob();
    fn IsMovable();
    fn DeploymentAppEnumerationHubFilter();
    fn ModifiedDate();
    fn IsOriginallyRestored();
    fn ShouldDeferMdilBind();
    fn IsFullyPreInstall();
    fn set_IsMdilMaintenanceNeeded();
    fn set_Title();
}
impl ::windows::core::RuntimeName for IPMApplicationInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMApplicationInfo";
}
impl IPMApplicationInfoVtbl {
    pub const fn new<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMApplicationInfoVtbl {
        unsafe extern "system" fn ProductID<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProductID(::core::mem::transmute_copy(&pproductid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceID<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstanceID(::core::mem::transmute_copy(&pinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfferID<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pofferid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OfferID(::core::mem::transmute_copy(&pofferid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultTask<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdefaulttask: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultTask(&*(&pdefaulttask as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppTitle<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppTitle(&*(&papptitle as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconPath<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappiconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IconPath(&*(&pappiconpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotificationState<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotificationState(::core::mem::transmute_copy(&pisnotified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppInstallType<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppInstallType(::core::mem::transmute_copy(&pappinstalltype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut PM_APPLICATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRevoked<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisrevoked: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRevoked(::core::mem::transmute_copy(&pisrevoked)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAvailable<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisupdateavailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateAvailable(::core::mem::transmute_copy(&pisupdateavailable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallDate<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstalldate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstallDate(::core::mem::transmute_copy(&pinstalldate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUninstallable<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisuninstallable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUninstallable(::core::mem::transmute_copy(&pisuninstallable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThemable<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsThemable(::core::mem::transmute_copy(&pisthemable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTrial<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistrial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTrial(::core::mem::transmute_copy(&pistrial)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallPath<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstallpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstallPath(&*(&pinstallpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataRoot<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataroot: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataRoot(&*(&pdataroot as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Genre<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgenre: *mut PM_APP_GENRE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Genre(::core::mem::transmute_copy(&pgenre)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Publisher<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppublisher: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Publisher(&*(&ppublisher as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Author(&*(&pauthor as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Description(&*(&pdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Version(&*(&pversion as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvocationInfo(&*(&pimageurn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pparameters as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMajorVersion<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppPlatMajorVersion(::core::mem::transmute_copy(&pmajorver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMinorVersion<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppPlatMinorVersion(::core::mem::transmute_copy(&pminorver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublisherID<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppublisherid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PublisherID(::core::mem::transmute_copy(&ppublisherid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMultiCore<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pismulticore: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMultiCore(::core::mem::transmute_copy(&pismulticore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SID<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SID(&*(&psid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMajorVersionLightUp<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppPlatMajorVersionLightUp(::core::mem::transmute_copy(&pmajorver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMinorVersionLightUp<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppPlatMinorVersionLightUp(::core::mem::transmute_copy(&pminorver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_UpdateAvailable<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isupdateavailable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_UpdateAvailable(&*(&isupdateavailable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_NotificationState<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isnotified: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_NotificationState(&*(&isnotified as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IconPath<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appiconpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_IconPath(&*(&appiconpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_UninstallableState<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_UninstallableState(&*(&isuninstallable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPinableOnKidZone<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pispinable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPinableOnKidZone(::core::mem::transmute_copy(&pispinable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOriginallyPreInstalled<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pispreinstalled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOriginallyPreInstalled(::core::mem::transmute_copy(&pispreinstalled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInstallOnSD<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisinstallonsd: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInstallOnSD(::core::mem::transmute_copy(&pisinstallonsd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptoutOnSD<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisoptoutonsd: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOptoutOnSD(::core::mem::transmute_copy(&pisoptoutonsd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptoutBackupRestore<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisoptoutbackuprestore: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOptoutBackupRestore(::core::mem::transmute_copy(&pisoptoutbackuprestore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_EnterpriseDisabled<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isdisabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_EnterpriseDisabled(&*(&isdisabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_EnterpriseUninstallable<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_EnterpriseUninstallable(&*(&isuninstallable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterpriseDisabled<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnterpriseDisabled(::core::mem::transmute_copy(&isdisabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterpriseUninstallable<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isuninstallable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnterpriseUninstallable(::core::mem::transmute_copy(&isuninstallable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisibleOnAppList<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisvisible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsVisibleOnAppList(::core::mem::transmute_copy(&pisvisible)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInboxApp<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisinboxapp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInboxApp(::core::mem::transmute_copy(&pisinboxapp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageID<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstorageid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StorageID(::core::mem::transmute_copy(&pstorageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAppBlob<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTAPPBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAppBlob(&*(&pblob as *const <PM_STARTAPPBLOB as ::windows::core::Abi>::Abi as *const <PM_STARTAPPBLOB as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMovable<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pismovable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMovable(::core::mem::transmute_copy(&pismovable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeploymentAppEnumerationHubFilter<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: *mut PM_TILE_HUBTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeploymentAppEnumerationHubFilter(::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiedDate<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmodifieddate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModifiedDate(::core::mem::transmute_copy(&pmodifieddate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOriginallyRestored<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisrestored: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOriginallyRestored(::core::mem::transmute_copy(&pisrestored)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldDeferMdilBind<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfdefermdilbind: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldDeferMdilBind(::core::mem::transmute_copy(&pfdefermdilbind)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFullyPreInstall<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisfullypreinstall: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFullyPreInstall(::core::mem::transmute_copy(&pfisfullypreinstall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsMdilMaintenanceNeeded<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_IsMdilMaintenanceNeeded(&*(&fismdilmaintenanceneeded as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_Title<Impl: IPMApplicationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, apptitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_Title(&*(&apptitle as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPMApplicationInfo>,
            base.5,
            ProductID::<Impl, OFFSET>,
            InstanceID::<Impl, OFFSET>,
            OfferID::<Impl, OFFSET>,
            DefaultTask::<Impl, OFFSET>,
            AppTitle::<Impl, OFFSET>,
            IconPath::<Impl, OFFSET>,
            NotificationState::<Impl, OFFSET>,
            AppInstallType::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            IsRevoked::<Impl, OFFSET>,
            UpdateAvailable::<Impl, OFFSET>,
            InstallDate::<Impl, OFFSET>,
            IsUninstallable::<Impl, OFFSET>,
            IsThemable::<Impl, OFFSET>,
            IsTrial::<Impl, OFFSET>,
            InstallPath::<Impl, OFFSET>,
            DataRoot::<Impl, OFFSET>,
            Genre::<Impl, OFFSET>,
            Publisher::<Impl, OFFSET>,
            Author::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            Version::<Impl, OFFSET>,
            InvocationInfo::<Impl, OFFSET>,
            AppPlatMajorVersion::<Impl, OFFSET>,
            AppPlatMinorVersion::<Impl, OFFSET>,
            PublisherID::<Impl, OFFSET>,
            IsMultiCore::<Impl, OFFSET>,
            SID::<Impl, OFFSET>,
            AppPlatMajorVersionLightUp::<Impl, OFFSET>,
            AppPlatMinorVersionLightUp::<Impl, OFFSET>,
            set_UpdateAvailable::<Impl, OFFSET>,
            set_NotificationState::<Impl, OFFSET>,
            set_IconPath::<Impl, OFFSET>,
            set_UninstallableState::<Impl, OFFSET>,
            IsPinableOnKidZone::<Impl, OFFSET>,
            IsOriginallyPreInstalled::<Impl, OFFSET>,
            IsInstallOnSD::<Impl, OFFSET>,
            IsOptoutOnSD::<Impl, OFFSET>,
            IsOptoutBackupRestore::<Impl, OFFSET>,
            set_EnterpriseDisabled::<Impl, OFFSET>,
            set_EnterpriseUninstallable::<Impl, OFFSET>,
            EnterpriseDisabled::<Impl, OFFSET>,
            EnterpriseUninstallable::<Impl, OFFSET>,
            IsVisibleOnAppList::<Impl, OFFSET>,
            IsInboxApp::<Impl, OFFSET>,
            StorageID::<Impl, OFFSET>,
            StartAppBlob::<Impl, OFFSET>,
            IsMovable::<Impl, OFFSET>,
            DeploymentAppEnumerationHubFilter::<Impl, OFFSET>,
            ModifiedDate::<Impl, OFFSET>,
            IsOriginallyRestored::<Impl, OFFSET>,
            ShouldDeferMdilBind::<Impl, OFFSET>,
            IsFullyPreInstall::<Impl, OFFSET>,
            set_IsMdilMaintenanceNeeded::<Impl, OFFSET>,
            set_Title::<Impl, OFFSET>,
        )
    }
}
pub trait IPMApplicationInfoEnumeratorImpl: Sized {
    fn Next();
}
impl ::windows::core::RuntimeName for IPMApplicationInfoEnumerator {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMApplicationInfoEnumerator";
}
impl IPMApplicationInfoEnumeratorVtbl {
    pub const fn new<Impl: IPMApplicationInfoEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMApplicationInfoEnumeratorVtbl {
        unsafe extern "system" fn Next<Impl: IPMApplicationInfoEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&ppappinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMApplicationInfoEnumerator>, base.5, Next::<Impl, OFFSET>)
    }
}
pub trait IPMBackgroundServiceAgentInfoImpl: Sized {
    fn ProductID();
    fn TaskID();
    fn BSAID();
    fn BGSpecifier();
    fn BGName();
    fn BGSource();
    fn BGType();
    fn IsPeriodic();
    fn IsScheduled();
    fn IsScheduleAllowed();
    fn Description();
    fn IsLaunchOnBoot();
    fn set_IsScheduled();
    fn set_IsScheduleAllowed();
}
impl ::windows::core::RuntimeName for IPMBackgroundServiceAgentInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMBackgroundServiceAgentInfo";
}
impl IPMBackgroundServiceAgentInfoVtbl {
    pub const fn new<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMBackgroundServiceAgentInfoVtbl {
        unsafe extern "system" fn ProductID<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProductID(::core::mem::transmute_copy(&pproductid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskID(&*(&ptaskid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BSAID<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsaid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BSAID(::core::mem::transmute_copy(&pbsaid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BGSpecifier<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbgspecifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BGSpecifier(&*(&pbgspecifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BGName<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BGName(&*(&pbgname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BGSource<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbgsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BGSource(&*(&pbgsource as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BGType<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbgtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BGType(&*(&pbgtype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPeriodic<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisperiodic: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPeriodic(::core::mem::transmute_copy(&pisperiodic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScheduled<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisscheduled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsScheduled(::core::mem::transmute_copy(&pisscheduled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScheduleAllowed<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisscheduleallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsScheduleAllowed(::core::mem::transmute_copy(&pisscheduleallowed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Description(&*(&pdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLaunchOnBoot<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plaunchonboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLaunchOnBoot(::core::mem::transmute_copy(&plaunchonboot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsScheduled<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isscheduled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_IsScheduled(&*(&isscheduled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsScheduleAllowed<Impl: IPMBackgroundServiceAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isscheduleallowed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_IsScheduleAllowed(&*(&isscheduleallowed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPMBackgroundServiceAgentInfo>,
            base.5,
            ProductID::<Impl, OFFSET>,
            TaskID::<Impl, OFFSET>,
            BSAID::<Impl, OFFSET>,
            BGSpecifier::<Impl, OFFSET>,
            BGName::<Impl, OFFSET>,
            BGSource::<Impl, OFFSET>,
            BGType::<Impl, OFFSET>,
            IsPeriodic::<Impl, OFFSET>,
            IsScheduled::<Impl, OFFSET>,
            IsScheduleAllowed::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            IsLaunchOnBoot::<Impl, OFFSET>,
            set_IsScheduled::<Impl, OFFSET>,
            set_IsScheduleAllowed::<Impl, OFFSET>,
        )
    }
}
pub trait IPMBackgroundServiceAgentInfoEnumeratorImpl: Sized {
    fn Next();
}
impl ::windows::core::RuntimeName for IPMBackgroundServiceAgentInfoEnumerator {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMBackgroundServiceAgentInfoEnumerator";
}
impl IPMBackgroundServiceAgentInfoEnumeratorVtbl {
    pub const fn new<Impl: IPMBackgroundServiceAgentInfoEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMBackgroundServiceAgentInfoEnumeratorVtbl {
        unsafe extern "system" fn Next<Impl: IPMBackgroundServiceAgentInfoEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbsainfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&ppbsainfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMBackgroundServiceAgentInfoEnumerator>, base.5, Next::<Impl, OFFSET>)
    }
}
pub trait IPMBackgroundWorkerInfoImpl: Sized {
    fn ProductID();
    fn TaskID();
    fn BGName();
    fn MaxStartupLatency();
    fn ExpectedRuntime();
    fn IsBootWorker();
}
impl ::windows::core::RuntimeName for IPMBackgroundWorkerInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMBackgroundWorkerInfo";
}
impl IPMBackgroundWorkerInfoVtbl {
    pub const fn new<Impl: IPMBackgroundWorkerInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMBackgroundWorkerInfoVtbl {
        unsafe extern "system" fn ProductID<Impl: IPMBackgroundWorkerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProductID(::core::mem::transmute_copy(&pproductid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Impl: IPMBackgroundWorkerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskID(&*(&ptaskid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BGName<Impl: IPMBackgroundWorkerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BGName(&*(&pbgname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxStartupLatency<Impl: IPMBackgroundWorkerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaxstartuplatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxStartupLatency(::core::mem::transmute_copy(&pmaxstartuplatency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedRuntime<Impl: IPMBackgroundWorkerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexpectedruntime: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpectedRuntime(::core::mem::transmute_copy(&pexpectedruntime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBootWorker<Impl: IPMBackgroundWorkerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisbootworker: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBootWorker(::core::mem::transmute_copy(&pisbootworker)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMBackgroundWorkerInfo>, base.5, ProductID::<Impl, OFFSET>, TaskID::<Impl, OFFSET>, BGName::<Impl, OFFSET>, MaxStartupLatency::<Impl, OFFSET>, ExpectedRuntime::<Impl, OFFSET>, IsBootWorker::<Impl, OFFSET>)
    }
}
pub trait IPMBackgroundWorkerInfoEnumeratorImpl: Sized {
    fn Next();
}
impl ::windows::core::RuntimeName for IPMBackgroundWorkerInfoEnumerator {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMBackgroundWorkerInfoEnumerator";
}
impl IPMBackgroundWorkerInfoEnumeratorVtbl {
    pub const fn new<Impl: IPMBackgroundWorkerInfoEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMBackgroundWorkerInfoEnumeratorVtbl {
        unsafe extern "system" fn Next<Impl: IPMBackgroundWorkerInfoEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbwinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&ppbwinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMBackgroundWorkerInfoEnumerator>, base.5, Next::<Impl, OFFSET>)
    }
}
pub trait IPMDeploymentManagerImpl: Sized {
    fn ReportDownloadBegin();
    fn ReportDownloadProgress();
    fn ReportDownloadComplete();
    fn BeginInstall();
    fn BeginUpdate();
    fn BeginDeployPackage();
    fn BeginUpdateDeployedPackageLegacy();
    fn BeginUninstall();
    fn BeginEnterpriseAppInstall();
    fn BeginEnterpriseAppUpdate();
    fn BeginUpdateLicense();
    fn GetLicenseChallenge();
    fn GetLicenseChallengeByProductID();
    fn GetLicenseChallengeByProductID2();
    fn RevokeLicense();
    fn RebindMdilBinaries();
    fn RebindAllMdilBinaries();
    fn RegenerateXbf();
    fn GenerateXbfForCurrentLocale();
    fn BeginProvision();
    fn BeginDeprovision();
    fn ReindexSQLCEDatabases();
    fn SetApplicationsNeedMaintenance();
    fn UpdateChamberProfile();
    fn EnterprisePolicyIsApplicationAllowed();
    fn BeginUpdateDeployedPackage();
    fn ReportRestoreCancelled();
    fn ResolveResourceString();
    fn UpdateCapabilitiesForModernApps();
    fn ReportDownloadStatusUpdate();
    fn BeginUninstallWithOptions();
    fn BindDeferredMdilBinaries();
    fn GenerateXamlLightupXbfForCurrentLocale();
    fn AddLicenseForAppx();
    fn FixJunctionsForAppsOnSDCard();
}
impl ::windows::core::RuntimeName for IPMDeploymentManager {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMDeploymentManager";
}
impl IPMDeploymentManagerVtbl {
    pub const fn new<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMDeploymentManagerVtbl {
        unsafe extern "system" fn ReportDownloadBegin<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportDownloadBegin(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportDownloadProgress<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, usprogress: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportDownloadProgress(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), usprogress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportDownloadComplete<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportDownloadComplete(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), hrresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginInstall<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginInstall(&*(&pinstallinfo as *const <PM_INSTALLINFO as ::windows::core::Abi>::Abi as *const <PM_INSTALLINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUpdate<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginUpdate(&*(&pupdateinfo as *const <PM_UPDATEINFO as ::windows::core::Abi>::Abi as *const <PM_UPDATEINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDeployPackage<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginDeployPackage(&*(&pinstallinfo as *const <PM_INSTALLINFO as ::windows::core::Abi>::Abi as *const <PM_INSTALLINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUpdateDeployedPackageLegacy<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginUpdateDeployedPackageLegacy(&*(&pupdateinfo as *const <PM_UPDATEINFO_LEGACY as ::windows::core::Abi>::Abi as *const <PM_UPDATEINFO_LEGACY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUninstall<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginUninstall(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnterpriseAppInstall<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginEnterpriseAppInstall(&*(&pinstallinfo as *const <PM_INSTALLINFO as ::windows::core::Abi>::Abi as *const <PM_INSTALLINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnterpriseAppUpdate<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginEnterpriseAppUpdate(&*(&pupdateinfo as *const <PM_UPDATEINFO as ::windows::core::Abi>::Abi as *const <PM_UPDATEINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUpdateLicense<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, offerid: ::windows::core::GUID, pblicense: *const u8, cblicense: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginUpdateLicense(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&offerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pblicense, cblicense) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLicenseChallenge<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLicenseChallenge(&*(&packagepath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcbchallenge), ppbkid, pcbkid, ppbdeviceid, pcbdeviceid, ppbsaltvalue, pcbsaltvalue, ppbkgvvalue, pcbkgvvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLicenseChallengeByProductID(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcblicense)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID2<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLicenseChallengeByProductID2(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcblicense), ppbkid, pcbkid, ppbdeviceid, pcbdeviceid, ppbsaltvalue, pcbsaltvalue, ppbkgvvalue, pcbkgvvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeLicense<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevokeLicense(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebindMdilBinaries<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RebindMdilBinaries(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&filenames as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebindAllMdilBinaries<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, instanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RebindAllMdilBinaries(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&instanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegenerateXbf<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegenerateXbf(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&assemblypaths as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateXbfForCurrentLocale<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateXbfForCurrentLocale(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginProvision<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, xmlpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginProvision(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&xmlpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDeprovision<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginDeprovision(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReindexSQLCEDatabases<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReindexSQLCEDatabases(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationsNeedMaintenance<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requiredmaintenanceoperations: u32, pcapplications: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetApplicationsNeedMaintenance(requiredmaintenanceoperations, ::core::mem::transmute_copy(&pcapplications)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateChamberProfile<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateChamberProfile(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterprisePolicyIsApplicationAllowed<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, publishername: super::super::Foundation::PWSTR, pisallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnterprisePolicyIsApplicationAllowed(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&publishername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pisallowed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUpdateDeployedPackage<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginUpdateDeployedPackage(&*(&pupdateinfo as *const <PM_UPDATEINFO as ::windows::core::Abi>::Abi as *const <PM_UPDATEINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportRestoreCancelled<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportRestoreCancelled(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveResourceString<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcestring: super::super::Foundation::PWSTR, presolvedresourcestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResolveResourceString(&*(&resourcestring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&presolvedresourcestring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateCapabilitiesForModernApps<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateCapabilitiesForModernApps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportDownloadStatusUpdate<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportDownloadStatusUpdate(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUninstallWithOptions<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, removaloptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginUninstallWithOptions(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), removaloptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindDeferredMdilBinaries<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BindDeferredMdilBinaries() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateXamlLightupXbfForCurrentLocale<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateXamlLightupXbfForCurrentLocale(&*(&packagefamilyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLicenseForAppx<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddLicenseForAppx(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pblicense, cblicense, pbplayreadyheader, cbplayreadyheader) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FixJunctionsForAppsOnSDCard<Impl: IPMDeploymentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FixJunctionsForAppsOnSDCard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPMDeploymentManager>,
            base.5,
            ReportDownloadBegin::<Impl, OFFSET>,
            ReportDownloadProgress::<Impl, OFFSET>,
            ReportDownloadComplete::<Impl, OFFSET>,
            BeginInstall::<Impl, OFFSET>,
            BeginUpdate::<Impl, OFFSET>,
            BeginDeployPackage::<Impl, OFFSET>,
            BeginUpdateDeployedPackageLegacy::<Impl, OFFSET>,
            BeginUninstall::<Impl, OFFSET>,
            BeginEnterpriseAppInstall::<Impl, OFFSET>,
            BeginEnterpriseAppUpdate::<Impl, OFFSET>,
            BeginUpdateLicense::<Impl, OFFSET>,
            GetLicenseChallenge::<Impl, OFFSET>,
            GetLicenseChallengeByProductID::<Impl, OFFSET>,
            GetLicenseChallengeByProductID2::<Impl, OFFSET>,
            RevokeLicense::<Impl, OFFSET>,
            RebindMdilBinaries::<Impl, OFFSET>,
            RebindAllMdilBinaries::<Impl, OFFSET>,
            RegenerateXbf::<Impl, OFFSET>,
            GenerateXbfForCurrentLocale::<Impl, OFFSET>,
            BeginProvision::<Impl, OFFSET>,
            BeginDeprovision::<Impl, OFFSET>,
            ReindexSQLCEDatabases::<Impl, OFFSET>,
            SetApplicationsNeedMaintenance::<Impl, OFFSET>,
            UpdateChamberProfile::<Impl, OFFSET>,
            EnterprisePolicyIsApplicationAllowed::<Impl, OFFSET>,
            BeginUpdateDeployedPackage::<Impl, OFFSET>,
            ReportRestoreCancelled::<Impl, OFFSET>,
            ResolveResourceString::<Impl, OFFSET>,
            UpdateCapabilitiesForModernApps::<Impl, OFFSET>,
            ReportDownloadStatusUpdate::<Impl, OFFSET>,
            BeginUninstallWithOptions::<Impl, OFFSET>,
            BindDeferredMdilBinaries::<Impl, OFFSET>,
            GenerateXamlLightupXbfForCurrentLocale::<Impl, OFFSET>,
            AddLicenseForAppx::<Impl, OFFSET>,
            FixJunctionsForAppsOnSDCard::<Impl, OFFSET>,
        )
    }
}
pub trait IPMEnumerationManagerImpl: Sized {
    fn AllApplications();
    fn AllTiles();
    fn AllTasks();
    fn AllExtensions();
    fn AllBackgroundServiceAgents();
    fn AllBackgroundWorkers();
    fn ApplicationInfo();
    fn TileInfo();
    fn TaskInfo();
    fn TaskInfoEx();
    fn BackgroundServiceAgentInfo();
    fn AllLiveTileJobs();
    fn LiveTileJob();
    fn ApplicationInfoExternal();
    fn FileHandlerGenericLogo();
    fn ApplicationInfoFromAccessClaims();
    fn StartTileEnumeratorBlob();
    fn StartAppEnumeratorBlob();
}
impl ::windows::core::RuntimeName for IPMEnumerationManager {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMEnumerationManager";
}
impl IPMEnumerationManagerVtbl {
    pub const fn new<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMEnumerationManagerVtbl {
        unsafe extern "system" fn AllApplications<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppappenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllApplications(::core::mem::transmute_copy(&ppappenum), &*(&filter as *const <PM_ENUM_FILTER as ::windows::core::Abi>::Abi as *const <PM_ENUM_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllTiles<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptileenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllTiles(::core::mem::transmute_copy(&pptileenum), &*(&filter as *const <PM_ENUM_FILTER as ::windows::core::Abi>::Abi as *const <PM_ENUM_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllTasks<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptaskenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllTasks(::core::mem::transmute_copy(&pptaskenum), &*(&filter as *const <PM_ENUM_FILTER as ::windows::core::Abi>::Abi as *const <PM_ENUM_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllExtensions<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppextensionenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllExtensions(::core::mem::transmute_copy(&ppextensionenum), &*(&filter as *const <PM_ENUM_FILTER as ::windows::core::Abi>::Abi as *const <PM_ENUM_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllBackgroundServiceAgents<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbsaenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllBackgroundServiceAgents(::core::mem::transmute_copy(&ppbsaenum), &*(&filter as *const <PM_ENUM_FILTER as ::windows::core::Abi>::Abi as *const <PM_ENUM_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllBackgroundWorkers<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbswenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllBackgroundWorkers(::core::mem::transmute_copy(&ppbswenum), &*(&filter as *const <PM_ENUM_FILTER as ::windows::core::Abi>::Abi as *const <PM_ENUM_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationInfo<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationInfo(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppappinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileInfo<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, tileid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptileinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TileInfo(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&tileid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptileinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskInfo<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, taskid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskInfo(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&taskid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptaskinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskInfoEx<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, taskid: super::super::Foundation::PWSTR, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskInfoEx(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&taskid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptaskinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundServiceAgentInfo<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsaid: u32, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackgroundServiceAgentInfo(bsaid, ::core::mem::transmute_copy(&pptaskinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllLiveTileJobs<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplivetilejobenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllLiveTileJobs(::core::mem::transmute_copy(&pplivetilejobenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LiveTileJob<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, tileid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE, pplivetilejobinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LiveTileJob(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&tileid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), recurrencetype, ::core::mem::transmute_copy(&pplivetilejobinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationInfoExternal<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationInfoExternal(&*(&productid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppappinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileHandlerGenericLogo<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileHandlerGenericLogo(&*(&filetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), logosize, &*(&plogo as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationInfoFromAccessClaims<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sysappid0: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sysappid1: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationInfoFromAccessClaims(&*(&sysappid0 as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&sysappid1 as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppappinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTileEnumeratorBlob<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartTileEnumeratorBlob(&*(&filter as *const <PM_ENUM_FILTER as ::windows::core::Abi>::Abi as *const <PM_ENUM_FILTER as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pctiles), ::core::mem::transmute_copy(&pptileblobs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAppEnumeratorBlob<Impl: IPMEnumerationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAppEnumeratorBlob(&*(&filter as *const <PM_ENUM_FILTER as ::windows::core::Abi>::Abi as *const <PM_ENUM_FILTER as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcapps), ::core::mem::transmute_copy(&ppappblobs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPMEnumerationManager>,
            base.5,
            AllApplications::<Impl, OFFSET>,
            AllTiles::<Impl, OFFSET>,
            AllTasks::<Impl, OFFSET>,
            AllExtensions::<Impl, OFFSET>,
            AllBackgroundServiceAgents::<Impl, OFFSET>,
            AllBackgroundWorkers::<Impl, OFFSET>,
            ApplicationInfo::<Impl, OFFSET>,
            TileInfo::<Impl, OFFSET>,
            TaskInfo::<Impl, OFFSET>,
            TaskInfoEx::<Impl, OFFSET>,
            BackgroundServiceAgentInfo::<Impl, OFFSET>,
            AllLiveTileJobs::<Impl, OFFSET>,
            LiveTileJob::<Impl, OFFSET>,
            ApplicationInfoExternal::<Impl, OFFSET>,
            FileHandlerGenericLogo::<Impl, OFFSET>,
            ApplicationInfoFromAccessClaims::<Impl, OFFSET>,
            StartTileEnumeratorBlob::<Impl, OFFSET>,
            StartAppEnumeratorBlob::<Impl, OFFSET>,
        )
    }
}
pub trait IPMExtensionCachedFileUpdaterInfoImpl: Sized {
    fn SupportsUpdates();
}
impl ::windows::core::RuntimeName for IPMExtensionCachedFileUpdaterInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMExtensionCachedFileUpdaterInfo";
}
impl IPMExtensionCachedFileUpdaterInfoVtbl {
    pub const fn new<Impl: IPMExtensionCachedFileUpdaterInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMExtensionCachedFileUpdaterInfoVtbl {
        unsafe extern "system" fn SupportsUpdates<Impl: IPMExtensionCachedFileUpdaterInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupportsupdates: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportsUpdates(::core::mem::transmute_copy(&psupportsupdates)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMExtensionCachedFileUpdaterInfo>, base.5, SupportsUpdates::<Impl, OFFSET>)
    }
}
pub trait IPMExtensionContractInfoImpl: Sized {
    fn InvocationInfo();
}
impl ::windows::core::RuntimeName for IPMExtensionContractInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMExtensionContractInfo";
}
impl IPMExtensionContractInfoVtbl {
    pub const fn new<Impl: IPMExtensionContractInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMExtensionContractInfoVtbl {
        unsafe extern "system" fn InvocationInfo<Impl: IPMExtensionContractInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paumid: *mut super::super::Foundation::BSTR, pargs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvocationInfo(&*(&paumid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pargs as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMExtensionContractInfo>, base.5, InvocationInfo::<Impl, OFFSET>)
    }
}
pub trait IPMExtensionFileExtensionInfoImpl: Sized {
    fn Name();
    fn DisplayName();
    fn Logo();
    fn ContentType();
    fn FileType();
    fn InvocationInfo();
    fn AllFileTypes();
}
impl ::windows::core::RuntimeName for IPMExtensionFileExtensionInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMExtensionFileExtensionInfo";
}
impl IPMExtensionFileExtensionInfoVtbl {
    pub const fn new<Impl: IPMExtensionFileExtensionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMExtensionFileExtensionInfoVtbl {
        unsafe extern "system" fn Name<Impl: IPMExtensionFileExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(&*(&pname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IPMExtensionFileExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName(&*(&pdisplayname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Logo<Impl: IPMExtensionFileExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Logo(logosize, &*(&plogo as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentType<Impl: IPMExtensionFileExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontenttype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentType(&*(&filetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcontenttype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileType<Impl: IPMExtensionFileExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfiletype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileType(&*(&contenttype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pfiletype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMExtensionFileExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvocationInfo(&*(&pimageurn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pparameters as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllFileTypes<Impl: IPMExtensionFileExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbtypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllFileTypes(::core::mem::transmute_copy(&pcbtypes), ::core::mem::transmute_copy(&pptypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMExtensionFileExtensionInfo>, base.5, Name::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, Logo::<Impl, OFFSET>, ContentType::<Impl, OFFSET>, FileType::<Impl, OFFSET>, InvocationInfo::<Impl, OFFSET>, AllFileTypes::<Impl, OFFSET>)
    }
}
pub trait IPMExtensionFileOpenPickerInfoImpl: Sized {
    fn AllFileTypes();
    fn SupportsAllFileTypes();
}
impl ::windows::core::RuntimeName for IPMExtensionFileOpenPickerInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMExtensionFileOpenPickerInfo";
}
impl IPMExtensionFileOpenPickerInfoVtbl {
    pub const fn new<Impl: IPMExtensionFileOpenPickerInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMExtensionFileOpenPickerInfoVtbl {
        unsafe extern "system" fn AllFileTypes<Impl: IPMExtensionFileOpenPickerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsAllFileTypes<Impl: IPMExtensionFileOpenPickerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportsAllFileTypes(::core::mem::transmute_copy(&psupportsalltypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMExtensionFileOpenPickerInfo>, base.5, AllFileTypes::<Impl, OFFSET>, SupportsAllFileTypes::<Impl, OFFSET>)
    }
}
pub trait IPMExtensionFileSavePickerInfoImpl: Sized {
    fn AllFileTypes();
    fn SupportsAllFileTypes();
}
impl ::windows::core::RuntimeName for IPMExtensionFileSavePickerInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMExtensionFileSavePickerInfo";
}
impl IPMExtensionFileSavePickerInfoVtbl {
    pub const fn new<Impl: IPMExtensionFileSavePickerInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMExtensionFileSavePickerInfoVtbl {
        unsafe extern "system" fn AllFileTypes<Impl: IPMExtensionFileSavePickerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsAllFileTypes<Impl: IPMExtensionFileSavePickerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportsAllFileTypes(::core::mem::transmute_copy(&psupportsalltypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMExtensionFileSavePickerInfo>, base.5, AllFileTypes::<Impl, OFFSET>, SupportsAllFileTypes::<Impl, OFFSET>)
    }
}
pub trait IPMExtensionInfoImpl: Sized {
    fn SupplierPID();
    fn SupplierTaskID();
    fn Title();
    fn IconPath();
    fn ExtraFile();
    fn InvocationInfo();
}
impl ::windows::core::RuntimeName for IPMExtensionInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMExtensionInfo";
}
impl IPMExtensionInfoVtbl {
    pub const fn new<Impl: IPMExtensionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMExtensionInfoVtbl {
        unsafe extern "system" fn SupplierPID<Impl: IPMExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupplierpid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupplierPID(::core::mem::transmute_copy(&psupplierpid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupplierTaskID<Impl: IPMExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psuppliertid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupplierTaskID(&*(&psuppliertid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IPMExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title(&*(&ptitle as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconPath<Impl: IPMExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IconPath(&*(&piconpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtraFile<Impl: IPMExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtraFile(&*(&pfilepath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMExtensionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvocationInfo(&*(&pimageurn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pparameters as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMExtensionInfo>, base.5, SupplierPID::<Impl, OFFSET>, SupplierTaskID::<Impl, OFFSET>, Title::<Impl, OFFSET>, IconPath::<Impl, OFFSET>, ExtraFile::<Impl, OFFSET>, InvocationInfo::<Impl, OFFSET>)
    }
}
pub trait IPMExtensionInfoEnumeratorImpl: Sized {
    fn Next();
}
impl ::windows::core::RuntimeName for IPMExtensionInfoEnumerator {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMExtensionInfoEnumerator";
}
impl IPMExtensionInfoEnumeratorVtbl {
    pub const fn new<Impl: IPMExtensionInfoEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMExtensionInfoEnumeratorVtbl {
        unsafe extern "system" fn Next<Impl: IPMExtensionInfoEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppextensioninfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&ppextensioninfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMExtensionInfoEnumerator>, base.5, Next::<Impl, OFFSET>)
    }
}
pub trait IPMExtensionProtocolInfoImpl: Sized {
    fn Protocol();
    fn InvocationInfo();
}
impl ::windows::core::RuntimeName for IPMExtensionProtocolInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMExtensionProtocolInfo";
}
impl IPMExtensionProtocolInfoVtbl {
    pub const fn new<Impl: IPMExtensionProtocolInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMExtensionProtocolInfoVtbl {
        unsafe extern "system" fn Protocol<Impl: IPMExtensionProtocolInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Protocol(&*(&pprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMExtensionProtocolInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvocationInfo(&*(&pimageurn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pparameters as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMExtensionProtocolInfo>, base.5, Protocol::<Impl, OFFSET>, InvocationInfo::<Impl, OFFSET>)
    }
}
pub trait IPMExtensionShareTargetInfoImpl: Sized {
    fn AllFileTypes();
    fn AllDataFormats();
    fn SupportsAllFileTypes();
}
impl ::windows::core::RuntimeName for IPMExtensionShareTargetInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMExtensionShareTargetInfo";
}
impl IPMExtensionShareTargetInfoVtbl {
    pub const fn new<Impl: IPMExtensionShareTargetInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMExtensionShareTargetInfoVtbl {
        unsafe extern "system" fn AllFileTypes<Impl: IPMExtensionShareTargetInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllDataFormats<Impl: IPMExtensionShareTargetInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdataformats: *mut u32, ppdataformats: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllDataFormats(::core::mem::transmute_copy(&pcdataformats), ::core::mem::transmute_copy(&ppdataformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsAllFileTypes<Impl: IPMExtensionShareTargetInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportsAllFileTypes(::core::mem::transmute_copy(&psupportsalltypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMExtensionShareTargetInfo>, base.5, AllFileTypes::<Impl, OFFSET>, AllDataFormats::<Impl, OFFSET>, SupportsAllFileTypes::<Impl, OFFSET>)
    }
}
pub trait IPMLiveTileJobInfoImpl: Sized {
    fn ProductID();
    fn TileID();
    fn NextSchedule();
    fn set_NextSchedule();
    fn StartSchedule();
    fn set_StartSchedule();
    fn IntervalDuration();
    fn set_IntervalDuration();
    fn RunForever();
    fn set_RunForever();
    fn MaxRunCount();
    fn set_MaxRunCount();
    fn RunCount();
    fn set_RunCount();
    fn RecurrenceType();
    fn set_RecurrenceType();
    fn TileXML();
    fn set_TileXML();
    fn UrlXML();
    fn set_UrlXML();
    fn AttemptCount();
    fn set_AttemptCount();
    fn DownloadState();
    fn set_DownloadState();
}
impl ::windows::core::RuntimeName for IPMLiveTileJobInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMLiveTileJobInfo";
}
impl IPMLiveTileJobInfoVtbl {
    pub const fn new<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMLiveTileJobInfoVtbl {
        unsafe extern "system" fn ProductID<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProductID(::core::mem::transmute_copy(&pproductid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileID<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TileID(&*(&ptileid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextSchedule<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnextschedule: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextSchedule(::core::mem::transmute_copy(&pnextschedule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_NextSchedule<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ftnextschedule: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_NextSchedule(&*(&ftnextschedule as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartSchedule<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstartschedule: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartSchedule(::core::mem::transmute_copy(&pstartschedule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_StartSchedule<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ftstartschedule: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_StartSchedule(&*(&ftstartschedule as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntervalDuration<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pintervalduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IntervalDuration(::core::mem::transmute_copy(&pintervalduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IntervalDuration<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulintervalduration: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_IntervalDuration(ulintervalduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunForever<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isrunforever: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunForever(::core::mem::transmute_copy(&isrunforever)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RunForever<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frunforever: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_RunForever(&*(&frunforever as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxRunCount<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaxruncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxRunCount(::core::mem::transmute_copy(&pmaxruncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_MaxRunCount<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulmaxruncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_MaxRunCount(ulmaxruncount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunCount<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pruncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunCount(::core::mem::transmute_copy(&pruncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RunCount<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulruncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_RunCount(ulruncount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecurrenceType<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precurrencetype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecurrenceType(::core::mem::transmute_copy(&precurrencetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RecurrenceType<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulrecurrencetype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_RecurrenceType(ulrecurrencetype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileXML<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TileXML(::core::mem::transmute_copy(&ptilexml), ::core::mem::transmute_copy(&pcbtilexml)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_TileXML<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptilexml: *const u8, cbtilexml: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_TileXML(ptilexml, cbtilexml) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UrlXML<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UrlXML(::core::mem::transmute_copy(&purlxml), ::core::mem::transmute_copy(&pcburlxml)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_UrlXML<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, purlxml: *const u8, cburlxml: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_UrlXML(purlxml, cburlxml) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttemptCount<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattemptcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AttemptCount(::core::mem::transmute_copy(&pattemptcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_AttemptCount<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulattemptcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_AttemptCount(ulattemptcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadState<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdownloadstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DownloadState(::core::mem::transmute_copy(&pdownloadstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_DownloadState<Impl: IPMLiveTileJobInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uldownloadstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_DownloadState(uldownloadstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPMLiveTileJobInfo>,
            base.5,
            ProductID::<Impl, OFFSET>,
            TileID::<Impl, OFFSET>,
            NextSchedule::<Impl, OFFSET>,
            set_NextSchedule::<Impl, OFFSET>,
            StartSchedule::<Impl, OFFSET>,
            set_StartSchedule::<Impl, OFFSET>,
            IntervalDuration::<Impl, OFFSET>,
            set_IntervalDuration::<Impl, OFFSET>,
            RunForever::<Impl, OFFSET>,
            set_RunForever::<Impl, OFFSET>,
            MaxRunCount::<Impl, OFFSET>,
            set_MaxRunCount::<Impl, OFFSET>,
            RunCount::<Impl, OFFSET>,
            set_RunCount::<Impl, OFFSET>,
            RecurrenceType::<Impl, OFFSET>,
            set_RecurrenceType::<Impl, OFFSET>,
            TileXML::<Impl, OFFSET>,
            set_TileXML::<Impl, OFFSET>,
            UrlXML::<Impl, OFFSET>,
            set_UrlXML::<Impl, OFFSET>,
            AttemptCount::<Impl, OFFSET>,
            set_AttemptCount::<Impl, OFFSET>,
            DownloadState::<Impl, OFFSET>,
            set_DownloadState::<Impl, OFFSET>,
        )
    }
}
pub trait IPMLiveTileJobInfoEnumeratorImpl: Sized {
    fn Next();
}
impl ::windows::core::RuntimeName for IPMLiveTileJobInfoEnumerator {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMLiveTileJobInfoEnumerator";
}
impl IPMLiveTileJobInfoEnumeratorVtbl {
    pub const fn new<Impl: IPMLiveTileJobInfoEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMLiveTileJobInfoEnumeratorVtbl {
        unsafe extern "system" fn Next<Impl: IPMLiveTileJobInfoEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplivetilejobinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&pplivetilejobinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMLiveTileJobInfoEnumerator>, base.5, Next::<Impl, OFFSET>)
    }
}
pub trait IPMTaskInfoImpl: Sized {
    fn ProductID();
    fn TaskID();
    fn NavigationPage();
    fn TaskTransition();
    fn RuntimeType();
    fn ActivationPolicy();
    fn TaskType();
    fn InvocationInfo();
    fn ImagePath();
    fn ImageParams();
    fn InstallRootFolder();
    fn DataRootFolder();
    fn IsSingleInstanceHost();
    fn IsInteropEnabled();
    fn ApplicationState();
    fn InstallType();
    fn Version();
    fn BitsPerPixel();
    fn SuppressesDehydration();
    fn BackgroundExecutionAbilities();
    fn IsOptedForExtendedMem();
}
impl ::windows::core::RuntimeName for IPMTaskInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMTaskInfo";
}
impl IPMTaskInfoVtbl {
    pub const fn new<Impl: IPMTaskInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMTaskInfoVtbl {
        unsafe extern "system" fn ProductID<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProductID(::core::mem::transmute_copy(&pproductid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskID(&*(&ptaskid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavigationPage<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnavigationpage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigationPage(&*(&pnavigationpage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskTransition<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptasktransition: *mut PM_TASK_TRANSITION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskTransition(::core::mem::transmute_copy(&ptasktransition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuntimeType<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pruntimetype: *mut PACKMAN_RUNTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RuntimeType(::core::mem::transmute_copy(&pruntimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivationPolicy<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pactivationpolicy: *mut PM_ACTIVATION_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivationPolicy(::core::mem::transmute_copy(&pactivationpolicy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskType<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptasktype: *mut PM_TASK_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskType(::core::mem::transmute_copy(&ptasktype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvocationInfo(&*(&pimageurn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pparameters as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImagePath<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimagepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImagePath(&*(&pimagepath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageParams<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageparams: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageParams(&*(&pimageparams as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallRootFolder<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstallrootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstallRootFolder(&*(&pinstallrootfolder as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataRootFolder<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatarootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataRootFolder(&*(&pdatarootfolder as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSingleInstanceHost<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pissingleinstancehost: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSingleInstanceHost(::core::mem::transmute_copy(&pissingleinstancehost)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInteropEnabled<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisinteropenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInteropEnabled(::core::mem::transmute_copy(&pisinteropenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationState<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papplicationstate: *mut PM_APPLICATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationState(::core::mem::transmute_copy(&papplicationstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallType<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstallType(::core::mem::transmute_copy(&pinstalltype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Version(::core::mem::transmute_copy(&ptargetmajorversion), ::core::mem::transmute_copy(&ptargetminorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitsPerPixel<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitsperpixel: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitsPerPixel(::core::mem::transmute_copy(&pbitsperpixel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuppressesDehydration<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psuppressesdehydration: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuppressesDehydration(::core::mem::transmute_copy(&psuppressesdehydration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundExecutionAbilities<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbackgroundexecutionabilities: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackgroundExecutionAbilities(&*(&pbackgroundexecutionabilities as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptedForExtendedMem<Impl: IPMTaskInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisoptedin: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOptedForExtendedMem(::core::mem::transmute_copy(&pisoptedin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPMTaskInfo>,
            base.5,
            ProductID::<Impl, OFFSET>,
            TaskID::<Impl, OFFSET>,
            NavigationPage::<Impl, OFFSET>,
            TaskTransition::<Impl, OFFSET>,
            RuntimeType::<Impl, OFFSET>,
            ActivationPolicy::<Impl, OFFSET>,
            TaskType::<Impl, OFFSET>,
            InvocationInfo::<Impl, OFFSET>,
            ImagePath::<Impl, OFFSET>,
            ImageParams::<Impl, OFFSET>,
            InstallRootFolder::<Impl, OFFSET>,
            DataRootFolder::<Impl, OFFSET>,
            IsSingleInstanceHost::<Impl, OFFSET>,
            IsInteropEnabled::<Impl, OFFSET>,
            ApplicationState::<Impl, OFFSET>,
            InstallType::<Impl, OFFSET>,
            Version::<Impl, OFFSET>,
            BitsPerPixel::<Impl, OFFSET>,
            SuppressesDehydration::<Impl, OFFSET>,
            BackgroundExecutionAbilities::<Impl, OFFSET>,
            IsOptedForExtendedMem::<Impl, OFFSET>,
        )
    }
}
pub trait IPMTaskInfoEnumeratorImpl: Sized {
    fn Next();
}
impl ::windows::core::RuntimeName for IPMTaskInfoEnumerator {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMTaskInfoEnumerator";
}
impl IPMTaskInfoEnumeratorVtbl {
    pub const fn new<Impl: IPMTaskInfoEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMTaskInfoEnumeratorVtbl {
        unsafe extern "system" fn Next<Impl: IPMTaskInfoEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&pptaskinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMTaskInfoEnumerator>, base.5, Next::<Impl, OFFSET>)
    }
}
pub trait IPMTileInfoImpl: Sized {
    fn ProductID();
    fn TileID();
    fn TemplateType();
    fn HubPinnedState();
    fn HubPosition();
    fn IsNotified();
    fn IsDefault();
    fn TaskID();
    fn TileType();
    fn IsThemable();
    fn PropertyById();
    fn InvocationInfo();
    fn PropertyEnum();
    fn HubTileSize();
    fn set_HubPosition();
    fn set_NotifiedState();
    fn set_HubPinnedState();
    fn set_HubTileSize();
    fn set_InvocationInfo();
    fn StartTileBlob();
    fn IsRestoring();
    fn IsAutoRestoreDisabled();
    fn set_IsRestoring();
    fn set_IsAutoRestoreDisabled();
}
impl ::windows::core::RuntimeName for IPMTileInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMTileInfo";
}
impl IPMTileInfoVtbl {
    pub const fn new<Impl: IPMTileInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMTileInfoVtbl {
        unsafe extern "system" fn ProductID<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProductID(::core::mem::transmute_copy(&pproductid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileID<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TileID(&*(&ptileid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TemplateType<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptemplatetype: *mut TILE_TEMPLATE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TemplateType(::core::mem::transmute_copy(&ptemplatetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HubPinnedState<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, ppinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HubPinnedState(hubtype, ::core::mem::transmute_copy(&ppinned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HubPosition<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HubPosition(hubtype, ::core::mem::transmute_copy(&pposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNotified<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsNotified(::core::mem::transmute_copy(&pisnotified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisdefault: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDefault(::core::mem::transmute_copy(&pisdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskID(&*(&ptaskid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileType<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstarttiletype: *mut PM_STARTTILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TileType(::core::mem::transmute_copy(&pstarttiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThemable<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsThemable(::core::mem::transmute_copy(&pisthemable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyById<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32, pppropinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PropertyById(propid, ::core::mem::transmute_copy(&pppropinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvocationInfo(&*(&pimageurn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pparameters as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyEnum<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptilepropenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PropertyEnum(::core::mem::transmute_copy(&pptilepropenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HubTileSize<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, psize: *mut PM_TILE_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HubTileSize(hubtype, ::core::mem::transmute_copy(&psize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_HubPosition<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_HubPosition(hubtype, position) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_NotifiedState<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notified: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_NotifiedState(&*(&notified as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_HubPinnedState<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_HubPinnedState(hubtype, &*(&pinned as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_HubTileSize<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_HubTileSize(hubtype, size) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_InvocationInfo<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskparameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_InvocationInfo(&*(&taskname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&taskparameters as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTileBlob<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTTILEBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartTileBlob(&*(&pblob as *const <PM_STARTTILEBLOB as ::windows::core::Abi>::Abi as *const <PM_STARTTILEBLOB as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRestoring<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisrestoring: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRestoring(::core::mem::transmute_copy(&pisrestoring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAutoRestoreDisabled<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisautorestoredisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAutoRestoreDisabled(::core::mem::transmute_copy(&pisautorestoredisabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsRestoring<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restoring: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_IsRestoring(&*(&restoring as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsAutoRestoreDisabled<Impl: IPMTileInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, autorestoredisabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_IsAutoRestoreDisabled(&*(&autorestoredisabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPMTileInfo>,
            base.5,
            ProductID::<Impl, OFFSET>,
            TileID::<Impl, OFFSET>,
            TemplateType::<Impl, OFFSET>,
            HubPinnedState::<Impl, OFFSET>,
            HubPosition::<Impl, OFFSET>,
            IsNotified::<Impl, OFFSET>,
            IsDefault::<Impl, OFFSET>,
            TaskID::<Impl, OFFSET>,
            TileType::<Impl, OFFSET>,
            IsThemable::<Impl, OFFSET>,
            PropertyById::<Impl, OFFSET>,
            InvocationInfo::<Impl, OFFSET>,
            PropertyEnum::<Impl, OFFSET>,
            HubTileSize::<Impl, OFFSET>,
            set_HubPosition::<Impl, OFFSET>,
            set_NotifiedState::<Impl, OFFSET>,
            set_HubPinnedState::<Impl, OFFSET>,
            set_HubTileSize::<Impl, OFFSET>,
            set_InvocationInfo::<Impl, OFFSET>,
            StartTileBlob::<Impl, OFFSET>,
            IsRestoring::<Impl, OFFSET>,
            IsAutoRestoreDisabled::<Impl, OFFSET>,
            set_IsRestoring::<Impl, OFFSET>,
            set_IsAutoRestoreDisabled::<Impl, OFFSET>,
        )
    }
}
pub trait IPMTileInfoEnumeratorImpl: Sized {
    fn Next();
}
impl ::windows::core::RuntimeName for IPMTileInfoEnumerator {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMTileInfoEnumerator";
}
impl IPMTileInfoEnumeratorVtbl {
    pub const fn new<Impl: IPMTileInfoEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMTileInfoEnumeratorVtbl {
        unsafe extern "system" fn Next<Impl: IPMTileInfoEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptileinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&pptileinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMTileInfoEnumerator>, base.5, Next::<Impl, OFFSET>)
    }
}
pub trait IPMTilePropertyEnumeratorImpl: Sized {
    fn Next();
}
impl ::windows::core::RuntimeName for IPMTilePropertyEnumerator {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMTilePropertyEnumerator";
}
impl IPMTilePropertyEnumeratorVtbl {
    pub const fn new<Impl: IPMTilePropertyEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMTilePropertyEnumeratorVtbl {
        unsafe extern "system" fn Next<Impl: IPMTilePropertyEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&pppropinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMTilePropertyEnumerator>, base.5, Next::<Impl, OFFSET>)
    }
}
pub trait IPMTilePropertyInfoImpl: Sized {
    fn PropertyID();
    fn PropertyValue();
    fn set_Property();
}
impl ::windows::core::RuntimeName for IPMTilePropertyInfo {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IPMTilePropertyInfo";
}
impl IPMTilePropertyInfoVtbl {
    pub const fn new<Impl: IPMTilePropertyInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPMTilePropertyInfoVtbl {
        unsafe extern "system" fn PropertyID<Impl: IPMTilePropertyInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PropertyID(::core::mem::transmute_copy(&ppropid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Impl: IPMTilePropertyInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PropertyValue(&*(&ppropvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_Property<Impl: IPMTilePropertyInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).set_Property(&*(&propvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPMTilePropertyInfo>, base.5, PropertyID::<Impl, OFFSET>, PropertyValue::<Impl, OFFSET>, set_Property::<Impl, OFFSET>)
    }
}
pub trait IValidateImpl: Sized {
    fn OpenDatabase();
    fn OpenCUB();
    fn CloseDatabase();
    fn CloseCUB();
    fn SetDisplay();
    fn SetStatus();
    fn Validate();
}
impl ::windows::core::RuntimeName for IValidate {
    const NAME: &'static str = "Windows.Win32.System.ApplicationInstallationAndServicing.IValidate";
}
impl IValidateVtbl {
    pub const fn new<Impl: IValidateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IValidateVtbl {
        unsafe extern "system" fn OpenDatabase<Impl: IValidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szdatabase: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenDatabase(&*(&szdatabase as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenCUB<Impl: IValidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szcubfile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenCUB(&*(&szcubfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseDatabase<Impl: IValidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseDatabase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseCUB<Impl: IValidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseCUB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplay<Impl: IValidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisplayfunction: ::windows::core::RawPtr, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDisplay(&*(&pdisplayfunction as *const <LPDISPLAYVAL as ::windows::core::Abi>::Abi as *const <LPDISPLAYVAL as ::windows::core::DefaultType>::DefaultType), &*(&pcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IValidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatusfunction: ::windows::core::RawPtr, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStatus(&*(&pstatusfunction as *const <LPEVALCOMCALLBACK as ::windows::core::Abi>::Abi as *const <LPEVALCOMCALLBACK as ::windows::core::DefaultType>::DefaultType), &*(&pcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Impl: IValidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wzices: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Validate(&*(&wzices as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IValidate>, base.5, OpenDatabase::<Impl, OFFSET>, OpenCUB::<Impl, OFFSET>, CloseDatabase::<Impl, OFFSET>, CloseCUB::<Impl, OFFSET>, SetDisplay::<Impl, OFFSET>, SetStatus::<Impl, OFFSET>, Validate::<Impl, OFFSET>)
    }
}
