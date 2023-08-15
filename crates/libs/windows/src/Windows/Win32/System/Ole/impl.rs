#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSinkEx_Impl: Sized + super::Com::IAdviseSink_Impl {
    fn OnViewStatusChange(&self, dwviewstatus: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::RuntimeName for IAdviseSinkEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSinkEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdviseSinkEx_Impl, const OFFSET: isize>() -> IAdviseSinkEx_Vtbl {
        unsafe extern "system" fn OnViewStatusChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdviseSinkEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwviewstatus: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnViewStatusChange(::core::mem::transmute_copy(&dwviewstatus))
        }
        Self { base__: super::Com::IAdviseSink_Vtbl::new::<Identity, Impl, OFFSET>(), OnViewStatusChange: OnViewStatusChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAdviseSinkEx as ::windows_core::ComInterface>::IID || iid == &<super::Com::IAdviseSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait ICanHandleException_Impl: Sized {
    fn CanHandleException(&self, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICanHandleException {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ICanHandleException_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICanHandleException_Impl, const OFFSET: isize>() -> ICanHandleException_Vtbl {
        unsafe extern "system" fn CanHandleException<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICanHandleException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanHandleException(::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&pvar)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CanHandleException: CanHandleException::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICanHandleException as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IClassFactory2_Impl: Sized + super::Com::IClassFactory_Impl {
    fn GetLicInfo(&self, plicinfo: *mut LICINFO) -> ::windows_core::Result<()>;
    fn RequestLicKey(&self, dwreserved: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateInstanceLic(&self, punkouter: ::core::option::Option<&::windows_core::IUnknown>, punkreserved: ::core::option::Option<::windows_core::IUnknown>, riid: *const ::windows_core::GUID, bstrkey: &::windows_core::BSTR, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IClassFactory2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IClassFactory2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IClassFactory2_Impl, const OFFSET: isize>() -> IClassFactory2_Vtbl {
        unsafe extern "system" fn GetLicInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IClassFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plicinfo: *mut LICINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLicInfo(::core::mem::transmute_copy(&plicinfo)).into()
        }
        unsafe extern "system" fn RequestLicKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IClassFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, pbstrkey: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestLicKey(::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceLic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IClassFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, bstrkey: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstanceLic(::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute(&punkreserved), ::core::mem::transmute_copy(&riid), ::core::mem::transmute(&bstrkey), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        Self {
            base__: super::Com::IClassFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLicInfo: GetLicInfo::<Identity, Impl, OFFSET>,
            RequestLicKey: RequestLicKey::<Identity, Impl, OFFSET>,
            CreateInstanceLic: CreateInstanceLic::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IClassFactory2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IClassFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IContinue_Impl: Sized {
    fn FContinue(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IContinue {}
impl IContinue_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContinue_Impl, const OFFSET: isize>() -> IContinue_Vtbl {
        unsafe extern "system" fn FContinue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContinue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FContinue().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FContinue: FContinue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IContinue as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IContinueCallback_Impl: Sized {
    fn FContinue(&self) -> ::windows_core::Result<()>;
    fn FContinuePrinting(&self, ncntprinted: i32, ncurpage: i32, pwszprintstatus: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IContinueCallback {}
impl IContinueCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContinueCallback_Impl, const OFFSET: isize>() -> IContinueCallback_Vtbl {
        unsafe extern "system" fn FContinue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContinueCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FContinue().into()
        }
        unsafe extern "system" fn FContinuePrinting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContinueCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncntprinted: i32, ncurpage: i32, pwszprintstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FContinuePrinting(::core::mem::transmute_copy(&ncntprinted), ::core::mem::transmute_copy(&ncurpage), ::core::mem::transmute(&pwszprintstatus)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FContinue: FContinue::<Identity, Impl, OFFSET>,
            FContinuePrinting: FContinuePrinting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IContinueCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait ICreateErrorInfo_Impl: Sized {
    fn SetGUID(&self, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetSource(&self, szsource: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetDescription(&self, szdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHelpFile(&self, szhelpfile: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICreateErrorInfo {}
impl ICreateErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>() -> ICreateErrorInfo_Vtbl {
        unsafe extern "system" fn SetGUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGUID(::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn SetSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsource: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSource(::core::mem::transmute(&szsource)).into()
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&szdescription)).into()
        }
        unsafe extern "system" fn SetHelpFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhelpfile: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHelpFile(::core::mem::transmute(&szhelpfile)).into()
        }
        unsafe extern "system" fn SetHelpContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHelpContext(::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGUID: SetGUID::<Identity, Impl, OFFSET>,
            SetSource: SetSource::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            SetHelpFile: SetHelpFile::<Identity, Impl, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICreateErrorInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait ICreateTypeInfo_Impl: Sized {
    fn SetGuid(&self, guid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetTypeFlags(&self, utypeflags: u32) -> ::windows_core::Result<()>;
    fn SetDocString(&self, pstrdoc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows_core::Result<()>;
    fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> ::windows_core::Result<()>;
    fn AddRefTypeInfo(&self, ptinfo: ::core::option::Option<&super::Com::ITypeInfo>, phreftype: *const u32) -> ::windows_core::Result<()>;
    fn AddFuncDesc(&self, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows_core::Result<()>;
    fn AddImplType(&self, index: u32, hreftype: u32) -> ::windows_core::Result<()>;
    fn SetImplTypeFlags(&self, index: u32, impltypeflags: super::Com::IMPLTYPEFLAGS) -> ::windows_core::Result<()>;
    fn SetAlignment(&self, cbalignment: u16) -> ::windows_core::Result<()>;
    fn SetSchema(&self, pstrschema: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddVarDesc(&self, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows_core::Result<()>;
    fn SetFuncAndParamNames(&self, index: u32, rgsznames: *const ::windows_core::PCWSTR, cnames: u32) -> ::windows_core::Result<()>;
    fn SetVarName(&self, index: u32, szname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetTypeDescAlias(&self, ptdescalias: *const super::Com::TYPEDESC) -> ::windows_core::Result<()>;
    fn DefineFuncAsDllEntry(&self, index: u32, szdllname: &::windows_core::PCWSTR, szprocname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetFuncDocString(&self, index: u32, szdocstring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetVarDocString(&self, index: u32, szdocstring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetFuncHelpContext(&self, index: u32, dwhelpcontext: u32) -> ::windows_core::Result<()>;
    fn SetVarHelpContext(&self, index: u32, dwhelpcontext: u32) -> ::windows_core::Result<()>;
    fn SetMops(&self, index: u32, bstrmops: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetTypeIdldesc(&self, pidldesc: *const super::Com::IDLDESC) -> ::windows_core::Result<()>;
    fn LayOut(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICreateTypeInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ICreateTypeInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>() -> ICreateTypeInfo_Vtbl {
        unsafe extern "system" fn SetGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGuid(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn SetTypeFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utypeflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTypeFlags(::core::mem::transmute_copy(&utypeflags)).into()
        }
        unsafe extern "system" fn SetDocString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdoc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDocString(::core::mem::transmute(&pstrdoc)).into()
        }
        unsafe extern "system" fn SetHelpContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHelpContext(::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVersion(::core::mem::transmute_copy(&wmajorvernum), ::core::mem::transmute_copy(&wminorvernum)).into()
        }
        unsafe extern "system" fn AddRefTypeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptinfo: *mut ::core::ffi::c_void, phreftype: *const u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRefTypeInfo(::windows_core::from_raw_borrowed(&ptinfo), ::core::mem::transmute_copy(&phreftype)).into()
        }
        unsafe extern "system" fn AddFuncDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFuncDesc(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pfuncdesc)).into()
        }
        unsafe extern "system" fn AddImplType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, hreftype: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddImplType(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&hreftype)).into()
        }
        unsafe extern "system" fn SetImplTypeFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, impltypeflags: super::Com::IMPLTYPEFLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetImplTypeFlags(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&impltypeflags)).into()
        }
        unsafe extern "system" fn SetAlignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbalignment: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAlignment(::core::mem::transmute_copy(&cbalignment)).into()
        }
        unsafe extern "system" fn SetSchema<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrschema: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSchema(::core::mem::transmute(&pstrschema)).into()
        }
        unsafe extern "system" fn AddVarDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddVarDesc(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pvardesc)).into()
        }
        unsafe extern "system" fn SetFuncAndParamNames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, rgsznames: *const ::windows_core::PCWSTR, cnames: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFuncAndParamNames(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames)).into()
        }
        unsafe extern "system" fn SetVarName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVarName(::core::mem::transmute_copy(&index), ::core::mem::transmute(&szname)).into()
        }
        unsafe extern "system" fn SetTypeDescAlias<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptdescalias: *const super::Com::TYPEDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTypeDescAlias(::core::mem::transmute_copy(&ptdescalias)).into()
        }
        unsafe extern "system" fn DefineFuncAsDllEntry<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdllname: ::windows_core::PCWSTR, szprocname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DefineFuncAsDllEntry(::core::mem::transmute_copy(&index), ::core::mem::transmute(&szdllname), ::core::mem::transmute(&szprocname)).into()
        }
        unsafe extern "system" fn SetFuncDocString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFuncDocString(::core::mem::transmute_copy(&index), ::core::mem::transmute(&szdocstring)).into()
        }
        unsafe extern "system" fn SetVarDocString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVarDocString(::core::mem::transmute_copy(&index), ::core::mem::transmute(&szdocstring)).into()
        }
        unsafe extern "system" fn SetFuncHelpContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFuncHelpContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetVarHelpContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVarHelpContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetMops<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, bstrmops: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMops(::core::mem::transmute_copy(&index), ::core::mem::transmute(&bstrmops)).into()
        }
        unsafe extern "system" fn SetTypeIdldesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidldesc: *const super::Com::IDLDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTypeIdldesc(::core::mem::transmute_copy(&pidldesc)).into()
        }
        unsafe extern "system" fn LayOut<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LayOut().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICreateTypeInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait ICreateTypeInfo2_Impl: Sized + ICreateTypeInfo_Impl {
    fn DeleteFuncDesc(&self, index: u32) -> ::windows_core::Result<()>;
    fn DeleteFuncDescByMemId(&self, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows_core::Result<()>;
    fn DeleteVarDesc(&self, index: u32) -> ::windows_core::Result<()>;
    fn DeleteVarDescByMemId(&self, memid: i32) -> ::windows_core::Result<()>;
    fn DeleteImplType(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetCustData(&self, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetFuncCustData(&self, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetVarCustData(&self, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetImplTypeCustData(&self, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> ::windows_core::Result<()>;
    fn SetFuncHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> ::windows_core::Result<()>;
    fn SetVarHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> ::windows_core::Result<()>;
    fn Invalidate(&self) -> ::windows_core::Result<()>;
    fn SetName(&self, szname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICreateTypeInfo2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ICreateTypeInfo2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>() -> ICreateTypeInfo2_Vtbl {
        unsafe extern "system" fn DeleteFuncDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteFuncDesc(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn DeleteFuncDescByMemId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteFuncDescByMemId(::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&invkind)).into()
        }
        unsafe extern "system" fn DeleteVarDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteVarDesc(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn DeleteVarDescByMemId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteVarDescByMemId(::core::mem::transmute_copy(&memid)).into()
        }
        unsafe extern "system" fn DeleteImplType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteImplType(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetCustData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCustData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetFuncCustData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFuncCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetParamCustData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParamCustData(::core::mem::transmute_copy(&indexfunc), ::core::mem::transmute_copy(&indexparam), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetVarCustData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVarCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetImplTypeCustData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetImplTypeCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHelpStringContext(::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetFuncHelpStringContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFuncHelpStringContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetVarHelpStringContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVarHelpStringContext(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn Invalidate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invalidate().into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&szname)).into()
        }
        Self {
            base__: ICreateTypeInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICreateTypeInfo2 as ::windows_core::ComInterface>::IID || iid == &<ICreateTypeInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ICreateTypeLib_Impl: Sized {
    fn CreateTypeInfo(&self, szname: &::windows_core::PCWSTR, tkind: super::Com::TYPEKIND) -> ::windows_core::Result<ICreateTypeInfo>;
    fn SetName(&self, szname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> ::windows_core::Result<()>;
    fn SetGuid(&self, guid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetDocString(&self, szdoc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHelpFileName(&self, szhelpfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows_core::Result<()>;
    fn SetLcid(&self, lcid: u32) -> ::windows_core::Result<()>;
    fn SetLibFlags(&self, ulibflags: u32) -> ::windows_core::Result<()>;
    fn SaveAllChanges(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ICreateTypeLib {}
#[cfg(feature = "Win32_System_Com")]
impl ICreateTypeLib_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: isize>() -> ICreateTypeLib_Vtbl {
        unsafe extern "system" fn CreateTypeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, tkind: super::Com::TYPEKIND, ppctinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTypeInfo(::core::mem::transmute(&szname), ::core::mem::transmute_copy(&tkind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppctinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&szname)).into()
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVersion(::core::mem::transmute_copy(&wmajorvernum), ::core::mem::transmute_copy(&wminorvernum)).into()
        }
        unsafe extern "system" fn SetGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGuid(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn SetDocString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdoc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDocString(::core::mem::transmute(&szdoc)).into()
        }
        unsafe extern "system" fn SetHelpFileName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhelpfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHelpFileName(::core::mem::transmute(&szhelpfilename)).into()
        }
        unsafe extern "system" fn SetHelpContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHelpContext(::core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetLcid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLcid(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn SetLibFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulibflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLibFlags(::core::mem::transmute_copy(&ulibflags)).into()
        }
        unsafe extern "system" fn SaveAllChanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveAllChanges().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICreateTypeLib as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait ICreateTypeLib2_Impl: Sized + ICreateTypeLib_Impl {
    fn DeleteTypeInfo(&self, szname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetCustData(&self, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> ::windows_core::Result<()>;
    fn SetHelpStringDll(&self, szfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICreateTypeLib2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ICreateTypeLib2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib2_Impl, const OFFSET: isize>() -> ICreateTypeLib2_Vtbl {
        unsafe extern "system" fn DeleteTypeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTypeInfo(::core::mem::transmute(&szname)).into()
        }
        unsafe extern "system" fn SetCustData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCustData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHelpStringContext(::core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetHelpStringDll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHelpStringDll(::core::mem::transmute(&szfilename)).into()
        }
        Self {
            base__: ICreateTypeLib_Vtbl::new::<Identity, Impl, OFFSET>(),
            DeleteTypeInfo: DeleteTypeInfo::<Identity, Impl, OFFSET>,
            SetCustData: SetCustData::<Identity, Impl, OFFSET>,
            SetHelpStringContext: SetHelpStringContext::<Identity, Impl, OFFSET>,
            SetHelpStringDll: SetHelpStringDll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICreateTypeLib2 as ::windows_core::ComInterface>::IID || iid == &<ICreateTypeLib as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IDispError_Impl: Sized {
    fn QueryErrorInfo(&self, guiderrortype: &::windows_core::GUID) -> ::windows_core::Result<IDispError>;
    fn GetNext(&self) -> ::windows_core::Result<IDispError>;
    fn GetHresult(&self) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetSource(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetHelpInfo(&self, pbstrfilename: *mut ::windows_core::BSTR, pdwcontext: *mut u32) -> ::windows_core::Result<()>;
    fn GetDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for IDispError {}
impl IDispError_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: isize>() -> IDispError_Vtbl {
        unsafe extern "system" fn QueryErrorInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guiderrortype: ::windows_core::GUID, ppde: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryErrorInfo(::core::mem::transmute(&guiderrortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppde, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppde: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppde, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHresult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHresult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcontext: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHelpInfo(::core::mem::transmute_copy(&pbstrfilename), ::core::mem::transmute_copy(&pdwcontext)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryErrorInfo: QueryErrorInfo::<Identity, Impl, OFFSET>,
            GetNext: GetNext::<Identity, Impl, OFFSET>,
            GetHresult: GetHresult::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDispError as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IDispatchEx_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetDispID(&self, bstrname: &::windows_core::BSTR, grfdex: u32) -> ::windows_core::Result<i32>;
    fn InvokeEx(&self, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut super::Variant::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: ::core::option::Option<&super::Com::IServiceProvider>) -> ::windows_core::Result<()>;
    fn DeleteMemberByName(&self, bstrname: &::windows_core::BSTR, grfdex: u32) -> ::windows_core::Result<()>;
    fn DeleteMemberByDispID(&self, id: i32) -> ::windows_core::Result<()>;
    fn GetMemberProperties(&self, id: i32, grfdexfetch: u32) -> ::windows_core::Result<FDEX_PROP_FLAGS>;
    fn GetMemberName(&self, id: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetNextDispID(&self, grfdex: u32, id: i32) -> ::windows_core::Result<i32>;
    fn GetNameSpaceParent(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDispatchEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl IDispatchEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: isize>() -> IDispatchEx_Vtbl {
        unsafe extern "system" fn GetDispID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, grfdex: u32, pid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDispID(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&grfdex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut super::Variant::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvokeEx(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdp), ::core::mem::transmute_copy(&pvarres), ::core::mem::transmute_copy(&pei), ::windows_core::from_raw_borrowed(&pspcaller)).into()
        }
        unsafe extern "system" fn DeleteMemberByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, grfdex: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMemberByName(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&grfdex)).into()
        }
        unsafe extern "system" fn DeleteMemberByDispID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMemberByDispID(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetMemberProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, grfdexfetch: u32, pgrfdex: *mut FDEX_PROP_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMemberProperties(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&grfdexfetch)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgrfdex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMemberName(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextDispID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfdex: u32, id: i32, pid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNextDispID(::core::mem::transmute_copy(&grfdex), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameSpaceParent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNameSpaceParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDispatchEx as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemServices\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub trait IDropSource_Impl: Sized {
    fn QueryContinueDrag(&self, fescapepressed: super::super::Foundation::BOOL, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS) -> ::windows_core::HRESULT;
    fn GiveFeedback(&self, dweffect: DROPEFFECT) -> ::windows_core::HRESULT;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::windows_core::RuntimeName for IDropSource {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl IDropSource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropSource_Impl, const OFFSET: isize>() -> IDropSource_Vtbl {
        unsafe extern "system" fn QueryContinueDrag<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fescapepressed: super::super::Foundation::BOOL, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryContinueDrag(::core::mem::transmute_copy(&fescapepressed), ::core::mem::transmute_copy(&grfkeystate))
        }
        unsafe extern "system" fn GiveFeedback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweffect: DROPEFFECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GiveFeedback(::core::mem::transmute_copy(&dweffect))
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryContinueDrag: QueryContinueDrag::<Identity, Impl, OFFSET>,
            GiveFeedback: GiveFeedback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDropSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDropSourceNotify_Impl: Sized {
    fn DragEnterTarget(&self, hwndtarget: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn DragLeaveTarget(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDropSourceNotify {}
#[cfg(feature = "Win32_Foundation")]
impl IDropSourceNotify_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropSourceNotify_Impl, const OFFSET: isize>() -> IDropSourceNotify_Vtbl {
        unsafe extern "system" fn DragEnterTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropSourceNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndtarget: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DragEnterTarget(::core::mem::transmute_copy(&hwndtarget)).into()
        }
        unsafe extern "system" fn DragLeaveTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropSourceNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DragLeaveTarget().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DragEnterTarget: DragEnterTarget::<Identity, Impl, OFFSET>,
            DragLeaveTarget: DragLeaveTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDropSourceNotify as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_SystemServices\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
pub trait IDropTarget_Impl: Sized {
    fn DragEnter(&self, pdataobj: ::core::option::Option<&super::Com::IDataObject>, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::Result<()>;
    fn DragOver(&self, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::Result<()>;
    fn DragLeave(&self) -> ::windows_core::Result<()>;
    fn Drop(&self, pdataobj: ::core::option::Option<&super::Com::IDataObject>, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
impl ::windows_core::RuntimeName for IDropTarget {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
impl IDropTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropTarget_Impl, const OFFSET: isize>() -> IDropTarget_Vtbl {
        unsafe extern "system" fn DragEnter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobj: *mut ::core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DragEnter(::windows_core::from_raw_borrowed(&pdataobj), ::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&pdweffect)).into()
        }
        unsafe extern "system" fn DragOver<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DragOver(::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&pdweffect)).into()
        }
        unsafe extern "system" fn DragLeave<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DragLeave().into()
        }
        unsafe extern "system" fn Drop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobj: *mut ::core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Drop(::windows_core::from_raw_borrowed(&pdataobj), ::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&pdweffect)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DragEnter: DragEnter::<Identity, Impl, OFFSET>,
            DragOver: DragOver::<Identity, Impl, OFFSET>,
            DragLeave: DragLeave::<Identity, Impl, OFFSET>,
            Drop: Drop::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDropTarget as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnterpriseDropTarget_Impl: Sized {
    fn SetDropSourceEnterpriseId(&self, identity: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn IsEvaluatingEdpPolicy(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IEnterpriseDropTarget {}
#[cfg(feature = "Win32_Foundation")]
impl IEnterpriseDropTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnterpriseDropTarget_Impl, const OFFSET: isize>() -> IEnterpriseDropTarget_Vtbl {
        unsafe extern "system" fn SetDropSourceEnterpriseId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnterpriseDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDropSourceEnterpriseId(::core::mem::transmute(&identity)).into()
        }
        unsafe extern "system" fn IsEvaluatingEdpPolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnterpriseDropTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEvaluatingEdpPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDropSourceEnterpriseId: SetDropSourceEnterpriseId::<Identity, Impl, OFFSET>,
            IsEvaluatingEdpPolicy: IsEvaluatingEdpPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnterpriseDropTarget as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IEnumOLEVERB_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumOLEVERB>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows_core::RuntimeName for IEnumOLEVERB {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IEnumOLEVERB_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOLEVERB_Impl, const OFFSET: isize>() -> IEnumOLEVERB_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumOLEVERB as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IEnumOleDocumentViews_Impl: Sized {
    fn Next(&self, cviews: u32, rgpview: *mut ::core::option::Option<IOleDocumentView>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, cviews: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumOleDocumentViews>;
}
impl ::windows_core::RuntimeName for IEnumOleDocumentViews {}
impl IEnumOleDocumentViews_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>() -> IEnumOleDocumentViews_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cviews: u32, rgpview: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&cviews), ::core::mem::transmute_copy(&rgpview), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cviews: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&cviews)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumOleDocumentViews as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IEnumOleUndoUnits_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IOleUndoUnit>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumOleUndoUnits>;
}
impl ::windows_core::RuntimeName for IEnumOleUndoUnits {}
impl IEnumOleUndoUnits_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>() -> IEnumOleUndoUnits_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumOleUndoUnits as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IEnumVARIANT_Impl: Sized {
    fn Next(&self, celt: u32, rgvar: *mut super::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> ::windows_core::HRESULT;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IEnumVARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl IEnumVARIANT_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumVARIANT_Impl, const OFFSET: isize>() -> IEnumVARIANT_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumVARIANT as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IFont_Impl: Sized {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Size(&self) -> ::windows_core::Result<super::Com::CY>;
    fn SetSize(&self, size: &super::Com::CY) -> ::windows_core::Result<()>;
    fn Bold(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBold(&self, bold: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Italic(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetItalic(&self, italic: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Underline(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetUnderline(&self, underline: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Strikethrough(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetStrikethrough(&self, strikethrough: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Weight(&self) -> ::windows_core::Result<i16>;
    fn SetWeight(&self, weight: i16) -> ::windows_core::Result<()>;
    fn Charset(&self) -> ::windows_core::Result<i16>;
    fn SetCharset(&self, charset: i16) -> ::windows_core::Result<()>;
    fn hFont(&self) -> ::windows_core::Result<super::super::Graphics::Gdi::HFONT>;
    fn Clone(&self) -> ::windows_core::Result<IFont>;
    fn IsEqual(&self, pfontother: ::core::option::Option<&IFont>) -> ::windows_core::Result<()>;
    fn SetRatio(&self, cylogical: i32, cyhimetric: i32) -> ::windows_core::Result<()>;
    fn QueryTextMetrics(&self, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows_core::Result<()>;
    fn AddRefHfont(&self, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows_core::Result<()>;
    fn ReleaseHfont(&self, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows_core::Result<()>;
    fn SetHdc(&self, hdc: super::super::Graphics::Gdi::HDC) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IFont {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IFont_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>() -> IFont_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::Com::CY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::Com::CY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSize(::core::mem::transmute(&size)).into()
        }
        unsafe extern "system" fn Bold<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbold: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Bold() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbold, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bold: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBold(::core::mem::transmute_copy(&bold)).into()
        }
        unsafe extern "system" fn Italic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitalic: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Italic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitalic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, italic: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetItalic(::core::mem::transmute_copy(&italic)).into()
        }
        unsafe extern "system" fn Underline<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punderline: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Underline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punderline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, underline: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUnderline(::core::mem::transmute_copy(&underline)).into()
        }
        unsafe extern "system" fn Strikethrough<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrikethrough: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Strikethrough() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstrikethrough, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strikethrough: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrikethrough(::core::mem::transmute_copy(&strikethrough)).into()
        }
        unsafe extern "system" fn Weight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweight: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Weight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pweight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWeight(::core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn Charset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcharset: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Charset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcharset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, charset: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCharset(::core::mem::transmute_copy(&charset)).into()
        }
        unsafe extern "system" fn hFont<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phfont: *mut super::super::Graphics::Gdi::HFONT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hFont() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phfont, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfont, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfontother: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsEqual(::windows_core::from_raw_borrowed(&pfontother)).into()
        }
        unsafe extern "system" fn SetRatio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cylogical: i32, cyhimetric: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRatio(::core::mem::transmute_copy(&cylogical), ::core::mem::transmute_copy(&cyhimetric)).into()
        }
        unsafe extern "system" fn QueryTextMetrics<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryTextMetrics(::core::mem::transmute_copy(&ptm)).into()
        }
        unsafe extern "system" fn AddRefHfont<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRefHfont(::core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn ReleaseHfont<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseHfont(::core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn SetHdc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHdc(::core::mem::transmute_copy(&hdc)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFont as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IFontDisp_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFontDisp {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl IFontDisp_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFontDisp_Impl, const OFFSET: isize>() -> IFontDisp_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFontDisp as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IFontEventsDisp_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFontEventsDisp {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl IFontEventsDisp_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFontEventsDisp_Impl, const OFFSET: isize>() -> IFontEventsDisp_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFontEventsDisp as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IGetOleObject_Impl: Sized {
    fn GetOleObject(&self, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IGetOleObject {}
impl IGetOleObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetOleObject_Impl, const OFFSET: isize>() -> IGetOleObject_Vtbl {
        unsafe extern "system" fn GetOleObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOleObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOleObject: GetOleObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IGetOleObject as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IGetVBAObject_Impl: Sized {
    fn GetObject(&self, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IGetVBAObject {}
impl IGetVBAObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetVBAObject_Impl, const OFFSET: isize>() -> IGetVBAObject_Vtbl {
        unsafe extern "system" fn GetObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetVBAObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetObject: GetObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IGetVBAObject as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IObjectIdentity_Impl: Sized {
    fn IsEqualObject(&self, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IObjectIdentity {}
impl IObjectIdentity_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectIdentity_Impl, const OFFSET: isize>() -> IObjectIdentity_Vtbl {
        unsafe extern "system" fn IsEqualObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsEqualObject(::windows_core::from_raw_borrowed(&punk)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsEqualObject: IsEqualObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectIdentity as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IObjectWithSite_Impl: Sized {
    fn SetSite(&self, punksite: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetSite(&self, riid: *const ::windows_core::GUID, ppvsite: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IObjectWithSite {}
impl IObjectWithSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectWithSite_Impl, const OFFSET: isize>() -> IObjectWithSite_Vtbl {
        unsafe extern "system" fn SetSite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectWithSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punksite: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSite(::windows_core::from_raw_borrowed(&punksite)).into()
        }
        unsafe extern "system" fn GetSite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectWithSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvsite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSite(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvsite)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSite: SetSite::<Identity, Impl, OFFSET>,
            GetSite: GetSite::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectWithSite as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOleAdviseHolder_Impl: Sized {
    fn Advise(&self, padvise: ::core::option::Option<&super::Com::IAdviseSink>) -> ::windows_core::Result<u32>;
    fn Unadvise(&self, dwconnection: u32) -> ::windows_core::Result<()>;
    fn EnumAdvise(&self) -> ::windows_core::Result<super::Com::IEnumSTATDATA>;
    fn SendOnRename(&self, pmk: ::core::option::Option<&super::Com::IMoniker>) -> ::windows_core::Result<()>;
    fn SendOnSave(&self) -> ::windows_core::Result<()>;
    fn SendOnClose(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IOleAdviseHolder {}
#[cfg(feature = "Win32_System_Com")]
impl IOleAdviseHolder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>() -> IOleAdviseHolder_Vtbl {
        unsafe extern "system" fn Advise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvise: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Advise(::windows_core::from_raw_borrowed(&padvise)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumAdvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumAdvise() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumadvise, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOnRename<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendOnRename(::windows_core::from_raw_borrowed(&pmk)).into()
        }
        unsafe extern "system" fn SendOnSave<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendOnSave().into()
        }
        unsafe extern "system" fn SendOnClose<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendOnClose().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, Impl, OFFSET>,
            SendOnRename: SendOnRename::<Identity, Impl, OFFSET>,
            SendOnSave: SendOnSave::<Identity, Impl, OFFSET>,
            SendOnClose: SendOnClose::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleAdviseHolder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IOleCache_Impl: Sized {
    fn Cache(&self, pformatetc: *const super::Com::FORMATETC, advf: u32) -> ::windows_core::Result<u32>;
    fn Uncache(&self, dwconnection: u32) -> ::windows_core::Result<()>;
    fn EnumCache(&self) -> ::windows_core::Result<super::Com::IEnumSTATDATA>;
    fn InitCache(&self, pdataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn SetData(&self, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::RuntimeName for IOleCache {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IOleCache_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: isize>() -> IOleCache_Vtbl {
        unsafe extern "system" fn Cache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, advf: u32, pdwconnection: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cache(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&advf)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uncache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Uncache(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstatdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumCache() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstatdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitCache(::windows_core::from_raw_borrowed(&pdataobject)).into()
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pmedium), ::core::mem::transmute_copy(&frelease)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cache: Cache::<Identity, Impl, OFFSET>,
            Uncache: Uncache::<Identity, Impl, OFFSET>,
            EnumCache: EnumCache::<Identity, Impl, OFFSET>,
            InitCache: InitCache::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleCache as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IOleCache2_Impl: Sized + IOleCache_Impl {
    fn UpdateCache(&self, pdataobject: ::core::option::Option<&super::Com::IDataObject>, grfupdf: UPDFCACHE_FLAGS, preserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DiscardCache(&self, dwdiscardoptions: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::RuntimeName for IOleCache2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IOleCache2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCache2_Impl, const OFFSET: isize>() -> IOleCache2_Vtbl {
        unsafe extern "system" fn UpdateCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCache2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, grfupdf: UPDFCACHE_FLAGS, preserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateCache(::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&grfupdf), ::core::mem::transmute_copy(&preserved)).into()
        }
        unsafe extern "system" fn DiscardCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCache2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdiscardoptions: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DiscardCache(::core::mem::transmute_copy(&dwdiscardoptions)).into()
        }
        Self {
            base__: IOleCache_Vtbl::new::<Identity, Impl, OFFSET>(),
            UpdateCache: UpdateCache::<Identity, Impl, OFFSET>,
            DiscardCache: DiscardCache::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleCache2 as ::windows_core::ComInterface>::IID || iid == &<IOleCache as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOleCacheControl_Impl: Sized {
    fn OnRun(&self, pdataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn OnStop(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IOleCacheControl {}
#[cfg(feature = "Win32_System_Com")]
impl IOleCacheControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCacheControl_Impl, const OFFSET: isize>() -> IOleCacheControl_Vtbl {
        unsafe extern "system" fn OnRun<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCacheControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRun(::windows_core::from_raw_borrowed(&pdataobject)).into()
        }
        unsafe extern "system" fn OnStop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCacheControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStop().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRun: OnRun::<Identity, Impl, OFFSET>,
            OnStop: OnStop::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleCacheControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleClientSite_Impl: Sized {
    fn SaveObject(&self) -> ::windows_core::Result<()>;
    fn GetMoniker(&self, dwassign: &OLEGETMONIKER, dwwhichmoniker: &OLEWHICHMK) -> ::windows_core::Result<super::Com::IMoniker>;
    fn GetContainer(&self) -> ::windows_core::Result<IOleContainer>;
    fn ShowObject(&self) -> ::windows_core::Result<()>;
    fn OnShowWindow(&self, fshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RequestNewObjectLayout(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IOleClientSite {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleClientSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: isize>() -> IOleClientSite_Vtbl {
        unsafe extern "system" fn SaveObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveObject().into()
        }
        unsafe extern "system" fn GetMoniker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMoniker(::core::mem::transmute(&dwassign), ::core::mem::transmute(&dwwhichmoniker)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontainer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowObject().into()
        }
        unsafe extern "system" fn OnShowWindow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnShowWindow(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn RequestNewObjectLayout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestNewObjectLayout().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SaveObject: SaveObject::<Identity, Impl, OFFSET>,
            GetMoniker: GetMoniker::<Identity, Impl, OFFSET>,
            GetContainer: GetContainer::<Identity, Impl, OFFSET>,
            ShowObject: ShowObject::<Identity, Impl, OFFSET>,
            OnShowWindow: OnShowWindow::<Identity, Impl, OFFSET>,
            RequestNewObjectLayout: RequestNewObjectLayout::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleClientSite as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IOleCommandTarget_Impl: Sized {
    fn QueryStatus(&self, pguidcmdgroup: *const ::windows_core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows_core::Result<()>;
    fn Exec(&self, pguidcmdgroup: *const ::windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Variant::VARIANT, pvaout: *mut super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IOleCommandTarget {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl IOleCommandTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCommandTarget_Impl, const OFFSET: isize>() -> IOleCommandTarget_Vtbl {
        unsafe extern "system" fn QueryStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCommandTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows_core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryStatus(::core::mem::transmute_copy(&pguidcmdgroup), ::core::mem::transmute_copy(&ccmds), ::core::mem::transmute_copy(&prgcmds), ::core::mem::transmute_copy(&pcmdtext)).into()
        }
        unsafe extern "system" fn Exec<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleCommandTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Variant::VARIANT, pvaout: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Exec(::core::mem::transmute_copy(&pguidcmdgroup), ::core::mem::transmute_copy(&ncmdid), ::core::mem::transmute_copy(&ncmdexecopt), ::core::mem::transmute_copy(&pvain), ::core::mem::transmute_copy(&pvaout)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
            Exec: Exec::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleCommandTarget as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleContainer_Impl: Sized + IParseDisplayName_Impl {
    fn EnumObjects(&self, grfflags: &OLECONTF) -> ::windows_core::Result<super::Com::IEnumUnknown>;
    fn LockContainer(&self, flock: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IOleContainer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleContainer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleContainer_Impl, const OFFSET: isize>() -> IOleContainer_Vtbl {
        unsafe extern "system" fn EnumObjects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumObjects(::core::mem::transmute(&grfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockContainer(::core::mem::transmute_copy(&flock)).into()
        }
        Self {
            base__: IParseDisplayName_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            LockContainer: LockContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleContainer as ::windows_core::ComInterface>::IID || iid == &<IParseDisplayName as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleControl_Impl: Sized {
    fn GetControlInfo(&self, pci: *mut CONTROLINFO) -> ::windows_core::Result<()>;
    fn OnMnemonic(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::Result<()>;
    fn OnAmbientPropertyChange(&self, dispid: i32) -> ::windows_core::Result<()>;
    fn FreezeEvents(&self, bfreeze: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IOleControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControl_Impl, const OFFSET: isize>() -> IOleControl_Vtbl {
        unsafe extern "system" fn GetControlInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut CONTROLINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetControlInfo(::core::mem::transmute_copy(&pci)).into()
        }
        unsafe extern "system" fn OnMnemonic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMnemonic(::core::mem::transmute_copy(&pmsg)).into()
        }
        unsafe extern "system" fn OnAmbientPropertyChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAmbientPropertyChange(::core::mem::transmute_copy(&dispid)).into()
        }
        unsafe extern "system" fn FreezeEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfreeze: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreezeEvents(::core::mem::transmute_copy(&bfreeze)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetControlInfo: GetControlInfo::<Identity, Impl, OFFSET>,
            OnMnemonic: OnMnemonic::<Identity, Impl, OFFSET>,
            OnAmbientPropertyChange: OnAmbientPropertyChange::<Identity, Impl, OFFSET>,
            FreezeEvents: FreezeEvents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleControlSite_Impl: Sized {
    fn OnControlInfoChanged(&self) -> ::windows_core::Result<()>;
    fn LockInPlaceActive(&self, flock: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetExtendedControl(&self) -> ::windows_core::Result<super::Com::IDispatch>;
    fn TransformCoords(&self, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: &XFORMCOORDS) -> ::windows_core::Result<()>;
    fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: KEYMODIFIERS) -> ::windows_core::Result<()>;
    fn OnFocus(&self, fgotfocus: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ShowPropertyFrame(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IOleControlSite {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleControlSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: isize>() -> IOleControlSite_Vtbl {
        unsafe extern "system" fn OnControlInfoChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnControlInfoChanged().into()
        }
        unsafe extern "system" fn LockInPlaceActive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockInPlaceActive(::core::mem::transmute_copy(&flock)).into()
        }
        unsafe extern "system" fn GetExtendedControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExtendedControl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformCoords<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransformCoords(::core::mem::transmute_copy(&pptlhimetric), ::core::mem::transmute_copy(&pptfcontainer), ::core::mem::transmute(&dwflags)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: KEYMODIFIERS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TranslateAccelerator(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&grfmodifiers)).into()
        }
        unsafe extern "system" fn OnFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgotfocus: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFocus(::core::mem::transmute_copy(&fgotfocus)).into()
        }
        unsafe extern "system" fn ShowPropertyFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowPropertyFrame().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnControlInfoChanged: OnControlInfoChanged::<Identity, Impl, OFFSET>,
            LockInPlaceActive: LockInPlaceActive::<Identity, Impl, OFFSET>,
            GetExtendedControl: GetExtendedControl::<Identity, Impl, OFFSET>,
            TransformCoords: TransformCoords::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
            OnFocus: OnFocus::<Identity, Impl, OFFSET>,
            ShowPropertyFrame: ShowPropertyFrame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleControlSite as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOleDocument_Impl: Sized {
    fn CreateView(&self, pipsite: ::core::option::Option<&IOleInPlaceSite>, pstm: ::core::option::Option<&super::Com::IStream>, dwreserved: u32) -> ::windows_core::Result<IOleDocumentView>;
    fn GetDocMiscStatus(&self) -> ::windows_core::Result<u32>;
    fn EnumViews(&self, ppenum: *mut ::core::option::Option<IEnumOleDocumentViews>, ppview: *mut ::core::option::Option<IOleDocumentView>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IOleDocument {}
#[cfg(feature = "Win32_System_Com")]
impl IOleDocument_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocument_Impl, const OFFSET: isize>() -> IOleDocument_Vtbl {
        unsafe extern "system" fn CreateView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsite: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, dwreserved: u32, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateView(::windows_core::from_raw_borrowed(&pipsite), ::windows_core::from_raw_borrowed(&pstm), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocMiscStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocMiscStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumViews<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumViews(::core::mem::transmute_copy(&ppenum), ::core::mem::transmute_copy(&ppview)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateView: CreateView::<Identity, Impl, OFFSET>,
            GetDocMiscStatus: GetDocMiscStatus::<Identity, Impl, OFFSET>,
            EnumViews: EnumViews::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleDocument as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IOleDocumentSite_Impl: Sized {
    fn ActivateMe(&self, pviewtoactivate: ::core::option::Option<&IOleDocumentView>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IOleDocumentSite {}
impl IOleDocumentSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentSite_Impl, const OFFSET: isize>() -> IOleDocumentSite_Vtbl {
        unsafe extern "system" fn ActivateMe<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewtoactivate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ActivateMe(::windows_core::from_raw_borrowed(&pviewtoactivate)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ActivateMe: ActivateMe::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleDocumentSite as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleDocumentView_Impl: Sized {
    fn SetInPlaceSite(&self, pipsite: ::core::option::Option<&IOleInPlaceSite>) -> ::windows_core::Result<()>;
    fn GetInPlaceSite(&self) -> ::windows_core::Result<IOleInPlaceSite>;
    fn GetDocument(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetRect(&self, prcview: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn GetRect(&self) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn SetRectComplex(&self, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Show(&self, fshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UIActivate(&self, fuiactivate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Open(&self) -> ::windows_core::Result<()>;
    fn CloseView(&self, dwreserved: u32) -> ::windows_core::Result<()>;
    fn SaveViewState(&self, pstm: ::core::option::Option<&super::Com::IStream>) -> ::windows_core::Result<()>;
    fn ApplyViewState(&self, pstm: ::core::option::Option<&super::Com::IStream>) -> ::windows_core::Result<()>;
    fn Clone(&self, pipsitenew: ::core::option::Option<&IOleInPlaceSite>) -> ::windows_core::Result<IOleDocumentView>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IOleDocumentView {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleDocumentView_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>() -> IOleDocumentView_Vtbl {
        unsafe extern "system" fn SetInPlaceSite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsite: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInPlaceSite(::windows_core::from_raw_borrowed(&pipsite)).into()
        }
        unsafe extern "system" fn GetInPlaceSite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipsite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInPlaceSite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipsite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRect(::core::mem::transmute_copy(&prcview)).into()
        }
        unsafe extern "system" fn GetRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prcview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRectComplex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRectComplex(::core::mem::transmute_copy(&prcview), ::core::mem::transmute_copy(&prchscroll), ::core::mem::transmute_copy(&prcvscroll), ::core::mem::transmute_copy(&prcsizebox)).into()
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn UIActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuiactivate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UIActivate(::core::mem::transmute_copy(&fuiactivate)).into()
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open().into()
        }
        unsafe extern "system" fn CloseView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseView(::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SaveViewState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveViewState(::windows_core::from_raw_borrowed(&pstm)).into()
        }
        unsafe extern "system" fn ApplyViewState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyViewState(::windows_core::from_raw_borrowed(&pstm)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipsitenew: *mut ::core::ffi::c_void, ppviewnew: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone(::windows_core::from_raw_borrowed(&pipsitenew)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppviewnew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleDocumentView as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceActiveObject_Impl: Sized + IOleWindow_Impl {
    fn TranslateAccelerator(&self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::Result<()>;
    fn OnFrameWindowActivate(&self, factivate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnDocWindowActivate(&self, factivate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ResizeBorder(&self, prcborder: *const super::super::Foundation::RECT, puiwindow: ::core::option::Option<&IOleInPlaceUIWindow>, fframewindow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn EnableModeless(&self, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IOleInPlaceActiveObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceActiveObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>() -> IOleInPlaceActiveObject_Vtbl {
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TranslateAccelerator(::core::mem::transmute_copy(&lpmsg)).into()
        }
        unsafe extern "system" fn OnFrameWindowActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFrameWindowActivate(::core::mem::transmute_copy(&factivate)).into()
        }
        unsafe extern "system" fn OnDocWindowActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDocWindowActivate(::core::mem::transmute_copy(&factivate)).into()
        }
        unsafe extern "system" fn ResizeBorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcborder: *const super::super::Foundation::RECT, puiwindow: *mut ::core::ffi::c_void, fframewindow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResizeBorder(::core::mem::transmute_copy(&prcborder), ::windows_core::from_raw_borrowed(&puiwindow), ::core::mem::transmute_copy(&fframewindow)).into()
        }
        unsafe extern "system" fn EnableModeless<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableModeless(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, Impl, OFFSET>(),
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
            OnFrameWindowActivate: OnFrameWindowActivate::<Identity, Impl, OFFSET>,
            OnDocWindowActivate: OnDocWindowActivate::<Identity, Impl, OFFSET>,
            ResizeBorder: ResizeBorder::<Identity, Impl, OFFSET>,
            EnableModeless: EnableModeless::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleInPlaceActiveObject as ::windows_core::ComInterface>::IID || iid == &<IOleWindow as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceFrame_Impl: Sized + IOleInPlaceUIWindow_Impl {
    fn InsertMenus(&self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> ::windows_core::Result<()>;
    fn SetMenu(&self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn RemoveMenus(&self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_core::Result<()>;
    fn SetStatusText(&self, pszstatustext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnableModeless(&self, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn TranslateAccelerator(&self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IOleInPlaceFrame {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceFrame_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>() -> IOleInPlaceFrame_Vtbl {
        unsafe extern "system" fn InsertMenus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertMenus(::core::mem::transmute_copy(&hmenushared), ::core::mem::transmute_copy(&lpmenuwidths)).into()
        }
        unsafe extern "system" fn SetMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMenu(::core::mem::transmute_copy(&hmenushared), ::core::mem::transmute_copy(&holemenu), ::core::mem::transmute_copy(&hwndactiveobject)).into()
        }
        unsafe extern "system" fn RemoveMenus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveMenus(::core::mem::transmute_copy(&hmenushared)).into()
        }
        unsafe extern "system" fn SetStatusText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatusText(::core::mem::transmute(&pszstatustext)).into()
        }
        unsafe extern "system" fn EnableModeless<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableModeless(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TranslateAccelerator(::core::mem::transmute_copy(&lpmsg), ::core::mem::transmute_copy(&wid)).into()
        }
        Self {
            base__: IOleInPlaceUIWindow_Vtbl::new::<Identity, Impl, OFFSET>(),
            InsertMenus: InsertMenus::<Identity, Impl, OFFSET>,
            SetMenu: SetMenu::<Identity, Impl, OFFSET>,
            RemoveMenus: RemoveMenus::<Identity, Impl, OFFSET>,
            SetStatusText: SetStatusText::<Identity, Impl, OFFSET>,
            EnableModeless: EnableModeless::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleInPlaceFrame as ::windows_core::ComInterface>::IID || iid == &<IOleWindow as ::windows_core::ComInterface>::IID || iid == &<IOleInPlaceUIWindow as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceObject_Impl: Sized + IOleWindow_Impl {
    fn InPlaceDeactivate(&self) -> ::windows_core::Result<()>;
    fn UIDeactivate(&self) -> ::windows_core::Result<()>;
    fn SetObjectRects(&self, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn ReactivateAndUndo(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleInPlaceObject {}
#[cfg(feature = "Win32_Foundation")]
impl IOleInPlaceObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceObject_Impl, const OFFSET: isize>() -> IOleInPlaceObject_Vtbl {
        unsafe extern "system" fn InPlaceDeactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InPlaceDeactivate().into()
        }
        unsafe extern "system" fn UIDeactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UIDeactivate().into()
        }
        unsafe extern "system" fn SetObjectRects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectRects(::core::mem::transmute_copy(&lprcposrect), ::core::mem::transmute_copy(&lprccliprect)).into()
        }
        unsafe extern "system" fn ReactivateAndUndo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReactivateAndUndo().into()
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, Impl, OFFSET>(),
            InPlaceDeactivate: InPlaceDeactivate::<Identity, Impl, OFFSET>,
            UIDeactivate: UIDeactivate::<Identity, Impl, OFFSET>,
            SetObjectRects: SetObjectRects::<Identity, Impl, OFFSET>,
            ReactivateAndUndo: ReactivateAndUndo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleInPlaceObject as ::windows_core::ComInterface>::IID || iid == &<IOleWindow as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceObjectWindowless_Impl: Sized + IOleInPlaceObject_Impl {
    fn OnWindowMessage(&self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::LRESULT>;
    fn GetDropTarget(&self) -> ::windows_core::Result<IDropTarget>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleInPlaceObjectWindowless {}
#[cfg(feature = "Win32_Foundation")]
impl IOleInPlaceObjectWindowless_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>() -> IOleInPlaceObjectWindowless_Vtbl {
        unsafe extern "system" fn OnWindowMessage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnWindowMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDropTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdroptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDropTarget() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdroptarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IOleInPlaceObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnWindowMessage: OnWindowMessage::<Identity, Impl, OFFSET>,
            GetDropTarget: GetDropTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleInPlaceObjectWindowless as ::windows_core::ComInterface>::IID || iid == &<IOleWindow as ::windows_core::ComInterface>::IID || iid == &<IOleInPlaceObject as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceSite_Impl: Sized + IOleWindow_Impl {
    fn CanInPlaceActivate(&self) -> ::windows_core::Result<()>;
    fn OnInPlaceActivate(&self) -> ::windows_core::Result<()>;
    fn OnUIActivate(&self) -> ::windows_core::Result<()>;
    fn GetWindowContext(&self, ppframe: *mut ::core::option::Option<IOleInPlaceFrame>, ppdoc: *mut ::core::option::Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> ::windows_core::Result<()>;
    fn Scroll(&self, scrollextant: &super::super::Foundation::SIZE) -> ::windows_core::Result<()>;
    fn OnUIDeactivate(&self, fundoable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnInPlaceDeactivate(&self) -> ::windows_core::Result<()>;
    fn DiscardUndoState(&self) -> ::windows_core::Result<()>;
    fn DeactivateAndUndo(&self) -> ::windows_core::Result<()>;
    fn OnPosRectChange(&self, lprcposrect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IOleInPlaceSite {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>() -> IOleInPlaceSite_Vtbl {
        unsafe extern "system" fn CanInPlaceActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanInPlaceActivate().into()
        }
        unsafe extern "system" fn OnInPlaceActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInPlaceActivate().into()
        }
        unsafe extern "system" fn OnUIActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUIActivate().into()
        }
        unsafe extern "system" fn GetWindowContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppframe: *mut *mut ::core::ffi::c_void, ppdoc: *mut *mut ::core::ffi::c_void, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWindowContext(::core::mem::transmute_copy(&ppframe), ::core::mem::transmute_copy(&ppdoc), ::core::mem::transmute_copy(&lprcposrect), ::core::mem::transmute_copy(&lprccliprect), ::core::mem::transmute_copy(&lpframeinfo)).into()
        }
        unsafe extern "system" fn Scroll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollextant: super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Scroll(::core::mem::transmute(&scrollextant)).into()
        }
        unsafe extern "system" fn OnUIDeactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fundoable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUIDeactivate(::core::mem::transmute_copy(&fundoable)).into()
        }
        unsafe extern "system" fn OnInPlaceDeactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInPlaceDeactivate().into()
        }
        unsafe extern "system" fn DiscardUndoState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DiscardUndoState().into()
        }
        unsafe extern "system" fn DeactivateAndUndo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeactivateAndUndo().into()
        }
        unsafe extern "system" fn OnPosRectChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPosRectChange(::core::mem::transmute_copy(&lprcposrect)).into()
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleInPlaceSite as ::windows_core::ComInterface>::IID || iid == &<IOleWindow as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceSiteEx_Impl: Sized + IOleInPlaceSite_Impl {
    fn OnInPlaceActivateEx(&self, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows_core::Result<()>;
    fn OnInPlaceDeactivateEx(&self, fnoredraw: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RequestUIActivate(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IOleInPlaceSiteEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceSiteEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: isize>() -> IOleInPlaceSiteEx_Vtbl {
        unsafe extern "system" fn OnInPlaceActivateEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInPlaceActivateEx(::core::mem::transmute_copy(&pfnoredraw), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnInPlaceDeactivateEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnoredraw: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInPlaceDeactivateEx(::core::mem::transmute_copy(&fnoredraw)).into()
        }
        unsafe extern "system" fn RequestUIActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestUIActivate().into()
        }
        Self {
            base__: IOleInPlaceSite_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnInPlaceActivateEx: OnInPlaceActivateEx::<Identity, Impl, OFFSET>,
            OnInPlaceDeactivateEx: OnInPlaceDeactivateEx::<Identity, Impl, OFFSET>,
            RequestUIActivate: RequestUIActivate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleInPlaceSiteEx as ::windows_core::ComInterface>::IID || iid == &<IOleWindow as ::windows_core::ComInterface>::IID || iid == &<IOleInPlaceSite as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceSiteWindowless_Impl: Sized + IOleInPlaceSiteEx_Impl {
    fn CanWindowlessActivate(&self) -> ::windows_core::Result<()>;
    fn GetCapture(&self) -> ::windows_core::Result<()>;
    fn SetCapture(&self, fcapture: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFocus(&self) -> ::windows_core::Result<()>;
    fn SetFocus(&self, ffocus: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDC(&self, prect: *const super::super::Foundation::RECT, grfflags: u32) -> ::windows_core::Result<super::super::Graphics::Gdi::HDC>;
    fn ReleaseDC(&self, hdc: super::super::Graphics::Gdi::HDC) -> ::windows_core::Result<()>;
    fn InvalidateRect(&self, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn InvalidateRgn(&self, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ScrollRect(&self, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn AdjustRect(&self, prc: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn OnDefWindowMessage(&self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::LRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IOleInPlaceSiteWindowless {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceSiteWindowless_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>() -> IOleInPlaceSiteWindowless_Vtbl {
        unsafe extern "system" fn CanWindowlessActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanWindowlessActivate().into()
        }
        unsafe extern "system" fn GetCapture<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCapture().into()
        }
        unsafe extern "system" fn SetCapture<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcapture: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCapture(::core::mem::transmute_copy(&fcapture)).into()
        }
        unsafe extern "system" fn GetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFocus().into()
        }
        unsafe extern "system" fn SetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffocus: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFocus(::core::mem::transmute_copy(&ffocus)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, grfflags: u32, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDC(::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&grfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phdc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseDC(::core::mem::transmute_copy(&hdc)).into()
        }
        unsafe extern "system" fn InvalidateRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvalidateRect(::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&ferase)).into()
        }
        unsafe extern "system" fn InvalidateRgn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvalidateRgn(::core::mem::transmute_copy(&hrgn), ::core::mem::transmute_copy(&ferase)).into()
        }
        unsafe extern "system" fn ScrollRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScrollRect(::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy), ::core::mem::transmute_copy(&prectscroll), ::core::mem::transmute_copy(&prectclip)).into()
        }
        unsafe extern "system" fn AdjustRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdjustRect(::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn OnDefWindowMessage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnDefWindowMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IOleInPlaceSiteEx_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleInPlaceSiteWindowless as ::windows_core::ComInterface>::IID || iid == &<IOleWindow as ::windows_core::ComInterface>::IID || iid == &<IOleInPlaceSite as ::windows_core::ComInterface>::IID || iid == &<IOleInPlaceSiteEx as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceUIWindow_Impl: Sized + IOleWindow_Impl {
    fn GetBorder(&self) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn RequestBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn SetBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn SetActiveObject(&self, pactiveobject: ::core::option::Option<&IOleInPlaceActiveObject>, pszobjname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleInPlaceUIWindow {}
#[cfg(feature = "Win32_Foundation")]
impl IOleInPlaceUIWindow_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>() -> IOleInPlaceUIWindow_Vtbl {
        unsafe extern "system" fn GetBorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprectborder: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBorder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lprectborder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestBorderSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestBorderSpace(::core::mem::transmute_copy(&pborderwidths)).into()
        }
        unsafe extern "system" fn SetBorderSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBorderSpace(::core::mem::transmute_copy(&pborderwidths)).into()
        }
        unsafe extern "system" fn SetActiveObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactiveobject: *mut ::core::ffi::c_void, pszobjname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActiveObject(::windows_core::from_raw_borrowed(&pactiveobject), ::core::mem::transmute(&pszobjname)).into()
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetBorder: GetBorder::<Identity, Impl, OFFSET>,
            RequestBorderSpace: RequestBorderSpace::<Identity, Impl, OFFSET>,
            SetBorderSpace: SetBorderSpace::<Identity, Impl, OFFSET>,
            SetActiveObject: SetActiveObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleInPlaceUIWindow as ::windows_core::ComInterface>::IID || iid == &<IOleWindow as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleItemContainer_Impl: Sized + IOleContainer_Impl {
    fn GetObject(&self, pszitem: &::windows_core::PCWSTR, dwspeedneeded: u32, pbc: ::core::option::Option<&super::Com::IBindCtx>, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetObjectStorage(&self, pszitem: &::windows_core::PCWSTR, pbc: ::core::option::Option<&super::Com::IBindCtx>, riid: *const ::windows_core::GUID, ppvstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn IsRunning(&self, pszitem: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IOleItemContainer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOleItemContainer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleItemContainer_Impl, const OFFSET: isize>() -> IOleItemContainer_Vtbl {
        unsafe extern "system" fn GetObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: ::windows_core::PCWSTR, dwspeedneeded: u32, pbc: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObject(::core::mem::transmute(&pszitem), ::core::mem::transmute_copy(&dwspeedneeded), ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn GetObjectStorage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: ::windows_core::PCWSTR, pbc: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectStorage(::core::mem::transmute(&pszitem), ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvstorage)).into()
        }
        unsafe extern "system" fn IsRunning<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitem: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsRunning(::core::mem::transmute(&pszitem)).into()
        }
        Self {
            base__: IOleContainer_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            GetObjectStorage: GetObjectStorage::<Identity, Impl, OFFSET>,
            IsRunning: IsRunning::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleItemContainer as ::windows_core::ComInterface>::IID || iid == &<IParseDisplayName as ::windows_core::ComInterface>::IID || iid == &<IOleContainer as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOleLink_Impl: Sized {
    fn SetUpdateOptions(&self, dwupdateopt: u32) -> ::windows_core::Result<()>;
    fn GetUpdateOptions(&self) -> ::windows_core::Result<u32>;
    fn SetSourceMoniker(&self, pmk: ::core::option::Option<&super::Com::IMoniker>, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetSourceMoniker(&self) -> ::windows_core::Result<super::Com::IMoniker>;
    fn SetSourceDisplayName(&self, pszstatustext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSourceDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn BindToSource(&self, bindflags: u32, pbc: ::core::option::Option<&super::Com::IBindCtx>) -> ::windows_core::Result<()>;
    fn BindIfRunning(&self) -> ::windows_core::Result<()>;
    fn GetBoundSource(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn UnbindSource(&self) -> ::windows_core::Result<()>;
    fn Update(&self, pbc: ::core::option::Option<&super::Com::IBindCtx>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IOleLink {}
#[cfg(feature = "Win32_System_Com")]
impl IOleLink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>() -> IOleLink_Vtbl {
        unsafe extern "system" fn SetUpdateOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwupdateopt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUpdateOptions(::core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetUpdateOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwupdateopt: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUpdateOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwupdateopt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceMoniker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSourceMoniker(::windows_core::from_raw_borrowed(&pmk), ::core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn GetSourceMoniker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceMoniker() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSourceDisplayName(::core::mem::transmute(&pszstatustext)).into()
        }
        unsafe extern "system" fn GetSourceDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdisplayname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindflags: u32, pbc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindToSource(::core::mem::transmute_copy(&bindflags), ::windows_core::from_raw_borrowed(&pbc)).into()
        }
        unsafe extern "system" fn BindIfRunning<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindIfRunning().into()
        }
        unsafe extern "system" fn GetBoundSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBoundSource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnbindSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnbindSource().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update(::windows_core::from_raw_borrowed(&pbc)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleLink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleObject_Impl: Sized {
    fn SetClientSite(&self, pclientsite: ::core::option::Option<&IOleClientSite>) -> ::windows_core::Result<()>;
    fn GetClientSite(&self) -> ::windows_core::Result<IOleClientSite>;
    fn SetHostNames(&self, szcontainerapp: &::windows_core::PCWSTR, szcontainerobj: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Close(&self, dwsaveoption: &OLECLOSE) -> ::windows_core::Result<()>;
    fn SetMoniker(&self, dwwhichmoniker: &OLEWHICHMK, pmk: ::core::option::Option<&super::Com::IMoniker>) -> ::windows_core::Result<()>;
    fn GetMoniker(&self, dwassign: &OLEGETMONIKER, dwwhichmoniker: &OLEWHICHMK) -> ::windows_core::Result<super::Com::IMoniker>;
    fn InitFromData(&self, pdataobject: ::core::option::Option<&super::Com::IDataObject>, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows_core::Result<()>;
    fn GetClipboardData(&self, dwreserved: u32) -> ::windows_core::Result<super::Com::IDataObject>;
    fn DoVerb(&self, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: ::core::option::Option<&IOleClientSite>, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn EnumVerbs(&self) -> ::windows_core::Result<IEnumOLEVERB>;
    fn Update(&self) -> ::windows_core::Result<()>;
    fn IsUpToDate(&self) -> ::windows_core::Result<()>;
    fn GetUserClassID(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetUserType(&self, dwformoftype: &USERCLASSTYPE) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetExtent(&self, dwdrawaspect: super::Com::DVASPECT, psizel: *const super::super::Foundation::SIZE) -> ::windows_core::Result<()>;
    fn GetExtent(&self, dwdrawaspect: super::Com::DVASPECT) -> ::windows_core::Result<super::super::Foundation::SIZE>;
    fn Advise(&self, padvsink: ::core::option::Option<&super::Com::IAdviseSink>) -> ::windows_core::Result<u32>;
    fn Unadvise(&self, dwconnection: u32) -> ::windows_core::Result<()>;
    fn EnumAdvise(&self) -> ::windows_core::Result<super::Com::IEnumSTATDATA>;
    fn GetMiscStatus(&self, dwaspect: super::Com::DVASPECT) -> ::windows_core::Result<OLEMISC>;
    fn SetColorScheme(&self, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IOleObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>() -> IOleObject_Vtbl {
        unsafe extern "system" fn SetClientSite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientsite: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientSite(::windows_core::from_raw_borrowed(&pclientsite)).into()
        }
        unsafe extern "system" fn GetClientSite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclientsite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClientSite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientsite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostNames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcontainerapp: ::windows_core::PCWSTR, szcontainerobj: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHostNames(::core::mem::transmute(&szcontainerapp), ::core::mem::transmute(&szcontainerobj)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsaveoption: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close(::core::mem::transmute(&dwsaveoption)).into()
        }
        unsafe extern "system" fn SetMoniker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwhichmoniker: u32, pmk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMoniker(::core::mem::transmute(&dwwhichmoniker), ::windows_core::from_raw_borrowed(&pmk)).into()
        }
        unsafe extern "system" fn GetMoniker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMoniker(::core::mem::transmute(&dwassign), ::core::mem::transmute(&dwwhichmoniker)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitFromData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitFromData(::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&fcreation), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetClipboardData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClipboardData(::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoVerb<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: *mut ::core::ffi::c_void, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoVerb(::core::mem::transmute_copy(&iverb), ::core::mem::transmute_copy(&lpmsg), ::windows_core::from_raw_borrowed(&pactivesite), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lprcposrect)).into()
        }
        unsafe extern "system" fn EnumVerbs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumoleverb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumVerbs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumoleverb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update().into()
        }
        unsafe extern "system" fn IsUpToDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsUpToDate().into()
        }
        unsafe extern "system" fn GetUserClassID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserClassID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwformoftype: u32, pszusertype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserType(::core::mem::transmute(&dwformoftype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszusertype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, psizel: *const super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExtent(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&psizel)).into()
        }
        unsafe extern "system" fn GetExtent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, psizel: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExtent(::core::mem::transmute_copy(&dwdrawaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psizel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvsink: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Advise(::windows_core::from_raw_borrowed(&padvsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumAdvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumAdvise() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumadvise, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMiscStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: super::Com::DVASPECT, pdwstatus: *mut OLEMISC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMiscStatus(::core::mem::transmute_copy(&dwaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorScheme<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorScheme(::core::mem::transmute_copy(&plogpal)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleObject as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleParentUndoUnit_Impl: Sized + IOleUndoUnit_Impl {
    fn Open(&self, ppuu: ::core::option::Option<&IOleParentUndoUnit>) -> ::windows_core::Result<()>;
    fn Close(&self, ppuu: ::core::option::Option<&IOleParentUndoUnit>, fcommit: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Add(&self, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn FindUnit(&self, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn GetParentState(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleParentUndoUnit {}
#[cfg(feature = "Win32_Foundation")]
impl IOleParentUndoUnit_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>() -> IOleParentUndoUnit_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::windows_core::from_raw_borrowed(&ppuu)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void, fcommit: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close(::windows_core::from_raw_borrowed(&ppuu), ::core::mem::transmute_copy(&fcommit)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn FindUnit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindUnit(::windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn GetParentState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IOleUndoUnit_Vtbl::new::<Identity, Impl, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            FindUnit: FindUnit::<Identity, Impl, OFFSET>,
            GetParentState: GetParentState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleParentUndoUnit as ::windows_core::ComInterface>::IID || iid == &<IOleUndoUnit as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkContainerA_Impl: Sized {
    fn GetNextLink(&self, dwlink: u32) -> u32;
    fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows_core::Result<()>;
    fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows_core::Result<u32>;
    fn SetLinkSource(&self, dwlink: u32, lpszdisplayname: &::windows_core::PCSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: *mut ::windows_core::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows_core::PSTR, lplpszshortlinktype: *mut ::windows_core::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OpenLinkSource(&self, dwlink: u32) -> ::windows_core::Result<()>;
    fn UpdateLink(&self, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CancelLink(&self, dwlink: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleUILinkContainerA {}
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkContainerA_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>() -> IOleUILinkContainerA_Vtbl {
        unsafe extern "system" fn GetNextLink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextLink(::core::mem::transmute_copy(&dwlink))
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdwupdateopt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: ::windows_core::PCSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute(&lpszdisplayname), ::core::mem::transmute_copy(&lenfilename), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&fvalidatesource)).into()
        }
        unsafe extern "system" fn GetLinkSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut ::windows_core::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows_core::PSTR, lplpszshortlinktype: *mut ::windows_core::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lplpszdisplayname), ::core::mem::transmute_copy(&lplenfilename), ::core::mem::transmute_copy(&lplpszfulllinktype), ::core::mem::transmute_copy(&lplpszshortlinktype), ::core::mem::transmute_copy(&lpfsourceavailable), ::core::mem::transmute_copy(&lpfisselected)).into()
        }
        unsafe extern "system" fn OpenLinkSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenLinkSource(::core::mem::transmute_copy(&dwlink)).into()
        }
        unsafe extern "system" fn UpdateLink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateLink(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&ferrormessage), ::core::mem::transmute_copy(&freserved)).into()
        }
        unsafe extern "system" fn CancelLink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelLink(::core::mem::transmute_copy(&dwlink)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleUILinkContainerA as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkContainerW_Impl: Sized {
    fn GetNextLink(&self, dwlink: u32) -> u32;
    fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows_core::Result<()>;
    fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows_core::Result<u32>;
    fn SetLinkSource(&self, dwlink: u32, lpszdisplayname: &::windows_core::PCWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: *mut ::windows_core::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows_core::PWSTR, lplpszshortlinktype: *mut ::windows_core::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OpenLinkSource(&self, dwlink: u32) -> ::windows_core::Result<()>;
    fn UpdateLink(&self, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CancelLink(&self, dwlink: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleUILinkContainerW {}
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkContainerW_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>() -> IOleUILinkContainerW_Vtbl {
        unsafe extern "system" fn GetNextLink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextLink(::core::mem::transmute_copy(&dwlink))
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLinkUpdateOptions(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdwupdateopt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: ::windows_core::PCWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute(&lpszdisplayname), ::core::mem::transmute_copy(&lenfilename), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&fvalidatesource)).into()
        }
        unsafe extern "system" fn GetLinkSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut ::windows_core::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows_core::PWSTR, lplpszshortlinktype: *mut ::windows_core::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLinkSource(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lplpszdisplayname), ::core::mem::transmute_copy(&lplenfilename), ::core::mem::transmute_copy(&lplpszfulllinktype), ::core::mem::transmute_copy(&lplpszshortlinktype), ::core::mem::transmute_copy(&lpfsourceavailable), ::core::mem::transmute_copy(&lpfisselected)).into()
        }
        unsafe extern "system" fn OpenLinkSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenLinkSource(::core::mem::transmute_copy(&dwlink)).into()
        }
        unsafe extern "system" fn UpdateLink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateLink(::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&ferrormessage), ::core::mem::transmute_copy(&freserved)).into()
        }
        unsafe extern "system" fn CancelLink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelLink(::core::mem::transmute_copy(&dwlink)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleUILinkContainerW as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkInfoA_Impl: Sized + IOleUILinkContainerA_Impl {
    fn GetLastUpdate(&self, dwlink: u32) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleUILinkInfoA {}
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkInfoA_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkInfoA_Impl, const OFFSET: isize>() -> IOleUILinkInfoA_Vtbl {
        unsafe extern "system" fn GetLastUpdate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastUpdate(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lplastupdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IOleUILinkContainerA_Vtbl::new::<Identity, Impl, OFFSET>(), GetLastUpdate: GetLastUpdate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleUILinkInfoA as ::windows_core::ComInterface>::IID || iid == &<IOleUILinkContainerA as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkInfoW_Impl: Sized + IOleUILinkContainerW_Impl {
    fn GetLastUpdate(&self, dwlink: u32) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleUILinkInfoW {}
#[cfg(feature = "Win32_Foundation")]
impl IOleUILinkInfoW_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkInfoW_Impl, const OFFSET: isize>() -> IOleUILinkInfoW_Vtbl {
        unsafe extern "system" fn GetLastUpdate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUILinkInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastUpdate(::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lplastupdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IOleUILinkContainerW_Vtbl::new::<Identity, Impl, OFFSET>(), GetLastUpdate: GetLastUpdate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleUILinkInfoW as ::windows_core::ComInterface>::IID || iid == &<IOleUILinkContainerW as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUIObjInfoA_Impl: Sized {
    fn GetObjectInfo(&self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows_core::PSTR, lplpsztype: *mut ::windows_core::PSTR, lplpszshorttype: *mut ::windows_core::PSTR, lplpszlocation: *mut ::windows_core::PSTR) -> ::windows_core::Result<()>;
    fn GetConvertInfo(&self, dwobject: u32, lpclassid: *mut ::windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows_core::GUID, lplpclsidexclude: *mut *mut ::windows_core::GUID, lpcclsidexclude: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertObject(&self, dwobject: u32, clsidnew: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetViewInfo(&self, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows_core::Result<()>;
    fn SetViewInfo(&self, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleUIObjInfoA {}
#[cfg(feature = "Win32_Foundation")]
impl IOleUIObjInfoA_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>() -> IOleUIObjInfoA_Vtbl {
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows_core::PSTR, lplpsztype: *mut ::windows_core::PSTR, lplpszshorttype: *mut ::windows_core::PSTR, lplpszlocation: *mut ::windows_core::PSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpdwobjsize), ::core::mem::transmute_copy(&lplpszlabel), ::core::mem::transmute_copy(&lplpsztype), ::core::mem::transmute_copy(&lplpszshorttype), ::core::mem::transmute_copy(&lplpszlocation)).into()
        }
        unsafe extern "system" fn GetConvertInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows_core::GUID, lplpclsidexclude: *mut *mut ::windows_core::GUID, lpcclsidexclude: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConvertInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpclassid), ::core::mem::transmute_copy(&lpwformat), ::core::mem::transmute_copy(&lpconvertdefaultclassid), ::core::mem::transmute_copy(&lplpclsidexclude), ::core::mem::transmute_copy(&lpcclsidexclude)).into()
        }
        unsafe extern "system" fn ConvertObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertObject(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&clsidnew)).into()
        }
        unsafe extern "system" fn GetViewInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&phmetapict), ::core::mem::transmute_copy(&pdvaspect), ::core::mem::transmute_copy(&pncurrentscale)).into()
        }
        unsafe extern "system" fn SetViewInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&hmetapict), ::core::mem::transmute_copy(&dvaspect), ::core::mem::transmute_copy(&ncurrentscale), ::core::mem::transmute_copy(&brelativetoorig)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetConvertInfo: GetConvertInfo::<Identity, Impl, OFFSET>,
            ConvertObject: ConvertObject::<Identity, Impl, OFFSET>,
            GetViewInfo: GetViewInfo::<Identity, Impl, OFFSET>,
            SetViewInfo: SetViewInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleUIObjInfoA as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUIObjInfoW_Impl: Sized {
    fn GetObjectInfo(&self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows_core::PWSTR, lplpsztype: *mut ::windows_core::PWSTR, lplpszshorttype: *mut ::windows_core::PWSTR, lplpszlocation: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetConvertInfo(&self, dwobject: u32, lpclassid: *mut ::windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows_core::GUID, lplpclsidexclude: *mut *mut ::windows_core::GUID, lpcclsidexclude: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertObject(&self, dwobject: u32, clsidnew: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetViewInfo(&self, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows_core::Result<()>;
    fn SetViewInfo(&self, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleUIObjInfoW {}
#[cfg(feature = "Win32_Foundation")]
impl IOleUIObjInfoW_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>() -> IOleUIObjInfoW_Vtbl {
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows_core::PWSTR, lplpsztype: *mut ::windows_core::PWSTR, lplpszshorttype: *mut ::windows_core::PWSTR, lplpszlocation: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpdwobjsize), ::core::mem::transmute_copy(&lplpszlabel), ::core::mem::transmute_copy(&lplpsztype), ::core::mem::transmute_copy(&lplpszshorttype), ::core::mem::transmute_copy(&lplpszlocation)).into()
        }
        unsafe extern "system" fn GetConvertInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows_core::GUID, lplpclsidexclude: *mut *mut ::windows_core::GUID, lpcclsidexclude: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConvertInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpclassid), ::core::mem::transmute_copy(&lpwformat), ::core::mem::transmute_copy(&lpconvertdefaultclassid), ::core::mem::transmute_copy(&lplpclsidexclude), ::core::mem::transmute_copy(&lpcclsidexclude)).into()
        }
        unsafe extern "system" fn ConvertObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertObject(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&clsidnew)).into()
        }
        unsafe extern "system" fn GetViewInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&phmetapict), ::core::mem::transmute_copy(&pdvaspect), ::core::mem::transmute_copy(&pncurrentscale)).into()
        }
        unsafe extern "system" fn SetViewInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetViewInfo(::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&hmetapict), ::core::mem::transmute_copy(&dvaspect), ::core::mem::transmute_copy(&ncurrentscale), ::core::mem::transmute_copy(&brelativetoorig)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetConvertInfo: GetConvertInfo::<Identity, Impl, OFFSET>,
            ConvertObject: ConvertObject::<Identity, Impl, OFFSET>,
            GetViewInfo: GetViewInfo::<Identity, Impl, OFFSET>,
            SetViewInfo: SetViewInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleUIObjInfoW as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUndoManager_Impl: Sized {
    fn Open(&self, ppuu: ::core::option::Option<&IOleParentUndoUnit>) -> ::windows_core::Result<()>;
    fn Close(&self, ppuu: ::core::option::Option<&IOleParentUndoUnit>, fcommit: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Add(&self, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn GetOpenParentState(&self) -> ::windows_core::Result<u32>;
    fn DiscardFrom(&self, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn UndoTo(&self, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn RedoTo(&self, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn EnumUndoable(&self) -> ::windows_core::Result<IEnumOleUndoUnits>;
    fn EnumRedoable(&self) -> ::windows_core::Result<IEnumOleUndoUnits>;
    fn GetLastUndoDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetLastRedoDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Enable(&self, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleUndoManager {}
#[cfg(feature = "Win32_Foundation")]
impl IOleUndoManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>() -> IOleUndoManager_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::windows_core::from_raw_borrowed(&ppuu)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void, fcommit: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close(::windows_core::from_raw_borrowed(&ppuu), ::core::mem::transmute_copy(&fcommit)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn GetOpenParentState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOpenParentState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardFrom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DiscardFrom(::windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn UndoTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UndoTo(::windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn RedoTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RedoTo(::windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn EnumUndoable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumUndoable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRedoable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumRedoable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastUndoDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastUndoDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastRedoDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastRedoDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enable(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleUndoManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IOleUndoUnit_Impl: Sized {
    fn Do(&self, pundomanager: ::core::option::Option<&IOleUndoManager>) -> ::windows_core::Result<()>;
    fn GetDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetUnitType(&self, pclsid: *mut ::windows_core::GUID, plid: *mut i32) -> ::windows_core::Result<()>;
    fn OnNextAdd(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IOleUndoUnit {}
impl IOleUndoUnit_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoUnit_Impl, const OFFSET: isize>() -> IOleUndoUnit_Vtbl {
        unsafe extern "system" fn Do<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pundomanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Do(::windows_core::from_raw_borrowed(&pundomanager)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnitType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID, plid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUnitType(::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn OnNextAdd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNextAdd().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Do: Do::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetUnitType: GetUnitType::<Identity, Impl, OFFSET>,
            OnNextAdd: OnNextAdd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleUndoUnit as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleWindow_Impl: Sized {
    fn GetWindow(&self) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn ContextSensitiveHelp(&self, fentermode: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IOleWindow {}
#[cfg(feature = "Win32_Foundation")]
impl IOleWindow_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleWindow_Impl, const OFFSET: isize>() -> IOleWindow_Vtbl {
        unsafe extern "system" fn GetWindow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOleWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ContextSensitiveHelp(::core::mem::transmute_copy(&fentermode)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOleWindow as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IParseDisplayName_Impl: Sized {
    fn ParseDisplayName(&self, pbc: ::core::option::Option<&super::Com::IBindCtx>, pszdisplayname: &::windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<super::Com::IMoniker>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IParseDisplayName {}
#[cfg(feature = "Win32_System_Com")]
impl IParseDisplayName_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IParseDisplayName_Impl, const OFFSET: isize>() -> IParseDisplayName_Vtbl {
        unsafe extern "system" fn ParseDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IParseDisplayName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pszdisplayname: ::windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParseDisplayName(::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute(&pszdisplayname), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&ppmkout)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseDisplayName: ParseDisplayName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IParseDisplayName as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IPerPropertyBrowsing_Impl: Sized {
    fn GetDisplayString(&self, dispid: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MapPropertyToPage(&self, dispid: i32) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetPredefinedStrings(&self, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows_core::Result<()>;
    fn GetPredefinedValue(&self, dispid: i32, dwcookie: u32) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPerPropertyBrowsing {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl IPerPropertyBrowsing_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>() -> IPerPropertyBrowsing_Vtbl {
        unsafe extern "system" fn GetDisplayString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayString(::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapPropertyToPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MapPropertyToPage(::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPredefinedStrings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPredefinedStrings(::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&pcastringsout), ::core::mem::transmute_copy(&pcacookiesout)).into()
        }
        unsafe extern "system" fn GetPredefinedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, dwcookie: u32, pvarout: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPredefinedValue(::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&dwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayString: GetDisplayString::<Identity, Impl, OFFSET>,
            MapPropertyToPage: MapPropertyToPage::<Identity, Impl, OFFSET>,
            GetPredefinedStrings: GetPredefinedStrings::<Identity, Impl, OFFSET>,
            GetPredefinedValue: GetPredefinedValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPerPropertyBrowsing as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPersistPropertyBag_Impl: Sized + super::Com::IPersist_Impl {
    fn InitNew(&self) -> ::windows_core::Result<()>;
    fn Load(&self, ppropbag: ::core::option::Option<&super::Com::StructuredStorage::IPropertyBag>, perrorlog: ::core::option::Option<&super::Com::IErrorLog>) -> ::windows_core::Result<()>;
    fn Save(&self, ppropbag: ::core::option::Option<&super::Com::StructuredStorage::IPropertyBag>, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::RuntimeName for IPersistPropertyBag {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IPersistPropertyBag_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistPropertyBag_Impl, const OFFSET: isize>() -> IPersistPropertyBag_Vtbl {
        unsafe extern "system" fn InitNew<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitNew().into()
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, perrorlog: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::windows_core::from_raw_borrowed(&ppropbag), ::windows_core::from_raw_borrowed(&perrorlog)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::windows_core::from_raw_borrowed(&ppropbag), ::core::mem::transmute_copy(&fcleardirty), ::core::mem::transmute_copy(&fsaveallproperties)).into()
        }
        Self {
            base__: super::Com::IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPersistPropertyBag as ::windows_core::ComInterface>::IID || iid == &<super::Com::IPersist as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPersistPropertyBag2_Impl: Sized + super::Com::IPersist_Impl {
    fn InitNew(&self) -> ::windows_core::Result<()>;
    fn Load(&self, ppropbag: ::core::option::Option<&super::Com::StructuredStorage::IPropertyBag2>, perrlog: ::core::option::Option<&super::Com::IErrorLog>) -> ::windows_core::Result<()>;
    fn Save(&self, ppropbag: ::core::option::Option<&super::Com::StructuredStorage::IPropertyBag2>, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsDirty(&self) -> ::windows_core::HRESULT;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::RuntimeName for IPersistPropertyBag2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IPersistPropertyBag2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>() -> IPersistPropertyBag2_Vtbl {
        unsafe extern "system" fn InitNew<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitNew().into()
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, perrlog: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::windows_core::from_raw_borrowed(&ppropbag), ::windows_core::from_raw_borrowed(&perrlog)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::windows_core::from_raw_borrowed(&ppropbag), ::core::mem::transmute_copy(&fcleardirty), ::core::mem::transmute_copy(&fsaveallproperties)).into()
        }
        unsafe extern "system" fn IsDirty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDirty()
        }
        Self {
            base__: super::Com::IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPersistPropertyBag2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IPersist as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPicture_Impl: Sized {
    fn Handle(&self) -> ::windows_core::Result<OLE_HANDLE>;
    fn hPal(&self) -> ::windows_core::Result<OLE_HANDLE>;
    fn Type(&self) -> ::windows_core::Result<PICTYPE>;
    fn Width(&self) -> ::windows_core::Result<i32>;
    fn Height(&self) -> ::windows_core::Result<i32>;
    fn Render(&self, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn set_hPal(&self, hpal: OLE_HANDLE) -> ::windows_core::Result<()>;
    fn CurDC(&self) -> ::windows_core::Result<super::super::Graphics::Gdi::HDC>;
    fn SelectPicture(&self, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut OLE_HANDLE) -> ::windows_core::Result<()>;
    fn KeepOriginalFormat(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetKeepOriginalFormat(&self, keep: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn PictureChanged(&self) -> ::windows_core::Result<()>;
    fn SaveAsFile(&self, pstream: ::core::option::Option<&super::Com::IStream>, fsavememcopy: super::super::Foundation::BOOL) -> ::windows_core::Result<i32>;
    fn Attributes(&self) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IPicture {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPicture_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>() -> IPicture_Vtbl {
        unsafe extern "system" fn Handle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut OLE_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hPal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phpal: *mut OLE_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hPal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phpal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut PICTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Width() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Height() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Render<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Render(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy), ::core::mem::transmute_copy(&xsrc), ::core::mem::transmute_copy(&ysrc), ::core::mem::transmute_copy(&cxsrc), ::core::mem::transmute_copy(&cysrc), ::core::mem::transmute_copy(&prcwbounds)).into()
        }
        unsafe extern "system" fn set_hPal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpal: OLE_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_hPal(::core::mem::transmute_copy(&hpal)).into()
        }
        unsafe extern "system" fn CurDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurDC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phdc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPicture<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut OLE_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectPicture(::core::mem::transmute_copy(&hdcin), ::core::mem::transmute_copy(&phdcout), ::core::mem::transmute_copy(&phbmpout)).into()
        }
        unsafe extern "system" fn KeepOriginalFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KeepOriginalFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pkeep, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKeepOriginalFormat(::core::mem::transmute_copy(&keep)).into()
        }
        unsafe extern "system" fn PictureChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PictureChanged().into()
        }
        unsafe extern "system" fn SaveAsFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaveAsFile(::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&fsavememcopy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwattr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPicture as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPicture2_Impl: Sized {
    fn Handle(&self) -> ::windows_core::Result<usize>;
    fn hPal(&self) -> ::windows_core::Result<usize>;
    fn Type(&self) -> ::windows_core::Result<i16>;
    fn Width(&self) -> ::windows_core::Result<i32>;
    fn Height(&self) -> ::windows_core::Result<i32>;
    fn Render(&self, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn set_hPal(&self, hpal: usize) -> ::windows_core::Result<()>;
    fn CurDC(&self) -> ::windows_core::Result<super::super::Graphics::Gdi::HDC>;
    fn SelectPicture(&self, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows_core::Result<()>;
    fn KeepOriginalFormat(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetKeepOriginalFormat(&self, keep: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn PictureChanged(&self) -> ::windows_core::Result<()>;
    fn SaveAsFile(&self, pstream: ::core::option::Option<&super::Com::IStream>, fsavememcopy: super::super::Foundation::BOOL) -> ::windows_core::Result<i32>;
    fn Attributes(&self) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IPicture2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPicture2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>() -> IPicture2_Vtbl {
        unsafe extern "system" fn Handle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hPal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phpal: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hPal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phpal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Width() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Height() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Render<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Render(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy), ::core::mem::transmute_copy(&xsrc), ::core::mem::transmute_copy(&ysrc), ::core::mem::transmute_copy(&cxsrc), ::core::mem::transmute_copy(&cysrc), ::core::mem::transmute_copy(&prcwbounds)).into()
        }
        unsafe extern "system" fn set_hPal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpal: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_hPal(::core::mem::transmute_copy(&hpal)).into()
        }
        unsafe extern "system" fn CurDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurDC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phdc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPicture<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectPicture(::core::mem::transmute_copy(&hdcin), ::core::mem::transmute_copy(&phdcout), ::core::mem::transmute_copy(&phbmpout)).into()
        }
        unsafe extern "system" fn KeepOriginalFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KeepOriginalFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pkeep, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKeepOriginalFormat(::core::mem::transmute_copy(&keep)).into()
        }
        unsafe extern "system" fn PictureChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PictureChanged().into()
        }
        unsafe extern "system" fn SaveAsFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaveAsFile(::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&fsavememcopy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwattr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPicture2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IPictureDisp_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPictureDisp {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl IPictureDisp_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPictureDisp_Impl, const OFFSET: isize>() -> IPictureDisp_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPictureDisp as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPointerInactive_Impl: Sized {
    fn GetActivationPolicy(&self) -> ::windows_core::Result<POINTERINACTIVE>;
    fn OnInactiveMouseMove(&self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows_core::Result<()>;
    fn OnInactiveSetCursor(&self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IPointerInactive {}
#[cfg(feature = "Win32_Foundation")]
impl IPointerInactive_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPointerInactive_Impl, const OFFSET: isize>() -> IPointerInactive_Vtbl {
        unsafe extern "system" fn GetActivationPolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPointerInactive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpolicy: *mut POINTERINACTIVE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActivationPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpolicy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInactiveMouseMove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPointerInactive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInactiveMouseMove(::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&grfkeystate)).into()
        }
        unsafe extern "system" fn OnInactiveSetCursor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPointerInactive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInactiveSetCursor(::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&dwmousemsg), ::core::mem::transmute_copy(&fsetalways)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetActivationPolicy: GetActivationPolicy::<Identity, Impl, OFFSET>,
            OnInactiveMouseMove: OnInactiveMouseMove::<Identity, Impl, OFFSET>,
            OnInactiveSetCursor: OnInactiveSetCursor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPointerInactive as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPrint_Impl: Sized {
    fn SetInitialPageNum(&self, nfirstpage: i32) -> ::windows_core::Result<()>;
    fn GetPageInfo(&self, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows_core::Result<()>;
    fn Print(&self, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: ::core::option::Option<&IContinueCallback>, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::RuntimeName for IPrint {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IPrint_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrint_Impl, const OFFSET: isize>() -> IPrint_Vtbl {
        unsafe extern "system" fn SetInitialPageNum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nfirstpage: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitialPageNum(::core::mem::transmute_copy(&nfirstpage)).into()
        }
        unsafe extern "system" fn GetPageInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPageInfo(::core::mem::transmute_copy(&pnfirstpage), ::core::mem::transmute_copy(&pcpages)).into()
        }
        unsafe extern "system" fn Print<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: *mut ::core::ffi::c_void, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Print(::core::mem::transmute_copy(&grfflags), ::core::mem::transmute_copy(&pptd), ::core::mem::transmute_copy(&pppageset), ::core::mem::transmute_copy(&pstgmoptions), ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&nfirstpage), ::core::mem::transmute_copy(&pcpagesprinted), ::core::mem::transmute_copy(&pnlastpage)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInitialPageNum: SetInitialPageNum::<Identity, Impl, OFFSET>,
            GetPageInfo: GetPageInfo::<Identity, Impl, OFFSET>,
            Print: Print::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPrint as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IPropertyNotifySink_Impl: Sized {
    fn OnChanged(&self, dispid: i32) -> ::windows_core::Result<()>;
    fn OnRequestEdit(&self, dispid: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPropertyNotifySink {}
impl IPropertyNotifySink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyNotifySink_Impl, const OFFSET: isize>() -> IPropertyNotifySink_Vtbl {
        unsafe extern "system" fn OnChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChanged(::core::mem::transmute_copy(&dispid)).into()
        }
        unsafe extern "system" fn OnRequestEdit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRequestEdit(::core::mem::transmute_copy(&dispid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnChanged: OnChanged::<Identity, Impl, OFFSET>,
            OnRequestEdit: OnRequestEdit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyNotifySink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPropertyPage_Impl: Sized {
    fn SetPageSite(&self, ppagesite: ::core::option::Option<&IPropertyPageSite>) -> ::windows_core::Result<()>;
    fn Activate(&self, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Deactivate(&self) -> ::windows_core::Result<()>;
    fn GetPageInfo(&self, ppageinfo: *mut PROPPAGEINFO) -> ::windows_core::Result<()>;
    fn SetObjects(&self, cobjects: u32, ppunk: *const ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Show(&self, ncmdshow: u32) -> ::windows_core::Result<()>;
    fn Move(&self, prect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn IsPageDirty(&self) -> ::windows_core::Result<()>;
    fn Apply(&self) -> ::windows_core::Result<()>;
    fn Help(&self, pszhelpdir: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IPropertyPage {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPropertyPage_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>() -> IPropertyPage_Vtbl {
        unsafe extern "system" fn SetPageSite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagesite: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPageSite(::windows_core::from_raw_borrowed(&ppagesite)).into()
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&bmodal)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate().into()
        }
        unsafe extern "system" fn GetPageInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppageinfo: *mut PROPPAGEINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPageInfo(::core::mem::transmute_copy(&ppageinfo)).into()
        }
        unsafe extern "system" fn SetObjects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32, ppunk: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjects(::core::mem::transmute_copy(&cobjects), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncmdshow: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show(::core::mem::transmute_copy(&ncmdshow)).into()
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn IsPageDirty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsPageDirty().into()
        }
        unsafe extern "system" fn Apply<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Apply().into()
        }
        unsafe extern "system" fn Help<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelpdir: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Help(::core::mem::transmute(&pszhelpdir)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TranslateAccelerator(::core::mem::transmute_copy(&pmsg)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyPage as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPropertyPage2_Impl: Sized + IPropertyPage_Impl {
    fn EditProperty(&self, dispid: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IPropertyPage2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPropertyPage2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage2_Impl, const OFFSET: isize>() -> IPropertyPage2_Vtbl {
        unsafe extern "system" fn EditProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EditProperty(::core::mem::transmute_copy(&dispid)).into()
        }
        Self { base__: IPropertyPage_Vtbl::new::<Identity, Impl, OFFSET>(), EditProperty: EditProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyPage2 as ::windows_core::ComInterface>::IID || iid == &<IPropertyPage as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPropertyPageSite_Impl: Sized {
    fn OnStatusChange(&self, dwflags: &PROPPAGESTATUS) -> ::windows_core::Result<()>;
    fn GetLocaleID(&self) -> ::windows_core::Result<u32>;
    fn GetPageContainer(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IPropertyPageSite {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPropertyPageSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPageSite_Impl, const OFFSET: isize>() -> IPropertyPageSite_Vtbl {
        unsafe extern "system" fn OnStatusChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStatusChange(::core::mem::transmute(&dwflags)).into()
        }
        unsafe extern "system" fn GetLocaleID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocaleid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocaleID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plocaleid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPageContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TranslateAccelerator(::core::mem::transmute_copy(&pmsg)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            GetLocaleID: GetLocaleID::<Identity, Impl, OFFSET>,
            GetPageContainer: GetPageContainer::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyPageSite as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IProtectFocus_Impl: Sized {
    fn AllowFocusChange(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IProtectFocus {}
#[cfg(feature = "Win32_Foundation")]
impl IProtectFocus_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectFocus_Impl, const OFFSET: isize>() -> IProtectFocus_Vtbl {
        unsafe extern "system" fn AllowFocusChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectFocus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallow: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowFocusChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AllowFocusChange: AllowFocusChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProtectFocus as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IProtectedModeMenuServices_Impl: Sized {
    fn CreateMenu(&self) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
    fn LoadMenu(&self, pszmodulename: &::windows_core::PCWSTR, pszmenuname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
    fn LoadMenuID(&self, pszmodulename: &::windows_core::PCWSTR, wresourceid: u16) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows_core::RuntimeName for IProtectedModeMenuServices {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IProtectedModeMenuServices_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectedModeMenuServices_Impl, const OFFSET: isize>() -> IProtectedModeMenuServices_Vtbl {
        unsafe extern "system" fn CreateMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMenu() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmodulename: ::windows_core::PCWSTR, pszmenuname: ::windows_core::PCWSTR, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadMenu(::core::mem::transmute(&pszmodulename), ::core::mem::transmute(&pszmenuname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMenuID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmodulename: ::windows_core::PCWSTR, wresourceid: u16, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadMenuID(::core::mem::transmute(&pszmodulename), ::core::mem::transmute_copy(&wresourceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMenu: CreateMenu::<Identity, Impl, OFFSET>,
            LoadMenu: LoadMenu::<Identity, Impl, OFFSET>,
            LoadMenuID: LoadMenuID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProtectedModeMenuServices as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideClassInfo_Impl: Sized {
    fn GetClassInfo(&self) -> ::windows_core::Result<super::Com::ITypeInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IProvideClassInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IProvideClassInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideClassInfo_Impl, const OFFSET: isize>() -> IProvideClassInfo_Vtbl {
        unsafe extern "system" fn GetClassInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppti: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClassInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppti, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassInfo: GetClassInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProvideClassInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideClassInfo2_Impl: Sized + IProvideClassInfo_Impl {
    fn GetGUID(&self, dwguidkind: u32) -> ::windows_core::Result<::windows_core::GUID>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IProvideClassInfo2 {}
#[cfg(feature = "Win32_System_Com")]
impl IProvideClassInfo2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideClassInfo2_Impl, const OFFSET: isize>() -> IProvideClassInfo2_Vtbl {
        unsafe extern "system" fn GetGUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideClassInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwguidkind: u32, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGUID(::core::mem::transmute_copy(&dwguidkind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IProvideClassInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetGUID: GetGUID::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProvideClassInfo2 as ::windows_core::ComInterface>::IID || iid == &<IProvideClassInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideMultipleClassInfo_Impl: Sized + IProvideClassInfo2_Impl {
    fn GetMultiTypeInfoCount(&self) -> ::windows_core::Result<u32>;
    fn GetInfoOfIndex(&self, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut ::core::option::Option<super::Com::ITypeInfo>, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows_core::GUID, piidsource: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IProvideMultipleClassInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IProvideMultipleClassInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideMultipleClassInfo_Impl, const OFFSET: isize>() -> IProvideMultipleClassInfo_Vtbl {
        unsafe extern "system" fn GetMultiTypeInfoCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideMultipleClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcti: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMultiTypeInfoCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcti, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfoOfIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideMultipleClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut *mut ::core::ffi::c_void, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows_core::GUID, piidsource: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInfoOfIndex(::core::mem::transmute_copy(&iti), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppticoclass), ::core::mem::transmute_copy(&pdwtiflags), ::core::mem::transmute_copy(&pcdispidreserved), ::core::mem::transmute_copy(&piidprimary), ::core::mem::transmute_copy(&piidsource)).into()
        }
        Self {
            base__: IProvideClassInfo2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMultiTypeInfoCount: GetMultiTypeInfoCount::<Identity, Impl, OFFSET>,
            GetInfoOfIndex: GetInfoOfIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProvideMultipleClassInfo as ::windows_core::ComInterface>::IID || iid == &<IProvideClassInfo as ::windows_core::ComInterface>::IID || iid == &<IProvideClassInfo2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IProvideRuntimeContext_Impl: Sized {
    fn GetCurrentSourceContext(&self, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IProvideRuntimeContext {}
#[cfg(feature = "Win32_Foundation")]
impl IProvideRuntimeContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideRuntimeContext_Impl, const OFFSET: isize>() -> IProvideRuntimeContext_Vtbl {
        unsafe extern "system" fn GetCurrentSourceContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideRuntimeContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentSourceContext(::core::mem::transmute_copy(&pdwcontext), ::core::mem::transmute_copy(&pfexecutingglobalcode)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCurrentSourceContext: GetCurrentSourceContext::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProvideRuntimeContext as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IQuickActivate_Impl: Sized {
    fn QuickActivate(&self, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows_core::Result<()>;
    fn SetContentExtent(&self, psizel: *const super::super::Foundation::SIZE) -> ::windows_core::Result<()>;
    fn GetContentExtent(&self) -> ::windows_core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IQuickActivate {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IQuickActivate_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IQuickActivate_Impl, const OFFSET: isize>() -> IQuickActivate_Vtbl {
        unsafe extern "system" fn QuickActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IQuickActivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QuickActivate(::core::mem::transmute_copy(&pqacontainer), ::core::mem::transmute_copy(&pqacontrol)).into()
        }
        unsafe extern "system" fn SetContentExtent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IQuickActivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizel: *const super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContentExtent(::core::mem::transmute_copy(&psizel)).into()
        }
        unsafe extern "system" fn GetContentExtent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IQuickActivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizel: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentExtent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psizel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QuickActivate: QuickActivate::<Identity, Impl, OFFSET>,
            SetContentExtent: SetContentExtent::<Identity, Impl, OFFSET>,
            GetContentExtent: GetContentExtent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IQuickActivate as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IRecordInfo_Impl: Sized {
    fn RecordInit(&self, pvnew: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RecordClear(&self, pvexisting: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RecordCopy(&self, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetGuid(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSize(&self) -> ::windows_core::Result<u32>;
    fn GetTypeInfo(&self) -> ::windows_core::Result<super::Com::ITypeInfo>;
    fn GetField(&self, pvdata: *const ::core::ffi::c_void, szfieldname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetFieldNoCopy(&self, pvdata: *const ::core::ffi::c_void, szfieldname: &::windows_core::PCWSTR, pvarfield: *mut super::Variant::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn PutField(&self, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: &::windows_core::PCWSTR, pvarfield: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PutFieldNoCopy(&self, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: &::windows_core::PCWSTR, pvarfield: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetFieldNames(&self, pcnames: *mut u32, rgbstrnames: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsMatchingType(&self, precordinfo: ::core::option::Option<&IRecordInfo>) -> super::super::Foundation::BOOL;
    fn RecordCreate(&self) -> *mut ::core::ffi::c_void;
    fn RecordCreateCopy(&self, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RecordDestroy(&self, pvrecord: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IRecordInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl IRecordInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>() -> IRecordInfo_Vtbl {
        unsafe extern "system" fn RecordInit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecordInit(::core::mem::transmute_copy(&pvnew)).into()
        }
        unsafe extern "system" fn RecordClear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecordClear(::core::mem::transmute_copy(&pvexisting)).into()
        }
        unsafe extern "system" fn RecordCopy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecordCopy(::core::mem::transmute_copy(&pvexisting), ::core::mem::transmute_copy(&pvnew)).into()
        }
        unsafe extern "system" fn GetGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptypeinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetField<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: ::windows_core::PCWSTR, pvarfield: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetField(::core::mem::transmute_copy(&pvdata), ::core::mem::transmute(&szfieldname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarfield, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFieldNoCopy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: ::windows_core::PCWSTR, pvarfield: *mut super::Variant::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFieldNoCopy(::core::mem::transmute_copy(&pvdata), ::core::mem::transmute(&szfieldname), ::core::mem::transmute_copy(&pvarfield), ::core::mem::transmute_copy(&ppvdatacarray)).into()
        }
        unsafe extern "system" fn PutField<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: ::windows_core::PCWSTR, pvarfield: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutField(::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute(&szfieldname), ::core::mem::transmute_copy(&pvarfield)).into()
        }
        unsafe extern "system" fn PutFieldNoCopy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: ::windows_core::PCWSTR, pvarfield: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutFieldNoCopy(::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute(&szfieldname), ::core::mem::transmute_copy(&pvarfield)).into()
        }
        unsafe extern "system" fn GetFieldNames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnames: *mut u32, rgbstrnames: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFieldNames(::core::mem::transmute_copy(&pcnames), ::core::mem::transmute_copy(&rgbstrnames)).into()
        }
        unsafe extern "system" fn IsMatchingType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordinfo: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsMatchingType(::windows_core::from_raw_borrowed(&precordinfo))
        }
        unsafe extern "system" fn RecordCreate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecordCreate()
        }
        unsafe extern "system" fn RecordCreateCopy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecordCreateCopy(::core::mem::transmute_copy(&pvsource), ::core::mem::transmute_copy(&ppvdest)).into()
        }
        unsafe extern "system" fn RecordDestroy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvrecord: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecordDestroy(::core::mem::transmute_copy(&pvrecord)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRecordInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimpleFrameSite_Impl: Sized {
    fn PreMessageFilter(&self, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows_core::Result<()>;
    fn PostMessageFilter(&self, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISimpleFrameSite {}
#[cfg(feature = "Win32_Foundation")]
impl ISimpleFrameSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimpleFrameSite_Impl, const OFFSET: isize>() -> ISimpleFrameSite_Vtbl {
        unsafe extern "system" fn PreMessageFilter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimpleFrameSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreMessageFilter(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wp), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&plresult), ::core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn PostMessageFilter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimpleFrameSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostMessageFilter(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wp), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&plresult), ::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreMessageFilter: PreMessageFilter::<Identity, Impl, OFFSET>,
            PostMessageFilter: PostMessageFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISimpleFrameSite as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait ISpecifyPropertyPages_Impl: Sized {
    fn GetPages(&self) -> ::windows_core::Result<CAUUID>;
}
impl ::windows_core::RuntimeName for ISpecifyPropertyPages {}
impl ISpecifyPropertyPages_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpecifyPropertyPages_Impl, const OFFSET: isize>() -> ISpecifyPropertyPages_Vtbl {
        unsafe extern "system" fn GetPages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpecifyPropertyPages_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppages: *mut CAUUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPages: GetPages::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpecifyPropertyPages as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITypeChangeEvents_Impl: Sized {
    fn RequestTypeChange(&self, changekind: CHANGEKIND, ptinfobefore: ::core::option::Option<&super::Com::ITypeInfo>, pstrname: &::windows_core::PCWSTR) -> ::windows_core::Result<i32>;
    fn AfterTypeChange(&self, changekind: CHANGEKIND, ptinfoafter: ::core::option::Option<&super::Com::ITypeInfo>, pstrname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITypeChangeEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ITypeChangeEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeChangeEvents_Impl, const OFFSET: isize>() -> ITypeChangeEvents_Vtbl {
        unsafe extern "system" fn RequestTypeChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeChangeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfobefore: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR, pfcancel: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestTypeChange(::core::mem::transmute_copy(&changekind), ::windows_core::from_raw_borrowed(&ptinfobefore), ::core::mem::transmute(&pstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcancel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AfterTypeChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeChangeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfoafter: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AfterTypeChange(::core::mem::transmute_copy(&changekind), ::windows_core::from_raw_borrowed(&ptinfoafter), ::core::mem::transmute(&pstrname)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestTypeChange: RequestTypeChange::<Identity, Impl, OFFSET>,
            AfterTypeChange: AfterTypeChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITypeChangeEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITypeFactory_Impl: Sized {
    fn CreateFromTypeInfo(&self, ptypeinfo: ::core::option::Option<&super::Com::ITypeInfo>, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITypeFactory {}
#[cfg(feature = "Win32_System_Com")]
impl ITypeFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeFactory_Impl, const OFFSET: isize>() -> ITypeFactory_Vtbl {
        unsafe extern "system" fn CreateFromTypeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeinfo: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFromTypeInfo(::windows_core::from_raw_borrowed(&ptypeinfo), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppv, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateFromTypeInfo: CreateFromTypeInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITypeFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait ITypeMarshal_Impl: Sized {
    fn Size(&self, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<u32>;
    fn Marshal(&self, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> ::windows_core::Result<()>;
    fn Unmarshal(&self, pvtype: *mut ::core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> ::windows_core::Result<()>;
    fn Free(&self, pvtype: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITypeMarshal {}
impl ITypeMarshal_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeMarshal_Impl, const OFFSET: isize>() -> ITypeMarshal_Vtbl {
        unsafe extern "system" fn Size<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, psize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size(::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Marshal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Marshal(::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&cbbufferlength), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbwritten)).into()
        }
        unsafe extern "system" fn Unmarshal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *mut ::core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unmarshal(::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cbbufferlength), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbread)).into()
        }
        unsafe extern "system" fn Free<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Free(::core::mem::transmute_copy(&pvtype)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Size: Size::<Identity, Impl, OFFSET>,
            Marshal: Marshal::<Identity, Impl, OFFSET>,
            Unmarshal: Unmarshal::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITypeMarshal as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IVBFormat_Impl: Sized {
    fn Format(&self, vdata: *mut super::Variant::VARIANT, bstrformat: &::windows_core::BSTR, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBFormat {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl IVBFormat_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBFormat_Impl, const OFFSET: isize>() -> IVBFormat_Vtbl {
        unsafe extern "system" fn Format<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdata: *mut super::Variant::VARIANT, bstrformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Format(::core::mem::transmute_copy(&vdata), ::core::mem::transmute(&bstrformat), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&sfirstdayofweek), ::core::mem::transmute_copy(&sfirstweekofyear), ::core::mem::transmute_copy(&rcb)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Format: Format::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBFormat as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IVBGetControl_Impl: Sized {
    fn EnumControls(&self, dwolecontf: &OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS) -> ::windows_core::Result<super::Com::IEnumUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IVBGetControl {}
#[cfg(feature = "Win32_System_Com")]
impl IVBGetControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBGetControl_Impl, const OFFSET: isize>() -> IVBGetControl_Vtbl {
        unsafe extern "system" fn EnumControls<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBGetControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwolecontf: u32, dwwhich: ENUM_CONTROLS_WHICH_FLAGS, ppenumunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumControls(::core::mem::transmute(&dwolecontf), ::core::mem::transmute_copy(&dwwhich)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumControls: EnumControls::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBGetControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IVariantChangeType_Impl: Sized {
    fn ChangeType(&self, pvardst: *mut super::Variant::VARIANT, pvarsrc: *const super::Variant::VARIANT, lcid: u32, vtnew: super::Variant::VARENUM) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVariantChangeType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl IVariantChangeType_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVariantChangeType_Impl, const OFFSET: isize>() -> IVariantChangeType_Vtbl {
        unsafe extern "system" fn ChangeType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVariantChangeType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardst: *mut super::Variant::VARIANT, pvarsrc: *const super::Variant::VARIANT, lcid: u32, vtnew: super::Variant::VARENUM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangeType(::core::mem::transmute_copy(&pvardst), ::core::mem::transmute_copy(&pvarsrc), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&vtnew)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ChangeType: ChangeType::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVariantChangeType as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObject_Impl: Sized {
    fn Draw(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows_core::Result<()>;
    fn GetColorSet(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows_core::Result<()>;
    fn Freeze(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows_core::Result<()>;
    fn Unfreeze(&self, dwfreeze: u32) -> ::windows_core::Result<()>;
    fn SetAdvise(&self, aspects: super::Com::DVASPECT, advf: u32, padvsink: ::core::option::Option<&super::Com::IAdviseSink>) -> ::windows_core::Result<()>;
    fn GetAdvise(&self, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::core::option::Option<super::Com::IAdviseSink>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IViewObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: isize>() -> IViewObject_Vtbl {
        unsafe extern "system" fn Draw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Draw(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hdctargetdev), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&lprcbounds), ::core::mem::transmute_copy(&lprcwbounds), ::core::mem::transmute_copy(&pfncontinue), ::core::mem::transmute_copy(&dwcontinue)).into()
        }
        unsafe extern "system" fn GetColorSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorSet(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&ppcolorset)).into()
        }
        unsafe extern "system" fn Freeze<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Freeze(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&pdwfreeze)).into()
        }
        unsafe extern "system" fn Unfreeze<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfreeze: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unfreeze(::core::mem::transmute_copy(&dwfreeze)).into()
        }
        unsafe extern "system" fn SetAdvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aspects: super::Com::DVASPECT, advf: u32, padvsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAdvise(::core::mem::transmute_copy(&aspects), ::core::mem::transmute_copy(&advf), ::windows_core::from_raw_borrowed(&padvsink)).into()
        }
        unsafe extern "system" fn GetAdvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdvise(::core::mem::transmute_copy(&paspects), ::core::mem::transmute_copy(&padvf), ::core::mem::transmute_copy(&ppadvsink)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Draw: Draw::<Identity, Impl, OFFSET>,
            GetColorSet: GetColorSet::<Identity, Impl, OFFSET>,
            Freeze: Freeze::<Identity, Impl, OFFSET>,
            Unfreeze: Unfreeze::<Identity, Impl, OFFSET>,
            SetAdvise: SetAdvise::<Identity, Impl, OFFSET>,
            GetAdvise: GetAdvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IViewObject as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObject2_Impl: Sized + IViewObject_Impl {
    fn GetExtent(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE) -> ::windows_core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IViewObject2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObject2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObject2_Impl, const OFFSET: isize>() -> IViewObject2_Vtbl {
        unsafe extern "system" fn GetExtent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, lpsizel: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExtent(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&ptd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpsizel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IViewObject_Vtbl::new::<Identity, Impl, OFFSET>(), GetExtent: GetExtent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IViewObject2 as ::windows_core::ComInterface>::IID || iid == &<IViewObject as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObjectEx_Impl: Sized + IViewObject2_Impl {
    fn GetRect(&self, dwaspect: u32) -> ::windows_core::Result<super::super::Foundation::RECTL>;
    fn GetViewStatus(&self) -> ::windows_core::Result<u32>;
    fn QueryHitPoint(&self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: &super::super::Foundation::POINT, lclosehint: i32) -> ::windows_core::Result<u32>;
    fn QueryHitRect(&self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32) -> ::windows_core::Result<u32>;
    fn GetNaturalExtent(&self, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const DVEXTENTINFO) -> ::windows_core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IViewObjectEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObjectEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: isize>() -> IViewObjectEx_Vtbl {
        unsafe extern "system" fn GetRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prect: *mut super::super::Foundation::RECTL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRect(::core::mem::transmute_copy(&dwaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetViewStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHitPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32, phitresult: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryHitPoint(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute(&ptlloc), ::core::mem::transmute_copy(&lclosehint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phitresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHitRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32, phitresult: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryHitRect(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&prectloc), ::core::mem::transmute_copy(&lclosehint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phitresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNaturalExtent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const DVEXTENTINFO, psizel: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNaturalExtent(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&pextentinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psizel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IViewObject2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRect: GetRect::<Identity, Impl, OFFSET>,
            GetViewStatus: GetViewStatus::<Identity, Impl, OFFSET>,
            QueryHitPoint: QueryHitPoint::<Identity, Impl, OFFSET>,
            QueryHitRect: QueryHitRect::<Identity, Impl, OFFSET>,
            GetNaturalExtent: GetNaturalExtent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IViewObjectEx as ::windows_core::ComInterface>::IID || iid == &<IViewObject as ::windows_core::ComInterface>::IID || iid == &<IViewObject2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"implement\"`*"]
pub trait IZoomEvents_Impl: Sized {
    fn OnZoomPercentChanged(&self, ulzoompercent: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IZoomEvents {}
impl IZoomEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IZoomEvents_Impl, const OFFSET: isize>() -> IZoomEvents_Vtbl {
        unsafe extern "system" fn OnZoomPercentChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IZoomEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulzoompercent: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnZoomPercentChanged(::core::mem::transmute_copy(&ulzoompercent)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnZoomPercentChanged: OnZoomPercentChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IZoomEvents as ::windows_core::ComInterface>::IID
    }
}
