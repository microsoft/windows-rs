#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSinkExImpl: Sized + IAdviseSinkImpl {
    fn OnViewStatusChange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSinkExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdviseSinkExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdviseSinkExVtbl {
        unsafe extern "system" fn OnViewStatusChange<Impl: IAdviseSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwviewstatus: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnDataChange::<Impl, IMPL_OFFSET>, OnViewChange::<Impl, IMPL_OFFSET>, OnRename::<Impl, IMPL_OFFSET>, OnSave::<Impl, IMPL_OFFSET>, OnClose::<Impl, IMPL_OFFSET>, OnViewStatusChange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdviseSinkEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICanHandleExceptionImpl: Sized {
    fn CanHandleException();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICanHandleExceptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICanHandleExceptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICanHandleExceptionVtbl {
        unsafe extern "system" fn CanHandleException<Impl: ICanHandleExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CanHandleException::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanHandleException as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IClassFactory2Impl: Sized + IClassFactoryImpl {
    fn GetLicInfo();
    fn RequestLicKey();
    fn CreateInstanceLic();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IClassFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClassFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClassFactory2Vtbl {
        unsafe extern "system" fn GetLicInfo<Impl: IClassFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plicinfo: *mut LICINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestLicKey<Impl: IClassFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, pbstrkey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstanceLic<Impl: IClassFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, bstrkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateInstance::<Impl, IMPL_OFFSET>, LockServer::<Impl, IMPL_OFFSET>, GetLicInfo::<Impl, IMPL_OFFSET>, RequestLicKey::<Impl, IMPL_OFFSET>, CreateInstanceLic::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClassFactory2 as ::windows::core::Interface>::IID
    }
}
pub trait IContinueImpl: Sized {
    fn FContinue();
}
impl IContinueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContinueVtbl {
        unsafe extern "system" fn FContinue<Impl: IContinueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FContinue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContinue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContinueCallbackImpl: Sized {
    fn FContinue();
    fn FContinuePrinting();
}
#[cfg(feature = "Win32_Foundation")]
impl IContinueCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinueCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContinueCallbackVtbl {
        unsafe extern "system" fn FContinue<Impl: IContinueCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FContinuePrinting<Impl: IContinueCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncntprinted: i32, ncurpage: i32, pwszprintstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FContinue::<Impl, IMPL_OFFSET>, FContinuePrinting::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContinueCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICreateErrorInfoImpl: Sized {
    fn SetGUID();
    fn SetSource();
    fn SetDescription();
    fn SetHelpFile();
    fn SetHelpContext();
}
#[cfg(feature = "Win32_Foundation")]
impl ICreateErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateErrorInfoVtbl {
        unsafe extern "system" fn SetGUID<Impl: ICreateErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSource<Impl: ICreateErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsource: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: ICreateErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHelpFile<Impl: ICreateErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhelpfile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHelpContext<Impl: ICreateErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetGUID::<Impl, IMPL_OFFSET>, SetSource::<Impl, IMPL_OFFSET>, SetDescription::<Impl, IMPL_OFFSET>, SetHelpFile::<Impl, IMPL_OFFSET>, SetHelpContext::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICreateTypeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateTypeInfoVtbl {
        unsafe extern "system" fn SetGuid<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTypeFlags<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utypeflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDocString<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdoc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHelpContext<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVersion<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRefTypeInfo<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptinfo: ::windows::core::RawPtr, phreftype: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFuncDesc<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddImplType<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, hreftype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetImplTypeFlags<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, impltypeflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlignment<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbalignment: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSchema<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrschema: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddVarDesc<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFuncAndParamNames<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVarName<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTypeDescAlias<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptdescalias: *const super::Com::TYPEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefineFuncAsDllEntry<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdllname: super::super::Foundation::PWSTR, szprocname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFuncDocString<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVarDocString<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFuncHelpContext<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVarHelpContext<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMops<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, bstrmops: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTypeIdldesc<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidldesc: *const super::Com::IDLDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LayOut<Impl: ICreateTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetGuid::<Impl, IMPL_OFFSET>,
            SetTypeFlags::<Impl, IMPL_OFFSET>,
            SetDocString::<Impl, IMPL_OFFSET>,
            SetHelpContext::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            AddRefTypeInfo::<Impl, IMPL_OFFSET>,
            AddFuncDesc::<Impl, IMPL_OFFSET>,
            AddImplType::<Impl, IMPL_OFFSET>,
            SetImplTypeFlags::<Impl, IMPL_OFFSET>,
            SetAlignment::<Impl, IMPL_OFFSET>,
            SetSchema::<Impl, IMPL_OFFSET>,
            AddVarDesc::<Impl, IMPL_OFFSET>,
            SetFuncAndParamNames::<Impl, IMPL_OFFSET>,
            SetVarName::<Impl, IMPL_OFFSET>,
            SetTypeDescAlias::<Impl, IMPL_OFFSET>,
            DefineFuncAsDllEntry::<Impl, IMPL_OFFSET>,
            SetFuncDocString::<Impl, IMPL_OFFSET>,
            SetVarDocString::<Impl, IMPL_OFFSET>,
            SetFuncHelpContext::<Impl, IMPL_OFFSET>,
            SetVarHelpContext::<Impl, IMPL_OFFSET>,
            SetMops::<Impl, IMPL_OFFSET>,
            SetTypeIdldesc::<Impl, IMPL_OFFSET>,
            LayOut::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateTypeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICreateTypeInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateTypeInfo2Vtbl {
        unsafe extern "system" fn DeleteFuncDesc<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteFuncDescByMemId<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteVarDesc<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteVarDescByMemId<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteImplType<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCustData<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFuncCustData<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParamCustData<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVarCustData<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetImplTypeCustData<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHelpStringContext<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFuncHelpStringContext<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVarHelpStringContext<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Invalidate<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: ICreateTypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetGuid::<Impl, IMPL_OFFSET>,
            SetTypeFlags::<Impl, IMPL_OFFSET>,
            SetDocString::<Impl, IMPL_OFFSET>,
            SetHelpContext::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            AddRefTypeInfo::<Impl, IMPL_OFFSET>,
            AddFuncDesc::<Impl, IMPL_OFFSET>,
            AddImplType::<Impl, IMPL_OFFSET>,
            SetImplTypeFlags::<Impl, IMPL_OFFSET>,
            SetAlignment::<Impl, IMPL_OFFSET>,
            SetSchema::<Impl, IMPL_OFFSET>,
            AddVarDesc::<Impl, IMPL_OFFSET>,
            SetFuncAndParamNames::<Impl, IMPL_OFFSET>,
            SetVarName::<Impl, IMPL_OFFSET>,
            SetTypeDescAlias::<Impl, IMPL_OFFSET>,
            DefineFuncAsDllEntry::<Impl, IMPL_OFFSET>,
            SetFuncDocString::<Impl, IMPL_OFFSET>,
            SetVarDocString::<Impl, IMPL_OFFSET>,
            SetFuncHelpContext::<Impl, IMPL_OFFSET>,
            SetVarHelpContext::<Impl, IMPL_OFFSET>,
            SetMops::<Impl, IMPL_OFFSET>,
            SetTypeIdldesc::<Impl, IMPL_OFFSET>,
            LayOut::<Impl, IMPL_OFFSET>,
            DeleteFuncDesc::<Impl, IMPL_OFFSET>,
            DeleteFuncDescByMemId::<Impl, IMPL_OFFSET>,
            DeleteVarDesc::<Impl, IMPL_OFFSET>,
            DeleteVarDescByMemId::<Impl, IMPL_OFFSET>,
            DeleteImplType::<Impl, IMPL_OFFSET>,
            SetCustData::<Impl, IMPL_OFFSET>,
            SetFuncCustData::<Impl, IMPL_OFFSET>,
            SetParamCustData::<Impl, IMPL_OFFSET>,
            SetVarCustData::<Impl, IMPL_OFFSET>,
            SetImplTypeCustData::<Impl, IMPL_OFFSET>,
            SetHelpStringContext::<Impl, IMPL_OFFSET>,
            SetFuncHelpStringContext::<Impl, IMPL_OFFSET>,
            SetVarHelpStringContext::<Impl, IMPL_OFFSET>,
            Invalidate::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateTypeInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICreateTypeLibVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLibImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateTypeLibVtbl {
        unsafe extern "system" fn CreateTypeInfo<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, tkind: super::Com::TYPEKIND, ppctinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVersion<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGuid<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDocString<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdoc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHelpFileName<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhelpfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHelpContext<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLcid<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLibFlags<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulibflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveAllChanges<Impl: ICreateTypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateTypeInfo::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            SetGuid::<Impl, IMPL_OFFSET>,
            SetDocString::<Impl, IMPL_OFFSET>,
            SetHelpFileName::<Impl, IMPL_OFFSET>,
            SetHelpContext::<Impl, IMPL_OFFSET>,
            SetLcid::<Impl, IMPL_OFFSET>,
            SetLibFlags::<Impl, IMPL_OFFSET>,
            SaveAllChanges::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateTypeLib as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICreateTypeLib2Impl: Sized + ICreateTypeLibImpl {
    fn DeleteTypeInfo();
    fn SetCustData();
    fn SetHelpStringContext();
    fn SetHelpStringDll();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICreateTypeLib2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateTypeLib2Vtbl {
        unsafe extern "system" fn DeleteTypeInfo<Impl: ICreateTypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCustData<Impl: ICreateTypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHelpStringContext<Impl: ICreateTypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHelpStringDll<Impl: ICreateTypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateTypeInfo::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            SetGuid::<Impl, IMPL_OFFSET>,
            SetDocString::<Impl, IMPL_OFFSET>,
            SetHelpFileName::<Impl, IMPL_OFFSET>,
            SetHelpContext::<Impl, IMPL_OFFSET>,
            SetLcid::<Impl, IMPL_OFFSET>,
            SetLibFlags::<Impl, IMPL_OFFSET>,
            SaveAllChanges::<Impl, IMPL_OFFSET>,
            DeleteTypeInfo::<Impl, IMPL_OFFSET>,
            SetCustData::<Impl, IMPL_OFFSET>,
            SetHelpStringContext::<Impl, IMPL_OFFSET>,
            SetHelpStringDll::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateTypeLib2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDispErrorImpl: Sized {
    fn QueryErrorInfo();
    fn GetNext();
    fn GetHresult();
    fn GetSource();
    fn GetHelpInfo();
    fn GetDescription();
}
#[cfg(feature = "Win32_Foundation")]
impl IDispErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispErrorVtbl {
        unsafe extern "system" fn QueryErrorInfo<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guiderrortype: ::windows::core::GUID, ppde: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNext<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppde: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHresult<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSource<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHelpInfo<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR, pdwcontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: IDispErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryErrorInfo::<Impl, IMPL_OFFSET>, GetNext::<Impl, IMPL_OFFSET>, GetHresult::<Impl, IMPL_OFFSET>, GetSource::<Impl, IMPL_OFFSET>, GetHelpInfo::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDispatchExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatchExVtbl {
        unsafe extern "system" fn GetDispID<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grfdex: u32, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvokeEx<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut super::Com::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteMemberByName<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grfdex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteMemberByDispID<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberProperties<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, grfdexfetch: u32, pgrfdex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberName<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextDispID<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfdex: u32, id: i32, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNameSpaceParent<Impl: IDispatchExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetDispID::<Impl, IMPL_OFFSET>,
            InvokeEx::<Impl, IMPL_OFFSET>,
            DeleteMemberByName::<Impl, IMPL_OFFSET>,
            DeleteMemberByDispID::<Impl, IMPL_OFFSET>,
            GetMemberProperties::<Impl, IMPL_OFFSET>,
            GetMemberName::<Impl, IMPL_OFFSET>,
            GetNextDispID::<Impl, IMPL_OFFSET>,
            GetNameSpaceParent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatchEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDropSourceImpl: Sized {
    fn QueryContinueDrag();
    fn GiveFeedback();
}
#[cfg(feature = "Win32_Foundation")]
impl IDropSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropSourceVtbl {
        unsafe extern "system" fn QueryContinueDrag<Impl: IDropSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fescapepressed: super::super::Foundation::BOOL, grfkeystate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GiveFeedback<Impl: IDropSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweffect: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryContinueDrag::<Impl, IMPL_OFFSET>, GiveFeedback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDropSourceNotifyImpl: Sized {
    fn DragEnterTarget();
    fn DragLeaveTarget();
}
#[cfg(feature = "Win32_Foundation")]
impl IDropSourceNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropSourceNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropSourceNotifyVtbl {
        unsafe extern "system" fn DragEnterTarget<Impl: IDropSourceNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndtarget: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DragLeaveTarget<Impl: IDropSourceNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DragEnterTarget::<Impl, IMPL_OFFSET>, DragLeaveTarget::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropSourceNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDropTargetImpl: Sized {
    fn DragEnter();
    fn DragOver();
    fn DragLeave();
    fn Drop();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDropTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetVtbl {
        unsafe extern "system" fn DragEnter<Impl: IDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobj: ::windows::core::RawPtr, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DragOver<Impl: IDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DragLeave<Impl: IDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Drop<Impl: IDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobj: ::windows::core::RawPtr, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DragEnter::<Impl, IMPL_OFFSET>, DragOver::<Impl, IMPL_OFFSET>, DragLeave::<Impl, IMPL_OFFSET>, Drop::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnterpriseDropTargetImpl: Sized {
    fn SetDropSourceEnterpriseId();
    fn IsEvaluatingEdpPolicy();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnterpriseDropTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseDropTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnterpriseDropTargetVtbl {
        unsafe extern "system" fn SetDropSourceEnterpriseId<Impl: IEnterpriseDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEvaluatingEdpPolicy<Impl: IEnterpriseDropTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetDropSourceEnterpriseId::<Impl, IMPL_OFFSET>, IsEvaluatingEdpPolicy::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnterpriseDropTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumOLEVERBImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumOLEVERBVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOLEVERBImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumOLEVERBVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOLEVERBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumOLEVERBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumOLEVERBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumOLEVERBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumOLEVERB as ::windows::core::Interface>::IID
    }
}
pub trait IEnumOleDocumentViewsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumOleDocumentViewsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleDocumentViewsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumOleDocumentViewsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOleDocumentViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cviews: u32, rgpview: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumOleDocumentViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cviews: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumOleDocumentViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumOleDocumentViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumOleDocumentViews as ::windows::core::Interface>::IID
    }
}
pub trait IEnumOleUndoUnitsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumOleUndoUnitsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleUndoUnitsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumOleUndoUnitsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOleUndoUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumOleUndoUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumOleUndoUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumOleUndoUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumOleUndoUnits as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IEnumVARIANTImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IEnumVARIANTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVARIANTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumVARIANTVtbl {
        unsafe extern "system" fn Next<Impl: IEnumVARIANTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumVARIANTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumVARIANTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumVARIANTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumVARIANT as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IFontVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFontVtbl {
        unsafe extern "system" fn Name<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Size<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSize<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Bold<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbold: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBold<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bold: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Italic<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitalic: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetItalic<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, italic: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Underline<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punderline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnderline<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, underline: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Strikethrough<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrikethrough: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrikethrough<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strikethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Weight<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweight: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWeight<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Charset<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcharset: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCharset<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, charset: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn hFont<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phfont: *mut super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfontother: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRatio<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cylogical: i32, cyhimetric: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryTextMetrics<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRefHfont<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseHfont<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHdc<Impl: IFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Size::<Impl, IMPL_OFFSET>,
            SetSize::<Impl, IMPL_OFFSET>,
            Bold::<Impl, IMPL_OFFSET>,
            SetBold::<Impl, IMPL_OFFSET>,
            Italic::<Impl, IMPL_OFFSET>,
            SetItalic::<Impl, IMPL_OFFSET>,
            Underline::<Impl, IMPL_OFFSET>,
            SetUnderline::<Impl, IMPL_OFFSET>,
            Strikethrough::<Impl, IMPL_OFFSET>,
            SetStrikethrough::<Impl, IMPL_OFFSET>,
            Weight::<Impl, IMPL_OFFSET>,
            SetWeight::<Impl, IMPL_OFFSET>,
            Charset::<Impl, IMPL_OFFSET>,
            SetCharset::<Impl, IMPL_OFFSET>,
            hFont::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
            SetRatio::<Impl, IMPL_OFFSET>,
            QueryTextMetrics::<Impl, IMPL_OFFSET>,
            AddRefHfont::<Impl, IMPL_OFFSET>,
            ReleaseHfont::<Impl, IMPL_OFFSET>,
            SetHdc::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFont as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFontDispImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IFontDispVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontDispImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFontDispVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFontDisp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFontEventsDispImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IFontEventsDispVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontEventsDispImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFontEventsDispVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFontEventsDisp as ::windows::core::Interface>::IID
    }
}
pub trait IGetOleObjectImpl: Sized {
    fn GetOleObject();
}
impl IGetOleObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetOleObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetOleObjectVtbl {
        unsafe extern "system" fn GetOleObject<Impl: IGetOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOleObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetOleObject as ::windows::core::Interface>::IID
    }
}
pub trait IGetVBAObjectImpl: Sized {
    fn GetObject();
}
impl IGetVBAObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetVBAObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetVBAObjectVtbl {
        unsafe extern "system" fn GetObject<Impl: IGetVBAObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetVBAObject as ::windows::core::Interface>::IID
    }
}
pub trait IObjectIdentityImpl: Sized {
    fn IsEqualObject();
}
impl IObjectIdentityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectIdentityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectIdentityVtbl {
        unsafe extern "system" fn IsEqualObject<Impl: IObjectIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsEqualObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectIdentity as ::windows::core::Interface>::IID
    }
}
pub trait IObjectWithSiteImpl: Sized {
    fn SetSite();
    fn GetSite();
}
impl IObjectWithSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectWithSiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectWithSiteVtbl {
        unsafe extern "system" fn SetSite<Impl: IObjectWithSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punksite: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSite<Impl: IObjectWithSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvsite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetSite::<Impl, IMPL_OFFSET>, GetSite::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectWithSite as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleAdviseHolderImpl: Sized {
    fn Advise();
    fn Unadvise();
    fn EnumAdvise();
    fn SendOnRename();
    fn SendOnSave();
    fn SendOnClose();
}
#[cfg(feature = "Win32_System_Com")]
impl IOleAdviseHolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleAdviseHolderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleAdviseHolderVtbl {
        unsafe extern "system" fn Advise<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvise: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unadvise<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAdvise<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendOnRename<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendOnSave<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendOnClose<Impl: IOleAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Advise::<Impl, IMPL_OFFSET>, Unadvise::<Impl, IMPL_OFFSET>, EnumAdvise::<Impl, IMPL_OFFSET>, SendOnRename::<Impl, IMPL_OFFSET>, SendOnSave::<Impl, IMPL_OFFSET>, SendOnClose::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleAdviseHolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IOleCacheImpl: Sized {
    fn Cache();
    fn Uncache();
    fn EnumCache();
    fn InitCache();
    fn SetData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IOleCacheVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCacheImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleCacheVtbl {
        unsafe extern "system" fn Cache<Impl: IOleCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, advf: u32, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Uncache<Impl: IOleCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumCache<Impl: IOleCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstatdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitCache<Impl: IOleCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetData<Impl: IOleCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Cache::<Impl, IMPL_OFFSET>, Uncache::<Impl, IMPL_OFFSET>, EnumCache::<Impl, IMPL_OFFSET>, InitCache::<Impl, IMPL_OFFSET>, SetData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleCache as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IOleCache2Impl: Sized + IOleCacheImpl {
    fn UpdateCache();
    fn DiscardCache();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IOleCache2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleCache2Vtbl {
        unsafe extern "system" fn UpdateCache<Impl: IOleCache2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, grfupdf: UPDFCACHE_FLAGS, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscardCache<Impl: IOleCache2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdiscardoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Cache::<Impl, IMPL_OFFSET>, Uncache::<Impl, IMPL_OFFSET>, EnumCache::<Impl, IMPL_OFFSET>, InitCache::<Impl, IMPL_OFFSET>, SetData::<Impl, IMPL_OFFSET>, UpdateCache::<Impl, IMPL_OFFSET>, DiscardCache::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleCache2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleCacheControlImpl: Sized {
    fn OnRun();
    fn OnStop();
}
#[cfg(feature = "Win32_System_Com")]
impl IOleCacheControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCacheControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleCacheControlVtbl {
        unsafe extern "system" fn OnRun<Impl: IOleCacheControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnStop<Impl: IOleCacheControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnRun::<Impl, IMPL_OFFSET>, OnStop::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleCacheControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleClientSiteImpl: Sized {
    fn SaveObject();
    fn GetMoniker();
    fn GetContainer();
    fn ShowObject();
    fn OnShowWindow();
    fn RequestNewObjectLayout();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleClientSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleClientSiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleClientSiteVtbl {
        unsafe extern "system" fn SaveObject<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMoniker<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContainer<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowObject<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnShowWindow<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestNewObjectLayout<Impl: IOleClientSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SaveObject::<Impl, IMPL_OFFSET>, GetMoniker::<Impl, IMPL_OFFSET>, GetContainer::<Impl, IMPL_OFFSET>, ShowObject::<Impl, IMPL_OFFSET>, OnShowWindow::<Impl, IMPL_OFFSET>, RequestNewObjectLayout::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleClientSite as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleCommandTargetImpl: Sized {
    fn QueryStatus();
    fn Exec();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleCommandTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCommandTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleCommandTargetVtbl {
        unsafe extern "system" fn QueryStatus<Impl: IOleCommandTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows::core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Exec<Impl: IOleCommandTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows::core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Com::VARIANT, pvaout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryStatus::<Impl, IMPL_OFFSET>, Exec::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleCommandTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleContainerImpl: Sized + IParseDisplayNameImpl {
    fn EnumObjects();
    fn LockContainer();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleContainerVtbl {
        unsafe extern "system" fn EnumObjects<Impl: IOleContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockContainer<Impl: IOleContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ParseDisplayName::<Impl, IMPL_OFFSET>, EnumObjects::<Impl, IMPL_OFFSET>, LockContainer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleControlImpl: Sized {
    fn GetControlInfo();
    fn OnMnemonic();
    fn OnAmbientPropertyChange();
    fn FreezeEvents();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleControlVtbl {
        unsafe extern "system" fn GetControlInfo<Impl: IOleControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut CONTROLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnMnemonic<Impl: IOleControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnAmbientPropertyChange<Impl: IOleControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreezeEvents<Impl: IOleControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfreeze: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetControlInfo::<Impl, IMPL_OFFSET>, OnMnemonic::<Impl, IMPL_OFFSET>, OnAmbientPropertyChange::<Impl, IMPL_OFFSET>, FreezeEvents::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleControlSiteImpl: Sized {
    fn OnControlInfoChanged();
    fn LockInPlaceActive();
    fn GetExtendedControl();
    fn TransformCoords();
    fn TranslateAccelerator();
    fn OnFocus();
    fn ShowPropertyFrame();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleControlSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlSiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleControlSiteVtbl {
        unsafe extern "system" fn OnControlInfoChanged<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockInPlaceActive<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtendedControl<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransformCoords<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: XFORMCOORDS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnFocus<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgotfocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowPropertyFrame<Impl: IOleControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnControlInfoChanged::<Impl, IMPL_OFFSET>, LockInPlaceActive::<Impl, IMPL_OFFSET>, GetExtendedControl::<Impl, IMPL_OFFSET>, TransformCoords::<Impl, IMPL_OFFSET>, TranslateAccelerator::<Impl, IMPL_OFFSET>, OnFocus::<Impl, IMPL_OFFSET>, ShowPropertyFrame::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleControlSite as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleDocumentImpl: Sized {
    fn CreateView();
    fn GetDocMiscStatus();
    fn EnumViews();
}
#[cfg(feature = "Win32_System_Com")]
impl IOleDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleDocumentVtbl {
        unsafe extern "system" fn CreateView<Impl: IOleDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsite: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, dwreserved: u32, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocMiscStatus<Impl: IOleDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumViews<Impl: IOleDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateView::<Impl, IMPL_OFFSET>, GetDocMiscStatus::<Impl, IMPL_OFFSET>, EnumViews::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleDocument as ::windows::core::Interface>::IID
    }
}
pub trait IOleDocumentSiteImpl: Sized {
    fn ActivateMe();
}
impl IOleDocumentSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentSiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleDocumentSiteVtbl {
        unsafe extern "system" fn ActivateMe<Impl: IOleDocumentSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewtoactivate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ActivateMe::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleDocumentSite as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleDocumentViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleDocumentViewVtbl {
        unsafe extern "system" fn SetInPlaceSite<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInPlaceSite<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocument<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRect<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRect<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRectComplex<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Show<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UIActivate<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuiactivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Open<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseView<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveViewState<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplyViewState<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IOleDocumentViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsitenew: ::windows::core::RawPtr, ppviewnew: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetInPlaceSite::<Impl, IMPL_OFFSET>,
            GetInPlaceSite::<Impl, IMPL_OFFSET>,
            GetDocument::<Impl, IMPL_OFFSET>,
            SetRect::<Impl, IMPL_OFFSET>,
            GetRect::<Impl, IMPL_OFFSET>,
            SetRectComplex::<Impl, IMPL_OFFSET>,
            Show::<Impl, IMPL_OFFSET>,
            UIActivate::<Impl, IMPL_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            CloseView::<Impl, IMPL_OFFSET>,
            SaveViewState::<Impl, IMPL_OFFSET>,
            ApplyViewState::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleDocumentView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceActiveObjectImpl: Sized + IOleWindowImpl {
    fn TranslateAccelerator();
    fn OnFrameWindowActivate();
    fn OnDocWindowActivate();
    fn ResizeBorder();
    fn EnableModeless();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceActiveObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceActiveObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceActiveObjectVtbl {
        unsafe extern "system" fn TranslateAccelerator<Impl: IOleInPlaceActiveObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnFrameWindowActivate<Impl: IOleInPlaceActiveObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDocWindowActivate<Impl: IOleInPlaceActiveObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResizeBorder<Impl: IOleInPlaceActiveObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcborder: *const super::super::Foundation::RECT, puiwindow: ::windows::core::RawPtr, fframewindow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableModeless<Impl: IOleInPlaceActiveObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWindow::<Impl, IMPL_OFFSET>, ContextSensitiveHelp::<Impl, IMPL_OFFSET>, TranslateAccelerator::<Impl, IMPL_OFFSET>, OnFrameWindowActivate::<Impl, IMPL_OFFSET>, OnDocWindowActivate::<Impl, IMPL_OFFSET>, ResizeBorder::<Impl, IMPL_OFFSET>, EnableModeless::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceActiveObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceFrameImpl: Sized + IOleInPlaceUIWindowImpl + IOleWindowImpl {
    fn InsertMenus();
    fn SetMenu();
    fn RemoveMenus();
    fn SetStatusText();
    fn EnableModeless();
    fn TranslateAccelerator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceFrameVtbl {
        unsafe extern "system" fn InsertMenus<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMenu<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveMenus<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatusText<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableModeless<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IOleInPlaceFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetWindow::<Impl, IMPL_OFFSET>,
            ContextSensitiveHelp::<Impl, IMPL_OFFSET>,
            GetBorder::<Impl, IMPL_OFFSET>,
            RequestBorderSpace::<Impl, IMPL_OFFSET>,
            SetBorderSpace::<Impl, IMPL_OFFSET>,
            SetActiveObject::<Impl, IMPL_OFFSET>,
            InsertMenus::<Impl, IMPL_OFFSET>,
            SetMenu::<Impl, IMPL_OFFSET>,
            RemoveMenus::<Impl, IMPL_OFFSET>,
            SetStatusText::<Impl, IMPL_OFFSET>,
            EnableModeless::<Impl, IMPL_OFFSET>,
            TranslateAccelerator::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceObjectImpl: Sized + IOleWindowImpl {
    fn InPlaceDeactivate();
    fn UIDeactivate();
    fn SetObjectRects();
    fn ReactivateAndUndo();
}
#[cfg(feature = "Win32_Foundation")]
impl IOleInPlaceObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceObjectVtbl {
        unsafe extern "system" fn InPlaceDeactivate<Impl: IOleInPlaceObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UIDeactivate<Impl: IOleInPlaceObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetObjectRects<Impl: IOleInPlaceObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReactivateAndUndo<Impl: IOleInPlaceObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWindow::<Impl, IMPL_OFFSET>, ContextSensitiveHelp::<Impl, IMPL_OFFSET>, InPlaceDeactivate::<Impl, IMPL_OFFSET>, UIDeactivate::<Impl, IMPL_OFFSET>, SetObjectRects::<Impl, IMPL_OFFSET>, ReactivateAndUndo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceObjectWindowlessImpl: Sized + IOleInPlaceObjectImpl + IOleWindowImpl {
    fn OnWindowMessage();
    fn GetDropTarget();
}
#[cfg(feature = "Win32_Foundation")]
impl IOleInPlaceObjectWindowlessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObjectWindowlessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceObjectWindowlessVtbl {
        unsafe extern "system" fn OnWindowMessage<Impl: IOleInPlaceObjectWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDropTarget<Impl: IOleInPlaceObjectWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdroptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWindow::<Impl, IMPL_OFFSET>, ContextSensitiveHelp::<Impl, IMPL_OFFSET>, InPlaceDeactivate::<Impl, IMPL_OFFSET>, UIDeactivate::<Impl, IMPL_OFFSET>, SetObjectRects::<Impl, IMPL_OFFSET>, ReactivateAndUndo::<Impl, IMPL_OFFSET>, OnWindowMessage::<Impl, IMPL_OFFSET>, GetDropTarget::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceObjectWindowless as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceSiteVtbl {
        unsafe extern "system" fn CanInPlaceActivate<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInPlaceActivate<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnUIActivate<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWindowContext<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppframe: *mut ::windows::core::RawPtr, ppdoc: *mut ::windows::core::RawPtr, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Scroll<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollextant: super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnUIDeactivate<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fundoable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInPlaceDeactivate<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscardUndoState<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeactivateAndUndo<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnPosRectChange<Impl: IOleInPlaceSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetWindow::<Impl, IMPL_OFFSET>,
            ContextSensitiveHelp::<Impl, IMPL_OFFSET>,
            CanInPlaceActivate::<Impl, IMPL_OFFSET>,
            OnInPlaceActivate::<Impl, IMPL_OFFSET>,
            OnUIActivate::<Impl, IMPL_OFFSET>,
            GetWindowContext::<Impl, IMPL_OFFSET>,
            Scroll::<Impl, IMPL_OFFSET>,
            OnUIDeactivate::<Impl, IMPL_OFFSET>,
            OnInPlaceDeactivate::<Impl, IMPL_OFFSET>,
            DiscardUndoState::<Impl, IMPL_OFFSET>,
            DeactivateAndUndo::<Impl, IMPL_OFFSET>,
            OnPosRectChange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceSite as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceSiteExImpl: Sized + IOleInPlaceSiteImpl + IOleWindowImpl {
    fn OnInPlaceActivateEx();
    fn OnInPlaceDeactivateEx();
    fn RequestUIActivate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceSiteExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceSiteExVtbl {
        unsafe extern "system" fn OnInPlaceActivateEx<Impl: IOleInPlaceSiteExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInPlaceDeactivateEx<Impl: IOleInPlaceSiteExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnoredraw: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestUIActivate<Impl: IOleInPlaceSiteExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetWindow::<Impl, IMPL_OFFSET>,
            ContextSensitiveHelp::<Impl, IMPL_OFFSET>,
            CanInPlaceActivate::<Impl, IMPL_OFFSET>,
            OnInPlaceActivate::<Impl, IMPL_OFFSET>,
            OnUIActivate::<Impl, IMPL_OFFSET>,
            GetWindowContext::<Impl, IMPL_OFFSET>,
            Scroll::<Impl, IMPL_OFFSET>,
            OnUIDeactivate::<Impl, IMPL_OFFSET>,
            OnInPlaceDeactivate::<Impl, IMPL_OFFSET>,
            DiscardUndoState::<Impl, IMPL_OFFSET>,
            DeactivateAndUndo::<Impl, IMPL_OFFSET>,
            OnPosRectChange::<Impl, IMPL_OFFSET>,
            OnInPlaceActivateEx::<Impl, IMPL_OFFSET>,
            OnInPlaceDeactivateEx::<Impl, IMPL_OFFSET>,
            RequestUIActivate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceSiteEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceSiteWindowlessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowlessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceSiteWindowlessVtbl {
        unsafe extern "system" fn CanWindowlessActivate<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCapture<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCapture<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcapture: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFocus<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFocus<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDC<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, grfflags: u32, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvalidateRect<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvalidateRgn<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScrollRect<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdjustRect<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDefWindowMessage<Impl: IOleInPlaceSiteWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetWindow::<Impl, IMPL_OFFSET>,
            ContextSensitiveHelp::<Impl, IMPL_OFFSET>,
            CanInPlaceActivate::<Impl, IMPL_OFFSET>,
            OnInPlaceActivate::<Impl, IMPL_OFFSET>,
            OnUIActivate::<Impl, IMPL_OFFSET>,
            GetWindowContext::<Impl, IMPL_OFFSET>,
            Scroll::<Impl, IMPL_OFFSET>,
            OnUIDeactivate::<Impl, IMPL_OFFSET>,
            OnInPlaceDeactivate::<Impl, IMPL_OFFSET>,
            DiscardUndoState::<Impl, IMPL_OFFSET>,
            DeactivateAndUndo::<Impl, IMPL_OFFSET>,
            OnPosRectChange::<Impl, IMPL_OFFSET>,
            OnInPlaceActivateEx::<Impl, IMPL_OFFSET>,
            OnInPlaceDeactivateEx::<Impl, IMPL_OFFSET>,
            RequestUIActivate::<Impl, IMPL_OFFSET>,
            CanWindowlessActivate::<Impl, IMPL_OFFSET>,
            GetCapture::<Impl, IMPL_OFFSET>,
            SetCapture::<Impl, IMPL_OFFSET>,
            GetFocus::<Impl, IMPL_OFFSET>,
            SetFocus::<Impl, IMPL_OFFSET>,
            GetDC::<Impl, IMPL_OFFSET>,
            ReleaseDC::<Impl, IMPL_OFFSET>,
            InvalidateRect::<Impl, IMPL_OFFSET>,
            InvalidateRgn::<Impl, IMPL_OFFSET>,
            ScrollRect::<Impl, IMPL_OFFSET>,
            AdjustRect::<Impl, IMPL_OFFSET>,
            OnDefWindowMessage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceSiteWindowless as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceUIWindowImpl: Sized + IOleWindowImpl {
    fn GetBorder();
    fn RequestBorderSpace();
    fn SetBorderSpace();
    fn SetActiveObject();
}
#[cfg(feature = "Win32_Foundation")]
impl IOleInPlaceUIWindowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceUIWindowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceUIWindowVtbl {
        unsafe extern "system" fn GetBorder<Impl: IOleInPlaceUIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprectborder: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestBorderSpace<Impl: IOleInPlaceUIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBorderSpace<Impl: IOleInPlaceUIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActiveObject<Impl: IOleInPlaceUIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactiveobject: ::windows::core::RawPtr, pszobjname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWindow::<Impl, IMPL_OFFSET>, ContextSensitiveHelp::<Impl, IMPL_OFFSET>, GetBorder::<Impl, IMPL_OFFSET>, RequestBorderSpace::<Impl, IMPL_OFFSET>, SetBorderSpace::<Impl, IMPL_OFFSET>, SetActiveObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceUIWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleItemContainerImpl: Sized + IOleContainerImpl + IParseDisplayNameImpl {
    fn GetObject();
    fn GetObjectStorage();
    fn IsRunning();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleItemContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleItemContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleItemContainerVtbl {
        unsafe extern "system" fn GetObject<Impl: IOleItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR, dwspeedneeded: u32, pbc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectStorage<Impl: IOleItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRunning<Impl: IOleItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ParseDisplayName::<Impl, IMPL_OFFSET>, EnumObjects::<Impl, IMPL_OFFSET>, LockContainer::<Impl, IMPL_OFFSET>, GetObject::<Impl, IMPL_OFFSET>, GetObjectStorage::<Impl, IMPL_OFFSET>, IsRunning::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleItemContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleLinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleLinkVtbl {
        unsafe extern "system" fn SetUpdateOptions<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUpdateOptions<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSourceMoniker<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceMoniker<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSourceDisplayName<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceDisplayName<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindToSource<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindflags: u32, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindIfRunning<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoundSource<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnbindSource<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: IOleLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetUpdateOptions::<Impl, IMPL_OFFSET>,
            GetUpdateOptions::<Impl, IMPL_OFFSET>,
            SetSourceMoniker::<Impl, IMPL_OFFSET>,
            GetSourceMoniker::<Impl, IMPL_OFFSET>,
            SetSourceDisplayName::<Impl, IMPL_OFFSET>,
            GetSourceDisplayName::<Impl, IMPL_OFFSET>,
            BindToSource::<Impl, IMPL_OFFSET>,
            BindIfRunning::<Impl, IMPL_OFFSET>,
            GetBoundSource::<Impl, IMPL_OFFSET>,
            UnbindSource::<Impl, IMPL_OFFSET>,
            Update::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleLink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleObjectVtbl {
        unsafe extern "system" fn SetClientSite<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientsite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClientSite<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclientsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHostNames<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcontainerapp: super::super::Foundation::PWSTR, szcontainerobj: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsaveoption: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMoniker<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwhichmoniker: u32, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMoniker<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitFromData<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipboardData<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoVerb<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: ::windows::core::RawPtr, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumVerbs<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumoleverb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUpToDate<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserClassID<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserType<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwformoftype: u32, pszusertype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExtent<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtent<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Advise<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvsink: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unadvise<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAdvise<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMiscStatus<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorScheme<Impl: IOleObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetClientSite::<Impl, IMPL_OFFSET>,
            GetClientSite::<Impl, IMPL_OFFSET>,
            SetHostNames::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            SetMoniker::<Impl, IMPL_OFFSET>,
            GetMoniker::<Impl, IMPL_OFFSET>,
            InitFromData::<Impl, IMPL_OFFSET>,
            GetClipboardData::<Impl, IMPL_OFFSET>,
            DoVerb::<Impl, IMPL_OFFSET>,
            EnumVerbs::<Impl, IMPL_OFFSET>,
            Update::<Impl, IMPL_OFFSET>,
            IsUpToDate::<Impl, IMPL_OFFSET>,
            GetUserClassID::<Impl, IMPL_OFFSET>,
            GetUserType::<Impl, IMPL_OFFSET>,
            SetExtent::<Impl, IMPL_OFFSET>,
            GetExtent::<Impl, IMPL_OFFSET>,
            Advise::<Impl, IMPL_OFFSET>,
            Unadvise::<Impl, IMPL_OFFSET>,
            EnumAdvise::<Impl, IMPL_OFFSET>,
            GetMiscStatus::<Impl, IMPL_OFFSET>,
            SetColorScheme::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleParentUndoUnitImpl: Sized + IOleUndoUnitImpl {
    fn Open();
    fn Close();
    fn Add();
    fn FindUnit();
    fn GetParentState();
}
#[cfg(feature = "Win32_Foundation")]
impl IOleParentUndoUnitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleParentUndoUnitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleParentUndoUnitVtbl {
        unsafe extern "system" fn Open<Impl: IOleParentUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IOleParentUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr, fcommit: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IOleParentUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindUnit<Impl: IOleParentUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParentState<Impl: IOleParentUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Do::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>, GetUnitType::<Impl, IMPL_OFFSET>, OnNextAdd::<Impl, IMPL_OFFSET>, Open::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, FindUnit::<Impl, IMPL_OFFSET>, GetParentState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleParentUndoUnit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkContainerAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerAImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUILinkContainerAVtbl {
        unsafe extern "system" fn GetNextLink<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLinkSource<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: super::super::Foundation::PSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLinkSource<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PSTR, lplpszshortlinktype: *mut super::super::Foundation::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenLinkSource<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateLink<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelLink<Impl: IOleUILinkContainerAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetNextLink::<Impl, IMPL_OFFSET>, SetLinkUpdateOptions::<Impl, IMPL_OFFSET>, GetLinkUpdateOptions::<Impl, IMPL_OFFSET>, SetLinkSource::<Impl, IMPL_OFFSET>, GetLinkSource::<Impl, IMPL_OFFSET>, OpenLinkSource::<Impl, IMPL_OFFSET>, UpdateLink::<Impl, IMPL_OFFSET>, CancelLink::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUILinkContainerA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkContainerWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUILinkContainerWVtbl {
        unsafe extern "system" fn GetNextLink<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLinkSource<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: super::super::Foundation::PWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLinkSource<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PWSTR, lplpszshortlinktype: *mut super::super::Foundation::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenLinkSource<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateLink<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelLink<Impl: IOleUILinkContainerWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetNextLink::<Impl, IMPL_OFFSET>, SetLinkUpdateOptions::<Impl, IMPL_OFFSET>, GetLinkUpdateOptions::<Impl, IMPL_OFFSET>, SetLinkSource::<Impl, IMPL_OFFSET>, GetLinkSource::<Impl, IMPL_OFFSET>, OpenLinkSource::<Impl, IMPL_OFFSET>, UpdateLink::<Impl, IMPL_OFFSET>, CancelLink::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUILinkContainerW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkInfoAImpl: Sized + IOleUILinkContainerAImpl {
    fn GetLastUpdate();
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkInfoAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkInfoAImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUILinkInfoAVtbl {
        unsafe extern "system" fn GetLastUpdate<Impl: IOleUILinkInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetNextLink::<Impl, IMPL_OFFSET>,
            SetLinkUpdateOptions::<Impl, IMPL_OFFSET>,
            GetLinkUpdateOptions::<Impl, IMPL_OFFSET>,
            SetLinkSource::<Impl, IMPL_OFFSET>,
            GetLinkSource::<Impl, IMPL_OFFSET>,
            OpenLinkSource::<Impl, IMPL_OFFSET>,
            UpdateLink::<Impl, IMPL_OFFSET>,
            CancelLink::<Impl, IMPL_OFFSET>,
            GetLastUpdate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUILinkInfoA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkInfoWImpl: Sized + IOleUILinkContainerWImpl {
    fn GetLastUpdate();
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkInfoWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkInfoWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUILinkInfoWVtbl {
        unsafe extern "system" fn GetLastUpdate<Impl: IOleUILinkInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetNextLink::<Impl, IMPL_OFFSET>,
            SetLinkUpdateOptions::<Impl, IMPL_OFFSET>,
            GetLinkUpdateOptions::<Impl, IMPL_OFFSET>,
            SetLinkSource::<Impl, IMPL_OFFSET>,
            GetLinkSource::<Impl, IMPL_OFFSET>,
            OpenLinkSource::<Impl, IMPL_OFFSET>,
            UpdateLink::<Impl, IMPL_OFFSET>,
            CancelLink::<Impl, IMPL_OFFSET>,
            GetLastUpdate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUILinkInfoW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUIObjInfoAImpl: Sized {
    fn GetObjectInfo();
    fn GetConvertInfo();
    fn ConvertObject();
    fn GetViewInfo();
    fn SetViewInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUIObjInfoAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoAImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUIObjInfoAVtbl {
        unsafe extern "system" fn GetObjectInfo<Impl: IOleUIObjInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PSTR, lplpsztype: *mut super::super::Foundation::PSTR, lplpszshorttype: *mut super::super::Foundation::PSTR, lplpszlocation: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConvertInfo<Impl: IOleUIObjInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertObject<Impl: IOleUIObjInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetViewInfo<Impl: IOleUIObjInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetViewInfo<Impl: IOleUIObjInfoAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetObjectInfo::<Impl, IMPL_OFFSET>, GetConvertInfo::<Impl, IMPL_OFFSET>, ConvertObject::<Impl, IMPL_OFFSET>, GetViewInfo::<Impl, IMPL_OFFSET>, SetViewInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUIObjInfoA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUIObjInfoWImpl: Sized {
    fn GetObjectInfo();
    fn GetConvertInfo();
    fn ConvertObject();
    fn GetViewInfo();
    fn SetViewInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUIObjInfoWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUIObjInfoWVtbl {
        unsafe extern "system" fn GetObjectInfo<Impl: IOleUIObjInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PWSTR, lplpsztype: *mut super::super::Foundation::PWSTR, lplpszshorttype: *mut super::super::Foundation::PWSTR, lplpszlocation: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConvertInfo<Impl: IOleUIObjInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertObject<Impl: IOleUIObjInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetViewInfo<Impl: IOleUIObjInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetViewInfo<Impl: IOleUIObjInfoWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetObjectInfo::<Impl, IMPL_OFFSET>, GetConvertInfo::<Impl, IMPL_OFFSET>, ConvertObject::<Impl, IMPL_OFFSET>, GetViewInfo::<Impl, IMPL_OFFSET>, SetViewInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUIObjInfoW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IOleUndoManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUndoManagerVtbl {
        unsafe extern "system" fn Open<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr, fcommit: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOpenParentState<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscardFrom<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UndoTo<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RedoTo<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumUndoable<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRedoable<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastUndoDescription<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastRedoDescription<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enable<Impl: IOleUndoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            GetOpenParentState::<Impl, IMPL_OFFSET>,
            DiscardFrom::<Impl, IMPL_OFFSET>,
            UndoTo::<Impl, IMPL_OFFSET>,
            RedoTo::<Impl, IMPL_OFFSET>,
            EnumUndoable::<Impl, IMPL_OFFSET>,
            EnumRedoable::<Impl, IMPL_OFFSET>,
            GetLastUndoDescription::<Impl, IMPL_OFFSET>,
            GetLastRedoDescription::<Impl, IMPL_OFFSET>,
            Enable::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUndoManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUndoUnitImpl: Sized {
    fn Do();
    fn GetDescription();
    fn GetUnitType();
    fn OnNextAdd();
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUndoUnitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoUnitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUndoUnitVtbl {
        unsafe extern "system" fn Do<Impl: IOleUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pundomanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: IOleUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnitType<Impl: IOleUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnNextAdd<Impl: IOleUndoUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Do::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>, GetUnitType::<Impl, IMPL_OFFSET>, OnNextAdd::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUndoUnit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleWindowImpl: Sized {
    fn GetWindow();
    fn ContextSensitiveHelp();
}
#[cfg(feature = "Win32_Foundation")]
impl IOleWindowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleWindowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleWindowVtbl {
        unsafe extern "system" fn GetWindow<Impl: IOleWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContextSensitiveHelp<Impl: IOleWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWindow::<Impl, IMPL_OFFSET>, ContextSensitiveHelp::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IParseDisplayNameImpl: Sized {
    fn ParseDisplayName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IParseDisplayNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IParseDisplayNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IParseDisplayNameVtbl {
        unsafe extern "system" fn ParseDisplayName<Impl: IParseDisplayNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ParseDisplayName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IParseDisplayName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPerPropertyBrowsingImpl: Sized {
    fn GetDisplayString();
    fn MapPropertyToPage();
    fn GetPredefinedStrings();
    fn GetPredefinedValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPerPropertyBrowsingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerPropertyBrowsingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerPropertyBrowsingVtbl {
        unsafe extern "system" fn GetDisplayString<Impl: IPerPropertyBrowsingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MapPropertyToPage<Impl: IPerPropertyBrowsingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPredefinedStrings<Impl: IPerPropertyBrowsingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPredefinedValue<Impl: IPerPropertyBrowsingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, dwcookie: u32, pvarout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDisplayString::<Impl, IMPL_OFFSET>, MapPropertyToPage::<Impl, IMPL_OFFSET>, GetPredefinedStrings::<Impl, IMPL_OFFSET>, GetPredefinedValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerPropertyBrowsing as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPersistPropertyBagImpl: Sized + IPersistImpl {
    fn InitNew();
    fn Load();
    fn Save();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPersistPropertyBagVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBagImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistPropertyBagVtbl {
        unsafe extern "system" fn InitNew<Impl: IPersistPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IPersistPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, perrorlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IPersistPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassID::<Impl, IMPL_OFFSET>, InitNew::<Impl, IMPL_OFFSET>, Load::<Impl, IMPL_OFFSET>, Save::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistPropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPersistPropertyBag2Impl: Sized + IPersistImpl {
    fn InitNew();
    fn Load();
    fn Save();
    fn IsDirty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPersistPropertyBag2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistPropertyBag2Vtbl {
        unsafe extern "system" fn InitNew<Impl: IPersistPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IPersistPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, perrlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IPersistPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDirty<Impl: IPersistPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassID::<Impl, IMPL_OFFSET>, InitNew::<Impl, IMPL_OFFSET>, Load::<Impl, IMPL_OFFSET>, Save::<Impl, IMPL_OFFSET>, IsDirty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistPropertyBag2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPictureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPictureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPictureVtbl {
        unsafe extern "system" fn Handle<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn hPal<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phpal: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Width<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Height<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Render<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn set_hPal<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpal: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurDC<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectPicture<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeepOriginalFormat<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PictureChanged<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveAsFile<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Attributes<Impl: IPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Handle::<Impl, IMPL_OFFSET>,
            hPal::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            Width::<Impl, IMPL_OFFSET>,
            Height::<Impl, IMPL_OFFSET>,
            Render::<Impl, IMPL_OFFSET>,
            set_hPal::<Impl, IMPL_OFFSET>,
            CurDC::<Impl, IMPL_OFFSET>,
            SelectPicture::<Impl, IMPL_OFFSET>,
            KeepOriginalFormat::<Impl, IMPL_OFFSET>,
            SetKeepOriginalFormat::<Impl, IMPL_OFFSET>,
            PictureChanged::<Impl, IMPL_OFFSET>,
            SaveAsFile::<Impl, IMPL_OFFSET>,
            Attributes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPicture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPicture2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPicture2Vtbl {
        unsafe extern "system" fn Handle<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn hPal<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phpal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Width<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Height<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Render<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn set_hPal<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpal: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurDC<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectPicture<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeepOriginalFormat<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PictureChanged<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveAsFile<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Attributes<Impl: IPicture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Handle::<Impl, IMPL_OFFSET>,
            hPal::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            Width::<Impl, IMPL_OFFSET>,
            Height::<Impl, IMPL_OFFSET>,
            Render::<Impl, IMPL_OFFSET>,
            set_hPal::<Impl, IMPL_OFFSET>,
            CurDC::<Impl, IMPL_OFFSET>,
            SelectPicture::<Impl, IMPL_OFFSET>,
            KeepOriginalFormat::<Impl, IMPL_OFFSET>,
            SetKeepOriginalFormat::<Impl, IMPL_OFFSET>,
            PictureChanged::<Impl, IMPL_OFFSET>,
            SaveAsFile::<Impl, IMPL_OFFSET>,
            Attributes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPicture2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPictureDispImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPictureDispVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPictureDispImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPictureDispVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPictureDisp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPointerInactiveImpl: Sized {
    fn GetActivationPolicy();
    fn OnInactiveMouseMove();
    fn OnInactiveSetCursor();
}
#[cfg(feature = "Win32_Foundation")]
impl IPointerInactiveVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerInactiveImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerInactiveVtbl {
        unsafe extern "system" fn GetActivationPolicy<Impl: IPointerInactiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpolicy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInactiveMouseMove<Impl: IPointerInactiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInactiveSetCursor<Impl: IPointerInactiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetActivationPolicy::<Impl, IMPL_OFFSET>, OnInactiveMouseMove::<Impl, IMPL_OFFSET>, OnInactiveSetCursor::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerInactive as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPrintImpl: Sized {
    fn SetInitialPageNum();
    fn GetPageInfo();
    fn Print();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPrintVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintVtbl {
        unsafe extern "system" fn SetInitialPageNum<Impl: IPrintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nfirstpage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPageInfo<Impl: IPrintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Print<Impl: IPrintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: ::windows::core::RawPtr, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetInitialPageNum::<Impl, IMPL_OFFSET>, GetPageInfo::<Impl, IMPL_OFFSET>, Print::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint as ::windows::core::Interface>::IID
    }
}
pub trait IPropertyNotifySinkImpl: Sized {
    fn OnChanged();
    fn OnRequestEdit();
}
impl IPropertyNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyNotifySinkVtbl {
        unsafe extern "system" fn OnChanged<Impl: IPropertyNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnRequestEdit<Impl: IPropertyNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnChanged::<Impl, IMPL_OFFSET>, OnRequestEdit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPropertyPageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyPageVtbl {
        unsafe extern "system" fn SetPageSite<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagesite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Activate<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deactivate<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPageInfo<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppageinfo: *mut PROPPAGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetObjects<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32, ppunk: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Show<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncmdshow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPageDirty<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Apply<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Help<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelpdir: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IPropertyPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPageSite::<Impl, IMPL_OFFSET>,
            Activate::<Impl, IMPL_OFFSET>,
            Deactivate::<Impl, IMPL_OFFSET>,
            GetPageInfo::<Impl, IMPL_OFFSET>,
            SetObjects::<Impl, IMPL_OFFSET>,
            Show::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            IsPageDirty::<Impl, IMPL_OFFSET>,
            Apply::<Impl, IMPL_OFFSET>,
            Help::<Impl, IMPL_OFFSET>,
            TranslateAccelerator::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyPage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPropertyPage2Impl: Sized + IPropertyPageImpl {
    fn EditProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPropertyPage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyPage2Vtbl {
        unsafe extern "system" fn EditProperty<Impl: IPropertyPage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPageSite::<Impl, IMPL_OFFSET>,
            Activate::<Impl, IMPL_OFFSET>,
            Deactivate::<Impl, IMPL_OFFSET>,
            GetPageInfo::<Impl, IMPL_OFFSET>,
            SetObjects::<Impl, IMPL_OFFSET>,
            Show::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            IsPageDirty::<Impl, IMPL_OFFSET>,
            Apply::<Impl, IMPL_OFFSET>,
            Help::<Impl, IMPL_OFFSET>,
            TranslateAccelerator::<Impl, IMPL_OFFSET>,
            EditProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyPage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPropertyPageSiteImpl: Sized {
    fn OnStatusChange();
    fn GetLocaleID();
    fn GetPageContainer();
    fn TranslateAccelerator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPropertyPageSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPageSiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyPageSiteVtbl {
        unsafe extern "system" fn OnStatusChange<Impl: IPropertyPageSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: PROPPAGESTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleID<Impl: IPropertyPageSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocaleid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPageContainer<Impl: IPropertyPageSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IPropertyPageSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnStatusChange::<Impl, IMPL_OFFSET>, GetLocaleID::<Impl, IMPL_OFFSET>, GetPageContainer::<Impl, IMPL_OFFSET>, TranslateAccelerator::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyPageSite as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProtectFocusImpl: Sized {
    fn AllowFocusChange();
}
#[cfg(feature = "Win32_Foundation")]
impl IProtectFocusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectFocusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtectFocusVtbl {
        unsafe extern "system" fn AllowFocusChange<Impl: IProtectFocusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AllowFocusChange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtectFocus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IProtectedModeMenuServicesImpl: Sized {
    fn CreateMenu();
    fn LoadMenu();
    fn LoadMenuID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IProtectedModeMenuServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedModeMenuServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtectedModeMenuServicesVtbl {
        unsafe extern "system" fn CreateMenu<Impl: IProtectedModeMenuServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadMenu<Impl: IProtectedModeMenuServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmodulename: super::super::Foundation::PWSTR, pszmenuname: super::super::Foundation::PWSTR, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadMenuID<Impl: IProtectedModeMenuServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmodulename: super::super::Foundation::PWSTR, wresourceid: u16, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateMenu::<Impl, IMPL_OFFSET>, LoadMenu::<Impl, IMPL_OFFSET>, LoadMenuID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtectedModeMenuServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideClassInfoImpl: Sized {
    fn GetClassInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl IProvideClassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideClassInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideClassInfoVtbl {
        unsafe extern "system" fn GetClassInfo<Impl: IProvideClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppti: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideClassInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideClassInfo2Impl: Sized + IProvideClassInfoImpl {
    fn GetGUID();
}
#[cfg(feature = "Win32_System_Com")]
impl IProvideClassInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideClassInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideClassInfo2Vtbl {
        unsafe extern "system" fn GetGUID<Impl: IProvideClassInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwguidkind: u32, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassInfo::<Impl, IMPL_OFFSET>, GetGUID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideClassInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideMultipleClassInfoImpl: Sized + IProvideClassInfo2Impl + IProvideClassInfoImpl {
    fn GetMultiTypeInfoCount();
    fn GetInfoOfIndex();
}
#[cfg(feature = "Win32_System_Com")]
impl IProvideMultipleClassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideMultipleClassInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideMultipleClassInfoVtbl {
        unsafe extern "system" fn GetMultiTypeInfoCount<Impl: IProvideMultipleClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcti: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInfoOfIndex<Impl: IProvideMultipleClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut ::windows::core::RawPtr, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows::core::GUID, piidsource: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassInfo::<Impl, IMPL_OFFSET>, GetGUID::<Impl, IMPL_OFFSET>, GetMultiTypeInfoCount::<Impl, IMPL_OFFSET>, GetInfoOfIndex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideMultipleClassInfo as ::windows::core::Interface>::IID
    }
}
pub trait IProvideRuntimeContextImpl: Sized {
    fn GetCurrentSourceContext();
}
impl IProvideRuntimeContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideRuntimeContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideRuntimeContextVtbl {
        unsafe extern "system" fn GetCurrentSourceContext<Impl: IProvideRuntimeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcontext: *mut usize, pfexecutingglobalcode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCurrentSourceContext::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideRuntimeContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IQuickActivateImpl: Sized {
    fn QuickActivate();
    fn SetContentExtent();
    fn GetContentExtent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IQuickActivateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuickActivateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuickActivateVtbl {
        unsafe extern "system" fn QuickActivate<Impl: IQuickActivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContentExtent<Impl: IQuickActivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentExtent<Impl: IQuickActivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QuickActivate::<Impl, IMPL_OFFSET>, SetContentExtent::<Impl, IMPL_OFFSET>, GetContentExtent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuickActivate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRecordInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRecordInfoVtbl {
        unsafe extern "system" fn RecordInit<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecordClear<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecordCopy<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGuid<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSize<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetField<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFieldNoCopy<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *mut super::Com::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutField<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutFieldNoCopy<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFieldNames<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnames: *mut u32, rgbstrnames: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsMatchingType<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordinfo: ::windows::core::RawPtr) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecordCreate<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecordCreateCopy<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecordDestroy<Impl: IRecordInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvrecord: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            RecordInit::<Impl, IMPL_OFFSET>,
            RecordClear::<Impl, IMPL_OFFSET>,
            RecordCopy::<Impl, IMPL_OFFSET>,
            GetGuid::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetField::<Impl, IMPL_OFFSET>,
            GetFieldNoCopy::<Impl, IMPL_OFFSET>,
            PutField::<Impl, IMPL_OFFSET>,
            PutFieldNoCopy::<Impl, IMPL_OFFSET>,
            GetFieldNames::<Impl, IMPL_OFFSET>,
            IsMatchingType::<Impl, IMPL_OFFSET>,
            RecordCreate::<Impl, IMPL_OFFSET>,
            RecordCreateCopy::<Impl, IMPL_OFFSET>,
            RecordDestroy::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRecordInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimpleFrameSiteImpl: Sized {
    fn PreMessageFilter();
    fn PostMessageFilter();
}
#[cfg(feature = "Win32_Foundation")]
impl ISimpleFrameSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleFrameSiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleFrameSiteVtbl {
        unsafe extern "system" fn PreMessageFilter<Impl: ISimpleFrameSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostMessageFilter<Impl: ISimpleFrameSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, PreMessageFilter::<Impl, IMPL_OFFSET>, PostMessageFilter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleFrameSite as ::windows::core::Interface>::IID
    }
}
pub trait ISpecifyPropertyPagesImpl: Sized {
    fn GetPages();
}
impl ISpecifyPropertyPagesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpecifyPropertyPagesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpecifyPropertyPagesVtbl {
        unsafe extern "system" fn GetPages<Impl: ISpecifyPropertyPagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppages: *mut CAUUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPages::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpecifyPropertyPages as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITypeChangeEventsImpl: Sized {
    fn RequestTypeChange();
    fn AfterTypeChange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITypeChangeEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeChangeEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeChangeEventsVtbl {
        unsafe extern "system" fn RequestTypeChange<Impl: ITypeChangeEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfobefore: ::windows::core::RawPtr, pstrname: super::super::Foundation::PWSTR, pfcancel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AfterTypeChange<Impl: ITypeChangeEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfoafter: ::windows::core::RawPtr, pstrname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RequestTypeChange::<Impl, IMPL_OFFSET>, AfterTypeChange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeChangeEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITypeFactoryImpl: Sized {
    fn CreateFromTypeInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ITypeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeFactoryVtbl {
        unsafe extern "system" fn CreateFromTypeInfo<Impl: ITypeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeinfo: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateFromTypeInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeFactory as ::windows::core::Interface>::IID
    }
}
pub trait ITypeMarshalImpl: Sized {
    fn Size();
    fn Marshal();
    fn Unmarshal();
    fn Free();
}
impl ITypeMarshalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeMarshalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeMarshalVtbl {
        unsafe extern "system" fn Size<Impl: ITypeMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, psize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Marshal<Impl: ITypeMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unmarshal<Impl: ITypeMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *mut ::core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Free<Impl: ITypeMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Size::<Impl, IMPL_OFFSET>, Marshal::<Impl, IMPL_OFFSET>, Unmarshal::<Impl, IMPL_OFFSET>, Free::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeMarshal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVBFormatImpl: Sized {
    fn Format();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVBFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBFormatVtbl {
        unsafe extern "system" fn Format<Impl: IVBFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdata: *mut super::Com::VARIANT, bstrformat: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Format::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBGetControlImpl: Sized {
    fn EnumControls();
}
#[cfg(feature = "Win32_System_Com")]
impl IVBGetControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBGetControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBGetControlVtbl {
        unsafe extern "system" fn EnumControls<Impl: IVBGetControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwolecontf: OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS, ppenumunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, EnumControls::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBGetControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVariantChangeTypeImpl: Sized {
    fn ChangeType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVariantChangeTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVariantChangeTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVariantChangeTypeVtbl {
        unsafe extern "system" fn ChangeType<Impl: IVariantChangeTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardst: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, vtnew: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ChangeType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVariantChangeType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObjectImpl: Sized {
    fn Draw();
    fn GetColorSet();
    fn Freeze();
    fn Unfreeze();
    fn SetAdvise();
    fn GetAdvise();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewObjectVtbl {
        unsafe extern "system" fn Draw<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorSet<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Freeze<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unfreeze<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfreeze: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAdvise<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aspects: u32, advf: u32, padvsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdvise<Impl: IViewObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Draw::<Impl, IMPL_OFFSET>, GetColorSet::<Impl, IMPL_OFFSET>, Freeze::<Impl, IMPL_OFFSET>, Unfreeze::<Impl, IMPL_OFFSET>, SetAdvise::<Impl, IMPL_OFFSET>, GetAdvise::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObject2Impl: Sized + IViewObjectImpl {
    fn GetExtent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObject2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewObject2Vtbl {
        unsafe extern "system" fn GetExtent<Impl: IViewObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, lpsizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Draw::<Impl, IMPL_OFFSET>, GetColorSet::<Impl, IMPL_OFFSET>, Freeze::<Impl, IMPL_OFFSET>, Unfreeze::<Impl, IMPL_OFFSET>, SetAdvise::<Impl, IMPL_OFFSET>, GetAdvise::<Impl, IMPL_OFFSET>, GetExtent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewObject2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObjectExImpl: Sized + IViewObject2Impl + IViewObjectImpl {
    fn GetRect();
    fn GetViewStatus();
    fn QueryHitPoint();
    fn QueryHitRect();
    fn GetNaturalExtent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObjectExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObjectExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewObjectExVtbl {
        unsafe extern "system" fn GetRect<Impl: IViewObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prect: *mut super::super::Foundation::RECTL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetViewStatus<Impl: IViewObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryHitPoint<Impl: IViewObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryHitRect<Impl: IViewObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNaturalExtent<Impl: IViewObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const ExtentInfo, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            GetColorSet::<Impl, IMPL_OFFSET>,
            Freeze::<Impl, IMPL_OFFSET>,
            Unfreeze::<Impl, IMPL_OFFSET>,
            SetAdvise::<Impl, IMPL_OFFSET>,
            GetAdvise::<Impl, IMPL_OFFSET>,
            GetExtent::<Impl, IMPL_OFFSET>,
            GetRect::<Impl, IMPL_OFFSET>,
            GetViewStatus::<Impl, IMPL_OFFSET>,
            QueryHitPoint::<Impl, IMPL_OFFSET>,
            QueryHitRect::<Impl, IMPL_OFFSET>,
            GetNaturalExtent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewObjectEx as ::windows::core::Interface>::IID
    }
}
pub trait IZoomEventsImpl: Sized {
    fn OnZoomPercentChanged();
}
impl IZoomEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoomEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IZoomEventsVtbl {
        unsafe extern "system" fn OnZoomPercentChanged<Impl: IZoomEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulzoompercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnZoomPercentChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IZoomEvents as ::windows::core::Interface>::IID
    }
}
