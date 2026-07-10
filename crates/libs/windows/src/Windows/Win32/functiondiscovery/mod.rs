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
#[cfg(all(feature = "servprov", feature = "winnt"))]
pub trait CFunctionDiscoveryNotificationWrapper_Impl: IFunctionDiscoveryNotification_Impl {}
#[cfg(all(feature = "servprov", feature = "winnt"))]
impl CFunctionDiscoveryNotificationWrapper_Vtbl {
    pub const fn new<Identity: CFunctionDiscoveryNotificationWrapper_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IFunctionDiscoveryNotification_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<CFunctionDiscoveryNotificationWrapper as windows_core::Interface>::IID || iid == &<IFunctionDiscoveryNotification as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "servprov", feature = "winnt"))]
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
#[cfg(feature = "winnt")]
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
    #[cfg(feature = "servprov")]
    pub unsafe fn GetInstance(&self, pszfunctioninstanceidentity: *const u16) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstance)(windows_core::Interface::as_raw(self), pszfunctioninstanceidentity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn CreateInstanceCollectionQuery<P3>(&self, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: bool, pifunctiondiscoverynotification: P3, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceCollectionQuery>
    where
        P3: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceCollectionQuery)(windows_core::Interface::as_raw(self), pszcategory, pszsubcategory, fincludeallsubcategories.into(), pifunctiondiscoverynotification.param().abi(), pfdqcquerycontext as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn CreateInstanceQuery<P1>(&self, pszfunctioninstanceidentity: *const u16, pifunctiondiscoverynotification: P1, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceQuery>
    where
        P1: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceQuery)(windows_core::Interface::as_raw(self), pszfunctioninstanceidentity, pifunctiondiscoverynotification.param().abi(), pfdqcquerycontext as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "servprov")]
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
    #[cfg(feature = "servprov")]
    pub GetInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "servprov"))]
    GetInstance: usize,
    #[cfg(feature = "winnt")]
    pub CreateInstanceCollectionQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, windows_core::BOOL, *mut core::ffi::c_void, *mut FDQUERYCONTEXT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    CreateInstanceCollectionQuery: usize,
    #[cfg(feature = "winnt")]
    pub CreateInstanceQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut core::ffi::c_void, *mut FDQUERYCONTEXT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    CreateInstanceQuery: usize,
    #[cfg(feature = "servprov")]
    pub AddInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, *const u16, *const u16, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "servprov"))]
    AddInstance: usize,
    pub RemoveInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, *const u16, *const u16, *const u16) -> windows_core::HRESULT,
}
#[cfg(all(feature = "servprov", feature = "winnt"))]
pub trait IFunctionDiscovery_Impl: windows_core::IUnknownImpl {
    fn GetInstanceCollection(&self, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: windows_core::BOOL) -> windows_core::Result<IFunctionInstanceCollection>;
    fn GetInstance(&self, pszfunctioninstanceidentity: *const u16) -> windows_core::Result<IFunctionInstance>;
    fn CreateInstanceCollectionQuery(&self, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: windows_core::BOOL, pifunctiondiscoverynotification: windows_core::Ref<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceCollectionQuery>;
    fn CreateInstanceQuery(&self, pszfunctioninstanceidentity: *const u16, pifunctiondiscoverynotification: windows_core::Ref<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceQuery>;
    fn AddInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::Result<IFunctionInstance>;
    fn RemoveInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "servprov", feature = "winnt"))]
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
#[cfg(all(feature = "servprov", feature = "winnt"))]
impl windows_core::RuntimeName for IFunctionDiscovery {}
windows_core::imp::define_interface!(IFunctionDiscoveryNotification, IFunctionDiscoveryNotification_Vtbl, 0x5f6c1ba8_5330_422e_a368_572b244d3f87);
windows_core::imp::interface_hierarchy!(IFunctionDiscoveryNotification, windows_core::IUnknown);
impl IFunctionDiscoveryNotification {
    #[cfg(all(feature = "servprov", feature = "winnt"))]
    pub unsafe fn OnUpdate<P2>(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: FDQUERYCONTEXT, pifunctioninstance: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IFunctionInstance>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnUpdate)(windows_core::Interface::as_raw(self), enumqueryupdateaction, fdqcquerycontext, pifunctioninstance.param().abi()) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn OnError(&self, hr: windows_core::HRESULT, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hr, fdqcquerycontext, pszprovider) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn OnEvent(&self, dweventid: u32, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), dweventid, fdqcquerycontext, pszprovider) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "servprov", feature = "winnt"))]
    pub OnUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, QueryUpdateAction, FDQUERYCONTEXT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "servprov", feature = "winnt")))]
    OnUpdate: usize,
    #[cfg(feature = "winnt")]
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, FDQUERYCONTEXT, *const u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    OnError: usize,
    #[cfg(feature = "winnt")]
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, FDQUERYCONTEXT, *const u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    OnEvent: usize,
}
#[cfg(all(feature = "servprov", feature = "winnt"))]
pub trait IFunctionDiscoveryNotification_Impl: windows_core::IUnknownImpl {
    fn OnUpdate(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: FDQUERYCONTEXT, pifunctioninstance: windows_core::Ref<IFunctionInstance>) -> windows_core::Result<()>;
    fn OnError(&self, hr: windows_core::HRESULT, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::Result<()>;
    fn OnEvent(&self, dweventid: u32, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "servprov", feature = "winnt"))]
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
#[cfg(all(feature = "servprov", feature = "winnt"))]
impl windows_core::RuntimeName for IFunctionDiscoveryNotification {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IFunctionDiscoveryProviderRefresh(pub u8);
#[cfg(feature = "servprov")]
windows_core::imp::define_interface!(IFunctionInstance, IFunctionInstance_Vtbl, 0x33591c10_0bed_4f02_b0ab_1530d5533ee9);
#[cfg(feature = "servprov")]
impl core::ops::Deref for IFunctionInstance {
    type Target = super::servprov::IServiceProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "servprov")]
windows_core::imp::interface_hierarchy!(IFunctionInstance, windows_core::IUnknown, super::servprov::IServiceProvider);
#[cfg(feature = "servprov")]
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
    #[cfg(feature = "propsys")]
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
#[cfg(feature = "servprov")]
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstance_Vtbl {
    pub base__: super::servprov::IServiceProvider_Vtbl,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    pub GetProviderInstanceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(feature = "propsys")]
    pub OpenPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    OpenPropertyStore: usize,
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16, *mut *mut u16) -> windows_core::HRESULT,
}
#[cfg(all(feature = "propsys", feature = "servprov"))]
pub trait IFunctionInstance_Impl: super::servprov::IServiceProvider_Impl {
    fn GetID(&self) -> windows_core::Result<*mut u16>;
    fn GetProviderInstanceID(&self) -> windows_core::Result<*mut u16>;
    fn OpenPropertyStore(&self, dwstgaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "propsys", feature = "servprov"))]
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
#[cfg(all(feature = "propsys", feature = "servprov"))]
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
    #[cfg(feature = "servprov")]
    pub unsafe fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), pszinstanceidentity, pdwindex as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "servprov")]
    pub unsafe fn Item(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "servprov")]
    pub unsafe fn Add<P0>(&self, pifunctioninstance: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi()) }
    }
    #[cfg(feature = "servprov")]
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
    #[cfg(feature = "servprov")]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "servprov"))]
    Get: usize,
    #[cfg(feature = "servprov")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "servprov"))]
    Item: usize,
    #[cfg(feature = "servprov")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "servprov"))]
    Add: usize,
    #[cfg(feature = "servprov")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "servprov"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "servprov")]
pub trait IFunctionInstanceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<IFunctionInstance>;
    fn Item(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance>;
    fn Add(&self, pifunctioninstance: windows_core::Ref<IFunctionInstance>) -> windows_core::Result<()>;
    fn Remove(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance>;
    fn Delete(&self, dwindex: u32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "servprov")]
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
#[cfg(feature = "servprov")]
impl windows_core::RuntimeName for IFunctionInstanceCollection {}
windows_core::imp::define_interface!(IFunctionInstanceCollectionQuery, IFunctionInstanceCollectionQuery_Vtbl, 0x57cc6fd2_c09a_4289_bb72_25f04142058e);
windows_core::imp::interface_hierarchy!(IFunctionInstanceCollectionQuery, windows_core::IUnknown);
impl IFunctionInstanceCollectionQuery {
    pub unsafe fn AddQueryConstraint(&self, pszconstraintname: *const u16, pszconstraintvalue: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddQueryConstraint)(windows_core::Interface::as_raw(self), pszconstraintname, pszconstraintvalue) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub AddPropertyConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *const super::propidlbase::PROPVARIANT, PropertyConstraint) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    AddPropertyConstraint: usize,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFunctionInstanceCollectionQuery_Impl: windows_core::IUnknownImpl {
    fn AddQueryConstraint(&self, pszconstraintname: *const u16, pszconstraintvalue: *const u16) -> windows_core::Result<()>;
    fn AddPropertyConstraint(&self, key: *const super::wtypes::PROPERTYKEY, pv: *const super::propidlbase::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::Result<()>;
    fn Execute(&self) -> windows_core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "servprov")]
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
    #[cfg(feature = "propsys")]
    pub unsafe fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), pszinstanceidentity, pdwindex as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn Item(&self, dwindex: u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn Add<P0>(&self, pipropertystore: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::propsys::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pipropertystore.param().abi()) }
    }
    #[cfg(feature = "propsys")]
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
    #[cfg(feature = "propsys")]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    Get: usize,
    #[cfg(feature = "propsys")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    Item: usize,
    #[cfg(feature = "propsys")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    Add: usize,
    #[cfg(feature = "propsys")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "propsys")]
pub trait IPropertyStoreCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn Item(&self, dwindex: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn Add(&self, pipropertystore: windows_core::Ref<super::propsys::IPropertyStore>) -> windows_core::Result<()>;
    fn Remove(&self, dwindex: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn Delete(&self, dwindex: u32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "propsys")]
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
#[cfg(feature = "propsys")]
impl windows_core::RuntimeName for IPropertyStoreCollection {}
pub const MAX_FDCONSTRAINTNAME_LENGTH: u32 = 100;
pub const MAX_FDCONSTRAINTVALUE_LENGTH: u32 = 1000;
pub const ONLINE_PROVIDER_DEVICES_QUERYCONSTRAINT_OWNERNAME: windows_core::PCWSTR = windows_core::w!("OwnerName");
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_Characteristics: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 29 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_ClassCoInstallers: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x713d1703_a2e2_49f5_9214_56472ef3da5c), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_ClassInstaller: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 5 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_ClassName: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_DefaultService: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 11 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_DevType: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 27 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_Exclusive: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 28 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_Icon: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_IconPath: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 12 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_LowerFilters: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 20 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_Name: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_NoDisplayClass: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 8 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_NoInstallClass: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 7 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_NoUseClass: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 10 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_PropPageProvider: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 6 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_Security: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 25 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_SecuritySDS: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 26 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_SilentInstall: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 9 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceClass_UpperFilters: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 19 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_Address: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 51 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_AlwaysShowDeviceAsConnected: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 101 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_AssociationArray: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 80 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_BaselineExperienceId: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 78 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_Category: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 90 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_CategoryGroup_Desc: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 94 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_CategoryGroup_Icon: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 95 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_Category_Desc_Plural: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 92 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_Category_Desc_Singular: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 91 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_Category_Icon: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 93 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_DeviceDescription1: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 81 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_DeviceDescription2: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 82 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_DeviceFunctionSubRank: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 100 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_DiscoveryMethod: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 52 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_ExperienceId: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 89 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_FriendlyName: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12288 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_Icon: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 57 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_InstallInProgress: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29), pid: 9 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsAuthenticated: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 54 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsConnected: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 55 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsDefaultDevice: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 86 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsDeviceUniquelyIdentifiable: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 79 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsEncrypted: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 53 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsLocalMachine: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 70 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsMetadataSearchInProgress: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 72 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsNetworkDevice: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 85 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsNotInterestingForDisplay: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 74 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsNotWorkingProperly: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 83 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsPaired: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 56 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsSharedDevice: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 84 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_IsShowInDisconnectedState: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 68 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_Last_Connected: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 67 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_Last_Seen: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 66 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_LaunchDeviceStageFromExplorer: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 77 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_LaunchDeviceStageOnDeviceConnect: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 76 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_Manufacturer: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8192 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_MetadataCabinet: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 87 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_MetadataChecksum: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 73 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_MetadataPath: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 71 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_ModelName: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8194 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_ModelNumber: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8195 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_PrimaryCategory: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 97 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_RequiresPairingElevation: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 88 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_RequiresUninstallElevation: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 99 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_UnpairUninstall: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 98 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceDisplay_Version: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 65 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceInterfaceClass_DefaultInterface: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x14c83a99_0b3f_44b7_be4c_a178d3990564), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceInterface_ClassGuid: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceInterface_Enabled: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_DeviceInterface_FriendlyName: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_AdditionalSoftwareRequested: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 19 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Address: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 30 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_BIOSVersion: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xeaee7f1d_6a33_44d1_9441_5f46def23198), pid: 9 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_BaseContainerId: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 38 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_BusNumber: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 23 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_BusRelations: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 7 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_BusReportedDeviceDesc: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_BusTypeGuid: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 21 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Capabilities: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 17 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Characteristics: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 29 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Children: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 9 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Class: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 9 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_ClassGuid: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 10 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_CompatibleIds: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_ConfigFlags: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 12 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_ContainerId: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8c7ed206_3f8a_4827_b3ab_ae9e1faefc6c), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DHP_Rebalance_Policy: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DevNodeStatus: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DevType: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 27 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DeviceDesc: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Driver: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 11 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DriverCoInstallers: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 11 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DriverDate: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DriverDesc: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DriverInfPath: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 5 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DriverInfSection: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 6 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DriverInfSectionExt: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 7 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DriverLogoLevel: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 15 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DriverPropPageProvider: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 10 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DriverProvider: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 9 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DriverRank: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 14 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_DriverVersion: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_EjectionRelations: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_EnumeratorName: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 24 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Exclusive: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 28 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_FriendlyName: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 14 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_FriendlyNameAttributes: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_GenericDriverInstalled: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 18 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_HardwareIds: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_InstallInProgress: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29), pid: 9 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_InstallState: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 36 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_InstanceId: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 256 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_IsAssociateableByUserAction: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 7 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Legacy: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_LegacyBusType: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 22 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_LocationInfo: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 15 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_LocationPaths: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 37 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_LowerFilters: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 20 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Manufacturer: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 13 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_ManufacturerAttributes: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_MatchingDeviceId: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 8 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_ModelId: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_NoConnectSound: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 17 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Numa_Node: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_PDOName: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 16 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Parent: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 8 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_PowerData: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 32 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_PowerRelations: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 6 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_PresenceNotForDevice: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 5 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_ProblemCode: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_RemovalPolicy: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 33 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_RemovalPolicyDefault: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 34 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_RemovalPolicyOverride: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 35 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_RemovalRelations: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 5 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Reported: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_ResourcePickerExceptions: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 13 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_ResourcePickerTags: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 12 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_SafeRemovalRequired: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_SafeRemovalRequiredOverride: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Security: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 25 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_SecuritySDS: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 26 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Service: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 6 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_Siblings: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 10 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_SignalStrength: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 6 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_TransportRelations: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 11 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_UINumber: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 18 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_UINumberDescFormat: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 31 };
#[cfg(feature = "wtypes")]
pub const PKEY_Device_UpperFilters: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 19 };
#[cfg(feature = "wtypes")]
pub const PKEY_DrvPkg_BrandingIcon: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 7 };
#[cfg(feature = "wtypes")]
pub const PKEY_DrvPkg_DetailedDescription: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_DrvPkg_DocumentationLink: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 5 };
#[cfg(feature = "wtypes")]
pub const PKEY_DrvPkg_Icon: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 6 };
#[cfg(feature = "wtypes")]
pub const PKEY_DrvPkg_Model: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_DrvPkg_VendorWebSite: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_FunctionInstance: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x08c0c253_a154_4746_9005_82de5317148b), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_Devinst: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4097 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_DisplayAttribute: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 5 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_DriverDate: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 11 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_DriverProvider: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 10 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_DriverVersion: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 9 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_Function: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4099 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_Icon: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_Image: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4098 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_Manufacturer: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 6 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_Model: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 7 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_Name: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_SerialNumber: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 8 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_ShellAttributes: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4100 };
#[cfg(feature = "wtypes")]
pub const PKEY_Hardware_Status: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4096 };
#[cfg(feature = "wtypes")]
pub const PKEY_NAME: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 10 };
#[cfg(feature = "wtypes")]
pub const PKEY_Numa_Proximity_Domain: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_Associated: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_Category_Desc_NonPlural: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12304 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_CompactSignature: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28674 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_CompatibleTypes: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_DeviceCategory: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12292 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_DeviceCategory_Desc: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12293 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_DeviceCertHash: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28675 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_DomainName: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 20480 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_FirmwareVersion: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12289 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_GlobalIdentity: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4096 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_ID: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4101 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_IPBusEnumerated: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28688 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_InstallState: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_Installable: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_IpAddress: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12297 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_ManufacturerUrl: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8193 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_MetadataVersion: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4100 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_ModelUrl: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8196 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_NetworkInterfaceGuid: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12296 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_NetworkInterfaceLuid: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12295 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_PhysicalAddress: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12294 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_PresentationUrl: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8198 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_RemoteAddress: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4102 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_Removable: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28672 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_RootProxy: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4103 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_Scopes: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4098 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_SecureChannel: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28673 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_SerialNumber: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12290 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_ServiceAddress: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16384 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_ServiceControlUrl: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16388 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_ServiceDescUrl: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16389 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_ServiceEventSubUrl: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16390 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_ServiceId: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16385 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_ServiceTypes: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16386 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_ShareName: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 20482 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_Types: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4097 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_Upc: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8197 };
#[cfg(feature = "wtypes")]
pub const PKEY_PNPX_XAddrs: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4099 };
#[cfg(feature = "wtypes")]
pub const PKEY_Pairing_IsWifiOnlyDevice: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 16 };
#[cfg(feature = "wtypes")]
pub const PKEY_Pairing_ListItemDefault: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_Pairing_ListItemDescription: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_Pairing_ListItemIcon: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_Pairing_ListItemText: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_SSDP_AltLocationInfo: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24576 };
#[cfg(feature = "wtypes")]
pub const PKEY_SSDP_DevLifeTime: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24577 };
#[cfg(feature = "wtypes")]
pub const PKEY_SSDP_NetworkInterface: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24578 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_AssocState: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b88_4684_11da_a26a_0002b3988e81), pid: 9 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_AuthType: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b82_4684_11da_a26a_0002b3988e81), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_ConfigError: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 10 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_ConfigMethods: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b85_4684_11da_a26a_0002b3988e81), pid: 6 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_ConfigState: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 11 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_ConnType: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b84_4684_11da_a26a_0002b3988e81), pid: 5 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_DevicePasswordId: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 12 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_EncryptType: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b83_4684_11da_a26a_0002b3988e81), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_OSVersion: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 13 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_RegistrarType: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b8b_4684_11da_a26a_0002b3988e81), pid: 15 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_RequestType: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b81_4684_11da_a26a_0002b3988e81), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_RfBand: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b87_4684_11da_a26a_0002b3988e81), pid: 8 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_VendorExtension: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b8a_4684_11da_a26a_0002b3988e81), pid: 14 };
#[cfg(feature = "wtypes")]
pub const PKEY_WCN_Version: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b80_4684_11da_a26a_0002b3988e81), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_WNET_Comment: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 7 };
#[cfg(feature = "wtypes")]
pub const PKEY_WNET_DisplayType: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_WNET_LocalName: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 5 };
#[cfg(feature = "wtypes")]
pub const PKEY_WNET_Provider: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 8 };
#[cfg(feature = "wtypes")]
pub const PKEY_WNET_RemoteName: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 6 };
#[cfg(feature = "wtypes")]
pub const PKEY_WNET_Scope: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_WNET_Type: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_WNET_Usage: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 4 };
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
