#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait IDebugExtendedProperty_Impl: Sized + IDebugProperty_Impl {
    fn GetExtendedPropertyInfo(&self, dwfieldspec: u32, nradix: u32, pextendedpropertyinfo: *mut ExtendedDebugPropertyInfo) -> ::windows::core::Result<()>;
    fn EnumExtendedMembers(&self, dwfieldspec: u32, nradix: u32) -> ::windows::core::Result<IEnumDebugExtendedPropertyInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDebugExtendedProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl IDebugExtendedProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugExtendedProperty_Impl, const OFFSET: isize>() -> IDebugExtendedProperty_Vtbl {
        unsafe extern "system" fn GetExtendedPropertyInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugExtendedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, pextendedpropertyinfo: *mut ExtendedDebugPropertyInfo) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExtendedPropertyInfo(::core::mem::transmute_copy(&dwfieldspec), ::core::mem::transmute_copy(&nradix), ::core::mem::transmute_copy(&pextendedpropertyinfo)).into()
        }
        unsafe extern "system" fn EnumExtendedMembers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugExtendedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, ppeepi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumExtendedMembers(::core::mem::transmute_copy(&dwfieldspec), ::core::mem::transmute_copy(&nradix)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeepi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDebugProperty_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetExtendedPropertyInfo: GetExtendedPropertyInfo::<Identity, Impl, OFFSET>,
            EnumExtendedMembers: EnumExtendedMembers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebugExtendedProperty as ::windows::core::ComInterface>::IID || iid == &<IDebugProperty as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDebugProperty_Impl: Sized {
    fn GetPropertyInfo(&self, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> ::windows::core::Result<()>;
    fn GetExtendedInfo(&self, cinfos: u32, rgguidextendedinfo: *const ::windows::core::GUID, rgvar: *mut super::super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetValueAsString(&self, pszvalue: &::windows::core::PCWSTR, nradix: u32) -> ::windows::core::Result<()>;
    fn EnumMembers(&self, dwfieldspec: u32, nradix: u32, refiid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDebugPropertyInfo>;
    fn GetParent(&self) -> ::windows::core::Result<IDebugProperty>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDebugProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDebugProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: isize>() -> IDebugProperty_Vtbl {
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyInfo(::core::mem::transmute_copy(&dwfieldspec), ::core::mem::transmute_copy(&nradix), ::core::mem::transmute_copy(&ppropertyinfo)).into()
        }
        unsafe extern "system" fn GetExtendedInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinfos: u32, rgguidextendedinfo: *const ::windows::core::GUID, rgvar: *mut super::super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExtendedInfo(::core::mem::transmute_copy(&cinfos), ::core::mem::transmute_copy(&rgguidextendedinfo), ::core::mem::transmute_copy(&rgvar)).into()
        }
        unsafe extern "system" fn SetValueAsString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: ::windows::core::PCWSTR, nradix: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValueAsString(::core::mem::transmute(&pszvalue), ::core::mem::transmute_copy(&nradix)).into()
        }
        unsafe extern "system" fn EnumMembers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, refiid: *const ::windows::core::GUID, ppepi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumMembers(::core::mem::transmute_copy(&dwfieldspec), ::core::mem::transmute_copy(&nradix), ::core::mem::transmute_copy(&refiid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppepi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdebugprop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdebugprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            GetExtendedInfo: GetExtendedInfo::<Identity, Impl, OFFSET>,
            SetValueAsString: SetValueAsString::<Identity, Impl, OFFSET>,
            EnumMembers: EnumMembers::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebugProperty as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"implement\"`*"]
pub trait IDebugPropertyEnumType_All_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
impl ::windows::core::RuntimeName for IDebugPropertyEnumType_All {}
impl IDebugPropertyEnumType_All_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugPropertyEnumType_All_Impl, const OFFSET: isize>() -> IDebugPropertyEnumType_All_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugPropertyEnumType_All_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__idebugpropertyenumtype_all0000: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__idebugpropertyenumtype_all0000, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetName: GetName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_All as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"implement\"`*"]
pub trait IDebugPropertyEnumType_Arguments_Impl: Sized + IDebugPropertyEnumType_All_Impl {}
impl ::windows::core::RuntimeName for IDebugPropertyEnumType_Arguments {}
impl IDebugPropertyEnumType_Arguments_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugPropertyEnumType_Arguments_Impl, const OFFSET: isize>() -> IDebugPropertyEnumType_Arguments_Vtbl {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_Arguments as ::windows::core::ComInterface>::IID || iid == &<IDebugPropertyEnumType_All as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"implement\"`*"]
pub trait IDebugPropertyEnumType_Locals_Impl: Sized + IDebugPropertyEnumType_All_Impl {}
impl ::windows::core::RuntimeName for IDebugPropertyEnumType_Locals {}
impl IDebugPropertyEnumType_Locals_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugPropertyEnumType_Locals_Impl, const OFFSET: isize>() -> IDebugPropertyEnumType_Locals_Vtbl {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_Locals as ::windows::core::ComInterface>::IID || iid == &<IDebugPropertyEnumType_All as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"implement\"`*"]
pub trait IDebugPropertyEnumType_LocalsPlusArgs_Impl: Sized + IDebugPropertyEnumType_All_Impl {}
impl ::windows::core::RuntimeName for IDebugPropertyEnumType_LocalsPlusArgs {}
impl IDebugPropertyEnumType_LocalsPlusArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugPropertyEnumType_LocalsPlusArgs_Impl, const OFFSET: isize>() -> IDebugPropertyEnumType_LocalsPlusArgs_Vtbl {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_LocalsPlusArgs as ::windows::core::ComInterface>::IID || iid == &<IDebugPropertyEnumType_All as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"implement\"`*"]
pub trait IDebugPropertyEnumType_Registers_Impl: Sized + IDebugPropertyEnumType_All_Impl {}
impl ::windows::core::RuntimeName for IDebugPropertyEnumType_Registers {}
impl IDebugPropertyEnumType_Registers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebugPropertyEnumType_Registers_Impl, const OFFSET: isize>() -> IDebugPropertyEnumType_Registers_Vtbl {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_Registers as ::windows::core::ComInterface>::IID || iid == &<IDebugPropertyEnumType_All as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait IEnumDebugExtendedPropertyInfo_Impl: Sized {
    fn Next(&self, celt: u32, rgextendedpropertyinfo: *mut ExtendedDebugPropertyInfo, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumDebugExtendedPropertyInfo>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEnumDebugExtendedPropertyInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl IEnumDebugExtendedPropertyInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>() -> IEnumDebugExtendedPropertyInfo_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgextendedpropertyinfo: *mut ExtendedDebugPropertyInfo, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgextendedpropertyinfo), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pedpe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pedpe, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDebugExtendedPropertyInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"implement\"`*"]
pub trait IEnumDebugPropertyInfo_Impl: Sized {
    fn Next(&self, celt: u32, pi: *mut DebugPropertyInfo, pceltsfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumDebugPropertyInfo>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IEnumDebugPropertyInfo {}
impl IEnumDebugPropertyInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>() -> IEnumDebugPropertyInfo_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pi: *mut DebugPropertyInfo, pceltsfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pi), ::core::mem::transmute_copy(&pceltsfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppepi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppepi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDebugPropertyInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"implement\"`*"]
pub trait IObjectSafety_Impl: Sized {
    fn GetInterfaceSafetyOptions(&self, riid: *const ::windows::core::GUID, pdwsupportedoptions: *mut u32, pdwenabledoptions: *mut u32) -> ::windows::core::Result<()>;
    fn SetInterfaceSafetyOptions(&self, riid: *const ::windows::core::GUID, dwoptionsetmask: u32, dwenabledoptions: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IObjectSafety {}
impl IObjectSafety_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectSafety_Impl, const OFFSET: isize>() -> IObjectSafety_Vtbl {
        unsafe extern "system" fn GetInterfaceSafetyOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectSafety_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pdwsupportedoptions: *mut u32, pdwenabledoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInterfaceSafetyOptions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pdwsupportedoptions), ::core::mem::transmute_copy(&pdwenabledoptions)).into()
        }
        unsafe extern "system" fn SetInterfaceSafetyOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectSafety_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwoptionsetmask: u32, dwenabledoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInterfaceSafetyOptions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&dwoptionsetmask), ::core::mem::transmute_copy(&dwenabledoptions)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInterfaceSafetyOptions: GetInterfaceSafetyOptions::<Identity, Impl, OFFSET>,
            SetInterfaceSafetyOptions: SetInterfaceSafetyOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectSafety as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Ole")]
pub trait IPerPropertyBrowsing2_Impl: Sized {
    fn GetDisplayString(&self, dispid: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn MapPropertyToPage(&self, dispid: i32) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetPredefinedStrings(&self, dispid: i32, pcastrings: *mut super::super::Ole::CALPOLESTR, pcacookies: *mut super::super::Ole::CADWORD) -> ::windows::core::Result<()>;
    fn SetPredefinedValue(&self, dispid: i32, dwcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows::core::RuntimeName for IPerPropertyBrowsing2 {}
#[cfg(feature = "Win32_System_Ole")]
impl IPerPropertyBrowsing2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerPropertyBrowsing2_Impl, const OFFSET: isize>() -> IPerPropertyBrowsing2_Vtbl {
        unsafe extern "system" fn GetDisplayString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerPropertyBrowsing2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayString(::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapPropertyToPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerPropertyBrowsing2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pclsidproppage: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MapPropertyToPage(::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsidproppage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPredefinedStrings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerPropertyBrowsing2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, pcastrings: *mut super::super::Ole::CALPOLESTR, pcacookies: *mut super::super::Ole::CADWORD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPredefinedStrings(::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&pcastrings), ::core::mem::transmute_copy(&pcacookies)).into()
        }
        unsafe extern "system" fn SetPredefinedValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerPropertyBrowsing2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispid: i32, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPredefinedValue(::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayString: GetDisplayString::<Identity, Impl, OFFSET>,
            MapPropertyToPage: MapPropertyToPage::<Identity, Impl, OFFSET>,
            GetPredefinedStrings: GetPredefinedStrings::<Identity, Impl, OFFSET>,
            SetPredefinedValue: SetPredefinedValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerPropertyBrowsing2 as ::windows::core::ComInterface>::IID
    }
}
