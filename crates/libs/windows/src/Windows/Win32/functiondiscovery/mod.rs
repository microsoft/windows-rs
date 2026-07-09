windows_core::imp::define_interface!(CFunctionDiscoveryNotificationWrapper, CFunctionDiscoveryNotificationWrapper_Vtbl, 0);
impl core::ops::Deref for CFunctionDiscoveryNotificationWrapper {
    type Target = IFunctionDiscoveryNotification;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(CFunctionDiscoveryNotificationWrapper, windows_core::IUnknown, IFunctionDiscoveryNotification);
#[repr(C)]
#[doc(hidden)]
pub struct CFunctionDiscoveryNotificationWrapper_Vtbl {
    pub base__: IFunctionDiscoveryNotification_Vtbl,
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
pub trait CFunctionDiscoveryNotificationWrapper_Impl: IFunctionDiscoveryNotification_Impl {}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
impl CFunctionDiscoveryNotificationWrapper_Vtbl {
    pub const fn new<Identity: CFunctionDiscoveryNotificationWrapper_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IFunctionDiscoveryNotification_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<CFunctionDiscoveryNotificationWrapper as windows_core::Interface>::IID || iid == &<IFunctionDiscoveryNotification as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for CFunctionDiscoveryNotificationWrapper {}
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
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FDQUERYCONTEXT(pub super::winnt::DWORDLONG);
pub const FD_CONSTRAINTVALUE_ALL: windows_core::PCWSTR = windows_core::w!("All");
pub const FD_CONSTRAINTVALUE_COMCLSCONTEXT_INPROC_SERVER: windows_core::PCWSTR = windows_core::w!("1");
pub const FD_CONSTRAINTVALUE_COMCLSCONTEXT_LOCAL_SERVER: windows_core::PCWSTR = windows_core::w!("4");
pub const FD_CONSTRAINTVALUE_FALSE: windows_core::PCWSTR = windows_core::w!("FALSE");
pub const FD_CONSTRAINTVALUE_PAIRED: windows_core::PCWSTR = windows_core::w!("Paired");
pub const FD_CONSTRAINTVALUE_ROUTINGSCOPE_ALL: windows_core::PCWSTR = windows_core::w!("All");
pub const FD_CONSTRAINTVALUE_ROUTINGSCOPE_DIRECT: windows_core::PCWSTR = windows_core::w!("Direct");
pub const FD_CONSTRAINTVALUE_TRUE: windows_core::PCWSTR = windows_core::w!("TRUE");
pub const FD_CONSTRAINTVALUE_UNPAIRED: windows_core::PCWSTR = windows_core::w!("UnPaired");
pub const FD_CONSTRAINTVALUE_VISIBILITY_ALL: windows_core::PCWSTR = windows_core::w!("1");
pub const FD_CONSTRAINTVALUE_VISIBILITY_DEFAULT: windows_core::PCWSTR = windows_core::w!("0");
pub const FD_EVENTID: u32 = 1000;
pub const FD_EVENTID_ASYNCTHREADEXIT: u32 = 1001;
pub const FD_EVENTID_IPADDRESSCHANGE: u32 = 1003;
pub const FD_EVENTID_PRIVATE: u32 = 100;
pub const FD_EVENTID_QUERYREFRESH: u32 = 1004;
pub const FD_EVENTID_SEARCHCOMPLETE: u32 = 1000;
pub const FD_EVENTID_SEARCHSTART: u32 = 1002;
pub const FD_LONGHORN: u32 = 1;
pub const FD_QUERYCONSTRAINT_COMCLSCONTEXT: windows_core::PCWSTR = windows_core::w!("COMClsContext");
pub const FD_QUERYCONSTRAINT_INQUIRY_TIMEOUT: windows_core::PCWSTR = windows_core::w!("InquiryModeTimeout");
pub const FD_QUERYCONSTRAINT_PAIRING_STATE: windows_core::PCWSTR = windows_core::w!("PairingState");
pub const FD_QUERYCONSTRAINT_PROVIDERINSTANCEID: windows_core::PCWSTR = windows_core::w!("ProviderInstanceID");
pub const FD_QUERYCONSTRAINT_RECURSESUBCATEGORY: windows_core::PCWSTR = windows_core::w!("RecurseSubcategory");
pub const FD_QUERYCONSTRAINT_ROUTINGSCOPE: windows_core::PCWSTR = windows_core::w!("RoutingScope");
pub const FD_QUERYCONSTRAINT_SUBCATEGORY: windows_core::PCWSTR = windows_core::w!("Subcategory");
pub const FD_QUERYCONSTRAINT_VISIBILITY: windows_core::PCWSTR = windows_core::w!("Visibility");
pub const FD_SUBKEY: windows_core::PCWSTR = windows_core::w!("SOFTWARE\\Microsoft\\Function Discovery\\");
pub const FD_Visibility_Default: u32 = 0;
pub const FD_Visibility_Hidden: u32 = 1;
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
    pub unsafe fn GetInstanceCollection(&self, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: bool) -> windows_core::Result<IFunctionInstanceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstanceCollection)(windows_core::Interface::as_raw(self), pszcategory, pszsubcategory, fincludeallsubcategories.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn GetInstance(&self, pszfunctioninstanceidentity: *const u16) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstance)(windows_core::Interface::as_raw(self), pszfunctioninstanceidentity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CreateInstanceCollectionQuery<P3>(&self, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: bool, pifunctiondiscoverynotification: P3, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceCollectionQuery>
    where
        P3: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceCollectionQuery)(windows_core::Interface::as_raw(self), pszcategory, pszsubcategory, fincludeallsubcategories.into(), pifunctiondiscoverynotification.param().abi(), pfdqcquerycontext as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CreateInstanceQuery<P1>(&self, pszfunctioninstanceidentity: *const u16, pifunctiondiscoverynotification: P1, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceQuery>
    where
        P1: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceQuery)(windows_core::Interface::as_raw(self), pszfunctioninstanceidentity, pifunctiondiscoverynotification.param().abi(), pfdqcquerycontext as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn AddInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddInstance)(windows_core::Interface::as_raw(self), enumsystemvisibility, pszcategory, pszsubcategory, pszcategoryidentity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveInstance)(windows_core::Interface::as_raw(self), enumsystemvisibility, pszcategory, pszsubcategory, pszcategoryidentity) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscovery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInstanceCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_servprov")]
    pub GetInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    GetInstance: usize,
    #[cfg(feature = "Win32_winnt")]
    pub CreateInstanceCollectionQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, windows_core::BOOL, *mut core::ffi::c_void, *mut FDQUERYCONTEXT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CreateInstanceCollectionQuery: usize,
    #[cfg(feature = "Win32_winnt")]
    pub CreateInstanceQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut core::ffi::c_void, *mut FDQUERYCONTEXT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CreateInstanceQuery: usize,
    #[cfg(feature = "Win32_servprov")]
    pub AddInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, *const u16, *const u16, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    AddInstance: usize,
    pub RemoveInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, *const u16, *const u16, *const u16) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
pub trait IFunctionDiscovery_Impl: windows_core::IUnknownImpl {
    fn GetInstanceCollection(&self, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: windows_core::BOOL) -> windows_core::Result<IFunctionInstanceCollection>;
    fn GetInstance(&self, pszfunctioninstanceidentity: *const u16) -> windows_core::Result<IFunctionInstance>;
    fn CreateInstanceCollectionQuery(&self, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: windows_core::BOOL, pifunctiondiscoverynotification: windows_core::Ref<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceCollectionQuery>;
    fn CreateInstanceQuery(&self, pszfunctioninstanceidentity: *const u16, pifunctiondiscoverynotification: windows_core::Ref<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceQuery>;
    fn AddInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::Result<IFunctionInstance>;
    fn RemoveInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
impl IFunctionDiscovery_Vtbl {
    pub const fn new<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInstanceCollection<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: windows_core::BOOL, ppifunctioninstancecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetInstance<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfunctioninstanceidentity: *const u16, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceCollectionQuery<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: windows_core::BOOL, pifunctiondiscoverynotification: *mut core::ffi::c_void, pfdqcquerycontext: *mut FDQUERYCONTEXT, ppifunctioninstancecollectionquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceQuery<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfunctioninstanceidentity: *const u16, pifunctiondiscoverynotification: *mut core::ffi::c_void, pfdqcquerycontext: *mut FDQUERYCONTEXT, ppifunctioninstancequery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AddInstance<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RemoveInstance<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::HRESULT {
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
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IFunctionDiscovery {}
windows_core::imp::define_interface!(IFunctionDiscoveryNotification, IFunctionDiscoveryNotification_Vtbl, 0x5f6c1ba8_5330_422e_a368_572b244d3f87);
windows_core::imp::interface_hierarchy!(IFunctionDiscoveryNotification, windows_core::IUnknown);
impl IFunctionDiscoveryNotification {
    #[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
    pub unsafe fn OnUpdate<P2>(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: FDQUERYCONTEXT, pifunctioninstance: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IFunctionInstance>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnUpdate)(windows_core::Interface::as_raw(self), enumqueryupdateaction, fdqcquerycontext, pifunctioninstance.param().abi()) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn OnError(&self, hr: windows_core::HRESULT, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hr, fdqcquerycontext, pszprovider) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn OnEvent(&self, dweventid: u32, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), dweventid, fdqcquerycontext, pszprovider) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
    pub OnUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, QueryUpdateAction, FDQUERYCONTEXT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_servprov", feature = "Win32_winnt")))]
    OnUpdate: usize,
    #[cfg(feature = "Win32_winnt")]
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, FDQUERYCONTEXT, *const u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    OnError: usize,
    #[cfg(feature = "Win32_winnt")]
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, FDQUERYCONTEXT, *const u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    OnEvent: usize,
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
pub trait IFunctionDiscoveryNotification_Impl: windows_core::IUnknownImpl {
    fn OnUpdate(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: FDQUERYCONTEXT, pifunctioninstance: windows_core::Ref<IFunctionInstance>) -> windows_core::Result<()>;
    fn OnError(&self, hr: windows_core::HRESULT, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::Result<()>;
    fn OnEvent(&self, dweventid: u32, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
impl IFunctionDiscoveryNotification_Vtbl {
    pub const fn new<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdate<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: FDQUERYCONTEXT, pifunctioninstance: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionDiscoveryNotification_Impl::OnUpdate(this, core::mem::transmute_copy(&enumqueryupdateaction), core::mem::transmute_copy(&fdqcquerycontext), core::mem::transmute_copy(&pifunctioninstance)).into()
            }
        }
        unsafe extern "system" fn OnError<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionDiscoveryNotification_Impl::OnError(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&fdqcquerycontext), core::mem::transmute_copy(&pszprovider)).into()
            }
        }
        unsafe extern "system" fn OnEvent<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweventid: u32, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::HRESULT {
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
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IFunctionDiscoveryNotification {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IFunctionDiscoveryProviderRefresh(pub u8);
#[cfg(feature = "Win32_servprov")]
windows_core::imp::define_interface!(IFunctionInstance, IFunctionInstance_Vtbl, 0x33591c10_0bed_4f02_b0ab_1530d5533ee9);
#[cfg(feature = "Win32_servprov")]
impl core::ops::Deref for IFunctionInstance {
    type Target = super::servprov::IServiceProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_servprov")]
windows_core::imp::interface_hierarchy!(IFunctionInstance, windows_core::IUnknown, super::servprov::IServiceProvider);
#[cfg(feature = "Win32_servprov")]
impl IFunctionInstance {
    pub unsafe fn GetID(&self) -> windows_core::Result<*mut u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProviderInstanceID(&self) -> windows_core::Result<*mut u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderInstanceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_propsys")]
    pub unsafe fn OpenPropertyStore(&self, dwstgaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenPropertyStore)(windows_core::Interface::as_raw(self), dwstgaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), ppszcomemcategory as _, ppszcomemsubcategory as _) }
    }
}
#[cfg(feature = "Win32_servprov")]
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstance_Vtbl {
    pub base__: super::servprov::IServiceProvider_Vtbl,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    pub GetProviderInstanceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_propsys")]
    pub OpenPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_propsys"))]
    OpenPropertyStore: usize,
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16, *mut *mut u16) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_propsys", feature = "Win32_servprov"))]
pub trait IFunctionInstance_Impl: super::servprov::IServiceProvider_Impl {
    fn GetID(&self) -> windows_core::Result<*mut u16>;
    fn GetProviderInstanceID(&self) -> windows_core::Result<*mut u16>;
    fn OpenPropertyStore(&self, dwstgaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_propsys", feature = "Win32_servprov"))]
impl IFunctionInstance_Vtbl {
    pub const fn new<Identity: IFunctionInstance_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetID<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemidentity: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstance_Impl::GetID(this) {
                    Ok(ok__) => {
                        ppszcomemidentity.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProviderInstanceID<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstance_Impl::GetProviderInstanceID(this) {
                    Ok(ok__) => {
                        ppszcomemproviderinstanceidentity.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstgaccess: u32, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCategory<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstance_Impl::GetCategory(this, core::mem::transmute_copy(&ppszcomemcategory), core::mem::transmute_copy(&ppszcomemsubcategory)).into()
            }
        }
        Self {
            base__: super::servprov::IServiceProvider_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, OFFSET>,
            GetProviderInstanceID: GetProviderInstanceID::<Identity, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, OFFSET>,
            GetCategory: GetCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstance as windows_core::Interface>::IID || iid == &<super::servprov::IServiceProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_propsys", feature = "Win32_servprov"))]
impl windows_core::RuntimeName for IFunctionInstance {}
windows_core::imp::define_interface!(IFunctionInstanceCollection, IFunctionInstanceCollection_Vtbl, 0xf0a3d895_855c_42a2_948d_2f97d450ecb1);
windows_core::imp::interface_hierarchy!(IFunctionInstanceCollection, windows_core::IUnknown);
impl IFunctionInstanceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), pszinstanceidentity, pdwindex as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn Item(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn Add<P0>(&self, pifunctioninstance: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi()) }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn Remove(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self, dwindex: u32) -> windows_core::HRESULT {
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
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_servprov")]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    Get: usize,
    #[cfg(feature = "Win32_servprov")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    Item: usize,
    #[cfg(feature = "Win32_servprov")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    Add: usize,
    #[cfg(feature = "Win32_servprov")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_servprov")]
pub trait IFunctionInstanceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<IFunctionInstance>;
    fn Item(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance>;
    fn Add(&self, pifunctioninstance: windows_core::Ref<IFunctionInstance>) -> windows_core::Result<()>;
    fn Remove(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance>;
    fn Delete(&self, dwindex: u32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_servprov")]
impl IFunctionInstanceCollection_Vtbl {
    pub const fn new<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Get<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinstanceidentity: *const u16, pdwindex: *mut u32, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Item<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Remove<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Delete<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_servprov")]
impl windows_core::RuntimeName for IFunctionInstanceCollection {}
windows_core::imp::define_interface!(IFunctionInstanceCollectionQuery, IFunctionInstanceCollectionQuery_Vtbl, 0x57cc6fd2_c09a_4289_bb72_25f04142058e);
windows_core::imp::interface_hierarchy!(IFunctionInstanceCollectionQuery, windows_core::IUnknown);
impl IFunctionInstanceCollectionQuery {
    pub unsafe fn AddQueryConstraint(&self, pszconstraintname: *const u16, pszconstraintvalue: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddQueryConstraint)(windows_core::Interface::as_raw(self), pszconstraintname, pszconstraintvalue) }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn AddPropertyConstraint(&self, key: *const super::wtypes::PROPERTYKEY, pv: *const super::propidlbase::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyConstraint)(windows_core::Interface::as_raw(self), key, core::mem::transmute(pv), enumpropertyconstraint) }
    }
    pub unsafe fn Execute(&self) -> windows_core::Result<IFunctionInstanceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollectionQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddQueryConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub AddPropertyConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *const super::propidlbase::PROPVARIANT, PropertyConstraint) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    AddPropertyConstraint: usize,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFunctionInstanceCollectionQuery_Impl: windows_core::IUnknownImpl {
    fn AddQueryConstraint(&self, pszconstraintname: *const u16, pszconstraintvalue: *const u16) -> windows_core::Result<()>;
    fn AddPropertyConstraint(&self, key: *const super::wtypes::PROPERTYKEY, pv: *const super::propidlbase::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::Result<()>;
    fn Execute(&self) -> windows_core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFunctionInstanceCollectionQuery_Vtbl {
    pub const fn new<Identity: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddQueryConstraint<Identity: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconstraintname: *const u16, pszconstraintvalue: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstanceCollectionQuery_Impl::AddQueryConstraint(this, core::mem::transmute_copy(&pszconstraintname), core::mem::transmute_copy(&pszconstraintvalue)).into()
            }
        }
        unsafe extern "system" fn AddPropertyConstraint<Identity: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, pv: *const super::propidlbase::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::HRESULT {
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
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFunctionInstanceCollectionQuery {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IFunctionInstanceCollectionQuery2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IFunctionInstanceCollectionQueryCollection(pub u8);
windows_core::imp::define_interface!(IFunctionInstanceQuery, IFunctionInstanceQuery_Vtbl, 0x6242bc6b_90ec_4b37_bb46_e229fd84ed95);
windows_core::imp::interface_hierarchy!(IFunctionInstanceQuery, windows_core::IUnknown);
impl IFunctionInstanceQuery {
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn Execute(&self) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_servprov")]
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    Execute: usize,
}
#[cfg(feature = "Win32_servprov")]
pub trait IFunctionInstanceQuery_Impl: windows_core::IUnknownImpl {
    fn Execute(&self) -> windows_core::Result<IFunctionInstance>;
}
#[cfg(feature = "Win32_servprov")]
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
#[cfg(feature = "Win32_servprov")]
impl windows_core::RuntimeName for IFunctionInstanceQuery {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IFunctionInstanceQuery2(pub u8);
windows_core::imp::define_interface!(IPropertyStoreCollection, IPropertyStoreCollection_Vtbl, 0xd14d9c30_12d2_42d8_bce4_c60c2bb226fa);
windows_core::imp::interface_hierarchy!(IPropertyStoreCollection, windows_core::IUnknown);
impl IPropertyStoreCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_propsys")]
    pub unsafe fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), pszinstanceidentity, pdwindex as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_propsys")]
    pub unsafe fn Item(&self, dwindex: u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_propsys")]
    pub unsafe fn Add<P0>(&self, pipropertystore: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::propsys::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pipropertystore.param().abi()) }
    }
    #[cfg(feature = "Win32_propsys")]
    pub unsafe fn Remove(&self, dwindex: u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self, dwindex: u32) -> windows_core::HRESULT {
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
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_propsys")]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_propsys"))]
    Get: usize,
    #[cfg(feature = "Win32_propsys")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_propsys"))]
    Item: usize,
    #[cfg(feature = "Win32_propsys")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_propsys"))]
    Add: usize,
    #[cfg(feature = "Win32_propsys")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_propsys"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_propsys")]
pub trait IPropertyStoreCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn Item(&self, dwindex: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn Add(&self, pipropertystore: windows_core::Ref<super::propsys::IPropertyStore>) -> windows_core::Result<()>;
    fn Remove(&self, dwindex: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn Delete(&self, dwindex: u32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_propsys")]
impl IPropertyStoreCollection_Vtbl {
    pub const fn new<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Get<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinstanceidentity: *const u16, pdwindex: *mut u32, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Item<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Remove<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Delete<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_propsys")]
impl windows_core::RuntimeName for IPropertyStoreCollection {}
pub const MAX_FDCONSTRAINTNAME_LENGTH: u32 = 100;
pub const MAX_FDCONSTRAINTVALUE_LENGTH: u32 = 1000;
pub const ONLINE_PROVIDER_DEVICES_QUERYCONSTRAINT_OWNERNAME: windows_core::PCWSTR = windows_core::w!("OwnerName");
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
pub const PNPX_INSTALLSTATE_FAILED: u32 = 3;
pub const PNPX_INSTALLSTATE_INSTALLED: u32 = 1;
pub const PNPX_INSTALLSTATE_INSTALLING: u32 = 2;
pub const PNPX_INSTALLSTATE_NOTINSTALLED: u32 = 0;
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
pub const SSDP_CONSTRAINTVALUE_TYPE_ROOT: windows_core::PCWSTR = windows_core::w!("upnp:rootdevice");
pub const SSDP_CONSTRAINTVALUE_TYPE_SVC_PREFIX: windows_core::PCWSTR = windows_core::w!("urn:schemas-upnp-org:service:");
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
