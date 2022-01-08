#[cfg(feature = "Win32_System_Com")]
pub trait IAdviseSinkExImpl: Sized + IAdviseSinkImpl {
    fn OnViewStatusChange();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAdviseSinkEx {
    const NAME: &'static str = "Windows.Win32.System.Ole.IAdviseSinkEx";
}
#[cfg(feature = "Win32_System_Com")]
impl IAdviseSinkExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdviseSinkExImpl, const OFFSET: isize>() -> IAdviseSinkExVtbl {
        unsafe extern "system" fn OnViewStatusChange<Impl: IAdviseSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwviewstatus: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnViewStatusChange(dwviewstatus).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdviseSinkEx>, ::windows::core::GetTrustLevel, OnViewStatusChange::<Impl, OFFSET>)
    }
}
pub trait ICanHandleExceptionImpl: Sized {
    fn CanHandleException();
}
impl ::windows::core::RuntimeName for ICanHandleException {
    const NAME: &'static str = "Windows.Win32.System.Ole.ICanHandleException";
}
impl ICanHandleExceptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICanHandleExceptionImpl, const OFFSET: isize>() -> ICanHandleExceptionVtbl {
        unsafe extern "system" fn CanHandleException<Impl: ICanHandleExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanHandleException(&*(&pexcepinfo as *const <super::Com::EXCEPINFO as ::windows::core::Abi>::Abi as *const <super::Com::EXCEPINFO as ::windows::core::DefaultType>::DefaultType), &*(&pvar as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICanHandleException>, ::windows::core::GetTrustLevel, CanHandleException::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IClassFactory2Impl: Sized + IClassFactoryImpl {
    fn GetLicInfo();
    fn RequestLicKey();
    fn CreateInstanceLic();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IClassFactory2 {
    const NAME: &'static str = "Windows.Win32.System.Ole.IClassFactory2";
}
#[cfg(feature = "Win32_System_Com")]
impl IClassFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClassFactory2Impl, const OFFSET: isize>() -> IClassFactory2Vtbl {
        unsafe extern "system" fn GetLicInfo<Impl: IClassFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plicinfo: *mut LICINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLicInfo(&*(&plicinfo as *const <LICINFO as ::windows::core::Abi>::Abi as *const <LICINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestLicKey<Impl: IClassFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, pbstrkey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLicKey(dwreserved, ::core::mem::transmute_copy(&pbstrkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceLic<Impl: IClassFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, bstrkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceLic(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&punkreserved as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrkey as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvobj),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClassFactory2>, ::windows::core::GetTrustLevel, GetLicInfo::<Impl, OFFSET>, RequestLicKey::<Impl, OFFSET>, CreateInstanceLic::<Impl, OFFSET>)
    }
}
pub trait IContinueImpl: Sized {
    fn FContinue();
}
impl ::windows::core::RuntimeName for IContinue {
    const NAME: &'static str = "Windows.Win32.System.Ole.IContinue";
}
impl IContinueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinueImpl, const OFFSET: isize>() -> IContinueVtbl {
        unsafe extern "system" fn FContinue<Impl: IContinueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FContinue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContinue>, ::windows::core::GetTrustLevel, FContinue::<Impl, OFFSET>)
    }
}
pub trait IContinueCallbackImpl: Sized {
    fn FContinue();
    fn FContinuePrinting();
}
impl ::windows::core::RuntimeName for IContinueCallback {
    const NAME: &'static str = "Windows.Win32.System.Ole.IContinueCallback";
}
impl IContinueCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinueCallbackImpl, const OFFSET: isize>() -> IContinueCallbackVtbl {
        unsafe extern "system" fn FContinue<Impl: IContinueCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FContinue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FContinuePrinting<Impl: IContinueCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncntprinted: i32, ncurpage: i32, pwszprintstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FContinuePrinting(ncntprinted, ncurpage, &*(&pwszprintstatus as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContinueCallback>, ::windows::core::GetTrustLevel, FContinue::<Impl, OFFSET>, FContinuePrinting::<Impl, OFFSET>)
    }
}
pub trait ICreateErrorInfoImpl: Sized {
    fn SetGUID();
    fn SetSource();
    fn SetDescription();
    fn SetHelpFile();
    fn SetHelpContext();
}
impl ::windows::core::RuntimeName for ICreateErrorInfo {
    const NAME: &'static str = "Windows.Win32.System.Ole.ICreateErrorInfo";
}
impl ICreateErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateErrorInfoImpl, const OFFSET: isize>() -> ICreateErrorInfoVtbl {
        unsafe extern "system" fn SetGUID<Impl: ICreateErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGUID(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: ICreateErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsource: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSource(&*(&szsource as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: ICreateErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&szdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpFile<Impl: ICreateErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhelpfile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHelpFile(&*(&szhelpfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpContext<Impl: ICreateErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHelpContext(dwhelpcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICreateErrorInfo>, ::windows::core::GetTrustLevel, SetGUID::<Impl, OFFSET>, SetSource::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, SetHelpFile::<Impl, OFFSET>, SetHelpContext::<Impl, OFFSET>)
    }
}
pub trait ICreateTypeInfoImpl: Sized {
    fn SetGuid();
    fn SetTypeFlags();
    fn SetDocString();
    fn SetHelpContext();
    fn SetVersion();
    fn AddRefTypeInfo();
    fn AddFuncDesc();
    fn AddImplType();
    fn SetImplTypeFlags();
    fn SetAlignment();
    fn SetSchema();
    fn AddVarDesc();
    fn SetFuncAndParamNames();
    fn SetVarName();
    fn SetTypeDescAlias();
    fn DefineFuncAsDllEntry();
    fn SetFuncDocString();
    fn SetVarDocString();
    fn SetFuncHelpContext();
    fn SetVarHelpContext();
    fn SetMops();
    fn SetTypeIdldesc();
    fn LayOut();
}
impl ::windows::core::RuntimeName for ICreateTypeInfo {
    const NAME: &'static str = "Windows.Win32.System.Ole.ICreateTypeInfo";
}
impl ICreateTypeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfoImpl, const OFFSET: isize>() -> ICreateTypeInfoVtbl {
        unsafe extern "system" fn SetGuid<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGuid(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeFlags<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utypeflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTypeFlags(utypeflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocString<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdoc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDocString(&*(&pstrdoc as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpContext<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHelpContext(dwhelpcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVersion(wmajorvernum, wminorvernum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRefTypeInfo<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptinfo: ::windows::core::RawPtr, phreftype: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRefTypeInfo(&*(&ptinfo as *const <super::Com::ITypeInfo as ::windows::core::Abi>::Abi as *const <super::Com::ITypeInfo as ::windows::core::DefaultType>::DefaultType), phreftype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFuncDesc<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFuncDesc(index, &*(&pfuncdesc as *const <super::Com::FUNCDESC as ::windows::core::Abi>::Abi as *const <super::Com::FUNCDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddImplType<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, hreftype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddImplType(index, hreftype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImplTypeFlags<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, impltypeflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetImplTypeFlags(index, impltypeflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbalignment: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlignment(cbalignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchema<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrschema: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSchema(&*(&pstrschema as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddVarDesc<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddVarDesc(index, &*(&pvardesc as *const <super::Com::VARDESC as ::windows::core::Abi>::Abi as *const <super::Com::VARDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFuncAndParamNames<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFuncAndParamNames(index, &*(&rgsznames as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cnames) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVarName<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVarName(index, &*(&szname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeDescAlias<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptdescalias: *const super::Com::TYPEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTypeDescAlias(&*(&ptdescalias as *const <super::Com::TYPEDESC as ::windows::core::Abi>::Abi as *const <super::Com::TYPEDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefineFuncAsDllEntry<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdllname: super::super::Foundation::PWSTR, szprocname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefineFuncAsDllEntry(index, &*(&szdllname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&szprocname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFuncDocString<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFuncDocString(index, &*(&szdocstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVarDocString<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVarDocString(index, &*(&szdocstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFuncHelpContext<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFuncHelpContext(index, dwhelpcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVarHelpContext<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVarHelpContext(index, dwhelpcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMops<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, bstrmops: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMops(index, &*(&bstrmops as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeIdldesc<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidldesc: *const super::Com::IDLDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTypeIdldesc(&*(&pidldesc as *const <super::Com::IDLDESC as ::windows::core::Abi>::Abi as *const <super::Com::IDLDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LayOut<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LayOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICreateTypeInfo>,
            ::windows::core::GetTrustLevel,
            SetGuid::<Impl, OFFSET>,
            SetTypeFlags::<Impl, OFFSET>,
            SetDocString::<Impl, OFFSET>,
            SetHelpContext::<Impl, OFFSET>,
            SetVersion::<Impl, OFFSET>,
            AddRefTypeInfo::<Impl, OFFSET>,
            AddFuncDesc::<Impl, OFFSET>,
            AddImplType::<Impl, OFFSET>,
            SetImplTypeFlags::<Impl, OFFSET>,
            SetAlignment::<Impl, OFFSET>,
            SetSchema::<Impl, OFFSET>,
            AddVarDesc::<Impl, OFFSET>,
            SetFuncAndParamNames::<Impl, OFFSET>,
            SetVarName::<Impl, OFFSET>,
            SetTypeDescAlias::<Impl, OFFSET>,
            DefineFuncAsDllEntry::<Impl, OFFSET>,
            SetFuncDocString::<Impl, OFFSET>,
            SetVarDocString::<Impl, OFFSET>,
            SetFuncHelpContext::<Impl, OFFSET>,
            SetVarHelpContext::<Impl, OFFSET>,
            SetMops::<Impl, OFFSET>,
            SetTypeIdldesc::<Impl, OFFSET>,
            LayOut::<Impl, OFFSET>,
        )
    }
}
pub trait ICreateTypeInfo2Impl: Sized + ICreateTypeInfoImpl {
    fn DeleteFuncDesc();
    fn DeleteFuncDescByMemId();
    fn DeleteVarDesc();
    fn DeleteVarDescByMemId();
    fn DeleteImplType();
    fn SetCustData();
    fn SetFuncCustData();
    fn SetParamCustData();
    fn SetVarCustData();
    fn SetImplTypeCustData();
    fn SetHelpStringContext();
    fn SetFuncHelpStringContext();
    fn SetVarHelpStringContext();
    fn Invalidate();
    fn SetName();
}
impl ::windows::core::RuntimeName for ICreateTypeInfo2 {
    const NAME: &'static str = "Windows.Win32.System.Ole.ICreateTypeInfo2";
}
impl ICreateTypeInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2Impl, const OFFSET: isize>() -> ICreateTypeInfo2Vtbl {
        unsafe extern "system" fn DeleteFuncDesc<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteFuncDesc(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFuncDescByMemId<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteFuncDescByMemId(memid, invkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteVarDesc<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteVarDesc(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteVarDescByMemId<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteVarDescByMemId(memid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteImplType<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteImplType(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustData<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: &::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCustData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pvarval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFuncCustData<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: &::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFuncCustData(index, &*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pvarval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParamCustData<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: &::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetParamCustData(indexfunc, indexparam, &*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pvarval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVarCustData<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: &::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVarCustData(index, &*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pvarval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImplTypeCustData<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: &::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetImplTypeCustData(index, &*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pvarval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpStringContext<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHelpStringContext(dwhelpstringcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFuncHelpStringContext<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFuncHelpStringContext(index, dwhelpstringcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVarHelpStringContext<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVarHelpStringContext(index, dwhelpstringcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invalidate<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invalidate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&szname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICreateTypeInfo2>,
            ::windows::core::GetTrustLevel,
            DeleteFuncDesc::<Impl, OFFSET>,
            DeleteFuncDescByMemId::<Impl, OFFSET>,
            DeleteVarDesc::<Impl, OFFSET>,
            DeleteVarDescByMemId::<Impl, OFFSET>,
            DeleteImplType::<Impl, OFFSET>,
            SetCustData::<Impl, OFFSET>,
            SetFuncCustData::<Impl, OFFSET>,
            SetParamCustData::<Impl, OFFSET>,
            SetVarCustData::<Impl, OFFSET>,
            SetImplTypeCustData::<Impl, OFFSET>,
            SetHelpStringContext::<Impl, OFFSET>,
            SetFuncHelpStringContext::<Impl, OFFSET>,
            SetVarHelpStringContext::<Impl, OFFSET>,
            Invalidate::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
        )
    }
}
pub trait ICreateTypeLibImpl: Sized {
    fn CreateTypeInfo();
    fn SetName();
    fn SetVersion();
    fn SetGuid();
    fn SetDocString();
    fn SetHelpFileName();
    fn SetHelpContext();
    fn SetLcid();
    fn SetLibFlags();
    fn SaveAllChanges();
}
impl ::windows::core::RuntimeName for ICreateTypeLib {
    const NAME: &'static str = "Windows.Win32.System.Ole.ICreateTypeLib";
}
impl ICreateTypeLibVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLibImpl, const OFFSET: isize>() -> ICreateTypeLibVtbl {
        unsafe extern "system" fn CreateTypeInfo<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, tkind: super::Com::TYPEKIND, ppctinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTypeInfo(&*(&szname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), tkind, ::core::mem::transmute_copy(&ppctinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&szname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVersion(wmajorvernum, wminorvernum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGuid(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocString<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdoc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDocString(&*(&szdoc as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpFileName<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhelpfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHelpFileName(&*(&szhelpfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpContext<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHelpContext(dwhelpcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLcid<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLcid(lcid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLibFlags<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulibflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLibFlags(ulibflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAllChanges<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAllChanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICreateTypeLib>,
            ::windows::core::GetTrustLevel,
            CreateTypeInfo::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            SetVersion::<Impl, OFFSET>,
            SetGuid::<Impl, OFFSET>,
            SetDocString::<Impl, OFFSET>,
            SetHelpFileName::<Impl, OFFSET>,
            SetHelpContext::<Impl, OFFSET>,
            SetLcid::<Impl, OFFSET>,
            SetLibFlags::<Impl, OFFSET>,
            SaveAllChanges::<Impl, OFFSET>,
        )
    }
}
pub trait ICreateTypeLib2Impl: Sized + ICreateTypeLibImpl {
    fn DeleteTypeInfo();
    fn SetCustData();
    fn SetHelpStringContext();
    fn SetHelpStringDll();
}
impl ::windows::core::RuntimeName for ICreateTypeLib2 {
    const NAME: &'static str = "Windows.Win32.System.Ole.ICreateTypeLib2";
}
impl ICreateTypeLib2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib2Impl, const OFFSET: isize>() -> ICreateTypeLib2Vtbl {
        unsafe extern "system" fn DeleteTypeInfo<Impl: ICreateTypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteTypeInfo(&*(&szname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustData<Impl: ICreateTypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: &::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCustData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pvarval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpStringContext<Impl: ICreateTypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHelpStringContext(dwhelpstringcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpStringDll<Impl: ICreateTypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHelpStringDll(&*(&szfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICreateTypeLib2>, ::windows::core::GetTrustLevel, DeleteTypeInfo::<Impl, OFFSET>, SetCustData::<Impl, OFFSET>, SetHelpStringContext::<Impl, OFFSET>, SetHelpStringDll::<Impl, OFFSET>)
    }
}
pub trait IDispErrorImpl: Sized {
    fn QueryErrorInfo();
    fn GetNext();
    fn GetHresult();
    fn GetSource();
    fn GetHelpInfo();
    fn GetDescription();
}
impl ::windows::core::RuntimeName for IDispError {
    const NAME: &'static str = "Windows.Win32.System.Ole.IDispError";
}
impl IDispErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispErrorImpl, const OFFSET: isize>() -> IDispErrorVtbl {
        unsafe extern "system" fn QueryErrorInfo<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guiderrortype: ::windows::core::GUID, ppde: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryErrorInfo(&*(&guiderrortype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppde)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNext<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppde: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNext(::core::mem::transmute_copy(&ppde)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHresult<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHresult(::core::mem::transmute_copy(&phr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSource(::core::mem::transmute_copy(&pbstrsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpInfo<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR, pdwcontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpInfo(::core::mem::transmute_copy(&pbstrfilename), ::core::mem::transmute_copy(&pdwcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDispError>, ::windows::core::GetTrustLevel, QueryErrorInfo::<Impl, OFFSET>, GetNext::<Impl, OFFSET>, GetHresult::<Impl, OFFSET>, GetSource::<Impl, OFFSET>, GetHelpInfo::<Impl, OFFSET>, GetDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDispatchExImpl: Sized + IDispatchImpl {
    fn GetDispID();
    fn InvokeEx();
    fn DeleteMemberByName();
    fn DeleteMemberByDispID();
    fn GetMemberProperties();
    fn GetMemberName();
    fn GetNextDispID();
    fn GetNameSpaceParent();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDispatchEx {
    const NAME: &'static str = "Windows.Win32.System.Ole.IDispatchEx";
}
#[cfg(feature = "Win32_System_Com")]
impl IDispatchExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchExImpl, const OFFSET: isize>() -> IDispatchExVtbl {
        unsafe extern "system" fn GetDispID<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grfdex: u32, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDispID(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), grfdex, ::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeEx<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut super::Com::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeEx(id, lcid, wflags, &*(&pdp as *const <super::Com::DISPPARAMS as ::windows::core::Abi>::Abi as *const <super::Com::DISPPARAMS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarres), ::core::mem::transmute_copy(&pei), &*(&pspcaller as *const <super::Com::IServiceProvider as ::windows::core::Abi>::Abi as *const <super::Com::IServiceProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMemberByName<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grfdex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteMemberByName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), grfdex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMemberByDispID<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteMemberByDispID(id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberProperties<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, grfdexfetch: u32, pgrfdex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberProperties(id, grfdexfetch, ::core::mem::transmute_copy(&pgrfdex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberName<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberName(id, ::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextDispID<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfdex: u32, id: i32, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextDispID(grfdex, id, ::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameSpaceParent<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameSpaceParent(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDispatchEx>,
            ::windows::core::GetTrustLevel,
            GetDispID::<Impl, OFFSET>,
            InvokeEx::<Impl, OFFSET>,
            DeleteMemberByName::<Impl, OFFSET>,
            DeleteMemberByDispID::<Impl, OFFSET>,
            GetMemberProperties::<Impl, OFFSET>,
            GetMemberName::<Impl, OFFSET>,
            GetNextDispID::<Impl, OFFSET>,
            GetNameSpaceParent::<Impl, OFFSET>,
        )
    }
}
pub trait IDropSourceImpl: Sized {
    fn QueryContinueDrag();
    fn GiveFeedback();
}
impl ::windows::core::RuntimeName for IDropSource {
    const NAME: &'static str = "Windows.Win32.System.Ole.IDropSource";
}
impl IDropSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropSourceImpl, const OFFSET: isize>() -> IDropSourceVtbl {
        unsafe extern "system" fn QueryContinueDrag<Impl: IDropSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fescapepressed: super::super::Foundation::BOOL, grfkeystate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryContinueDrag(&*(&fescapepressed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), grfkeystate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GiveFeedback<Impl: IDropSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweffect: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GiveFeedback(dweffect) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDropSource>, ::windows::core::GetTrustLevel, QueryContinueDrag::<Impl, OFFSET>, GiveFeedback::<Impl, OFFSET>)
    }
}
pub trait IDropSourceNotifyImpl: Sized {
    fn DragEnterTarget();
    fn DragLeaveTarget();
}
impl ::windows::core::RuntimeName for IDropSourceNotify {
    const NAME: &'static str = "Windows.Win32.System.Ole.IDropSourceNotify";
}
impl IDropSourceNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropSourceNotifyImpl, const OFFSET: isize>() -> IDropSourceNotifyVtbl {
        unsafe extern "system" fn DragEnterTarget<Impl: IDropSourceNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndtarget: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragEnterTarget(&*(&hwndtarget as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragLeaveTarget<Impl: IDropSourceNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragLeaveTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDropSourceNotify>, ::windows::core::GetTrustLevel, DragEnterTarget::<Impl, OFFSET>, DragLeaveTarget::<Impl, OFFSET>)
    }
}
pub trait IDropTargetImpl: Sized {
    fn DragEnter();
    fn DragOver();
    fn DragLeave();
    fn Drop();
}
impl ::windows::core::RuntimeName for IDropTarget {
    const NAME: &'static str = "Windows.Win32.System.Ole.IDropTarget";
}
impl IDropTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetImpl, const OFFSET: isize>() -> IDropTargetVtbl {
        unsafe extern "system" fn DragEnter<Impl: IDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobj: ::windows::core::RawPtr, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragEnter(&*(&pdataobj as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), grfkeystate, &*(&pt as *const <super::super::Foundation::POINTL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINTL as ::windows::core::DefaultType>::DefaultType), pdweffect) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragOver<Impl: IDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragOver(grfkeystate, &*(&pt as *const <super::super::Foundation::POINTL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINTL as ::windows::core::DefaultType>::DefaultType), pdweffect) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragLeave<Impl: IDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragLeave() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Drop<Impl: IDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobj: ::windows::core::RawPtr, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Drop(&*(&pdataobj as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), grfkeystate, &*(&pt as *const <super::super::Foundation::POINTL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINTL as ::windows::core::DefaultType>::DefaultType), pdweffect) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDropTarget>, ::windows::core::GetTrustLevel, DragEnter::<Impl, OFFSET>, DragOver::<Impl, OFFSET>, DragLeave::<Impl, OFFSET>, Drop::<Impl, OFFSET>)
    }
}
pub trait IEnterpriseDropTargetImpl: Sized {
    fn SetDropSourceEnterpriseId();
    fn IsEvaluatingEdpPolicy();
}
impl ::windows::core::RuntimeName for IEnterpriseDropTarget {
    const NAME: &'static str = "Windows.Win32.System.Ole.IEnterpriseDropTarget";
}
impl IEnterpriseDropTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseDropTargetImpl, const OFFSET: isize>() -> IEnterpriseDropTargetVtbl {
        unsafe extern "system" fn SetDropSourceEnterpriseId<Impl: IEnterpriseDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDropSourceEnterpriseId(&*(&identity as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEvaluatingEdpPolicy<Impl: IEnterpriseDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEvaluatingEdpPolicy(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnterpriseDropTarget>, ::windows::core::GetTrustLevel, SetDropSourceEnterpriseId::<Impl, OFFSET>, IsEvaluatingEdpPolicy::<Impl, OFFSET>)
    }
}
pub trait IEnumOLEVERBImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumOLEVERB {
    const NAME: &'static str = "Windows.Win32.System.Ole.IEnumOLEVERB";
}
impl IEnumOLEVERBVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOLEVERBImpl, const OFFSET: isize>() -> IEnumOLEVERBVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOLEVERBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumOLEVERBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumOLEVERBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumOLEVERBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumOLEVERB>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumOleDocumentViewsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumOleDocumentViews {
    const NAME: &'static str = "Windows.Win32.System.Ole.IEnumOleDocumentViews";
}
impl IEnumOleDocumentViewsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleDocumentViewsImpl, const OFFSET: isize>() -> IEnumOleDocumentViewsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOleDocumentViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cviews: u32, rgpview: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(cviews, ::core::mem::transmute_copy(&rgpview), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumOleDocumentViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cviews: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(cviews) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumOleDocumentViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumOleDocumentViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumOleDocumentViews>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumOleUndoUnitsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumOleUndoUnits {
    const NAME: &'static str = "Windows.Win32.System.Ole.IEnumOleUndoUnits";
}
impl IEnumOleUndoUnitsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleUndoUnitsImpl, const OFFSET: isize>() -> IEnumOleUndoUnitsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOleUndoUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumOleUndoUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumOleUndoUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumOleUndoUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumOleUndoUnits>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumVARIANTImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumVARIANT {
    const NAME: &'static str = "Windows.Win32.System.Ole.IEnumVARIANT";
}
impl IEnumVARIANTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVARIANTImpl, const OFFSET: isize>() -> IEnumVARIANTVtbl {
        unsafe extern "system" fn Next<Impl: IEnumVARIANTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumVARIANTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumVARIANTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumVARIANTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumVARIANT>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IFontImpl: Sized {
    fn Name();
    fn SetName();
    fn Size();
    fn SetSize();
    fn Bold();
    fn SetBold();
    fn Italic();
    fn SetItalic();
    fn Underline();
    fn SetUnderline();
    fn Strikethrough();
    fn SetStrikethrough();
    fn Weight();
    fn SetWeight();
    fn Charset();
    fn SetCharset();
    fn hFont();
    fn Clone();
    fn IsEqual();
    fn SetRatio();
    fn QueryTextMetrics();
    fn AddRefHfont();
    fn ReleaseHfont();
    fn SetHdc();
}
impl ::windows::core::RuntimeName for IFont {
    const NAME: &'static str = "Windows.Win32.System.Ole.IFont";
}
impl IFontVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontImpl, const OFFSET: isize>() -> IFontVtbl {
        unsafe extern "system" fn Name<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size(::core::mem::transmute_copy(&psize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSize(&*(&size as *const <super::Com::CY as ::windows::core::Abi>::Abi as *const <super::Com::CY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bold<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbold: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bold(::core::mem::transmute_copy(&pbold)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bold: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBold(&*(&bold as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Italic<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitalic: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Italic(::core::mem::transmute_copy(&pitalic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, italic: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetItalic(&*(&italic as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Underline<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punderline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Underline(::core::mem::transmute_copy(&punderline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, underline: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUnderline(&*(&underline as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strikethrough<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrikethrough: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strikethrough(::core::mem::transmute_copy(&pstrikethrough)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strikethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStrikethrough(&*(&strikethrough as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Weight<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweight: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Weight(::core::mem::transmute_copy(&pweight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWeight(weight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Charset<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcharset: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Charset(::core::mem::transmute_copy(&pcharset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharset<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, charset: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCharset(charset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hFont<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phfont: *mut super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).hFont(::core::mem::transmute_copy(&phfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfontother: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&pfontother as *const <IFont as ::windows::core::Abi>::Abi as *const <IFont as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRatio<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cylogical: i32, cyhimetric: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRatio(cylogical, cyhimetric) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryTextMetrics<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryTextMetrics(::core::mem::transmute_copy(&ptm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRefHfont<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRefHfont(&*(&hfont as *const <super::super::Graphics::Gdi::HFONT as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HFONT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseHfont<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseHfont(&*(&hfont as *const <super::super::Graphics::Gdi::HFONT as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HFONT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHdc<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHdc(&*(&hdc as *const <super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFont>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Size::<Impl, OFFSET>,
            SetSize::<Impl, OFFSET>,
            Bold::<Impl, OFFSET>,
            SetBold::<Impl, OFFSET>,
            Italic::<Impl, OFFSET>,
            SetItalic::<Impl, OFFSET>,
            Underline::<Impl, OFFSET>,
            SetUnderline::<Impl, OFFSET>,
            Strikethrough::<Impl, OFFSET>,
            SetStrikethrough::<Impl, OFFSET>,
            Weight::<Impl, OFFSET>,
            SetWeight::<Impl, OFFSET>,
            Charset::<Impl, OFFSET>,
            SetCharset::<Impl, OFFSET>,
            hFont::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
            SetRatio::<Impl, OFFSET>,
            QueryTextMetrics::<Impl, OFFSET>,
            AddRefHfont::<Impl, OFFSET>,
            ReleaseHfont::<Impl, OFFSET>,
            SetHdc::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFontDispImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFontDisp {
    const NAME: &'static str = "Windows.Win32.System.Ole.IFontDisp";
}
#[cfg(feature = "Win32_System_Com")]
impl IFontDispVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontDispImpl, const OFFSET: isize>() -> IFontDispVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFontDisp>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFontEventsDispImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFontEventsDisp {
    const NAME: &'static str = "Windows.Win32.System.Ole.IFontEventsDisp";
}
#[cfg(feature = "Win32_System_Com")]
impl IFontEventsDispVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontEventsDispImpl, const OFFSET: isize>() -> IFontEventsDispVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFontEventsDisp>, ::windows::core::GetTrustLevel)
    }
}
pub trait IGetOleObjectImpl: Sized {
    fn GetOleObject();
}
impl ::windows::core::RuntimeName for IGetOleObject {
    const NAME: &'static str = "Windows.Win32.System.Ole.IGetOleObject";
}
impl IGetOleObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetOleObjectImpl, const OFFSET: isize>() -> IGetOleObjectVtbl {
        unsafe extern "system" fn GetOleObject<Impl: IGetOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOleObject(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppvobj as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGetOleObject>, ::windows::core::GetTrustLevel, GetOleObject::<Impl, OFFSET>)
    }
}
pub trait IGetVBAObjectImpl: Sized {
    fn GetObject();
}
impl ::windows::core::RuntimeName for IGetVBAObject {
    const NAME: &'static str = "Windows.Win32.System.Ole.IGetVBAObject";
}
impl IGetVBAObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetVBAObjectImpl, const OFFSET: isize>() -> IGetVBAObjectVtbl {
        unsafe extern "system" fn GetObject<Impl: IGetVBAObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppvobj as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGetVBAObject>, ::windows::core::GetTrustLevel, GetObject::<Impl, OFFSET>)
    }
}
pub trait IObjectIdentityImpl: Sized {
    fn IsEqualObject();
}
impl ::windows::core::RuntimeName for IObjectIdentity {
    const NAME: &'static str = "Windows.Win32.System.Ole.IObjectIdentity";
}
impl IObjectIdentityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectIdentityImpl, const OFFSET: isize>() -> IObjectIdentityVtbl {
        unsafe extern "system" fn IsEqualObject<Impl: IObjectIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqualObject(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectIdentity>, ::windows::core::GetTrustLevel, IsEqualObject::<Impl, OFFSET>)
    }
}
pub trait IObjectWithSiteImpl: Sized {
    fn SetSite();
    fn GetSite();
}
impl ::windows::core::RuntimeName for IObjectWithSite {
    const NAME: &'static str = "Windows.Win32.System.Ole.IObjectWithSite";
}
impl IObjectWithSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectWithSiteImpl, const OFFSET: isize>() -> IObjectWithSiteVtbl {
        unsafe extern "system" fn SetSite<Impl: IObjectWithSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punksite: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSite(&*(&punksite as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSite<Impl: IObjectWithSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppvsite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSite(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvsite)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectWithSite>, ::windows::core::GetTrustLevel, SetSite::<Impl, OFFSET>, GetSite::<Impl, OFFSET>)
    }
}
pub trait IOleAdviseHolderImpl: Sized {
    fn Advise();
    fn Unadvise();
    fn EnumAdvise();
    fn SendOnRename();
    fn SendOnSave();
    fn SendOnClose();
}
impl ::windows::core::RuntimeName for IOleAdviseHolder {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleAdviseHolder";
}
impl IOleAdviseHolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleAdviseHolderImpl, const OFFSET: isize>() -> IOleAdviseHolderVtbl {
        unsafe extern "system" fn Advise<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvise: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(&*(&padvise as *const <super::Com::IAdviseSink as ::windows::core::Abi>::Abi as *const <super::Com::IAdviseSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unadvise(dwconnection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAdvise<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAdvise(::core::mem::transmute_copy(&ppenumadvise)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOnRename<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendOnRename(&*(&pmk as *const <super::Com::IMoniker as ::windows::core::Abi>::Abi as *const <super::Com::IMoniker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOnSave<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendOnSave() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOnClose<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendOnClose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleAdviseHolder>, ::windows::core::GetTrustLevel, Advise::<Impl, OFFSET>, Unadvise::<Impl, OFFSET>, EnumAdvise::<Impl, OFFSET>, SendOnRename::<Impl, OFFSET>, SendOnSave::<Impl, OFFSET>, SendOnClose::<Impl, OFFSET>)
    }
}
pub trait IOleCacheImpl: Sized {
    fn Cache();
    fn Uncache();
    fn EnumCache();
    fn InitCache();
    fn SetData();
}
impl ::windows::core::RuntimeName for IOleCache {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleCache";
}
impl IOleCacheVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCacheImpl, const OFFSET: isize>() -> IOleCacheVtbl {
        unsafe extern "system" fn Cache<Impl: IOleCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, advf: u32, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cache(&*(&pformatetc as *const <super::Com::FORMATETC as ::windows::core::Abi>::Abi as *const <super::Com::FORMATETC as ::windows::core::DefaultType>::DefaultType), advf, ::core::mem::transmute_copy(&pdwconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uncache<Impl: IOleCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uncache(dwconnection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCache<Impl: IOleCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstatdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCache(::core::mem::transmute_copy(&ppenumstatdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitCache<Impl: IOleCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitCache(&*(&pdataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IOleCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetData(
                &*(&pformatetc as *const <super::Com::FORMATETC as ::windows::core::Abi>::Abi as *const <super::Com::FORMATETC as ::windows::core::DefaultType>::DefaultType),
                &*(&pmedium as *const <super::Com::STGMEDIUM as ::windows::core::Abi>::Abi as *const <super::Com::STGMEDIUM as ::windows::core::DefaultType>::DefaultType),
                &*(&frelease as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleCache>, ::windows::core::GetTrustLevel, Cache::<Impl, OFFSET>, Uncache::<Impl, OFFSET>, EnumCache::<Impl, OFFSET>, InitCache::<Impl, OFFSET>, SetData::<Impl, OFFSET>)
    }
}
pub trait IOleCache2Impl: Sized + IOleCacheImpl {
    fn UpdateCache();
    fn DiscardCache();
}
impl ::windows::core::RuntimeName for IOleCache2 {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleCache2";
}
impl IOleCache2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache2Impl, const OFFSET: isize>() -> IOleCache2Vtbl {
        unsafe extern "system" fn UpdateCache<Impl: IOleCache2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, grfupdf: UPDFCACHE_FLAGS, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateCache(&*(&pdataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), grfupdf, &*(&preserved as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardCache<Impl: IOleCache2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdiscardoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscardCache(dwdiscardoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleCache2>, ::windows::core::GetTrustLevel, UpdateCache::<Impl, OFFSET>, DiscardCache::<Impl, OFFSET>)
    }
}
pub trait IOleCacheControlImpl: Sized {
    fn OnRun();
    fn OnStop();
}
impl ::windows::core::RuntimeName for IOleCacheControl {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleCacheControl";
}
impl IOleCacheControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCacheControlImpl, const OFFSET: isize>() -> IOleCacheControlVtbl {
        unsafe extern "system" fn OnRun<Impl: IOleCacheControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnRun(&*(&pdataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStop<Impl: IOleCacheControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleCacheControl>, ::windows::core::GetTrustLevel, OnRun::<Impl, OFFSET>, OnStop::<Impl, OFFSET>)
    }
}
pub trait IOleClientSiteImpl: Sized {
    fn SaveObject();
    fn GetMoniker();
    fn GetContainer();
    fn ShowObject();
    fn OnShowWindow();
    fn RequestNewObjectLayout();
}
impl ::windows::core::RuntimeName for IOleClientSite {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleClientSite";
}
impl IOleClientSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleClientSiteImpl, const OFFSET: isize>() -> IOleClientSiteVtbl {
        unsafe extern "system" fn SaveObject<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveObject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMoniker<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMoniker(dwassign, dwwhichmoniker, ::core::mem::transmute_copy(&ppmk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainer(::core::mem::transmute_copy(&ppcontainer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowObject<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowObject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnShowWindow<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnShowWindow(&*(&fshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestNewObjectLayout<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestNewObjectLayout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleClientSite>, ::windows::core::GetTrustLevel, SaveObject::<Impl, OFFSET>, GetMoniker::<Impl, OFFSET>, GetContainer::<Impl, OFFSET>, ShowObject::<Impl, OFFSET>, OnShowWindow::<Impl, OFFSET>, RequestNewObjectLayout::<Impl, OFFSET>)
    }
}
pub trait IOleCommandTargetImpl: Sized {
    fn QueryStatus();
    fn Exec();
}
impl ::windows::core::RuntimeName for IOleCommandTarget {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleCommandTarget";
}
impl IOleCommandTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCommandTargetImpl, const OFFSET: isize>() -> IOleCommandTargetVtbl {
        unsafe extern "system" fn QueryStatus<Impl: IOleCommandTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: &::windows::core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryStatus(&*(&pguidcmdgroup as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ccmds, &*(&prgcmds as *const <OLECMD as ::windows::core::Abi>::Abi as *const <OLECMD as ::windows::core::DefaultType>::DefaultType), &*(&pcmdtext as *const <OLECMDTEXT as ::windows::core::Abi>::Abi as *const <OLECMDTEXT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Exec<Impl: IOleCommandTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: &::windows::core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Com::VARIANT, pvaout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exec(
                &*(&pguidcmdgroup as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ncmdid,
                ncmdexecopt,
                &*(&pvain as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvaout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleCommandTarget>, ::windows::core::GetTrustLevel, QueryStatus::<Impl, OFFSET>, Exec::<Impl, OFFSET>)
    }
}
pub trait IOleContainerImpl: Sized + IParseDisplayNameImpl {
    fn EnumObjects();
    fn LockContainer();
}
impl ::windows::core::RuntimeName for IOleContainer {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleContainer";
}
impl IOleContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleContainerImpl, const OFFSET: isize>() -> IOleContainerVtbl {
        unsafe extern "system" fn EnumObjects<Impl: IOleContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumObjects(grfflags, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockContainer<Impl: IOleContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockContainer(&*(&flock as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleContainer>, ::windows::core::GetTrustLevel, EnumObjects::<Impl, OFFSET>, LockContainer::<Impl, OFFSET>)
    }
}
pub trait IOleControlImpl: Sized {
    fn GetControlInfo();
    fn OnMnemonic();
    fn OnAmbientPropertyChange();
    fn FreezeEvents();
}
impl ::windows::core::RuntimeName for IOleControl {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleControl";
}
impl IOleControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlImpl, const OFFSET: isize>() -> IOleControlVtbl {
        unsafe extern "system" fn GetControlInfo<Impl: IOleControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut CONTROLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControlInfo(&*(&pci as *const <CONTROLINFO as ::windows::core::Abi>::Abi as *const <CONTROLINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnMnemonic<Impl: IOleControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMnemonic(&*(&pmsg as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAmbientPropertyChange<Impl: IOleControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAmbientPropertyChange(dispid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreezeEvents<Impl: IOleControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfreeze: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreezeEvents(&*(&bfreeze as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleControl>, ::windows::core::GetTrustLevel, GetControlInfo::<Impl, OFFSET>, OnMnemonic::<Impl, OFFSET>, OnAmbientPropertyChange::<Impl, OFFSET>, FreezeEvents::<Impl, OFFSET>)
    }
}
pub trait IOleControlSiteImpl: Sized {
    fn OnControlInfoChanged();
    fn LockInPlaceActive();
    fn GetExtendedControl();
    fn TransformCoords();
    fn TranslateAccelerator();
    fn OnFocus();
    fn ShowPropertyFrame();
}
impl ::windows::core::RuntimeName for IOleControlSite {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleControlSite";
}
impl IOleControlSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlSiteImpl, const OFFSET: isize>() -> IOleControlSiteVtbl {
        unsafe extern "system" fn OnControlInfoChanged<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnControlInfoChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockInPlaceActive<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockInPlaceActive(&*(&flock as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtendedControl<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtendedControl(::core::mem::transmute_copy(&ppdisp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformCoords<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: XFORMCOORDS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformCoords(&*(&pptlhimetric as *const <super::super::Foundation::POINTL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINTL as ::windows::core::DefaultType>::DefaultType), &*(&pptfcontainer as *const <POINTF as ::windows::core::Abi>::Abi as *const <POINTF as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateAccelerator(&*(&pmsg as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::DefaultType>::DefaultType), grfmodifiers) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnFocus<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgotfocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnFocus(&*(&fgotfocus as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPropertyFrame<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowPropertyFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOleControlSite>,
            ::windows::core::GetTrustLevel,
            OnControlInfoChanged::<Impl, OFFSET>,
            LockInPlaceActive::<Impl, OFFSET>,
            GetExtendedControl::<Impl, OFFSET>,
            TransformCoords::<Impl, OFFSET>,
            TranslateAccelerator::<Impl, OFFSET>,
            OnFocus::<Impl, OFFSET>,
            ShowPropertyFrame::<Impl, OFFSET>,
        )
    }
}
pub trait IOleDocumentImpl: Sized {
    fn CreateView();
    fn GetDocMiscStatus();
    fn EnumViews();
}
impl ::windows::core::RuntimeName for IOleDocument {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleDocument";
}
impl IOleDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentImpl, const OFFSET: isize>() -> IOleDocumentVtbl {
        unsafe extern "system" fn CreateView<Impl: IOleDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsite: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, dwreserved: u32, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateView(&*(&pipsite as *const <IOleInPlaceSite as ::windows::core::Abi>::Abi as *const <IOleInPlaceSite as ::windows::core::DefaultType>::DefaultType), &*(&pstm as *const <super::Com::IStream as ::windows::core::Abi>::Abi as *const <super::Com::IStream as ::windows::core::DefaultType>::DefaultType), dwreserved, ::core::mem::transmute_copy(&ppview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocMiscStatus<Impl: IOleDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocMiscStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumViews<Impl: IOleDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumViews(::core::mem::transmute_copy(&ppenum), ::core::mem::transmute_copy(&ppview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleDocument>, ::windows::core::GetTrustLevel, CreateView::<Impl, OFFSET>, GetDocMiscStatus::<Impl, OFFSET>, EnumViews::<Impl, OFFSET>)
    }
}
pub trait IOleDocumentSiteImpl: Sized {
    fn ActivateMe();
}
impl ::windows::core::RuntimeName for IOleDocumentSite {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleDocumentSite";
}
impl IOleDocumentSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentSiteImpl, const OFFSET: isize>() -> IOleDocumentSiteVtbl {
        unsafe extern "system" fn ActivateMe<Impl: IOleDocumentSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewtoactivate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateMe(&*(&pviewtoactivate as *const <IOleDocumentView as ::windows::core::Abi>::Abi as *const <IOleDocumentView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleDocumentSite>, ::windows::core::GetTrustLevel, ActivateMe::<Impl, OFFSET>)
    }
}
pub trait IOleDocumentViewImpl: Sized {
    fn SetInPlaceSite();
    fn GetInPlaceSite();
    fn GetDocument();
    fn SetRect();
    fn GetRect();
    fn SetRectComplex();
    fn Show();
    fn UIActivate();
    fn Open();
    fn CloseView();
    fn SaveViewState();
    fn ApplyViewState();
    fn Clone();
}
impl ::windows::core::RuntimeName for IOleDocumentView {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleDocumentView";
}
impl IOleDocumentViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentViewImpl, const OFFSET: isize>() -> IOleDocumentViewVtbl {
        unsafe extern "system" fn SetInPlaceSite<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInPlaceSite(&*(&pipsite as *const <IOleInPlaceSite as ::windows::core::Abi>::Abi as *const <IOleInPlaceSite as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInPlaceSite<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInPlaceSite(::core::mem::transmute_copy(&ppipsite)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocument(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRect<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRect(&*(&prcview as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRect<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRect(::core::mem::transmute_copy(&prcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRectComplex<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRectComplex(
                &*(&prcview as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&prchscroll as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&prcvscroll as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&prcsizebox as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(&*(&fshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UIActivate<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuiactivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIActivate(&*(&fuiactivate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseView<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseView(dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveViewState<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveViewState(&*(&pstm as *const <super::Com::IStream as ::windows::core::Abi>::Abi as *const <super::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyViewState<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyViewState(&*(&pstm as *const <super::Com::IStream as ::windows::core::Abi>::Abi as *const <super::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsitenew: ::windows::core::RawPtr, ppviewnew: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(&*(&pipsitenew as *const <IOleInPlaceSite as ::windows::core::Abi>::Abi as *const <IOleInPlaceSite as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppviewnew)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOleDocumentView>,
            ::windows::core::GetTrustLevel,
            SetInPlaceSite::<Impl, OFFSET>,
            GetInPlaceSite::<Impl, OFFSET>,
            GetDocument::<Impl, OFFSET>,
            SetRect::<Impl, OFFSET>,
            GetRect::<Impl, OFFSET>,
            SetRectComplex::<Impl, OFFSET>,
            Show::<Impl, OFFSET>,
            UIActivate::<Impl, OFFSET>,
            Open::<Impl, OFFSET>,
            CloseView::<Impl, OFFSET>,
            SaveViewState::<Impl, OFFSET>,
            ApplyViewState::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
        )
    }
}
pub trait IOleInPlaceActiveObjectImpl: Sized + IOleWindowImpl {
    fn TranslateAccelerator();
    fn OnFrameWindowActivate();
    fn OnDocWindowActivate();
    fn ResizeBorder();
    fn EnableModeless();
}
impl ::windows::core::RuntimeName for IOleInPlaceActiveObject {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleInPlaceActiveObject";
}
impl IOleInPlaceActiveObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceActiveObjectImpl, const OFFSET: isize>() -> IOleInPlaceActiveObjectVtbl {
        unsafe extern "system" fn TranslateAccelerator<Impl: IOleInPlaceActiveObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateAccelerator(&*(&lpmsg as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnFrameWindowActivate<Impl: IOleInPlaceActiveObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnFrameWindowActivate(&*(&factivate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDocWindowActivate<Impl: IOleInPlaceActiveObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDocWindowActivate(&*(&factivate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeBorder<Impl: IOleInPlaceActiveObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcborder: *const super::super::Foundation::RECT, puiwindow: ::windows::core::RawPtr, fframewindow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResizeBorder(
                &*(&prcborder as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&puiwindow as *const <IOleInPlaceUIWindow as ::windows::core::Abi>::Abi as *const <IOleInPlaceUIWindow as ::windows::core::DefaultType>::DefaultType),
                &*(&fframewindow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableModeless<Impl: IOleInPlaceActiveObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableModeless(&*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleInPlaceActiveObject>, ::windows::core::GetTrustLevel, TranslateAccelerator::<Impl, OFFSET>, OnFrameWindowActivate::<Impl, OFFSET>, OnDocWindowActivate::<Impl, OFFSET>, ResizeBorder::<Impl, OFFSET>, EnableModeless::<Impl, OFFSET>)
    }
}
pub trait IOleInPlaceFrameImpl: Sized + IOleInPlaceUIWindowImpl + IOleWindowImpl {
    fn InsertMenus();
    fn SetMenu();
    fn RemoveMenus();
    fn SetStatusText();
    fn EnableModeless();
    fn TranslateAccelerator();
}
impl ::windows::core::RuntimeName for IOleInPlaceFrame {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleInPlaceFrame";
}
impl IOleInPlaceFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceFrameImpl, const OFFSET: isize>() -> IOleInPlaceFrameVtbl {
        unsafe extern "system" fn InsertMenus<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertMenus(&*(&hmenushared as *const <super::super::UI::WindowsAndMessaging::HMENU as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::HMENU as ::windows::core::DefaultType>::DefaultType), &*(&lpmenuwidths as *const <OleMenuGroupWidths as ::windows::core::Abi>::Abi as *const <OleMenuGroupWidths as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMenu<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMenu(&*(&hmenushared as *const <super::super::UI::WindowsAndMessaging::HMENU as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::HMENU as ::windows::core::DefaultType>::DefaultType), holemenu, &*(&hwndactiveobject as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMenus<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveMenus(&*(&hmenushared as *const <super::super::UI::WindowsAndMessaging::HMENU as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::HMENU as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatusText<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatusText(&*(&pszstatustext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableModeless<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableModeless(&*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateAccelerator(&*(&lpmsg as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::DefaultType>::DefaultType), wid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleInPlaceFrame>, ::windows::core::GetTrustLevel, InsertMenus::<Impl, OFFSET>, SetMenu::<Impl, OFFSET>, RemoveMenus::<Impl, OFFSET>, SetStatusText::<Impl, OFFSET>, EnableModeless::<Impl, OFFSET>, TranslateAccelerator::<Impl, OFFSET>)
    }
}
pub trait IOleInPlaceObjectImpl: Sized + IOleWindowImpl {
    fn InPlaceDeactivate();
    fn UIDeactivate();
    fn SetObjectRects();
    fn ReactivateAndUndo();
}
impl ::windows::core::RuntimeName for IOleInPlaceObject {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleInPlaceObject";
}
impl IOleInPlaceObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObjectImpl, const OFFSET: isize>() -> IOleInPlaceObjectVtbl {
        unsafe extern "system" fn InPlaceDeactivate<Impl: IOleInPlaceObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InPlaceDeactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UIDeactivate<Impl: IOleInPlaceObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIDeactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectRects<Impl: IOleInPlaceObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetObjectRects(&*(&lprcposrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&lprccliprect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReactivateAndUndo<Impl: IOleInPlaceObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReactivateAndUndo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleInPlaceObject>, ::windows::core::GetTrustLevel, InPlaceDeactivate::<Impl, OFFSET>, UIDeactivate::<Impl, OFFSET>, SetObjectRects::<Impl, OFFSET>, ReactivateAndUndo::<Impl, OFFSET>)
    }
}
pub trait IOleInPlaceObjectWindowlessImpl: Sized + IOleInPlaceObjectImpl + IOleWindowImpl {
    fn OnWindowMessage();
    fn GetDropTarget();
}
impl ::windows::core::RuntimeName for IOleInPlaceObjectWindowless {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleInPlaceObjectWindowless";
}
impl IOleInPlaceObjectWindowlessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObjectWindowlessImpl, const OFFSET: isize>() -> IOleInPlaceObjectWindowlessVtbl {
        unsafe extern "system" fn OnWindowMessage<Impl: IOleInPlaceObjectWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnWindowMessage(msg, &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDropTarget<Impl: IOleInPlaceObjectWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdroptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDropTarget(::core::mem::transmute_copy(&ppdroptarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleInPlaceObjectWindowless>, ::windows::core::GetTrustLevel, OnWindowMessage::<Impl, OFFSET>, GetDropTarget::<Impl, OFFSET>)
    }
}
pub trait IOleInPlaceSiteImpl: Sized + IOleWindowImpl {
    fn CanInPlaceActivate();
    fn OnInPlaceActivate();
    fn OnUIActivate();
    fn GetWindowContext();
    fn Scroll();
    fn OnUIDeactivate();
    fn OnInPlaceDeactivate();
    fn DiscardUndoState();
    fn DeactivateAndUndo();
    fn OnPosRectChange();
}
impl ::windows::core::RuntimeName for IOleInPlaceSite {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleInPlaceSite";
}
impl IOleInPlaceSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteImpl, const OFFSET: isize>() -> IOleInPlaceSiteVtbl {
        unsafe extern "system" fn CanInPlaceActivate<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanInPlaceActivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInPlaceActivate<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInPlaceActivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUIActivate<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUIActivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowContext<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppframe: *mut ::windows::core::RawPtr, ppdoc: *mut ::windows::core::RawPtr, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindowContext(::core::mem::transmute_copy(&ppframe), ::core::mem::transmute_copy(&ppdoc), ::core::mem::transmute_copy(&lprcposrect), ::core::mem::transmute_copy(&lprccliprect), &*(&lpframeinfo as *const <OIFI as ::windows::core::Abi>::Abi as *const <OIFI as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scroll<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollextant: super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scroll(&*(&scrollextant as *const <super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUIDeactivate<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fundoable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUIDeactivate(&*(&fundoable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInPlaceDeactivate<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInPlaceDeactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardUndoState<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscardUndoState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeactivateAndUndo<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeactivateAndUndo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPosRectChange<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnPosRectChange(&*(&lprcposrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOleInPlaceSite>,
            ::windows::core::GetTrustLevel,
            CanInPlaceActivate::<Impl, OFFSET>,
            OnInPlaceActivate::<Impl, OFFSET>,
            OnUIActivate::<Impl, OFFSET>,
            GetWindowContext::<Impl, OFFSET>,
            Scroll::<Impl, OFFSET>,
            OnUIDeactivate::<Impl, OFFSET>,
            OnInPlaceDeactivate::<Impl, OFFSET>,
            DiscardUndoState::<Impl, OFFSET>,
            DeactivateAndUndo::<Impl, OFFSET>,
            OnPosRectChange::<Impl, OFFSET>,
        )
    }
}
pub trait IOleInPlaceSiteExImpl: Sized + IOleInPlaceSiteImpl + IOleWindowImpl {
    fn OnInPlaceActivateEx();
    fn OnInPlaceDeactivateEx();
    fn RequestUIActivate();
}
impl ::windows::core::RuntimeName for IOleInPlaceSiteEx {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleInPlaceSiteEx";
}
impl IOleInPlaceSiteExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteExImpl, const OFFSET: isize>() -> IOleInPlaceSiteExVtbl {
        unsafe extern "system" fn OnInPlaceActivateEx<Impl: IOleInPlaceSiteExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInPlaceActivateEx(::core::mem::transmute_copy(&pfnoredraw), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInPlaceDeactivateEx<Impl: IOleInPlaceSiteExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnoredraw: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInPlaceDeactivateEx(&*(&fnoredraw as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestUIActivate<Impl: IOleInPlaceSiteExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestUIActivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleInPlaceSiteEx>, ::windows::core::GetTrustLevel, OnInPlaceActivateEx::<Impl, OFFSET>, OnInPlaceDeactivateEx::<Impl, OFFSET>, RequestUIActivate::<Impl, OFFSET>)
    }
}
pub trait IOleInPlaceSiteWindowlessImpl: Sized + IOleInPlaceSiteExImpl + IOleInPlaceSiteImpl + IOleWindowImpl {
    fn CanWindowlessActivate();
    fn GetCapture();
    fn SetCapture();
    fn GetFocus();
    fn SetFocus();
    fn GetDC();
    fn ReleaseDC();
    fn InvalidateRect();
    fn InvalidateRgn();
    fn ScrollRect();
    fn AdjustRect();
    fn OnDefWindowMessage();
}
impl ::windows::core::RuntimeName for IOleInPlaceSiteWindowless {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleInPlaceSiteWindowless";
}
impl IOleInPlaceSiteWindowlessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>() -> IOleInPlaceSiteWindowlessVtbl {
        unsafe extern "system" fn CanWindowlessActivate<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanWindowlessActivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapture<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCapture<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcapture: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCapture(&*(&fcapture as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFocus(&*(&ffocus as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDC<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, grfflags: u32, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDC(&*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), grfflags, ::core::mem::transmute_copy(&phdc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseDC(&*(&hdc as *const <super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateRect<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidateRect(&*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&ferase as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateRgn<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidateRgn(&*(&hrgn as *const <super::super::Graphics::Gdi::HRGN as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HRGN as ::windows::core::DefaultType>::DefaultType), &*(&ferase as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollRect<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollRect(dx, dy, &*(&prectscroll as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&prectclip as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjustRect<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdjustRect(&*(&prc as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDefWindowMessage<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDefWindowMessage(msg, &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOleInPlaceSiteWindowless>,
            ::windows::core::GetTrustLevel,
            CanWindowlessActivate::<Impl, OFFSET>,
            GetCapture::<Impl, OFFSET>,
            SetCapture::<Impl, OFFSET>,
            GetFocus::<Impl, OFFSET>,
            SetFocus::<Impl, OFFSET>,
            GetDC::<Impl, OFFSET>,
            ReleaseDC::<Impl, OFFSET>,
            InvalidateRect::<Impl, OFFSET>,
            InvalidateRgn::<Impl, OFFSET>,
            ScrollRect::<Impl, OFFSET>,
            AdjustRect::<Impl, OFFSET>,
            OnDefWindowMessage::<Impl, OFFSET>,
        )
    }
}
pub trait IOleInPlaceUIWindowImpl: Sized + IOleWindowImpl {
    fn GetBorder();
    fn RequestBorderSpace();
    fn SetBorderSpace();
    fn SetActiveObject();
}
impl ::windows::core::RuntimeName for IOleInPlaceUIWindow {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleInPlaceUIWindow";
}
impl IOleInPlaceUIWindowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceUIWindowImpl, const OFFSET: isize>() -> IOleInPlaceUIWindowVtbl {
        unsafe extern "system" fn GetBorder<Impl: IOleInPlaceUIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprectborder: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBorder(::core::mem::transmute_copy(&lprectborder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestBorderSpace<Impl: IOleInPlaceUIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestBorderSpace(&*(&pborderwidths as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderSpace<Impl: IOleInPlaceUIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBorderSpace(&*(&pborderwidths as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveObject<Impl: IOleInPlaceUIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactiveobject: ::windows::core::RawPtr, pszobjname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveObject(&*(&pactiveobject as *const <IOleInPlaceActiveObject as ::windows::core::Abi>::Abi as *const <IOleInPlaceActiveObject as ::windows::core::DefaultType>::DefaultType), &*(&pszobjname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleInPlaceUIWindow>, ::windows::core::GetTrustLevel, GetBorder::<Impl, OFFSET>, RequestBorderSpace::<Impl, OFFSET>, SetBorderSpace::<Impl, OFFSET>, SetActiveObject::<Impl, OFFSET>)
    }
}
pub trait IOleItemContainerImpl: Sized + IOleContainerImpl + IParseDisplayNameImpl {
    fn GetObject();
    fn GetObjectStorage();
    fn IsRunning();
}
impl ::windows::core::RuntimeName for IOleItemContainer {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleItemContainer";
}
impl IOleItemContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleItemContainerImpl, const OFFSET: isize>() -> IOleItemContainerVtbl {
        unsafe extern "system" fn GetObject<Impl: IOleItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR, dwspeedneeded: u32, pbc: ::windows::core::RawPtr, riid: &::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(
                &*(&pszitem as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwspeedneeded,
                &*(&pbc as *const <super::Com::IBindCtx as ::windows::core::Abi>::Abi as *const <super::Com::IBindCtx as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvobject),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectStorage<Impl: IOleItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, riid: &::windows::core::GUID, ppvstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectStorage(
                &*(&pszitem as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbc as *const <super::Com::IBindCtx as ::windows::core::Abi>::Abi as *const <super::Com::IBindCtx as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvstorage),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRunning<Impl: IOleItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRunning(&*(&pszitem as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleItemContainer>, ::windows::core::GetTrustLevel, GetObject::<Impl, OFFSET>, GetObjectStorage::<Impl, OFFSET>, IsRunning::<Impl, OFFSET>)
    }
}
pub trait IOleLinkImpl: Sized {
    fn SetUpdateOptions();
    fn GetUpdateOptions();
    fn SetSourceMoniker();
    fn GetSourceMoniker();
    fn SetSourceDisplayName();
    fn GetSourceDisplayName();
    fn BindToSource();
    fn BindIfRunning();
    fn GetBoundSource();
    fn UnbindSource();
    fn Update();
}
impl ::windows::core::RuntimeName for IOleLink {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleLink";
}
impl IOleLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleLinkImpl, const OFFSET: isize>() -> IOleLinkVtbl {
        unsafe extern "system" fn SetUpdateOptions<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUpdateOptions(dwupdateopt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateOptions<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdateOptions(::core::mem::transmute_copy(&pdwupdateopt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceMoniker<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, rclsid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSourceMoniker(&*(&pmk as *const <super::Com::IMoniker as ::windows::core::Abi>::Abi as *const <super::Com::IMoniker as ::windows::core::DefaultType>::DefaultType), &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceMoniker<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceMoniker(::core::mem::transmute_copy(&ppmk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceDisplayName<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSourceDisplayName(&*(&pszstatustext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceDisplayName<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceDisplayName(::core::mem::transmute_copy(&ppszdisplayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToSource<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindflags: u32, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindToSource(bindflags, &*(&pbc as *const <super::Com::IBindCtx as ::windows::core::Abi>::Abi as *const <super::Com::IBindCtx as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindIfRunning<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindIfRunning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundSource<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundSource(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnbindSource<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnbindSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update(&*(&pbc as *const <super::Com::IBindCtx as ::windows::core::Abi>::Abi as *const <super::Com::IBindCtx as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOleLink>,
            ::windows::core::GetTrustLevel,
            SetUpdateOptions::<Impl, OFFSET>,
            GetUpdateOptions::<Impl, OFFSET>,
            SetSourceMoniker::<Impl, OFFSET>,
            GetSourceMoniker::<Impl, OFFSET>,
            SetSourceDisplayName::<Impl, OFFSET>,
            GetSourceDisplayName::<Impl, OFFSET>,
            BindToSource::<Impl, OFFSET>,
            BindIfRunning::<Impl, OFFSET>,
            GetBoundSource::<Impl, OFFSET>,
            UnbindSource::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
        )
    }
}
pub trait IOleObjectImpl: Sized {
    fn SetClientSite();
    fn GetClientSite();
    fn SetHostNames();
    fn Close();
    fn SetMoniker();
    fn GetMoniker();
    fn InitFromData();
    fn GetClipboardData();
    fn DoVerb();
    fn EnumVerbs();
    fn Update();
    fn IsUpToDate();
    fn GetUserClassID();
    fn GetUserType();
    fn SetExtent();
    fn GetExtent();
    fn Advise();
    fn Unadvise();
    fn EnumAdvise();
    fn GetMiscStatus();
    fn SetColorScheme();
}
impl ::windows::core::RuntimeName for IOleObject {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleObject";
}
impl IOleObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleObjectImpl, const OFFSET: isize>() -> IOleObjectVtbl {
        unsafe extern "system" fn SetClientSite<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientsite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClientSite(&*(&pclientsite as *const <IOleClientSite as ::windows::core::Abi>::Abi as *const <IOleClientSite as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientSite<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclientsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientSite(::core::mem::transmute_copy(&ppclientsite)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostNames<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcontainerapp: super::super::Foundation::PWSTR, szcontainerobj: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHostNames(&*(&szcontainerapp as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&szcontainerobj as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsaveoption: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close(dwsaveoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMoniker<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwhichmoniker: u32, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMoniker(dwwhichmoniker, &*(&pmk as *const <super::Com::IMoniker as ::windows::core::Abi>::Abi as *const <super::Com::IMoniker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMoniker<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMoniker(dwassign, dwwhichmoniker, ::core::mem::transmute_copy(&ppmk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitFromData<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitFromData(&*(&pdataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), &*(&fcreation as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipboardData<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipboardData(dwreserved, ::core::mem::transmute_copy(&ppdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoVerb<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: ::windows::core::RawPtr, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoVerb(
                iverb,
                &*(&lpmsg as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::DefaultType>::DefaultType),
                &*(&pactivesite as *const <IOleClientSite as ::windows::core::Abi>::Abi as *const <IOleClientSite as ::windows::core::DefaultType>::DefaultType),
                lindex,
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&lprcposrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumVerbs<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumoleverb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumVerbs(::core::mem::transmute_copy(&ppenumoleverb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUpToDate<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUpToDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserClassID<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserClassID(::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserType<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwformoftype: u32, pszusertype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserType(dwformoftype, ::core::mem::transmute_copy(&pszusertype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtent<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExtent(dwdrawaspect, &*(&psizel as *const <super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtent<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtent(dwdrawaspect, ::core::mem::transmute_copy(&psizel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvsink: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(&*(&padvsink as *const <super::Com::IAdviseSink as ::windows::core::Abi>::Abi as *const <super::Com::IAdviseSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unadvise(dwconnection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAdvise<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAdvise(::core::mem::transmute_copy(&ppenumadvise)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMiscStatus<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMiscStatus(dwaspect, ::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorScheme<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColorScheme(&*(&plogpal as *const <super::super::Graphics::Gdi::LOGPALETTE as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::LOGPALETTE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOleObject>,
            ::windows::core::GetTrustLevel,
            SetClientSite::<Impl, OFFSET>,
            GetClientSite::<Impl, OFFSET>,
            SetHostNames::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            SetMoniker::<Impl, OFFSET>,
            GetMoniker::<Impl, OFFSET>,
            InitFromData::<Impl, OFFSET>,
            GetClipboardData::<Impl, OFFSET>,
            DoVerb::<Impl, OFFSET>,
            EnumVerbs::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
            IsUpToDate::<Impl, OFFSET>,
            GetUserClassID::<Impl, OFFSET>,
            GetUserType::<Impl, OFFSET>,
            SetExtent::<Impl, OFFSET>,
            GetExtent::<Impl, OFFSET>,
            Advise::<Impl, OFFSET>,
            Unadvise::<Impl, OFFSET>,
            EnumAdvise::<Impl, OFFSET>,
            GetMiscStatus::<Impl, OFFSET>,
            SetColorScheme::<Impl, OFFSET>,
        )
    }
}
pub trait IOleParentUndoUnitImpl: Sized + IOleUndoUnitImpl {
    fn Open();
    fn Close();
    fn Add();
    fn FindUnit();
    fn GetParentState();
}
impl ::windows::core::RuntimeName for IOleParentUndoUnit {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleParentUndoUnit";
}
impl IOleParentUndoUnitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleParentUndoUnitImpl, const OFFSET: isize>() -> IOleParentUndoUnitVtbl {
        unsafe extern "system" fn Open<Impl: IOleParentUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&ppuu as *const <IOleParentUndoUnit as ::windows::core::Abi>::Abi as *const <IOleParentUndoUnit as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IOleParentUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr, fcommit: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close(&*(&ppuu as *const <IOleParentUndoUnit as ::windows::core::Abi>::Abi as *const <IOleParentUndoUnit as ::windows::core::DefaultType>::DefaultType), &*(&fcommit as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IOleParentUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&puu as *const <IOleUndoUnit as ::windows::core::Abi>::Abi as *const <IOleUndoUnit as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUnit<Impl: IOleParentUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindUnit(&*(&puu as *const <IOleUndoUnit as ::windows::core::Abi>::Abi as *const <IOleUndoUnit as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentState<Impl: IOleParentUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentState(::core::mem::transmute_copy(&pdwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleParentUndoUnit>, ::windows::core::GetTrustLevel, Open::<Impl, OFFSET>, Close::<Impl, OFFSET>, Add::<Impl, OFFSET>, FindUnit::<Impl, OFFSET>, GetParentState::<Impl, OFFSET>)
    }
}
pub trait IOleUILinkContainerAImpl: Sized {
    fn GetNextLink();
    fn SetLinkUpdateOptions();
    fn GetLinkUpdateOptions();
    fn SetLinkSource();
    fn GetLinkSource();
    fn OpenLinkSource();
    fn UpdateLink();
    fn CancelLink();
}
impl ::windows::core::RuntimeName for IOleUILinkContainerA {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleUILinkContainerA";
}
impl IOleUILinkContainerAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerAImpl, const OFFSET: isize>() -> IOleUILinkContainerAVtbl {
        unsafe extern "system" fn GetNextLink<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextLink(dwlink) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLinkUpdateOptions(dwlink, dwupdateopt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinkUpdateOptions(dwlink, ::core::mem::transmute_copy(&lpdwupdateopt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkSource<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: super::super::Foundation::PSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLinkSource(dwlink, &*(&lpszdisplayname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), lenfilename, ::core::mem::transmute_copy(&pcheaten), &*(&fvalidatesource as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLinkSource<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PSTR, lplpszshortlinktype: *mut super::super::Foundation::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinkSource(dwlink, ::core::mem::transmute_copy(&lplpszdisplayname), ::core::mem::transmute_copy(&lplenfilename), ::core::mem::transmute_copy(&lplpszfulllinktype), ::core::mem::transmute_copy(&lplpszshortlinktype), ::core::mem::transmute_copy(&lpfsourceavailable), ::core::mem::transmute_copy(&lpfisselected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenLinkSource<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenLinkSource(dwlink) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateLink<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateLink(dwlink, &*(&ferrormessage as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&freserved as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelLink<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelLink(dwlink) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOleUILinkContainerA>,
            ::windows::core::GetTrustLevel,
            GetNextLink::<Impl, OFFSET>,
            SetLinkUpdateOptions::<Impl, OFFSET>,
            GetLinkUpdateOptions::<Impl, OFFSET>,
            SetLinkSource::<Impl, OFFSET>,
            GetLinkSource::<Impl, OFFSET>,
            OpenLinkSource::<Impl, OFFSET>,
            UpdateLink::<Impl, OFFSET>,
            CancelLink::<Impl, OFFSET>,
        )
    }
}
pub trait IOleUILinkContainerWImpl: Sized {
    fn GetNextLink();
    fn SetLinkUpdateOptions();
    fn GetLinkUpdateOptions();
    fn SetLinkSource();
    fn GetLinkSource();
    fn OpenLinkSource();
    fn UpdateLink();
    fn CancelLink();
}
impl ::windows::core::RuntimeName for IOleUILinkContainerW {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleUILinkContainerW";
}
impl IOleUILinkContainerWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerWImpl, const OFFSET: isize>() -> IOleUILinkContainerWVtbl {
        unsafe extern "system" fn GetNextLink<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextLink(dwlink) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLinkUpdateOptions(dwlink, dwupdateopt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinkUpdateOptions(dwlink, ::core::mem::transmute_copy(&lpdwupdateopt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkSource<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: super::super::Foundation::PWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLinkSource(dwlink, &*(&lpszdisplayname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lenfilename, ::core::mem::transmute_copy(&pcheaten), &*(&fvalidatesource as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLinkSource<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PWSTR, lplpszshortlinktype: *mut super::super::Foundation::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinkSource(dwlink, ::core::mem::transmute_copy(&lplpszdisplayname), ::core::mem::transmute_copy(&lplenfilename), ::core::mem::transmute_copy(&lplpszfulllinktype), ::core::mem::transmute_copy(&lplpszshortlinktype), ::core::mem::transmute_copy(&lpfsourceavailable), ::core::mem::transmute_copy(&lpfisselected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenLinkSource<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenLinkSource(dwlink) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateLink<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateLink(dwlink, &*(&ferrormessage as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&freserved as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelLink<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelLink(dwlink) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOleUILinkContainerW>,
            ::windows::core::GetTrustLevel,
            GetNextLink::<Impl, OFFSET>,
            SetLinkUpdateOptions::<Impl, OFFSET>,
            GetLinkUpdateOptions::<Impl, OFFSET>,
            SetLinkSource::<Impl, OFFSET>,
            GetLinkSource::<Impl, OFFSET>,
            OpenLinkSource::<Impl, OFFSET>,
            UpdateLink::<Impl, OFFSET>,
            CancelLink::<Impl, OFFSET>,
        )
    }
}
pub trait IOleUILinkInfoAImpl: Sized + IOleUILinkContainerAImpl {
    fn GetLastUpdate();
}
impl ::windows::core::RuntimeName for IOleUILinkInfoA {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleUILinkInfoA";
}
impl IOleUILinkInfoAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkInfoAImpl, const OFFSET: isize>() -> IOleUILinkInfoAVtbl {
        unsafe extern "system" fn GetLastUpdate<Impl: IOleUILinkInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastUpdate(dwlink, ::core::mem::transmute_copy(&lplastupdate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleUILinkInfoA>, ::windows::core::GetTrustLevel, GetLastUpdate::<Impl, OFFSET>)
    }
}
pub trait IOleUILinkInfoWImpl: Sized + IOleUILinkContainerWImpl {
    fn GetLastUpdate();
}
impl ::windows::core::RuntimeName for IOleUILinkInfoW {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleUILinkInfoW";
}
impl IOleUILinkInfoWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkInfoWImpl, const OFFSET: isize>() -> IOleUILinkInfoWVtbl {
        unsafe extern "system" fn GetLastUpdate<Impl: IOleUILinkInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastUpdate(dwlink, ::core::mem::transmute_copy(&lplastupdate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleUILinkInfoW>, ::windows::core::GetTrustLevel, GetLastUpdate::<Impl, OFFSET>)
    }
}
pub trait IOleUIObjInfoAImpl: Sized {
    fn GetObjectInfo();
    fn GetConvertInfo();
    fn ConvertObject();
    fn GetViewInfo();
    fn SetViewInfo();
}
impl ::windows::core::RuntimeName for IOleUIObjInfoA {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleUIObjInfoA";
}
impl IOleUIObjInfoAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoAImpl, const OFFSET: isize>() -> IOleUIObjInfoAVtbl {
        unsafe extern "system" fn GetObjectInfo<Impl: IOleUIObjInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PSTR, lplpsztype: *mut super::super::Foundation::PSTR, lplpszshorttype: *mut super::super::Foundation::PSTR, lplpszlocation: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectInfo(dwobject, ::core::mem::transmute_copy(&lpdwobjsize), ::core::mem::transmute_copy(&lplpszlabel), ::core::mem::transmute_copy(&lplpsztype), ::core::mem::transmute_copy(&lplpszshorttype), ::core::mem::transmute_copy(&lplpszlocation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConvertInfo<Impl: IOleUIObjInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConvertInfo(dwobject, ::core::mem::transmute_copy(&lpclassid), ::core::mem::transmute_copy(&lpwformat), ::core::mem::transmute_copy(&lpconvertdefaultclassid), ::core::mem::transmute_copy(&lplpclsidexclude), ::core::mem::transmute_copy(&lpcclsidexclude)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertObject<Impl: IOleUIObjInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertObject(dwobject, &*(&clsidnew as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewInfo<Impl: IOleUIObjInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewInfo(dwobject, phmetapict, pdvaspect, pncurrentscale) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewInfo<Impl: IOleUIObjInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetViewInfo(dwobject, hmetapict, dvaspect, ncurrentscale, &*(&brelativetoorig as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleUIObjInfoA>, ::windows::core::GetTrustLevel, GetObjectInfo::<Impl, OFFSET>, GetConvertInfo::<Impl, OFFSET>, ConvertObject::<Impl, OFFSET>, GetViewInfo::<Impl, OFFSET>, SetViewInfo::<Impl, OFFSET>)
    }
}
pub trait IOleUIObjInfoWImpl: Sized {
    fn GetObjectInfo();
    fn GetConvertInfo();
    fn ConvertObject();
    fn GetViewInfo();
    fn SetViewInfo();
}
impl ::windows::core::RuntimeName for IOleUIObjInfoW {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleUIObjInfoW";
}
impl IOleUIObjInfoWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoWImpl, const OFFSET: isize>() -> IOleUIObjInfoWVtbl {
        unsafe extern "system" fn GetObjectInfo<Impl: IOleUIObjInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PWSTR, lplpsztype: *mut super::super::Foundation::PWSTR, lplpszshorttype: *mut super::super::Foundation::PWSTR, lplpszlocation: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectInfo(dwobject, ::core::mem::transmute_copy(&lpdwobjsize), ::core::mem::transmute_copy(&lplpszlabel), ::core::mem::transmute_copy(&lplpsztype), ::core::mem::transmute_copy(&lplpszshorttype), ::core::mem::transmute_copy(&lplpszlocation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConvertInfo<Impl: IOleUIObjInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConvertInfo(dwobject, ::core::mem::transmute_copy(&lpclassid), ::core::mem::transmute_copy(&lpwformat), ::core::mem::transmute_copy(&lpconvertdefaultclassid), ::core::mem::transmute_copy(&lplpclsidexclude), ::core::mem::transmute_copy(&lpcclsidexclude)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertObject<Impl: IOleUIObjInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertObject(dwobject, &*(&clsidnew as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewInfo<Impl: IOleUIObjInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewInfo(dwobject, phmetapict, pdvaspect, pncurrentscale) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewInfo<Impl: IOleUIObjInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetViewInfo(dwobject, hmetapict, dvaspect, ncurrentscale, &*(&brelativetoorig as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleUIObjInfoW>, ::windows::core::GetTrustLevel, GetObjectInfo::<Impl, OFFSET>, GetConvertInfo::<Impl, OFFSET>, ConvertObject::<Impl, OFFSET>, GetViewInfo::<Impl, OFFSET>, SetViewInfo::<Impl, OFFSET>)
    }
}
pub trait IOleUndoManagerImpl: Sized {
    fn Open();
    fn Close();
    fn Add();
    fn GetOpenParentState();
    fn DiscardFrom();
    fn UndoTo();
    fn RedoTo();
    fn EnumUndoable();
    fn EnumRedoable();
    fn GetLastUndoDescription();
    fn GetLastRedoDescription();
    fn Enable();
}
impl ::windows::core::RuntimeName for IOleUndoManager {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleUndoManager";
}
impl IOleUndoManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManagerImpl, const OFFSET: isize>() -> IOleUndoManagerVtbl {
        unsafe extern "system" fn Open<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&ppuu as *const <IOleParentUndoUnit as ::windows::core::Abi>::Abi as *const <IOleParentUndoUnit as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr, fcommit: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close(&*(&ppuu as *const <IOleParentUndoUnit as ::windows::core::Abi>::Abi as *const <IOleParentUndoUnit as ::windows::core::DefaultType>::DefaultType), &*(&fcommit as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&puu as *const <IOleUndoUnit as ::windows::core::Abi>::Abi as *const <IOleUndoUnit as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpenParentState<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpenParentState(::core::mem::transmute_copy(&pdwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardFrom<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscardFrom(&*(&puu as *const <IOleUndoUnit as ::windows::core::Abi>::Abi as *const <IOleUndoUnit as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UndoTo<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UndoTo(&*(&puu as *const <IOleUndoUnit as ::windows::core::Abi>::Abi as *const <IOleUndoUnit as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedoTo<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedoTo(&*(&puu as *const <IOleUndoUnit as ::windows::core::Abi>::Abi as *const <IOleUndoUnit as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumUndoable<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumUndoable(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRedoable<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRedoable(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastUndoDescription<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastUndoDescription(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastRedoDescription<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastRedoDescription(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enable(&*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOleUndoManager>,
            ::windows::core::GetTrustLevel,
            Open::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            Add::<Impl, OFFSET>,
            GetOpenParentState::<Impl, OFFSET>,
            DiscardFrom::<Impl, OFFSET>,
            UndoTo::<Impl, OFFSET>,
            RedoTo::<Impl, OFFSET>,
            EnumUndoable::<Impl, OFFSET>,
            EnumRedoable::<Impl, OFFSET>,
            GetLastUndoDescription::<Impl, OFFSET>,
            GetLastRedoDescription::<Impl, OFFSET>,
            Enable::<Impl, OFFSET>,
        )
    }
}
pub trait IOleUndoUnitImpl: Sized {
    fn Do();
    fn GetDescription();
    fn GetUnitType();
    fn OnNextAdd();
}
impl ::windows::core::RuntimeName for IOleUndoUnit {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleUndoUnit";
}
impl IOleUndoUnitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoUnitImpl, const OFFSET: isize>() -> IOleUndoUnitVtbl {
        unsafe extern "system" fn Do<Impl: IOleUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pundomanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Do(&*(&pundomanager as *const <IOleUndoManager as ::windows::core::Abi>::Abi as *const <IOleUndoManager as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IOleUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnitType<Impl: IOleUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnitType(::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&plid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnNextAdd<Impl: IOleUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnNextAdd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleUndoUnit>, ::windows::core::GetTrustLevel, Do::<Impl, OFFSET>, GetDescription::<Impl, OFFSET>, GetUnitType::<Impl, OFFSET>, OnNextAdd::<Impl, OFFSET>)
    }
}
pub trait IOleWindowImpl: Sized {
    fn GetWindow();
    fn ContextSensitiveHelp();
}
impl ::windows::core::RuntimeName for IOleWindow {
    const NAME: &'static str = "Windows.Win32.System.Ole.IOleWindow";
}
impl IOleWindowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleWindowImpl, const OFFSET: isize>() -> IOleWindowVtbl {
        unsafe extern "system" fn GetWindow<Impl: IOleWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow(::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Impl: IOleWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextSensitiveHelp(&*(&fentermode as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOleWindow>, ::windows::core::GetTrustLevel, GetWindow::<Impl, OFFSET>, ContextSensitiveHelp::<Impl, OFFSET>)
    }
}
pub trait IParseDisplayNameImpl: Sized {
    fn ParseDisplayName();
}
impl ::windows::core::RuntimeName for IParseDisplayName {
    const NAME: &'static str = "Windows.Win32.System.Ole.IParseDisplayName";
}
impl IParseDisplayNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IParseDisplayNameImpl, const OFFSET: isize>() -> IParseDisplayNameVtbl {
        unsafe extern "system" fn ParseDisplayName<Impl: IParseDisplayNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParseDisplayName(&*(&pbc as *const <super::Com::IBindCtx as ::windows::core::Abi>::Abi as *const <super::Com::IBindCtx as ::windows::core::DefaultType>::DefaultType), &*(&pszdisplayname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&ppmkout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IParseDisplayName>, ::windows::core::GetTrustLevel, ParseDisplayName::<Impl, OFFSET>)
    }
}
pub trait IPerPropertyBrowsingImpl: Sized {
    fn GetDisplayString();
    fn MapPropertyToPage();
    fn GetPredefinedStrings();
    fn GetPredefinedValue();
}
impl ::windows::core::RuntimeName for IPerPropertyBrowsing {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPerPropertyBrowsing";
}
impl IPerPropertyBrowsingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerPropertyBrowsingImpl, const OFFSET: isize>() -> IPerPropertyBrowsingVtbl {
        unsafe extern "system" fn GetDisplayString<Impl: IPerPropertyBrowsingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayString(dispid, ::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapPropertyToPage<Impl: IPerPropertyBrowsingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapPropertyToPage(dispid, ::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPredefinedStrings<Impl: IPerPropertyBrowsingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPredefinedStrings(dispid, ::core::mem::transmute_copy(&pcastringsout), ::core::mem::transmute_copy(&pcacookiesout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPredefinedValue<Impl: IPerPropertyBrowsingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, dwcookie: u32, pvarout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPredefinedValue(dispid, dwcookie, ::core::mem::transmute_copy(&pvarout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPerPropertyBrowsing>, ::windows::core::GetTrustLevel, GetDisplayString::<Impl, OFFSET>, MapPropertyToPage::<Impl, OFFSET>, GetPredefinedStrings::<Impl, OFFSET>, GetPredefinedValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistPropertyBagImpl: Sized + IPersistImpl {
    fn InitNew();
    fn Load();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPersistPropertyBag {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPersistPropertyBag";
}
#[cfg(feature = "Win32_System_Com")]
impl IPersistPropertyBagVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBagImpl, const OFFSET: isize>() -> IPersistPropertyBagVtbl {
        unsafe extern "system" fn InitNew<Impl: IPersistPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitNew() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IPersistPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, perrorlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(&*(&ppropbag as *const <super::Com::StructuredStorage::IPropertyBag as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::IPropertyBag as ::windows::core::DefaultType>::DefaultType), &*(&perrorlog as *const <super::Com::IErrorLog as ::windows::core::Abi>::Abi as *const <super::Com::IErrorLog as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IPersistPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save(
                &*(&ppropbag as *const <super::Com::StructuredStorage::IPropertyBag as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::IPropertyBag as ::windows::core::DefaultType>::DefaultType),
                &*(&fcleardirty as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fsaveallproperties as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPersistPropertyBag>, ::windows::core::GetTrustLevel, InitNew::<Impl, OFFSET>, Load::<Impl, OFFSET>, Save::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistPropertyBag2Impl: Sized + IPersistImpl {
    fn InitNew();
    fn Load();
    fn Save();
    fn IsDirty();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPersistPropertyBag2 {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPersistPropertyBag2";
}
#[cfg(feature = "Win32_System_Com")]
impl IPersistPropertyBag2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag2Impl, const OFFSET: isize>() -> IPersistPropertyBag2Vtbl {
        unsafe extern "system" fn InitNew<Impl: IPersistPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitNew() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IPersistPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, perrlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(&*(&ppropbag as *const <super::Com::StructuredStorage::IPropertyBag2 as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::IPropertyBag2 as ::windows::core::DefaultType>::DefaultType), &*(&perrlog as *const <super::Com::IErrorLog as ::windows::core::Abi>::Abi as *const <super::Com::IErrorLog as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IPersistPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save(
                &*(&ppropbag as *const <super::Com::StructuredStorage::IPropertyBag2 as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::IPropertyBag2 as ::windows::core::DefaultType>::DefaultType),
                &*(&fcleardirty as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fsaveallproperties as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDirty<Impl: IPersistPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDirty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPersistPropertyBag2>, ::windows::core::GetTrustLevel, InitNew::<Impl, OFFSET>, Load::<Impl, OFFSET>, Save::<Impl, OFFSET>, IsDirty::<Impl, OFFSET>)
    }
}
pub trait IPictureImpl: Sized {
    fn Handle();
    fn hPal();
    fn Type();
    fn Width();
    fn Height();
    fn Render();
    fn set_hPal();
    fn CurDC();
    fn SelectPicture();
    fn KeepOriginalFormat();
    fn SetKeepOriginalFormat();
    fn PictureChanged();
    fn SaveAsFile();
    fn Attributes();
}
impl ::windows::core::RuntimeName for IPicture {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPicture";
}
impl IPictureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPictureImpl, const OFFSET: isize>() -> IPictureVtbl {
        unsafe extern "system" fn Handle<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hPal<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phpal: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).hPal(::core::mem::transmute_copy(&phpal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width(::core::mem::transmute_copy(&pwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height(::core::mem::transmute_copy(&pheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Render<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Render(&*(&hdc as *const <super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), x, y, cx, cy, xsrc, ysrc, cxsrc, cysrc, &*(&prcwbounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_hPal<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpal: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).set_hPal(hpal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurDC<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurDC(::core::mem::transmute_copy(&phdc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPicture<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectPicture(&*(&hdcin as *const <super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phdcout), ::core::mem::transmute_copy(&phbmpout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeepOriginalFormat<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeepOriginalFormat(::core::mem::transmute_copy(&pkeep)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetKeepOriginalFormat(&*(&keep as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PictureChanged<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PictureChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsFile<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsFile(&*(&pstream as *const <super::Com::IStream as ::windows::core::Abi>::Abi as *const <super::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&fsavememcopy as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcbsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes(::core::mem::transmute_copy(&pdwattr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPicture>,
            ::windows::core::GetTrustLevel,
            Handle::<Impl, OFFSET>,
            hPal::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            Width::<Impl, OFFSET>,
            Height::<Impl, OFFSET>,
            Render::<Impl, OFFSET>,
            set_hPal::<Impl, OFFSET>,
            CurDC::<Impl, OFFSET>,
            SelectPicture::<Impl, OFFSET>,
            KeepOriginalFormat::<Impl, OFFSET>,
            SetKeepOriginalFormat::<Impl, OFFSET>,
            PictureChanged::<Impl, OFFSET>,
            SaveAsFile::<Impl, OFFSET>,
            Attributes::<Impl, OFFSET>,
        )
    }
}
pub trait IPicture2Impl: Sized {
    fn Handle();
    fn hPal();
    fn Type();
    fn Width();
    fn Height();
    fn Render();
    fn set_hPal();
    fn CurDC();
    fn SelectPicture();
    fn KeepOriginalFormat();
    fn SetKeepOriginalFormat();
    fn PictureChanged();
    fn SaveAsFile();
    fn Attributes();
}
impl ::windows::core::RuntimeName for IPicture2 {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPicture2";
}
impl IPicture2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2Impl, const OFFSET: isize>() -> IPicture2Vtbl {
        unsafe extern "system" fn Handle<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hPal<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phpal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).hPal(::core::mem::transmute_copy(&phpal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width(::core::mem::transmute_copy(&pwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height(::core::mem::transmute_copy(&pheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Render<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Render(&*(&hdc as *const <super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), x, y, cx, cy, xsrc, ysrc, cxsrc, cysrc, &*(&prcwbounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_hPal<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpal: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).set_hPal(hpal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurDC<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurDC(::core::mem::transmute_copy(&phdc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPicture<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectPicture(&*(&hdcin as *const <super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phdcout), ::core::mem::transmute_copy(&phbmpout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeepOriginalFormat<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeepOriginalFormat(::core::mem::transmute_copy(&pkeep)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetKeepOriginalFormat(&*(&keep as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PictureChanged<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PictureChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsFile<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsFile(&*(&pstream as *const <super::Com::IStream as ::windows::core::Abi>::Abi as *const <super::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&fsavememcopy as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcbsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes(::core::mem::transmute_copy(&pdwattr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPicture2>,
            ::windows::core::GetTrustLevel,
            Handle::<Impl, OFFSET>,
            hPal::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            Width::<Impl, OFFSET>,
            Height::<Impl, OFFSET>,
            Render::<Impl, OFFSET>,
            set_hPal::<Impl, OFFSET>,
            CurDC::<Impl, OFFSET>,
            SelectPicture::<Impl, OFFSET>,
            KeepOriginalFormat::<Impl, OFFSET>,
            SetKeepOriginalFormat::<Impl, OFFSET>,
            PictureChanged::<Impl, OFFSET>,
            SaveAsFile::<Impl, OFFSET>,
            Attributes::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPictureDispImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPictureDisp {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPictureDisp";
}
#[cfg(feature = "Win32_System_Com")]
impl IPictureDispVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPictureDispImpl, const OFFSET: isize>() -> IPictureDispVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPictureDisp>, ::windows::core::GetTrustLevel)
    }
}
pub trait IPointerInactiveImpl: Sized {
    fn GetActivationPolicy();
    fn OnInactiveMouseMove();
    fn OnInactiveSetCursor();
}
impl ::windows::core::RuntimeName for IPointerInactive {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPointerInactive";
}
impl IPointerInactiveVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerInactiveImpl, const OFFSET: isize>() -> IPointerInactiveVtbl {
        unsafe extern "system" fn GetActivationPolicy<Impl: IPointerInactiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpolicy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivationPolicy(::core::mem::transmute_copy(&pdwpolicy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInactiveMouseMove<Impl: IPointerInactiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInactiveMouseMove(&*(&prectbounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), x, y, grfkeystate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInactiveSetCursor<Impl: IPointerInactiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInactiveSetCursor(&*(&prectbounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), x, y, dwmousemsg, &*(&fsetalways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPointerInactive>, ::windows::core::GetTrustLevel, GetActivationPolicy::<Impl, OFFSET>, OnInactiveMouseMove::<Impl, OFFSET>, OnInactiveSetCursor::<Impl, OFFSET>)
    }
}
pub trait IPrintImpl: Sized {
    fn SetInitialPageNum();
    fn GetPageInfo();
    fn Print();
}
impl ::windows::core::RuntimeName for IPrint {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPrint";
}
impl IPrintVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintImpl, const OFFSET: isize>() -> IPrintVtbl {
        unsafe extern "system" fn SetInitialPageNum<Impl: IPrintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nfirstpage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInitialPageNum(nfirstpage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageInfo<Impl: IPrintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageInfo(::core::mem::transmute_copy(&pnfirstpage), ::core::mem::transmute_copy(&pcpages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Print<Impl: IPrintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: ::windows::core::RawPtr, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Print(
                grfflags,
                &*(&pptd as *const <super::Com::DVTARGETDEVICE as ::windows::core::Abi>::Abi as *const <super::Com::DVTARGETDEVICE as ::windows::core::DefaultType>::DefaultType),
                &*(&pppageset as *const <PAGESET as ::windows::core::Abi>::Abi as *const <PAGESET as ::windows::core::DefaultType>::DefaultType),
                &*(&pstgmoptions as *const <super::Com::STGMEDIUM as ::windows::core::Abi>::Abi as *const <super::Com::STGMEDIUM as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallback as *const <IContinueCallback as ::windows::core::Abi>::Abi as *const <IContinueCallback as ::windows::core::DefaultType>::DefaultType),
                nfirstpage,
                ::core::mem::transmute_copy(&pcpagesprinted),
                ::core::mem::transmute_copy(&pnlastpage),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint>, ::windows::core::GetTrustLevel, SetInitialPageNum::<Impl, OFFSET>, GetPageInfo::<Impl, OFFSET>, Print::<Impl, OFFSET>)
    }
}
pub trait IPropertyNotifySinkImpl: Sized {
    fn OnChanged();
    fn OnRequestEdit();
}
impl ::windows::core::RuntimeName for IPropertyNotifySink {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPropertyNotifySink";
}
impl IPropertyNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyNotifySinkImpl, const OFFSET: isize>() -> IPropertyNotifySinkVtbl {
        unsafe extern "system" fn OnChanged<Impl: IPropertyNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnChanged(dispid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnRequestEdit<Impl: IPropertyNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnRequestEdit(dispid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPropertyNotifySink>, ::windows::core::GetTrustLevel, OnChanged::<Impl, OFFSET>, OnRequestEdit::<Impl, OFFSET>)
    }
}
pub trait IPropertyPageImpl: Sized {
    fn SetPageSite();
    fn Activate();
    fn Deactivate();
    fn GetPageInfo();
    fn SetObjects();
    fn Show();
    fn Move();
    fn IsPageDirty();
    fn Apply();
    fn Help();
    fn TranslateAccelerator();
}
impl ::windows::core::RuntimeName for IPropertyPage {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPropertyPage";
}
impl IPropertyPageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPageImpl, const OFFSET: isize>() -> IPropertyPageVtbl {
        unsafe extern "system" fn SetPageSite<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagesite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPageSite(&*(&ppagesite as *const <IPropertyPageSite as ::windows::core::Abi>::Abi as *const <IPropertyPageSite as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&bmodal as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageInfo<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppageinfo: *mut PROPPAGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageInfo(::core::mem::transmute_copy(&ppageinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjects<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32, ppunk: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetObjects(cobjects, &*(&ppunk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncmdshow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(ncmdshow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(&*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPageDirty<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPageDirty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Apply<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Apply() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Help<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelpdir: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Help(&*(&pszhelpdir as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateAccelerator(&*(&pmsg as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPropertyPage>,
            ::windows::core::GetTrustLevel,
            SetPageSite::<Impl, OFFSET>,
            Activate::<Impl, OFFSET>,
            Deactivate::<Impl, OFFSET>,
            GetPageInfo::<Impl, OFFSET>,
            SetObjects::<Impl, OFFSET>,
            Show::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            IsPageDirty::<Impl, OFFSET>,
            Apply::<Impl, OFFSET>,
            Help::<Impl, OFFSET>,
            TranslateAccelerator::<Impl, OFFSET>,
        )
    }
}
pub trait IPropertyPage2Impl: Sized + IPropertyPageImpl {
    fn EditProperty();
}
impl ::windows::core::RuntimeName for IPropertyPage2 {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPropertyPage2";
}
impl IPropertyPage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage2Impl, const OFFSET: isize>() -> IPropertyPage2Vtbl {
        unsafe extern "system" fn EditProperty<Impl: IPropertyPage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EditProperty(dispid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPropertyPage2>, ::windows::core::GetTrustLevel, EditProperty::<Impl, OFFSET>)
    }
}
pub trait IPropertyPageSiteImpl: Sized {
    fn OnStatusChange();
    fn GetLocaleID();
    fn GetPageContainer();
    fn TranslateAccelerator();
}
impl ::windows::core::RuntimeName for IPropertyPageSite {
    const NAME: &'static str = "Windows.Win32.System.Ole.IPropertyPageSite";
}
impl IPropertyPageSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPageSiteImpl, const OFFSET: isize>() -> IPropertyPageSiteVtbl {
        unsafe extern "system" fn OnStatusChange<Impl: IPropertyPageSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: PROPPAGESTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStatusChange(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleID<Impl: IPropertyPageSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocaleid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocaleID(::core::mem::transmute_copy(&plocaleid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageContainer<Impl: IPropertyPageSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageContainer(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IPropertyPageSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateAccelerator(&*(&pmsg as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPropertyPageSite>, ::windows::core::GetTrustLevel, OnStatusChange::<Impl, OFFSET>, GetLocaleID::<Impl, OFFSET>, GetPageContainer::<Impl, OFFSET>, TranslateAccelerator::<Impl, OFFSET>)
    }
}
pub trait IProtectFocusImpl: Sized {
    fn AllowFocusChange();
}
impl ::windows::core::RuntimeName for IProtectFocus {
    const NAME: &'static str = "Windows.Win32.System.Ole.IProtectFocus";
}
impl IProtectFocusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectFocusImpl, const OFFSET: isize>() -> IProtectFocusVtbl {
        unsafe extern "system" fn AllowFocusChange<Impl: IProtectFocusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusChange(::core::mem::transmute_copy(&pfallow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectFocus>, ::windows::core::GetTrustLevel, AllowFocusChange::<Impl, OFFSET>)
    }
}
pub trait IProtectedModeMenuServicesImpl: Sized {
    fn CreateMenu();
    fn LoadMenu();
    fn LoadMenuID();
}
impl ::windows::core::RuntimeName for IProtectedModeMenuServices {
    const NAME: &'static str = "Windows.Win32.System.Ole.IProtectedModeMenuServices";
}
impl IProtectedModeMenuServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedModeMenuServicesImpl, const OFFSET: isize>() -> IProtectedModeMenuServicesVtbl {
        unsafe extern "system" fn CreateMenu<Impl: IProtectedModeMenuServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMenu(::core::mem::transmute_copy(&phmenu)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMenu<Impl: IProtectedModeMenuServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmodulename: super::super::Foundation::PWSTR, pszmenuname: super::super::Foundation::PWSTR, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadMenu(&*(&pszmodulename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszmenuname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phmenu)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMenuID<Impl: IProtectedModeMenuServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmodulename: super::super::Foundation::PWSTR, wresourceid: u16, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadMenuID(&*(&pszmodulename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), wresourceid, ::core::mem::transmute_copy(&phmenu)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectedModeMenuServices>, ::windows::core::GetTrustLevel, CreateMenu::<Impl, OFFSET>, LoadMenu::<Impl, OFFSET>, LoadMenuID::<Impl, OFFSET>)
    }
}
pub trait IProvideClassInfoImpl: Sized {
    fn GetClassInfo();
}
impl ::windows::core::RuntimeName for IProvideClassInfo {
    const NAME: &'static str = "Windows.Win32.System.Ole.IProvideClassInfo";
}
impl IProvideClassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideClassInfoImpl, const OFFSET: isize>() -> IProvideClassInfoVtbl {
        unsafe extern "system" fn GetClassInfo<Impl: IProvideClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppti: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassInfo(::core::mem::transmute_copy(&ppti)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvideClassInfo>, ::windows::core::GetTrustLevel, GetClassInfo::<Impl, OFFSET>)
    }
}
pub trait IProvideClassInfo2Impl: Sized + IProvideClassInfoImpl {
    fn GetGUID();
}
impl ::windows::core::RuntimeName for IProvideClassInfo2 {
    const NAME: &'static str = "Windows.Win32.System.Ole.IProvideClassInfo2";
}
impl IProvideClassInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideClassInfo2Impl, const OFFSET: isize>() -> IProvideClassInfo2Vtbl {
        unsafe extern "system" fn GetGUID<Impl: IProvideClassInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwguidkind: u32, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID(dwguidkind, ::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvideClassInfo2>, ::windows::core::GetTrustLevel, GetGUID::<Impl, OFFSET>)
    }
}
pub trait IProvideMultipleClassInfoImpl: Sized + IProvideClassInfo2Impl + IProvideClassInfoImpl {
    fn GetMultiTypeInfoCount();
    fn GetInfoOfIndex();
}
impl ::windows::core::RuntimeName for IProvideMultipleClassInfo {
    const NAME: &'static str = "Windows.Win32.System.Ole.IProvideMultipleClassInfo";
}
impl IProvideMultipleClassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideMultipleClassInfoImpl, const OFFSET: isize>() -> IProvideMultipleClassInfoVtbl {
        unsafe extern "system" fn GetMultiTypeInfoCount<Impl: IProvideMultipleClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcti: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMultiTypeInfoCount(::core::mem::transmute_copy(&pcti)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfoOfIndex<Impl: IProvideMultipleClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut ::windows::core::RawPtr, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows::core::GUID, piidsource: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfoOfIndex(iti, dwflags, ::core::mem::transmute_copy(&ppticoclass), ::core::mem::transmute_copy(&pdwtiflags), ::core::mem::transmute_copy(&pcdispidreserved), ::core::mem::transmute_copy(&piidprimary), ::core::mem::transmute_copy(&piidsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvideMultipleClassInfo>, ::windows::core::GetTrustLevel, GetMultiTypeInfoCount::<Impl, OFFSET>, GetInfoOfIndex::<Impl, OFFSET>)
    }
}
pub trait IProvideRuntimeContextImpl: Sized {
    fn GetCurrentSourceContext();
}
impl ::windows::core::RuntimeName for IProvideRuntimeContext {
    const NAME: &'static str = "Windows.Win32.System.Ole.IProvideRuntimeContext";
}
impl IProvideRuntimeContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideRuntimeContextImpl, const OFFSET: isize>() -> IProvideRuntimeContextVtbl {
        unsafe extern "system" fn GetCurrentSourceContext<Impl: IProvideRuntimeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcontext: *mut usize, pfexecutingglobalcode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSourceContext(::core::mem::transmute_copy(&pdwcontext), ::core::mem::transmute_copy(&pfexecutingglobalcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvideRuntimeContext>, ::windows::core::GetTrustLevel, GetCurrentSourceContext::<Impl, OFFSET>)
    }
}
pub trait IQuickActivateImpl: Sized {
    fn QuickActivate();
    fn SetContentExtent();
    fn GetContentExtent();
}
impl ::windows::core::RuntimeName for IQuickActivate {
    const NAME: &'static str = "Windows.Win32.System.Ole.IQuickActivate";
}
impl IQuickActivateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuickActivateImpl, const OFFSET: isize>() -> IQuickActivateVtbl {
        unsafe extern "system" fn QuickActivate<Impl: IQuickActivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuickActivate(&*(&pqacontainer as *const <QACONTAINER as ::windows::core::Abi>::Abi as *const <QACONTAINER as ::windows::core::DefaultType>::DefaultType), &*(&pqacontrol as *const <QACONTROL as ::windows::core::Abi>::Abi as *const <QACONTROL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentExtent<Impl: IQuickActivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContentExtent(&*(&psizel as *const <super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentExtent<Impl: IQuickActivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentExtent(::core::mem::transmute_copy(&psizel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IQuickActivate>, ::windows::core::GetTrustLevel, QuickActivate::<Impl, OFFSET>, SetContentExtent::<Impl, OFFSET>, GetContentExtent::<Impl, OFFSET>)
    }
}
pub trait IRecordInfoImpl: Sized {
    fn RecordInit();
    fn RecordClear();
    fn RecordCopy();
    fn GetGuid();
    fn GetName();
    fn GetSize();
    fn GetTypeInfo();
    fn GetField();
    fn GetFieldNoCopy();
    fn PutField();
    fn PutFieldNoCopy();
    fn GetFieldNames();
    fn IsMatchingType();
    fn RecordCreate();
    fn RecordCreateCopy();
    fn RecordDestroy();
}
impl ::windows::core::RuntimeName for IRecordInfo {
    const NAME: &'static str = "Windows.Win32.System.Ole.IRecordInfo";
}
impl IRecordInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfoImpl, const OFFSET: isize>() -> IRecordInfoVtbl {
        unsafe extern "system" fn RecordInit<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordInit(::core::mem::transmute_copy(&pvnew)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordClear<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordClear(&*(&pvexisting as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordCopy<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordCopy(&*(&pvexisting as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvnew)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuid<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuid(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&pcbsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeInfo(::core::mem::transmute_copy(&pptypeinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetField<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetField(&*(&pvdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&szfieldname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarfield)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFieldNoCopy<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *mut super::Com::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFieldNoCopy(&*(&pvdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&szfieldname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarfield), ::core::mem::transmute_copy(&ppvdatacarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutField<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PutField(
                wflags,
                &*(&pvdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&szfieldname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarfield as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutFieldNoCopy<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PutFieldNoCopy(
                wflags,
                &*(&pvdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&szfieldname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarfield as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFieldNames<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnames: *mut u32, rgbstrnames: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFieldNames(pcnames, ::core::mem::transmute_copy(&rgbstrnames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMatchingType<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordinfo: ::windows::core::RawPtr) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMatchingType(&*(&precordinfo as *const <IRecordInfo as ::windows::core::Abi>::Abi as *const <IRecordInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordCreate<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordCreate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordCreateCopy<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordCreateCopy(&*(&pvsource as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvdest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordDestroy<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvrecord: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordDestroy(&*(&pvrecord as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRecordInfo>,
            ::windows::core::GetTrustLevel,
            RecordInit::<Impl, OFFSET>,
            RecordClear::<Impl, OFFSET>,
            RecordCopy::<Impl, OFFSET>,
            GetGuid::<Impl, OFFSET>,
            GetName::<Impl, OFFSET>,
            GetSize::<Impl, OFFSET>,
            GetTypeInfo::<Impl, OFFSET>,
            GetField::<Impl, OFFSET>,
            GetFieldNoCopy::<Impl, OFFSET>,
            PutField::<Impl, OFFSET>,
            PutFieldNoCopy::<Impl, OFFSET>,
            GetFieldNames::<Impl, OFFSET>,
            IsMatchingType::<Impl, OFFSET>,
            RecordCreate::<Impl, OFFSET>,
            RecordCreateCopy::<Impl, OFFSET>,
            RecordDestroy::<Impl, OFFSET>,
        )
    }
}
pub trait ISimpleFrameSiteImpl: Sized {
    fn PreMessageFilter();
    fn PostMessageFilter();
}
impl ::windows::core::RuntimeName for ISimpleFrameSite {
    const NAME: &'static str = "Windows.Win32.System.Ole.ISimpleFrameSite";
}
impl ISimpleFrameSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleFrameSiteImpl, const OFFSET: isize>() -> ISimpleFrameSiteVtbl {
        unsafe extern "system" fn PreMessageFilter<Impl: ISimpleFrameSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreMessageFilter(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                msg,
                &*(&wp as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lp as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plresult),
                ::core::mem::transmute_copy(&pdwcookie),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostMessageFilter<Impl: ISimpleFrameSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostMessageFilter(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                msg,
                &*(&wp as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lp as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plresult),
                dwcookie,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimpleFrameSite>, ::windows::core::GetTrustLevel, PreMessageFilter::<Impl, OFFSET>, PostMessageFilter::<Impl, OFFSET>)
    }
}
pub trait ISpecifyPropertyPagesImpl: Sized {
    fn GetPages();
}
impl ::windows::core::RuntimeName for ISpecifyPropertyPages {
    const NAME: &'static str = "Windows.Win32.System.Ole.ISpecifyPropertyPages";
}
impl ISpecifyPropertyPagesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpecifyPropertyPagesImpl, const OFFSET: isize>() -> ISpecifyPropertyPagesVtbl {
        unsafe extern "system" fn GetPages<Impl: ISpecifyPropertyPagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppages: *mut CAUUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPages(::core::mem::transmute_copy(&ppages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpecifyPropertyPages>, ::windows::core::GetTrustLevel, GetPages::<Impl, OFFSET>)
    }
}
pub trait ITypeChangeEventsImpl: Sized {
    fn RequestTypeChange();
    fn AfterTypeChange();
}
impl ::windows::core::RuntimeName for ITypeChangeEvents {
    const NAME: &'static str = "Windows.Win32.System.Ole.ITypeChangeEvents";
}
impl ITypeChangeEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeChangeEventsImpl, const OFFSET: isize>() -> ITypeChangeEventsVtbl {
        unsafe extern "system" fn RequestTypeChange<Impl: ITypeChangeEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfobefore: ::windows::core::RawPtr, pstrname: super::super::Foundation::PWSTR, pfcancel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestTypeChange(changekind, &*(&ptinfobefore as *const <super::Com::ITypeInfo as ::windows::core::Abi>::Abi as *const <super::Com::ITypeInfo as ::windows::core::DefaultType>::DefaultType), &*(&pstrname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfcancel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AfterTypeChange<Impl: ITypeChangeEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfoafter: ::windows::core::RawPtr, pstrname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AfterTypeChange(changekind, &*(&ptinfoafter as *const <super::Com::ITypeInfo as ::windows::core::Abi>::Abi as *const <super::Com::ITypeInfo as ::windows::core::DefaultType>::DefaultType), &*(&pstrname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITypeChangeEvents>, ::windows::core::GetTrustLevel, RequestTypeChange::<Impl, OFFSET>, AfterTypeChange::<Impl, OFFSET>)
    }
}
pub trait ITypeFactoryImpl: Sized {
    fn CreateFromTypeInfo();
}
impl ::windows::core::RuntimeName for ITypeFactory {
    const NAME: &'static str = "Windows.Win32.System.Ole.ITypeFactory";
}
impl ITypeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeFactoryImpl, const OFFSET: isize>() -> ITypeFactoryVtbl {
        unsafe extern "system" fn CreateFromTypeInfo<Impl: ITypeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeinfo: ::windows::core::RawPtr, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromTypeInfo(&*(&ptypeinfo as *const <super::Com::ITypeInfo as ::windows::core::Abi>::Abi as *const <super::Com::ITypeInfo as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITypeFactory>, ::windows::core::GetTrustLevel, CreateFromTypeInfo::<Impl, OFFSET>)
    }
}
pub trait ITypeMarshalImpl: Sized {
    fn Size();
    fn Marshal();
    fn Unmarshal();
    fn Free();
}
impl ::windows::core::RuntimeName for ITypeMarshal {
    const NAME: &'static str = "Windows.Win32.System.Ole.ITypeMarshal";
}
impl ITypeMarshalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeMarshalImpl, const OFFSET: isize>() -> ITypeMarshalVtbl {
        unsafe extern "system" fn Size<Impl: ITypeMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, psize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size(&*(&pvtype as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwdestcontext, &*(&pvdestcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&psize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Marshal<Impl: ITypeMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Marshal(&*(&pvtype as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwdestcontext, &*(&pvdestcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbbufferlength, ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbwritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmarshal<Impl: ITypeMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *mut ::core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unmarshal(::core::mem::transmute_copy(&pvtype), dwflags, cbbufferlength, pbuffer, ::core::mem::transmute_copy(&pcbread)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Free<Impl: ITypeMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Free(&*(&pvtype as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITypeMarshal>, ::windows::core::GetTrustLevel, Size::<Impl, OFFSET>, Marshal::<Impl, OFFSET>, Unmarshal::<Impl, OFFSET>, Free::<Impl, OFFSET>)
    }
}
pub trait IVBFormatImpl: Sized {
    fn Format();
}
impl ::windows::core::RuntimeName for IVBFormat {
    const NAME: &'static str = "Windows.Win32.System.Ole.IVBFormat";
}
impl IVBFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBFormatImpl, const OFFSET: isize>() -> IVBFormatVtbl {
        unsafe extern "system" fn Format<Impl: IVBFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdata: *mut super::Com::VARIANT, bstrformat: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format(
                &*(&vdata as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrformat as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cb,
                lcid,
                sfirstdayofweek,
                sfirstweekofyear,
                rcb,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVBFormat>, ::windows::core::GetTrustLevel, Format::<Impl, OFFSET>)
    }
}
pub trait IVBGetControlImpl: Sized {
    fn EnumControls();
}
impl ::windows::core::RuntimeName for IVBGetControl {
    const NAME: &'static str = "Windows.Win32.System.Ole.IVBGetControl";
}
impl IVBGetControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBGetControlImpl, const OFFSET: isize>() -> IVBGetControlVtbl {
        unsafe extern "system" fn EnumControls<Impl: IVBGetControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwolecontf: OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS, ppenumunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumControls(dwolecontf, dwwhich, ::core::mem::transmute_copy(&ppenumunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVBGetControl>, ::windows::core::GetTrustLevel, EnumControls::<Impl, OFFSET>)
    }
}
pub trait IVariantChangeTypeImpl: Sized {
    fn ChangeType();
}
impl ::windows::core::RuntimeName for IVariantChangeType {
    const NAME: &'static str = "Windows.Win32.System.Ole.IVariantChangeType";
}
impl IVariantChangeTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVariantChangeTypeImpl, const OFFSET: isize>() -> IVariantChangeTypeVtbl {
        unsafe extern "system" fn ChangeType<Impl: IVariantChangeTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardst: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, vtnew: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeType(&*(&pvardst as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&pvarsrc as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), lcid, vtnew) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVariantChangeType>, ::windows::core::GetTrustLevel, ChangeType::<Impl, OFFSET>)
    }
}
pub trait IViewObjectImpl: Sized {
    fn Draw();
    fn GetColorSet();
    fn Freeze();
    fn Unfreeze();
    fn SetAdvise();
    fn GetAdvise();
}
impl ::windows::core::RuntimeName for IViewObject {
    const NAME: &'static str = "Windows.Win32.System.Ole.IViewObject";
}
impl IViewObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObjectImpl, const OFFSET: isize>() -> IViewObjectVtbl {
        unsafe extern "system" fn Draw<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Draw(
                dwdrawaspect,
                lindex,
                &*(&pvaspect as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&ptd as *const <super::Com::DVTARGETDEVICE as ::windows::core::Abi>::Abi as *const <super::Com::DVTARGETDEVICE as ::windows::core::DefaultType>::DefaultType),
                &*(&hdctargetdev as *const <super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&hdcdraw as *const <super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&lprcbounds as *const <super::super::Foundation::RECTL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECTL as ::windows::core::DefaultType>::DefaultType),
                &*(&lprcwbounds as *const <super::super::Foundation::RECTL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECTL as ::windows::core::DefaultType>::DefaultType),
                pfncontinue,
                dwcontinue,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorSet<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorSet(
                dwdrawaspect,
                lindex,
                &*(&pvaspect as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&ptd as *const <super::Com::DVTARGETDEVICE as ::windows::core::Abi>::Abi as *const <super::Com::DVTARGETDEVICE as ::windows::core::DefaultType>::DefaultType),
                &*(&hictargetdev as *const <super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppcolorset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Freeze<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Freeze(dwdrawaspect, lindex, &*(&pvaspect as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwfreeze)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unfreeze<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfreeze: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unfreeze(dwfreeze) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvise<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aspects: u32, advf: u32, padvsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAdvise(aspects, advf, &*(&padvsink as *const <super::Com::IAdviseSink as ::windows::core::Abi>::Abi as *const <super::Com::IAdviseSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdvise<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdvise(::core::mem::transmute_copy(&paspects), ::core::mem::transmute_copy(&padvf), ::core::mem::transmute_copy(&ppadvsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IViewObject>, ::windows::core::GetTrustLevel, Draw::<Impl, OFFSET>, GetColorSet::<Impl, OFFSET>, Freeze::<Impl, OFFSET>, Unfreeze::<Impl, OFFSET>, SetAdvise::<Impl, OFFSET>, GetAdvise::<Impl, OFFSET>)
    }
}
pub trait IViewObject2Impl: Sized + IViewObjectImpl {
    fn GetExtent();
}
impl ::windows::core::RuntimeName for IViewObject2 {
    const NAME: &'static str = "Windows.Win32.System.Ole.IViewObject2";
}
impl IViewObject2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject2Impl, const OFFSET: isize>() -> IViewObject2Vtbl {
        unsafe extern "system" fn GetExtent<Impl: IViewObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, lpsizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtent(dwdrawaspect, lindex, &*(&ptd as *const <super::Com::DVTARGETDEVICE as ::windows::core::Abi>::Abi as *const <super::Com::DVTARGETDEVICE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&lpsizel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IViewObject2>, ::windows::core::GetTrustLevel, GetExtent::<Impl, OFFSET>)
    }
}
pub trait IViewObjectExImpl: Sized + IViewObject2Impl + IViewObjectImpl {
    fn GetRect();
    fn GetViewStatus();
    fn QueryHitPoint();
    fn QueryHitRect();
    fn GetNaturalExtent();
}
impl ::windows::core::RuntimeName for IViewObjectEx {
    const NAME: &'static str = "Windows.Win32.System.Ole.IViewObjectEx";
}
impl IViewObjectExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObjectExImpl, const OFFSET: isize>() -> IViewObjectExVtbl {
        unsafe extern "system" fn GetRect<Impl: IViewObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prect: *mut super::super::Foundation::RECTL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRect(dwaspect, ::core::mem::transmute_copy(&prect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewStatus<Impl: IViewObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHitPoint<Impl: IViewObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHitPoint(dwaspect, &*(&prectbounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&ptlloc as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), lclosehint, ::core::mem::transmute_copy(&phitresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHitRect<Impl: IViewObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHitRect(dwaspect, &*(&prectbounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&prectloc as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), lclosehint, ::core::mem::transmute_copy(&phitresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNaturalExtent<Impl: IViewObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const ExtentInfo, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNaturalExtent(
                dwaspect,
                lindex,
                &*(&ptd as *const <super::Com::DVTARGETDEVICE as ::windows::core::Abi>::Abi as *const <super::Com::DVTARGETDEVICE as ::windows::core::DefaultType>::DefaultType),
                &*(&hictargetdev as *const <super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&pextentinfo as *const <ExtentInfo as ::windows::core::Abi>::Abi as *const <ExtentInfo as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&psizel),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IViewObjectEx>, ::windows::core::GetTrustLevel, GetRect::<Impl, OFFSET>, GetViewStatus::<Impl, OFFSET>, QueryHitPoint::<Impl, OFFSET>, QueryHitRect::<Impl, OFFSET>, GetNaturalExtent::<Impl, OFFSET>)
    }
}
pub trait IZoomEventsImpl: Sized {
    fn OnZoomPercentChanged();
}
impl ::windows::core::RuntimeName for IZoomEvents {
    const NAME: &'static str = "Windows.Win32.System.Ole.IZoomEvents";
}
impl IZoomEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoomEventsImpl, const OFFSET: isize>() -> IZoomEventsVtbl {
        unsafe extern "system" fn OnZoomPercentChanged<Impl: IZoomEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulzoompercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnZoomPercentChanged(ulzoompercent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IZoomEvents>, ::windows::core::GetTrustLevel, OnZoomPercentChanged::<Impl, OFFSET>)
    }
}
