#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSinkEx_Impl: Sized + super::Com::IAdviseSink_Impl {
    fn OnViewStatusChange(&mut self, dwviewstatus: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSinkEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdviseSinkEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdviseSinkEx_Vtbl {
        unsafe extern "system" fn OnViewStatusChange<Impl: IAdviseSinkEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwviewstatus: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnViewStatusChange(::core::mem::transmute_copy(&dwviewstatus))
        }
        Self {
            base: super::Com::IAdviseSink_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnViewStatusChange: OnViewStatusChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdviseSinkEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICanHandleException_Impl: Sized {
    fn CanHandleException(&mut self, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICanHandleException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICanHandleException_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICanHandleException_Vtbl {
        unsafe extern "system" fn CanHandleException<Impl: ICanHandleException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CanHandleException(::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&pvar)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CanHandleException: CanHandleException::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClassFactory2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClassFactory2_Vtbl {
        unsafe extern "system" fn GetLicInfo<Impl: IClassFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plicinfo: *mut LICINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLicInfo(::core::mem::transmute_copy(&plicinfo)).into()
        }
        unsafe extern "system" fn RequestLicKey<Impl: IClassFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, pbstrkey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLicKey(::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceLic<Impl: IClassFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, bstrkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInstanceLic(::core::mem::transmute(&punkouter), ::core::mem::transmute(&punkreserved), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&bstrkey), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        Self {
            base: super::Com::IClassFactory_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLicInfo: GetLicInfo::<Impl, IMPL_OFFSET>,
            RequestLicKey: RequestLicKey::<Impl, IMPL_OFFSET>,
            CreateInstanceLic: CreateInstanceLic::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClassFactory2 as ::windows::core::Interface>::IID
    }
}
pub trait IContinue_Impl: Sized {
    fn FContinue(&mut self) -> ::windows::core::Result<()>;
}
impl IContinue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContinue_Vtbl {
        unsafe extern "system" fn FContinue<Impl: IContinue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FContinue().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FContinue: FContinue::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinueCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContinueCallback_Vtbl {
        unsafe extern "system" fn FContinue<Impl: IContinueCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FContinue().into()
        }
        unsafe extern "system" fn FContinuePrinting<Impl: IContinueCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncntprinted: i32, ncurpage: i32, pwszprintstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FContinuePrinting(::core::mem::transmute_copy(&ncntprinted), ::core::mem::transmute_copy(&ncurpage), ::core::mem::transmute_copy(&pwszprintstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FContinue: FContinue::<Impl, IMPL_OFFSET>,
            FContinuePrinting: FContinuePrinting::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateErrorInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateErrorInfo_Vtbl {
        unsafe extern "system" fn SetGUID<Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGUID(::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn SetSource<Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsource: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(::core::mem::transmute_copy(&szsource)).into()
        }
        unsafe extern "system" fn SetDescription<Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&szdescription)).into()
        }
        unsafe extern "system" fn SetHelpFile<Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhelpfile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpFile(::core::mem::transmute_copy(&szhelpfile)).into()
        }
        unsafe extern "system" fn SetHelpContext<Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpContext(::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetGUID: SetGUID::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            SetHelpFile: SetHelpFile::<Impl, IMPL_OFFSET>,
            SetHelpContext: SetHelpContext::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateTypeInfo_Vtbl {
        unsafe extern "system" fn SetGuid<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGuid(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn SetTypeFlags<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utypeflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTypeFlags(::core::mem::transmute_copy(&utypeflags)).into()
        }
        unsafe extern "system" fn SetDocString<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdoc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocString(::core::mem::transmute_copy(&pstrdoc)).into()
        }
        unsafe extern "system" fn SetHelpContext<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpContext(::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetVersion<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&wmajorvernum), ::core::mem::transmute_copy(&wminorvernum)).into()
        }
        unsafe extern "system" fn AddRefTypeInfo<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptinfo: ::windows::core::RawPtr, phreftype: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRefTypeInfo(::core::mem::transmute(&ptinfo), ::core::mem::transmute_copy(&phreftype)).into()
        }
        unsafe extern "system" fn AddFuncDesc<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFuncDesc(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pfuncdesc)).into()
        }
        unsafe extern "system" fn AddImplType<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, hreftype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddImplType(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&hreftype)).into()
        }
        unsafe extern "system" fn SetImplTypeFlags<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, impltypeflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImplTypeFlags(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&impltypeflags)).into()
        }
        unsafe extern "system" fn SetAlignment<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbalignment: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlignment(::core::mem::transmute_copy(&cbalignment)).into()
        }
        unsafe extern "system" fn SetSchema<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrschema: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSchema(::core::mem::transmute_copy(&pstrschema)).into()
        }
        unsafe extern "system" fn AddVarDesc<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddVarDesc(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pvardesc)).into()
        }
        unsafe extern "system" fn SetFuncAndParamNames<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFuncAndParamNames(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames)).into()
        }
        unsafe extern "system" fn SetVarName<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVarName(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&szname)).into()
        }
        unsafe extern "system" fn SetTypeDescAlias<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptdescalias: *const super::Com::TYPEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTypeDescAlias(::core::mem::transmute_copy(&ptdescalias)).into()
        }
        unsafe extern "system" fn DefineFuncAsDllEntry<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdllname: super::super::Foundation::PWSTR, szprocname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DefineFuncAsDllEntry(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&szdllname), ::core::mem::transmute_copy(&szprocname)).into()
        }
        unsafe extern "system" fn SetFuncDocString<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFuncDocString(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&szdocstring)).into()
        }
        unsafe extern "system" fn SetVarDocString<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVarDocString(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&szdocstring)).into()
        }
        unsafe extern "system" fn SetFuncHelpContext<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFuncHelpContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetVarHelpContext<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVarHelpContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetMops<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, bstrmops: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMops(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&bstrmops)).into()
        }
        unsafe extern "system" fn SetTypeIdldesc<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidldesc: *const super::Com::IDLDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTypeIdldesc(::core::mem::transmute_copy(&pidldesc)).into()
        }
        unsafe extern "system" fn LayOut<Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LayOut().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetGuid: SetGuid::<Impl, IMPL_OFFSET>,
            SetTypeFlags: SetTypeFlags::<Impl, IMPL_OFFSET>,
            SetDocString: SetDocString::<Impl, IMPL_OFFSET>,
            SetHelpContext: SetHelpContext::<Impl, IMPL_OFFSET>,
            SetVersion: SetVersion::<Impl, IMPL_OFFSET>,
            AddRefTypeInfo: AddRefTypeInfo::<Impl, IMPL_OFFSET>,
            AddFuncDesc: AddFuncDesc::<Impl, IMPL_OFFSET>,
            AddImplType: AddImplType::<Impl, IMPL_OFFSET>,
            SetImplTypeFlags: SetImplTypeFlags::<Impl, IMPL_OFFSET>,
            SetAlignment: SetAlignment::<Impl, IMPL_OFFSET>,
            SetSchema: SetSchema::<Impl, IMPL_OFFSET>,
            AddVarDesc: AddVarDesc::<Impl, IMPL_OFFSET>,
            SetFuncAndParamNames: SetFuncAndParamNames::<Impl, IMPL_OFFSET>,
            SetVarName: SetVarName::<Impl, IMPL_OFFSET>,
            SetTypeDescAlias: SetTypeDescAlias::<Impl, IMPL_OFFSET>,
            DefineFuncAsDllEntry: DefineFuncAsDllEntry::<Impl, IMPL_OFFSET>,
            SetFuncDocString: SetFuncDocString::<Impl, IMPL_OFFSET>,
            SetVarDocString: SetVarDocString::<Impl, IMPL_OFFSET>,
            SetFuncHelpContext: SetFuncHelpContext::<Impl, IMPL_OFFSET>,
            SetVarHelpContext: SetVarHelpContext::<Impl, IMPL_OFFSET>,
            SetMops: SetMops::<Impl, IMPL_OFFSET>,
            SetTypeIdldesc: SetTypeIdldesc::<Impl, IMPL_OFFSET>,
            LayOut: LayOut::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateTypeInfo2_Vtbl {
        unsafe extern "system" fn DeleteFuncDesc<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteFuncDesc(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn DeleteFuncDescByMemId<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteFuncDescByMemId(::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&invkind)).into()
        }
        unsafe extern "system" fn DeleteVarDesc<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteVarDesc(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn DeleteVarDescByMemId<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteVarDescByMemId(::core::mem::transmute_copy(&memid)).into()
        }
        unsafe extern "system" fn DeleteImplType<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteImplType(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetCustData<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetFuncCustData<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFuncCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetParamCustData<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParamCustData(::core::mem::transmute_copy(&indexfunc), ::core::mem::transmute_copy(&indexparam), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetVarCustData<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVarCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetImplTypeCustData<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImplTypeCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetHelpStringContext<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpStringContext(::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetFuncHelpStringContext<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFuncHelpStringContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetVarHelpStringContext<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVarHelpStringContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn Invalidate<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invalidate().into()
        }
        unsafe extern "system" fn SetName<Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&szname)).into()
        }
        Self {
            base: ICreateTypeInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DeleteFuncDesc: DeleteFuncDesc::<Impl, IMPL_OFFSET>,
            DeleteFuncDescByMemId: DeleteFuncDescByMemId::<Impl, IMPL_OFFSET>,
            DeleteVarDesc: DeleteVarDesc::<Impl, IMPL_OFFSET>,
            DeleteVarDescByMemId: DeleteVarDescByMemId::<Impl, IMPL_OFFSET>,
            DeleteImplType: DeleteImplType::<Impl, IMPL_OFFSET>,
            SetCustData: SetCustData::<Impl, IMPL_OFFSET>,
            SetFuncCustData: SetFuncCustData::<Impl, IMPL_OFFSET>,
            SetParamCustData: SetParamCustData::<Impl, IMPL_OFFSET>,
            SetVarCustData: SetVarCustData::<Impl, IMPL_OFFSET>,
            SetImplTypeCustData: SetImplTypeCustData::<Impl, IMPL_OFFSET>,
            SetHelpStringContext: SetHelpStringContext::<Impl, IMPL_OFFSET>,
            SetFuncHelpStringContext: SetFuncHelpStringContext::<Impl, IMPL_OFFSET>,
            SetVarHelpStringContext: SetVarHelpStringContext::<Impl, IMPL_OFFSET>,
            Invalidate: Invalidate::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateTypeInfo2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateTypeLib_Vtbl {
        unsafe extern "system" fn CreateTypeInfo<Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, tkind: super::Com::TYPEKIND, ppctinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTypeInfo(::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&tkind)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppctinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&szname)).into()
        }
        unsafe extern "system" fn SetVersion<Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&wmajorvernum), ::core::mem::transmute_copy(&wminorvernum)).into()
        }
        unsafe extern "system" fn SetGuid<Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGuid(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn SetDocString<Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdoc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocString(::core::mem::transmute_copy(&szdoc)).into()
        }
        unsafe extern "system" fn SetHelpFileName<Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhelpfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpFileName(::core::mem::transmute_copy(&szhelpfilename)).into()
        }
        unsafe extern "system" fn SetHelpContext<Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpContext(::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetLcid<Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLcid(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn SetLibFlags<Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulibflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLibFlags(::core::mem::transmute_copy(&ulibflags)).into()
        }
        unsafe extern "system" fn SaveAllChanges<Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveAllChanges().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateTypeInfo: CreateTypeInfo::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            SetVersion: SetVersion::<Impl, IMPL_OFFSET>,
            SetGuid: SetGuid::<Impl, IMPL_OFFSET>,
            SetDocString: SetDocString::<Impl, IMPL_OFFSET>,
            SetHelpFileName: SetHelpFileName::<Impl, IMPL_OFFSET>,
            SetHelpContext: SetHelpContext::<Impl, IMPL_OFFSET>,
            SetLcid: SetLcid::<Impl, IMPL_OFFSET>,
            SetLibFlags: SetLibFlags::<Impl, IMPL_OFFSET>,
            SaveAllChanges: SaveAllChanges::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateTypeLib2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateTypeLib2_Vtbl {
        unsafe extern "system" fn DeleteTypeInfo<Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteTypeInfo(::core::mem::transmute_copy(&szname)).into()
        }
        unsafe extern "system" fn SetCustData<Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetHelpStringContext<Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpStringContext(::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetHelpStringDll<Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpStringDll(::core::mem::transmute_copy(&szfilename)).into()
        }
        Self {
            base: ICreateTypeLib_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DeleteTypeInfo: DeleteTypeInfo::<Impl, IMPL_OFFSET>,
            SetCustData: SetCustData::<Impl, IMPL_OFFSET>,
            SetHelpStringContext: SetHelpStringContext::<Impl, IMPL_OFFSET>,
            SetHelpStringDll: SetHelpStringDll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateTypeLib2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispError_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispError_Vtbl {
        unsafe extern "system" fn QueryErrorInfo<Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guiderrortype: ::windows::core::GUID, ppde: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryErrorInfo(::core::mem::transmute_copy(&guiderrortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppde = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNext<Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppde: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppde = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHresult<Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHresult() {
                ::core::result::Result::Ok(ok__) => {
                    *phr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSource() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpInfo<Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR, pdwcontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHelpInfo(::core::mem::transmute_copy(&pbstrfilename), ::core::mem::transmute_copy(&pdwcontext)).into()
        }
        unsafe extern "system" fn GetDescription<Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryErrorInfo: QueryErrorInfo::<Impl, IMPL_OFFSET>,
            GetNext: GetNext::<Impl, IMPL_OFFSET>,
            GetHresult: GetHresult::<Impl, IMPL_OFFSET>,
            GetSource: GetSource::<Impl, IMPL_OFFSET>,
            GetHelpInfo: GetHelpInfo::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatchEx_Vtbl {
        unsafe extern "system" fn GetDispID<Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grfdex: u32, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDispID(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&grfdex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeEx<Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut super::Com::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeEx(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdp), ::core::mem::transmute_copy(&pvarres), ::core::mem::transmute_copy(&pei), ::core::mem::transmute(&pspcaller)).into()
        }
        unsafe extern "system" fn DeleteMemberByName<Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grfdex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteMemberByName(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&grfdex)).into()
        }
        unsafe extern "system" fn DeleteMemberByDispID<Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteMemberByDispID(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetMemberProperties<Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, grfdexfetch: u32, pgrfdex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberProperties(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&grfdexfetch)) {
                ::core::result::Result::Ok(ok__) => {
                    *pgrfdex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberName<Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberName(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextDispID<Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfdex: u32, id: i32, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextDispID(::core::mem::transmute_copy(&grfdex), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameSpaceParent<Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameSpaceParent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDispID: GetDispID::<Impl, IMPL_OFFSET>,
            InvokeEx: InvokeEx::<Impl, IMPL_OFFSET>,
            DeleteMemberByName: DeleteMemberByName::<Impl, IMPL_OFFSET>,
            DeleteMemberByDispID: DeleteMemberByDispID::<Impl, IMPL_OFFSET>,
            GetMemberProperties: GetMemberProperties::<Impl, IMPL_OFFSET>,
            GetMemberName: GetMemberName::<Impl, IMPL_OFFSET>,
            GetNextDispID: GetNextDispID::<Impl, IMPL_OFFSET>,
            GetNameSpaceParent: GetNameSpaceParent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatchEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDropSource_Impl: Sized {
    fn QueryContinueDrag(&mut self, fescapepressed: super::super::Foundation::BOOL, grfkeystate: u32) -> ::windows::core::Result<()>;
    fn GiveFeedback(&mut self, dweffect: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDropSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropSource_Vtbl {
        unsafe extern "system" fn QueryContinueDrag<Impl: IDropSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fescapepressed: super::super::Foundation::BOOL, grfkeystate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryContinueDrag(::core::mem::transmute_copy(&fescapepressed), ::core::mem::transmute_copy(&grfkeystate)).into()
        }
        unsafe extern "system" fn GiveFeedback<Impl: IDropSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweffect: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GiveFeedback(::core::mem::transmute_copy(&dweffect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryContinueDrag: QueryContinueDrag::<Impl, IMPL_OFFSET>,
            GiveFeedback: GiveFeedback::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropSourceNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropSourceNotify_Vtbl {
        unsafe extern "system" fn DragEnterTarget<Impl: IDropSourceNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndtarget: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DragEnterTarget(::core::mem::transmute_copy(&hwndtarget)).into()
        }
        unsafe extern "system" fn DragLeaveTarget<Impl: IDropSourceNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DragLeaveTarget().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DragEnterTarget: DragEnterTarget::<Impl, IMPL_OFFSET>,
            DragLeaveTarget: DragLeaveTarget::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTarget_Vtbl {
        unsafe extern "system" fn DragEnter<Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobj: ::windows::core::RawPtr, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DragEnter(::core::mem::transmute(&pdataobj), ::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&pdweffect)).into()
        }
        unsafe extern "system" fn DragOver<Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DragOver(::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&pdweffect)).into()
        }
        unsafe extern "system" fn DragLeave<Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DragLeave().into()
        }
        unsafe extern "system" fn Drop<Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobj: ::windows::core::RawPtr, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Drop(::core::mem::transmute(&pdataobj), ::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&pdweffect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DragEnter: DragEnter::<Impl, IMPL_OFFSET>,
            DragOver: DragOver::<Impl, IMPL_OFFSET>,
            DragLeave: DragLeave::<Impl, IMPL_OFFSET>,
            Drop: Drop::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseDropTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnterpriseDropTarget_Vtbl {
        unsafe extern "system" fn SetDropSourceEnterpriseId<Impl: IEnterpriseDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDropSourceEnterpriseId(::core::mem::transmute_copy(&identity)).into()
        }
        unsafe extern "system" fn IsEvaluatingEdpPolicy<Impl: IEnterpriseDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEvaluatingEdpPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDropSourceEnterpriseId: SetDropSourceEnterpriseId::<Impl, IMPL_OFFSET>,
            IsEvaluatingEdpPolicy: IsEvaluatingEdpPolicy::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOLEVERB_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumOLEVERB_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleDocumentViews_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumOleDocumentViews_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cviews: u32, rgpview: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cviews), ::core::mem::transmute_copy(&rgpview), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cviews: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cviews)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOleUndoUnits_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumOleUndoUnits_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVARIANT_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumVARIANT_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFont_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFont_Vtbl {
        unsafe extern "system" fn Name<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Size<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *psize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn Bold<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbold: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bold() {
                ::core::result::Result::Ok(ok__) => {
                    *pbold = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bold: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBold(::core::mem::transmute_copy(&bold)).into()
        }
        unsafe extern "system" fn Italic<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitalic: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Italic() {
                ::core::result::Result::Ok(ok__) => {
                    *pitalic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, italic: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItalic(::core::mem::transmute_copy(&italic)).into()
        }
        unsafe extern "system" fn Underline<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punderline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Underline() {
                ::core::result::Result::Ok(ok__) => {
                    *punderline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, underline: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnderline(::core::mem::transmute_copy(&underline)).into()
        }
        unsafe extern "system" fn Strikethrough<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrikethrough: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strikethrough() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrikethrough = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strikethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrikethrough(::core::mem::transmute_copy(&strikethrough)).into()
        }
        unsafe extern "system" fn Weight<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweight: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *pweight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWeight(::core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn Charset<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcharset: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Charset() {
                ::core::result::Result::Ok(ok__) => {
                    *pcharset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharset<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, charset: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharset(::core::mem::transmute_copy(&charset)).into()
        }
        unsafe extern "system" fn hFont<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phfont: *mut super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).hFont() {
                ::core::result::Result::Ok(ok__) => {
                    *phfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfontother: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsEqual(::core::mem::transmute(&pfontother)).into()
        }
        unsafe extern "system" fn SetRatio<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cylogical: i32, cyhimetric: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRatio(::core::mem::transmute_copy(&cylogical), ::core::mem::transmute_copy(&cyhimetric)).into()
        }
        unsafe extern "system" fn QueryTextMetrics<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryTextMetrics() {
                ::core::result::Result::Ok(ok__) => {
                    *ptm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRefHfont<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRefHfont(::core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn ReleaseHfont<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseHfont(::core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn SetHdc<Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHdc(::core::mem::transmute_copy(&hdc)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            Bold: Bold::<Impl, IMPL_OFFSET>,
            SetBold: SetBold::<Impl, IMPL_OFFSET>,
            Italic: Italic::<Impl, IMPL_OFFSET>,
            SetItalic: SetItalic::<Impl, IMPL_OFFSET>,
            Underline: Underline::<Impl, IMPL_OFFSET>,
            SetUnderline: SetUnderline::<Impl, IMPL_OFFSET>,
            Strikethrough: Strikethrough::<Impl, IMPL_OFFSET>,
            SetStrikethrough: SetStrikethrough::<Impl, IMPL_OFFSET>,
            Weight: Weight::<Impl, IMPL_OFFSET>,
            SetWeight: SetWeight::<Impl, IMPL_OFFSET>,
            Charset: Charset::<Impl, IMPL_OFFSET>,
            SetCharset: SetCharset::<Impl, IMPL_OFFSET>,
            hFont: hFont::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            SetRatio: SetRatio::<Impl, IMPL_OFFSET>,
            QueryTextMetrics: QueryTextMetrics::<Impl, IMPL_OFFSET>,
            AddRefHfont: AddRefHfont::<Impl, IMPL_OFFSET>,
            ReleaseHfont: ReleaseHfont::<Impl, IMPL_OFFSET>,
            SetHdc: SetHdc::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontDisp_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFontDisp_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFontDisp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFontEventsDisp_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IFontEventsDisp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontEventsDisp_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFontEventsDisp_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFontEventsDisp as ::windows::core::Interface>::IID
    }
}
pub trait IGetOleObject_Impl: Sized {
    fn GetOleObject(&mut self, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IGetOleObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetOleObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetOleObject_Vtbl {
        unsafe extern "system" fn GetOleObject<Impl: IGetOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOleObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetOleObject: GetOleObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetOleObject as ::windows::core::Interface>::IID
    }
}
pub trait IGetVBAObject_Impl: Sized {
    fn GetObject(&mut self, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl IGetVBAObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetVBAObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetVBAObject_Vtbl {
        unsafe extern "system" fn GetObject<Impl: IGetVBAObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetObject: GetObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetVBAObject as ::windows::core::Interface>::IID
    }
}
pub trait IObjectIdentity_Impl: Sized {
    fn IsEqualObject(&mut self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IObjectIdentity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectIdentity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectIdentity_Vtbl {
        unsafe extern "system" fn IsEqualObject<Impl: IObjectIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsEqualObject(::core::mem::transmute(&punk)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IsEqualObject: IsEqualObject::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectWithSite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectWithSite_Vtbl {
        unsafe extern "system" fn SetSite<Impl: IObjectWithSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punksite: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSite(::core::mem::transmute(&punksite)).into()
        }
        unsafe extern "system" fn GetSite<Impl: IObjectWithSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvsite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSite(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvsite)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetSite: SetSite::<Impl, IMPL_OFFSET>,
            GetSite: GetSite::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleAdviseHolder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleAdviseHolder_Vtbl {
        unsafe extern "system" fn Advise<Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvise: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(::core::mem::transmute(&padvise)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumAdvise<Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAdvise() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumadvise = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOnRename<Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendOnRename(::core::mem::transmute(&pmk)).into()
        }
        unsafe extern "system" fn SendOnSave<Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendOnSave().into()
        }
        unsafe extern "system" fn SendOnClose<Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendOnClose().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Advise: Advise::<Impl, IMPL_OFFSET>,
            Unadvise: Unadvise::<Impl, IMPL_OFFSET>,
            EnumAdvise: EnumAdvise::<Impl, IMPL_OFFSET>,
            SendOnRename: SendOnRename::<Impl, IMPL_OFFSET>,
            SendOnSave: SendOnSave::<Impl, IMPL_OFFSET>,
            SendOnClose: SendOnClose::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleCache_Vtbl {
        unsafe extern "system" fn Cache<Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, advf: u32, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cache(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&advf)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uncache<Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uncache(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumCache<Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstatdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCache() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstatdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitCache<Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitCache(::core::mem::transmute(&pdataobject)).into()
        }
        unsafe extern "system" fn SetData<Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pmedium), ::core::mem::transmute_copy(&frelease)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Cache: Cache::<Impl, IMPL_OFFSET>,
            Uncache: Uncache::<Impl, IMPL_OFFSET>,
            EnumCache: EnumCache::<Impl, IMPL_OFFSET>,
            InitCache: InitCache::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCache2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleCache2_Vtbl {
        unsafe extern "system" fn UpdateCache<Impl: IOleCache2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, grfupdf: UPDFCACHE_FLAGS, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateCache(::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&grfupdf), ::core::mem::transmute_copy(&preserved)).into()
        }
        unsafe extern "system" fn DiscardCache<Impl: IOleCache2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdiscardoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardCache(::core::mem::transmute_copy(&dwdiscardoptions)).into()
        }
        Self {
            base: IOleCache_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UpdateCache: UpdateCache::<Impl, IMPL_OFFSET>,
            DiscardCache: DiscardCache::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleCache2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleCacheControl_Impl: Sized {
    fn OnRun(&mut self, pdataobject: &::core::option::Option<super::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn OnStop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOleCacheControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCacheControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleCacheControl_Vtbl {
        unsafe extern "system" fn OnRun<Impl: IOleCacheControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRun(::core::mem::transmute(&pdataobject)).into()
        }
        unsafe extern "system" fn OnStop<Impl: IOleCacheControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStop().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnRun: OnRun::<Impl, IMPL_OFFSET>, OnStop: OnStop::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleClientSite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleClientSite_Vtbl {
        unsafe extern "system" fn SaveObject<Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveObject().into()
        }
        unsafe extern "system" fn GetMoniker<Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMoniker(::core::mem::transmute_copy(&dwassign), ::core::mem::transmute_copy(&dwwhichmoniker)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontainer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowObject<Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowObject().into()
        }
        unsafe extern "system" fn OnShowWindow<Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnShowWindow(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn RequestNewObjectLayout<Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestNewObjectLayout().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SaveObject: SaveObject::<Impl, IMPL_OFFSET>,
            GetMoniker: GetMoniker::<Impl, IMPL_OFFSET>,
            GetContainer: GetContainer::<Impl, IMPL_OFFSET>,
            ShowObject: ShowObject::<Impl, IMPL_OFFSET>,
            OnShowWindow: OnShowWindow::<Impl, IMPL_OFFSET>,
            RequestNewObjectLayout: RequestNewObjectLayout::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleCommandTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleCommandTarget_Vtbl {
        unsafe extern "system" fn QueryStatus<Impl: IOleCommandTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows::core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryStatus(::core::mem::transmute_copy(&pguidcmdgroup), ::core::mem::transmute_copy(&ccmds), ::core::mem::transmute_copy(&prgcmds), ::core::mem::transmute_copy(&pcmdtext)).into()
        }
        unsafe extern "system" fn Exec<Impl: IOleCommandTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows::core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Com::VARIANT, pvaout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Exec(::core::mem::transmute_copy(&pguidcmdgroup), ::core::mem::transmute_copy(&ncmdid), ::core::mem::transmute_copy(&ncmdexecopt), ::core::mem::transmute_copy(&pvain), ::core::mem::transmute_copy(&pvaout)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryStatus: QueryStatus::<Impl, IMPL_OFFSET>,
            Exec: Exec::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleContainer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleContainer_Vtbl {
        unsafe extern "system" fn EnumObjects<Impl: IOleContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumObjects(::core::mem::transmute_copy(&grfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockContainer<Impl: IOleContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockContainer(::core::mem::transmute_copy(&flock)).into()
        }
        Self {
            base: IParseDisplayName_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumObjects: EnumObjects::<Impl, IMPL_OFFSET>,
            LockContainer: LockContainer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleContainer as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleControl_Vtbl {
        unsafe extern "system" fn GetControlInfo<Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut CONTROLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetControlInfo(::core::mem::transmute_copy(&pci)).into()
        }
        unsafe extern "system" fn OnMnemonic<Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMnemonic(::core::mem::transmute_copy(&pmsg)).into()
        }
        unsafe extern "system" fn OnAmbientPropertyChange<Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAmbientPropertyChange(::core::mem::transmute_copy(&dispid)).into()
        }
        unsafe extern "system" fn FreezeEvents<Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfreeze: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreezeEvents(::core::mem::transmute_copy(&bfreeze)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetControlInfo: GetControlInfo::<Impl, IMPL_OFFSET>,
            OnMnemonic: OnMnemonic::<Impl, IMPL_OFFSET>,
            OnAmbientPropertyChange: OnAmbientPropertyChange::<Impl, IMPL_OFFSET>,
            FreezeEvents: FreezeEvents::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleControlSite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleControlSite_Vtbl {
        unsafe extern "system" fn OnControlInfoChanged<Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnControlInfoChanged().into()
        }
        unsafe extern "system" fn LockInPlaceActive<Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockInPlaceActive(::core::mem::transmute_copy(&flock)).into()
        }
        unsafe extern "system" fn GetExtendedControl<Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtendedControl() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformCoords<Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: XFORMCOORDS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransformCoords(::core::mem::transmute_copy(&pptlhimetric), ::core::mem::transmute_copy(&pptfcontainer), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&grfmodifiers)).into()
        }
        unsafe extern "system" fn OnFocus<Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgotfocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFocus(::core::mem::transmute_copy(&fgotfocus)).into()
        }
        unsafe extern "system" fn ShowPropertyFrame<Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowPropertyFrame().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnControlInfoChanged: OnControlInfoChanged::<Impl, IMPL_OFFSET>,
            LockInPlaceActive: LockInPlaceActive::<Impl, IMPL_OFFSET>,
            GetExtendedControl: GetExtendedControl::<Impl, IMPL_OFFSET>,
            TransformCoords: TransformCoords::<Impl, IMPL_OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Impl, IMPL_OFFSET>,
            OnFocus: OnFocus::<Impl, IMPL_OFFSET>,
            ShowPropertyFrame: ShowPropertyFrame::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocument_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleDocument_Vtbl {
        unsafe extern "system" fn CreateView<Impl: IOleDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsite: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, dwreserved: u32, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateView(::core::mem::transmute(&pipsite), ::core::mem::transmute(&pstm), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocMiscStatus<Impl: IOleDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocMiscStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumViews<Impl: IOleDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumViews(::core::mem::transmute_copy(&ppenum), ::core::mem::transmute_copy(&ppview)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateView: CreateView::<Impl, IMPL_OFFSET>,
            GetDocMiscStatus: GetDocMiscStatus::<Impl, IMPL_OFFSET>,
            EnumViews: EnumViews::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentSite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleDocumentSite_Vtbl {
        unsafe extern "system" fn ActivateMe<Impl: IOleDocumentSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewtoactivate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateMe(::core::mem::transmute(&pviewtoactivate)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ActivateMe: ActivateMe::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleDocumentView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleDocumentView_Vtbl {
        unsafe extern "system" fn SetInPlaceSite<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInPlaceSite(::core::mem::transmute(&pipsite)).into()
        }
        unsafe extern "system" fn GetInPlaceSite<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInPlaceSite() {
                ::core::result::Result::Ok(ok__) => {
                    *ppipsite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRect<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRect(::core::mem::transmute_copy(&prcview)).into()
        }
        unsafe extern "system" fn GetRect<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRect() {
                ::core::result::Result::Ok(ok__) => {
                    *prcview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRectComplex<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRectComplex(::core::mem::transmute_copy(&prcview), ::core::mem::transmute_copy(&prchscroll), ::core::mem::transmute_copy(&prcvscroll), ::core::mem::transmute_copy(&prcsizebox)).into()
        }
        unsafe extern "system" fn Show<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn UIActivate<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuiactivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UIActivate(::core::mem::transmute_copy(&fuiactivate)).into()
        }
        unsafe extern "system" fn Open<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn CloseView<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseView(::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SaveViewState<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveViewState(::core::mem::transmute(&pstm)).into()
        }
        unsafe extern "system" fn ApplyViewState<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyViewState(::core::mem::transmute(&pstm)).into()
        }
        unsafe extern "system" fn Clone<Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsitenew: ::windows::core::RawPtr, ppviewnew: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute(&pipsitenew)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppviewnew = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetInPlaceSite: SetInPlaceSite::<Impl, IMPL_OFFSET>,
            GetInPlaceSite: GetInPlaceSite::<Impl, IMPL_OFFSET>,
            GetDocument: GetDocument::<Impl, IMPL_OFFSET>,
            SetRect: SetRect::<Impl, IMPL_OFFSET>,
            GetRect: GetRect::<Impl, IMPL_OFFSET>,
            SetRectComplex: SetRectComplex::<Impl, IMPL_OFFSET>,
            Show: Show::<Impl, IMPL_OFFSET>,
            UIActivate: UIActivate::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            CloseView: CloseView::<Impl, IMPL_OFFSET>,
            SaveViewState: SaveViewState::<Impl, IMPL_OFFSET>,
            ApplyViewState: ApplyViewState::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceActiveObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceActiveObject_Vtbl {
        unsafe extern "system" fn TranslateAccelerator<Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&lpmsg)).into()
        }
        unsafe extern "system" fn OnFrameWindowActivate<Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFrameWindowActivate(::core::mem::transmute_copy(&factivate)).into()
        }
        unsafe extern "system" fn OnDocWindowActivate<Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDocWindowActivate(::core::mem::transmute_copy(&factivate)).into()
        }
        unsafe extern "system" fn ResizeBorder<Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcborder: *const super::super::Foundation::RECT, puiwindow: ::windows::core::RawPtr, fframewindow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResizeBorder(::core::mem::transmute_copy(&prcborder), ::core::mem::transmute(&puiwindow), ::core::mem::transmute_copy(&fframewindow)).into()
        }
        unsafe extern "system" fn EnableModeless<Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableModeless(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base: IOleWindow_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TranslateAccelerator: TranslateAccelerator::<Impl, IMPL_OFFSET>,
            OnFrameWindowActivate: OnFrameWindowActivate::<Impl, IMPL_OFFSET>,
            OnDocWindowActivate: OnDocWindowActivate::<Impl, IMPL_OFFSET>,
            ResizeBorder: ResizeBorder::<Impl, IMPL_OFFSET>,
            EnableModeless: EnableModeless::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceActiveObject as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceFrame_Vtbl {
        unsafe extern "system" fn InsertMenus<Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertMenus(::core::mem::transmute_copy(&hmenushared), ::core::mem::transmute_copy(&lpmenuwidths)).into()
        }
        unsafe extern "system" fn SetMenu<Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMenu(::core::mem::transmute_copy(&hmenushared), ::core::mem::transmute_copy(&holemenu), ::core::mem::transmute_copy(&hwndactiveobject)).into()
        }
        unsafe extern "system" fn RemoveMenus<Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMenus(::core::mem::transmute_copy(&hmenushared)).into()
        }
        unsafe extern "system" fn SetStatusText<Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatusText(::core::mem::transmute_copy(&pszstatustext)).into()
        }
        unsafe extern "system" fn EnableModeless<Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableModeless(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&lpmsg), ::core::mem::transmute_copy(&wid)).into()
        }
        Self {
            base: IOleInPlaceUIWindow_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InsertMenus: InsertMenus::<Impl, IMPL_OFFSET>,
            SetMenu: SetMenu::<Impl, IMPL_OFFSET>,
            RemoveMenus: RemoveMenus::<Impl, IMPL_OFFSET>,
            SetStatusText: SetStatusText::<Impl, IMPL_OFFSET>,
            EnableModeless: EnableModeless::<Impl, IMPL_OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceFrame as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceObject_Vtbl {
        unsafe extern "system" fn InPlaceDeactivate<Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InPlaceDeactivate().into()
        }
        unsafe extern "system" fn UIDeactivate<Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UIDeactivate().into()
        }
        unsafe extern "system" fn SetObjectRects<Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectRects(::core::mem::transmute_copy(&lprcposrect), ::core::mem::transmute_copy(&lprccliprect)).into()
        }
        unsafe extern "system" fn ReactivateAndUndo<Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReactivateAndUndo().into()
        }
        Self {
            base: IOleWindow_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InPlaceDeactivate: InPlaceDeactivate::<Impl, IMPL_OFFSET>,
            UIDeactivate: UIDeactivate::<Impl, IMPL_OFFSET>,
            SetObjectRects: SetObjectRects::<Impl, IMPL_OFFSET>,
            ReactivateAndUndo: ReactivateAndUndo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceObjectWindowless_Impl: Sized + IOleWindow_Impl + IOleInPlaceObject_Impl {
    fn OnWindowMessage(&mut self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::LRESULT>;
    fn GetDropTarget(&mut self) -> ::windows::core::Result<IDropTarget>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleInPlaceObjectWindowless_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceObjectWindowless_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceObjectWindowless_Vtbl {
        unsafe extern "system" fn OnWindowMessage<Impl: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnWindowMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDropTarget<Impl: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdroptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDropTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdroptarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOleInPlaceObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnWindowMessage: OnWindowMessage::<Impl, IMPL_OFFSET>,
            GetDropTarget: GetDropTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceObjectWindowless as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceSite_Vtbl {
        unsafe extern "system" fn CanInPlaceActivate<Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CanInPlaceActivate().into()
        }
        unsafe extern "system" fn OnInPlaceActivate<Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInPlaceActivate().into()
        }
        unsafe extern "system" fn OnUIActivate<Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUIActivate().into()
        }
        unsafe extern "system" fn GetWindowContext<Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppframe: *mut ::windows::core::RawPtr, ppdoc: *mut ::windows::core::RawPtr, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWindowContext(::core::mem::transmute_copy(&ppframe), ::core::mem::transmute_copy(&ppdoc), ::core::mem::transmute_copy(&lprcposrect), ::core::mem::transmute_copy(&lprccliprect), ::core::mem::transmute_copy(&lpframeinfo)).into()
        }
        unsafe extern "system" fn Scroll<Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollextant: super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Scroll(::core::mem::transmute_copy(&scrollextant)).into()
        }
        unsafe extern "system" fn OnUIDeactivate<Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fundoable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUIDeactivate(::core::mem::transmute_copy(&fundoable)).into()
        }
        unsafe extern "system" fn OnInPlaceDeactivate<Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInPlaceDeactivate().into()
        }
        unsafe extern "system" fn DiscardUndoState<Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardUndoState().into()
        }
        unsafe extern "system" fn DeactivateAndUndo<Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeactivateAndUndo().into()
        }
        unsafe extern "system" fn OnPosRectChange<Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPosRectChange(::core::mem::transmute_copy(&lprcposrect)).into()
        }
        Self {
            base: IOleWindow_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CanInPlaceActivate: CanInPlaceActivate::<Impl, IMPL_OFFSET>,
            OnInPlaceActivate: OnInPlaceActivate::<Impl, IMPL_OFFSET>,
            OnUIActivate: OnUIActivate::<Impl, IMPL_OFFSET>,
            GetWindowContext: GetWindowContext::<Impl, IMPL_OFFSET>,
            Scroll: Scroll::<Impl, IMPL_OFFSET>,
            OnUIDeactivate: OnUIDeactivate::<Impl, IMPL_OFFSET>,
            OnInPlaceDeactivate: OnInPlaceDeactivate::<Impl, IMPL_OFFSET>,
            DiscardUndoState: DiscardUndoState::<Impl, IMPL_OFFSET>,
            DeactivateAndUndo: DeactivateAndUndo::<Impl, IMPL_OFFSET>,
            OnPosRectChange: OnPosRectChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceSite as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceSiteEx_Vtbl {
        unsafe extern "system" fn OnInPlaceActivateEx<Impl: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInPlaceActivateEx(::core::mem::transmute_copy(&pfnoredraw), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnInPlaceDeactivateEx<Impl: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnoredraw: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInPlaceDeactivateEx(::core::mem::transmute_copy(&fnoredraw)).into()
        }
        unsafe extern "system" fn RequestUIActivate<Impl: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestUIActivate().into()
        }
        Self {
            base: IOleInPlaceSite_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnInPlaceActivateEx: OnInPlaceActivateEx::<Impl, IMPL_OFFSET>,
            OnInPlaceDeactivateEx: OnInPlaceDeactivateEx::<Impl, IMPL_OFFSET>,
            RequestUIActivate: RequestUIActivate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceSiteEx as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceSiteWindowless_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceSiteWindowless_Vtbl {
        unsafe extern "system" fn CanWindowlessActivate<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CanWindowlessActivate().into()
        }
        unsafe extern "system" fn GetCapture<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapture().into()
        }
        unsafe extern "system" fn SetCapture<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcapture: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCapture(::core::mem::transmute_copy(&fcapture)).into()
        }
        unsafe extern "system" fn GetFocus<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFocus().into()
        }
        unsafe extern "system" fn SetFocus<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocus(::core::mem::transmute_copy(&ffocus)).into()
        }
        unsafe extern "system" fn GetDC<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, grfflags: u32, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDC(::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&grfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *phdc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&hdc)).into()
        }
        unsafe extern "system" fn InvalidateRect<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidateRect(::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&ferase)).into()
        }
        unsafe extern "system" fn InvalidateRgn<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidateRgn(::core::mem::transmute_copy(&hrgn), ::core::mem::transmute_copy(&ferase)).into()
        }
        unsafe extern "system" fn ScrollRect<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollRect(::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy), ::core::mem::transmute_copy(&prectscroll), ::core::mem::transmute_copy(&prectclip)).into()
        }
        unsafe extern "system" fn AdjustRect<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdjustRect(::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn OnDefWindowMessage<Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDefWindowMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOleInPlaceSiteEx_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CanWindowlessActivate: CanWindowlessActivate::<Impl, IMPL_OFFSET>,
            GetCapture: GetCapture::<Impl, IMPL_OFFSET>,
            SetCapture: SetCapture::<Impl, IMPL_OFFSET>,
            GetFocus: GetFocus::<Impl, IMPL_OFFSET>,
            SetFocus: SetFocus::<Impl, IMPL_OFFSET>,
            GetDC: GetDC::<Impl, IMPL_OFFSET>,
            ReleaseDC: ReleaseDC::<Impl, IMPL_OFFSET>,
            InvalidateRect: InvalidateRect::<Impl, IMPL_OFFSET>,
            InvalidateRgn: InvalidateRgn::<Impl, IMPL_OFFSET>,
            ScrollRect: ScrollRect::<Impl, IMPL_OFFSET>,
            AdjustRect: AdjustRect::<Impl, IMPL_OFFSET>,
            OnDefWindowMessage: OnDefWindowMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceSiteWindowless as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleInPlaceUIWindow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleInPlaceUIWindow_Vtbl {
        unsafe extern "system" fn GetBorder<Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprectborder: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBorder() {
                ::core::result::Result::Ok(ok__) => {
                    *lprectborder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestBorderSpace<Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestBorderSpace(::core::mem::transmute_copy(&pborderwidths)).into()
        }
        unsafe extern "system" fn SetBorderSpace<Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBorderSpace(::core::mem::transmute_copy(&pborderwidths)).into()
        }
        unsafe extern "system" fn SetActiveObject<Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactiveobject: ::windows::core::RawPtr, pszobjname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActiveObject(::core::mem::transmute(&pactiveobject), ::core::mem::transmute_copy(&pszobjname)).into()
        }
        Self {
            base: IOleWindow_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetBorder: GetBorder::<Impl, IMPL_OFFSET>,
            RequestBorderSpace: RequestBorderSpace::<Impl, IMPL_OFFSET>,
            SetBorderSpace: SetBorderSpace::<Impl, IMPL_OFFSET>,
            SetActiveObject: SetActiveObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleInPlaceUIWindow as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleItemContainer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleItemContainer_Vtbl {
        unsafe extern "system" fn GetObject<Impl: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR, dwspeedneeded: u32, pbc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObject(::core::mem::transmute_copy(&pszitem), ::core::mem::transmute_copy(&dwspeedneeded), ::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn GetObjectStorage<Impl: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectStorage(::core::mem::transmute_copy(&pszitem), ::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvstorage)).into()
        }
        unsafe extern "system" fn IsRunning<Impl: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsRunning(::core::mem::transmute_copy(&pszitem)).into()
        }
        Self {
            base: IOleContainer_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
            GetObjectStorage: GetObjectStorage::<Impl, IMPL_OFFSET>,
            IsRunning: IsRunning::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleItemContainer as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleLink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleLink_Vtbl {
        unsafe extern "system" fn SetUpdateOptions<Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdateOptions(::core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetUpdateOptions<Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdateOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwupdateopt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceMoniker<Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceMoniker(::core::mem::transmute(&pmk), ::core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn GetSourceMoniker<Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceMoniker() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceDisplayName<Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceDisplayName(::core::mem::transmute_copy(&pszstatustext)).into()
        }
        unsafe extern "system" fn GetSourceDisplayName<Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToSource<Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindflags: u32, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindToSource(::core::mem::transmute_copy(&bindflags), ::core::mem::transmute(&pbc)).into()
        }
        unsafe extern "system" fn BindIfRunning<Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindIfRunning().into()
        }
        unsafe extern "system" fn GetBoundSource<Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundSource() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnbindSource<Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnbindSource().into()
        }
        unsafe extern "system" fn Update<Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(::core::mem::transmute(&pbc)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetUpdateOptions: SetUpdateOptions::<Impl, IMPL_OFFSET>,
            GetUpdateOptions: GetUpdateOptions::<Impl, IMPL_OFFSET>,
            SetSourceMoniker: SetSourceMoniker::<Impl, IMPL_OFFSET>,
            GetSourceMoniker: GetSourceMoniker::<Impl, IMPL_OFFSET>,
            SetSourceDisplayName: SetSourceDisplayName::<Impl, IMPL_OFFSET>,
            GetSourceDisplayName: GetSourceDisplayName::<Impl, IMPL_OFFSET>,
            BindToSource: BindToSource::<Impl, IMPL_OFFSET>,
            BindIfRunning: BindIfRunning::<Impl, IMPL_OFFSET>,
            GetBoundSource: GetBoundSource::<Impl, IMPL_OFFSET>,
            UnbindSource: UnbindSource::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleObject_Vtbl {
        unsafe extern "system" fn SetClientSite<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientsite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientSite(::core::mem::transmute(&pclientsite)).into()
        }
        unsafe extern "system" fn GetClientSite<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclientsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientSite() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientsite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostNames<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcontainerapp: super::super::Foundation::PWSTR, szcontainerobj: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHostNames(::core::mem::transmute_copy(&szcontainerapp), ::core::mem::transmute_copy(&szcontainerobj)).into()
        }
        unsafe extern "system" fn Close<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsaveoption: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(::core::mem::transmute_copy(&dwsaveoption)).into()
        }
        unsafe extern "system" fn SetMoniker<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwhichmoniker: u32, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMoniker(::core::mem::transmute_copy(&dwwhichmoniker), ::core::mem::transmute(&pmk)).into()
        }
        unsafe extern "system" fn GetMoniker<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMoniker(::core::mem::transmute_copy(&dwassign), ::core::mem::transmute_copy(&dwwhichmoniker)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitFromData<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitFromData(::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&fcreation), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetClipboardData<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipboardData(::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoVerb<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: ::windows::core::RawPtr, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoVerb(::core::mem::transmute_copy(&iverb), ::core::mem::transmute_copy(&lpmsg), ::core::mem::transmute(&pactivesite), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lprcposrect)).into()
        }
        unsafe extern "system" fn EnumVerbs<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumoleverb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumVerbs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumoleverb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        unsafe extern "system" fn IsUpToDate<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsUpToDate().into()
        }
        unsafe extern "system" fn GetUserClassID<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserClassID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserType<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwformoftype: u32, pszusertype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserType(::core::mem::transmute_copy(&dwformoftype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszusertype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtent<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtent(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&psizel)).into()
        }
        unsafe extern "system" fn GetExtent<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtent(::core::mem::transmute_copy(&dwdrawaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    *psizel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvsink: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(::core::mem::transmute(&padvsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumAdvise<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAdvise() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumadvise = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMiscStatus<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMiscStatus(::core::mem::transmute_copy(&dwaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorScheme<Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorScheme(::core::mem::transmute_copy(&plogpal)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetClientSite: SetClientSite::<Impl, IMPL_OFFSET>,
            GetClientSite: GetClientSite::<Impl, IMPL_OFFSET>,
            SetHostNames: SetHostNames::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            SetMoniker: SetMoniker::<Impl, IMPL_OFFSET>,
            GetMoniker: GetMoniker::<Impl, IMPL_OFFSET>,
            InitFromData: InitFromData::<Impl, IMPL_OFFSET>,
            GetClipboardData: GetClipboardData::<Impl, IMPL_OFFSET>,
            DoVerb: DoVerb::<Impl, IMPL_OFFSET>,
            EnumVerbs: EnumVerbs::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
            IsUpToDate: IsUpToDate::<Impl, IMPL_OFFSET>,
            GetUserClassID: GetUserClassID::<Impl, IMPL_OFFSET>,
            GetUserType: GetUserType::<Impl, IMPL_OFFSET>,
            SetExtent: SetExtent::<Impl, IMPL_OFFSET>,
            GetExtent: GetExtent::<Impl, IMPL_OFFSET>,
            Advise: Advise::<Impl, IMPL_OFFSET>,
            Unadvise: Unadvise::<Impl, IMPL_OFFSET>,
            EnumAdvise: EnumAdvise::<Impl, IMPL_OFFSET>,
            GetMiscStatus: GetMiscStatus::<Impl, IMPL_OFFSET>,
            SetColorScheme: SetColorScheme::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleParentUndoUnit_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleParentUndoUnit_Vtbl {
        unsafe extern "system" fn Open<Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute(&ppuu)).into()
        }
        unsafe extern "system" fn Close<Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr, fcommit: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(::core::mem::transmute(&ppuu), ::core::mem::transmute_copy(&fcommit)).into()
        }
        unsafe extern "system" fn Add<Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn FindUnit<Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindUnit(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn GetParentState<Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOleUndoUnit_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            FindUnit: FindUnit::<Impl, IMPL_OFFSET>,
            GetParentState: GetParentState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleParentUndoUnit as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerA_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUILinkContainerA_Vtbl {
        unsafe extern "system" fn GetNextLink<Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextLink(::core::mem::transmute_copy(&dwlink))
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpdwupdateopt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkSource<Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: super::super::Foundation::PSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lpszdisplayname), ::core::mem::transmute_copy(&lenfilename), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&fvalidatesource)).into()
        }
        unsafe extern "system" fn GetLinkSource<Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PSTR, lplpszshortlinktype: *mut super::super::Foundation::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lplpszdisplayname), ::core::mem::transmute_copy(&lplenfilename), ::core::mem::transmute_copy(&lplpszfulllinktype), ::core::mem::transmute_copy(&lplpszshortlinktype), ::core::mem::transmute_copy(&lpfsourceavailable), ::core::mem::transmute_copy(&lpfisselected)).into()
        }
        unsafe extern "system" fn OpenLinkSource<Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenLinkSource(::core::mem::transmute_copy(&dwlink)).into()
        }
        unsafe extern "system" fn UpdateLink<Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateLink(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&ferrormessage), ::core::mem::transmute_copy(&freserved)).into()
        }
        unsafe extern "system" fn CancelLink<Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelLink(::core::mem::transmute_copy(&dwlink)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNextLink: GetNextLink::<Impl, IMPL_OFFSET>,
            SetLinkUpdateOptions: SetLinkUpdateOptions::<Impl, IMPL_OFFSET>,
            GetLinkUpdateOptions: GetLinkUpdateOptions::<Impl, IMPL_OFFSET>,
            SetLinkSource: SetLinkSource::<Impl, IMPL_OFFSET>,
            GetLinkSource: GetLinkSource::<Impl, IMPL_OFFSET>,
            OpenLinkSource: OpenLinkSource::<Impl, IMPL_OFFSET>,
            UpdateLink: UpdateLink::<Impl, IMPL_OFFSET>,
            CancelLink: CancelLink::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkContainerW_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUILinkContainerW_Vtbl {
        unsafe extern "system" fn GetNextLink<Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextLink(::core::mem::transmute_copy(&dwlink))
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpdwupdateopt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkSource<Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: super::super::Foundation::PWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lpszdisplayname), ::core::mem::transmute_copy(&lenfilename), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&fvalidatesource)).into()
        }
        unsafe extern "system" fn GetLinkSource<Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PWSTR, lplpszshortlinktype: *mut super::super::Foundation::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lplpszdisplayname), ::core::mem::transmute_copy(&lplenfilename), ::core::mem::transmute_copy(&lplpszfulllinktype), ::core::mem::transmute_copy(&lplpszshortlinktype), ::core::mem::transmute_copy(&lpfsourceavailable), ::core::mem::transmute_copy(&lpfisselected)).into()
        }
        unsafe extern "system" fn OpenLinkSource<Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenLinkSource(::core::mem::transmute_copy(&dwlink)).into()
        }
        unsafe extern "system" fn UpdateLink<Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateLink(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&ferrormessage), ::core::mem::transmute_copy(&freserved)).into()
        }
        unsafe extern "system" fn CancelLink<Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelLink(::core::mem::transmute_copy(&dwlink)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNextLink: GetNextLink::<Impl, IMPL_OFFSET>,
            SetLinkUpdateOptions: SetLinkUpdateOptions::<Impl, IMPL_OFFSET>,
            GetLinkUpdateOptions: GetLinkUpdateOptions::<Impl, IMPL_OFFSET>,
            SetLinkSource: SetLinkSource::<Impl, IMPL_OFFSET>,
            GetLinkSource: GetLinkSource::<Impl, IMPL_OFFSET>,
            OpenLinkSource: OpenLinkSource::<Impl, IMPL_OFFSET>,
            UpdateLink: UpdateLink::<Impl, IMPL_OFFSET>,
            CancelLink: CancelLink::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkInfoA_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUILinkInfoA_Vtbl {
        unsafe extern "system" fn GetLastUpdate<Impl: IOleUILinkInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastUpdate(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    *lplastupdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IOleUILinkContainerA_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetLastUpdate: GetLastUpdate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUILinkInfoA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkInfoW_Impl: Sized + IOleUILinkContainerW_Impl {
    fn GetLastUpdate(&mut self, dwlink: u32) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkInfoW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUILinkInfoW_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUILinkInfoW_Vtbl {
        unsafe extern "system" fn GetLastUpdate<Impl: IOleUILinkInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastUpdate(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    *lplastupdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IOleUILinkContainerW_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetLastUpdate: GetLastUpdate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOleUILinkInfoW as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoA_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUIObjInfoA_Vtbl {
        unsafe extern "system" fn GetObjectInfo<Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PSTR, lplpsztype: *mut super::super::Foundation::PSTR, lplpszshorttype: *mut super::super::Foundation::PSTR, lplpszlocation: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpdwobjsize), ::core::mem::transmute_copy(&lplpszlabel), ::core::mem::transmute_copy(&lplpsztype), ::core::mem::transmute_copy(&lplpszshorttype), ::core::mem::transmute_copy(&lplpszlocation)).into()
        }
        unsafe extern "system" fn GetConvertInfo<Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConvertInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpclassid), ::core::mem::transmute_copy(&lpwformat), ::core::mem::transmute_copy(&lpconvertdefaultclassid), ::core::mem::transmute_copy(&lplpclsidexclude), ::core::mem::transmute_copy(&lpcclsidexclude)).into()
        }
        unsafe extern "system" fn ConvertObject<Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertObject(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&clsidnew)).into()
        }
        unsafe extern "system" fn GetViewInfo<Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&phmetapict), ::core::mem::transmute_copy(&pdvaspect), ::core::mem::transmute_copy(&pncurrentscale)).into()
        }
        unsafe extern "system" fn SetViewInfo<Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&hmetapict), ::core::mem::transmute_copy(&dvaspect), ::core::mem::transmute_copy(&ncurrentscale), ::core::mem::transmute_copy(&brelativetoorig)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetObjectInfo: GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetConvertInfo: GetConvertInfo::<Impl, IMPL_OFFSET>,
            ConvertObject: ConvertObject::<Impl, IMPL_OFFSET>,
            GetViewInfo: GetViewInfo::<Impl, IMPL_OFFSET>,
            SetViewInfo: SetViewInfo::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUIObjInfoW_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUIObjInfoW_Vtbl {
        unsafe extern "system" fn GetObjectInfo<Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PWSTR, lplpsztype: *mut super::super::Foundation::PWSTR, lplpszshorttype: *mut super::super::Foundation::PWSTR, lplpszlocation: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpdwobjsize), ::core::mem::transmute_copy(&lplpszlabel), ::core::mem::transmute_copy(&lplpsztype), ::core::mem::transmute_copy(&lplpszshorttype), ::core::mem::transmute_copy(&lplpszlocation)).into()
        }
        unsafe extern "system" fn GetConvertInfo<Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConvertInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpclassid), ::core::mem::transmute_copy(&lpwformat), ::core::mem::transmute_copy(&lpconvertdefaultclassid), ::core::mem::transmute_copy(&lplpclsidexclude), ::core::mem::transmute_copy(&lpcclsidexclude)).into()
        }
        unsafe extern "system" fn ConvertObject<Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertObject(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&clsidnew)).into()
        }
        unsafe extern "system" fn GetViewInfo<Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&phmetapict), ::core::mem::transmute_copy(&pdvaspect), ::core::mem::transmute_copy(&pncurrentscale)).into()
        }
        unsafe extern "system" fn SetViewInfo<Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&hmetapict), ::core::mem::transmute_copy(&dvaspect), ::core::mem::transmute_copy(&ncurrentscale), ::core::mem::transmute_copy(&brelativetoorig)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetObjectInfo: GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetConvertInfo: GetConvertInfo::<Impl, IMPL_OFFSET>,
            ConvertObject: ConvertObject::<Impl, IMPL_OFFSET>,
            GetViewInfo: GetViewInfo::<Impl, IMPL_OFFSET>,
            SetViewInfo: SetViewInfo::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUndoManager_Vtbl {
        unsafe extern "system" fn Open<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute(&ppuu)).into()
        }
        unsafe extern "system" fn Close<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: ::windows::core::RawPtr, fcommit: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(::core::mem::transmute(&ppuu), ::core::mem::transmute_copy(&fcommit)).into()
        }
        unsafe extern "system" fn Add<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn GetOpenParentState<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpenParentState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardFrom<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardFrom(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn UndoTo<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UndoTo(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn RedoTo<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RedoTo(::core::mem::transmute(&puu)).into()
        }
        unsafe extern "system" fn EnumUndoable<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumUndoable() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRedoable<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRedoable() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastUndoDescription<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastUndoDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastRedoDescription<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastRedoDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            GetOpenParentState: GetOpenParentState::<Impl, IMPL_OFFSET>,
            DiscardFrom: DiscardFrom::<Impl, IMPL_OFFSET>,
            UndoTo: UndoTo::<Impl, IMPL_OFFSET>,
            RedoTo: RedoTo::<Impl, IMPL_OFFSET>,
            EnumUndoable: EnumUndoable::<Impl, IMPL_OFFSET>,
            EnumRedoable: EnumRedoable::<Impl, IMPL_OFFSET>,
            GetLastUndoDescription: GetLastUndoDescription::<Impl, IMPL_OFFSET>,
            GetLastRedoDescription: GetLastRedoDescription::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleUndoUnit_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleUndoUnit_Vtbl {
        unsafe extern "system" fn Do<Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pundomanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Do(::core::mem::transmute(&pundomanager)).into()
        }
        unsafe extern "system" fn GetDescription<Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnitType<Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUnitType(::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn OnNextAdd<Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNextAdd().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Do: Do::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            GetUnitType: GetUnitType::<Impl, IMPL_OFFSET>,
            OnNextAdd: OnNextAdd::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOleWindow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOleWindow_Vtbl {
        unsafe extern "system" fn GetWindow<Impl: IOleWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Impl: IOleWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContextSensitiveHelp(::core::mem::transmute_copy(&fentermode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetWindow: GetWindow::<Impl, IMPL_OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IParseDisplayName_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IParseDisplayName_Vtbl {
        unsafe extern "system" fn ParseDisplayName<Impl: IParseDisplayName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseDisplayName(::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&pszdisplayname), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&ppmkout)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ParseDisplayName: ParseDisplayName::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerPropertyBrowsing_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerPropertyBrowsing_Vtbl {
        unsafe extern "system" fn GetDisplayString<Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayString(::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapPropertyToPage<Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapPropertyToPage(::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPredefinedStrings<Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPredefinedStrings(::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&pcastringsout), ::core::mem::transmute_copy(&pcacookiesout)).into()
        }
        unsafe extern "system" fn GetPredefinedValue<Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, dwcookie: u32, pvarout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPredefinedValue(::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&dwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDisplayString: GetDisplayString::<Impl, IMPL_OFFSET>,
            MapPropertyToPage: MapPropertyToPage::<Impl, IMPL_OFFSET>,
            GetPredefinedStrings: GetPredefinedStrings::<Impl, IMPL_OFFSET>,
            GetPredefinedValue: GetPredefinedValue::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistPropertyBag_Vtbl {
        unsafe extern "system" fn InitNew<Impl: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitNew().into()
        }
        unsafe extern "system" fn Load<Impl: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, perrorlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Load(::core::mem::transmute(&ppropbag), ::core::mem::transmute(&perrorlog)).into()
        }
        unsafe extern "system" fn Save<Impl: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save(::core::mem::transmute(&ppropbag), ::core::mem::transmute_copy(&fcleardirty), ::core::mem::transmute_copy(&fsaveallproperties)).into()
        }
        Self {
            base: super::Com::IPersist_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitNew: InitNew::<Impl, IMPL_OFFSET>,
            Load: Load::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistPropertyBag as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistPropertyBag2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistPropertyBag2_Vtbl {
        unsafe extern "system" fn InitNew<Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitNew().into()
        }
        unsafe extern "system" fn Load<Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, perrlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Load(::core::mem::transmute(&ppropbag), ::core::mem::transmute(&perrlog)).into()
        }
        unsafe extern "system" fn Save<Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save(::core::mem::transmute(&ppropbag), ::core::mem::transmute_copy(&fcleardirty), ::core::mem::transmute_copy(&fsaveallproperties)).into()
        }
        unsafe extern "system" fn IsDirty<Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsDirty().into()
        }
        Self {
            base: super::Com::IPersist_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitNew: InitNew::<Impl, IMPL_OFFSET>,
            Load: Load::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            IsDirty: IsDirty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistPropertyBag2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPicture_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPicture_Vtbl {
        unsafe extern "system" fn Handle<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hPal<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phpal: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).hPal() {
                ::core::result::Result::Ok(ok__) => {
                    *phpal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *pwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *pheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Render<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Render(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy), ::core::mem::transmute_copy(&xsrc), ::core::mem::transmute_copy(&ysrc), ::core::mem::transmute_copy(&cxsrc), ::core::mem::transmute_copy(&cysrc), ::core::mem::transmute_copy(&prcwbounds)).into()
        }
        unsafe extern "system" fn set_hPal<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpal: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_hPal(::core::mem::transmute_copy(&hpal)).into()
        }
        unsafe extern "system" fn CurDC<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurDC() {
                ::core::result::Result::Ok(ok__) => {
                    *phdc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPicture<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectPicture(::core::mem::transmute_copy(&hdcin), ::core::mem::transmute_copy(&phdcout), ::core::mem::transmute_copy(&phbmpout)).into()
        }
        unsafe extern "system" fn KeepOriginalFormat<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeepOriginalFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pkeep = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepOriginalFormat(::core::mem::transmute_copy(&keep)).into()
        }
        unsafe extern "system" fn PictureChanged<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PictureChanged().into()
        }
        unsafe extern "system" fn SaveAsFile<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsFile(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&fsavememcopy)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwattr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Handle: Handle::<Impl, IMPL_OFFSET>,
            hPal: hPal::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            Render: Render::<Impl, IMPL_OFFSET>,
            set_hPal: set_hPal::<Impl, IMPL_OFFSET>,
            CurDC: CurDC::<Impl, IMPL_OFFSET>,
            SelectPicture: SelectPicture::<Impl, IMPL_OFFSET>,
            KeepOriginalFormat: KeepOriginalFormat::<Impl, IMPL_OFFSET>,
            SetKeepOriginalFormat: SetKeepOriginalFormat::<Impl, IMPL_OFFSET>,
            PictureChanged: PictureChanged::<Impl, IMPL_OFFSET>,
            SaveAsFile: SaveAsFile::<Impl, IMPL_OFFSET>,
            Attributes: Attributes::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPicture2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPicture2_Vtbl {
        unsafe extern "system" fn Handle<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hPal<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phpal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).hPal() {
                ::core::result::Result::Ok(ok__) => {
                    *phpal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *pwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *pheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Render<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Render(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy), ::core::mem::transmute_copy(&xsrc), ::core::mem::transmute_copy(&ysrc), ::core::mem::transmute_copy(&cxsrc), ::core::mem::transmute_copy(&cysrc), ::core::mem::transmute_copy(&prcwbounds)).into()
        }
        unsafe extern "system" fn set_hPal<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpal: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_hPal(::core::mem::transmute_copy(&hpal)).into()
        }
        unsafe extern "system" fn CurDC<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurDC() {
                ::core::result::Result::Ok(ok__) => {
                    *phdc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPicture<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectPicture(::core::mem::transmute_copy(&hdcin), ::core::mem::transmute_copy(&phdcout), ::core::mem::transmute_copy(&phbmpout)).into()
        }
        unsafe extern "system" fn KeepOriginalFormat<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeepOriginalFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pkeep = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepOriginalFormat(::core::mem::transmute_copy(&keep)).into()
        }
        unsafe extern "system" fn PictureChanged<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PictureChanged().into()
        }
        unsafe extern "system" fn SaveAsFile<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsFile(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&fsavememcopy)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwattr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Handle: Handle::<Impl, IMPL_OFFSET>,
            hPal: hPal::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            Render: Render::<Impl, IMPL_OFFSET>,
            set_hPal: set_hPal::<Impl, IMPL_OFFSET>,
            CurDC: CurDC::<Impl, IMPL_OFFSET>,
            SelectPicture: SelectPicture::<Impl, IMPL_OFFSET>,
            KeepOriginalFormat: KeepOriginalFormat::<Impl, IMPL_OFFSET>,
            SetKeepOriginalFormat: SetKeepOriginalFormat::<Impl, IMPL_OFFSET>,
            PictureChanged: PictureChanged::<Impl, IMPL_OFFSET>,
            SaveAsFile: SaveAsFile::<Impl, IMPL_OFFSET>,
            Attributes: Attributes::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPictureDisp_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPictureDisp_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPictureDisp as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerInactive_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerInactive_Vtbl {
        unsafe extern "system" fn GetActivationPolicy<Impl: IPointerInactive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpolicy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivationPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwpolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInactiveMouseMove<Impl: IPointerInactive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInactiveMouseMove(::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&grfkeystate)).into()
        }
        unsafe extern "system" fn OnInactiveSetCursor<Impl: IPointerInactive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInactiveSetCursor(::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&dwmousemsg), ::core::mem::transmute_copy(&fsetalways)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetActivationPolicy: GetActivationPolicy::<Impl, IMPL_OFFSET>,
            OnInactiveMouseMove: OnInactiveMouseMove::<Impl, IMPL_OFFSET>,
            OnInactiveSetCursor: OnInactiveSetCursor::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint_Vtbl {
        unsafe extern "system" fn SetInitialPageNum<Impl: IPrint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nfirstpage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialPageNum(::core::mem::transmute_copy(&nfirstpage)).into()
        }
        unsafe extern "system" fn GetPageInfo<Impl: IPrint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPageInfo(::core::mem::transmute_copy(&pnfirstpage), ::core::mem::transmute_copy(&pcpages)).into()
        }
        unsafe extern "system" fn Print<Impl: IPrint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: ::windows::core::RawPtr, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Print(::core::mem::transmute_copy(&grfflags), ::core::mem::transmute_copy(&pptd), ::core::mem::transmute_copy(&pppageset), ::core::mem::transmute_copy(&pstgmoptions), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&nfirstpage), ::core::mem::transmute_copy(&pcpagesprinted), ::core::mem::transmute_copy(&pnlastpage)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetInitialPageNum: SetInitialPageNum::<Impl, IMPL_OFFSET>,
            GetPageInfo: GetPageInfo::<Impl, IMPL_OFFSET>,
            Print: Print::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyNotifySink_Vtbl {
        unsafe extern "system" fn OnChanged<Impl: IPropertyNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChanged(::core::mem::transmute_copy(&dispid)).into()
        }
        unsafe extern "system" fn OnRequestEdit<Impl: IPropertyNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRequestEdit(::core::mem::transmute_copy(&dispid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnChanged: OnChanged::<Impl, IMPL_OFFSET>,
            OnRequestEdit: OnRequestEdit::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyPage_Vtbl {
        unsafe extern "system" fn SetPageSite<Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagesite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageSite(::core::mem::transmute(&ppagesite)).into()
        }
        unsafe extern "system" fn Activate<Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&bmodal)).into()
        }
        unsafe extern "system" fn Deactivate<Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate().into()
        }
        unsafe extern "system" fn GetPageInfo<Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppageinfo: *mut PROPPAGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppageinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjects<Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32, ppunk: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjects(::core::mem::transmute_copy(&cobjects), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn Show<Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncmdshow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&ncmdshow)).into()
        }
        unsafe extern "system" fn Move<Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn IsPageDirty<Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPageDirty().into()
        }
        unsafe extern "system" fn Apply<Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Apply().into()
        }
        unsafe extern "system" fn Help<Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelpdir: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Help(::core::mem::transmute_copy(&pszhelpdir)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&pmsg)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetPageSite: SetPageSite::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Deactivate: Deactivate::<Impl, IMPL_OFFSET>,
            GetPageInfo: GetPageInfo::<Impl, IMPL_OFFSET>,
            SetObjects: SetObjects::<Impl, IMPL_OFFSET>,
            Show: Show::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            IsPageDirty: IsPageDirty::<Impl, IMPL_OFFSET>,
            Apply: Apply::<Impl, IMPL_OFFSET>,
            Help: Help::<Impl, IMPL_OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPage2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyPage2_Vtbl {
        unsafe extern "system" fn EditProperty<Impl: IPropertyPage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EditProperty(::core::mem::transmute_copy(&dispid)).into()
        }
        Self { base: IPropertyPage_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EditProperty: EditProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyPage2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPageSite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyPageSite_Vtbl {
        unsafe extern "system" fn OnStatusChange<Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: PROPPAGESTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatusChange(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetLocaleID<Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocaleid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocaleID() {
                ::core::result::Result::Ok(ok__) => {
                    *plocaleid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageContainer<Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&pmsg)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStatusChange: OnStatusChange::<Impl, IMPL_OFFSET>,
            GetLocaleID: GetLocaleID::<Impl, IMPL_OFFSET>,
            GetPageContainer: GetPageContainer::<Impl, IMPL_OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectFocus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtectFocus_Vtbl {
        unsafe extern "system" fn AllowFocusChange<Impl: IProtectFocus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pfallow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AllowFocusChange: AllowFocusChange::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedModeMenuServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtectedModeMenuServices_Vtbl {
        unsafe extern "system" fn CreateMenu<Impl: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMenu() {
                ::core::result::Result::Ok(ok__) => {
                    *phmenu = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMenu<Impl: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmodulename: super::super::Foundation::PWSTR, pszmenuname: super::super::Foundation::PWSTR, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadMenu(::core::mem::transmute_copy(&pszmodulename), ::core::mem::transmute_copy(&pszmenuname)) {
                ::core::result::Result::Ok(ok__) => {
                    *phmenu = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMenuID<Impl: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmodulename: super::super::Foundation::PWSTR, wresourceid: u16, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadMenuID(::core::mem::transmute_copy(&pszmodulename), ::core::mem::transmute_copy(&wresourceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *phmenu = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateMenu: CreateMenu::<Impl, IMPL_OFFSET>,
            LoadMenu: LoadMenu::<Impl, IMPL_OFFSET>,
            LoadMenuID: LoadMenuID::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideClassInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideClassInfo_Vtbl {
        unsafe extern "system" fn GetClassInfo<Impl: IProvideClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppti: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppti = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetClassInfo: GetClassInfo::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideClassInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideClassInfo2_Vtbl {
        unsafe extern "system" fn GetGUID<Impl: IProvideClassInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwguidkind: u32, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID(::core::mem::transmute_copy(&dwguidkind)) {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IProvideClassInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetGUID: GetGUID::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideClassInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideMultipleClassInfo_Impl: Sized + IProvideClassInfo_Impl + IProvideClassInfo2_Impl {
    fn GetMultiTypeInfoCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetInfoOfIndex(&mut self, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut ::core::option::Option<super::Com::ITypeInfo>, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows::core::GUID, piidsource: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IProvideMultipleClassInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideMultipleClassInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideMultipleClassInfo_Vtbl {
        unsafe extern "system" fn GetMultiTypeInfoCount<Impl: IProvideMultipleClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcti: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMultiTypeInfoCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcti = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfoOfIndex<Impl: IProvideMultipleClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut ::windows::core::RawPtr, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows::core::GUID, piidsource: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInfoOfIndex(::core::mem::transmute_copy(&iti), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppticoclass), ::core::mem::transmute_copy(&pdwtiflags), ::core::mem::transmute_copy(&pcdispidreserved), ::core::mem::transmute_copy(&piidprimary), ::core::mem::transmute_copy(&piidsource)).into()
        }
        Self {
            base: IProvideClassInfo2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMultiTypeInfoCount: GetMultiTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetInfoOfIndex: GetInfoOfIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideMultipleClassInfo as ::windows::core::Interface>::IID
    }
}
pub trait IProvideRuntimeContext_Impl: Sized {
    fn GetCurrentSourceContext(&mut self, pdwcontext: *mut usize, pfexecutingglobalcode: *mut i16) -> ::windows::core::Result<()>;
}
impl IProvideRuntimeContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideRuntimeContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideRuntimeContext_Vtbl {
        unsafe extern "system" fn GetCurrentSourceContext<Impl: IProvideRuntimeContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcontext: *mut usize, pfexecutingglobalcode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentSourceContext(::core::mem::transmute_copy(&pdwcontext), ::core::mem::transmute_copy(&pfexecutingglobalcode)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCurrentSourceContext: GetCurrentSourceContext::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuickActivate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuickActivate_Vtbl {
        unsafe extern "system" fn QuickActivate<Impl: IQuickActivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QuickActivate(::core::mem::transmute_copy(&pqacontainer), ::core::mem::transmute_copy(&pqacontrol)).into()
        }
        unsafe extern "system" fn SetContentExtent<Impl: IQuickActivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentExtent(::core::mem::transmute_copy(&psizel)).into()
        }
        unsafe extern "system" fn GetContentExtent<Impl: IQuickActivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentExtent() {
                ::core::result::Result::Ok(ok__) => {
                    *psizel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QuickActivate: QuickActivate::<Impl, IMPL_OFFSET>,
            SetContentExtent: SetContentExtent::<Impl, IMPL_OFFSET>,
            GetContentExtent: GetContentExtent::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecordInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRecordInfo_Vtbl {
        unsafe extern "system" fn RecordInit<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecordInit(::core::mem::transmute_copy(&pvnew)).into()
        }
        unsafe extern "system" fn RecordClear<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecordClear(::core::mem::transmute_copy(&pvexisting)).into()
        }
        unsafe extern "system" fn RecordCopy<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecordCopy(::core::mem::transmute_copy(&pvexisting), ::core::mem::transmute_copy(&pvnew)).into()
        }
        unsafe extern "system" fn GetGuid<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pptypeinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetField<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetField(::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&szfieldname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarfield = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFieldNoCopy<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *mut super::Com::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFieldNoCopy(::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&szfieldname), ::core::mem::transmute_copy(&pvarfield), ::core::mem::transmute_copy(&ppvdatacarray)).into()
        }
        unsafe extern "system" fn PutField<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutField(::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&szfieldname), ::core::mem::transmute_copy(&pvarfield)).into()
        }
        unsafe extern "system" fn PutFieldNoCopy<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: super::super::Foundation::PWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutFieldNoCopy(::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&szfieldname), ::core::mem::transmute_copy(&pvarfield)).into()
        }
        unsafe extern "system" fn GetFieldNames<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnames: *mut u32, rgbstrnames: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFieldNames(::core::mem::transmute_copy(&pcnames), ::core::mem::transmute_copy(&rgbstrnames)).into()
        }
        unsafe extern "system" fn IsMatchingType<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordinfo: ::windows::core::RawPtr) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsMatchingType(::core::mem::transmute(&precordinfo))
        }
        unsafe extern "system" fn RecordCreate<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecordCreate()
        }
        unsafe extern "system" fn RecordCreateCopy<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecordCreateCopy(::core::mem::transmute_copy(&pvsource), ::core::mem::transmute_copy(&ppvdest)).into()
        }
        unsafe extern "system" fn RecordDestroy<Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvrecord: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecordDestroy(::core::mem::transmute_copy(&pvrecord)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RecordInit: RecordInit::<Impl, IMPL_OFFSET>,
            RecordClear: RecordClear::<Impl, IMPL_OFFSET>,
            RecordCopy: RecordCopy::<Impl, IMPL_OFFSET>,
            GetGuid: GetGuid::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetSize: GetSize::<Impl, IMPL_OFFSET>,
            GetTypeInfo: GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetField: GetField::<Impl, IMPL_OFFSET>,
            GetFieldNoCopy: GetFieldNoCopy::<Impl, IMPL_OFFSET>,
            PutField: PutField::<Impl, IMPL_OFFSET>,
            PutFieldNoCopy: PutFieldNoCopy::<Impl, IMPL_OFFSET>,
            GetFieldNames: GetFieldNames::<Impl, IMPL_OFFSET>,
            IsMatchingType: IsMatchingType::<Impl, IMPL_OFFSET>,
            RecordCreate: RecordCreate::<Impl, IMPL_OFFSET>,
            RecordCreateCopy: RecordCreateCopy::<Impl, IMPL_OFFSET>,
            RecordDestroy: RecordDestroy::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleFrameSite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleFrameSite_Vtbl {
        unsafe extern "system" fn PreMessageFilter<Impl: ISimpleFrameSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreMessageFilter(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wp), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&plresult), ::core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn PostMessageFilter<Impl: ISimpleFrameSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostMessageFilter(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wp), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&plresult), ::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PreMessageFilter: PreMessageFilter::<Impl, IMPL_OFFSET>,
            PostMessageFilter: PostMessageFilter::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpecifyPropertyPages_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpecifyPropertyPages_Vtbl {
        unsafe extern "system" fn GetPages<Impl: ISpecifyPropertyPages_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppages: *mut CAUUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPages() {
                ::core::result::Result::Ok(ok__) => {
                    *ppages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetPages: GetPages::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeChangeEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeChangeEvents_Vtbl {
        unsafe extern "system" fn RequestTypeChange<Impl: ITypeChangeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfobefore: ::windows::core::RawPtr, pstrname: super::super::Foundation::PWSTR, pfcancel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestTypeChange(::core::mem::transmute_copy(&changekind), ::core::mem::transmute(&ptinfobefore), ::core::mem::transmute_copy(&pstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfcancel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AfterTypeChange<Impl: ITypeChangeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfoafter: ::windows::core::RawPtr, pstrname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AfterTypeChange(::core::mem::transmute_copy(&changekind), ::core::mem::transmute(&ptinfoafter), ::core::mem::transmute_copy(&pstrname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RequestTypeChange: RequestTypeChange::<Impl, IMPL_OFFSET>,
            AfterTypeChange: AfterTypeChange::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeFactory_Vtbl {
        unsafe extern "system" fn CreateFromTypeInfo<Impl: ITypeFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeinfo: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromTypeInfo(::core::mem::transmute(&ptypeinfo), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppv = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateFromTypeInfo: CreateFromTypeInfo::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeMarshal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeMarshal_Vtbl {
        unsafe extern "system" fn Size<Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, psize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size(::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *psize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Marshal<Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Marshal(::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&cbbufferlength), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbwritten)).into()
        }
        unsafe extern "system" fn Unmarshal<Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *mut ::core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmarshal(::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cbbufferlength), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbread)).into()
        }
        unsafe extern "system" fn Free<Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Free(::core::mem::transmute_copy(&pvtype)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Size: Size::<Impl, IMPL_OFFSET>,
            Marshal: Marshal::<Impl, IMPL_OFFSET>,
            Unmarshal: Unmarshal::<Impl, IMPL_OFFSET>,
            Free: Free::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBFormat_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBFormat_Vtbl {
        unsafe extern "system" fn Format<Impl: IVBFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdata: *mut super::Com::VARIANT, bstrformat: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Format(::core::mem::transmute_copy(&vdata), ::core::mem::transmute_copy(&bstrformat), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&sfirstdayofweek), ::core::mem::transmute_copy(&sfirstweekofyear), ::core::mem::transmute_copy(&rcb)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Format: Format::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBGetControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBGetControl_Vtbl {
        unsafe extern "system" fn EnumControls<Impl: IVBGetControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwolecontf: OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS, ppenumunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumControls(::core::mem::transmute_copy(&dwolecontf), ::core::mem::transmute_copy(&dwwhich)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), EnumControls: EnumControls::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVariantChangeType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVariantChangeType_Vtbl {
        unsafe extern "system" fn ChangeType<Impl: IVariantChangeType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardst: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, vtnew: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeType(::core::mem::transmute_copy(&pvardst), ::core::mem::transmute_copy(&pvarsrc), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&vtnew)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ChangeType: ChangeType::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewObject_Vtbl {
        unsafe extern "system" fn Draw<Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Draw(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hdctargetdev), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&lprcbounds), ::core::mem::transmute_copy(&lprcwbounds), ::core::mem::transmute_copy(&pfncontinue), ::core::mem::transmute_copy(&dwcontinue)).into()
        }
        unsafe extern "system" fn GetColorSet<Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorSet(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&ppcolorset)).into()
        }
        unsafe extern "system" fn Freeze<Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Freeze(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&pdwfreeze)).into()
        }
        unsafe extern "system" fn Unfreeze<Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfreeze: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unfreeze(::core::mem::transmute_copy(&dwfreeze)).into()
        }
        unsafe extern "system" fn SetAdvise<Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aspects: u32, advf: u32, padvsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdvise(::core::mem::transmute_copy(&aspects), ::core::mem::transmute_copy(&advf), ::core::mem::transmute(&padvsink)).into()
        }
        unsafe extern "system" fn GetAdvise<Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAdvise(::core::mem::transmute_copy(&paspects), ::core::mem::transmute_copy(&padvf), ::core::mem::transmute_copy(&ppadvsink)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Draw: Draw::<Impl, IMPL_OFFSET>,
            GetColorSet: GetColorSet::<Impl, IMPL_OFFSET>,
            Freeze: Freeze::<Impl, IMPL_OFFSET>,
            Unfreeze: Unfreeze::<Impl, IMPL_OFFSET>,
            SetAdvise: SetAdvise::<Impl, IMPL_OFFSET>,
            GetAdvise: GetAdvise::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObject2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewObject2_Vtbl {
        unsafe extern "system" fn GetExtent<Impl: IViewObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: u32, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, lpsizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtent(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&ptd)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpsizel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IViewObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetExtent: GetExtent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewObject2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewObjectEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewObjectEx_Vtbl {
        unsafe extern "system" fn GetRect<Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prect: *mut super::super::Foundation::RECTL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRect(::core::mem::transmute_copy(&dwaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    *prect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewStatus<Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHitPoint<Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHitPoint(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&ptlloc), ::core::mem::transmute_copy(&lclosehint)) {
                ::core::result::Result::Ok(ok__) => {
                    *phitresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHitRect<Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHitRect(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&prectloc), ::core::mem::transmute_copy(&lclosehint)) {
                ::core::result::Result::Ok(ok__) => {
                    *phitresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNaturalExtent<Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const ExtentInfo, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNaturalExtent(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&pextentinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *psizel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IViewObject2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetRect: GetRect::<Impl, IMPL_OFFSET>,
            GetViewStatus: GetViewStatus::<Impl, IMPL_OFFSET>,
            QueryHitPoint: QueryHitPoint::<Impl, IMPL_OFFSET>,
            QueryHitRect: QueryHitRect::<Impl, IMPL_OFFSET>,
            GetNaturalExtent: GetNaturalExtent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewObjectEx as ::windows::core::Interface>::IID
    }
}
pub trait IZoomEvents_Impl: Sized {
    fn OnZoomPercentChanged(&mut self, ulzoompercent: u32) -> ::windows::core::Result<()>;
}
impl IZoomEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoomEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IZoomEvents_Vtbl {
        unsafe extern "system" fn OnZoomPercentChanged<Impl: IZoomEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulzoompercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnZoomPercentChanged(::core::mem::transmute_copy(&ulzoompercent)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnZoomPercentChanged: OnZoomPercentChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IZoomEvents as ::windows::core::Interface>::IID
    }
}
