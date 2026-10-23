pub const DEVICEDISPLAY_DISCOVERYMETHOD_AD_PRINTER: windows_core::PCWSTR = windows_core::w!("Published Printer");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_ASP_INFRA: windows_core::PCWSTR = windows_core::w!("AspInfra");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_BLUETOOTH: windows_core::PCWSTR = windows_core::w!("Bluetooth");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_BLUETOOTH_LE: windows_core::PCWSTR = windows_core::w!("Bluetooth Low Energy");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_NETBIOS: windows_core::PCWSTR = windows_core::w!("NetBIOS");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_PNP: windows_core::PCWSTR = windows_core::w!("PnP");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_UPNP: windows_core::PCWSTR = windows_core::w!("UPnP");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WFD: windows_core::PCWSTR = windows_core::w!("WiFiDirect");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WSD: windows_core::PCWSTR = windows_core::w!("WSD");
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WUSB: windows_core::PCWSTR = windows_core::w!("WUSB");
pub const E_FDPAIRING_AUTHFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8FD00003_u32 as _);
pub const E_FDPAIRING_AUTHNOTALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8FD00006_u32 as _);
pub const E_FDPAIRING_CONNECTTIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8FD00004_u32 as _);
pub const E_FDPAIRING_HWFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8FD00002_u32 as _);
pub const E_FDPAIRING_IPBUSDISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8FD00007_u32 as _);
pub const E_FDPAIRING_NOCONNECTION: windows_core::HRESULT = windows_core::HRESULT(0x8FD00001_u32 as _);
pub const E_FDPAIRING_NOPROFILES: windows_core::HRESULT = windows_core::HRESULT(0x8FD00008_u32 as _);
pub const E_FDPAIRING_TOOMANYCONNECTIONS: windows_core::HRESULT = windows_core::HRESULT(0x8FD00005_u32 as _);
pub const FCTN_CATEGORY_BT: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Devices.Bluetooth");
pub const FCTN_CATEGORY_DEVICEDISPLAYOBJECTS: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Base.DeviceDisplayObjects");
pub const FCTN_CATEGORY_DEVICEFUNCTIONENUMERATORS: windows_core::PCWSTR = windows_core::w!("Layered\\Microsoft.Devices.FunctionEnumerators");
pub const FCTN_CATEGORY_DEVICEPAIRING: windows_core::PCWSTR = windows_core::w!("Layered\\Microsoft.Base.DevicePairing");
pub const FCTN_CATEGORY_DEVICES: windows_core::PCWSTR = windows_core::w!("Layered\\Microsoft.Base.Devices");
pub const FCTN_CATEGORY_DEVQUERYOBJECTS: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Base.DevQueryObjects");
pub const FCTN_CATEGORY_NETBIOS: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Networking.Netbios");
pub const FCTN_CATEGORY_NETWORKDEVICES: windows_core::PCWSTR = windows_core::w!("Layered\\Microsoft.Networking.Devices");
pub const FCTN_CATEGORY_PNP: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Base.PnP");
pub const FCTN_CATEGORY_PNPXASSOCIATION: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.PnPX.Association");
pub const FCTN_CATEGORY_PUBLICATION: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Base.Publication");
pub const FCTN_CATEGORY_REGISTRY: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Base.Registry");
pub const FCTN_CATEGORY_SSDP: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Networking.SSDP");
pub const FCTN_CATEGORY_WCN: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Networking.WCN");
pub const FCTN_CATEGORY_WSDISCOVERY: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Networking.WSD");
pub const FCTN_CATEGORY_WUSB: windows_core::PCWSTR = windows_core::w!("Provider\\Microsoft.Devices.WirelessUSB");
pub const FCTN_SUBCAT_DEVICES_WSDPRINTERS: windows_core::PCWSTR = windows_core::w!("WSDPrinters");
pub const FCTN_SUBCAT_NETWORKDEVICES_SSDP: windows_core::PCWSTR = windows_core::w!("SSDP");
pub const FCTN_SUBCAT_NETWORKDEVICES_WSD: windows_core::PCWSTR = windows_core::w!("WSD");
pub const FCTN_SUBCAT_REG_DIRECTED: windows_core::PCWSTR = windows_core::w!("Directed");
pub const FCTN_SUBCAT_REG_PUBLICATION: windows_core::PCWSTR = windows_core::w!("Publication");
pub type FDQUERYCONTEXT = u64;
pub const FD_CONSTRAINTVALUE_ALL: windows_core::PCWSTR = windows_core::w!("All");
pub const FD_CONSTRAINTVALUE_COMCLSCONTEXT_INPROC_SERVER: windows_core::PCWSTR = windows_core::w!("1");
pub const FD_CONSTRAINTVALUE_COMCLSCONTEXT_LOCAL_SERVER: windows_core::PCWSTR = windows_core::w!("4");
pub const FD_CONSTRAINTVALUE_FALSE: windows_core::PCWSTR = windows_core::w!("FALSE");
pub const FD_CONSTRAINTVALUE_PAIRED: windows_core::PCWSTR = windows_core::w!("Paired");
pub const FD_CONSTRAINTVALUE_RECURSESUBCATEGORY_TRUE: windows_core::PCWSTR = windows_core::w!("TRUE");
pub const FD_CONSTRAINTVALUE_ROUTINGSCOPE_ALL: windows_core::PCWSTR = windows_core::w!("All");
pub const FD_CONSTRAINTVALUE_ROUTINGSCOPE_DIRECT: windows_core::PCWSTR = windows_core::w!("Direct");
pub const FD_CONSTRAINTVALUE_TRUE: windows_core::PCWSTR = windows_core::w!("TRUE");
pub const FD_CONSTRAINTVALUE_UNPAIRED: windows_core::PCWSTR = windows_core::w!("UnPaired");
pub const FD_CONSTRAINTVALUE_VISIBILITY_ALL: windows_core::PCWSTR = windows_core::w!("1");
pub const FD_CONSTRAINTVALUE_VISIBILITY_DEFAULT: windows_core::PCWSTR = windows_core::w!("0");
pub const FD_EVENTID: i32 = 1000;
pub const FD_EVENTID_ASYNCTHREADEXIT: i32 = 1001;
pub const FD_EVENTID_IPADDRESSCHANGE: i32 = 1003;
pub const FD_EVENTID_PRIVATE: i32 = 100;
pub const FD_EVENTID_QUERYREFRESH: i32 = 1004;
pub const FD_EVENTID_SEARCHCOMPLETE: i32 = 1000;
pub const FD_EVENTID_SEARCHSTART: i32 = 1002;
pub const FD_LONGHORN: i32 = 1;
pub const FD_QUERYCONSTRAINT_COMCLSCONTEXT: windows_core::PCWSTR = windows_core::w!("COMClsContext");
pub const FD_QUERYCONSTRAINT_INQUIRY_TIMEOUT: windows_core::PCWSTR = windows_core::w!("InquiryModeTimeout");
pub const FD_QUERYCONSTRAINT_PAIRING_STATE: windows_core::PCWSTR = windows_core::w!("PairingState");
pub const FD_QUERYCONSTRAINT_PROVIDERINSTANCEID: windows_core::PCWSTR = windows_core::w!("ProviderInstanceID");
pub const FD_QUERYCONSTRAINT_RECURSESUBCATEGORY: windows_core::PCWSTR = windows_core::w!("RecurseSubcategory");
pub const FD_QUERYCONSTRAINT_ROUTINGSCOPE: windows_core::PCWSTR = windows_core::w!("RoutingScope");
pub const FD_QUERYCONSTRAINT_SUBCATEGORY: windows_core::PCWSTR = windows_core::w!("Subcategory");
pub const FD_QUERYCONSTRAINT_VISIBILITY: windows_core::PCWSTR = windows_core::w!("Visibility");
pub const FD_SUBKEY: windows_core::PCWSTR = windows_core::w!("SOFTWARE\\Microsoft\\Function Discovery\\");
pub const FD_SUBKEY_CATEGORIES: windows_core::PCWSTR = windows_core::w!("SOFTWARE\\Microsoft\\Function Discovery\\Categories\\");
pub const FD_Visibility_Default: i32 = 0;
pub const FD_Visibility_Hidden: i32 = 1;
pub const FMTID_Device: windows_core::GUID = windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57);
pub const FMTID_DeviceInterface: windows_core::GUID = windows_core::GUID::from_u128(0x53808008_07bb_4661_bc3c_b5953e708560);
pub const FMTID_FD: windows_core::GUID = windows_core::GUID::from_u128(0x904b03a2_471d_423c_a584_f3483238a146);
pub const FMTID_PNPX: windows_core::GUID = windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd);
pub const FMTID_PNPXDynamicProperty: windows_core::GUID = windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd);
pub const FMTID_Pairing: windows_core::GUID = windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc);
pub const FMTID_WSD: windows_core::GUID = windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92);
windows_core::imp::define_interface!(IFunctionDiscovery, IFunctionDiscovery_Vtbl, 0x4df99b70_e148_4432_b004_4c9eeb535a5e);
windows_core::imp::interface_hierarchy!(IFunctionDiscovery, windows_core::IUnknown);
impl IFunctionDiscovery {
    #[cfg(feature = "winnt")]
    pub unsafe fn GetInstanceCollection(&self, pszcategory: *const super::WCHAR, pszsubcategory: Option<*const super::WCHAR>, fincludeallsubcategories: bool) -> windows_core::Result<IFunctionInstanceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstanceCollection)(windows_core::Interface::as_raw(self), pszcategory, pszsubcategory.unwrap_or(core::mem::zeroed()) as _, fincludeallsubcategories.into(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "servprov", feature = "winnt"))]
    pub unsafe fn GetInstance(&self, pszfunctioninstanceidentity: *const super::WCHAR) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstance)(windows_core::Interface::as_raw(self), pszfunctioninstanceidentity, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn CreateInstanceCollectionQuery<P3>(&self, pszcategory: *const super::WCHAR, pszsubcategory: Option<*const super::WCHAR>, fincludeallsubcategories: bool, pifunctiondiscoverynotification: P3, pfdqcquerycontext: Option<*mut FDQUERYCONTEXT>) -> windows_core::Result<IFunctionInstanceCollectionQuery>
    where
        P3: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceCollectionQuery)(windows_core::Interface::as_raw(self), pszcategory, pszsubcategory.unwrap_or(core::mem::zeroed()) as _, fincludeallsubcategories.into(), pifunctiondiscoverynotification.param().abi(), pfdqcquerycontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn CreateInstanceQuery<P1>(&self, pszfunctioninstanceidentity: *const super::WCHAR, pifunctiondiscoverynotification: P1, pfdqcquerycontext: Option<*mut FDQUERYCONTEXT>) -> windows_core::Result<IFunctionInstanceQuery>
    where
        P1: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceQuery)(windows_core::Interface::as_raw(self), pszfunctioninstanceidentity, pifunctiondiscoverynotification.param().abi(), pfdqcquerycontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "servprov", feature = "winnt"))]
    pub unsafe fn AddInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const super::WCHAR, pszsubcategory: Option<*const super::WCHAR>, pszcategoryidentity: *const super::WCHAR) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddInstance)(windows_core::Interface::as_raw(self), enumsystemvisibility, pszcategory, pszsubcategory.unwrap_or(core::mem::zeroed()) as _, pszcategoryidentity, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn RemoveInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const super::WCHAR, pszsubcategory: Option<*const super::WCHAR>, pszcategoryidentity: *const super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveInstance)(windows_core::Interface::as_raw(self), enumsystemvisibility, pszcategory, pszsubcategory.unwrap_or(core::mem::zeroed()) as _, pszcategoryidentity) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscovery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetInstanceCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, *const super::WCHAR, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetInstanceCollection: usize,
    #[cfg(all(feature = "servprov", feature = "winnt"))]
    pub GetInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "servprov", feature = "winnt")))]
    GetInstance: usize,
    #[cfg(feature = "winnt")]
    pub CreateInstanceCollectionQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, *const super::WCHAR, windows_core::BOOL, *mut core::ffi::c_void, *mut FDQUERYCONTEXT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    CreateInstanceCollectionQuery: usize,
    #[cfg(feature = "winnt")]
    pub CreateInstanceQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, *mut core::ffi::c_void, *mut FDQUERYCONTEXT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    CreateInstanceQuery: usize,
    #[cfg(all(feature = "servprov", feature = "winnt"))]
    pub AddInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, *const super::WCHAR, *const super::WCHAR, *const super::WCHAR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "servprov", feature = "winnt")))]
    AddInstance: usize,
    #[cfg(feature = "winnt")]
    pub RemoveInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, *const super::WCHAR, *const super::WCHAR, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    RemoveInstance: usize,
}
#[cfg(all(feature = "servprov", feature = "winnt"))]
pub trait IFunctionDiscovery_Impl: windows_core::IUnknownImpl {
    fn GetInstanceCollection(&self, pszcategory: *const super::WCHAR, pszsubcategory: *const super::WCHAR, fincludeallsubcategories: windows_core::BOOL) -> windows_core::Result<IFunctionInstanceCollection>;
    fn GetInstance(&self, pszfunctioninstanceidentity: *const super::WCHAR) -> windows_core::Result<IFunctionInstance>;
    fn CreateInstanceCollectionQuery(&self, pszcategory: *const super::WCHAR, pszsubcategory: *const super::WCHAR, fincludeallsubcategories: windows_core::BOOL, pifunctiondiscoverynotification: windows_core::Ref<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceCollectionQuery>;
    fn CreateInstanceQuery(&self, pszfunctioninstanceidentity: *const super::WCHAR, pifunctiondiscoverynotification: windows_core::Ref<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceQuery>;
    fn AddInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const super::WCHAR, pszsubcategory: *const super::WCHAR, pszcategoryidentity: *const super::WCHAR) -> windows_core::Result<IFunctionInstance>;
    fn RemoveInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const super::WCHAR, pszsubcategory: *const super::WCHAR, pszcategoryidentity: *const super::WCHAR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "servprov", feature = "winnt"))]
impl IFunctionDiscovery_Vtbl {
    pub const fn new<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInstanceCollection<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcategory: *const super::WCHAR, pszsubcategory: *const super::WCHAR, fincludeallsubcategories: windows_core::BOOL, ppifunctioninstancecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionDiscovery_Impl::GetInstanceCollection(this, core::mem::transmute_copy(&pszcategory), core::mem::transmute_copy(&pszsubcategory), core::mem::transmute_copy(&fincludeallsubcategories)) {
                    Ok(ok__) => {
                        ppifunctioninstancecollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInstance<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfunctioninstanceidentity: *const super::WCHAR, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionDiscovery_Impl::GetInstance(this, core::mem::transmute_copy(&pszfunctioninstanceidentity)) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInstanceCollectionQuery<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcategory: *const super::WCHAR, pszsubcategory: *const super::WCHAR, fincludeallsubcategories: windows_core::BOOL, pifunctiondiscoverynotification: *mut core::ffi::c_void, pfdqcquerycontext: *mut FDQUERYCONTEXT, ppifunctioninstancecollectionquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionDiscovery_Impl::CreateInstanceCollectionQuery(this, core::mem::transmute_copy(&pszcategory), core::mem::transmute_copy(&pszsubcategory), core::mem::transmute_copy(&fincludeallsubcategories), core::mem::transmute_copy(&pifunctiondiscoverynotification), core::mem::transmute_copy(&pfdqcquerycontext)) {
                    Ok(ok__) => {
                        ppifunctioninstancecollectionquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInstanceQuery<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfunctioninstanceidentity: *const super::WCHAR, pifunctiondiscoverynotification: *mut core::ffi::c_void, pfdqcquerycontext: *mut FDQUERYCONTEXT, ppifunctioninstancequery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionDiscovery_Impl::CreateInstanceQuery(this, core::mem::transmute_copy(&pszfunctioninstanceidentity), core::mem::transmute_copy(&pifunctiondiscoverynotification), core::mem::transmute_copy(&pfdqcquerycontext)) {
                    Ok(ok__) => {
                        ppifunctioninstancequery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddInstance<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const super::WCHAR, pszsubcategory: *const super::WCHAR, pszcategoryidentity: *const super::WCHAR, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionDiscovery_Impl::AddInstance(this, core::mem::transmute_copy(&enumsystemvisibility), core::mem::transmute_copy(&pszcategory), core::mem::transmute_copy(&pszsubcategory), core::mem::transmute_copy(&pszcategoryidentity)) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveInstance<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const super::WCHAR, pszsubcategory: *const super::WCHAR, pszcategoryidentity: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionDiscovery_Impl::RemoveInstance(this, core::mem::transmute_copy(&enumsystemvisibility), core::mem::transmute_copy(&pszcategory), core::mem::transmute_copy(&pszsubcategory), core::mem::transmute_copy(&pszcategoryidentity)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInstanceCollection: GetInstanceCollection::<Identity, OFFSET>,
            GetInstance: GetInstance::<Identity, OFFSET>,
            CreateInstanceCollectionQuery: CreateInstanceCollectionQuery::<Identity, OFFSET>,
            CreateInstanceQuery: CreateInstanceQuery::<Identity, OFFSET>,
            AddInstance: AddInstance::<Identity, OFFSET>,
            RemoveInstance: RemoveInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionDiscovery as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "servprov", feature = "winnt"))]
impl windows_core::RuntimeName for IFunctionDiscovery {}
windows_core::imp::define_interface!(IFunctionDiscoveryNotification, IFunctionDiscoveryNotification_Vtbl, 0x5f6c1ba8_5330_422e_a368_572b244d3f87);
windows_core::imp::interface_hierarchy!(IFunctionDiscoveryNotification, windows_core::IUnknown);
impl IFunctionDiscoveryNotification {
    #[cfg(feature = "servprov")]
    pub unsafe fn OnUpdate<P2>(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: FDQUERYCONTEXT, pifunctioninstance: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IFunctionInstance>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnUpdate)(windows_core::Interface::as_raw(self), enumqueryupdateaction, fdqcquerycontext, pifunctioninstance.param().abi()) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn OnError(&self, hr: windows_core::HRESULT, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hr, fdqcquerycontext, pszprovider) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn OnEvent(&self, dweventid: super::DWORD, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), dweventid, fdqcquerycontext, pszprovider) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "servprov")]
    pub OnUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, QueryUpdateAction, FDQUERYCONTEXT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "servprov"))]
    OnUpdate: usize,
    #[cfg(feature = "winnt")]
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, FDQUERYCONTEXT, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    OnError: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, FDQUERYCONTEXT, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    OnEvent: usize,
}
#[cfg(all(feature = "servprov", feature = "shared", feature = "winnt"))]
pub trait IFunctionDiscoveryNotification_Impl: windows_core::IUnknownImpl {
    fn OnUpdate(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: FDQUERYCONTEXT, pifunctioninstance: windows_core::Ref<IFunctionInstance>) -> windows_core::Result<()>;
    fn OnError(&self, hr: windows_core::HRESULT, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const super::WCHAR) -> windows_core::Result<()>;
    fn OnEvent(&self, dweventid: super::DWORD, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const super::WCHAR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "servprov", feature = "shared", feature = "winnt"))]
impl IFunctionDiscoveryNotification_Vtbl {
    pub const fn new<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdate<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: FDQUERYCONTEXT, pifunctioninstance: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionDiscoveryNotification_Impl::OnUpdate(this, core::mem::transmute_copy(&enumqueryupdateaction), core::mem::transmute_copy(&fdqcquerycontext), core::mem::transmute_copy(&pifunctioninstance)).into()
            }
        }
        unsafe extern "system" fn OnError<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionDiscoveryNotification_Impl::OnError(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&fdqcquerycontext), core::mem::transmute_copy(&pszprovider)).into()
            }
        }
        unsafe extern "system" fn OnEvent<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweventid: super::DWORD, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionDiscoveryNotification_Impl::OnEvent(this, core::mem::transmute_copy(&dweventid), core::mem::transmute_copy(&fdqcquerycontext), core::mem::transmute_copy(&pszprovider)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdate: OnUpdate::<Identity, OFFSET>,
            OnError: OnError::<Identity, OFFSET>,
            OnEvent: OnEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryNotification as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "servprov", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IFunctionDiscoveryNotification {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IFunctionDiscoveryProviderRefresh(pub u8);
#[cfg(feature = "servprov")]
windows_core::imp::define_interface!(IFunctionInstance, IFunctionInstance_Vtbl, 0x33591c10_0bed_4f02_b0ab_1530d5533ee9);
#[cfg(feature = "servprov")]
impl core::ops::Deref for IFunctionInstance {
    type Target = super::IServiceProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "servprov")]
windows_core::imp::interface_hierarchy!(IFunctionInstance, windows_core::IUnknown, super::IServiceProvider);
#[cfg(feature = "servprov")]
impl IFunctionInstance {
    #[cfg(feature = "winnt")]
    pub unsafe fn GetID(&self, ppszcomemidentity: *mut *mut super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), ppszcomemidentity as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetProviderInstanceID(&self, ppszcomemproviderinstanceidentity: *mut *mut super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProviderInstanceID)(windows_core::Interface::as_raw(self), ppszcomemproviderinstanceidentity as _) }
    }
    #[cfg(all(feature = "propsys", feature = "shared"))]
    pub unsafe fn OpenPropertyStore(&self, dwstgaccess: super::DWORD) -> windows_core::Result<super::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenPropertyStore)(windows_core::Interface::as_raw(self), dwstgaccess, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetCategory(&self, ppszcomemcategory: *mut *mut super::WCHAR, ppszcomemsubcategory: *mut *mut super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), ppszcomemcategory as _, ppszcomemsubcategory as _) }
    }
}
#[cfg(feature = "servprov")]
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstance_Vtbl {
    pub base__: super::IServiceProvider_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetID: usize,
    #[cfg(feature = "winnt")]
    pub GetProviderInstanceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetProviderInstanceID: usize,
    #[cfg(all(feature = "propsys", feature = "shared"))]
    pub OpenPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "propsys", feature = "shared")))]
    OpenPropertyStore: usize,
    #[cfg(feature = "winnt")]
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::WCHAR, *mut *mut super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetCategory: usize,
}
#[cfg(all(feature = "guiddef", feature = "propsys", feature = "servprov", feature = "shared", feature = "winnt"))]
pub trait IFunctionInstance_Impl: super::IServiceProvider_Impl {
    fn GetID(&self, ppszcomemidentity: *mut *mut super::WCHAR) -> windows_core::Result<()>;
    fn GetProviderInstanceID(&self, ppszcomemproviderinstanceidentity: *mut *mut super::WCHAR) -> windows_core::Result<()>;
    fn OpenPropertyStore(&self, dwstgaccess: super::DWORD) -> windows_core::Result<super::IPropertyStore>;
    fn GetCategory(&self, ppszcomemcategory: *mut *mut super::WCHAR, ppszcomemsubcategory: *mut *mut super::WCHAR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "propsys", feature = "servprov", feature = "shared", feature = "winnt"))]
impl IFunctionInstance_Vtbl {
    pub const fn new<Identity: IFunctionInstance_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetID<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemidentity: *mut *mut super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstance_Impl::GetID(this, core::mem::transmute_copy(&ppszcomemidentity)).into()
            }
        }
        unsafe extern "system" fn GetProviderInstanceID<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstance_Impl::GetProviderInstanceID(this, core::mem::transmute_copy(&ppszcomemproviderinstanceidentity)).into()
            }
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstgaccess: super::DWORD, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstance_Impl::OpenPropertyStore(this, core::mem::transmute_copy(&dwstgaccess)) {
                    Ok(ok__) => {
                        ppipropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCategory<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemcategory: *mut *mut super::WCHAR, ppszcomemsubcategory: *mut *mut super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstance_Impl::GetCategory(this, core::mem::transmute_copy(&ppszcomemcategory), core::mem::transmute_copy(&ppszcomemsubcategory)).into()
            }
        }
        Self {
            base__: super::IServiceProvider_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, OFFSET>,
            GetProviderInstanceID: GetProviderInstanceID::<Identity, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, OFFSET>,
            GetCategory: GetCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstance as windows_core::Interface>::IID || iid == &<super::IServiceProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "propsys", feature = "servprov", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IFunctionInstance {}
windows_core::imp::define_interface!(IFunctionInstanceCollection, IFunctionInstanceCollection_Vtbl, 0xf0a3d895_855c_42a2_948d_2f97d450ecb1);
windows_core::imp::interface_hierarchy!(IFunctionInstanceCollection, windows_core::IUnknown);
impl IFunctionInstanceCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "servprov", feature = "shared", feature = "winnt"))]
    pub unsafe fn Get(&self, pszinstanceidentity: *const super::WCHAR, pdwindex: *mut super::DWORD) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), pszinstanceidentity, pdwindex as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "servprov", feature = "shared"))]
    pub unsafe fn Item(&self, dwindex: super::DWORD) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "servprov")]
    pub unsafe fn Add<P0>(&self, pifunctioninstance: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi()) }
    }
    #[cfg(all(feature = "servprov", feature = "shared"))]
    pub unsafe fn Remove(&self, dwindex: super::DWORD) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Delete(&self, dwindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), dwindex) }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
    #[cfg(all(feature = "servprov", feature = "shared", feature = "winnt"))]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, *mut super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "servprov", feature = "shared", feature = "winnt")))]
    Get: usize,
    #[cfg(all(feature = "servprov", feature = "shared"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "servprov", feature = "shared")))]
    Item: usize,
    #[cfg(feature = "servprov")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "servprov"))]
    Add: usize,
    #[cfg(all(feature = "servprov", feature = "shared"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "servprov", feature = "shared")))]
    Remove: usize,
    #[cfg(feature = "shared")]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Delete: usize,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "servprov", feature = "shared", feature = "winnt"))]
pub trait IFunctionInstanceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<super::DWORD>;
    fn Get(&self, pszinstanceidentity: *const super::WCHAR, pdwindex: *mut super::DWORD) -> windows_core::Result<IFunctionInstance>;
    fn Item(&self, dwindex: super::DWORD) -> windows_core::Result<IFunctionInstance>;
    fn Add(&self, pifunctioninstance: windows_core::Ref<IFunctionInstance>) -> windows_core::Result<()>;
    fn Remove(&self, dwindex: super::DWORD) -> windows_core::Result<IFunctionInstance>;
    fn Delete(&self, dwindex: super::DWORD) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "servprov", feature = "shared", feature = "winnt"))]
impl IFunctionInstanceCollection_Vtbl {
    pub const fn new<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Get<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinstanceidentity: *const super::WCHAR, pdwindex: *mut super::DWORD, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceCollection_Impl::Get(this, core::mem::transmute_copy(&pszinstanceidentity), core::mem::transmute_copy(&pdwindex)) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceCollection_Impl::Item(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstanceCollection_Impl::Add(this, core::mem::transmute_copy(&pifunctioninstance)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceCollection_Impl::Remove(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstanceCollection_Impl::Delete(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstanceCollection_Impl::DeleteAll(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "servprov", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IFunctionInstanceCollection {}
windows_core::imp::define_interface!(IFunctionInstanceCollectionQuery, IFunctionInstanceCollectionQuery_Vtbl, 0x57cc6fd2_c09a_4289_bb72_25f04142058e);
windows_core::imp::interface_hierarchy!(IFunctionInstanceCollectionQuery, windows_core::IUnknown);
impl IFunctionInstanceCollectionQuery {
    #[cfg(feature = "winnt")]
    pub unsafe fn AddQueryConstraint(&self, pszconstraintname: *const super::WCHAR, pszconstraintvalue: *const super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddQueryConstraint)(windows_core::Interface::as_raw(self), pszconstraintname, pszconstraintvalue) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddPropertyConstraint(&self, key: *const super::PROPERTYKEY, pv: *const super::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyConstraint)(windows_core::Interface::as_raw(self), key, pv, enumpropertyconstraint) }
    }
    pub unsafe fn Execute(&self) -> windows_core::Result<IFunctionInstanceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollectionQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub AddQueryConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    AddQueryConstraint: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub AddPropertyConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *const super::PROPVARIANT, PropertyConstraint) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    AddPropertyConstraint: usize,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFunctionInstanceCollectionQuery_Impl: windows_core::IUnknownImpl {
    fn AddQueryConstraint(&self, pszconstraintname: *const super::WCHAR, pszconstraintvalue: *const super::WCHAR) -> windows_core::Result<()>;
    fn AddPropertyConstraint(&self, key: *const super::PROPERTYKEY, pv: *const super::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::Result<()>;
    fn Execute(&self) -> windows_core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFunctionInstanceCollectionQuery_Vtbl {
    pub const fn new<Identity: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddQueryConstraint<Identity: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconstraintname: *const super::WCHAR, pszconstraintvalue: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstanceCollectionQuery_Impl::AddQueryConstraint(this, core::mem::transmute_copy(&pszconstraintname), core::mem::transmute_copy(&pszconstraintvalue)).into()
            }
        }
        unsafe extern "system" fn AddPropertyConstraint<Identity: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pv: *const super::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstanceCollectionQuery_Impl::AddPropertyConstraint(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&enumpropertyconstraint)).into()
            }
        }
        unsafe extern "system" fn Execute<Identity: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppifunctioninstancecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceCollectionQuery_Impl::Execute(this) {
                    Ok(ok__) => {
                        ppifunctioninstancecollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddQueryConstraint: AddQueryConstraint::<Identity, OFFSET>,
            AddPropertyConstraint: AddPropertyConstraint::<Identity, OFFSET>,
            Execute: Execute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceCollectionQuery as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFunctionInstanceCollectionQuery {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IFunctionInstanceCollectionQuery2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IFunctionInstanceCollectionQueryCollection(pub u8);
windows_core::imp::define_interface!(IFunctionInstanceQuery, IFunctionInstanceQuery_Vtbl, 0x6242bc6b_90ec_4b37_bb46_e229fd84ed95);
windows_core::imp::interface_hierarchy!(IFunctionInstanceQuery, windows_core::IUnknown);
impl IFunctionInstanceQuery {
    #[cfg(feature = "servprov")]
    pub unsafe fn Execute(&self) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "servprov")]
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "servprov"))]
    Execute: usize,
}
#[cfg(feature = "servprov")]
pub trait IFunctionInstanceQuery_Impl: windows_core::IUnknownImpl {
    fn Execute(&self) -> windows_core::Result<IFunctionInstance>;
}
#[cfg(feature = "servprov")]
impl IFunctionInstanceQuery_Vtbl {
    pub const fn new<Identity: IFunctionInstanceQuery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Execute<Identity: IFunctionInstanceQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceQuery_Impl::Execute(this) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Execute: Execute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceQuery as windows_core::Interface>::IID
    }
}
#[cfg(feature = "servprov")]
impl windows_core::RuntimeName for IFunctionInstanceQuery {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IFunctionInstanceQuery2(pub u8);
windows_core::imp::define_interface!(IPropertyStoreCollection, IPropertyStoreCollection_Vtbl, 0xd14d9c30_12d2_42d8_bce4_c60c2bb226fa);
windows_core::imp::interface_hierarchy!(IPropertyStoreCollection, windows_core::IUnknown);
impl IPropertyStoreCollection {
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "propsys", feature = "winnt"))]
    pub unsafe fn Get(&self, pszinstanceidentity: *const super::WCHAR, pdwindex: super::LPDWORD) -> windows_core::Result<super::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), pszinstanceidentity, pdwindex as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "propsys", feature = "shared"))]
    pub unsafe fn Item(&self, dwindex: super::DWORD) -> windows_core::Result<super::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn Add<P0>(&self, pipropertystore: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pipropertystore.param().abi()) }
    }
    #[cfg(all(feature = "propsys", feature = "shared"))]
    pub unsafe fn Remove(&self, dwindex: super::DWORD) -> windows_core::Result<super::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Delete(&self, dwindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), dwindex) }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetCount: usize,
    #[cfg(all(feature = "minwindef", feature = "propsys", feature = "winnt"))]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, super::LPDWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "propsys", feature = "winnt")))]
    Get: usize,
    #[cfg(all(feature = "propsys", feature = "shared"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "propsys", feature = "shared")))]
    Item: usize,
    #[cfg(feature = "propsys")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    Add: usize,
    #[cfg(all(feature = "propsys", feature = "shared"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "propsys", feature = "shared")))]
    Remove: usize,
    #[cfg(feature = "shared")]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Delete: usize,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "propsys", feature = "shared", feature = "winnt"))]
pub trait IPropertyStoreCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Get(&self, pszinstanceidentity: *const super::WCHAR, pdwindex: super::LPDWORD) -> windows_core::Result<super::IPropertyStore>;
    fn Item(&self, dwindex: super::DWORD) -> windows_core::Result<super::IPropertyStore>;
    fn Add(&self, pipropertystore: windows_core::Ref<super::IPropertyStore>) -> windows_core::Result<()>;
    fn Remove(&self, dwindex: super::DWORD) -> windows_core::Result<super::IPropertyStore>;
    fn Delete(&self, dwindex: super::DWORD) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "propsys", feature = "shared", feature = "winnt"))]
impl IPropertyStoreCollection_Vtbl {
    pub const fn new<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStoreCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Get<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinstanceidentity: *const super::WCHAR, pdwindex: super::LPDWORD, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStoreCollection_Impl::Get(this, core::mem::transmute_copy(&pszinstanceidentity), core::mem::transmute_copy(&pdwindex)) {
                    Ok(ok__) => {
                        ppipropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStoreCollection_Impl::Item(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppipropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreCollection_Impl::Add(this, core::mem::transmute_copy(&pipropertystore)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, pipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStoreCollection_Impl::Remove(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        pipropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreCollection_Impl::Delete(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreCollection_Impl::DeleteAll(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStoreCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "propsys", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IPropertyStoreCollection {}
pub const MAX_FDCONSTRAINTNAME_LENGTH: i32 = 100;
pub const MAX_FDCONSTRAINTVALUE_LENGTH: i32 = 1000;
pub const ONLINE_PROVIDER_DEVICES_QUERYCONSTRAINT_OWNERNAME: windows_core::PCWSTR = windows_core::w!("OwnerName");
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Create_Time: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf53b7e1c_77e0_4450_8c5f_a76cc7fde058), pid: 257 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_Characteristics: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 29 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_ClassCoInstallers: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x713d1703_a2e2_49f5_9214_56472ef3da5c), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_ClassInstaller: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_ClassName: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_DefaultService: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 11 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_DevType: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 27 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_Exclusive: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 28 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_Icon: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_IconPath: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 12 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_LowerFilters: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 20 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_Name: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_NoDisplayClass: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_NoInstallClass: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_NoUseClass: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_PropPageProvider: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_Security: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 25 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_SecuritySDS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 26 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_SilentInstall: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceClass_UpperFilters: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 19 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_Address: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 51 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_AlwaysShowDeviceAsConnected: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 101 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_AssociationArray: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 80 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_BaselineExperienceId: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 78 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_Category: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 90 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_CategoryGroup_Desc: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 94 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_CategoryGroup_Icon: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 95 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_Category_Desc_Plural: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 92 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_Category_Desc_Singular: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 91 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_Category_Icon: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 93 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_DeviceDescription1: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 81 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_DeviceDescription2: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 82 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_DeviceFunctionSubRank: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 100 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_DiscoveryMethod: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 52 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_ExperienceId: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 89 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_FriendlyName: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12288 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_Icon: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 57 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_InstallInProgress: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsAuthenticated: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 54 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsConnected: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 55 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsDefaultDevice: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 86 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsDeviceUniquelyIdentifiable: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 79 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsEncrypted: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 53 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsLocalMachine: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 70 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsMetadataSearchInProgress: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 72 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsNetworkDevice: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 85 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsNotInterestingForDisplay: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 74 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsNotWorkingProperly: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 83 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsPaired: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 56 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsSharedDevice: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 84 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_IsShowInDisconnectedState: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 68 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_Last_Connected: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 67 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_Last_Seen: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 66 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_LaunchDeviceStageFromExplorer: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 77 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_LaunchDeviceStageOnDeviceConnect: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 76 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_Manufacturer: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8192 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_MetadataCabinet: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 87 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_MetadataChecksum: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 73 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_MetadataPath: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 71 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_ModelName: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8194 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_ModelNumber: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8195 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_PrimaryCategory: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 97 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_RequiresPairingElevation: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 88 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_RequiresUninstallElevation: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 99 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_UnpairUninstall: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 98 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceDisplay_Version: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 65 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceInterfaceClass_DefaultInterface: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x14c83a99_0b3f_44b7_be4c_a178d3990564), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceInterface_ClassGuid: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceInterface_DevicePath: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x53808008_07bb_4661_bc3c_b5953e708560), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceInterface_Enabled: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DeviceInterface_FriendlyName: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_AdditionalSoftwareRequested: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 19 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Address: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 30 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_BIOSVersion: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xeaee7f1d_6a33_44d1_9441_5f46def23198), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_BaseContainerId: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 38 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_BusNumber: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 23 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_BusRelations: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_BusReportedDeviceDesc: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_BusTypeGuid: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 21 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Capabilities: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 17 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Characteristics: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 29 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Children: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Class: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_ClassGuid: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Comment: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 38 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_CompatibleIds: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_ConfigFlags: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 12 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_ContainerId: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8c7ed206_3f8a_4827_b3ab_ae9e1faefc6c), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DHP_Rebalance_Policy: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DevNodeStatus: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DevType: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 27 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DeviceDesc: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Driver: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 11 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DriverCoInstallers: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 11 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DriverDate: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DriverDesc: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DriverInfPath: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DriverInfSection: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DriverInfSectionExt: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DriverLogoLevel: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 15 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DriverPropPageProvider: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DriverProvider: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DriverRank: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 14 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_DriverVersion: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_EjectionRelations: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_EnumeratorName: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 24 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Exclusive: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 28 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_FriendlyName: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 14 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_FriendlyNameAttributes: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_GenericDriverInstalled: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 18 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_HardwareIds: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_InstallInProgress: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_InstallState: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 36 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_InstanceId: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 256 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Interface: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 257 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_IsAssociateableByUserAction: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Legacy: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_LegacyBusType: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 22 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_LocationInfo: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 15 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_LocationPaths: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 37 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_LowerFilters: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 20 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Manufacturer: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 13 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_ManufacturerAttributes: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_MatchingDeviceId: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Model: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 39 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_ModelId: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_NoConnectSound: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 17 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_NotPresent: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x904b03a2_471d_423c_a584_f3483238a146), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Numa_Node: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_PDOName: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 16 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Parent: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_PowerData: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 32 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_PowerRelations: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_PresenceNotForDevice: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_ProblemCode: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_QueueSize: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 36 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_RemovalPolicy: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 33 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_RemovalPolicyDefault: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 34 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_RemovalPolicyOverride: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 35 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_RemovalRelations: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Reported: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_ResourcePickerExceptions: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 13 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_ResourcePickerTags: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 12 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_SafeRemovalRequired: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_SafeRemovalRequiredOverride: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Security: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 25 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_SecuritySDS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 26 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Service: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Siblings: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_SignalStrength: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_Status: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 37 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_TransportRelations: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 11 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_UINumber: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 18 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_UINumberDescFormat: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 31 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Device_UpperFilters: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 19 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DrvPkg_BrandingIcon: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DrvPkg_DetailedDescription: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DrvPkg_DocumentationLink: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DrvPkg_Icon: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DrvPkg_Model: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_DrvPkg_VendorWebSite: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_ExposedCLSIDs: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 12291 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_ExposedIIDs: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 12290 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_FD_Visibility: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x904b03a2_471d_423c_a584_f3483238a146), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_FunctionInstance: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x08c0c253_a154_4746_9005_82de5317148b), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_Devinst: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4097 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_DisplayAttribute: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_DriverDate: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 11 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_DriverProvider: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_DriverVersion: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_Function: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4099 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_Icon: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_Image: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4098 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_Manufacturer: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_Model: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_Name: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_SerialNumber: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_ShellAttributes: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4100 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Hardware_Status: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4096 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_InstanceValidatorClsid: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 12292 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_NAME: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Numa_Proximity_Domain: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_Associated: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_AssociatedInstanceId: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_AssociationState: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_Category_Desc_NonPlural: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12304 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_CompactSignature: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28674 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_CompatibleTypes: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_DeviceCategory: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12292 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_DeviceCategory_Desc: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12293 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_DeviceCertHash: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28675 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_Devnode: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_DomainName: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 20480 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_FirmwareVersion: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12289 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_GlobalIdentity: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4096 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_ID: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4101 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_IPBusEnumerated: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28688 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_InstallState: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_Installable: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_IpAddress: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12297 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_LastNotificationTime: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_ManufacturerUrl: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8193 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_MetadataVersion: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4100 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_ModelUrl: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8196 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_NetworkInterfaceGuid: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12296 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_NetworkInterfaceLuid: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12295 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_PhysicalAddress: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12294 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_PresentationUrl: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8198 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_RemoteAddress: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4102 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_Removable: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28672 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_RootProxy: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4103 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_Scopes: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4098 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_SecureChannel: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28673 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_SerialNumber: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12290 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_ServiceAddress: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16384 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_ServiceControlUrl: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16388 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_ServiceDescUrl: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16389 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_ServiceEventSubUrl: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16390 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_ServiceId: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16385 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_ServiceTypes: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16386 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_ShareName: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 20482 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_Types: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4097 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_Upc: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8197 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PNPX_XAddrs: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4099 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PUBSVCS_METADATA: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63c6d5b8_f73a_4aca_967e_0cc787e0b559), pid: 20483 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PUBSVCS_METADATA_VERSION: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xc0c96c15_1823_4e5b_9348_e82519923f04), pid: 20484 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PUBSVCS_NETWORK_PROFILES_ALLOWED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63c6d5b8_f73a_4aca_967e_0cc787e0b559), pid: 20485 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PUBSVCS_NETWORK_PROFILES_DEFAULT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63c6d5b8_f73a_4aca_967e_0cc787e0b559), pid: 20487 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PUBSVCS_NETWORK_PROFILES_DENIED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63c6d5b8_f73a_4aca_967e_0cc787e0b559), pid: 20486 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PUBSVCS_SCOPE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ae2b567_eecb_4a3e_b753_54c725494366), pid: 20482 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_PUBSVCS_TYPE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf1b88ad3_109c_4fd2_ba3f_535a765f82f4), pid: 20481 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Pairing_IsWifiOnlyDevice: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 16 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Pairing_ListItemDefault: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Pairing_ListItemDescription: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Pairing_ListItemIcon: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Pairing_ListItemText: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_SSDP_AltLocationInfo: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24576 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_SSDP_DevLifeTime: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24577 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_SSDP_NetworkInterface: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24578 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_AssocState: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b88_4684_11da_a26a_0002b3988e81), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_AuthType: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b82_4684_11da_a26a_0002b3988e81), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_ConfigError: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_ConfigMethods: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b85_4684_11da_a26a_0002b3988e81), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_ConfigState: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 11 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_ConnType: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b84_4684_11da_a26a_0002b3988e81), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_DevicePasswordId: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 12 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_EncryptType: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b83_4684_11da_a26a_0002b3988e81), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_OSVersion: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 13 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_RegistrarType: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b8b_4684_11da_a26a_0002b3988e81), pid: 15 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_RequestType: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b81_4684_11da_a26a_0002b3988e81), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_RfBand: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b87_4684_11da_a26a_0002b3988e81), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_VendorExtension: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b8a_4684_11da_a26a_0002b3988e81), pid: 14 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WCN_Version: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b80_4684_11da_a26a_0002b3988e81), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WNET_Comment: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WNET_DisplayType: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WNET_LocalName: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WNET_Provider: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WNET_RemoteName: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WNET_Scope: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WNET_Type: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WNET_Usage: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WSD_AddressURI: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92), pid: 4096 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WSD_AppSeqInstanceID: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92), pid: 4100 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WSD_AppSeqMessageNumber: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92), pid: 4102 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WSD_AppSeqSessionID: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92), pid: 4101 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WSD_MetadataClean: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WSD_MetadataVersion: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92), pid: 4099 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WSD_Scopes: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92), pid: 4098 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WSD_ServiceInfo: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WSD_Types: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92), pid: 4097 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_WSD_XAddrs: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92), pid: 8192 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_Write_Time: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf53b7e1c_77e0_4450_8c5f_a76cc7fde058), pid: 256 };
pub const PNPX_DEVICECATEGORY_CAMERA: windows_core::PCWSTR = windows_core::w!("Cameras");
pub const PNPX_DEVICECATEGORY_COMPUTER: windows_core::PCWSTR = windows_core::w!("Computers");
pub const PNPX_DEVICECATEGORY_DISPLAYS: windows_core::PCWSTR = windows_core::w!("Displays");
pub const PNPX_DEVICECATEGORY_FAX: windows_core::PCWSTR = windows_core::w!("FAX");
pub const PNPX_DEVICECATEGORY_GAMING_DEVICE: windows_core::PCWSTR = windows_core::w!("Gaming");
pub const PNPX_DEVICECATEGORY_HOME_AUTOMATION_SYSTEM: windows_core::PCWSTR = windows_core::w!("HomeAutomation");
pub const PNPX_DEVICECATEGORY_HOME_SECURITY_SYSTEM: windows_core::PCWSTR = windows_core::w!("HomeSecurity");
pub const PNPX_DEVICECATEGORY_INPUTDEVICE: windows_core::PCWSTR = windows_core::w!("Input");
pub const PNPX_DEVICECATEGORY_MFP: windows_core::PCWSTR = windows_core::w!("MFP");
pub const PNPX_DEVICECATEGORY_MULTIMEDIA_DEVICE: windows_core::PCWSTR = windows_core::w!("MediaDevices");
pub const PNPX_DEVICECATEGORY_NETWORK_INFRASTRUCTURE: windows_core::PCWSTR = windows_core::w!("NetworkInfrastructure");
pub const PNPX_DEVICECATEGORY_OTHER: windows_core::PCWSTR = windows_core::w!("Other");
pub const PNPX_DEVICECATEGORY_PRINTER: windows_core::PCWSTR = windows_core::w!("Printers");
pub const PNPX_DEVICECATEGORY_SCANNER: windows_core::PCWSTR = windows_core::w!("Scanners");
pub const PNPX_DEVICECATEGORY_STORAGE: windows_core::PCWSTR = windows_core::w!("Storage");
pub const PNPX_DEVICECATEGORY_TELEPHONE: windows_core::PCWSTR = windows_core::w!("Phones");
pub const PNPX_INSTALLSTATE_FAILED: i32 = 3;
pub const PNPX_INSTALLSTATE_INSTALLED: i32 = 1;
pub const PNPX_INSTALLSTATE_INSTALLING: i32 = 2;
pub const PNPX_INSTALLSTATE_NOTINSTALLED: i32 = 0;
pub const PNP_CONSTRAINTVALUE_NOTIFICATIONSONLY: windows_core::PCWSTR = windows_core::w!("TRUE");
pub const PNP_CONSTRAINTVALUE_NOTPRESENT: windows_core::PCWSTR = windows_core::w!("TRUE");
pub const PROVIDERDDO_QUERYCONSTRAINT_DEVICEFUNCTIONDISPLAYOBJECTS: windows_core::PCWSTR = windows_core::w!("DeviceFunctionDisplayObjects");
pub const PROVIDERDDO_QUERYCONSTRAINT_DEVICEINTERFACES: windows_core::PCWSTR = windows_core::w!("DeviceInterfaces");
pub const PROVIDERDDO_QUERYCONSTRAINT_ONLYCONNECTEDDEVICES: windows_core::PCWSTR = windows_core::w!("OnlyConnectedDevices");
pub const PROVIDERPNP_QUERYCONSTRAINT_INTERFACECLASS: windows_core::PCWSTR = windows_core::w!("InterfaceClass");
pub const PROVIDERPNP_QUERYCONSTRAINT_NOTIFICATIONSONLY: windows_core::PCWSTR = windows_core::w!("NotifyOnly");
pub const PROVIDERPNP_QUERYCONSTRAINT_NOTPRESENT: windows_core::PCWSTR = windows_core::w!("NotPresent");
pub const PROVIDERSSDP_QUERYCONSTRAINT_CUSTOMXMLPROPERTY: windows_core::PCWSTR = windows_core::w!("CustomXmlProperty");
pub const PROVIDERSSDP_QUERYCONSTRAINT_TYPE: windows_core::PCWSTR = windows_core::w!("Type");
pub const PROVIDERWNET_QUERYCONSTRAINT_PROPERTIES: windows_core::PCWSTR = windows_core::w!("Properties");
pub const PROVIDERWNET_QUERYCONSTRAINT_RESOURCETYPE: windows_core::PCWSTR = windows_core::w!("ResourceType");
pub const PROVIDERWNET_QUERYCONSTRAINT_TYPE: windows_core::PCWSTR = windows_core::w!("Type");
pub const PROVIDERWSD_QUERYCONSTRAINT_DIRECTEDADDRESS: windows_core::PCWSTR = windows_core::w!("RemoteAddress");
pub const PROVIDERWSD_QUERYCONSTRAINT_SCOPE: windows_core::PCWSTR = windows_core::w!("Scope");
pub const PROVIDERWSD_QUERYCONSTRAINT_SECURITY_REQUIREMENTS: windows_core::PCWSTR = windows_core::w!("SecurityRequirements");
pub const PROVIDERWSD_QUERYCONSTRAINT_SSL_CERTHASH_FOR_SERVER_AUTH: windows_core::PCWSTR = windows_core::w!("SSLServerAuthCertHash");
pub const PROVIDERWSD_QUERYCONSTRAINT_SSL_CERT_FOR_CLIENT_AUTH: windows_core::PCWSTR = windows_core::w!("SSLClientAuthCert");
pub const PROVIDERWSD_QUERYCONSTRAINT_TYPE: windows_core::PCWSTR = windows_core::w!("Type");
pub type PropertyConstraint = i32;
pub const QCT_LAYERED: QueryCategoryType = 1;
pub const QCT_PROVIDER: QueryCategoryType = 0;
pub const QC_CONTAINS: PropertyConstraint = 9;
pub const QC_DOESNOTEXIST: PropertyConstraint = 8;
pub const QC_EQUALS: PropertyConstraint = 0;
pub const QC_EXISTS: PropertyConstraint = 7;
pub const QC_GREATERTHAN: PropertyConstraint = 4;
pub const QC_GREATERTHANOREQUAL: PropertyConstraint = 5;
pub const QC_LESSTHAN: PropertyConstraint = 2;
pub const QC_LESSTHANOREQUAL: PropertyConstraint = 3;
pub const QC_NOTEQUAL: PropertyConstraint = 1;
pub const QC_STARTSWITH: PropertyConstraint = 6;
pub const QUA_ADD: QueryUpdateAction = 0;
pub const QUA_CHANGE: QueryUpdateAction = 2;
pub const QUA_REMOVE: QueryUpdateAction = 1;
pub type QueryCategoryType = i32;
pub type QueryUpdateAction = i32;
pub const SID_DeviceDisplayStatusManager: windows_core::GUID = windows_core::GUID::from_u128(0xf59aa553_8309_46ca_9736_1ac3c62d6031);
pub const SID_EnumDeviceFunction: windows_core::GUID = windows_core::GUID::from_u128(0x13e0e9e2_c3fa_4e3c_906e_64502fa4dc95);
pub const SID_EnumInterface: windows_core::GUID = windows_core::GUID::from_u128(0x40eab0b9_4d7f_4b53_a334_1581dd9041f4);
pub const SID_FDPairingHandler: windows_core::GUID = windows_core::GUID::from_u128(0x383b69fa_5486_49da_91f5_d63c24c8e9d0);
pub const SID_FunctionDiscoveryProviderRefresh: windows_core::GUID = windows_core::GUID::from_u128(0x2b4cbdc9_31c4_40d4_a62d_772aa174ed52);
pub const SID_PNPXAssociation: windows_core::GUID = windows_core::GUID::from_u128(0xcee8ccc9_4f6b_4469_a235_5a22869eef03);
pub const SID_PNPXPropertyStore: windows_core::GUID = windows_core::GUID::from_u128(0xa86530b1_542f_439f_b71c_b0756b13677a);
pub const SID_PNPXServiceCollection: windows_core::GUID = windows_core::GUID::from_u128(0x439e80ee_a217_4712_9fa6_deabd9c2a727);
pub const SID_PnpProvider: windows_core::GUID = windows_core::GUID::from_u128(0x8101368e_cabb_4426_acff_96c410812000);
pub const SID_UPnPActivator: windows_core::GUID = windows_core::GUID::from_u128(0x0d0d66eb_cf74_4164_b52f_08344672dd46);
pub const SID_UninstallDeviceFunction: windows_core::GUID = windows_core::GUID::from_u128(0xc920566e_5671_4496_8025_bf0b89bd44cd);
pub const SID_UnpairProvider: windows_core::GUID = windows_core::GUID::from_u128(0x89a502fc_857b_4698_a0b7_027192002f9e);
pub const SSDP_CONSTRAINTVALUE_TYPE_ALL: windows_core::PCWSTR = windows_core::w!("ssdp:all");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEVICE_PREFIX: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEV_IGD: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:InternetGatewayDevice:1");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEV_LANDEVICE: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:LANDevice:1");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEV_LIGHTING: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:Lighting:1");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEV_LUXMETER: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:Luxmeter:1");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEV_MDARNDR: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:MediaRenderer:1");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEV_MDASRVR: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:MediaServer:1");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEV_POWERDEVICE: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:PowerDevice:1");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEV_REMINDER: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:Reminder:1");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEV_WANCONNDEVICE: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:WANConnectionDevice:1");
pub const SSDP_CONSTRAINTVALUE_TYPE_DEV_WANDEVICE: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:device:WANDevice:1");
pub const SSDP_CONSTRAINTVALUE_TYPE_ROOT: windows_core::PCWSTR = windows_core::w!("upnp:rootdevice");
pub const SSDP_CONSTRAINTVALUE_TYPE_SVC_DIMMING: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:service:DimmingService:1");
pub const SSDP_CONSTRAINTVALUE_TYPE_SVC_PREFIX: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:service:");
pub const SSDP_CONSTRAINTVALUE_TYPE_SVC_SCANNER: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:service:Scanner:1");
pub const SVF_SYSTEM: SystemVisibilityFlags = 0;
pub const SVF_USER: SystemVisibilityFlags = 1;
pub type SystemVisibilityFlags = i32;
pub const WNET_CONSTRAINTVALUE_PROPERTIES_ALL: windows_core::PCWSTR = windows_core::w!("All");
pub const WNET_CONSTRAINTVALUE_PROPERTIES_LIMITED: windows_core::PCWSTR = windows_core::w!("Limited");
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_DISK: windows_core::PCWSTR = windows_core::w!("Disk");
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_DISKORPRINTER: windows_core::PCWSTR = windows_core::w!("DiskOrPrinter");
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_PRINTER: windows_core::PCWSTR = windows_core::w!("Printer");
pub const WNET_CONSTRAINTVALUE_TYPE_ALL: windows_core::PCWSTR = windows_core::w!("All");
pub const WNET_CONSTRAINTVALUE_TYPE_DOMAIN: windows_core::PCWSTR = windows_core::w!("Domain");
pub const WNET_CONSTRAINTVALUE_TYPE_SERVER: windows_core::PCWSTR = windows_core::w!("Server");
pub const WSD_CONSTRAINTVALUE_NO_TRUST_VERIFICATION: windows_core::PCWSTR = windows_core::w!("3");
pub const WSD_CONSTRAINTVALUE_REQUIRE_SECURECHANNEL: windows_core::PCWSTR = windows_core::w!("1");
pub const WSD_CONSTRAINTVALUE_REQUIRE_SECURECHANNEL_AND_COMPACTSIGNATURE: windows_core::PCWSTR = windows_core::w!("2");
