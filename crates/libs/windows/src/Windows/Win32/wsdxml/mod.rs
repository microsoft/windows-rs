#[inline]
pub unsafe fn WSDXMLCreateContext() -> windows_core::Result<IWSDXMLContext> {
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLCreateContext(ppcontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDXMLCreateContext(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_wsdxmldom")]
#[inline]
pub unsafe fn WSDXMLGetNameFromBuiltinNamespace<P0, P1>(psznamespace: P0, pszname: P1) -> windows_core::Result<*mut super::wsdxmldom::WSDXML_NAME>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLGetNameFromBuiltinNamespace(psznamespace : windows_core::PCWSTR, pszname : windows_core::PCWSTR, ppname : *mut *mut super::wsdxmldom::WSDXML_NAME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDXMLGetNameFromBuiltinNamespace(psznamespace.param().abi(), pszname.param().abi(), &mut result__).map(|| result__)
    }
}
windows_core::imp::define_interface!(IWSDXMLContext, IWSDXMLContext_Vtbl, 0x75d8f3ee_3e5a_43b4_a15a_bcf6887460c0);
windows_core::imp::interface_hierarchy!(IWSDXMLContext, windows_core::IUnknown);
impl IWSDXMLContext {
    #[cfg(feature = "Win32_wsdxmldom")]
    pub unsafe fn AddNamespace<P0, P1>(&self, pszuri: P0, pszsuggestedprefix: P1) -> windows_core::Result<*mut super::wsdxmldom::WSDXML_NAMESPACE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddNamespace)(windows_core::Interface::as_raw(self), pszuri.param().abi(), pszsuggestedprefix.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wsdxmldom")]
    pub unsafe fn AddNameToNamespace<P0, P1>(&self, pszuri: P0, pszname: P1) -> windows_core::Result<*mut super::wsdxmldom::WSDXML_NAME>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddNameToNamespace)(windows_core::Interface::as_raw(self), pszuri.param().abi(), pszname.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wsdxmldom")]
    pub unsafe fn SetNamespaces(&self, pnamespaces: &[PCWSDXML_NAMESPACE], blayernumber: u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNamespaces)(windows_core::Interface::as_raw(self), core::mem::transmute(pnamespaces.as_ptr()), pnamespaces.len().try_into().unwrap(), blayernumber) }
    }
    #[cfg(feature = "Win32_wsdxmldom")]
    pub unsafe fn SetTypes(&self, ptypes: &[PCWSDXML_TYPE], blayernumber: u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTypes)(windows_core::Interface::as_raw(self), core::mem::transmute(ptypes.as_ptr()), ptypes.len().try_into().unwrap(), blayernumber) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDXMLContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_wsdxmldom")]
    pub AddNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut super::wsdxmldom::WSDXML_NAMESPACE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdxmldom"))]
    AddNamespace: usize,
    #[cfg(feature = "Win32_wsdxmldom")]
    pub AddNameToNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut super::wsdxmldom::WSDXML_NAME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdxmldom"))]
    AddNameToNamespace: usize,
    #[cfg(feature = "Win32_wsdxmldom")]
    pub SetNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void, *const PCWSDXML_NAMESPACE, u16, u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdxmldom"))]
    SetNamespaces: usize,
    #[cfg(feature = "Win32_wsdxmldom")]
    pub SetTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *const PCWSDXML_TYPE, u32, u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdxmldom"))]
    SetTypes: usize,
}
#[cfg(feature = "Win32_wsdxmldom")]
pub trait IWSDXMLContext_Impl: windows_core::IUnknownImpl {
    fn AddNamespace(&self, pszuri: &windows_core::PCWSTR, pszsuggestedprefix: &windows_core::PCWSTR) -> windows_core::Result<*mut super::wsdxmldom::WSDXML_NAMESPACE>;
    fn AddNameToNamespace(&self, pszuri: &windows_core::PCWSTR, pszname: &windows_core::PCWSTR) -> windows_core::Result<*mut super::wsdxmldom::WSDXML_NAME>;
    fn SetNamespaces(&self, pnamespaces: *const PCWSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> windows_core::Result<()>;
    fn SetTypes(&self, ptypes: *const PCWSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_wsdxmldom")]
impl IWSDXMLContext_Vtbl {
    pub const fn new<Identity: IWSDXMLContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddNamespace<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuri: windows_core::PCWSTR, pszsuggestedprefix: windows_core::PCWSTR, ppnamespace: *mut *mut super::wsdxmldom::WSDXML_NAMESPACE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDXMLContext_Impl::AddNamespace(this, core::mem::transmute(&pszuri), core::mem::transmute(&pszsuggestedprefix)) {
                    Ok(ok__) => {
                        ppnamespace.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddNameToNamespace<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuri: windows_core::PCWSTR, pszname: windows_core::PCWSTR, ppname: *mut *mut super::wsdxmldom::WSDXML_NAME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDXMLContext_Impl::AddNameToNamespace(this, core::mem::transmute(&pszuri), core::mem::transmute(&pszname)) {
                    Ok(ok__) => {
                        ppname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamespaces<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespaces: *const PCWSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDXMLContext_Impl::SetNamespaces(this, core::mem::transmute_copy(&pnamespaces), core::mem::transmute_copy(&wnamespacescount), core::mem::transmute_copy(&blayernumber)).into()
            }
        }
        unsafe extern "system" fn SetTypes<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypes: *const PCWSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDXMLContext_Impl::SetTypes(this, core::mem::transmute_copy(&ptypes), core::mem::transmute_copy(&dwtypescount), core::mem::transmute_copy(&blayernumber)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddNamespace: AddNamespace::<Identity, OFFSET>,
            AddNameToNamespace: AddNameToNamespace::<Identity, OFFSET>,
            SetNamespaces: SetNamespaces::<Identity, OFFSET>,
            SetTypes: SetTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDXMLContext as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_wsdxmldom")]
impl windows_core::RuntimeName for IWSDXMLContext {}
pub const OpAnyElement: i32 = 6;
pub const OpAnyElements: i32 = 7;
pub const OpAnyNumber: i32 = 17;
pub const OpAnyText: i32 = 8;
pub const OpAnything: i32 = 16;
pub const OpAttribute_: i32 = 9;
pub const OpBeginAll: i32 = 14;
pub const OpBeginAnyElement: i32 = 3;
pub const OpBeginChoice: i32 = 10;
pub const OpBeginElement_: i32 = 2;
pub const OpBeginSequence: i32 = 12;
pub const OpElement_: i32 = 5;
pub const OpEndAll: i32 = 15;
pub const OpEndChoice: i32 = 11;
pub const OpEndElement: i32 = 4;
pub const OpEndOfTable: i32 = 1;
pub const OpEndSequence: i32 = 13;
pub const OpFormatBool_: i32 = 20;
pub const OpFormatDateTime_: i32 = 40;
pub const OpFormatDom_: i32 = 30;
pub const OpFormatDouble_: i32 = 42;
pub const OpFormatDuration_: i32 = 39;
pub const OpFormatDynamicType_: i32 = 37;
pub const OpFormatFloat_: i32 = 41;
pub const OpFormatInt16_: i32 = 22;
pub const OpFormatInt32_: i32 = 23;
pub const OpFormatInt64_: i32 = 24;
pub const OpFormatInt8_: i32 = 21;
pub const OpFormatListInsertTail_: i32 = 35;
pub const OpFormatLookupType_: i32 = 38;
pub const OpFormatMax: i32 = 46;
pub const OpFormatName_: i32 = 34;
pub const OpFormatStruct_: i32 = 31;
pub const OpFormatType_: i32 = 36;
pub const OpFormatUInt16_: i32 = 26;
pub const OpFormatUInt32_: i32 = 27;
pub const OpFormatUInt64_: i32 = 28;
pub const OpFormatUInt8_: i32 = 25;
pub const OpFormatUnicodeString_: i32 = 29;
pub const OpFormatUri_: i32 = 32;
pub const OpFormatUuidUri_: i32 = 33;
pub const OpFormatXMLDeclaration_: i32 = 45;
pub const OpNone: i32 = 0;
pub const OpOneOrMore: i32 = 18;
pub const OpOptional: i32 = 19;
pub const OpProcess_: i32 = 43;
pub const OpQualifiedAttribute_: i32 = 44;
#[cfg(feature = "Win32_wsdxmldom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCWSDXML_NAMESPACE(pub *const super::wsdxmldom::WSDXML_NAMESPACE);
#[cfg(feature = "Win32_wsdxmldom")]
impl PCWSDXML_NAMESPACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for PCWSDXML_NAMESPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wsdxmldom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCWSDXML_TYPE(pub *const super::wsdxmldom::WSDXML_TYPE);
#[cfg(feature = "Win32_wsdxmldom")]
impl PCWSDXML_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for PCWSDXML_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSD_DATETIME {
    pub isPositive: windows_core::BOOL,
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u32,
    pub TZIsLocal: windows_core::BOOL,
    pub TZIsPositive: windows_core::BOOL,
    pub TZHour: u8,
    pub TZMinute: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSD_DURATION {
    pub isPositive: windows_core::BOOL,
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}
