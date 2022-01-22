#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSinkEx_Impl: Sized + super::Com::IAdviseSink_Impl {
    fn OnViewStatusChange(&mut self, dwviewstatus: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSinkEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdviseSinkEx_Impl, const OFFSET: isize>() -> IAdviseSinkEx_Vtbl {
        unsafe extern "system" fn OnViewStatusChange<Identity: ::windows::core::IUnknownImpl, Impl: IAdviseSinkEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwviewstatus: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnViewStatusChange(::core::mem::transmute_copy(&dwviewstatus))
        }
        Self { base: super::Com::IAdviseSink_Vtbl::new::<Identity, Impl, OFFSET>(), OnViewStatusChange: OnViewStatusChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdviseSinkEx as ::windows::core::Interface>::IID || iid == &<super::Com::IAdviseSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICanHandleException_Impl: Sized {
    fn CanHandleException(&mut self, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICanHandleException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICanHandleException_Impl, const OFFSET: isize>() -> ICanHandleException_Vtbl {
        unsafe extern "system" fn CanHandleException<Identity: ::windows::core::IUnknownImpl, Impl: ICanHandleException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CanHandleException(::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&pvar)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CanHandleException: CanHandleException::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanHandleException as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IClassFactory2_Impl: Sized + super::Com::IClassFactory_Impl {
    fn GetLicInfo(&mut self, plicinfo: *mut LICINFO) -> ::windows::core::Result<()>;
    fn RequestLicKey(&mut self, dwreserved: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateInstanceLic(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, punkreserved: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, bstrkey: &super::super::Foundation::BSTR, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IClassFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClassFactory2_Impl, const OFFSET: isize>() -> IClassFactory2_Vtbl {
        unsafe extern "system" fn GetLicInfo<Identity: ::windows::core::IUnknownImpl, Impl: IClassFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plicinfo: *mut LICINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLicInfo(::core::mem::transmute_copy(&plicinfo)).into()
        }
        unsafe extern "system" fn RequestLicKey<Identity: ::windows::core::IUnknownImpl, Impl: IClassFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, pbstrkey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestLicKey(::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceLic<Identity: ::windows::core::IUnknownImpl, Impl: IClassFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, bstrkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateInstanceLic(::core::mem::transmute(&punkouter), ::core::mem::transmute(&punkreserved), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&bstrkey), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        Self {
            base: super::Com::IClassFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLicInfo: GetLicInfo::<Identity, Impl, OFFSET>,
            RequestLicKey: RequestLicKey::<Identity, Impl, OFFSET>,
            CreateInstanceLic: CreateInstanceLic::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClassFactory2 as ::windows::core::Interface>::IID || iid == &<super::Com::IClassFactory as ::windows::core::Interface>::IID
    }
}
pub trait IContinue_Impl: Sized {
    fn FContinue(&mut self) -> ::windows::core::Result<()>;
}
impl IContinue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinue_Impl, const OFFSET: isize>() -> IContinue_Vtbl {
        unsafe extern "system" fn FContinue<Identity: ::windows::core::IUnknownImpl, Impl: IContinue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FContinue().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), FContinue: FContinue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContinue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContinueCallback_Impl: Sized {
    fn FContinue(&mut self) -> ::windows::core::Result<()>;
    fn FContinuePrinting(&mut self, ncntprinted: i32, ncurpage: i32, pwszprintstatus: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContinueCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinueCallback_Impl, const OFFSET: isize>() -> IContinueCallback_Vtbl {
        unsafe extern "system" fn FContinue<Identity: ::windows::core::IUnknownImpl, Impl: IContinueCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FContinue().into()
        }
        unsafe extern "system" fn FContinuePrinting<Identity: ::windows::core::IUnknownImpl, Impl: IContinueCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncntprinted: i32, ncurpage: i32, pwszprintstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FContinuePrinting(::core::mem::transmute_copy(&ncntprinted), ::core::mem::transmute_copy(&ncurpage), ::core::mem::transmute_copy(&pwszprintstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FContinue: FContinue::<Identity, Impl, OFFSET>,
            FContinuePrinting: FContinuePrinting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContinueCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICreateErrorInfo_Impl: Sized {
    fn SetGUID(&mut self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetSource(&mut self, szsource: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetDescription(&mut self, szdescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetHelpFile(&mut self, szhelpfile: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetHelpContext(&mut self, dwhelpcontext: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICreateErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>() -> ICreateErrorInfo_Vtbl {
        unsafe extern "system" fn SetGUID<Identity: ::windows::core::IUnknownImpl, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGUID(::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn SetSource<Identity: ::windows::core::IUnknownImpl, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsource: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSource(::core::mem::transmute_copy(&szsource)).into()
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&szdescription)).into()
        }
        unsafe extern "system" fn SetHelpFile<Identity: ::windows::core::IUnknownImpl, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhelpfile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelpFile(::core::mem::transmute_copy(&szhelpfile)).into()
        }
        unsafe extern "system" fn SetHelpContext<Identity: ::windows::core::IUnknownImpl, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelpContext(::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetGUID: SetGUID::<Identity, Impl, OFFSET>,
            SetSource: SetSource::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            SetHelpFile: SetHelpFile::<Identity, Impl, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICreateTypeInfo_Impl: Sized {
    fn SetGuid(&mut self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetTypeFlags(&mut self, utypeflags: u32) -> ::windows::core::Result<()>;
    fn SetDocString(&mut self, pstrdoc: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetHelpContext(&mut self, dwhelpcontext: u32) -> ::windows::core::Result<()>;
    fn SetVersion(&mut self, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::Result<()>;
    fn AddRefTypeInfo(&mut self, ptinfo: &::core::option::Option<super::Com::ITypeInfo>, phreftype: *const u32) -> ::windows::core::Result<()>;
    fn AddFuncDesc(&mut self, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows::core::Result<()>;
    fn AddImplType(&mut self, index: u32, hreftype: u32) -> ::windows::core::Result<()>;
    fn SetImplTypeFlags(&mut self, index: u32, impltypeflags: i32) -> ::windows::core::Result<()>;
    fn SetAlignment(&mut self, cbalignment: u16) -> ::windows::core::Result<()>;
    fn SetSchema(&mut self, pstrschema: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddVarDesc(&mut self, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows::core::Result<()>;
    fn SetFuncAndParamNames(&mut self, index: u32, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32) -> ::windows::core::Result<()>;
    fn SetVarName(&mut self, index: u32, szname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetTypeDescAlias(&mut self, ptdescalias: *const super::Com::TYPEDESC) -> ::windows::core::Result<()>;
    fn DefineFuncAsDllEntry(&mut self, index: u32, szdllname: super::super::Foundation::PWSTR, szprocname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetFuncDocString(&mut self, index: u32, szdocstring: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetVarDocString(&mut self, index: u32, szdocstring: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetFuncHelpContext(&mut self, index: u32, dwhelpcontext: u32) -> ::windows::core::Result<()>;
    fn SetVarHelpContext(&mut self, index: u32, dwhelpcontext: u32) -> ::windows::core::Result<()>;
    fn SetMops(&mut self, index: u32, bstrmops: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetTypeIdldesc(&mut self, pidldesc: *const super::Com::IDLDESC) -> ::windows::core::Result<()>;
    fn LayOut(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICreateTypeInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>() -> ICreateTypeInfo_Vtbl {
        unsafe extern "system" fn SetGuid<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGuid(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn SetTypeFlags<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utypeflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTypeFlags(::core::mem::transmute_copy(&utypeflags)).into()
        }
        unsafe extern "system" fn SetDocString<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdoc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDocString(::core::mem::transmute_copy(&pstrdoc)).into()
        }
        unsafe extern "system" fn SetHelpContext<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelpContext(::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&wmajorvernum), ::core::mem::transmute_copy(&wminorvernum)).into()
        }
        unsafe extern "system" fn AddRefTypeInfo<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptinfo: ::windows::core::RawPtr, phreftype: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRefTypeInfo(::core::mem::transmute(&ptinfo), ::core::mem::transmute_copy(&phreftype)).into()
        }
        unsafe extern "system" fn AddFuncDesc<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFuncDesc(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pfuncdesc)).into()
        }
        unsafe extern "system" fn AddImplType<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, hreftype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddImplType(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&hreftype)).into()
        }
        unsafe extern "system" fn SetImplTypeFlags<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, impltypeflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetImplTypeFlags(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&impltypeflags)).into()
        }
        unsafe extern "system" fn SetAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbalignment: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlignment(::core::mem::transmute_copy(&cbalignment)).into()
        }
        unsafe extern "system" fn SetSchema<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrschema: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSchema(::core::mem::transmute_copy(&pstrschema)).into()
        }
        unsafe extern "system" fn AddVarDesc<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddVarDesc(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pvardesc)).into()
        }
        unsafe extern "system" fn SetFuncAndParamNames<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFuncAndParamNames(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames)).into()
        }
        unsafe extern "system" fn SetVarName<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVarName(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&szname)).into()
        }
        unsafe extern "system" fn SetTypeDescAlias<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptdescalias: *const super::Com::TYPEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTypeDescAlias(::core::mem::transmute_copy(&ptdescalias)).into()
        }
        unsafe extern "system" fn DefineFuncAsDllEntry<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdllname: super::super::Foundation::PWSTR, szprocname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DefineFuncAsDllEntry(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&szdllname), ::core::mem::transmute_copy(&szprocname)).into()
        }
        unsafe extern "system" fn SetFuncDocString<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFuncDocString(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&szdocstring)).into()
        }
        unsafe extern "system" fn SetVarDocString<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVarDocString(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&szdocstring)).into()
        }
        unsafe extern "system" fn SetFuncHelpContext<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFuncHelpContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetVarHelpContext<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVarHelpContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetMops<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, bstrmops: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMops(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&bstrmops)).into()
        }
        unsafe extern "system" fn SetTypeIdldesc<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidldesc: *const super::Com::IDLDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTypeIdldesc(::core::mem::transmute_copy(&pidldesc)).into()
        }
        unsafe extern "system" fn LayOut<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LayOut().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetGuid: SetGuid::<Identity, Impl, OFFSET>,
            SetTypeFlags: SetTypeFlags::<Identity, Impl, OFFSET>,
            SetDocString: SetDocString::<Identity, Impl, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            AddRefTypeInfo: AddRefTypeInfo::<Identity, Impl, OFFSET>,
            AddFuncDesc: AddFuncDesc::<Identity, Impl, OFFSET>,
            AddImplType: AddImplType::<Identity, Impl, OFFSET>,
            SetImplTypeFlags: SetImplTypeFlags::<Identity, Impl, OFFSET>,
            SetAlignment: SetAlignment::<Identity, Impl, OFFSET>,
            SetSchema: SetSchema::<Identity, Impl, OFFSET>,
            AddVarDesc: AddVarDesc::<Identity, Impl, OFFSET>,
            SetFuncAndParamNames: SetFuncAndParamNames::<Identity, Impl, OFFSET>,
            SetVarName: SetVarName::<Identity, Impl, OFFSET>,
            SetTypeDescAlias: SetTypeDescAlias::<Identity, Impl, OFFSET>,
            DefineFuncAsDllEntry: DefineFuncAsDllEntry::<Identity, Impl, OFFSET>,
            SetFuncDocString: SetFuncDocString::<Identity, Impl, OFFSET>,
            SetVarDocString: SetVarDocString::<Identity, Impl, OFFSET>,
            SetFuncHelpContext: SetFuncHelpContext::<Identity, Impl, OFFSET>,
            SetVarHelpContext: SetVarHelpContext::<Identity, Impl, OFFSET>,
            SetMops: SetMops::<Identity, Impl, OFFSET>,
            SetTypeIdldesc: SetTypeIdldesc::<Identity, Impl, OFFSET>,
            LayOut: LayOut::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateTypeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICreateTypeInfo2_Impl: Sized + ICreateTypeInfo_Impl {
    fn DeleteFuncDesc(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn DeleteFuncDescByMemId(&mut self, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows::core::Result<()>;
    fn DeleteVarDesc(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn DeleteVarDescByMemId(&mut self, memid: i32) -> ::windows::core::Result<()>;
    fn DeleteImplType(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetCustData(&mut self, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetFuncCustData(&mut self, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetParamCustData(&mut self, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetVarCustData(&mut self, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetImplTypeCustData(&mut self, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetHelpStringContext(&mut self, dwhelpstringcontext: u32) -> ::windows::core::Result<()>;
    fn SetFuncHelpStringContext(&mut self, index: u32, dwhelpstringcontext: u32) -> ::windows::core::Result<()>;
    fn SetVarHelpStringContext(&mut self, index: u32, dwhelpstringcontext: u32) -> ::windows::core::Result<()>;
    fn Invalidate(&mut self) -> ::windows::core::Result<()>;
    fn SetName(&mut self, szname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICreateTypeInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>() -> ICreateTypeInfo2_Vtbl {
        unsafe extern "system" fn DeleteFuncDesc<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteFuncDesc(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn DeleteFuncDescByMemId<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteFuncDescByMemId(::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&invkind)).into()
        }
        unsafe extern "system" fn DeleteVarDesc<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteVarDesc(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn DeleteVarDescByMemId<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteVarDescByMemId(::core::mem::transmute_copy(&memid)).into()
        }
        unsafe extern "system" fn DeleteImplType<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteImplType(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetCustData<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCustData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetFuncCustData<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFuncCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetParamCustData<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParamCustData(::core::mem::transmute_copy(&indexfunc), ::core::mem::transmute_copy(&indexparam), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetVarCustData<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVarCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetImplTypeCustData<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetImplTypeCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelpStringContext(::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetFuncHelpStringContext<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFuncHelpStringContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetVarHelpStringContext<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVarHelpStringContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn Invalidate<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invalidate().into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&szname)).into()
        }
        Self {
            base: ICreateTypeInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            DeleteFuncDesc: DeleteFuncDesc::<Identity, Impl, OFFSET>,
            DeleteFuncDescByMemId: DeleteFuncDescByMemId::<Identity, Impl, OFFSET>,
            DeleteVarDesc: DeleteVarDesc::<Identity, Impl, OFFSET>,
            DeleteVarDescByMemId: DeleteVarDescByMemId::<Identity, Impl, OFFSET>,
            DeleteImplType: DeleteImplType::<Identity, Impl, OFFSET>,
            SetCustData: SetCustData::<Identity, Impl, OFFSET>,
            SetFuncCustData: SetFuncCustData::<Identity, Impl, OFFSET>,
            SetParamCustData: SetParamCustData::<Identity, Impl, OFFSET>,
            SetVarCustData: SetVarCustData::<Identity, Impl, OFFSET>,
            SetImplTypeCustData: SetImplTypeCustData::<Identity, Impl, OFFSET>,
            SetHelpStringContext: SetHelpStringContext::<Identity, Impl, OFFSET>,
            SetFuncHelpStringContext: SetFuncHelpStringContext::<Identity, Impl, OFFSET>,
            SetVarHelpStringContext: SetVarHelpStringContext::<Identity, Impl, OFFSET>,
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateTypeInfo2 as ::windows::core::Interface>::IID || iid == &<ICreateTypeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICreateTypeLib_Impl: Sized {
    fn CreateTypeInfo(&mut self, szname: super::super::Foundation::PWSTR, tkind: super::Com::TYPEKIND) -> ::windows::core::Result<ICreateTypeInfo>;
    fn SetName(&mut self, szname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetVersion(&mut self, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::Result<()>;
    fn SetGuid(&mut self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetDocString(&mut self, szdoc: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetHelpFileName(&mut self, szhelpfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetHelpContext(&mut self, dwhelpcontext: u32) -> ::windows::core::Result<()>;
    fn SetLcid(&mut self, lcid: u32) -> ::windows::core::Result<()>;
    fn SetLibFlags(&mut self, ulibflags: u32) -> ::windows::core::Result<()>;
    fn SaveAllChanges(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICreateTypeLib_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const OFFSET: isize>() -> ICreateTypeLib_Vtbl {
        unsafe extern "system" fn CreateTypeInfo<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, tkind: super::Com::TYPEKIND, ppctinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTypeInfo(::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&tkind)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppctinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&szname)).into()
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&wmajorvernum), ::core::mem::transmute_copy(&wminorvernum)).into()
        }
        unsafe extern "system" fn SetGuid<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGuid(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn SetDocString<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdoc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDocString(::core::mem::transmute_copy(&szdoc)).into()
        }
        unsafe extern "system" fn SetHelpFileName<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhelpfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelpFileName(::core::mem::transmute_copy(&szhelpfilename)).into()
        }
        unsafe extern "system" fn SetHelpContext<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelpContext(::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetLcid<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLcid(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn SetLibFlags<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulibflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLibFlags(::core::mem::transmute_copy(&ulibflags)).into()
        }
        unsafe extern "system" fn SaveAllChanges<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveAllChanges().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateTypeInfo: CreateTypeInfo::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            SetGuid: SetGuid::<Identity, Impl, OFFSET>,
            SetDocString: SetDocString::<Identity, Impl, OFFSET>,
            SetHelpFileName: SetHelpFileName::<Identity, Impl, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, Impl, OFFSET>,
            SetLcid: SetLcid::<Identity, Impl, OFFSET>,
            SetLibFlags: SetLibFlags::<Identity, Impl, OFFSET>,
            SaveAllChanges: SaveAllChanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateTypeLib as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICreateTypeLib2_Impl: Sized + ICreateTypeLib_Impl {
    fn DeleteTypeInfo(&mut self, szname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetCustData(&mut self, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetHelpStringContext(&mut self, dwhelpstringcontext: u32) -> ::windows::core::Result<()>;
    fn SetHelpStringDll(&mut self, szfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICreateTypeLib2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib2_Impl, const OFFSET: isize>() -> ICreateTypeLib2_Vtbl {
        unsafe extern "system" fn DeleteTypeInfo<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteTypeInfo(::core::mem::transmute_copy(&szname)).into()
        }
        unsafe extern "system" fn SetCustData<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCustData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelpStringContext(::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetHelpStringDll<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelpStringDll(::core::mem::transmute_copy(&szfilename)).into()
        }
        Self {
            base: ICreateTypeLib_Vtbl::new::<Identity, Impl, OFFSET>(),
            DeleteTypeInfo: DeleteTypeInfo::<Identity, Impl, OFFSET>,
            SetCustData: SetCustData::<Identity, Impl, OFFSET>,
            SetHelpStringContext: SetHelpStringContext::<Identity, Impl, OFFSET>,
            SetHelpStringDll: SetHelpStringDll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateTypeLib2 as ::windows::core::Interface>::IID || iid == &<ICreateTypeLib as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDispError_Impl: Sized {
    fn QueryErrorInfo(&mut self, guiderrortype: &::windows::core::GUID) -> ::windows::core::Result<IDispError>;
    fn GetNext(&mut self) -> ::windows::core::Result<IDispError>;
    fn GetHresult(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetSource(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetHelpInfo(&mut self, pbstrfilename: *mut super::super::Foundation::BSTR, pdwcontext: *mut u32) -> ::windows::core::Result<()>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDispError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispError_Impl, const OFFSET: isize>() -> IDispError_Vtbl {
        unsafe extern "system" fn QueryErrorInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guiderrortype: ::windows::core::GUID, ppde: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryErrorInfo(::core::mem::transmute_copy(&guiderrortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppde = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNext<Identity: ::windows::core::IUnknownImpl, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppde: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppde = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHresult<Identity: ::windows::core::IUnknownImpl, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHresult() {
                ::core::result::Result::Ok(ok__) => {
                    *phr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: ::windows::core::IUnknownImpl, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSource() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR, pdwcontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHelpInfo(::core::mem::transmute_copy(&pbstrfilename), ::core::mem::transmute_copy(&pdwcontext)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryErrorInfo: QueryErrorInfo::<Identity, Impl, OFFSET>,
            GetNext: GetNext::<Identity, Impl, OFFSET>,
            GetHresult: GetHresult::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDispatchEx_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetDispID(&mut self, bstrname: &super::super::Foundation::BSTR, grfdex: u32) -> ::windows::core::Result<i32>;
    fn InvokeEx(&mut self, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut super::Com::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: &::core::option::Option<super::Com::IServiceProvider>) -> ::windows::core::Result<()>;
    fn DeleteMemberByName(&mut self, bstrname: &super::super::Foundation::BSTR, grfdex: u32) -> ::windows::core::Result<()>;
    fn DeleteMemberByDispID(&mut self, id: i32) -> ::windows::core::Result<()>;
    fn GetMemberProperties(&mut self, id: i32, grfdexfetch: u32) -> ::windows::core::Result<u32>;
    fn GetMemberName(&mut self, id: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetNextDispID(&mut self, grfdex: u32, id: i32) -> ::windows::core::Result<i32>;
    fn GetNameSpaceParent(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDispatchEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchEx_Impl, const OFFSET: isize>() -> IDispatchEx_Vtbl {
        unsafe extern "system" fn GetDispID<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grfdex: u32, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDispID(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&grfdex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeEx<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut super::Com::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvokeEx(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdp), ::core::mem::transmute_copy(&pvarres), ::core::mem::transmute_copy(&pei), ::core::mem::transmute(&pspcaller)).into()
        }
        unsafe extern "system" fn DeleteMemberByName<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grfdex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteMemberByName(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&grfdex)).into()
        }
        unsafe extern "system" fn DeleteMemberByDispID<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteMemberByDispID(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetMemberProperties<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, grfdexfetch: u32, pgrfdex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMemberProperties(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&grfdexfetch)) {
                ::core::result::Result::Ok(ok__) => {
                    *pgrfdex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberName<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMemberName(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextDispID<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfdex: u32, id: i32, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNextDispID(::core::mem::transmute_copy(&grfdex), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameSpaceParent<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNameSpaceParent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDispID: GetDispID::<Identity, Impl, OFFSET>,
            InvokeEx: InvokeEx::<Identity, Impl, OFFSET>,
            DeleteMemberByName: DeleteMemberByName::<Identity, Impl, OFFSET>,
            DeleteMemberByDispID: DeleteMemberByDispID::<Identity, Impl, OFFSET>,
            GetMemberProperties: GetMemberProperties::<Identity, Impl, OFFSET>,
            GetMemberName: GetMemberName::<Identity, Impl, OFFSET>,
            GetNextDispID: GetNextDispID::<Identity, Impl, OFFSET>,
            GetNameSpaceParent: GetNameSpaceParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatchEx as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDropSource_Impl: Sized {
    fn QueryContinueDrag(&mut self, fescapepressed: super::super::Foundation::BOOL, grfkeystate: u32) -> ::windows::core::Result<()>;
    fn GiveFeedback(&mut self, dweffect: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDropSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropSource_Impl, const OFFSET: isize>() -> IDropSource_Vtbl {
        unsafe extern "system" fn QueryContinueDrag<Identity: ::windows::core::IUnknownImpl, Impl: IDropSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fescapepressed: super::super::Foundation::BOOL, grfkeystate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryContinueDrag(::core::mem::transmute_copy(&fescapepressed), ::core::mem::transmute_copy(&grfkeystate)).into()
        }
        unsafe extern "system" fn GiveFeedback<Identity: ::windows::core::IUnknownImpl, Impl: IDropSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweffect: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GiveFeedback(::core::mem::transmute_copy(&dweffect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryContinueDrag: QueryContinueDrag::<Identity, Impl, OFFSET>,
            GiveFeedback: GiveFeedback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDropSourceNotify_Impl: Sized {
    fn DragEnterTarget(&mut self, hwndtarget: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn DragLeaveTarget(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDropSourceNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropSourceNotify_Impl, const OFFSET: isize>() -> IDropSourceNotify_Vtbl {
        unsafe extern "system" fn DragEnterTarget<Identity: ::windows::core::IUnknownImpl, Impl: IDropSourceNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndtarget: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DragEnterTarget(::core::mem::transmute_copy(&hwndtarget)).into()
        }
        unsafe extern "system" fn DragLeaveTarget<Identity: ::windows::core::IUnknownImpl, Impl: IDropSourceNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DragLeaveTarget().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DragEnterTarget: DragEnterTarget::<Identity, Impl, OFFSET>,
            DragLeaveTarget: DragLeaveTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropSourceNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDropTarget_Impl: Sized {
    fn DragEnter(&mut self, pdataobj: &::core::option::Option<super::Com::IDataObject>, grfkeystate: u32, pt: &super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::Result<()>;
    fn DragOver(&mut self, grfkeystate: u32, pt: &super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::Result<()>;
    fn DragLeave(&mut self) -> ::windows::core::Result<()>;
    fn Drop(&mut self, pdataobj: &::core::option::Option<super::Com::IDataObject>, grfkeystate: u32, pt: &super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDropTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTarget_Impl, const OFFSET: isize>() -> IDropTarget_Vtbl {
        unsafe extern "system" fn DragEnter<Identity: ::windows::core::IUnknownImpl, Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobj: ::windows::core::RawPtr, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DragEnter(::core::mem::transmute(&pdataobj), ::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&pdweffect)).into()
        }
        unsafe extern "system" fn DragOver<Identity: ::windows::core::IUnknownImpl, Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DragOver(::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&pdweffect)).into()
        }
        unsafe extern "system" fn DragLeave<Identity: ::windows::core::IUnknownImpl, Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DragLeave().into()
        }
        unsafe extern "system" fn Drop<Identity: ::windows::core::IUnknownImpl, Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobj: ::windows::core::RawPtr, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Drop(::core::mem::transmute(&pdataobj), ::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&pdweffect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DragEnter: DragEnter::<Identity, Impl, OFFSET>,
            DragOver: DragOver::<Identity, Impl, OFFSET>,
            DragLeave: DragLeave::<Identity, Impl, OFFSET>,
            Drop: Drop::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnterpriseDropTarget_Impl: Sized {
    fn SetDropSourceEnterpriseId(&mut self, identity: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn IsEvaluatingEdpPolicy(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnterpriseDropTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseDropTarget_Impl, const OFFSET: isize>() -> IEnterpriseDropTarget_Vtbl {
        unsafe extern "system" fn SetDropSourceEnterpriseId<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDropSourceEnterpriseId(::core::mem::transmute_copy(&identity)).into()
        }
        unsafe extern "system" fn IsEvaluatingEdpPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEvaluatingEdpPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDropSourceEnterpriseId: SetDropSourceEnterpriseId::<Identity, Impl, OFFSET>,
            IsEvaluatingEdpPolicy: IsEvaluatingEdpPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnterpriseDropTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumOLEVERB_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumOLEVERB>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumOLEVERB_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOLEVERB_Impl, const OFFSET: isize>() -> IEnumOLEVERB_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumOLEVERB as ::windows::core::Interface>::IID
    }
}
pub trait IEnumOleDocumentViews_Impl: Sized {
    fn Next(&mut self, cviews: u32, rgpview: *mut ::core::option::Option<IOleDocumentView>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cviews: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumOleDocumentViews>;
}
impl IEnumOleDocumentViews_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>() -> IEnumOleDocumentViews_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cviews: u32, rgpview: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cviews), ::core::mem::transmute_copy(&rgpview), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cviews: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cviews)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumOleDocumentViews as ::windows::core::Interface>::IID
    }
}
pub trait IEnumOleUndoUnits_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IOleUndoUnit>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumOleUndoUnits>;
}
impl IEnumOleUndoUnits_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>() -> IEnumOleUndoUnits_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumOleUndoUnits as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IEnumVARIANT_Impl: Sized {
    fn Next(&mut self, celt: u32, rgvar: *mut super::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&mut self, celt: u32) -> ::windows::core::HRESULT;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IEnumVARIANT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVARIANT_Impl, const OFFSET: isize>() -> IEnumVARIANT_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumVARIANT as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IFont_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<super::Com::CY>;
    fn SetSize(&mut self, size: &super::Com::CY) -> ::windows::core::Result<()>;
    fn Bold(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetBold(&mut self, bold: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Italic(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetItalic(&mut self, italic: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Underline(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetUnderline(&mut self, underline: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Strikethrough(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetStrikethrough(&mut self, strikethrough: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Weight(&mut self) -> ::windows::core::Result<i16>;
    fn SetWeight(&mut self, weight: i16) -> ::windows::core::Result<()>;
    fn Charset(&mut self) -> ::windows::core::Result<i16>;
    fn SetCharset(&mut self, charset: i16) -> ::windows::core::Result<()>;
    fn hFont(&mut self) -> ::windows::core::Result<super::super::Graphics::Gdi::HFONT>;
    fn Clone(&mut self) -> ::windows::core::Result<IFont>;
    fn IsEqual(&mut self, pfontother: &::core::option::Option<IFont>) -> ::windows::core::Result<()>;
    fn SetRatio(&mut self, cylogical: i32, cyhimetric: i32) -> ::windows::core::Result<()>;
    fn QueryTextMetrics(&mut self) -> ::windows::core::Result<super::super::Graphics::Gdi::TEXTMETRICW>;
    fn AddRefHfont(&mut self, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::Result<()>;
    fn ReleaseHfont(&mut self, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::Result<()>;
    fn SetHdc(&mut self, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IFont_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>() -> IFont_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *psize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn Bold<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbold: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Bold() {
                ::core::result::Result::Ok(ok__) => {
                    *pbold = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bold: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBold(::core::mem::transmute_copy(&bold)).into()
        }
        unsafe extern "system" fn Italic<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitalic: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Italic() {
                ::core::result::Result::Ok(ok__) => {
                    *pitalic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, italic: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetItalic(::core::mem::transmute_copy(&italic)).into()
        }
        unsafe extern "system" fn Underline<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punderline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Underline() {
                ::core::result::Result::Ok(ok__) => {
                    *punderline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, underline: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUnderline(::core::mem::transmute_copy(&underline)).into()
        }
        unsafe extern "system" fn Strikethrough<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrikethrough: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Strikethrough() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrikethrough = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strikethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStrikethrough(::core::mem::transmute_copy(&strikethrough)).into()
        }
        unsafe extern "system" fn Weight<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweight: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *pweight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWeight(::core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn Charset<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcharset: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Charset() {
                ::core::result::Result::Ok(ok__) => {
                    *pcharset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharset<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, charset: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCharset(::core::mem::transmute_copy(&charset)).into()
        }
        unsafe extern "system" fn hFont<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phfont: *mut super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).hFont() {
                ::core::result::Result::Ok(ok__) => {
                    *phfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfontother: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsEqual(::core::mem::transmute(&pfontother)).into()
        }
        unsafe extern "system" fn SetRatio<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cylogical: i32, cyhimetric: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRatio(::core::mem::transmute_copy(&cylogical), ::core::mem::transmute_copy(&cyhimetric)).into()
        }
        unsafe extern "system" fn QueryTextMetrics<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryTextMetrics() {
                ::core::result::Result::Ok(ok__) => {
                    *ptm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRefHfont<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRefHfont(::core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn ReleaseHfont<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseHfont(::core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn SetHdc<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHdc(::core::mem::transmute_copy(&hdc)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            Bold: Bold::<Identity, Impl, OFFSET>,
            SetBold: SetBold::<Identity, Impl, OFFSET>,
            Italic: Italic::<Identity, Impl, OFFSET>,
            SetItalic: SetItalic::<Identity, Impl, OFFSET>,
            Underline: Underline::<Identity, Impl, OFFSET>,
            SetUnderline: SetUnderline::<Identity, Impl, OFFSET>,
            Strikethrough: Strikethrough::<Identity, Impl, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, Impl, OFFSET>,
            Weight: Weight::<Identity, Impl, OFFSET>,
            SetWeight: SetWeight::<Identity, Impl, OFFSET>,
            Charset: Charset::<Identity, Impl, OFFSET>,
            SetCharset: SetCharset::<Identity, Impl, OFFSET>,
            hFont: hFont::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            SetRatio: SetRatio::<Identity, Impl, OFFSET>,
            QueryTextMetrics: QueryTextMetrics::<Identity, Impl, OFFSET>,
            AddRefHfont: AddRefHfont::<Identity, Impl, OFFSET>,
            ReleaseHfont: ReleaseHfont::<Identity, Impl, OFFSET>,
            SetHdc: SetHdc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFont as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFontDisp_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IFontDisp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontDisp_Impl, const OFFSET: isize>() -> IFontDisp_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFontDisp as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFontEventsDisp_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IFontEventsDisp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontEventsDisp_Impl, const OFFSET: isize>() -> IFontEventsDisp_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFontEventsDisp as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IGetOleObject_Impl: Sized {
    fn GetOleObject(&mut self, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IGetOleObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetOleObject_Impl, const OFFSET: isize>() -> IGetOleObject_Vtbl {
        unsafe extern "system" fn GetOleObject<Identity: ::windows::core::IUnknownImpl, Impl: IGetOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOleObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetOleObject: GetOleObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetOleObject as ::windows::core::Interface>::IID
    }
}
pub trait IGetVBAObject_Impl: Sized {
    fn GetObject(&mut self, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl IGetVBAObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetVBAObject_Impl, const OFFSET: isize>() -> IGetVBAObject_Vtbl {
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl, Impl: IGetVBAObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetObject: GetObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetVBAObject as ::windows::core::Interface>::IID
    }
}
pub trait IObjectIdentity_Impl: Sized {
    fn IsEqualObject(&mut self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IObjectIdentity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectIdentity_Impl, const OFFSET: isize>() -> IObjectIdentity_Vtbl {
        unsafe extern "system" fn IsEqualObject<Identity: ::windows::core::IUnknownImpl, Impl: IObjectIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsEqualObject(::core::mem::transmute(&punk)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsEqualObject: IsEqualObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectIdentity as ::windows::core::Interface>::IID
    }
}
pub trait IObjectWithSite_Impl: Sized {
    fn SetSite(&mut self, punksite: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetSite(&mut self, riid: *const ::windows::core::GUID, ppvsite: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IObjectWithSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectWithSite_Impl, const OFFSET: isize>() -> IObjectWithSite_Vtbl {
        unsafe extern "system" fn SetSite<Identity: ::windows::core::IUnknownImpl, Impl: IObjectWithSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punksite: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSite(::core::mem::transmute(&punksite)).into()
        }
        unsafe extern "system" fn GetSite<Identity: ::windows::core::IUnknownImpl, Impl: IObjectWithSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvsite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSite(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvsite)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetSite: SetSite::<Identity, Impl, OFFSET>,
            GetSite: GetSite::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectWithSite as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleAdviseHolder_Impl: Sized {
    fn Advise(&mut self, padvise: &::core::option::Option<super::Com::IAdviseSink>) -> ::windows::core::Result<u32>;
    fn Unadvise(&mut self, dwconnection: u32) -> ::windows::core::Result<()>;
    fn EnumAdvise(&mut self) -> ::windows::core::Result<super::Com::IEnumSTATDATA>;
    fn SendOnRename(&mut self, pmk: &::core::option::Option<super::Com::IMoniker>) -> ::windows::core::Result<()>;
    fn SendOnSave(&mut self) -> ::windows::core::Result<()>;
    fn SendOnClose(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOleAdviseHolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>() -> IOleAdviseHolder_Vtbl {
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvise: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Advise(::core::mem::transmute(&padvise)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumAdvise<Identity: ::windows::core::IUnknownImpl, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumAdvise() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumadvise = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOnRename<Identity: ::windows::core::IUnknownImpl, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendOnRename(::core::mem::transmute(&pmk)).into()
        }
        unsafe extern "system" fn SendOnSave<Identity: ::windows::core::IUnknownImpl, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendOnSave().into()
        }
        unsafe extern "system" fn SendOnClose<Identity: ::windows::core::IUnknownImpl, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendOnClose().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, Impl, OFFSET>,
            SendOnRename: SendOnRename::<Identity, Impl, OFFSET>,
            SendOnSave: SendOnSave::<Identity, Impl, OFFSET>,
            SendOnClose: SendOnClose::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleAdviseHolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IOleCache_Impl: Sized {
    fn Cache(&mut self, pformatetc: *const super::Com::FORMATETC, advf: u32) -> ::windows::core::Result<u32>;
    fn Uncache(&mut self, dwconnection: u32) -> ::windows::core::Result<()>;
    fn EnumCache(&mut self) -> ::windows::core::Result<super::Com::IEnumSTATDATA>;
    fn InitCache(&mut self, pdataobject: &::core::option::Option<super::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn SetData(&mut self, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IOleCache_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache_Impl, const OFFSET: isize>() -> IOleCache_Vtbl {
        unsafe extern "system" fn Cache<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, advf: u32, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Cache(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&advf)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uncache<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Uncache(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumCache<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstatdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumCache() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstatdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitCache<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitCache(::core::mem::transmute(&pdataobject)).into()
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pmedium), ::core::mem::transmute_copy(&frelease)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Cache: Cache::<Identity, Impl, OFFSET>,
            Uncache: Uncache::<Identity, Impl, OFFSET>,
            EnumCache: EnumCache::<Identity, Impl, OFFSET>,
            InitCache: InitCache::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleCache as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IOleCache2_Impl: Sized + IOleCache_Impl {
    fn UpdateCache(&mut self, pdataobject: &::core::option::Option<super::Com::IDataObject>, grfupdf: UPDFCACHE_FLAGS, preserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DiscardCache(&mut self, dwdiscardoptions: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IOleCache2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache2_Impl, const OFFSET: isize>() -> IOleCache2_Vtbl {
        unsafe extern "system" fn UpdateCache<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, grfupdf: UPDFCACHE_FLAGS, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateCache(::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&grfupdf), ::core::mem::transmute_copy(&preserved)).into()
        }
        unsafe extern "system" fn DiscardCache<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdiscardoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardCache(::core::mem::transmute_copy(&dwdiscardoptions)).into()
        }
        Self {
            base: IOleCache_Vtbl::new::<Identity, Impl, OFFSET>(),
            UpdateCache: UpdateCache::<Identity, Impl, OFFSET>,
            DiscardCache: DiscardCache::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleCache2 as ::windows::core::Interface>::IID || iid == &<IOleCache as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleCacheControl_Impl: Sized {
    fn OnRun(&mut self, pdataobject: &::core::option::Option<super::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn OnStop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOleCacheControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCacheControl_Impl, const OFFSET: isize>() -> IOleCacheControl_Vtbl {
        unsafe extern "system" fn OnRun<Identity: ::windows::core::IUnknownImpl, Impl: IOleCacheControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRun(::core::mem::transmute(&pdataobject)).into()
        }
        unsafe extern "system" fn OnStop<Identity: ::windows::core::IUnknownImpl, Impl: IOleCacheControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStop().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnRun: OnRun::<Identity, Impl, OFFSET>, OnStop: OnStop::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleCacheControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleClientSite_Impl: Sized {
    fn SaveObject(&mut self) -> ::windows::core::Result<()>;
    fn GetMoniker(&mut self, dwassign: u32, dwwhichmoniker: u32) -> ::windows::core::Result<super::Com::IMoniker>;
    fn GetContainer(&mut self) -> ::windows::core::Result<IOleContainer>;
    fn ShowObject(&mut self) -> ::windows::core::Result<()>;
    fn OnShowWindow(&mut self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RequestNewObjectLayout(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleClientSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleClientSite_Impl, const OFFSET: isize>() -> IOleClientSite_Vtbl {
        unsafe extern "system" fn SaveObject<Identity: ::windows::core::IUnknownImpl, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveObject().into()
        }
        unsafe extern "system" fn GetMoniker<Identity: ::windows::core::IUnknownImpl, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMoniker(::core::mem::transmute_copy(&dwassign), ::core::mem::transmute_copy(&dwwhichmoniker)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Identity: ::windows::core::IUnknownImpl, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontainer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowObject<Identity: ::windows::core::IUnknownImpl, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowObject().into()
        }
        unsafe extern "system" fn OnShowWindow<Identity: ::windows::core::IUnknownImpl, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnShowWindow(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn RequestNewObjectLayout<Identity: ::windows::core::IUnknownImpl, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestNewObjectLayout().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SaveObject: SaveObject::<Identity, Impl, OFFSET>,
            GetMoniker: GetMoniker::<Identity, Impl, OFFSET>,
            GetContainer: GetContainer::<Identity, Impl, OFFSET>,
            ShowObject: ShowObject::<Identity, Impl, OFFSET>,
            OnShowWindow: OnShowWindow::<Identity, Impl, OFFSET>,
            RequestNewObjectLayout: RequestNewObjectLayout::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleClientSite as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleCommandTarget_Impl: Sized {
    fn QueryStatus(&mut self, pguidcmdgroup: *const ::windows::core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows::core::Result<()>;
    fn Exec(&mut self, pguidcmdgroup: *const ::windows::core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Com::VARIANT, pvaout: *mut super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleCommandTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCommandTarget_Impl, const OFFSET: isize>() -> IOleCommandTarget_Vtbl {
        unsafe extern "system" fn QueryStatus<Identity: ::windows::core::IUnknownImpl, Impl: IOleCommandTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows::core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryStatus(::core::mem::transmute_copy(&pguidcmdgroup), ::core::mem::transmute_copy(&ccmds), ::core::mem::transmute_copy(&prgcmds), ::core::mem::transmute_copy(&pcmdtext)).into()
        }
        unsafe extern "system" fn Exec<Identity: ::windows::core::IUnknownImpl, Impl: IOleCommandTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows::core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Com::VARIANT, pvaout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Exec(::core::mem::transmute_copy(&pguidcmdgroup), ::core::mem::transmute_copy(&ncmdid), ::core::mem::transmute_copy(&ncmdexecopt), ::core::mem::transmute_copy(&pvain), ::core::mem::transmute_copy(&pvaout)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
            Exec: Exec::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleCommandTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleContainer_Impl: Sized + IParseDisplayName_Impl {
    fn EnumObjects(&mut self, grfflags: u32) -> ::windows::core::Result<super::Com::IEnumUnknown>;
    fn LockContainer(&mut self, flock: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleContainer_Impl, const OFFSET: isize>() -> IOleContainer_Vtbl {
        unsafe extern "system" fn EnumObjects<Identity: ::windows::core::IUnknownImpl, Impl: IOleContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumObjects(::core::mem::transmute_copy(&grfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockContainer<Identity: ::windows::core::IUnknownImpl, Impl: IOleContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockContainer(::core::mem::transmute_copy(&flock)).into()
        }
        Self {
            base: IParseDisplayName_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            LockContainer: LockContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleContainer as ::windows::core::Interface>::IID || iid == &<IParseDisplayName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleControl_Impl: Sized {
    fn GetControlInfo(&mut self, pci: *mut CONTROLINFO) -> ::windows::core::Result<()>;
    fn OnMnemonic(&mut self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>;
    fn OnAmbientPropertyChange(&mut self, dispid: i32) -> ::windows::core::Result<()>;
    fn FreezeEvents(&mut self, bfreeze: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleControl_Impl, const OFFSET: isize>() -> IOleControl_Vtbl {
        unsafe extern "system" fn GetControlInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut CONTROLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetControlInfo(::core::mem::transmute_copy(&pci)).into()
        }
        unsafe extern "system" fn OnMnemonic<Identity: ::windows::core::IUnknownImpl, Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnMnemonic(::core::mem::transmute_copy(&pmsg)).into()
        }
        unsafe extern "system" fn OnAmbientPropertyChange<Identity: ::windows::core::IUnknownImpl, Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnAmbientPropertyChange(::core::mem::transmute_copy(&dispid)).into()
        }
        unsafe extern "system" fn FreezeEvents<Identity: ::windows::core::IUnknownImpl, Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfreeze: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreezeEvents(::core::mem::transmute_copy(&bfreeze)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetControlInfo: GetControlInfo::<Identity, Impl, OFFSET>,
            OnMnemonic: OnMnemonic::<Identity, Impl, OFFSET>,
            OnAmbientPropertyChange: OnAmbientPropertyChange::<Identity, Impl, OFFSET>,
            FreezeEvents: FreezeEvents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleControlSite_Impl: Sized {
    fn OnControlInfoChanged(&mut self) -> ::windows::core::Result<()>;
    fn LockInPlaceActive(&mut self, flock: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetExtendedControl(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransformCoords(&mut self, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: XFORMCOORDS) -> ::windows::core::Result<()>;
    fn TranslateAccelerator(&mut self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: u32) -> ::windows::core::Result<()>;
    fn OnFocus(&mut self, fgotfocus: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ShowPropertyFrame(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleControlSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlSite_Impl, const OFFSET: isize>() -> IOleControlSite_Vtbl {
        unsafe extern "system" fn OnControlInfoChanged<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnControlInfoChanged().into()
        }
        unsafe extern "system" fn LockInPlaceActive<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockInPlaceActive(::core::mem::transmute_copy(&flock)).into()
        }
        unsafe extern "system" fn GetExtendedControl<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetExtendedControl() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformCoords<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: XFORMCOORDS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TransformCoords(::core::mem::transmute_copy(&pptlhimetric), ::core::mem::transmute_copy(&pptfcontainer), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&grfmodifiers)).into()
        }
        unsafe extern "system" fn OnFocus<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgotfocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnFocus(::core::mem::transmute_copy(&fgotfocus)).into()
        }
        unsafe extern "system" fn ShowPropertyFrame<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowPropertyFrame().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnControlInfoChanged: OnControlInfoChanged::<Identity, Impl, OFFSET>,
            LockInPlaceActive: LockInPlaceActive::<Identity, Impl, OFFSET>,
            GetExtendedControl: GetExtendedControl::<Identity, Impl, OFFSET>,
            TransformCoords: TransformCoords::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
            OnFocus: OnFocus::<Identity, Impl, OFFSET>,
            ShowPropertyFrame: ShowPropertyFrame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleControlSite as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleDocument_Impl: Sized {
    fn CreateView(&mut self, pipsite: &::core::option::Option<IOleInPlaceSite>, pstm: &::core::option::Option<super::Com::IStream>, dwreserved: u32) -> ::windows::core::Result<IOleDocumentView>;
    fn GetDocMiscStatus(&mut self) -> ::windows::core::Result<u32>;
    fn EnumViews(&mut self, ppenum: *mut ::core::option::Option<IEnumOleDocumentViews>, ppview: *mut ::core::option::Option<IOleDocumentView>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOleDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocument_Impl, const OFFSET: isize>() -> IOleDocument_Vtbl {
        unsafe extern "system" fn CreateView<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsite: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, dwreserved: u32, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateView(::core::mem::transmute(&pipsite), ::core::mem::transmute(&pstm), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocMiscStatus<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDocMiscStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumViews<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumViews(::core::mem::transmute_copy(&ppenum), ::core::mem::transmute_copy(&ppview)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateView: CreateView::<Identity, Impl, OFFSET>,
            GetDocMiscStatus: GetDocMiscStatus::<Identity, Impl, OFFSET>,
            EnumViews: EnumViews::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleDocument as ::windows::core::Interface>::IID
    }
}
pub trait IOleDocumentSite_Impl: Sized {
    fn ActivateMe(&mut self, pviewtoactivate: &::core::option::Option<IOleDocumentView>) -> ::windows::core::Result<()>;
}
impl IOleDocumentSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentSite_Impl, const OFFSET: isize>() -> IOleDocumentSite_Vtbl {
        unsafe extern "system" fn ActivateMe<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewtoactivate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ActivateMe(::core::mem::transmute(&pviewtoactivate)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ActivateMe: ActivateMe::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleDocumentSite as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleDocumentView_Impl: Sized {
    fn SetInPlaceSite(&mut self, pipsite: &::core::option::Option<IOleInPlaceSite>) -> ::windows::core::Result<()>;
    fn GetInPlaceSite(&mut self) -> ::windows::core::Result<IOleInPlaceSite>;
    fn GetDocument(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetRect(&mut self, prcview: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetRect(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetRectComplex(&mut self, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn Show(&mut self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UIActivate(&mut self, fuiactivate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Open(&mut self) -> ::windows::core::Result<()>;
    fn CloseView(&mut self, dwreserved: u32) -> ::windows::core::Result<()>;
    fn SaveViewState(&mut self, pstm: &::core::option::Option<super::Com::IStream>) -> ::windows::core::Result<()>;
    fn ApplyViewState(&mut self, pstm: &::core::option::Option<super::Com::IStream>) -> ::windows::core::Result<()>;
    fn Clone(&mut self, pipsitenew: &::core::option::Option<IOleInPlaceSite>) -> ::windows::core::Result<IOleDocumentView>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleDocumentView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>() -> IOleDocumentView_Vtbl {
        unsafe extern "system" fn SetInPlaceSite<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInPlaceSite(::core::mem::transmute(&pipsite)).into()
        }
        unsafe extern "system" fn GetInPlaceSite<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInPlaceSite() {
                ::core::result::Result::Ok(ok__) => {
                    *ppipsite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRect<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRect(::core::mem::transmute_copy(&prcview)).into()
        }
        unsafe extern "system" fn GetRect<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRect() {
                ::core::result::Result::Ok(ok__) => {
                    *prcview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRectComplex<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRectComplex(::core::mem::transmute_copy(&prcview), ::core::mem::transmute_copy(&prchscroll), ::core::mem::transmute_copy(&prcvscroll), ::core::mem::transmute_copy(&prcsizebox)).into()
        }
        unsafe extern "system" fn Show<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn UIActivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuiactivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UIActivate(::core::mem::transmute_copy(&fuiactivate)).into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn CloseView<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseView(::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SaveViewState<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveViewState(::core::mem::transmute(&pstm)).into()
        }
        unsafe extern "system" fn ApplyViewState<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ApplyViewState(::core::mem::transmute(&pstm)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsitenew: ::windows::core::RawPtr, ppviewnew: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone(::core::mem::transmute(&pipsitenew)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppviewnew = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetInPlaceSite: SetInPlaceSite::<Identity, Impl, OFFSET>,
            GetInPlaceSite: GetInPlaceSite::<Identity, Impl, OFFSET>,
            GetDocument: GetDocument::<Identity, Impl, OFFSET>,
            SetRect: SetRect::<Identity, Impl, OFFSET>,
            GetRect: GetRect::<Identity, Impl, OFFSET>,
            SetRectComplex: SetRectComplex::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            UIActivate: UIActivate::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            CloseView: CloseView::<Identity, Impl, OFFSET>,
            SaveViewState: SaveViewState::<Identity, Impl, OFFSET>,
            ApplyViewState: ApplyViewState::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleDocumentView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceActiveObject_Impl: Sized + IOleWindow_Impl {
    fn TranslateAccelerator(&mut self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>;
    fn OnFrameWindowActivate(&mut self, factivate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnDocWindowActivate(&mut self, factivate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ResizeBorder(&mut self, prcborder: *const super::super::Foundation::RECT, puiwindow: &::core::option::Option<IOleInPlaceUIWindow>, fframewindow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnableModeless(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceActiveObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>() -> IOleInPlaceActiveObject_Vtbl {
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&lpmsg)).into()
        }
        unsafe extern "system" fn OnFrameWindowActivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnFrameWindowActivate(::core::mem::transmute_copy(&factivate)).into()
        }
        unsafe extern "system" fn OnDocWindowActivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDocWindowActivate(::core::mem::transmute_copy(&factivate)).into()
        }
        unsafe extern "system" fn ResizeBorder<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcborder: *const super::super::Foundation::RECT, puiwindow: ::windows::core::RawPtr, fframewindow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResizeBorder(::core::mem::transmute_copy(&prcborder), ::core::mem::transmute(&puiwindow), ::core::mem::transmute_copy(&fframewindow)).into()
        }
        unsafe extern "system" fn EnableModeless<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableModeless(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base: IOleWindow_Vtbl::new::<Identity, Impl, OFFSET>(),
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
            OnFrameWindowActivate: OnFrameWindowActivate::<Identity, Impl, OFFSET>,
            OnDocWindowActivate: OnDocWindowActivate::<Identity, Impl, OFFSET>,
            ResizeBorder: ResizeBorder::<Identity, Impl, OFFSET>,
            EnableModeless: EnableModeless::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceActiveObject as ::windows::core::Interface>::IID || iid == &<IOleWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceFrame_Impl: Sized + IOleWindow_Impl + IOleInPlaceUIWindow_Impl {
    fn InsertMenus(&mut self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> ::windows::core::Result<()>;
    fn SetMenu(&mut self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn RemoveMenus(&mut self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::Result<()>;
    fn SetStatusText(&mut self, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn EnableModeless(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn TranslateAccelerator(&mut self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>() -> IOleInPlaceFrame_Vtbl {
        unsafe extern "system" fn InsertMenus<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertMenus(::core::mem::transmute_copy(&hmenushared), ::core::mem::transmute_copy(&lpmenuwidths)).into()
        }
        unsafe extern "system" fn SetMenu<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMenu(::core::mem::transmute_copy(&hmenushared), ::core::mem::transmute_copy(&holemenu), ::core::mem::transmute_copy(&hwndactiveobject)).into()
        }
        unsafe extern "system" fn RemoveMenus<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveMenus(::core::mem::transmute_copy(&hmenushared)).into()
        }
        unsafe extern "system" fn SetStatusText<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStatusText(::core::mem::transmute_copy(&pszstatustext)).into()
        }
        unsafe extern "system" fn EnableModeless<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableModeless(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&lpmsg), ::core::mem::transmute_copy(&wid)).into()
        }
        Self {
            base: IOleInPlaceUIWindow_Vtbl::new::<Identity, Impl, OFFSET>(),
            InsertMenus: InsertMenus::<Identity, Impl, OFFSET>,
            SetMenu: SetMenu::<Identity, Impl, OFFSET>,
            RemoveMenus: RemoveMenus::<Identity, Impl, OFFSET>,
            SetStatusText: SetStatusText::<Identity, Impl, OFFSET>,
            EnableModeless: EnableModeless::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceFrame as ::windows::core::Interface>::IID || iid == &<IOleWindow as ::windows::core::Interface>::IID || iid == &<IOleInPlaceUIWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceObject_Impl: Sized + IOleWindow_Impl {
    fn InPlaceDeactivate(&mut self) -> ::windows::core::Result<()>;
    fn UIDeactivate(&mut self) -> ::windows::core::Result<()>;
    fn SetObjectRects(&mut self, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn ReactivateAndUndo(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleInPlaceObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObject_Impl, const OFFSET: isize>() -> IOleInPlaceObject_Vtbl {
        unsafe extern "system" fn InPlaceDeactivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InPlaceDeactivate().into()
        }
        unsafe extern "system" fn UIDeactivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UIDeactivate().into()
        }
        unsafe extern "system" fn SetObjectRects<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObjectRects(::core::mem::transmute_copy(&lprcposrect), ::core::mem::transmute_copy(&lprccliprect)).into()
        }
        unsafe extern "system" fn ReactivateAndUndo<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReactivateAndUndo().into()
        }
        Self {
            base: IOleWindow_Vtbl::new::<Identity, Impl, OFFSET>(),
            InPlaceDeactivate: InPlaceDeactivate::<Identity, Impl, OFFSET>,
            UIDeactivate: UIDeactivate::<Identity, Impl, OFFSET>,
            SetObjectRects: SetObjectRects::<Identity, Impl, OFFSET>,
            ReactivateAndUndo: ReactivateAndUndo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceObject as ::windows::core::Interface>::IID || iid == &<IOleWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceObjectWindowless_Impl: Sized + IOleWindow_Impl + IOleInPlaceObject_Impl {
    fn OnWindowMessage(&mut self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::LRESULT>;
    fn GetDropTarget(&mut self) -> ::windows::core::Result<IDropTarget>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleInPlaceObjectWindowless_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>() -> IOleInPlaceObjectWindowless_Vtbl {
        unsafe extern "system" fn OnWindowMessage<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OnWindowMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDropTarget<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdroptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDropTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdroptarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOleInPlaceObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnWindowMessage: OnWindowMessage::<Identity, Impl, OFFSET>,
            GetDropTarget: GetDropTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceObjectWindowless as ::windows::core::Interface>::IID || iid == &<IOleWindow as ::windows::core::Interface>::IID || iid == &<IOleInPlaceObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceSite_Impl: Sized + IOleWindow_Impl {
    fn CanInPlaceActivate(&mut self) -> ::windows::core::Result<()>;
    fn OnInPlaceActivate(&mut self) -> ::windows::core::Result<()>;
    fn OnUIActivate(&mut self) -> ::windows::core::Result<()>;
    fn GetWindowContext(&mut self, ppframe: *mut ::core::option::Option<IOleInPlaceFrame>, ppdoc: *mut ::core::option::Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows::core::Result<()>;
    fn Scroll(&mut self, scrollextant: &super::super::Foundation::SIZE) -> ::windows::core::Result<()>;
    fn OnUIDeactivate(&mut self, fundoable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnInPlaceDeactivate(&mut self) -> ::windows::core::Result<()>;
    fn DiscardUndoState(&mut self) -> ::windows::core::Result<()>;
    fn DeactivateAndUndo(&mut self) -> ::windows::core::Result<()>;
    fn OnPosRectChange(&mut self, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>() -> IOleInPlaceSite_Vtbl {
        unsafe extern "system" fn CanInPlaceActivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CanInPlaceActivate().into()
        }
        unsafe extern "system" fn OnInPlaceActivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInPlaceActivate().into()
        }
        unsafe extern "system" fn OnUIActivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnUIActivate().into()
        }
        unsafe extern "system" fn GetWindowContext<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppframe: *mut ::windows::core::RawPtr, ppdoc: *mut ::windows::core::RawPtr, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWindowContext(::core::mem::transmute_copy(&ppframe), ::core::mem::transmute_copy(&ppdoc), ::core::mem::transmute_copy(&lprcposrect), ::core::mem::transmute_copy(&lprccliprect), ::core::mem::transmute_copy(&lpframeinfo)).into()
        }
        unsafe extern "system" fn Scroll<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollextant: super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Scroll(::core::mem::transmute_copy(&scrollextant)).into()
        }
        unsafe extern "system" fn OnUIDeactivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fundoable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnUIDeactivate(::core::mem::transmute_copy(&fundoable)).into()
        }
        unsafe extern "system" fn OnInPlaceDeactivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInPlaceDeactivate().into()
        }
        unsafe extern "system" fn DiscardUndoState<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardUndoState().into()
        }
        unsafe extern "system" fn DeactivateAndUndo<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeactivateAndUndo().into()
        }
        unsafe extern "system" fn OnPosRectChange<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPosRectChange(::core::mem::transmute_copy(&lprcposrect)).into()
        }
        Self {
            base: IOleWindow_Vtbl::new::<Identity, Impl, OFFSET>(),
            CanInPlaceActivate: CanInPlaceActivate::<Identity, Impl, OFFSET>,
            OnInPlaceActivate: OnInPlaceActivate::<Identity, Impl, OFFSET>,
            OnUIActivate: OnUIActivate::<Identity, Impl, OFFSET>,
            GetWindowContext: GetWindowContext::<Identity, Impl, OFFSET>,
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            OnUIDeactivate: OnUIDeactivate::<Identity, Impl, OFFSET>,
            OnInPlaceDeactivate: OnInPlaceDeactivate::<Identity, Impl, OFFSET>,
            DiscardUndoState: DiscardUndoState::<Identity, Impl, OFFSET>,
            DeactivateAndUndo: DeactivateAndUndo::<Identity, Impl, OFFSET>,
            OnPosRectChange: OnPosRectChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceSite as ::windows::core::Interface>::IID || iid == &<IOleWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceSiteEx_Impl: Sized + IOleWindow_Impl + IOleInPlaceSite_Impl {
    fn OnInPlaceActivateEx(&mut self, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::Result<()>;
    fn OnInPlaceDeactivateEx(&mut self, fnoredraw: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RequestUIActivate(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceSiteEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: isize>() -> IOleInPlaceSiteEx_Vtbl {
        unsafe extern "system" fn OnInPlaceActivateEx<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInPlaceActivateEx(::core::mem::transmute_copy(&pfnoredraw), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnInPlaceDeactivateEx<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnoredraw: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInPlaceDeactivateEx(::core::mem::transmute_copy(&fnoredraw)).into()
        }
        unsafe extern "system" fn RequestUIActivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestUIActivate().into()
        }
        Self {
            base: IOleInPlaceSite_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnInPlaceActivateEx: OnInPlaceActivateEx::<Identity, Impl, OFFSET>,
            OnInPlaceDeactivateEx: OnInPlaceDeactivateEx::<Identity, Impl, OFFSET>,
            RequestUIActivate: RequestUIActivate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceSiteEx as ::windows::core::Interface>::IID || iid == &<IOleWindow as ::windows::core::Interface>::IID || iid == &<IOleInPlaceSite as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceSiteWindowless_Impl: Sized + IOleWindow_Impl + IOleInPlaceSite_Impl + IOleInPlaceSiteEx_Impl {
    fn CanWindowlessActivate(&mut self) -> ::windows::core::Result<()>;
    fn GetCapture(&mut self) -> ::windows::core::Result<()>;
    fn SetCapture(&mut self, fcapture: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFocus(&mut self) -> ::windows::core::Result<()>;
    fn SetFocus(&mut self, ffocus: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDC(&mut self, prect: *const super::super::Foundation::RECT, grfflags: u32) -> ::windows::core::Result<super::super::Graphics::Gdi::HDC>;
    fn ReleaseDC(&mut self, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::Result<()>;
    fn InvalidateRect(&mut self, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InvalidateRgn(&mut self, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ScrollRect(&mut self, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn AdjustRect(&mut self, prc: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn OnDefWindowMessage(&mut self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::LRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceSiteWindowless_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>() -> IOleInPlaceSiteWindowless_Vtbl {
        unsafe extern "system" fn CanWindowlessActivate<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CanWindowlessActivate().into()
        }
        unsafe extern "system" fn GetCapture<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCapture().into()
        }
        unsafe extern "system" fn SetCapture<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcapture: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCapture(::core::mem::transmute_copy(&fcapture)).into()
        }
        unsafe extern "system" fn GetFocus<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFocus().into()
        }
        unsafe extern "system" fn SetFocus<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFocus(::core::mem::transmute_copy(&ffocus)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, grfflags: u32, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDC(::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&grfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *phdc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&hdc)).into()
        }
        unsafe extern "system" fn InvalidateRect<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvalidateRect(::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&ferase)).into()
        }
        unsafe extern "system" fn InvalidateRgn<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvalidateRgn(::core::mem::transmute_copy(&hrgn), ::core::mem::transmute_copy(&ferase)).into()
        }
        unsafe extern "system" fn ScrollRect<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScrollRect(::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy), ::core::mem::transmute_copy(&prectscroll), ::core::mem::transmute_copy(&prectclip)).into()
        }
        unsafe extern "system" fn AdjustRect<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AdjustRect(::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn OnDefWindowMessage<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OnDefWindowMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOleInPlaceSiteEx_Vtbl::new::<Identity, Impl, OFFSET>(),
            CanWindowlessActivate: CanWindowlessActivate::<Identity, Impl, OFFSET>,
            GetCapture: GetCapture::<Identity, Impl, OFFSET>,
            SetCapture: SetCapture::<Identity, Impl, OFFSET>,
            GetFocus: GetFocus::<Identity, Impl, OFFSET>,
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            InvalidateRect: InvalidateRect::<Identity, Impl, OFFSET>,
            InvalidateRgn: InvalidateRgn::<Identity, Impl, OFFSET>,
            ScrollRect: ScrollRect::<Identity, Impl, OFFSET>,
            AdjustRect: AdjustRect::<Identity, Impl, OFFSET>,
            OnDefWindowMessage: OnDefWindowMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceSiteWindowless as ::windows::core::Interface>::IID || iid == &<IOleWindow as ::windows::core::Interface>::IID || iid == &<IOleInPlaceSite as ::windows::core::Interface>::IID || iid == &<IOleInPlaceSiteEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceUIWindow_Impl: Sized + IOleWindow_Impl {
    fn GetBorder(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn RequestBorderSpace(&mut self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetBorderSpace(&mut self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetActiveObject(&mut self, pactiveobject: &::core::option::Option<IOleInPlaceActiveObject>, pszobjname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleInPlaceUIWindow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>() -> IOleInPlaceUIWindow_Vtbl {
        unsafe extern "system" fn GetBorder<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprectborder: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBorder() {
                ::core::result::Result::Ok(ok__) => {
                    *lprectborder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestBorderSpace<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestBorderSpace(::core::mem::transmute_copy(&pborderwidths)).into()
        }
        unsafe extern "system" fn SetBorderSpace<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBorderSpace(::core::mem::transmute_copy(&pborderwidths)).into()
        }
        unsafe extern "system" fn SetActiveObject<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactiveobject: ::windows::core::RawPtr, pszobjname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActiveObject(::core::mem::transmute(&pactiveobject), ::core::mem::transmute_copy(&pszobjname)).into()
        }
        Self {
            base: IOleWindow_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetBorder: GetBorder::<Identity, Impl, OFFSET>,
            RequestBorderSpace: RequestBorderSpace::<Identity, Impl, OFFSET>,
            SetBorderSpace: SetBorderSpace::<Identity, Impl, OFFSET>,
            SetActiveObject: SetActiveObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceUIWindow as ::windows::core::Interface>::IID || iid == &<IOleWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleItemContainer_Impl: Sized + IParseDisplayName_Impl + IOleContainer_Impl {
    fn GetObject(&mut self, pszitem: super::super::Foundation::PWSTR, dwspeedneeded: u32, pbc: &::core::option::Option<super::Com::IBindCtx>, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetObjectStorage(&mut self, pszitem: super::super::Foundation::PWSTR, pbc: &::core::option::Option<super::Com::IBindCtx>, riid: *const ::windows::core::GUID, ppvstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsRunning(&mut self, pszitem: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleItemContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleItemContainer_Impl, const OFFSET: isize>() -> IOleItemContainer_Vtbl {
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl, Impl: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR, dwspeedneeded: u32, pbc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObject(::core::mem::transmute_copy(&pszitem), ::core::mem::transmute_copy(&dwspeedneeded), ::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn GetObjectStorage<Identity: ::windows::core::IUnknownImpl, Impl: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectStorage(::core::mem::transmute_copy(&pszitem), ::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvstorage)).into()
        }
        unsafe extern "system" fn IsRunning<Identity: ::windows::core::IUnknownImpl, Impl: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsRunning(::core::mem::transmute_copy(&pszitem)).into()
        }
        Self {
            base: IOleContainer_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            GetObjectStorage: GetObjectStorage::<Identity, Impl, OFFSET>,
            IsRunning: IsRunning::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleItemContainer as ::windows::core::Interface>::IID || iid == &<IParseDisplayName as ::windows::core::Interface>::IID || iid == &<IOleContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleLink_Impl: Sized {
    fn SetUpdateOptions(&mut self, dwupdateopt: u32) -> ::windows::core::Result<()>;
    fn GetUpdateOptions(&mut self) -> ::windows::core::Result<u32>;
    fn SetSourceMoniker(&mut self, pmk: &::core::option::Option<super::Com::IMoniker>, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetSourceMoniker(&mut self) -> ::windows::core::Result<super::Com::IMoniker>;
    fn SetSourceDisplayName(&mut self, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSourceDisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn BindToSource(&mut self, bindflags: u32, pbc: &::core::option::Option<super::Com::IBindCtx>) -> ::windows::core::Result<()>;
    fn BindIfRunning(&mut self) -> ::windows::core::Result<()>;
    fn GetBoundSource(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn UnbindSource(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self, pbc: &::core::option::Option<super::Com::IBindCtx>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleLink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>() -> IOleLink_Vtbl {
        unsafe extern "system" fn SetUpdateOptions<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUpdateOptions(::core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetUpdateOptions<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUpdateOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwupdateopt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceMoniker<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSourceMoniker(::core::mem::transmute(&pmk), ::core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn GetSourceMoniker<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceMoniker() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSourceDisplayName(::core::mem::transmute_copy(&pszstatustext)).into()
        }
        unsafe extern "system" fn GetSourceDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToSource<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindflags: u32, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindToSource(::core::mem::transmute_copy(&bindflags), ::core::mem::transmute(&pbc)).into()
        }
        unsafe extern "system" fn BindIfRunning<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindIfRunning().into()
        }
        unsafe extern "system" fn GetBoundSource<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBoundSource() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnbindSource<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnbindSource().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update(::core::mem::transmute(&pbc)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetUpdateOptions: SetUpdateOptions::<Identity, Impl, OFFSET>,
            GetUpdateOptions: GetUpdateOptions::<Identity, Impl, OFFSET>,
            SetSourceMoniker: SetSourceMoniker::<Identity, Impl, OFFSET>,
            GetSourceMoniker: GetSourceMoniker::<Identity, Impl, OFFSET>,
            SetSourceDisplayName: SetSourceDisplayName::<Identity, Impl, OFFSET>,
            GetSourceDisplayName: GetSourceDisplayName::<Identity, Impl, OFFSET>,
            BindToSource: BindToSource::<Identity, Impl, OFFSET>,
            BindIfRunning: BindIfRunning::<Identity, Impl, OFFSET>,
            GetBoundSource: GetBoundSource::<Identity, Impl, OFFSET>,
            UnbindSource: UnbindSource::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleLink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleObject_Impl: Sized {
    fn SetClientSite(&mut self, pclientsite: &::core::option::Option<IOleClientSite>) -> ::windows::core::Result<()>;
    fn GetClientSite(&mut self) -> ::windows::core::Result<IOleClientSite>;
    fn SetHostNames(&mut self, szcontainerapp: super::super::Foundation::PWSTR, szcontainerobj: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Close(&mut self, dwsaveoption: u32) -> ::windows::core::Result<()>;
    fn SetMoniker(&mut self, dwwhichmoniker: u32, pmk: &::core::option::Option<super::Com::IMoniker>) -> ::windows::core::Result<()>;
    fn GetMoniker(&mut self, dwassign: u32, dwwhichmoniker: u32) -> ::windows::core::Result<super::Com::IMoniker>;
    fn InitFromData(&mut self, pdataobject: &::core::option::Option<super::Com::IDataObject>, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetClipboardData(&mut self, dwreserved: u32) -> ::windows::core::Result<super::Com::IDataObject>;
    fn DoVerb(&mut self, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: &::core::option::Option<IOleClientSite>, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn EnumVerbs(&mut self) -> ::windows::core::Result<IEnumOLEVERB>;
    fn Update(&mut self) -> ::windows::core::Result<()>;
    fn IsUpToDate(&mut self) -> ::windows::core::Result<()>;
    fn GetUserClassID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetUserType(&mut self, dwformoftype: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetExtent(&mut self, dwdrawaspect: u32, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::Result<()>;
    fn GetExtent(&mut self, dwdrawaspect: u32) -> ::windows::core::Result<super::super::Foundation::SIZE>;
    fn Advise(&mut self, padvsink: &::core::option::Option<super::Com::IAdviseSink>) -> ::windows::core::Result<u32>;
    fn Unadvise(&mut self, dwconnection: u32) -> ::windows::core::Result<()>;
    fn EnumAdvise(&mut self) -> ::windows::core::Result<super::Com::IEnumSTATDATA>;
    fn GetMiscStatus(&mut self, dwaspect: u32) -> ::windows::core::Result<u32>;
    fn SetColorScheme(&mut self, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>() -> IOleObject_Vtbl {
        unsafe extern "system" fn SetClientSite<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientsite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientSite(::core::mem::transmute(&pclientsite)).into()
        }
        unsafe extern "system" fn GetClientSite<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclientsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClientSite() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientsite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostNames<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcontainerapp: super::super::Foundation::PWSTR, szcontainerobj: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHostNames(::core::mem::transmute_copy(&szcontainerapp), ::core::mem::transmute_copy(&szcontainerobj)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsaveoption: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close(::core::mem::transmute_copy(&dwsaveoption)).into()
        }
        unsafe extern "system" fn SetMoniker<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwhichmoniker: u32, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMoniker(::core::mem::transmute_copy(&dwwhichmoniker), ::core::mem::transmute(&pmk)).into()
        }
        unsafe extern "system" fn GetMoniker<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMoniker(::core::mem::transmute_copy(&dwassign), ::core::mem::transmute_copy(&dwwhichmoniker)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitFromData<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitFromData(::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&fcreation), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetClipboardData<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClipboardData(::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoVerb<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: ::windows::core::RawPtr, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DoVerb(::core::mem::transmute_copy(&iverb), ::core::mem::transmute_copy(&lpmsg), ::core::mem::transmute(&pactivesite), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lprcposrect)).into()
        }
        unsafe extern "system" fn EnumVerbs<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumoleverb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumVerbs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumoleverb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update().into()
        }
        unsafe extern "system" fn IsUpToDate<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsUpToDate().into()
        }
        unsafe extern "system" fn GetUserClassID<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUserClassID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserType<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwformoftype: u32, pszusertype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUserType(::core::mem::transmute_copy(&dwformoftype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszusertype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtent<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExtent(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&psizel)).into()
        }
        unsafe extern "system" fn GetExtent<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetExtent(::core::mem::transmute_copy(&dwdrawaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    *psizel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvsink: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Advise(::core::mem::transmute(&padvsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumAdvise<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumAdvise() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumadvise = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMiscStatus<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMiscStatus(::core::mem::transmute_copy(&dwaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorScheme<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorScheme(::core::mem::transmute_copy(&plogpal)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetClientSite: SetClientSite::<Identity, Impl, OFFSET>,
            GetClientSite: GetClientSite::<Identity, Impl, OFFSET>,
            SetHostNames: SetHostNames::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            SetMoniker: SetMoniker::<Identity, Impl, OFFSET>,
            GetMoniker: GetMoniker::<Identity, Impl, OFFSET>,
            InitFromData: InitFromData::<Identity, Impl, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, Impl, OFFSET>,
            DoVerb: DoVerb::<Identity, Impl, OFFSET>,
            EnumVerbs: EnumVerbs::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            IsUpToDate: IsUpToDate::<Identity, Impl, OFFSET>,
            GetUserClassID: GetUserClassID::<Identity, Impl, OFFSET>,
            GetUserType: GetUserType::<Identity, Impl, OFFSET>,
            SetExtent: SetExtent::<Identity, Impl, OFFSET>,
            GetExtent: GetExtent::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, Impl, OFFSET>,
            GetMiscStatus: GetMiscStatus::<Identity, Impl, OFFSET>,
            SetColorScheme: SetColorScheme::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleParentUndoUnit_Impl: Sized + IOleUndoUnit_Impl {
    fn Open(&mut self, ppuu: &::core::option::Option<IOleParentUndoUnit>) -> ::windows::core::Result<()>;
    fn Close(&mut self, ppuu: &::core::option::Option<IOleParentUndoUnit>, fcommit: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Add(&mut self, puu: &::core::option::Option<IOleUndoUnit>) -> ::windows::core::Result<()>;
    fn FindUnit(&mut self, puu: &::core::option::Option<IOleUndoUnit>) -> ::windows::core::Result<()>;
    fn GetParentState(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleParentUndoUnit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>() -> IOleParentUndoUnit_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute(&ppuu)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr, fcommit: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close(::core::mem::transmute(&ppuu), ::core::mem::transmute_copy(&fcommit)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn FindUnit<Identity: ::windows::core::IUnknownImpl, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindUnit(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn GetParentState<Identity: ::windows::core::IUnknownImpl, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParentState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOleUndoUnit_Vtbl::new::<Identity, Impl, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            FindUnit: FindUnit::<Identity, Impl, OFFSET>,
            GetParentState: GetParentState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleParentUndoUnit as ::windows::core::Interface>::IID || iid == &<IOleUndoUnit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkContainerA_Impl: Sized {
    fn GetNextLink(&mut self, dwlink: u32) -> u32;
    fn SetLinkUpdateOptions(&mut self, dwlink: u32, dwupdateopt: u32) -> ::windows::core::Result<()>;
    fn GetLinkUpdateOptions(&mut self, dwlink: u32) -> ::windows::core::Result<u32>;
    fn SetLinkSource(&mut self, dwlink: u32, lpszdisplayname: super::super::Foundation::PSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLinkSource(&mut self, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PSTR, lplpszshortlinktype: *mut super::super::Foundation::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OpenLinkSource(&mut self, dwlink: u32) -> ::windows::core::Result<()>;
    fn UpdateLink(&mut self, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CancelLink(&mut self, dwlink: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkContainerA_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>() -> IOleUILinkContainerA_Vtbl {
        unsafe extern "system" fn GetNextLink<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNextLink(::core::mem::transmute_copy(&dwlink))
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpdwupdateopt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkSource<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: super::super::Foundation::PSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lpszdisplayname), ::core::mem::transmute_copy(&lenfilename), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&fvalidatesource)).into()
        }
        unsafe extern "system" fn GetLinkSource<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PSTR, lplpszshortlinktype: *mut super::super::Foundation::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lplpszdisplayname), ::core::mem::transmute_copy(&lplenfilename), ::core::mem::transmute_copy(&lplpszfulllinktype), ::core::mem::transmute_copy(&lplpszshortlinktype), ::core::mem::transmute_copy(&lpfsourceavailable), ::core::mem::transmute_copy(&lpfisselected)).into()
        }
        unsafe extern "system" fn OpenLinkSource<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenLinkSource(::core::mem::transmute_copy(&dwlink)).into()
        }
        unsafe extern "system" fn UpdateLink<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateLink(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&ferrormessage), ::core::mem::transmute_copy(&freserved)).into()
        }
        unsafe extern "system" fn CancelLink<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelLink(::core::mem::transmute_copy(&dwlink)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetNextLink: GetNextLink::<Identity, Impl, OFFSET>,
            SetLinkUpdateOptions: SetLinkUpdateOptions::<Identity, Impl, OFFSET>,
            GetLinkUpdateOptions: GetLinkUpdateOptions::<Identity, Impl, OFFSET>,
            SetLinkSource: SetLinkSource::<Identity, Impl, OFFSET>,
            GetLinkSource: GetLinkSource::<Identity, Impl, OFFSET>,
            OpenLinkSource: OpenLinkSource::<Identity, Impl, OFFSET>,
            UpdateLink: UpdateLink::<Identity, Impl, OFFSET>,
            CancelLink: CancelLink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUILinkContainerA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkContainerW_Impl: Sized {
    fn GetNextLink(&mut self, dwlink: u32) -> u32;
    fn SetLinkUpdateOptions(&mut self, dwlink: u32, dwupdateopt: u32) -> ::windows::core::Result<()>;
    fn GetLinkUpdateOptions(&mut self, dwlink: u32) -> ::windows::core::Result<u32>;
    fn SetLinkSource(&mut self, dwlink: u32, lpszdisplayname: super::super::Foundation::PWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLinkSource(&mut self, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PWSTR, lplpszshortlinktype: *mut super::super::Foundation::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OpenLinkSource(&mut self, dwlink: u32) -> ::windows::core::Result<()>;
    fn UpdateLink(&mut self, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CancelLink(&mut self, dwlink: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkContainerW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>() -> IOleUILinkContainerW_Vtbl {
        unsafe extern "system" fn GetNextLink<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNextLink(::core::mem::transmute_copy(&dwlink))
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpdwupdateopt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkSource<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: super::super::Foundation::PWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lpszdisplayname), ::core::mem::transmute_copy(&lenfilename), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&fvalidatesource)).into()
        }
        unsafe extern "system" fn GetLinkSource<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PWSTR, lplpszshortlinktype: *mut super::super::Foundation::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lplpszdisplayname), ::core::mem::transmute_copy(&lplenfilename), ::core::mem::transmute_copy(&lplpszfulllinktype), ::core::mem::transmute_copy(&lplpszshortlinktype), ::core::mem::transmute_copy(&lpfsourceavailable), ::core::mem::transmute_copy(&lpfisselected)).into()
        }
        unsafe extern "system" fn OpenLinkSource<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenLinkSource(::core::mem::transmute_copy(&dwlink)).into()
        }
        unsafe extern "system" fn UpdateLink<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateLink(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&ferrormessage), ::core::mem::transmute_copy(&freserved)).into()
        }
        unsafe extern "system" fn CancelLink<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelLink(::core::mem::transmute_copy(&dwlink)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetNextLink: GetNextLink::<Identity, Impl, OFFSET>,
            SetLinkUpdateOptions: SetLinkUpdateOptions::<Identity, Impl, OFFSET>,
            GetLinkUpdateOptions: GetLinkUpdateOptions::<Identity, Impl, OFFSET>,
            SetLinkSource: SetLinkSource::<Identity, Impl, OFFSET>,
            GetLinkSource: GetLinkSource::<Identity, Impl, OFFSET>,
            OpenLinkSource: OpenLinkSource::<Identity, Impl, OFFSET>,
            UpdateLink: UpdateLink::<Identity, Impl, OFFSET>,
            CancelLink: CancelLink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUILinkContainerW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkInfoA_Impl: Sized + IOleUILinkContainerA_Impl {
    fn GetLastUpdate(&mut self, dwlink: u32) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkInfoA_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkInfoA_Impl, const OFFSET: isize>() -> IOleUILinkInfoA_Vtbl {
        unsafe extern "system" fn GetLastUpdate<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastUpdate(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    *lplastupdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IOleUILinkContainerA_Vtbl::new::<Identity, Impl, OFFSET>(), GetLastUpdate: GetLastUpdate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUILinkInfoA as ::windows::core::Interface>::IID || iid == &<IOleUILinkContainerA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkInfoW_Impl: Sized + IOleUILinkContainerW_Impl {
    fn GetLastUpdate(&mut self, dwlink: u32) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkInfoW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkInfoW_Impl, const OFFSET: isize>() -> IOleUILinkInfoW_Vtbl {
        unsafe extern "system" fn GetLastUpdate<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastUpdate(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    *lplastupdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IOleUILinkContainerW_Vtbl::new::<Identity, Impl, OFFSET>(), GetLastUpdate: GetLastUpdate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUILinkInfoW as ::windows::core::Interface>::IID || iid == &<IOleUILinkContainerW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUIObjInfoA_Impl: Sized {
    fn GetObjectInfo(&mut self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PSTR, lplpsztype: *mut super::super::Foundation::PSTR, lplpszshorttype: *mut super::super::Foundation::PSTR, lplpszlocation: *mut super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn GetConvertInfo(&mut self, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::Result<()>;
    fn ConvertObject(&mut self, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetViewInfo(&mut self, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::Result<()>;
    fn SetViewInfo(&mut self, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUIObjInfoA_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>() -> IOleUIObjInfoA_Vtbl {
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PSTR, lplpsztype: *mut super::super::Foundation::PSTR, lplpszshorttype: *mut super::super::Foundation::PSTR, lplpszlocation: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpdwobjsize), ::core::mem::transmute_copy(&lplpszlabel), ::core::mem::transmute_copy(&lplpsztype), ::core::mem::transmute_copy(&lplpszshorttype), ::core::mem::transmute_copy(&lplpszlocation)).into()
        }
        unsafe extern "system" fn GetConvertInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConvertInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpclassid), ::core::mem::transmute_copy(&lpwformat), ::core::mem::transmute_copy(&lpconvertdefaultclassid), ::core::mem::transmute_copy(&lplpclsidexclude), ::core::mem::transmute_copy(&lpcclsidexclude)).into()
        }
        unsafe extern "system" fn ConvertObject<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConvertObject(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&clsidnew)).into()
        }
        unsafe extern "system" fn GetViewInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&phmetapict), ::core::mem::transmute_copy(&pdvaspect), ::core::mem::transmute_copy(&pncurrentscale)).into()
        }
        unsafe extern "system" fn SetViewInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&hmetapict), ::core::mem::transmute_copy(&dvaspect), ::core::mem::transmute_copy(&ncurrentscale), ::core::mem::transmute_copy(&brelativetoorig)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetConvertInfo: GetConvertInfo::<Identity, Impl, OFFSET>,
            ConvertObject: ConvertObject::<Identity, Impl, OFFSET>,
            GetViewInfo: GetViewInfo::<Identity, Impl, OFFSET>,
            SetViewInfo: SetViewInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUIObjInfoA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUIObjInfoW_Impl: Sized {
    fn GetObjectInfo(&mut self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PWSTR, lplpsztype: *mut super::super::Foundation::PWSTR, lplpszshorttype: *mut super::super::Foundation::PWSTR, lplpszlocation: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetConvertInfo(&mut self, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::Result<()>;
    fn ConvertObject(&mut self, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetViewInfo(&mut self, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::Result<()>;
    fn SetViewInfo(&mut self, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUIObjInfoW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>() -> IOleUIObjInfoW_Vtbl {
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PWSTR, lplpsztype: *mut super::super::Foundation::PWSTR, lplpszshorttype: *mut super::super::Foundation::PWSTR, lplpszlocation: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpdwobjsize), ::core::mem::transmute_copy(&lplpszlabel), ::core::mem::transmute_copy(&lplpsztype), ::core::mem::transmute_copy(&lplpszshorttype), ::core::mem::transmute_copy(&lplpszlocation)).into()
        }
        unsafe extern "system" fn GetConvertInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConvertInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpclassid), ::core::mem::transmute_copy(&lpwformat), ::core::mem::transmute_copy(&lpconvertdefaultclassid), ::core::mem::transmute_copy(&lplpclsidexclude), ::core::mem::transmute_copy(&lpcclsidexclude)).into()
        }
        unsafe extern "system" fn ConvertObject<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConvertObject(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&clsidnew)).into()
        }
        unsafe extern "system" fn GetViewInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&phmetapict), ::core::mem::transmute_copy(&pdvaspect), ::core::mem::transmute_copy(&pncurrentscale)).into()
        }
        unsafe extern "system" fn SetViewInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&hmetapict), ::core::mem::transmute_copy(&dvaspect), ::core::mem::transmute_copy(&ncurrentscale), ::core::mem::transmute_copy(&brelativetoorig)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetConvertInfo: GetConvertInfo::<Identity, Impl, OFFSET>,
            ConvertObject: ConvertObject::<Identity, Impl, OFFSET>,
            GetViewInfo: GetViewInfo::<Identity, Impl, OFFSET>,
            SetViewInfo: SetViewInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUIObjInfoW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUndoManager_Impl: Sized {
    fn Open(&mut self, ppuu: &::core::option::Option<IOleParentUndoUnit>) -> ::windows::core::Result<()>;
    fn Close(&mut self, ppuu: &::core::option::Option<IOleParentUndoUnit>, fcommit: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Add(&mut self, puu: &::core::option::Option<IOleUndoUnit>) -> ::windows::core::Result<()>;
    fn GetOpenParentState(&mut self) -> ::windows::core::Result<u32>;
    fn DiscardFrom(&mut self, puu: &::core::option::Option<IOleUndoUnit>) -> ::windows::core::Result<()>;
    fn UndoTo(&mut self, puu: &::core::option::Option<IOleUndoUnit>) -> ::windows::core::Result<()>;
    fn RedoTo(&mut self, puu: &::core::option::Option<IOleUndoUnit>) -> ::windows::core::Result<()>;
    fn EnumUndoable(&mut self) -> ::windows::core::Result<IEnumOleUndoUnits>;
    fn EnumRedoable(&mut self) -> ::windows::core::Result<IEnumOleUndoUnits>;
    fn GetLastUndoDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetLastRedoDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Enable(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUndoManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>() -> IOleUndoManager_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute(&ppuu)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr, fcommit: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close(::core::mem::transmute(&ppuu), ::core::mem::transmute_copy(&fcommit)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn GetOpenParentState<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOpenParentState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardFrom<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardFrom(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn UndoTo<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UndoTo(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn RedoTo<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RedoTo(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn EnumUndoable<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumUndoable() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRedoable<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumRedoable() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastUndoDescription<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastUndoDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastRedoDescription<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastRedoDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Enable(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            GetOpenParentState: GetOpenParentState::<Identity, Impl, OFFSET>,
            DiscardFrom: DiscardFrom::<Identity, Impl, OFFSET>,
            UndoTo: UndoTo::<Identity, Impl, OFFSET>,
            RedoTo: RedoTo::<Identity, Impl, OFFSET>,
            EnumUndoable: EnumUndoable::<Identity, Impl, OFFSET>,
            EnumRedoable: EnumRedoable::<Identity, Impl, OFFSET>,
            GetLastUndoDescription: GetLastUndoDescription::<Identity, Impl, OFFSET>,
            GetLastRedoDescription: GetLastRedoDescription::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUndoManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUndoUnit_Impl: Sized {
    fn Do(&mut self, pundomanager: &::core::option::Option<IOleUndoManager>) -> ::windows::core::Result<()>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetUnitType(&mut self, pclsid: *mut ::windows::core::GUID, plid: *mut i32) -> ::windows::core::Result<()>;
    fn OnNextAdd(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUndoUnit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoUnit_Impl, const OFFSET: isize>() -> IOleUndoUnit_Vtbl {
        unsafe extern "system" fn Do<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pundomanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Do(::core::mem::transmute(&pundomanager)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnitType<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUnitType(::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn OnNextAdd<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnNextAdd().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Do: Do::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetUnitType: GetUnitType::<Identity, Impl, OFFSET>,
            OnNextAdd: OnNextAdd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUndoUnit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleWindow_Impl: Sized {
    fn GetWindow(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn ContextSensitiveHelp(&mut self, fentermode: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleWindow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleWindow_Impl, const OFFSET: isize>() -> IOleWindow_Vtbl {
        unsafe extern "system" fn GetWindow<Identity: ::windows::core::IUnknownImpl, Impl: IOleWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: ::windows::core::IUnknownImpl, Impl: IOleWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ContextSensitiveHelp(::core::mem::transmute_copy(&fentermode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IParseDisplayName_Impl: Sized {
    fn ParseDisplayName(&mut self, pbc: &::core::option::Option<super::Com::IBindCtx>, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<super::Com::IMoniker>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IParseDisplayName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IParseDisplayName_Impl, const OFFSET: isize>() -> IParseDisplayName_Vtbl {
        unsafe extern "system" fn ParseDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IParseDisplayName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ParseDisplayName(::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&pszdisplayname), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&ppmkout)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ParseDisplayName: ParseDisplayName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IParseDisplayName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPerPropertyBrowsing_Impl: Sized {
    fn GetDisplayString(&mut self, dispid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MapPropertyToPage(&mut self, dispid: i32) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetPredefinedStrings(&mut self, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows::core::Result<()>;
    fn GetPredefinedValue(&mut self, dispid: i32, dwcookie: u32) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPerPropertyBrowsing_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>() -> IPerPropertyBrowsing_Vtbl {
        unsafe extern "system" fn GetDisplayString<Identity: ::windows::core::IUnknownImpl, Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDisplayString(::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapPropertyToPage<Identity: ::windows::core::IUnknownImpl, Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MapPropertyToPage(::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPredefinedStrings<Identity: ::windows::core::IUnknownImpl, Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPredefinedStrings(::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&pcastringsout), ::core::mem::transmute_copy(&pcacookiesout)).into()
        }
        unsafe extern "system" fn GetPredefinedValue<Identity: ::windows::core::IUnknownImpl, Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, dwcookie: u32, pvarout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPredefinedValue(::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&dwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDisplayString: GetDisplayString::<Identity, Impl, OFFSET>,
            MapPropertyToPage: MapPropertyToPage::<Identity, Impl, OFFSET>,
            GetPredefinedStrings: GetPredefinedStrings::<Identity, Impl, OFFSET>,
            GetPredefinedValue: GetPredefinedValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerPropertyBrowsing as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPersistPropertyBag_Impl: Sized + super::Com::IPersist_Impl {
    fn InitNew(&mut self) -> ::windows::core::Result<()>;
    fn Load(&mut self, ppropbag: &::core::option::Option<super::Com::StructuredStorage::IPropertyBag>, perrorlog: &::core::option::Option<super::Com::IErrorLog>) -> ::windows::core::Result<()>;
    fn Save(&mut self, ppropbag: &::core::option::Option<super::Com::StructuredStorage::IPropertyBag>, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPersistPropertyBag_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag_Impl, const OFFSET: isize>() -> IPersistPropertyBag_Vtbl {
        unsafe extern "system" fn InitNew<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitNew().into()
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, perrorlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Load(::core::mem::transmute(&ppropbag), ::core::mem::transmute(&perrorlog)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Save(::core::mem::transmute(&ppropbag), ::core::mem::transmute_copy(&fcleardirty), ::core::mem::transmute_copy(&fsaveallproperties)).into()
        }
        Self {
            base: super::Com::IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistPropertyBag as ::windows::core::Interface>::IID || iid == &<super::Com::IPersist as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPersistPropertyBag2_Impl: Sized + super::Com::IPersist_Impl {
    fn InitNew(&mut self) -> ::windows::core::Result<()>;
    fn Load(&mut self, ppropbag: &::core::option::Option<super::Com::StructuredStorage::IPropertyBag2>, perrlog: &::core::option::Option<super::Com::IErrorLog>) -> ::windows::core::Result<()>;
    fn Save(&mut self, ppropbag: &::core::option::Option<super::Com::StructuredStorage::IPropertyBag2>, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsDirty(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPersistPropertyBag2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>() -> IPersistPropertyBag2_Vtbl {
        unsafe extern "system" fn InitNew<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitNew().into()
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, perrlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Load(::core::mem::transmute(&ppropbag), ::core::mem::transmute(&perrlog)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Save(::core::mem::transmute(&ppropbag), ::core::mem::transmute_copy(&fcleardirty), ::core::mem::transmute_copy(&fsaveallproperties)).into()
        }
        unsafe extern "system" fn IsDirty<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsDirty().into()
        }
        Self {
            base: super::Com::IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistPropertyBag2 as ::windows::core::Interface>::IID || iid == &<super::Com::IPersist as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPicture_Impl: Sized {
    fn Handle(&mut self) -> ::windows::core::Result<u32>;
    fn hPal(&mut self) -> ::windows::core::Result<u32>;
    fn Type(&mut self) -> ::windows::core::Result<i16>;
    fn Width(&mut self) -> ::windows::core::Result<i32>;
    fn Height(&mut self) -> ::windows::core::Result<i32>;
    fn Render(&mut self, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn set_hPal(&mut self, hpal: u32) -> ::windows::core::Result<()>;
    fn CurDC(&mut self) -> ::windows::core::Result<super::super::Graphics::Gdi::HDC>;
    fn SelectPicture(&mut self, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut u32) -> ::windows::core::Result<()>;
    fn KeepOriginalFormat(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetKeepOriginalFormat(&mut self, keep: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn PictureChanged(&mut self) -> ::windows::core::Result<()>;
    fn SaveAsFile(&mut self, pstream: &::core::option::Option<super::Com::IStream>, fsavememcopy: super::super::Foundation::BOOL) -> ::windows::core::Result<i32>;
    fn Attributes(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPicture_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>() -> IPicture_Vtbl {
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hPal<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phpal: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).hPal() {
                ::core::result::Result::Ok(ok__) => {
                    *phpal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *pwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *pheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Render<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Render(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy), ::core::mem::transmute_copy(&xsrc), ::core::mem::transmute_copy(&ysrc), ::core::mem::transmute_copy(&cxsrc), ::core::mem::transmute_copy(&cysrc), ::core::mem::transmute_copy(&prcwbounds)).into()
        }
        unsafe extern "system" fn set_hPal<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpal: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_hPal(::core::mem::transmute_copy(&hpal)).into()
        }
        unsafe extern "system" fn CurDC<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurDC() {
                ::core::result::Result::Ok(ok__) => {
                    *phdc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPicture<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SelectPicture(::core::mem::transmute_copy(&hdcin), ::core::mem::transmute_copy(&phdcout), ::core::mem::transmute_copy(&phbmpout)).into()
        }
        unsafe extern "system" fn KeepOriginalFormat<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).KeepOriginalFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pkeep = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKeepOriginalFormat(::core::mem::transmute_copy(&keep)).into()
        }
        unsafe extern "system" fn PictureChanged<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PictureChanged().into()
        }
        unsafe extern "system" fn SaveAsFile<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SaveAsFile(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&fsavememcopy)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwattr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Handle: Handle::<Identity, Impl, OFFSET>,
            hPal: hPal::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            Render: Render::<Identity, Impl, OFFSET>,
            set_hPal: set_hPal::<Identity, Impl, OFFSET>,
            CurDC: CurDC::<Identity, Impl, OFFSET>,
            SelectPicture: SelectPicture::<Identity, Impl, OFFSET>,
            KeepOriginalFormat: KeepOriginalFormat::<Identity, Impl, OFFSET>,
            SetKeepOriginalFormat: SetKeepOriginalFormat::<Identity, Impl, OFFSET>,
            PictureChanged: PictureChanged::<Identity, Impl, OFFSET>,
            SaveAsFile: SaveAsFile::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPicture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPicture2_Impl: Sized {
    fn Handle(&mut self) -> ::windows::core::Result<usize>;
    fn hPal(&mut self) -> ::windows::core::Result<usize>;
    fn Type(&mut self) -> ::windows::core::Result<i16>;
    fn Width(&mut self) -> ::windows::core::Result<i32>;
    fn Height(&mut self) -> ::windows::core::Result<i32>;
    fn Render(&mut self, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn set_hPal(&mut self, hpal: usize) -> ::windows::core::Result<()>;
    fn CurDC(&mut self) -> ::windows::core::Result<super::super::Graphics::Gdi::HDC>;
    fn SelectPicture(&mut self, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows::core::Result<()>;
    fn KeepOriginalFormat(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetKeepOriginalFormat(&mut self, keep: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn PictureChanged(&mut self) -> ::windows::core::Result<()>;
    fn SaveAsFile(&mut self, pstream: &::core::option::Option<super::Com::IStream>, fsavememcopy: super::super::Foundation::BOOL) -> ::windows::core::Result<i32>;
    fn Attributes(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPicture2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>() -> IPicture2_Vtbl {
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hPal<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phpal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).hPal() {
                ::core::result::Result::Ok(ok__) => {
                    *phpal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *pwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *pheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Render<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Render(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy), ::core::mem::transmute_copy(&xsrc), ::core::mem::transmute_copy(&ysrc), ::core::mem::transmute_copy(&cxsrc), ::core::mem::transmute_copy(&cysrc), ::core::mem::transmute_copy(&prcwbounds)).into()
        }
        unsafe extern "system" fn set_hPal<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpal: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_hPal(::core::mem::transmute_copy(&hpal)).into()
        }
        unsafe extern "system" fn CurDC<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurDC() {
                ::core::result::Result::Ok(ok__) => {
                    *phdc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPicture<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SelectPicture(::core::mem::transmute_copy(&hdcin), ::core::mem::transmute_copy(&phdcout), ::core::mem::transmute_copy(&phbmpout)).into()
        }
        unsafe extern "system" fn KeepOriginalFormat<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).KeepOriginalFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pkeep = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKeepOriginalFormat(::core::mem::transmute_copy(&keep)).into()
        }
        unsafe extern "system" fn PictureChanged<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PictureChanged().into()
        }
        unsafe extern "system" fn SaveAsFile<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SaveAsFile(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&fsavememcopy)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwattr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Handle: Handle::<Identity, Impl, OFFSET>,
            hPal: hPal::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            Render: Render::<Identity, Impl, OFFSET>,
            set_hPal: set_hPal::<Identity, Impl, OFFSET>,
            CurDC: CurDC::<Identity, Impl, OFFSET>,
            SelectPicture: SelectPicture::<Identity, Impl, OFFSET>,
            KeepOriginalFormat: KeepOriginalFormat::<Identity, Impl, OFFSET>,
            SetKeepOriginalFormat: SetKeepOriginalFormat::<Identity, Impl, OFFSET>,
            PictureChanged: PictureChanged::<Identity, Impl, OFFSET>,
            SaveAsFile: SaveAsFile::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPicture2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPictureDisp_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPictureDisp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPictureDisp_Impl, const OFFSET: isize>() -> IPictureDisp_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPictureDisp as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPointerInactive_Impl: Sized {
    fn GetActivationPolicy(&mut self) -> ::windows::core::Result<u32>;
    fn OnInactiveMouseMove(&mut self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows::core::Result<()>;
    fn OnInactiveSetCursor(&mut self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPointerInactive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerInactive_Impl, const OFFSET: isize>() -> IPointerInactive_Vtbl {
        unsafe extern "system" fn GetActivationPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IPointerInactive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpolicy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActivationPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwpolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInactiveMouseMove<Identity: ::windows::core::IUnknownImpl, Impl: IPointerInactive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInactiveMouseMove(::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&grfkeystate)).into()
        }
        unsafe extern "system" fn OnInactiveSetCursor<Identity: ::windows::core::IUnknownImpl, Impl: IPointerInactive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInactiveSetCursor(::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&dwmousemsg), ::core::mem::transmute_copy(&fsetalways)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetActivationPolicy: GetActivationPolicy::<Identity, Impl, OFFSET>,
            OnInactiveMouseMove: OnInactiveMouseMove::<Identity, Impl, OFFSET>,
            OnInactiveSetCursor: OnInactiveSetCursor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerInactive as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPrint_Impl: Sized {
    fn SetInitialPageNum(&mut self, nfirstpage: i32) -> ::windows::core::Result<()>;
    fn GetPageInfo(&mut self, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows::core::Result<()>;
    fn Print(&mut self, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: &::core::option::Option<IContinueCallback>, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPrint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint_Impl, const OFFSET: isize>() -> IPrint_Vtbl {
        unsafe extern "system" fn SetInitialPageNum<Identity: ::windows::core::IUnknownImpl, Impl: IPrint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nfirstpage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialPageNum(::core::mem::transmute_copy(&nfirstpage)).into()
        }
        unsafe extern "system" fn GetPageInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPrint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPageInfo(::core::mem::transmute_copy(&pnfirstpage), ::core::mem::transmute_copy(&pcpages)).into()
        }
        unsafe extern "system" fn Print<Identity: ::windows::core::IUnknownImpl, Impl: IPrint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: ::windows::core::RawPtr, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Print(::core::mem::transmute_copy(&grfflags), ::core::mem::transmute_copy(&pptd), ::core::mem::transmute_copy(&pppageset), ::core::mem::transmute_copy(&pstgmoptions), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&nfirstpage), ::core::mem::transmute_copy(&pcpagesprinted), ::core::mem::transmute_copy(&pnlastpage)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetInitialPageNum: SetInitialPageNum::<Identity, Impl, OFFSET>,
            GetPageInfo: GetPageInfo::<Identity, Impl, OFFSET>,
            Print: Print::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint as ::windows::core::Interface>::IID
    }
}
pub trait IPropertyNotifySink_Impl: Sized {
    fn OnChanged(&mut self, dispid: i32) -> ::windows::core::Result<()>;
    fn OnRequestEdit(&mut self, dispid: i32) -> ::windows::core::Result<()>;
}
impl IPropertyNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyNotifySink_Impl, const OFFSET: isize>() -> IPropertyNotifySink_Vtbl {
        unsafe extern "system" fn OnChanged<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnChanged(::core::mem::transmute_copy(&dispid)).into()
        }
        unsafe extern "system" fn OnRequestEdit<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRequestEdit(::core::mem::transmute_copy(&dispid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnChanged: OnChanged::<Identity, Impl, OFFSET>,
            OnRequestEdit: OnRequestEdit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPropertyPage_Impl: Sized {
    fn SetPageSite(&mut self, ppagesite: &::core::option::Option<IPropertyPageSite>) -> ::windows::core::Result<()>;
    fn Activate(&mut self, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Deactivate(&mut self) -> ::windows::core::Result<()>;
    fn GetPageInfo(&mut self) -> ::windows::core::Result<PROPPAGEINFO>;
    fn SetObjects(&mut self, cobjects: u32, ppunk: *const ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Show(&mut self, ncmdshow: u32) -> ::windows::core::Result<()>;
    fn Move(&mut self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn IsPageDirty(&mut self) -> ::windows::core::Result<()>;
    fn Apply(&mut self) -> ::windows::core::Result<()>;
    fn Help(&mut self, pszhelpdir: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn TranslateAccelerator(&mut self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPropertyPage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>() -> IPropertyPage_Vtbl {
        unsafe extern "system" fn SetPageSite<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagesite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPageSite(::core::mem::transmute(&ppagesite)).into()
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&bmodal)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Deactivate().into()
        }
        unsafe extern "system" fn GetPageInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppageinfo: *mut PROPPAGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPageInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppageinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjects<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32, ppunk: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObjects(::core::mem::transmute_copy(&cobjects), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn Show<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncmdshow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&ncmdshow)).into()
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn IsPageDirty<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsPageDirty().into()
        }
        unsafe extern "system" fn Apply<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Apply().into()
        }
        unsafe extern "system" fn Help<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelpdir: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Help(::core::mem::transmute_copy(&pszhelpdir)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&pmsg)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetPageSite: SetPageSite::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            GetPageInfo: GetPageInfo::<Identity, Impl, OFFSET>,
            SetObjects: SetObjects::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            IsPageDirty: IsPageDirty::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            Help: Help::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyPage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPropertyPage2_Impl: Sized + IPropertyPage_Impl {
    fn EditProperty(&mut self, dispid: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPropertyPage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage2_Impl, const OFFSET: isize>() -> IPropertyPage2_Vtbl {
        unsafe extern "system" fn EditProperty<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EditProperty(::core::mem::transmute_copy(&dispid)).into()
        }
        Self { base: IPropertyPage_Vtbl::new::<Identity, Impl, OFFSET>(), EditProperty: EditProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyPage2 as ::windows::core::Interface>::IID || iid == &<IPropertyPage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPropertyPageSite_Impl: Sized {
    fn OnStatusChange(&mut self, dwflags: PROPPAGESTATUS) -> ::windows::core::Result<()>;
    fn GetLocaleID(&mut self) -> ::windows::core::Result<u32>;
    fn GetPageContainer(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn TranslateAccelerator(&mut self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPropertyPageSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPageSite_Impl, const OFFSET: isize>() -> IPropertyPageSite_Vtbl {
        unsafe extern "system" fn OnStatusChange<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: PROPPAGESTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStatusChange(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetLocaleID<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocaleid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLocaleID() {
                ::core::result::Result::Ok(ok__) => {
                    *plocaleid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageContainer<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPageContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&pmsg)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            GetLocaleID: GetLocaleID::<Identity, Impl, OFFSET>,
            GetPageContainer: GetPageContainer::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyPageSite as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProtectFocus_Impl: Sized {
    fn AllowFocusChange(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IProtectFocus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectFocus_Impl, const OFFSET: isize>() -> IProtectFocus_Vtbl {
        unsafe extern "system" fn AllowFocusChange<Identity: ::windows::core::IUnknownImpl, Impl: IProtectFocus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowFocusChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pfallow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AllowFocusChange: AllowFocusChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtectFocus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IProtectedModeMenuServices_Impl: Sized {
    fn CreateMenu(&mut self) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
    fn LoadMenu(&mut self, pszmodulename: super::super::Foundation::PWSTR, pszmenuname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
    fn LoadMenuID(&mut self, pszmodulename: super::super::Foundation::PWSTR, wresourceid: u16) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IProtectedModeMenuServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedModeMenuServices_Impl, const OFFSET: isize>() -> IProtectedModeMenuServices_Vtbl {
        unsafe extern "system" fn CreateMenu<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMenu() {
                ::core::result::Result::Ok(ok__) => {
                    *phmenu = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMenu<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmodulename: super::super::Foundation::PWSTR, pszmenuname: super::super::Foundation::PWSTR, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadMenu(::core::mem::transmute_copy(&pszmodulename), ::core::mem::transmute_copy(&pszmenuname)) {
                ::core::result::Result::Ok(ok__) => {
                    *phmenu = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMenuID<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmodulename: super::super::Foundation::PWSTR, wresourceid: u16, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadMenuID(::core::mem::transmute_copy(&pszmodulename), ::core::mem::transmute_copy(&wresourceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *phmenu = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateMenu: CreateMenu::<Identity, Impl, OFFSET>,
            LoadMenu: LoadMenu::<Identity, Impl, OFFSET>,
            LoadMenuID: LoadMenuID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtectedModeMenuServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideClassInfo_Impl: Sized {
    fn GetClassInfo(&mut self) -> ::windows::core::Result<super::Com::ITypeInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl IProvideClassInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideClassInfo_Impl, const OFFSET: isize>() -> IProvideClassInfo_Vtbl {
        unsafe extern "system" fn GetClassInfo<Identity: ::windows::core::IUnknownImpl, Impl: IProvideClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppti: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClassInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppti = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetClassInfo: GetClassInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideClassInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideClassInfo2_Impl: Sized + IProvideClassInfo_Impl {
    fn GetGUID(&mut self, dwguidkind: u32) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "Win32_System_Com")]
impl IProvideClassInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideClassInfo2_Impl, const OFFSET: isize>() -> IProvideClassInfo2_Vtbl {
        unsafe extern "system" fn GetGUID<Identity: ::windows::core::IUnknownImpl, Impl: IProvideClassInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwguidkind: u32, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGUID(::core::mem::transmute_copy(&dwguidkind)) {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IProvideClassInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetGUID: GetGUID::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideClassInfo2 as ::windows::core::Interface>::IID || iid == &<IProvideClassInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideMultipleClassInfo_Impl: Sized + IProvideClassInfo_Impl + IProvideClassInfo2_Impl {
    fn GetMultiTypeInfoCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetInfoOfIndex(&mut self, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut ::core::option::Option<super::Com::ITypeInfo>, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows::core::GUID, piidsource: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IProvideMultipleClassInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideMultipleClassInfo_Impl, const OFFSET: isize>() -> IProvideMultipleClassInfo_Vtbl {
        unsafe extern "system" fn GetMultiTypeInfoCount<Identity: ::windows::core::IUnknownImpl, Impl: IProvideMultipleClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcti: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMultiTypeInfoCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcti = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfoOfIndex<Identity: ::windows::core::IUnknownImpl, Impl: IProvideMultipleClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut ::windows::core::RawPtr, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows::core::GUID, piidsource: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInfoOfIndex(::core::mem::transmute_copy(&iti), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppticoclass), ::core::mem::transmute_copy(&pdwtiflags), ::core::mem::transmute_copy(&pcdispidreserved), ::core::mem::transmute_copy(&piidprimary), ::core::mem::transmute_copy(&piidsource)).into()
        }
        Self {
            base: IProvideClassInfo2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMultiTypeInfoCount: GetMultiTypeInfoCount::<Identity, Impl, OFFSET>,
            GetInfoOfIndex: GetInfoOfIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideMultipleClassInfo as ::windows::core::Interface>::IID || iid == &<IProvideClassInfo as ::windows::core::Interface>::IID || iid == &<IProvideClassInfo2 as ::windows::core::Interface>::IID
    }
}
pub trait IProvideRuntimeContext_Impl: Sized {
    fn GetCurrentSourceContext(&mut self, pdwcontext: *mut usize, pfexecutingglobalcode: *mut i16) -> ::windows::core::Result<()>;
}
impl IProvideRuntimeContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideRuntimeContext_Impl, const OFFSET: isize>() -> IProvideRuntimeContext_Vtbl {
        unsafe extern "system" fn GetCurrentSourceContext<Identity: ::windows::core::IUnknownImpl, Impl: IProvideRuntimeContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcontext: *mut usize, pfexecutingglobalcode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentSourceContext(::core::mem::transmute_copy(&pdwcontext), ::core::mem::transmute_copy(&pfexecutingglobalcode)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetCurrentSourceContext: GetCurrentSourceContext::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideRuntimeContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IQuickActivate_Impl: Sized {
    fn QuickActivate(&mut self, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows::core::Result<()>;
    fn SetContentExtent(&mut self, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::Result<()>;
    fn GetContentExtent(&mut self) -> ::windows::core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IQuickActivate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuickActivate_Impl, const OFFSET: isize>() -> IQuickActivate_Vtbl {
        unsafe extern "system" fn QuickActivate<Identity: ::windows::core::IUnknownImpl, Impl: IQuickActivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QuickActivate(::core::mem::transmute_copy(&pqacontainer), ::core::mem::transmute_copy(&pqacontrol)).into()
        }
        unsafe extern "system" fn SetContentExtent<Identity: ::windows::core::IUnknownImpl, Impl: IQuickActivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContentExtent(::core::mem::transmute_copy(&psizel)).into()
        }
        unsafe extern "system" fn GetContentExtent<Identity: ::windows::core::IUnknownImpl, Impl: IQuickActivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContentExtent() {
                ::core::result::Result::Ok(ok__) => {
                    *psizel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QuickActivate: QuickActivate::<Identity, Impl, OFFSET>,
            SetContentExtent: SetContentExtent::<Identity, Impl, OFFSET>,
            GetContentExtent: GetContentExtent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuickActivate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRecordInfo_Impl: Sized {
    fn RecordInit(&mut self, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RecordClear(&mut self, pvexisting: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RecordCopy(&mut self, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetGuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetTypeInfo(&mut self) -> ::windows::core::Result<super::Com::ITypeInfo>;
    fn GetField(&mut self, pvdata: *const ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetFieldNoCopy(&mut self, pvdata: *const ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *mut super::Com::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn PutField(&mut self, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PutFieldNoCopy(&mut self, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetFieldNames(&mut self, pcnames: *mut u32, rgbstrnames: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsMatchingType(&mut self, precordinfo: &::core::option::Option<IRecordInfo>) -> super::super::Foundation::BOOL;
    fn RecordCreate(&mut self) -> *mut ::core::ffi::c_void;
    fn RecordCreateCopy(&mut self, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RecordDestroy(&mut self, pvrecord: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRecordInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>() -> IRecordInfo_Vtbl {
        unsafe extern "system" fn RecordInit<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RecordInit(::core::mem::transmute_copy(&pvnew)).into()
        }
        unsafe extern "system" fn RecordClear<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RecordClear(::core::mem::transmute_copy(&pvexisting)).into()
        }
        unsafe extern "system" fn RecordCopy<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RecordCopy(::core::mem::transmute_copy(&pvexisting), ::core::mem::transmute_copy(&pvnew)).into()
        }
        unsafe extern "system" fn GetGuid<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTypeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pptypeinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetField<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetField(::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&szfieldname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarfield = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFieldNoCopy<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *mut super::Com::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFieldNoCopy(::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&szfieldname), ::core::mem::transmute_copy(&pvarfield), ::core::mem::transmute_copy(&ppvdatacarray)).into()
        }
        unsafe extern "system" fn PutField<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutField(::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&szfieldname), ::core::mem::transmute_copy(&pvarfield)).into()
        }
        unsafe extern "system" fn PutFieldNoCopy<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutFieldNoCopy(::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&szfieldname), ::core::mem::transmute_copy(&pvarfield)).into()
        }
        unsafe extern "system" fn GetFieldNames<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnames: *mut u32, rgbstrnames: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFieldNames(::core::mem::transmute_copy(&pcnames), ::core::mem::transmute_copy(&rgbstrnames)).into()
        }
        unsafe extern "system" fn IsMatchingType<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordinfo: ::windows::core::RawPtr) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsMatchingType(::core::mem::transmute(&precordinfo))
        }
        unsafe extern "system" fn RecordCreate<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RecordCreate()
        }
        unsafe extern "system" fn RecordCreateCopy<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RecordCreateCopy(::core::mem::transmute_copy(&pvsource), ::core::mem::transmute_copy(&ppvdest)).into()
        }
        unsafe extern "system" fn RecordDestroy<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvrecord: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RecordDestroy(::core::mem::transmute_copy(&pvrecord)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RecordInit: RecordInit::<Identity, Impl, OFFSET>,
            RecordClear: RecordClear::<Identity, Impl, OFFSET>,
            RecordCopy: RecordCopy::<Identity, Impl, OFFSET>,
            GetGuid: GetGuid::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, Impl, OFFSET>,
            GetField: GetField::<Identity, Impl, OFFSET>,
            GetFieldNoCopy: GetFieldNoCopy::<Identity, Impl, OFFSET>,
            PutField: PutField::<Identity, Impl, OFFSET>,
            PutFieldNoCopy: PutFieldNoCopy::<Identity, Impl, OFFSET>,
            GetFieldNames: GetFieldNames::<Identity, Impl, OFFSET>,
            IsMatchingType: IsMatchingType::<Identity, Impl, OFFSET>,
            RecordCreate: RecordCreate::<Identity, Impl, OFFSET>,
            RecordCreateCopy: RecordCreateCopy::<Identity, Impl, OFFSET>,
            RecordDestroy: RecordDestroy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRecordInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimpleFrameSite_Impl: Sized {
    fn PreMessageFilter(&mut self, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows::core::Result<()>;
    fn PostMessageFilter(&mut self, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISimpleFrameSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleFrameSite_Impl, const OFFSET: isize>() -> ISimpleFrameSite_Vtbl {
        unsafe extern "system" fn PreMessageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleFrameSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreMessageFilter(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wp), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&plresult), ::core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn PostMessageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleFrameSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PostMessageFilter(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wp), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&plresult), ::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PreMessageFilter: PreMessageFilter::<Identity, Impl, OFFSET>,
            PostMessageFilter: PostMessageFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleFrameSite as ::windows::core::Interface>::IID
    }
}
pub trait ISpecifyPropertyPages_Impl: Sized {
    fn GetPages(&mut self) -> ::windows::core::Result<CAUUID>;
}
impl ISpecifyPropertyPages_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpecifyPropertyPages_Impl, const OFFSET: isize>() -> ISpecifyPropertyPages_Vtbl {
        unsafe extern "system" fn GetPages<Identity: ::windows::core::IUnknownImpl, Impl: ISpecifyPropertyPages_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppages: *mut CAUUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPages() {
                ::core::result::Result::Ok(ok__) => {
                    *ppages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetPages: GetPages::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpecifyPropertyPages as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITypeChangeEvents_Impl: Sized {
    fn RequestTypeChange(&mut self, changekind: CHANGEKIND, ptinfobefore: &::core::option::Option<super::Com::ITypeInfo>, pstrname: super::super::Foundation::PWSTR) -> ::windows::core::Result<i32>;
    fn AfterTypeChange(&mut self, changekind: CHANGEKIND, ptinfoafter: &::core::option::Option<super::Com::ITypeInfo>, pstrname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITypeChangeEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeChangeEvents_Impl, const OFFSET: isize>() -> ITypeChangeEvents_Vtbl {
        unsafe extern "system" fn RequestTypeChange<Identity: ::windows::core::IUnknownImpl, Impl: ITypeChangeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfobefore: ::windows::core::RawPtr, pstrname: super::super::Foundation::PWSTR, pfcancel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestTypeChange(::core::mem::transmute_copy(&changekind), ::core::mem::transmute(&ptinfobefore), ::core::mem::transmute_copy(&pstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfcancel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AfterTypeChange<Identity: ::windows::core::IUnknownImpl, Impl: ITypeChangeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfoafter: ::windows::core::RawPtr, pstrname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AfterTypeChange(::core::mem::transmute_copy(&changekind), ::core::mem::transmute(&ptinfoafter), ::core::mem::transmute_copy(&pstrname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RequestTypeChange: RequestTypeChange::<Identity, Impl, OFFSET>,
            AfterTypeChange: AfterTypeChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeChangeEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITypeFactory_Impl: Sized {
    fn CreateFromTypeInfo(&mut self, ptypeinfo: &::core::option::Option<super::Com::ITypeInfo>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITypeFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeFactory_Impl, const OFFSET: isize>() -> ITypeFactory_Vtbl {
        unsafe extern "system" fn CreateFromTypeInfo<Identity: ::windows::core::IUnknownImpl, Impl: ITypeFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeinfo: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFromTypeInfo(::core::mem::transmute(&ptypeinfo), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppv = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateFromTypeInfo: CreateFromTypeInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeFactory as ::windows::core::Interface>::IID
    }
}
pub trait ITypeMarshal_Impl: Sized {
    fn Size(&mut self, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u32>;
    fn Marshal(&mut self, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> ::windows::core::Result<()>;
    fn Unmarshal(&mut self, pvtype: *mut ::core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> ::windows::core::Result<()>;
    fn Free(&mut self, pvtype: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ITypeMarshal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeMarshal_Impl, const OFFSET: isize>() -> ITypeMarshal_Vtbl {
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl, Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, psize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Size(::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *psize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Marshal<Identity: ::windows::core::IUnknownImpl, Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Marshal(::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&cbbufferlength), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbwritten)).into()
        }
        unsafe extern "system" fn Unmarshal<Identity: ::windows::core::IUnknownImpl, Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *mut ::core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unmarshal(::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cbbufferlength), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbread)).into()
        }
        unsafe extern "system" fn Free<Identity: ::windows::core::IUnknownImpl, Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Free(::core::mem::transmute_copy(&pvtype)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Size: Size::<Identity, Impl, OFFSET>,
            Marshal: Marshal::<Identity, Impl, OFFSET>,
            Unmarshal: Unmarshal::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeMarshal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVBFormat_Impl: Sized {
    fn Format(&mut self, vdata: *mut super::Com::VARIANT, bstrformat: &super::super::Foundation::BSTR, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVBFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBFormat_Impl, const OFFSET: isize>() -> IVBFormat_Vtbl {
        unsafe extern "system" fn Format<Identity: ::windows::core::IUnknownImpl, Impl: IVBFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdata: *mut super::Com::VARIANT, bstrformat: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Format(::core::mem::transmute_copy(&vdata), ::core::mem::transmute_copy(&bstrformat), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&sfirstdayofweek), ::core::mem::transmute_copy(&sfirstweekofyear), ::core::mem::transmute_copy(&rcb)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Format: Format::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBGetControl_Impl: Sized {
    fn EnumControls(&mut self, dwolecontf: OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS) -> ::windows::core::Result<super::Com::IEnumUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl IVBGetControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBGetControl_Impl, const OFFSET: isize>() -> IVBGetControl_Vtbl {
        unsafe extern "system" fn EnumControls<Identity: ::windows::core::IUnknownImpl, Impl: IVBGetControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwolecontf: OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS, ppenumunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumControls(::core::mem::transmute_copy(&dwolecontf), ::core::mem::transmute_copy(&dwwhich)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), EnumControls: EnumControls::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBGetControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVariantChangeType_Impl: Sized {
    fn ChangeType(&mut self, pvardst: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, vtnew: u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVariantChangeType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVariantChangeType_Impl, const OFFSET: isize>() -> IVariantChangeType_Vtbl {
        unsafe extern "system" fn ChangeType<Identity: ::windows::core::IUnknownImpl, Impl: IVariantChangeType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardst: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, vtnew: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangeType(::core::mem::transmute_copy(&pvardst), ::core::mem::transmute_copy(&pvarsrc), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&vtnew)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ChangeType: ChangeType::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVariantChangeType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObject_Impl: Sized {
    fn Draw(&mut self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows::core::Result<()>;
    fn GetColorSet(&mut self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::Result<()>;
    fn Freeze(&mut self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::core::Result<()>;
    fn Unfreeze(&mut self, dwfreeze: u32) -> ::windows::core::Result<()>;
    fn SetAdvise(&mut self, aspects: u32, advf: u32, padvsink: &::core::option::Option<super::Com::IAdviseSink>) -> ::windows::core::Result<()>;
    fn GetAdvise(&mut self, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::core::option::Option<super::Com::IAdviseSink>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject_Impl, const OFFSET: isize>() -> IViewObject_Vtbl {
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Draw(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hdctargetdev), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&lprcbounds), ::core::mem::transmute_copy(&lprcwbounds), ::core::mem::transmute_copy(&pfncontinue), ::core::mem::transmute_copy(&dwcontinue)).into()
        }
        unsafe extern "system" fn GetColorSet<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorSet(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&ppcolorset)).into()
        }
        unsafe extern "system" fn Freeze<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Freeze(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&pdwfreeze)).into()
        }
        unsafe extern "system" fn Unfreeze<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfreeze: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unfreeze(::core::mem::transmute_copy(&dwfreeze)).into()
        }
        unsafe extern "system" fn SetAdvise<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aspects: u32, advf: u32, padvsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAdvise(::core::mem::transmute_copy(&aspects), ::core::mem::transmute_copy(&advf), ::core::mem::transmute(&padvsink)).into()
        }
        unsafe extern "system" fn GetAdvise<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAdvise(::core::mem::transmute_copy(&paspects), ::core::mem::transmute_copy(&padvf), ::core::mem::transmute_copy(&ppadvsink)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Draw: Draw::<Identity, Impl, OFFSET>,
            GetColorSet: GetColorSet::<Identity, Impl, OFFSET>,
            Freeze: Freeze::<Identity, Impl, OFFSET>,
            Unfreeze: Unfreeze::<Identity, Impl, OFFSET>,
            SetAdvise: SetAdvise::<Identity, Impl, OFFSET>,
            GetAdvise: GetAdvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObject2_Impl: Sized + IViewObject_Impl {
    fn GetExtent(&mut self, dwdrawaspect: u32, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE) -> ::windows::core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObject2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject2_Impl, const OFFSET: isize>() -> IViewObject2_Vtbl {
        unsafe extern "system" fn GetExtent<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, lpsizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetExtent(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&ptd)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpsizel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IViewObject_Vtbl::new::<Identity, Impl, OFFSET>(), GetExtent: GetExtent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewObject2 as ::windows::core::Interface>::IID || iid == &<IViewObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObjectEx_Impl: Sized + IViewObject_Impl + IViewObject2_Impl {
    fn GetRect(&mut self, dwaspect: u32) -> ::windows::core::Result<super::super::Foundation::RECTL>;
    fn GetViewStatus(&mut self) -> ::windows::core::Result<u32>;
    fn QueryHitPoint(&mut self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: &super::super::Foundation::POINT, lclosehint: i32) -> ::windows::core::Result<u32>;
    fn QueryHitRect(&mut self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32) -> ::windows::core::Result<u32>;
    fn GetNaturalExtent(&mut self, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const ExtentInfo) -> ::windows::core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObjectEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObjectEx_Impl, const OFFSET: isize>() -> IViewObjectEx_Vtbl {
        unsafe extern "system" fn GetRect<Identity: ::windows::core::IUnknownImpl, Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prect: *mut super::super::Foundation::RECTL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRect(::core::mem::transmute_copy(&dwaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    *prect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewStatus<Identity: ::windows::core::IUnknownImpl, Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetViewStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHitPoint<Identity: ::windows::core::IUnknownImpl, Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryHitPoint(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&ptlloc), ::core::mem::transmute_copy(&lclosehint)) {
                ::core::result::Result::Ok(ok__) => {
                    *phitresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHitRect<Identity: ::windows::core::IUnknownImpl, Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryHitRect(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&prectloc), ::core::mem::transmute_copy(&lclosehint)) {
                ::core::result::Result::Ok(ok__) => {
                    *phitresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNaturalExtent<Identity: ::windows::core::IUnknownImpl, Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const ExtentInfo, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNaturalExtent(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&pextentinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *psizel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IViewObject2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRect: GetRect::<Identity, Impl, OFFSET>,
            GetViewStatus: GetViewStatus::<Identity, Impl, OFFSET>,
            QueryHitPoint: QueryHitPoint::<Identity, Impl, OFFSET>,
            QueryHitRect: QueryHitRect::<Identity, Impl, OFFSET>,
            GetNaturalExtent: GetNaturalExtent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewObjectEx as ::windows::core::Interface>::IID || iid == &<IViewObject as ::windows::core::Interface>::IID || iid == &<IViewObject2 as ::windows::core::Interface>::IID
    }
}
pub trait IZoomEvents_Impl: Sized {
    fn OnZoomPercentChanged(&mut self, ulzoompercent: u32) -> ::windows::core::Result<()>;
}
impl IZoomEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoomEvents_Impl, const OFFSET: isize>() -> IZoomEvents_Vtbl {
        unsafe extern "system" fn OnZoomPercentChanged<Identity: ::windows::core::IUnknownImpl, Impl: IZoomEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulzoompercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnZoomPercentChanged(::core::mem::transmute_copy(&ulzoompercent)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnZoomPercentChanged: OnZoomPercentChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IZoomEvents as ::windows::core::Interface>::IID
    }
}
