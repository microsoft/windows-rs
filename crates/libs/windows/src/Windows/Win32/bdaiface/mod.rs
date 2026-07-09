pub const Associated: SmartCardAssociationType = 1;
pub const AssociationUnknown: SmartCardAssociationType = 2;
pub type BDA_DrmPairingError = i32;
pub const BDA_DrmPairing_Aborted: BDA_DrmPairingError = 8;
pub const BDA_DrmPairing_DrmInitFailed: BDA_DrmPairingError = 5;
pub const BDA_DrmPairing_DrmNotPaired: BDA_DrmPairingError = 6;
pub const BDA_DrmPairing_DrmRePairSoon: BDA_DrmPairingError = 7;
pub const BDA_DrmPairing_HardwareFailure: BDA_DrmPairingError = 1;
pub const BDA_DrmPairing_NeedIndiv: BDA_DrmPairingError = 3;
pub const BDA_DrmPairing_NeedRevocationData: BDA_DrmPairingError = 2;
pub const BDA_DrmPairing_NeedSDKUpdate: BDA_DrmPairingError = 9;
pub const BDA_DrmPairing_Other: BDA_DrmPairingError = 4;
pub const BDA_DrmPairing_Succeeded: BDA_DrmPairingError = 0;
pub const CLSID_BroadcastEventService: windows_core::GUID = windows_core::GUID::from_u128(0x0b3ffb92_0919_4934_9d5b_619c719d0202);
pub const CLSID_PBDA_AUX_DATA_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xfd456373_3323_4090_adca_8ed45f55cf10);
pub const CLSID_PBDA_Encoder_DATA_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0x728fd6bc_5546_4716_b103_f899f5a1fa68);
pub const CLSID_PBDA_FDC_DATA_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xe7dbf9a0_22ab_4047_8e67_ef9ad504e729);
pub const CLSID_PBDA_GDDS_DATA_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xc80c0df3_6052_4c16_9f56_c44c21f73c45);
pub const CardDataChanged: SmartCardStatusType = 3;
pub const CardError: SmartCardStatusType = 2;
pub const CardFirmwareUpgrade: SmartCardStatusType = 4;
pub const CardInserted: SmartCardStatusType = 0;
pub const CardRemoved: SmartCardStatusType = 1;
pub const DeviceClosed: UICloseReasonType = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EALocationCodeType {
    pub LocationCodeScheme: LocationCodeSchemeType,
    pub state_code: u8,
    pub county_subdivision: u8,
    pub county_code: u16,
}
pub const Entitled: EntitlementType = 0;
pub type EntitlementType = i32;
pub const ErrorClosed: UICloseReasonType = 4;
windows_core::imp::define_interface!(IBDA_AUX, IBDA_AUX_Vtbl, 0x7def4c09_6e66_4567_a819_f0e17f4a81ab);
windows_core::imp::interface_hierarchy!(IBDA_AUX, windows_core::IUnknown);
impl IBDA_AUX {
    pub unsafe fn QueryCapabilities(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumCapability(&self, dwindex: u32, dwinputid: *mut u32, pconnectortype: *mut windows_core::GUID, conntypenum: *mut u32, numvideostds: *mut u32, analogstds: *mut u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumCapability)(windows_core::Interface::as_raw(self), dwindex, dwinputid as _, pconnectortype as _, conntypenum as _, numvideostds as _, analogstds as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_AUX_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumCapability: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut windows_core::GUID, *mut u32, *mut u32, *mut u64) -> windows_core::HRESULT,
}
pub trait IBDA_AUX_Impl: windows_core::IUnknownImpl {
    fn QueryCapabilities(&self) -> windows_core::Result<u32>;
    fn EnumCapability(&self, dwindex: u32, dwinputid: *mut u32, pconnectortype: *mut windows_core::GUID, conntypenum: *mut u32, numvideostds: *mut u32, analogstds: *mut u64) -> windows_core::Result<()>;
}
impl IBDA_AUX_Vtbl {
    pub const fn new<Identity: IBDA_AUX_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryCapabilities<Identity: IBDA_AUX_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwnumauxinputsbstr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_AUX_Impl::QueryCapabilities(this) {
                    Ok(ok__) => {
                        pdwnumauxinputsbstr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumCapability<Identity: IBDA_AUX_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, dwinputid: *mut u32, pconnectortype: *mut windows_core::GUID, conntypenum: *mut u32, numvideostds: *mut u32, analogstds: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_AUX_Impl::EnumCapability(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwinputid), core::mem::transmute_copy(&pconnectortype), core::mem::transmute_copy(&conntypenum), core::mem::transmute_copy(&numvideostds), core::mem::transmute_copy(&analogstds)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryCapabilities: QueryCapabilities::<Identity, OFFSET>,
            EnumCapability: EnumCapability::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_AUX as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_AUX {}
windows_core::imp::define_interface!(IBDA_AutoDemodulate, IBDA_AutoDemodulate_Vtbl, 0xddf15b12_bd25_11d2_9ca0_00c04f7971e0);
windows_core::imp::interface_hierarchy!(IBDA_AutoDemodulate, windows_core::IUnknown);
impl IBDA_AutoDemodulate {
    pub unsafe fn put_AutoDemodulate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AutoDemodulate)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_AutoDemodulate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub put_AutoDemodulate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBDA_AutoDemodulate_Impl: windows_core::IUnknownImpl {
    fn put_AutoDemodulate(&self) -> windows_core::Result<()>;
}
impl IBDA_AutoDemodulate_Vtbl {
    pub const fn new<Identity: IBDA_AutoDemodulate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_AutoDemodulate<Identity: IBDA_AutoDemodulate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_AutoDemodulate_Impl::put_AutoDemodulate(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), put_AutoDemodulate: put_AutoDemodulate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_AutoDemodulate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_AutoDemodulate {}
windows_core::imp::define_interface!(IBDA_AutoDemodulateEx, IBDA_AutoDemodulateEx_Vtbl, 0x34518d13_1182_48e6_b28f_b24987787326);
impl core::ops::Deref for IBDA_AutoDemodulateEx {
    type Target = IBDA_AutoDemodulate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBDA_AutoDemodulateEx, windows_core::IUnknown, IBDA_AutoDemodulate);
impl IBDA_AutoDemodulateEx {
    pub unsafe fn get_SupportedDeviceNodeTypes(&self, ulcdevicenodetypesmax: u32, pulcdevicenodetypes: *mut u32, pguiddevicenodetypes: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SupportedDeviceNodeTypes)(windows_core::Interface::as_raw(self), ulcdevicenodetypesmax, pulcdevicenodetypes as _, pguiddevicenodetypes as _) }
    }
    pub unsafe fn get_SupportedVideoFormats(&self, pulamtunermodetype: *mut u32, pulanalogvideostandard: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SupportedVideoFormats)(windows_core::Interface::as_raw(self), pulamtunermodetype as _, pulanalogvideostandard as _) }
    }
    pub unsafe fn get_AuxInputCount(&self, pulcompositecount: *mut u32, pulsvideocount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_AuxInputCount)(windows_core::Interface::as_raw(self), pulcompositecount as _, pulsvideocount as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_AutoDemodulateEx_Vtbl {
    pub base__: IBDA_AutoDemodulate_Vtbl,
    pub get_SupportedDeviceNodeTypes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub get_SupportedVideoFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub get_AuxInputCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IBDA_AutoDemodulateEx_Impl: IBDA_AutoDemodulate_Impl {
    fn get_SupportedDeviceNodeTypes(&self, ulcdevicenodetypesmax: u32, pulcdevicenodetypes: *mut u32, pguiddevicenodetypes: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn get_SupportedVideoFormats(&self, pulamtunermodetype: *mut u32, pulanalogvideostandard: *mut u32) -> windows_core::Result<()>;
    fn get_AuxInputCount(&self, pulcompositecount: *mut u32, pulsvideocount: *mut u32) -> windows_core::Result<()>;
}
impl IBDA_AutoDemodulateEx_Vtbl {
    pub const fn new<Identity: IBDA_AutoDemodulateEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_SupportedDeviceNodeTypes<Identity: IBDA_AutoDemodulateEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcdevicenodetypesmax: u32, pulcdevicenodetypes: *mut u32, pguiddevicenodetypes: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_AutoDemodulateEx_Impl::get_SupportedDeviceNodeTypes(this, core::mem::transmute_copy(&ulcdevicenodetypesmax), core::mem::transmute_copy(&pulcdevicenodetypes), core::mem::transmute_copy(&pguiddevicenodetypes)).into()
            }
        }
        unsafe extern "system" fn get_SupportedVideoFormats<Identity: IBDA_AutoDemodulateEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulamtunermodetype: *mut u32, pulanalogvideostandard: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_AutoDemodulateEx_Impl::get_SupportedVideoFormats(this, core::mem::transmute_copy(&pulamtunermodetype), core::mem::transmute_copy(&pulanalogvideostandard)).into()
            }
        }
        unsafe extern "system" fn get_AuxInputCount<Identity: IBDA_AutoDemodulateEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcompositecount: *mut u32, pulsvideocount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_AutoDemodulateEx_Impl::get_AuxInputCount(this, core::mem::transmute_copy(&pulcompositecount), core::mem::transmute_copy(&pulsvideocount)).into()
            }
        }
        Self {
            base__: IBDA_AutoDemodulate_Vtbl::new::<Identity, OFFSET>(),
            get_SupportedDeviceNodeTypes: get_SupportedDeviceNodeTypes::<Identity, OFFSET>,
            get_SupportedVideoFormats: get_SupportedVideoFormats::<Identity, OFFSET>,
            get_AuxInputCount: get_AuxInputCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_AutoDemodulateEx as windows_core::Interface>::IID || iid == &<IBDA_AutoDemodulate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_AutoDemodulateEx {}
windows_core::imp::define_interface!(IBDA_ConditionalAccess, IBDA_ConditionalAccess_Vtbl, 0xcd51f1e0_7be9_4123_8482_a2a796c0a6b0);
windows_core::imp::interface_hierarchy!(IBDA_ConditionalAccess, windows_core::IUnknown);
impl IBDA_ConditionalAccess {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn get_SmartCardStatus(&self, pcardstatus: *mut SmartCardStatusType, pcardassociation: *mut SmartCardAssociationType, pbstrcarderror: *mut windows_core::BSTR, pfooblocked: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SmartCardStatus)(windows_core::Interface::as_raw(self), pcardstatus as _, pcardassociation as _, core::mem::transmute(pbstrcarderror), pfooblocked as _) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn get_SmartCardInfo(&self, pbstrcardname: *mut windows_core::BSTR, pbstrcardmanufacturer: *mut windows_core::BSTR, pfdaylightsavings: *mut super::wtypes::VARIANT_BOOL, pbyratingregion: *mut u8, pltimezoneoffsetminutes: *mut i32, pbstrlanguage: *mut windows_core::BSTR, pealocationcode: *mut EALocationCodeType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SmartCardInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrcardname), core::mem::transmute(pbstrcardmanufacturer), pfdaylightsavings as _, pbyratingregion as _, pltimezoneoffsetminutes as _, core::mem::transmute(pbstrlanguage), pealocationcode as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_SmartCardApplications(&self, pulcapplications: *mut u32, ulcapplicationsmax: u32, rgapplications: *mut SmartCardApplication) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SmartCardApplications)(windows_core::Interface::as_raw(self), pulcapplications as _, ulcapplicationsmax, core::mem::transmute(rgapplications)) }
    }
    pub unsafe fn get_Entitlement(&self, usvirtualchannel: u16) -> windows_core::Result<EntitlementType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Entitlement)(windows_core::Interface::as_raw(self), usvirtualchannel, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TuneByChannel(&self, usvirtualchannel: u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TuneByChannel)(windows_core::Interface::as_raw(self), usvirtualchannel) }
    }
    pub unsafe fn SetProgram(&self, usprogramnumber: u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProgram)(windows_core::Interface::as_raw(self), usprogramnumber) }
    }
    pub unsafe fn AddProgram(&self, usprogramnumber: u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddProgram)(windows_core::Interface::as_raw(self), usprogramnumber) }
    }
    pub unsafe fn RemoveProgram(&self, usprogramnumber: u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveProgram)(windows_core::Interface::as_raw(self), usprogramnumber) }
    }
    pub unsafe fn GetModuleUI(&self, bydialognumber: u8) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModuleUI)(windows_core::Interface::as_raw(self), bydialognumber, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InformUIClosed(&self, bydialognumber: u8, closereason: UICloseReasonType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InformUIClosed)(windows_core::Interface::as_raw(self), bydialognumber, closereason) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_ConditionalAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub get_SmartCardStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardStatusType, *mut SmartCardAssociationType, *mut *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    get_SmartCardStatus: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub get_SmartCardInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL, *mut u8, *mut i32, *mut *mut core::ffi::c_void, *mut EALocationCodeType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    get_SmartCardInfo: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_SmartCardApplications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut SmartCardApplication) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_SmartCardApplications: usize,
    pub get_Entitlement: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut EntitlementType) -> windows_core::HRESULT,
    pub TuneByChannel: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub SetProgram: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub AddProgram: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub RemoveProgram: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetModuleUI: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InformUIClosed: unsafe extern "system" fn(*mut core::ffi::c_void, u8, UICloseReasonType) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bdatypes", feature = "Win32_wtypes"))]
pub trait IBDA_ConditionalAccess_Impl: windows_core::IUnknownImpl {
    fn get_SmartCardStatus(&self, pcardstatus: *mut SmartCardStatusType, pcardassociation: *mut SmartCardAssociationType, pbstrcarderror: *mut windows_core::BSTR, pfooblocked: *mut super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_SmartCardInfo(&self, pbstrcardname: *mut windows_core::BSTR, pbstrcardmanufacturer: *mut windows_core::BSTR, pfdaylightsavings: *mut super::wtypes::VARIANT_BOOL, pbyratingregion: *mut u8, pltimezoneoffsetminutes: *mut i32, pbstrlanguage: *mut windows_core::BSTR, pealocationcode: *mut EALocationCodeType) -> windows_core::Result<()>;
    fn get_SmartCardApplications(&self, pulcapplications: *mut u32, ulcapplicationsmax: u32, rgapplications: *mut SmartCardApplication) -> windows_core::Result<()>;
    fn get_Entitlement(&self, usvirtualchannel: u16) -> windows_core::Result<EntitlementType>;
    fn TuneByChannel(&self, usvirtualchannel: u16) -> windows_core::Result<()>;
    fn SetProgram(&self, usprogramnumber: u16) -> windows_core::Result<()>;
    fn AddProgram(&self, usprogramnumber: u16) -> windows_core::Result<()>;
    fn RemoveProgram(&self, usprogramnumber: u16) -> windows_core::Result<()>;
    fn GetModuleUI(&self, bydialognumber: u8) -> windows_core::Result<windows_core::BSTR>;
    fn InformUIClosed(&self, bydialognumber: u8, closereason: UICloseReasonType) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_bdatypes", feature = "Win32_wtypes"))]
impl IBDA_ConditionalAccess_Vtbl {
    pub const fn new<Identity: IBDA_ConditionalAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_SmartCardStatus<Identity: IBDA_ConditionalAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcardstatus: *mut SmartCardStatusType, pcardassociation: *mut SmartCardAssociationType, pbstrcarderror: *mut *mut core::ffi::c_void, pfooblocked: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_ConditionalAccess_Impl::get_SmartCardStatus(this, core::mem::transmute_copy(&pcardstatus), core::mem::transmute_copy(&pcardassociation), core::mem::transmute_copy(&pbstrcarderror), core::mem::transmute_copy(&pfooblocked)).into()
            }
        }
        unsafe extern "system" fn get_SmartCardInfo<Identity: IBDA_ConditionalAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcardname: *mut *mut core::ffi::c_void, pbstrcardmanufacturer: *mut *mut core::ffi::c_void, pfdaylightsavings: *mut super::wtypes::VARIANT_BOOL, pbyratingregion: *mut u8, pltimezoneoffsetminutes: *mut i32, pbstrlanguage: *mut *mut core::ffi::c_void, pealocationcode: *mut EALocationCodeType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_ConditionalAccess_Impl::get_SmartCardInfo(this, core::mem::transmute_copy(&pbstrcardname), core::mem::transmute_copy(&pbstrcardmanufacturer), core::mem::transmute_copy(&pfdaylightsavings), core::mem::transmute_copy(&pbyratingregion), core::mem::transmute_copy(&pltimezoneoffsetminutes), core::mem::transmute_copy(&pbstrlanguage), core::mem::transmute_copy(&pealocationcode)).into()
            }
        }
        unsafe extern "system" fn get_SmartCardApplications<Identity: IBDA_ConditionalAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcapplications: *mut u32, ulcapplicationsmax: u32, rgapplications: *mut SmartCardApplication) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_ConditionalAccess_Impl::get_SmartCardApplications(this, core::mem::transmute_copy(&pulcapplications), core::mem::transmute_copy(&ulcapplicationsmax), core::mem::transmute_copy(&rgapplications)).into()
            }
        }
        unsafe extern "system" fn get_Entitlement<Identity: IBDA_ConditionalAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usvirtualchannel: u16, pentitlement: *mut EntitlementType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_ConditionalAccess_Impl::get_Entitlement(this, core::mem::transmute_copy(&usvirtualchannel)) {
                    Ok(ok__) => {
                        pentitlement.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TuneByChannel<Identity: IBDA_ConditionalAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usvirtualchannel: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_ConditionalAccess_Impl::TuneByChannel(this, core::mem::transmute_copy(&usvirtualchannel)).into()
            }
        }
        unsafe extern "system" fn SetProgram<Identity: IBDA_ConditionalAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usprogramnumber: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_ConditionalAccess_Impl::SetProgram(this, core::mem::transmute_copy(&usprogramnumber)).into()
            }
        }
        unsafe extern "system" fn AddProgram<Identity: IBDA_ConditionalAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usprogramnumber: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_ConditionalAccess_Impl::AddProgram(this, core::mem::transmute_copy(&usprogramnumber)).into()
            }
        }
        unsafe extern "system" fn RemoveProgram<Identity: IBDA_ConditionalAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usprogramnumber: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_ConditionalAccess_Impl::RemoveProgram(this, core::mem::transmute_copy(&usprogramnumber)).into()
            }
        }
        unsafe extern "system" fn GetModuleUI<Identity: IBDA_ConditionalAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bydialognumber: u8, pbstrurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_ConditionalAccess_Impl::GetModuleUI(this, core::mem::transmute_copy(&bydialognumber)) {
                    Ok(ok__) => {
                        pbstrurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InformUIClosed<Identity: IBDA_ConditionalAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bydialognumber: u8, closereason: UICloseReasonType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_ConditionalAccess_Impl::InformUIClosed(this, core::mem::transmute_copy(&bydialognumber), core::mem::transmute_copy(&closereason)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_SmartCardStatus: get_SmartCardStatus::<Identity, OFFSET>,
            get_SmartCardInfo: get_SmartCardInfo::<Identity, OFFSET>,
            get_SmartCardApplications: get_SmartCardApplications::<Identity, OFFSET>,
            get_Entitlement: get_Entitlement::<Identity, OFFSET>,
            TuneByChannel: TuneByChannel::<Identity, OFFSET>,
            SetProgram: SetProgram::<Identity, OFFSET>,
            AddProgram: AddProgram::<Identity, OFFSET>,
            RemoveProgram: RemoveProgram::<Identity, OFFSET>,
            GetModuleUI: GetModuleUI::<Identity, OFFSET>,
            InformUIClosed: InformUIClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_ConditionalAccess as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bdatypes", feature = "Win32_wtypes"))]
impl windows_core::RuntimeName for IBDA_ConditionalAccess {}
windows_core::imp::define_interface!(IBDA_ConditionalAccessEx, IBDA_ConditionalAccessEx_Vtbl, 0x497c3418_23cb_44ba_bb62_769f506fcea7);
windows_core::imp::interface_hierarchy!(IBDA_ConditionalAccessEx, windows_core::IUnknown);
impl IBDA_ConditionalAccessEx {
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn CheckEntitlementToken(&self, uldialogrequest: u32, bstrlanguage: &windows_core::BSTR, requesttype: super::bdatypes::BDA_CONDITIONALACCESS_REQUESTTYPE, ulcbentitlementtokenlen: u32, pbentitlementtoken: *const u8) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckEntitlementToken)(windows_core::Interface::as_raw(self), uldialogrequest, core::mem::transmute_copy(bstrlanguage), requesttype, ulcbentitlementtokenlen, pbentitlementtoken, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCaptureToken(&self, ulcbcapturetokenlen: u32, pbcapturetoken: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCaptureToken)(windows_core::Interface::as_raw(self), ulcbcapturetokenlen, pbcapturetoken) }
    }
    pub unsafe fn OpenBroadcastMmi(&self, uldialogrequest: u32, bstrlanguage: &windows_core::BSTR, eventid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenBroadcastMmi)(windows_core::Interface::as_raw(self), uldialogrequest, core::mem::transmute_copy(bstrlanguage), eventid) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn CloseMmiDialog(&self, uldialogrequest: u32, bstrlanguage: &windows_core::BSTR, uldialognumber: u32, reasoncode: super::bdatypes::BDA_CONDITIONALACCESS_MMICLOSEREASON) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloseMmiDialog)(windows_core::Interface::as_raw(self), uldialogrequest, core::mem::transmute_copy(bstrlanguage), uldialognumber, reasoncode, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateDialogRequestNumber(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDialogRequestNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_ConditionalAccessEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_bdatypes")]
    pub CheckEntitlementToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, super::bdatypes::BDA_CONDITIONALACCESS_REQUESTTYPE, u32, *const u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    CheckEntitlementToken: usize,
    pub SetCaptureToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub OpenBroadcastMmi: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_bdatypes")]
    pub CloseMmiDialog: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, super::bdatypes::BDA_CONDITIONALACCESS_MMICLOSEREASON, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    CloseMmiDialog: usize,
    pub CreateDialogRequestNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_bdatypes")]
pub trait IBDA_ConditionalAccessEx_Impl: windows_core::IUnknownImpl {
    fn CheckEntitlementToken(&self, uldialogrequest: u32, bstrlanguage: &windows_core::BSTR, requesttype: super::bdatypes::BDA_CONDITIONALACCESS_REQUESTTYPE, ulcbentitlementtokenlen: u32, pbentitlementtoken: *const u8) -> windows_core::Result<u32>;
    fn SetCaptureToken(&self, ulcbcapturetokenlen: u32, pbcapturetoken: *const u8) -> windows_core::Result<()>;
    fn OpenBroadcastMmi(&self, uldialogrequest: u32, bstrlanguage: &windows_core::BSTR, eventid: u32) -> windows_core::Result<()>;
    fn CloseMmiDialog(&self, uldialogrequest: u32, bstrlanguage: &windows_core::BSTR, uldialognumber: u32, reasoncode: super::bdatypes::BDA_CONDITIONALACCESS_MMICLOSEREASON) -> windows_core::Result<u32>;
    fn CreateDialogRequestNumber(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_bdatypes")]
impl IBDA_ConditionalAccessEx_Vtbl {
    pub const fn new<Identity: IBDA_ConditionalAccessEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckEntitlementToken<Identity: IBDA_ConditionalAccessEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uldialogrequest: u32, bstrlanguage: *mut core::ffi::c_void, requesttype: super::bdatypes::BDA_CONDITIONALACCESS_REQUESTTYPE, ulcbentitlementtokenlen: u32, pbentitlementtoken: *const u8, puldescramblestatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_ConditionalAccessEx_Impl::CheckEntitlementToken(this, core::mem::transmute_copy(&uldialogrequest), core::mem::transmute(&bstrlanguage), core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&ulcbentitlementtokenlen), core::mem::transmute_copy(&pbentitlementtoken)) {
                    Ok(ok__) => {
                        puldescramblestatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCaptureToken<Identity: IBDA_ConditionalAccessEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcbcapturetokenlen: u32, pbcapturetoken: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_ConditionalAccessEx_Impl::SetCaptureToken(this, core::mem::transmute_copy(&ulcbcapturetokenlen), core::mem::transmute_copy(&pbcapturetoken)).into()
            }
        }
        unsafe extern "system" fn OpenBroadcastMmi<Identity: IBDA_ConditionalAccessEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uldialogrequest: u32, bstrlanguage: *mut core::ffi::c_void, eventid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_ConditionalAccessEx_Impl::OpenBroadcastMmi(this, core::mem::transmute_copy(&uldialogrequest), core::mem::transmute(&bstrlanguage), core::mem::transmute_copy(&eventid)).into()
            }
        }
        unsafe extern "system" fn CloseMmiDialog<Identity: IBDA_ConditionalAccessEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uldialogrequest: u32, bstrlanguage: *mut core::ffi::c_void, uldialognumber: u32, reasoncode: super::bdatypes::BDA_CONDITIONALACCESS_MMICLOSEREASON, pulsessionresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_ConditionalAccessEx_Impl::CloseMmiDialog(this, core::mem::transmute_copy(&uldialogrequest), core::mem::transmute(&bstrlanguage), core::mem::transmute_copy(&uldialognumber), core::mem::transmute_copy(&reasoncode)) {
                    Ok(ok__) => {
                        pulsessionresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDialogRequestNumber<Identity: IBDA_ConditionalAccessEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puldialogrequestnumber: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_ConditionalAccessEx_Impl::CreateDialogRequestNumber(this) {
                    Ok(ok__) => {
                        puldialogrequestnumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CheckEntitlementToken: CheckEntitlementToken::<Identity, OFFSET>,
            SetCaptureToken: SetCaptureToken::<Identity, OFFSET>,
            OpenBroadcastMmi: OpenBroadcastMmi::<Identity, OFFSET>,
            CloseMmiDialog: CloseMmiDialog::<Identity, OFFSET>,
            CreateDialogRequestNumber: CreateDialogRequestNumber::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_ConditionalAccessEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_bdatypes")]
impl windows_core::RuntimeName for IBDA_ConditionalAccessEx {}
windows_core::imp::define_interface!(IBDA_DRIDRMService, IBDA_DRIDRMService_Vtbl, 0x1f9bc2a5_44a3_4c52_aab1_0bbce5a1381d);
windows_core::imp::interface_hierarchy!(IBDA_DRIDRMService, windows_core::IUnknown);
impl IBDA_DRIDRMService {
    pub unsafe fn SetDRM(&self, bstrnewdrm: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDRM)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnewdrm)) }
    }
    pub unsafe fn GetDRMStatus(&self, pbstrdrmuuidlist: *mut windows_core::BSTR, drmuuid: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDRMStatus)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrdrmuuidlist), drmuuid as _) }
    }
    pub unsafe fn GetPairingStatus(&self, penumpairingstatus: *mut BDA_DrmPairingError) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPairingStatus)(windows_core::Interface::as_raw(self), penumpairingstatus as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_DRIDRMService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDRM: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDRMStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetPairingStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BDA_DrmPairingError) -> windows_core::HRESULT,
}
pub trait IBDA_DRIDRMService_Impl: windows_core::IUnknownImpl {
    fn SetDRM(&self, bstrnewdrm: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDRMStatus(&self, pbstrdrmuuidlist: *mut windows_core::BSTR, drmuuid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetPairingStatus(&self, penumpairingstatus: *mut BDA_DrmPairingError) -> windows_core::Result<()>;
}
impl IBDA_DRIDRMService_Vtbl {
    pub const fn new<Identity: IBDA_DRIDRMService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDRM<Identity: IBDA_DRIDRMService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnewdrm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRIDRMService_Impl::SetDRM(this, core::mem::transmute(&bstrnewdrm)).into()
            }
        }
        unsafe extern "system" fn GetDRMStatus<Identity: IBDA_DRIDRMService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdrmuuidlist: *mut *mut core::ffi::c_void, drmuuid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRIDRMService_Impl::GetDRMStatus(this, core::mem::transmute_copy(&pbstrdrmuuidlist), core::mem::transmute_copy(&drmuuid)).into()
            }
        }
        unsafe extern "system" fn GetPairingStatus<Identity: IBDA_DRIDRMService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penumpairingstatus: *mut BDA_DrmPairingError) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRIDRMService_Impl::GetPairingStatus(this, core::mem::transmute_copy(&penumpairingstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDRM: SetDRM::<Identity, OFFSET>,
            GetDRMStatus: GetDRMStatus::<Identity, OFFSET>,
            GetPairingStatus: GetPairingStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_DRIDRMService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_DRIDRMService {}
windows_core::imp::define_interface!(IBDA_DRIWMDRMSession, IBDA_DRIWMDRMSession_Vtbl, 0x05c690f8_56db_4bb2_b053_79c12098bb26);
windows_core::imp::interface_hierarchy!(IBDA_DRIWMDRMSession, windows_core::IUnknown);
impl IBDA_DRIWMDRMSession {
    pub unsafe fn AcknowledgeLicense(&self, hrlicenseack: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcknowledgeLicense)(windows_core::Interface::as_raw(self), hrlicenseack) }
    }
    pub unsafe fn ProcessLicenseChallenge(&self, dwcblicensemessage: u32, pblicensemessage: *const u8, pdwcblicenseresponse: *mut u32, ppblicenseresponse: *mut *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessLicenseChallenge)(windows_core::Interface::as_raw(self), dwcblicensemessage, pblicensemessage, pdwcblicenseresponse as _, ppblicenseresponse as _) }
    }
    pub unsafe fn ProcessRegistrationChallenge(&self, dwcbregistrationmessage: u32, pbregistrationmessage: *const u8, pdwcbregistrationresponse: *mut u32, ppbregistrationresponse: *mut *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessRegistrationChallenge)(windows_core::Interface::as_raw(self), dwcbregistrationmessage, pbregistrationmessage, pdwcbregistrationresponse as _, ppbregistrationresponse as _) }
    }
    pub unsafe fn SetRevInfo(&self, dwrevinfolen: u32, pbrevinfo: *const u8, pdwresponse: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRevInfo)(windows_core::Interface::as_raw(self), dwrevinfolen, pbrevinfo, pdwresponse as _) }
    }
    pub unsafe fn SetCrl(&self, dwcrllen: u32, pbcrllen: *const u8, pdwresponse: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCrl)(windows_core::Interface::as_raw(self), dwcrllen, pbcrllen, pdwresponse as _) }
    }
    pub unsafe fn GetHMSAssociationData(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHMSAssociationData)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLastCardeaError(&self, pdwerror: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLastCardeaError)(windows_core::Interface::as_raw(self), pdwerror as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_DRIWMDRMSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AcknowledgeLicense: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ProcessLicenseChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub ProcessRegistrationChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetRevInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut u32) -> windows_core::HRESULT,
    pub SetCrl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut u32) -> windows_core::HRESULT,
    pub GetHMSAssociationData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastCardeaError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IBDA_DRIWMDRMSession_Impl: windows_core::IUnknownImpl {
    fn AcknowledgeLicense(&self, hrlicenseack: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ProcessLicenseChallenge(&self, dwcblicensemessage: u32, pblicensemessage: *const u8, pdwcblicenseresponse: *mut u32, ppblicenseresponse: *mut *mut u8) -> windows_core::Result<()>;
    fn ProcessRegistrationChallenge(&self, dwcbregistrationmessage: u32, pbregistrationmessage: *const u8, pdwcbregistrationresponse: *mut u32, ppbregistrationresponse: *mut *mut u8) -> windows_core::Result<()>;
    fn SetRevInfo(&self, dwrevinfolen: u32, pbrevinfo: *const u8, pdwresponse: *mut u32) -> windows_core::Result<()>;
    fn SetCrl(&self, dwcrllen: u32, pbcrllen: *const u8, pdwresponse: *mut u32) -> windows_core::Result<()>;
    fn GetHMSAssociationData(&self) -> windows_core::Result<()>;
    fn GetLastCardeaError(&self, pdwerror: *mut u32) -> windows_core::Result<()>;
}
impl IBDA_DRIWMDRMSession_Vtbl {
    pub const fn new<Identity: IBDA_DRIWMDRMSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AcknowledgeLicense<Identity: IBDA_DRIWMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrlicenseack: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRIWMDRMSession_Impl::AcknowledgeLicense(this, core::mem::transmute_copy(&hrlicenseack)).into()
            }
        }
        unsafe extern "system" fn ProcessLicenseChallenge<Identity: IBDA_DRIWMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcblicensemessage: u32, pblicensemessage: *const u8, pdwcblicenseresponse: *mut u32, ppblicenseresponse: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRIWMDRMSession_Impl::ProcessLicenseChallenge(this, core::mem::transmute_copy(&dwcblicensemessage), core::mem::transmute_copy(&pblicensemessage), core::mem::transmute_copy(&pdwcblicenseresponse), core::mem::transmute_copy(&ppblicenseresponse)).into()
            }
        }
        unsafe extern "system" fn ProcessRegistrationChallenge<Identity: IBDA_DRIWMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcbregistrationmessage: u32, pbregistrationmessage: *const u8, pdwcbregistrationresponse: *mut u32, ppbregistrationresponse: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRIWMDRMSession_Impl::ProcessRegistrationChallenge(this, core::mem::transmute_copy(&dwcbregistrationmessage), core::mem::transmute_copy(&pbregistrationmessage), core::mem::transmute_copy(&pdwcbregistrationresponse), core::mem::transmute_copy(&ppbregistrationresponse)).into()
            }
        }
        unsafe extern "system" fn SetRevInfo<Identity: IBDA_DRIWMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrevinfolen: u32, pbrevinfo: *const u8, pdwresponse: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRIWMDRMSession_Impl::SetRevInfo(this, core::mem::transmute_copy(&dwrevinfolen), core::mem::transmute_copy(&pbrevinfo), core::mem::transmute_copy(&pdwresponse)).into()
            }
        }
        unsafe extern "system" fn SetCrl<Identity: IBDA_DRIWMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcrllen: u32, pbcrllen: *const u8, pdwresponse: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRIWMDRMSession_Impl::SetCrl(this, core::mem::transmute_copy(&dwcrllen), core::mem::transmute_copy(&pbcrllen), core::mem::transmute_copy(&pdwresponse)).into()
            }
        }
        unsafe extern "system" fn GetHMSAssociationData<Identity: IBDA_DRIWMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRIWMDRMSession_Impl::GetHMSAssociationData(this).into()
            }
        }
        unsafe extern "system" fn GetLastCardeaError<Identity: IBDA_DRIWMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwerror: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRIWMDRMSession_Impl::GetLastCardeaError(this, core::mem::transmute_copy(&pdwerror)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcknowledgeLicense: AcknowledgeLicense::<Identity, OFFSET>,
            ProcessLicenseChallenge: ProcessLicenseChallenge::<Identity, OFFSET>,
            ProcessRegistrationChallenge: ProcessRegistrationChallenge::<Identity, OFFSET>,
            SetRevInfo: SetRevInfo::<Identity, OFFSET>,
            SetCrl: SetCrl::<Identity, OFFSET>,
            GetHMSAssociationData: GetHMSAssociationData::<Identity, OFFSET>,
            GetLastCardeaError: GetLastCardeaError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_DRIWMDRMSession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_DRIWMDRMSession {}
windows_core::imp::define_interface!(IBDA_DRM, IBDA_DRM_Vtbl, 0xf98d88b0_1992_4cd6_a6d9_b9afab99330d);
windows_core::imp::interface_hierarchy!(IBDA_DRM, windows_core::IUnknown);
impl IBDA_DRM {
    pub unsafe fn GetDRMPairingStatus(&self, pdwstatus: *mut u32, pherror: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDRMPairingStatus)(windows_core::Interface::as_raw(self), pdwstatus as _, pherror as _) }
    }
    pub unsafe fn PerformDRMPairing(&self, fsync: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PerformDRMPairing)(windows_core::Interface::as_raw(self), fsync.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_DRM_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDRMPairingStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub PerformDRMPairing: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IBDA_DRM_Impl: windows_core::IUnknownImpl {
    fn GetDRMPairingStatus(&self, pdwstatus: *mut u32, pherror: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn PerformDRMPairing(&self, fsync: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IBDA_DRM_Vtbl {
    pub const fn new<Identity: IBDA_DRM_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDRMPairingStatus<Identity: IBDA_DRM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32, pherror: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRM_Impl::GetDRMPairingStatus(this, core::mem::transmute_copy(&pdwstatus), core::mem::transmute_copy(&pherror)).into()
            }
        }
        unsafe extern "system" fn PerformDRMPairing<Identity: IBDA_DRM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsync: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRM_Impl::PerformDRMPairing(this, core::mem::transmute_copy(&fsync)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDRMPairingStatus: GetDRMPairingStatus::<Identity, OFFSET>,
            PerformDRMPairing: PerformDRMPairing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_DRM as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_DRM {}
windows_core::imp::define_interface!(IBDA_DRMService, IBDA_DRMService_Vtbl, 0xbff6b5bb_b0ae_484c_9dca_73528fb0b46e);
windows_core::imp::interface_hierarchy!(IBDA_DRMService, windows_core::IUnknown);
impl IBDA_DRMService {
    pub unsafe fn SetDRM(&self, puuidnewdrm: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDRM)(windows_core::Interface::as_raw(self), puuidnewdrm) }
    }
    pub unsafe fn GetDRMStatus(&self, pbstrdrmuuidlist: *mut windows_core::BSTR, drmuuid: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDRMStatus)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrdrmuuidlist), drmuuid as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_DRMService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDRM: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetDRMStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IBDA_DRMService_Impl: windows_core::IUnknownImpl {
    fn SetDRM(&self, puuidnewdrm: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetDRMStatus(&self, pbstrdrmuuidlist: *mut windows_core::BSTR, drmuuid: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl IBDA_DRMService_Vtbl {
    pub const fn new<Identity: IBDA_DRMService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDRM<Identity: IBDA_DRMService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puuidnewdrm: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRMService_Impl::SetDRM(this, core::mem::transmute_copy(&puuidnewdrm)).into()
            }
        }
        unsafe extern "system" fn GetDRMStatus<Identity: IBDA_DRMService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdrmuuidlist: *mut *mut core::ffi::c_void, drmuuid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DRMService_Impl::GetDRMStatus(this, core::mem::transmute_copy(&pbstrdrmuuidlist), core::mem::transmute_copy(&drmuuid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDRM: SetDRM::<Identity, OFFSET>,
            GetDRMStatus: GetDRMStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_DRMService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_DRMService {}
windows_core::imp::define_interface!(IBDA_DeviceControl, IBDA_DeviceControl_Vtbl, 0xfd0a5af3_b41d_11d2_9c95_00c04f7971e0);
windows_core::imp::interface_hierarchy!(IBDA_DeviceControl, windows_core::IUnknown);
impl IBDA_DeviceControl {
    pub unsafe fn StartChanges(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartChanges)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CheckChanges(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckChanges)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CommitChanges(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommitChanges)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetChangeState(&self, pstate: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetChangeState)(windows_core::Interface::as_raw(self), pstate as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_DeviceControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommitChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChangeState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IBDA_DeviceControl_Impl: windows_core::IUnknownImpl {
    fn StartChanges(&self) -> windows_core::Result<()>;
    fn CheckChanges(&self) -> windows_core::Result<()>;
    fn CommitChanges(&self) -> windows_core::Result<()>;
    fn GetChangeState(&self, pstate: *mut u32) -> windows_core::Result<()>;
}
impl IBDA_DeviceControl_Vtbl {
    pub const fn new<Identity: IBDA_DeviceControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartChanges<Identity: IBDA_DeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DeviceControl_Impl::StartChanges(this).into()
            }
        }
        unsafe extern "system" fn CheckChanges<Identity: IBDA_DeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DeviceControl_Impl::CheckChanges(this).into()
            }
        }
        unsafe extern "system" fn CommitChanges<Identity: IBDA_DeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DeviceControl_Impl::CommitChanges(this).into()
            }
        }
        unsafe extern "system" fn GetChangeState<Identity: IBDA_DeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DeviceControl_Impl::GetChangeState(this, core::mem::transmute_copy(&pstate)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartChanges: StartChanges::<Identity, OFFSET>,
            CheckChanges: CheckChanges::<Identity, OFFSET>,
            CommitChanges: CommitChanges::<Identity, OFFSET>,
            GetChangeState: GetChangeState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_DeviceControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_DeviceControl {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IBDA_DiagnosticProperties, IBDA_DiagnosticProperties_Vtbl, 0x20e80cb5_c543_4c1b_8eb3_49e719eee7d4);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IBDA_DiagnosticProperties {
    type Target = super::oaidl::IPropertyBag;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IBDA_DiagnosticProperties, windows_core::IUnknown, super::oaidl::IPropertyBag);
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_DiagnosticProperties_Vtbl {
    pub base__: super::oaidl::IPropertyBag_Vtbl,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IBDA_DiagnosticProperties_Impl: super::oaidl::IPropertyBag_Impl {}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IBDA_DiagnosticProperties_Vtbl {
    pub const fn new<Identity: IBDA_DiagnosticProperties_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IPropertyBag_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_DiagnosticProperties as windows_core::Interface>::IID || iid == &<super::oaidl::IPropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IBDA_DiagnosticProperties {}
windows_core::imp::define_interface!(IBDA_DigitalDemodulator, IBDA_DigitalDemodulator_Vtbl, 0xef30f379_985b_4d10_b640_a79d5e04e1e0);
windows_core::imp::interface_hierarchy!(IBDA_DigitalDemodulator, windows_core::IUnknown);
impl IBDA_DigitalDemodulator {
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_ModulationType(&self, pmodulationtype: *const super::bdatypes::ModulationType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ModulationType)(windows_core::Interface::as_raw(self), pmodulationtype) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_ModulationType(&self, pmodulationtype: *mut super::bdatypes::ModulationType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_ModulationType)(windows_core::Interface::as_raw(self), pmodulationtype as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_InnerFECMethod(&self, pfecmethod: *const super::bdatypes::FECMethod) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InnerFECMethod)(windows_core::Interface::as_raw(self), pfecmethod) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_InnerFECMethod(&self, pfecmethod: *mut super::bdatypes::FECMethod) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_InnerFECMethod)(windows_core::Interface::as_raw(self), pfecmethod as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_InnerFECRate(&self, pfecrate: *const super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InnerFECRate)(windows_core::Interface::as_raw(self), pfecrate) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_InnerFECRate(&self, pfecrate: *mut super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_InnerFECRate)(windows_core::Interface::as_raw(self), pfecrate as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_OuterFECMethod(&self, pfecmethod: *const super::bdatypes::FECMethod) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OuterFECMethod)(windows_core::Interface::as_raw(self), pfecmethod) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_OuterFECMethod(&self, pfecmethod: *mut super::bdatypes::FECMethod) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_OuterFECMethod)(windows_core::Interface::as_raw(self), pfecmethod as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_OuterFECRate(&self, pfecrate: *const super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OuterFECRate)(windows_core::Interface::as_raw(self), pfecrate) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_OuterFECRate(&self, pfecrate: *mut super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_OuterFECRate)(windows_core::Interface::as_raw(self), pfecrate as _) }
    }
    pub unsafe fn put_SymbolRate(&self, psymbolrate: *const u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SymbolRate)(windows_core::Interface::as_raw(self), psymbolrate) }
    }
    pub unsafe fn get_SymbolRate(&self, psymbolrate: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SymbolRate)(windows_core::Interface::as_raw(self), psymbolrate as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_SpectralInversion(&self, pspectralinversion: *const super::bdatypes::SpectralInversion) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SpectralInversion)(windows_core::Interface::as_raw(self), pspectralinversion) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_SpectralInversion(&self, pspectralinversion: *mut super::bdatypes::SpectralInversion) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SpectralInversion)(windows_core::Interface::as_raw(self), pspectralinversion as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_DigitalDemodulator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_ModulationType: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::bdatypes::ModulationType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_ModulationType: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_ModulationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::ModulationType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_ModulationType: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_InnerFECMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::bdatypes::FECMethod) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_InnerFECMethod: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_InnerFECMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::FECMethod) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_InnerFECMethod: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_InnerFECRate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_InnerFECRate: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_InnerFECRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_InnerFECRate: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_OuterFECMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::bdatypes::FECMethod) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_OuterFECMethod: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_OuterFECMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::FECMethod) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_OuterFECMethod: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_OuterFECRate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_OuterFECRate: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_OuterFECRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_OuterFECRate: usize,
    pub put_SymbolRate: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub get_SymbolRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_SpectralInversion: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::bdatypes::SpectralInversion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_SpectralInversion: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_SpectralInversion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::SpectralInversion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_SpectralInversion: usize,
}
#[cfg(feature = "Win32_bdatypes")]
pub trait IBDA_DigitalDemodulator_Impl: windows_core::IUnknownImpl {
    fn put_ModulationType(&self, pmodulationtype: *const super::bdatypes::ModulationType) -> windows_core::Result<()>;
    fn get_ModulationType(&self, pmodulationtype: *mut super::bdatypes::ModulationType) -> windows_core::Result<()>;
    fn put_InnerFECMethod(&self, pfecmethod: *const super::bdatypes::FECMethod) -> windows_core::Result<()>;
    fn get_InnerFECMethod(&self, pfecmethod: *mut super::bdatypes::FECMethod) -> windows_core::Result<()>;
    fn put_InnerFECRate(&self, pfecrate: *const super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::Result<()>;
    fn get_InnerFECRate(&self, pfecrate: *mut super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::Result<()>;
    fn put_OuterFECMethod(&self, pfecmethod: *const super::bdatypes::FECMethod) -> windows_core::Result<()>;
    fn get_OuterFECMethod(&self, pfecmethod: *mut super::bdatypes::FECMethod) -> windows_core::Result<()>;
    fn put_OuterFECRate(&self, pfecrate: *const super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::Result<()>;
    fn get_OuterFECRate(&self, pfecrate: *mut super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::Result<()>;
    fn put_SymbolRate(&self, psymbolrate: *const u32) -> windows_core::Result<()>;
    fn get_SymbolRate(&self, psymbolrate: *mut u32) -> windows_core::Result<()>;
    fn put_SpectralInversion(&self, pspectralinversion: *const super::bdatypes::SpectralInversion) -> windows_core::Result<()>;
    fn get_SpectralInversion(&self, pspectralinversion: *mut super::bdatypes::SpectralInversion) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_bdatypes")]
impl IBDA_DigitalDemodulator_Vtbl {
    pub const fn new<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_ModulationType<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodulationtype: *const super::bdatypes::ModulationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::put_ModulationType(this, core::mem::transmute_copy(&pmodulationtype)).into()
            }
        }
        unsafe extern "system" fn get_ModulationType<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodulationtype: *mut super::bdatypes::ModulationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::get_ModulationType(this, core::mem::transmute_copy(&pmodulationtype)).into()
            }
        }
        unsafe extern "system" fn put_InnerFECMethod<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfecmethod: *const super::bdatypes::FECMethod) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::put_InnerFECMethod(this, core::mem::transmute_copy(&pfecmethod)).into()
            }
        }
        unsafe extern "system" fn get_InnerFECMethod<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfecmethod: *mut super::bdatypes::FECMethod) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::get_InnerFECMethod(this, core::mem::transmute_copy(&pfecmethod)).into()
            }
        }
        unsafe extern "system" fn put_InnerFECRate<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfecrate: *const super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::put_InnerFECRate(this, core::mem::transmute_copy(&pfecrate)).into()
            }
        }
        unsafe extern "system" fn get_InnerFECRate<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfecrate: *mut super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::get_InnerFECRate(this, core::mem::transmute_copy(&pfecrate)).into()
            }
        }
        unsafe extern "system" fn put_OuterFECMethod<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfecmethod: *const super::bdatypes::FECMethod) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::put_OuterFECMethod(this, core::mem::transmute_copy(&pfecmethod)).into()
            }
        }
        unsafe extern "system" fn get_OuterFECMethod<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfecmethod: *mut super::bdatypes::FECMethod) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::get_OuterFECMethod(this, core::mem::transmute_copy(&pfecmethod)).into()
            }
        }
        unsafe extern "system" fn put_OuterFECRate<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfecrate: *const super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::put_OuterFECRate(this, core::mem::transmute_copy(&pfecrate)).into()
            }
        }
        unsafe extern "system" fn get_OuterFECRate<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfecrate: *mut super::bdatypes::BinaryConvolutionCodeRate) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::get_OuterFECRate(this, core::mem::transmute_copy(&pfecrate)).into()
            }
        }
        unsafe extern "system" fn put_SymbolRate<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psymbolrate: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::put_SymbolRate(this, core::mem::transmute_copy(&psymbolrate)).into()
            }
        }
        unsafe extern "system" fn get_SymbolRate<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psymbolrate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::get_SymbolRate(this, core::mem::transmute_copy(&psymbolrate)).into()
            }
        }
        unsafe extern "system" fn put_SpectralInversion<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pspectralinversion: *const super::bdatypes::SpectralInversion) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::put_SpectralInversion(this, core::mem::transmute_copy(&pspectralinversion)).into()
            }
        }
        unsafe extern "system" fn get_SpectralInversion<Identity: IBDA_DigitalDemodulator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pspectralinversion: *mut super::bdatypes::SpectralInversion) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator_Impl::get_SpectralInversion(this, core::mem::transmute_copy(&pspectralinversion)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            put_ModulationType: put_ModulationType::<Identity, OFFSET>,
            get_ModulationType: get_ModulationType::<Identity, OFFSET>,
            put_InnerFECMethod: put_InnerFECMethod::<Identity, OFFSET>,
            get_InnerFECMethod: get_InnerFECMethod::<Identity, OFFSET>,
            put_InnerFECRate: put_InnerFECRate::<Identity, OFFSET>,
            get_InnerFECRate: get_InnerFECRate::<Identity, OFFSET>,
            put_OuterFECMethod: put_OuterFECMethod::<Identity, OFFSET>,
            get_OuterFECMethod: get_OuterFECMethod::<Identity, OFFSET>,
            put_OuterFECRate: put_OuterFECRate::<Identity, OFFSET>,
            get_OuterFECRate: get_OuterFECRate::<Identity, OFFSET>,
            put_SymbolRate: put_SymbolRate::<Identity, OFFSET>,
            get_SymbolRate: get_SymbolRate::<Identity, OFFSET>,
            put_SpectralInversion: put_SpectralInversion::<Identity, OFFSET>,
            get_SpectralInversion: get_SpectralInversion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_DigitalDemodulator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_bdatypes")]
impl windows_core::RuntimeName for IBDA_DigitalDemodulator {}
windows_core::imp::define_interface!(IBDA_DigitalDemodulator2, IBDA_DigitalDemodulator2_Vtbl, 0x525ed3ee_5cf3_4e1e_9a06_5368a84f9a6e);
impl core::ops::Deref for IBDA_DigitalDemodulator2 {
    type Target = IBDA_DigitalDemodulator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBDA_DigitalDemodulator2, windows_core::IUnknown, IBDA_DigitalDemodulator);
impl IBDA_DigitalDemodulator2 {
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_GuardInterval(&self, pguardinterval: *const super::bdatypes::GuardInterval) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_GuardInterval)(windows_core::Interface::as_raw(self), pguardinterval) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_GuardInterval(&self, pguardinterval: *mut super::bdatypes::GuardInterval) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_GuardInterval)(windows_core::Interface::as_raw(self), pguardinterval as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_TransmissionMode(&self, ptransmissionmode: *const super::bdatypes::TransmissionMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TransmissionMode)(windows_core::Interface::as_raw(self), ptransmissionmode) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_TransmissionMode(&self, ptransmissionmode: *mut super::bdatypes::TransmissionMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_TransmissionMode)(windows_core::Interface::as_raw(self), ptransmissionmode as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_RollOff(&self, prolloff: *const super::bdatypes::RollOff) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RollOff)(windows_core::Interface::as_raw(self), prolloff) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_RollOff(&self, prolloff: *mut super::bdatypes::RollOff) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_RollOff)(windows_core::Interface::as_raw(self), prolloff as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_Pilot(&self, ppilot: *const super::bdatypes::Pilot) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Pilot)(windows_core::Interface::as_raw(self), ppilot) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_Pilot(&self, ppilot: *mut super::bdatypes::Pilot) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_Pilot)(windows_core::Interface::as_raw(self), ppilot as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_DigitalDemodulator2_Vtbl {
    pub base__: IBDA_DigitalDemodulator_Vtbl,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_GuardInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::bdatypes::GuardInterval) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_GuardInterval: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_GuardInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::GuardInterval) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_GuardInterval: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_TransmissionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::bdatypes::TransmissionMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_TransmissionMode: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_TransmissionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::TransmissionMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_TransmissionMode: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_RollOff: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::bdatypes::RollOff) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_RollOff: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_RollOff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::RollOff) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_RollOff: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_Pilot: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::bdatypes::Pilot) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_Pilot: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_Pilot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::Pilot) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_Pilot: usize,
}
#[cfg(feature = "Win32_bdatypes")]
pub trait IBDA_DigitalDemodulator2_Impl: IBDA_DigitalDemodulator_Impl {
    fn put_GuardInterval(&self, pguardinterval: *const super::bdatypes::GuardInterval) -> windows_core::Result<()>;
    fn get_GuardInterval(&self, pguardinterval: *mut super::bdatypes::GuardInterval) -> windows_core::Result<()>;
    fn put_TransmissionMode(&self, ptransmissionmode: *const super::bdatypes::TransmissionMode) -> windows_core::Result<()>;
    fn get_TransmissionMode(&self, ptransmissionmode: *mut super::bdatypes::TransmissionMode) -> windows_core::Result<()>;
    fn put_RollOff(&self, prolloff: *const super::bdatypes::RollOff) -> windows_core::Result<()>;
    fn get_RollOff(&self, prolloff: *mut super::bdatypes::RollOff) -> windows_core::Result<()>;
    fn put_Pilot(&self, ppilot: *const super::bdatypes::Pilot) -> windows_core::Result<()>;
    fn get_Pilot(&self, ppilot: *mut super::bdatypes::Pilot) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_bdatypes")]
impl IBDA_DigitalDemodulator2_Vtbl {
    pub const fn new<Identity: IBDA_DigitalDemodulator2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_GuardInterval<Identity: IBDA_DigitalDemodulator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguardinterval: *const super::bdatypes::GuardInterval) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator2_Impl::put_GuardInterval(this, core::mem::transmute_copy(&pguardinterval)).into()
            }
        }
        unsafe extern "system" fn get_GuardInterval<Identity: IBDA_DigitalDemodulator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguardinterval: *mut super::bdatypes::GuardInterval) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator2_Impl::get_GuardInterval(this, core::mem::transmute_copy(&pguardinterval)).into()
            }
        }
        unsafe extern "system" fn put_TransmissionMode<Identity: IBDA_DigitalDemodulator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransmissionmode: *const super::bdatypes::TransmissionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator2_Impl::put_TransmissionMode(this, core::mem::transmute_copy(&ptransmissionmode)).into()
            }
        }
        unsafe extern "system" fn get_TransmissionMode<Identity: IBDA_DigitalDemodulator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransmissionmode: *mut super::bdatypes::TransmissionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator2_Impl::get_TransmissionMode(this, core::mem::transmute_copy(&ptransmissionmode)).into()
            }
        }
        unsafe extern "system" fn put_RollOff<Identity: IBDA_DigitalDemodulator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prolloff: *const super::bdatypes::RollOff) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator2_Impl::put_RollOff(this, core::mem::transmute_copy(&prolloff)).into()
            }
        }
        unsafe extern "system" fn get_RollOff<Identity: IBDA_DigitalDemodulator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prolloff: *mut super::bdatypes::RollOff) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator2_Impl::get_RollOff(this, core::mem::transmute_copy(&prolloff)).into()
            }
        }
        unsafe extern "system" fn put_Pilot<Identity: IBDA_DigitalDemodulator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppilot: *const super::bdatypes::Pilot) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator2_Impl::put_Pilot(this, core::mem::transmute_copy(&ppilot)).into()
            }
        }
        unsafe extern "system" fn get_Pilot<Identity: IBDA_DigitalDemodulator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppilot: *mut super::bdatypes::Pilot) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator2_Impl::get_Pilot(this, core::mem::transmute_copy(&ppilot)).into()
            }
        }
        Self {
            base__: IBDA_DigitalDemodulator_Vtbl::new::<Identity, OFFSET>(),
            put_GuardInterval: put_GuardInterval::<Identity, OFFSET>,
            get_GuardInterval: get_GuardInterval::<Identity, OFFSET>,
            put_TransmissionMode: put_TransmissionMode::<Identity, OFFSET>,
            get_TransmissionMode: get_TransmissionMode::<Identity, OFFSET>,
            put_RollOff: put_RollOff::<Identity, OFFSET>,
            get_RollOff: get_RollOff::<Identity, OFFSET>,
            put_Pilot: put_Pilot::<Identity, OFFSET>,
            get_Pilot: get_Pilot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_DigitalDemodulator2 as windows_core::Interface>::IID || iid == &<IBDA_DigitalDemodulator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_bdatypes")]
impl windows_core::RuntimeName for IBDA_DigitalDemodulator2 {}
windows_core::imp::define_interface!(IBDA_DigitalDemodulator3, IBDA_DigitalDemodulator3_Vtbl, 0x13f19604_7d32_4359_93a2_a05205d90ac9);
impl core::ops::Deref for IBDA_DigitalDemodulator3 {
    type Target = IBDA_DigitalDemodulator2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBDA_DigitalDemodulator3, windows_core::IUnknown, IBDA_DigitalDemodulator, IBDA_DigitalDemodulator2);
impl IBDA_DigitalDemodulator3 {
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_SignalTimeouts(&self, psignaltimeouts: *const super::bdatypes::BDA_SIGNAL_TIMEOUTS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SignalTimeouts)(windows_core::Interface::as_raw(self), psignaltimeouts) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_SignalTimeouts(&self, psignaltimeouts: *mut super::bdatypes::BDA_SIGNAL_TIMEOUTS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SignalTimeouts)(windows_core::Interface::as_raw(self), psignaltimeouts as _) }
    }
    pub unsafe fn put_PLPNumber(&self, pplpnumber: *const u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PLPNumber)(windows_core::Interface::as_raw(self), pplpnumber) }
    }
    pub unsafe fn get_PLPNumber(&self, pplpnumber: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_PLPNumber)(windows_core::Interface::as_raw(self), pplpnumber as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_DigitalDemodulator3_Vtbl {
    pub base__: IBDA_DigitalDemodulator2_Vtbl,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_SignalTimeouts: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::bdatypes::BDA_SIGNAL_TIMEOUTS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_SignalTimeouts: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_SignalTimeouts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::BDA_SIGNAL_TIMEOUTS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_SignalTimeouts: usize,
    pub put_PLPNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub get_PLPNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_bdatypes")]
pub trait IBDA_DigitalDemodulator3_Impl: IBDA_DigitalDemodulator2_Impl {
    fn put_SignalTimeouts(&self, psignaltimeouts: *const super::bdatypes::BDA_SIGNAL_TIMEOUTS) -> windows_core::Result<()>;
    fn get_SignalTimeouts(&self, psignaltimeouts: *mut super::bdatypes::BDA_SIGNAL_TIMEOUTS) -> windows_core::Result<()>;
    fn put_PLPNumber(&self, pplpnumber: *const u32) -> windows_core::Result<()>;
    fn get_PLPNumber(&self, pplpnumber: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_bdatypes")]
impl IBDA_DigitalDemodulator3_Vtbl {
    pub const fn new<Identity: IBDA_DigitalDemodulator3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_SignalTimeouts<Identity: IBDA_DigitalDemodulator3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psignaltimeouts: *const super::bdatypes::BDA_SIGNAL_TIMEOUTS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator3_Impl::put_SignalTimeouts(this, core::mem::transmute_copy(&psignaltimeouts)).into()
            }
        }
        unsafe extern "system" fn get_SignalTimeouts<Identity: IBDA_DigitalDemodulator3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psignaltimeouts: *mut super::bdatypes::BDA_SIGNAL_TIMEOUTS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator3_Impl::get_SignalTimeouts(this, core::mem::transmute_copy(&psignaltimeouts)).into()
            }
        }
        unsafe extern "system" fn put_PLPNumber<Identity: IBDA_DigitalDemodulator3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplpnumber: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator3_Impl::put_PLPNumber(this, core::mem::transmute_copy(&pplpnumber)).into()
            }
        }
        unsafe extern "system" fn get_PLPNumber<Identity: IBDA_DigitalDemodulator3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplpnumber: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DigitalDemodulator3_Impl::get_PLPNumber(this, core::mem::transmute_copy(&pplpnumber)).into()
            }
        }
        Self {
            base__: IBDA_DigitalDemodulator2_Vtbl::new::<Identity, OFFSET>(),
            put_SignalTimeouts: put_SignalTimeouts::<Identity, OFFSET>,
            get_SignalTimeouts: get_SignalTimeouts::<Identity, OFFSET>,
            put_PLPNumber: put_PLPNumber::<Identity, OFFSET>,
            get_PLPNumber: get_PLPNumber::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_DigitalDemodulator3 as windows_core::Interface>::IID || iid == &<IBDA_DigitalDemodulator as windows_core::Interface>::IID || iid == &<IBDA_DigitalDemodulator2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_bdatypes")]
impl windows_core::RuntimeName for IBDA_DigitalDemodulator3 {}
windows_core::imp::define_interface!(IBDA_DiseqCommand, IBDA_DiseqCommand_Vtbl, 0xf84e2ab0_3c6b_45e3_a0fc_8669d4b81f11);
windows_core::imp::interface_hierarchy!(IBDA_DiseqCommand, windows_core::IUnknown);
impl IBDA_DiseqCommand {
    pub unsafe fn put_EnableDiseqCommands(&self, benable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EnableDiseqCommands)(windows_core::Interface::as_raw(self), benable) }
    }
    pub unsafe fn put_DiseqLNBSource(&self, ullnbsource: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DiseqLNBSource)(windows_core::Interface::as_raw(self), ullnbsource) }
    }
    pub unsafe fn put_DiseqUseToneBurst(&self, busetoneburst: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DiseqUseToneBurst)(windows_core::Interface::as_raw(self), busetoneburst) }
    }
    pub unsafe fn put_DiseqRepeats(&self, ulrepeats: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DiseqRepeats)(windows_core::Interface::as_raw(self), ulrepeats) }
    }
    pub unsafe fn put_DiseqSendCommand(&self, ulrequestid: u32, ulcbcommandlen: u32, pbcommand: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DiseqSendCommand)(windows_core::Interface::as_raw(self), ulrequestid, ulcbcommandlen, pbcommand) }
    }
    pub unsafe fn get_DiseqResponse(&self, ulrequestid: u32, pulcbresponselen: *mut u32, pbresponse: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_DiseqResponse)(windows_core::Interface::as_raw(self), ulrequestid, pulcbresponselen as _, pbresponse as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_DiseqCommand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub put_EnableDiseqCommands: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub put_DiseqLNBSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub put_DiseqUseToneBurst: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub put_DiseqRepeats: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub put_DiseqSendCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8) -> windows_core::HRESULT,
    pub get_DiseqResponse: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u8) -> windows_core::HRESULT,
}
pub trait IBDA_DiseqCommand_Impl: windows_core::IUnknownImpl {
    fn put_EnableDiseqCommands(&self, benable: bool) -> windows_core::Result<()>;
    fn put_DiseqLNBSource(&self, ullnbsource: u32) -> windows_core::Result<()>;
    fn put_DiseqUseToneBurst(&self, busetoneburst: bool) -> windows_core::Result<()>;
    fn put_DiseqRepeats(&self, ulrepeats: u32) -> windows_core::Result<()>;
    fn put_DiseqSendCommand(&self, ulrequestid: u32, ulcbcommandlen: u32, pbcommand: *const u8) -> windows_core::Result<()>;
    fn get_DiseqResponse(&self, ulrequestid: u32, pulcbresponselen: *mut u32, pbresponse: *mut u8) -> windows_core::Result<()>;
}
impl IBDA_DiseqCommand_Vtbl {
    pub const fn new<Identity: IBDA_DiseqCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_EnableDiseqCommands<Identity: IBDA_DiseqCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DiseqCommand_Impl::put_EnableDiseqCommands(this, core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn put_DiseqLNBSource<Identity: IBDA_DiseqCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullnbsource: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DiseqCommand_Impl::put_DiseqLNBSource(this, core::mem::transmute_copy(&ullnbsource)).into()
            }
        }
        unsafe extern "system" fn put_DiseqUseToneBurst<Identity: IBDA_DiseqCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busetoneburst: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DiseqCommand_Impl::put_DiseqUseToneBurst(this, core::mem::transmute_copy(&busetoneburst)).into()
            }
        }
        unsafe extern "system" fn put_DiseqRepeats<Identity: IBDA_DiseqCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrepeats: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DiseqCommand_Impl::put_DiseqRepeats(this, core::mem::transmute_copy(&ulrepeats)).into()
            }
        }
        unsafe extern "system" fn put_DiseqSendCommand<Identity: IBDA_DiseqCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrequestid: u32, ulcbcommandlen: u32, pbcommand: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DiseqCommand_Impl::put_DiseqSendCommand(this, core::mem::transmute_copy(&ulrequestid), core::mem::transmute_copy(&ulcbcommandlen), core::mem::transmute_copy(&pbcommand)).into()
            }
        }
        unsafe extern "system" fn get_DiseqResponse<Identity: IBDA_DiseqCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrequestid: u32, pulcbresponselen: *mut u32, pbresponse: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_DiseqCommand_Impl::get_DiseqResponse(this, core::mem::transmute_copy(&ulrequestid), core::mem::transmute_copy(&pulcbresponselen), core::mem::transmute_copy(&pbresponse)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            put_EnableDiseqCommands: put_EnableDiseqCommands::<Identity, OFFSET>,
            put_DiseqLNBSource: put_DiseqLNBSource::<Identity, OFFSET>,
            put_DiseqUseToneBurst: put_DiseqUseToneBurst::<Identity, OFFSET>,
            put_DiseqRepeats: put_DiseqRepeats::<Identity, OFFSET>,
            put_DiseqSendCommand: put_DiseqSendCommand::<Identity, OFFSET>,
            get_DiseqResponse: get_DiseqResponse::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_DiseqCommand as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_DiseqCommand {}
windows_core::imp::define_interface!(IBDA_EasMessage, IBDA_EasMessage_Vtbl, 0xd806973d_3ebe_46de_8fbb_6358fe784208);
windows_core::imp::interface_hierarchy!(IBDA_EasMessage, windows_core::IUnknown);
impl IBDA_EasMessage {
    pub unsafe fn get_EasMessage(&self, uleventid: u32, ppeasobject: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_EasMessage)(windows_core::Interface::as_raw(self), uleventid, core::mem::transmute(ppeasobject)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_EasMessage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_EasMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBDA_EasMessage_Impl: windows_core::IUnknownImpl {
    fn get_EasMessage(&self, uleventid: u32, ppeasobject: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IBDA_EasMessage_Vtbl {
    pub const fn new<Identity: IBDA_EasMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_EasMessage<Identity: IBDA_EasMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uleventid: u32, ppeasobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_EasMessage_Impl::get_EasMessage(this, core::mem::transmute_copy(&uleventid), core::mem::transmute_copy(&ppeasobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), get_EasMessage: get_EasMessage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_EasMessage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_EasMessage {}
windows_core::imp::define_interface!(IBDA_Encoder, IBDA_Encoder_Vtbl, 0x3a8bad59_59fe_4559_a0ba_396cfaa98ae3);
windows_core::imp::interface_hierarchy!(IBDA_Encoder, windows_core::IUnknown);
impl IBDA_Encoder {
    pub unsafe fn QueryCapabilities(&self, numaudiofmts: *mut u32, numvideofmts: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryCapabilities)(windows_core::Interface::as_raw(self), numaudiofmts as _, numvideofmts as _) }
    }
    pub unsafe fn EnumAudioCapability(&self, fmtindex: u32, methodid: *mut u32, algorithmtype: *mut u32, samplingrate: *mut u32, bitdepth: *mut u32, numchannels: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumAudioCapability)(windows_core::Interface::as_raw(self), fmtindex, methodid as _, algorithmtype as _, samplingrate as _, bitdepth as _, numchannels as _) }
    }
    pub unsafe fn EnumVideoCapability(&self, fmtindex: u32, methodid: *mut u32, algorithmtype: *mut u32, verticalsize: *mut u32, horizontalsize: *mut u32, aspectratio: *mut u32, frameratecode: *mut u32, progressivesequence: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumVideoCapability)(windows_core::Interface::as_raw(self), fmtindex, methodid as _, algorithmtype as _, verticalsize as _, horizontalsize as _, aspectratio as _, frameratecode as _, progressivesequence as _) }
    }
    pub unsafe fn SetParameters(&self, audiobitratemode: u32, audiobitrate: u32, audiomethodid: u32, audioprogram: u32, videobitratemode: u32, videobitrate: u32, videomethodid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), audiobitratemode, audiobitrate, audiomethodid, audioprogram, videobitratemode, videobitrate, videomethodid) }
    }
    pub unsafe fn GetState(&self, audiobitratemax: *mut u32, audiobitratemin: *mut u32, audiobitratemode: *mut u32, audiobitratestepping: *mut u32, audiobitrate: *mut u32, audiomethodid: *mut u32, availableaudioprograms: *mut u32, audioprogram: *mut u32, videobitratemax: *mut u32, videobitratemin: *mut u32, videobitratemode: *mut u32, videobitrate: *mut u32, videobitratestepping: *mut u32, videomethodid: *mut u32, signalsourceid: *mut u32, signalformat: *mut u64, signallock: *mut windows_core::BOOL, signallevel: *mut i32, signaltonoiseratio: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), audiobitratemax as _, audiobitratemin as _, audiobitratemode as _, audiobitratestepping as _, audiobitrate as _, audiomethodid as _, availableaudioprograms as _, audioprogram as _, videobitratemax as _, videobitratemin as _, videobitratemode as _, videobitrate as _, videobitratestepping as _, videomethodid as _, signalsourceid as _, signalformat as _, signallock as _, signallevel as _, signaltonoiseratio as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_Encoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub EnumAudioCapability: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub EnumVideoCapability: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, u32, u32, u32) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u64, *mut windows_core::BOOL, *mut i32, *mut u32) -> windows_core::HRESULT,
}
pub trait IBDA_Encoder_Impl: windows_core::IUnknownImpl {
    fn QueryCapabilities(&self, numaudiofmts: *mut u32, numvideofmts: *mut u32) -> windows_core::Result<()>;
    fn EnumAudioCapability(&self, fmtindex: u32, methodid: *mut u32, algorithmtype: *mut u32, samplingrate: *mut u32, bitdepth: *mut u32, numchannels: *mut u32) -> windows_core::Result<()>;
    fn EnumVideoCapability(&self, fmtindex: u32, methodid: *mut u32, algorithmtype: *mut u32, verticalsize: *mut u32, horizontalsize: *mut u32, aspectratio: *mut u32, frameratecode: *mut u32, progressivesequence: *mut u32) -> windows_core::Result<()>;
    fn SetParameters(&self, audiobitratemode: u32, audiobitrate: u32, audiomethodid: u32, audioprogram: u32, videobitratemode: u32, videobitrate: u32, videomethodid: u32) -> windows_core::Result<()>;
    fn GetState(&self, audiobitratemax: *mut u32, audiobitratemin: *mut u32, audiobitratemode: *mut u32, audiobitratestepping: *mut u32, audiobitrate: *mut u32, audiomethodid: *mut u32, availableaudioprograms: *mut u32, audioprogram: *mut u32, videobitratemax: *mut u32, videobitratemin: *mut u32, videobitratemode: *mut u32, videobitrate: *mut u32, videobitratestepping: *mut u32, videomethodid: *mut u32, signalsourceid: *mut u32, signalformat: *mut u64, signallock: *mut windows_core::BOOL, signallevel: *mut i32, signaltonoiseratio: *mut u32) -> windows_core::Result<()>;
}
impl IBDA_Encoder_Vtbl {
    pub const fn new<Identity: IBDA_Encoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryCapabilities<Identity: IBDA_Encoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numaudiofmts: *mut u32, numvideofmts: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Encoder_Impl::QueryCapabilities(this, core::mem::transmute_copy(&numaudiofmts), core::mem::transmute_copy(&numvideofmts)).into()
            }
        }
        unsafe extern "system" fn EnumAudioCapability<Identity: IBDA_Encoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmtindex: u32, methodid: *mut u32, algorithmtype: *mut u32, samplingrate: *mut u32, bitdepth: *mut u32, numchannels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Encoder_Impl::EnumAudioCapability(this, core::mem::transmute_copy(&fmtindex), core::mem::transmute_copy(&methodid), core::mem::transmute_copy(&algorithmtype), core::mem::transmute_copy(&samplingrate), core::mem::transmute_copy(&bitdepth), core::mem::transmute_copy(&numchannels)).into()
            }
        }
        unsafe extern "system" fn EnumVideoCapability<Identity: IBDA_Encoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmtindex: u32, methodid: *mut u32, algorithmtype: *mut u32, verticalsize: *mut u32, horizontalsize: *mut u32, aspectratio: *mut u32, frameratecode: *mut u32, progressivesequence: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Encoder_Impl::EnumVideoCapability(this, core::mem::transmute_copy(&fmtindex), core::mem::transmute_copy(&methodid), core::mem::transmute_copy(&algorithmtype), core::mem::transmute_copy(&verticalsize), core::mem::transmute_copy(&horizontalsize), core::mem::transmute_copy(&aspectratio), core::mem::transmute_copy(&frameratecode), core::mem::transmute_copy(&progressivesequence)).into()
            }
        }
        unsafe extern "system" fn SetParameters<Identity: IBDA_Encoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiobitratemode: u32, audiobitrate: u32, audiomethodid: u32, audioprogram: u32, videobitratemode: u32, videobitrate: u32, videomethodid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Encoder_Impl::SetParameters(this, core::mem::transmute_copy(&audiobitratemode), core::mem::transmute_copy(&audiobitrate), core::mem::transmute_copy(&audiomethodid), core::mem::transmute_copy(&audioprogram), core::mem::transmute_copy(&videobitratemode), core::mem::transmute_copy(&videobitrate), core::mem::transmute_copy(&videomethodid)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IBDA_Encoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiobitratemax: *mut u32, audiobitratemin: *mut u32, audiobitratemode: *mut u32, audiobitratestepping: *mut u32, audiobitrate: *mut u32, audiomethodid: *mut u32, availableaudioprograms: *mut u32, audioprogram: *mut u32, videobitratemax: *mut u32, videobitratemin: *mut u32, videobitratemode: *mut u32, videobitrate: *mut u32, videobitratestepping: *mut u32, videomethodid: *mut u32, signalsourceid: *mut u32, signalformat: *mut u64, signallock: *mut windows_core::BOOL, signallevel: *mut i32, signaltonoiseratio: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Encoder_Impl::GetState(
                    this,
                    core::mem::transmute_copy(&audiobitratemax),
                    core::mem::transmute_copy(&audiobitratemin),
                    core::mem::transmute_copy(&audiobitratemode),
                    core::mem::transmute_copy(&audiobitratestepping),
                    core::mem::transmute_copy(&audiobitrate),
                    core::mem::transmute_copy(&audiomethodid),
                    core::mem::transmute_copy(&availableaudioprograms),
                    core::mem::transmute_copy(&audioprogram),
                    core::mem::transmute_copy(&videobitratemax),
                    core::mem::transmute_copy(&videobitratemin),
                    core::mem::transmute_copy(&videobitratemode),
                    core::mem::transmute_copy(&videobitrate),
                    core::mem::transmute_copy(&videobitratestepping),
                    core::mem::transmute_copy(&videomethodid),
                    core::mem::transmute_copy(&signalsourceid),
                    core::mem::transmute_copy(&signalformat),
                    core::mem::transmute_copy(&signallock),
                    core::mem::transmute_copy(&signallevel),
                    core::mem::transmute_copy(&signaltonoiseratio),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryCapabilities: QueryCapabilities::<Identity, OFFSET>,
            EnumAudioCapability: EnumAudioCapability::<Identity, OFFSET>,
            EnumVideoCapability: EnumVideoCapability::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_Encoder as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_Encoder {}
windows_core::imp::define_interface!(IBDA_EthernetFilter, IBDA_EthernetFilter_Vtbl, 0x71985f43_1ca1_11d3_9cc8_00c04f7971e0);
windows_core::imp::interface_hierarchy!(IBDA_EthernetFilter, windows_core::IUnknown);
impl IBDA_EthernetFilter {
    pub unsafe fn GetMulticastListSize(&self, pulcbaddresses: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMulticastListSize)(windows_core::Interface::as_raw(self), pulcbaddresses as _) }
    }
    pub unsafe fn PutMulticastList(&self, ulcbaddresses: u32, paddresslist: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutMulticastList)(windows_core::Interface::as_raw(self), ulcbaddresses, paddresslist) }
    }
    pub unsafe fn GetMulticastList(&self, pulcbaddresses: *mut u32, paddresslist: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMulticastList)(windows_core::Interface::as_raw(self), pulcbaddresses as _, paddresslist as _) }
    }
    pub unsafe fn PutMulticastMode(&self, ulmodemask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutMulticastMode)(windows_core::Interface::as_raw(self), ulmodemask) }
    }
    pub unsafe fn GetMulticastMode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMulticastMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_EthernetFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMulticastListSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PutMulticastList: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub GetMulticastList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub PutMulticastMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMulticastMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IBDA_EthernetFilter_Impl: windows_core::IUnknownImpl {
    fn GetMulticastListSize(&self, pulcbaddresses: *mut u32) -> windows_core::Result<()>;
    fn PutMulticastList(&self, ulcbaddresses: u32, paddresslist: *const u8) -> windows_core::Result<()>;
    fn GetMulticastList(&self, pulcbaddresses: *mut u32, paddresslist: *mut u8) -> windows_core::Result<()>;
    fn PutMulticastMode(&self, ulmodemask: u32) -> windows_core::Result<()>;
    fn GetMulticastMode(&self) -> windows_core::Result<u32>;
}
impl IBDA_EthernetFilter_Vtbl {
    pub const fn new<Identity: IBDA_EthernetFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMulticastListSize<Identity: IBDA_EthernetFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcbaddresses: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_EthernetFilter_Impl::GetMulticastListSize(this, core::mem::transmute_copy(&pulcbaddresses)).into()
            }
        }
        unsafe extern "system" fn PutMulticastList<Identity: IBDA_EthernetFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcbaddresses: u32, paddresslist: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_EthernetFilter_Impl::PutMulticastList(this, core::mem::transmute_copy(&ulcbaddresses), core::mem::transmute_copy(&paddresslist)).into()
            }
        }
        unsafe extern "system" fn GetMulticastList<Identity: IBDA_EthernetFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcbaddresses: *mut u32, paddresslist: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_EthernetFilter_Impl::GetMulticastList(this, core::mem::transmute_copy(&pulcbaddresses), core::mem::transmute_copy(&paddresslist)).into()
            }
        }
        unsafe extern "system" fn PutMulticastMode<Identity: IBDA_EthernetFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmodemask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_EthernetFilter_Impl::PutMulticastMode(this, core::mem::transmute_copy(&ulmodemask)).into()
            }
        }
        unsafe extern "system" fn GetMulticastMode<Identity: IBDA_EthernetFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmodemask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_EthernetFilter_Impl::GetMulticastMode(this) {
                    Ok(ok__) => {
                        pulmodemask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMulticastListSize: GetMulticastListSize::<Identity, OFFSET>,
            PutMulticastList: PutMulticastList::<Identity, OFFSET>,
            GetMulticastList: GetMulticastList::<Identity, OFFSET>,
            PutMulticastMode: PutMulticastMode::<Identity, OFFSET>,
            GetMulticastMode: GetMulticastMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_EthernetFilter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_EthernetFilter {}
windows_core::imp::define_interface!(IBDA_EventingService, IBDA_EventingService_Vtbl, 0x207c413f_00dc_4c61_bad6_6fee1ff07064);
windows_core::imp::interface_hierarchy!(IBDA_EventingService, windows_core::IUnknown);
impl IBDA_EventingService {
    pub unsafe fn CompleteEvent(&self, uleventid: u32, uleventresult: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CompleteEvent)(windows_core::Interface::as_raw(self), uleventid, uleventresult) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_EventingService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompleteEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait IBDA_EventingService_Impl: windows_core::IUnknownImpl {
    fn CompleteEvent(&self, uleventid: u32, uleventresult: u32) -> windows_core::Result<()>;
}
impl IBDA_EventingService_Vtbl {
    pub const fn new<Identity: IBDA_EventingService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompleteEvent<Identity: IBDA_EventingService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uleventid: u32, uleventresult: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_EventingService_Impl::CompleteEvent(this, core::mem::transmute_copy(&uleventid), core::mem::transmute_copy(&uleventresult)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CompleteEvent: CompleteEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_EventingService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_EventingService {}
windows_core::imp::define_interface!(IBDA_FDC, IBDA_FDC_Vtbl, 0x138adc7e_58ae_437f_b0b4_c9fe19d5b4ac);
windows_core::imp::interface_hierarchy!(IBDA_FDC, windows_core::IUnknown);
impl IBDA_FDC {
    pub unsafe fn GetStatus(&self, currentbitrate: *mut u32, carrierlock: *mut windows_core::BOOL, currentfrequency: *mut u32, currentspectruminversion: *mut windows_core::BOOL, currentpidlist: *mut windows_core::BSTR, currenttidlist: *mut windows_core::BSTR, overflow: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), currentbitrate as _, carrierlock as _, currentfrequency as _, currentspectruminversion as _, core::mem::transmute(currentpidlist), core::mem::transmute(currenttidlist), overflow as _) }
    }
    pub unsafe fn RequestTables(&self, tableids: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestTables)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(tableids)) }
    }
    pub unsafe fn AddPid(&self, pidstoadd: &windows_core::BSTR) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddPid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pidstoadd), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemovePid(&self, pidstoremove: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemovePid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pidstoremove)) }
    }
    pub unsafe fn AddTid(&self, tidstoadd: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddTid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(tidstoadd), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RemoveTid(&self, tidstoremove: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveTid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(tidstoremove)) }
    }
    pub unsafe fn GetTableSection(&self, pid: *mut u32, maxbuffersize: u32, actualsize: *mut u32, secbuffer: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTableSection)(windows_core::Interface::as_raw(self), pid as _, maxbuffersize, actualsize as _, secbuffer as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_FDC_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::BOOL, *mut u32, *mut windows_core::BOOL, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub RequestTables: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemovePid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveTid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTableSection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut u32, *mut u8) -> windows_core::HRESULT,
}
pub trait IBDA_FDC_Impl: windows_core::IUnknownImpl {
    fn GetStatus(&self, currentbitrate: *mut u32, carrierlock: *mut windows_core::BOOL, currentfrequency: *mut u32, currentspectruminversion: *mut windows_core::BOOL, currentpidlist: *mut windows_core::BSTR, currenttidlist: *mut windows_core::BSTR, overflow: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn RequestTables(&self, tableids: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddPid(&self, pidstoadd: &windows_core::BSTR) -> windows_core::Result<u32>;
    fn RemovePid(&self, pidstoremove: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddTid(&self, tidstoadd: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn RemoveTid(&self, tidstoremove: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetTableSection(&self, pid: *mut u32, maxbuffersize: u32, actualsize: *mut u32, secbuffer: *mut u8) -> windows_core::Result<()>;
}
impl IBDA_FDC_Vtbl {
    pub const fn new<Identity: IBDA_FDC_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: IBDA_FDC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentbitrate: *mut u32, carrierlock: *mut windows_core::BOOL, currentfrequency: *mut u32, currentspectruminversion: *mut windows_core::BOOL, currentpidlist: *mut *mut core::ffi::c_void, currenttidlist: *mut *mut core::ffi::c_void, overflow: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FDC_Impl::GetStatus(this, core::mem::transmute_copy(&currentbitrate), core::mem::transmute_copy(&carrierlock), core::mem::transmute_copy(&currentfrequency), core::mem::transmute_copy(&currentspectruminversion), core::mem::transmute_copy(&currentpidlist), core::mem::transmute_copy(&currenttidlist), core::mem::transmute_copy(&overflow)).into()
            }
        }
        unsafe extern "system" fn RequestTables<Identity: IBDA_FDC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tableids: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FDC_Impl::RequestTables(this, core::mem::transmute(&tableids)).into()
            }
        }
        unsafe extern "system" fn AddPid<Identity: IBDA_FDC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidstoadd: *mut core::ffi::c_void, remainingfilterentries: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_FDC_Impl::AddPid(this, core::mem::transmute(&pidstoadd)) {
                    Ok(ok__) => {
                        remainingfilterentries.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePid<Identity: IBDA_FDC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidstoremove: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FDC_Impl::RemovePid(this, core::mem::transmute(&pidstoremove)).into()
            }
        }
        unsafe extern "system" fn AddTid<Identity: IBDA_FDC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tidstoadd: *mut core::ffi::c_void, currenttidlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_FDC_Impl::AddTid(this, core::mem::transmute(&tidstoadd)) {
                    Ok(ok__) => {
                        currenttidlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveTid<Identity: IBDA_FDC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tidstoremove: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FDC_Impl::RemoveTid(this, core::mem::transmute(&tidstoremove)).into()
            }
        }
        unsafe extern "system" fn GetTableSection<Identity: IBDA_FDC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut u32, maxbuffersize: u32, actualsize: *mut u32, secbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FDC_Impl::GetTableSection(this, core::mem::transmute_copy(&pid), core::mem::transmute_copy(&maxbuffersize), core::mem::transmute_copy(&actualsize), core::mem::transmute_copy(&secbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            RequestTables: RequestTables::<Identity, OFFSET>,
            AddPid: AddPid::<Identity, OFFSET>,
            RemovePid: RemovePid::<Identity, OFFSET>,
            AddTid: AddTid::<Identity, OFFSET>,
            RemoveTid: RemoveTid::<Identity, OFFSET>,
            GetTableSection: GetTableSection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_FDC as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_FDC {}
windows_core::imp::define_interface!(IBDA_FrequencyFilter, IBDA_FrequencyFilter_Vtbl, 0x71985f47_1ca1_11d3_9cc8_00c04f7971e0);
windows_core::imp::interface_hierarchy!(IBDA_FrequencyFilter, windows_core::IUnknown);
impl IBDA_FrequencyFilter {
    pub unsafe fn put_Autotune(&self, ultransponder: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Autotune)(windows_core::Interface::as_raw(self), ultransponder) }
    }
    pub unsafe fn get_Autotune(&self, pultransponder: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_Autotune)(windows_core::Interface::as_raw(self), pultransponder as _) }
    }
    pub unsafe fn put_Frequency(&self, ulfrequency: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Frequency)(windows_core::Interface::as_raw(self), ulfrequency) }
    }
    pub unsafe fn get_Frequency(&self, pulfrequency: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_Frequency)(windows_core::Interface::as_raw(self), pulfrequency as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn put_Polarity(&self, polarity: super::bdatypes::Polarisation) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Polarity)(windows_core::Interface::as_raw(self), polarity) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn get_Polarity(&self, ppolarity: *mut super::bdatypes::Polarisation) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_Polarity)(windows_core::Interface::as_raw(self), ppolarity as _) }
    }
    pub unsafe fn put_Range(&self, ulrange: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Range)(windows_core::Interface::as_raw(self), ulrange) }
    }
    pub unsafe fn get_Range(&self, pulrange: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_Range)(windows_core::Interface::as_raw(self), pulrange as _) }
    }
    pub unsafe fn put_Bandwidth(&self, ulbandwidth: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Bandwidth)(windows_core::Interface::as_raw(self), ulbandwidth) }
    }
    pub unsafe fn get_Bandwidth(&self, pulbandwidth: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_Bandwidth)(windows_core::Interface::as_raw(self), pulbandwidth as _) }
    }
    pub unsafe fn put_FrequencyMultiplier(&self, ulmultiplier: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FrequencyMultiplier)(windows_core::Interface::as_raw(self), ulmultiplier) }
    }
    pub unsafe fn get_FrequencyMultiplier(&self, pulmultiplier: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_FrequencyMultiplier)(windows_core::Interface::as_raw(self), pulmultiplier as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_FrequencyFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub put_Autotune: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_Autotune: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_Frequency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_Frequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_bdatypes")]
    pub put_Polarity: unsafe extern "system" fn(*mut core::ffi::c_void, super::bdatypes::Polarisation) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    put_Polarity: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub get_Polarity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::bdatypes::Polarisation) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    get_Polarity: usize,
    pub put_Range: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_Range: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_Bandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_Bandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_FrequencyMultiplier: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_FrequencyMultiplier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_bdatypes")]
pub trait IBDA_FrequencyFilter_Impl: windows_core::IUnknownImpl {
    fn put_Autotune(&self, ultransponder: u32) -> windows_core::Result<()>;
    fn get_Autotune(&self, pultransponder: *mut u32) -> windows_core::Result<()>;
    fn put_Frequency(&self, ulfrequency: u32) -> windows_core::Result<()>;
    fn get_Frequency(&self, pulfrequency: *mut u32) -> windows_core::Result<()>;
    fn put_Polarity(&self, polarity: super::bdatypes::Polarisation) -> windows_core::Result<()>;
    fn get_Polarity(&self, ppolarity: *mut super::bdatypes::Polarisation) -> windows_core::Result<()>;
    fn put_Range(&self, ulrange: u32) -> windows_core::Result<()>;
    fn get_Range(&self, pulrange: *mut u32) -> windows_core::Result<()>;
    fn put_Bandwidth(&self, ulbandwidth: u32) -> windows_core::Result<()>;
    fn get_Bandwidth(&self, pulbandwidth: *mut u32) -> windows_core::Result<()>;
    fn put_FrequencyMultiplier(&self, ulmultiplier: u32) -> windows_core::Result<()>;
    fn get_FrequencyMultiplier(&self, pulmultiplier: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_bdatypes")]
impl IBDA_FrequencyFilter_Vtbl {
    pub const fn new<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_Autotune<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultransponder: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::put_Autotune(this, core::mem::transmute_copy(&ultransponder)).into()
            }
        }
        unsafe extern "system" fn get_Autotune<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultransponder: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::get_Autotune(this, core::mem::transmute_copy(&pultransponder)).into()
            }
        }
        unsafe extern "system" fn put_Frequency<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulfrequency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::put_Frequency(this, core::mem::transmute_copy(&ulfrequency)).into()
            }
        }
        unsafe extern "system" fn get_Frequency<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulfrequency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::get_Frequency(this, core::mem::transmute_copy(&pulfrequency)).into()
            }
        }
        unsafe extern "system" fn put_Polarity<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, polarity: super::bdatypes::Polarisation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::put_Polarity(this, core::mem::transmute_copy(&polarity)).into()
            }
        }
        unsafe extern "system" fn get_Polarity<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolarity: *mut super::bdatypes::Polarisation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::get_Polarity(this, core::mem::transmute_copy(&ppolarity)).into()
            }
        }
        unsafe extern "system" fn put_Range<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrange: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::put_Range(this, core::mem::transmute_copy(&ulrange)).into()
            }
        }
        unsafe extern "system" fn get_Range<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulrange: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::get_Range(this, core::mem::transmute_copy(&pulrange)).into()
            }
        }
        unsafe extern "system" fn put_Bandwidth<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulbandwidth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::put_Bandwidth(this, core::mem::transmute_copy(&ulbandwidth)).into()
            }
        }
        unsafe extern "system" fn get_Bandwidth<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulbandwidth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::get_Bandwidth(this, core::mem::transmute_copy(&pulbandwidth)).into()
            }
        }
        unsafe extern "system" fn put_FrequencyMultiplier<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmultiplier: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::put_FrequencyMultiplier(this, core::mem::transmute_copy(&ulmultiplier)).into()
            }
        }
        unsafe extern "system" fn get_FrequencyMultiplier<Identity: IBDA_FrequencyFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmultiplier: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_FrequencyFilter_Impl::get_FrequencyMultiplier(this, core::mem::transmute_copy(&pulmultiplier)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            put_Autotune: put_Autotune::<Identity, OFFSET>,
            get_Autotune: get_Autotune::<Identity, OFFSET>,
            put_Frequency: put_Frequency::<Identity, OFFSET>,
            get_Frequency: get_Frequency::<Identity, OFFSET>,
            put_Polarity: put_Polarity::<Identity, OFFSET>,
            get_Polarity: get_Polarity::<Identity, OFFSET>,
            put_Range: put_Range::<Identity, OFFSET>,
            get_Range: get_Range::<Identity, OFFSET>,
            put_Bandwidth: put_Bandwidth::<Identity, OFFSET>,
            get_Bandwidth: get_Bandwidth::<Identity, OFFSET>,
            put_FrequencyMultiplier: put_FrequencyMultiplier::<Identity, OFFSET>,
            get_FrequencyMultiplier: get_FrequencyMultiplier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_FrequencyFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_bdatypes")]
impl windows_core::RuntimeName for IBDA_FrequencyFilter {}
windows_core::imp::define_interface!(IBDA_GuideDataDeliveryService, IBDA_GuideDataDeliveryService_Vtbl, 0xc0afcb73_23e7_4bc6_bafa_fdc167b4719f);
windows_core::imp::interface_hierarchy!(IBDA_GuideDataDeliveryService, windows_core::IUnknown);
impl IBDA_GuideDataDeliveryService {
    pub unsafe fn GetGuideDataType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGuideDataType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGuideData(&self, pulcbbufferlen: *mut u32, pbbuffer: *mut u8, pulguidedatapercentageprogress: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGuideData)(windows_core::Interface::as_raw(self), pulcbbufferlen as _, pbbuffer as _, pulguidedatapercentageprogress as _) }
    }
    pub unsafe fn RequestGuideDataUpdate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestGuideDataUpdate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTuneXmlFromServiceIdx(&self, ul64serviceidx: u64) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTuneXmlFromServiceIdx)(windows_core::Interface::as_raw(self), ul64serviceidx, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetServices(&self, pulcbbufferlen: *mut u32, pbbuffer: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetServices)(windows_core::Interface::as_raw(self), pulcbbufferlen as _, pbbuffer as _) }
    }
    pub unsafe fn GetServiceInfoFromTuneXml(&self, bstrtunexml: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceInfoFromTuneXml)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtunexml), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_GuideDataDeliveryService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGuideDataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetGuideData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub RequestGuideDataUpdate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTuneXmlFromServiceIdx: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub GetServiceInfoFromTuneXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBDA_GuideDataDeliveryService_Impl: windows_core::IUnknownImpl {
    fn GetGuideDataType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetGuideData(&self, pulcbbufferlen: *mut u32, pbbuffer: *mut u8, pulguidedatapercentageprogress: *mut u32) -> windows_core::Result<()>;
    fn RequestGuideDataUpdate(&self) -> windows_core::Result<()>;
    fn GetTuneXmlFromServiceIdx(&self, ul64serviceidx: u64) -> windows_core::Result<windows_core::BSTR>;
    fn GetServices(&self, pulcbbufferlen: *mut u32, pbbuffer: *mut u8) -> windows_core::Result<()>;
    fn GetServiceInfoFromTuneXml(&self, bstrtunexml: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
impl IBDA_GuideDataDeliveryService_Vtbl {
    pub const fn new<Identity: IBDA_GuideDataDeliveryService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGuideDataType<Identity: IBDA_GuideDataDeliveryService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguiddatatype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_GuideDataDeliveryService_Impl::GetGuideDataType(this) {
                    Ok(ok__) => {
                        pguiddatatype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGuideData<Identity: IBDA_GuideDataDeliveryService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcbbufferlen: *mut u32, pbbuffer: *mut u8, pulguidedatapercentageprogress: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_GuideDataDeliveryService_Impl::GetGuideData(this, core::mem::transmute_copy(&pulcbbufferlen), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pulguidedatapercentageprogress)).into()
            }
        }
        unsafe extern "system" fn RequestGuideDataUpdate<Identity: IBDA_GuideDataDeliveryService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_GuideDataDeliveryService_Impl::RequestGuideDataUpdate(this).into()
            }
        }
        unsafe extern "system" fn GetTuneXmlFromServiceIdx<Identity: IBDA_GuideDataDeliveryService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ul64serviceidx: u64, pbstrtunexml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_GuideDataDeliveryService_Impl::GetTuneXmlFromServiceIdx(this, core::mem::transmute_copy(&ul64serviceidx)) {
                    Ok(ok__) => {
                        pbstrtunexml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetServices<Identity: IBDA_GuideDataDeliveryService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcbbufferlen: *mut u32, pbbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_GuideDataDeliveryService_Impl::GetServices(this, core::mem::transmute_copy(&pulcbbufferlen), core::mem::transmute_copy(&pbbuffer)).into()
            }
        }
        unsafe extern "system" fn GetServiceInfoFromTuneXml<Identity: IBDA_GuideDataDeliveryService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtunexml: *mut core::ffi::c_void, pbstrservicedescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_GuideDataDeliveryService_Impl::GetServiceInfoFromTuneXml(this, core::mem::transmute(&bstrtunexml)) {
                    Ok(ok__) => {
                        pbstrservicedescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGuideDataType: GetGuideDataType::<Identity, OFFSET>,
            GetGuideData: GetGuideData::<Identity, OFFSET>,
            RequestGuideDataUpdate: RequestGuideDataUpdate::<Identity, OFFSET>,
            GetTuneXmlFromServiceIdx: GetTuneXmlFromServiceIdx::<Identity, OFFSET>,
            GetServices: GetServices::<Identity, OFFSET>,
            GetServiceInfoFromTuneXml: GetServiceInfoFromTuneXml::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_GuideDataDeliveryService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_GuideDataDeliveryService {}
windows_core::imp::define_interface!(IBDA_IPSinkControl, IBDA_IPSinkControl_Vtbl, 0x3f4dc8e2_4050_11d3_8f4b_00c04f7971e2);
windows_core::imp::interface_hierarchy!(IBDA_IPSinkControl, windows_core::IUnknown);
impl IBDA_IPSinkControl {
    pub unsafe fn GetMulticastList(&self, pulcbsize: *mut u32, pbbuffer: *mut *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMulticastList)(windows_core::Interface::as_raw(self), pulcbsize as _, pbbuffer as _) }
    }
    pub unsafe fn GetAdapterIPAddress(&self, pulcbsize: *mut u32, pbbuffer: *mut *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterIPAddress)(windows_core::Interface::as_raw(self), pulcbsize as _, pbbuffer as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_IPSinkControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMulticastList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetAdapterIPAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
pub trait IBDA_IPSinkControl_Impl: windows_core::IUnknownImpl {
    fn GetMulticastList(&self, pulcbsize: *mut u32, pbbuffer: *mut *mut u8) -> windows_core::Result<()>;
    fn GetAdapterIPAddress(&self, pulcbsize: *mut u32, pbbuffer: *mut *mut u8) -> windows_core::Result<()>;
}
impl IBDA_IPSinkControl_Vtbl {
    pub const fn new<Identity: IBDA_IPSinkControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMulticastList<Identity: IBDA_IPSinkControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcbsize: *mut u32, pbbuffer: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_IPSinkControl_Impl::GetMulticastList(this, core::mem::transmute_copy(&pulcbsize), core::mem::transmute_copy(&pbbuffer)).into()
            }
        }
        unsafe extern "system" fn GetAdapterIPAddress<Identity: IBDA_IPSinkControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcbsize: *mut u32, pbbuffer: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_IPSinkControl_Impl::GetAdapterIPAddress(this, core::mem::transmute_copy(&pulcbsize), core::mem::transmute_copy(&pbbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMulticastList: GetMulticastList::<Identity, OFFSET>,
            GetAdapterIPAddress: GetAdapterIPAddress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_IPSinkControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_IPSinkControl {}
windows_core::imp::define_interface!(IBDA_IPSinkInfo, IBDA_IPSinkInfo_Vtbl, 0xa750108f_492e_4d51_95f7_649b23ff7ad7);
windows_core::imp::interface_hierarchy!(IBDA_IPSinkInfo, windows_core::IUnknown);
impl IBDA_IPSinkInfo {
    pub unsafe fn get_MulticastList(&self, pulcbaddresses: *mut u32, ppbaddresslist: *mut *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_MulticastList)(windows_core::Interface::as_raw(self), pulcbaddresses as _, ppbaddresslist as _) }
    }
    pub unsafe fn get_AdapterIPAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AdapterIPAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_AdapterDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AdapterDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_IPSinkInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_MulticastList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub get_AdapterIPAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_AdapterDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBDA_IPSinkInfo_Impl: windows_core::IUnknownImpl {
    fn get_MulticastList(&self, pulcbaddresses: *mut u32, ppbaddresslist: *mut *mut u8) -> windows_core::Result<()>;
    fn get_AdapterIPAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_AdapterDescription(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IBDA_IPSinkInfo_Vtbl {
    pub const fn new<Identity: IBDA_IPSinkInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_MulticastList<Identity: IBDA_IPSinkInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcbaddresses: *mut u32, ppbaddresslist: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_IPSinkInfo_Impl::get_MulticastList(this, core::mem::transmute_copy(&pulcbaddresses), core::mem::transmute_copy(&ppbaddresslist)).into()
            }
        }
        unsafe extern "system" fn get_AdapterIPAddress<Identity: IBDA_IPSinkInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_IPSinkInfo_Impl::get_AdapterIPAddress(this) {
                    Ok(ok__) => {
                        pbstrbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AdapterDescription<Identity: IBDA_IPSinkInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_IPSinkInfo_Impl::get_AdapterDescription(this) {
                    Ok(ok__) => {
                        pbstrbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_MulticastList: get_MulticastList::<Identity, OFFSET>,
            get_AdapterIPAddress: get_AdapterIPAddress::<Identity, OFFSET>,
            get_AdapterDescription: get_AdapterDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_IPSinkInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_IPSinkInfo {}
windows_core::imp::define_interface!(IBDA_IPV4Filter, IBDA_IPV4Filter_Vtbl, 0x71985f44_1ca1_11d3_9cc8_00c04f7971e0);
windows_core::imp::interface_hierarchy!(IBDA_IPV4Filter, windows_core::IUnknown);
impl IBDA_IPV4Filter {
    pub unsafe fn GetMulticastListSize(&self, pulcbaddresses: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMulticastListSize)(windows_core::Interface::as_raw(self), pulcbaddresses as _) }
    }
    pub unsafe fn PutMulticastList(&self, ulcbaddresses: u32, paddresslist: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutMulticastList)(windows_core::Interface::as_raw(self), ulcbaddresses, paddresslist) }
    }
    pub unsafe fn GetMulticastList(&self, pulcbaddresses: *mut u32, paddresslist: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMulticastList)(windows_core::Interface::as_raw(self), pulcbaddresses as _, paddresslist as _) }
    }
    pub unsafe fn PutMulticastMode(&self, ulmodemask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutMulticastMode)(windows_core::Interface::as_raw(self), ulmodemask) }
    }
    pub unsafe fn GetMulticastMode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMulticastMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_IPV4Filter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMulticastListSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PutMulticastList: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub GetMulticastList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub PutMulticastMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMulticastMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IBDA_IPV4Filter_Impl: windows_core::IUnknownImpl {
    fn GetMulticastListSize(&self, pulcbaddresses: *mut u32) -> windows_core::Result<()>;
    fn PutMulticastList(&self, ulcbaddresses: u32, paddresslist: *const u8) -> windows_core::Result<()>;
    fn GetMulticastList(&self, pulcbaddresses: *mut u32, paddresslist: *mut u8) -> windows_core::Result<()>;
    fn PutMulticastMode(&self, ulmodemask: u32) -> windows_core::Result<()>;
    fn GetMulticastMode(&self) -> windows_core::Result<u32>;
}
impl IBDA_IPV4Filter_Vtbl {
    pub const fn new<Identity: IBDA_IPV4Filter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMulticastListSize<Identity: IBDA_IPV4Filter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcbaddresses: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_IPV4Filter_Impl::GetMulticastListSize(this, core::mem::transmute_copy(&pulcbaddresses)).into()
            }
        }
        unsafe extern "system" fn PutMulticastList<Identity: IBDA_IPV4Filter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcbaddresses: u32, paddresslist: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_IPV4Filter_Impl::PutMulticastList(this, core::mem::transmute_copy(&ulcbaddresses), core::mem::transmute_copy(&paddresslist)).into()
            }
        }
        unsafe extern "system" fn GetMulticastList<Identity: IBDA_IPV4Filter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcbaddresses: *mut u32, paddresslist: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_IPV4Filter_Impl::GetMulticastList(this, core::mem::transmute_copy(&pulcbaddresses), core::mem::transmute_copy(&paddresslist)).into()
            }
        }
        unsafe extern "system" fn PutMulticastMode<Identity: IBDA_IPV4Filter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmodemask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_IPV4Filter_Impl::PutMulticastMode(this, core::mem::transmute_copy(&ulmodemask)).into()
            }
        }
        unsafe extern "system" fn GetMulticastMode<Identity: IBDA_IPV4Filter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmodemask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_IPV4Filter_Impl::GetMulticastMode(this) {
                    Ok(ok__) => {
                        pulmodemask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMulticastListSize: GetMulticastListSize::<Identity, OFFSET>,
            PutMulticastList: PutMulticastList::<Identity, OFFSET>,
            GetMulticastList: GetMulticastList::<Identity, OFFSET>,
            PutMulticastMode: PutMulticastMode::<Identity, OFFSET>,
            GetMulticastMode: GetMulticastMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_IPV4Filter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_IPV4Filter {}
windows_core::imp::define_interface!(IBDA_IPV6Filter, IBDA_IPV6Filter_Vtbl, 0xe1785a74_2a23_4fb3_9245_a8f88017ef33);
windows_core::imp::interface_hierarchy!(IBDA_IPV6Filter, windows_core::IUnknown);
impl IBDA_IPV6Filter {
    pub unsafe fn GetMulticastListSize(&self, pulcbaddresses: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMulticastListSize)(windows_core::Interface::as_raw(self), pulcbaddresses as _) }
    }
    pub unsafe fn PutMulticastList(&self, ulcbaddresses: u32, paddresslist: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutMulticastList)(windows_core::Interface::as_raw(self), ulcbaddresses, paddresslist) }
    }
    pub unsafe fn GetMulticastList(&self, pulcbaddresses: *mut u32, paddresslist: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMulticastList)(windows_core::Interface::as_raw(self), pulcbaddresses as _, paddresslist as _) }
    }
    pub unsafe fn PutMulticastMode(&self, ulmodemask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutMulticastMode)(windows_core::Interface::as_raw(self), ulmodemask) }
    }
    pub unsafe fn GetMulticastMode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMulticastMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_IPV6Filter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMulticastListSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PutMulticastList: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub GetMulticastList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub PutMulticastMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMulticastMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IBDA_IPV6Filter_Impl: windows_core::IUnknownImpl {
    fn GetMulticastListSize(&self, pulcbaddresses: *mut u32) -> windows_core::Result<()>;
    fn PutMulticastList(&self, ulcbaddresses: u32, paddresslist: *const u8) -> windows_core::Result<()>;
    fn GetMulticastList(&self, pulcbaddresses: *mut u32, paddresslist: *mut u8) -> windows_core::Result<()>;
    fn PutMulticastMode(&self, ulmodemask: u32) -> windows_core::Result<()>;
    fn GetMulticastMode(&self) -> windows_core::Result<u32>;
}
impl IBDA_IPV6Filter_Vtbl {
    pub const fn new<Identity: IBDA_IPV6Filter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMulticastListSize<Identity: IBDA_IPV6Filter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcbaddresses: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_IPV6Filter_Impl::GetMulticastListSize(this, core::mem::transmute_copy(&pulcbaddresses)).into()
            }
        }
        unsafe extern "system" fn PutMulticastList<Identity: IBDA_IPV6Filter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcbaddresses: u32, paddresslist: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_IPV6Filter_Impl::PutMulticastList(this, core::mem::transmute_copy(&ulcbaddresses), core::mem::transmute_copy(&paddresslist)).into()
            }
        }
        unsafe extern "system" fn GetMulticastList<Identity: IBDA_IPV6Filter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcbaddresses: *mut u32, paddresslist: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_IPV6Filter_Impl::GetMulticastList(this, core::mem::transmute_copy(&pulcbaddresses), core::mem::transmute_copy(&paddresslist)).into()
            }
        }
        unsafe extern "system" fn PutMulticastMode<Identity: IBDA_IPV6Filter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmodemask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_IPV6Filter_Impl::PutMulticastMode(this, core::mem::transmute_copy(&ulmodemask)).into()
            }
        }
        unsafe extern "system" fn GetMulticastMode<Identity: IBDA_IPV6Filter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmodemask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_IPV6Filter_Impl::GetMulticastMode(this) {
                    Ok(ok__) => {
                        pulmodemask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMulticastListSize: GetMulticastListSize::<Identity, OFFSET>,
            PutMulticastList: PutMulticastList::<Identity, OFFSET>,
            GetMulticastList: GetMulticastList::<Identity, OFFSET>,
            PutMulticastMode: PutMulticastMode::<Identity, OFFSET>,
            GetMulticastMode: GetMulticastMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_IPV6Filter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_IPV6Filter {}
windows_core::imp::define_interface!(IBDA_ISDBConditionalAccess, IBDA_ISDBConditionalAccess_Vtbl, 0x5e68c627_16c2_4e6c_b1e2_d00170cdaa0f);
windows_core::imp::interface_hierarchy!(IBDA_ISDBConditionalAccess, windows_core::IUnknown);
impl IBDA_ISDBConditionalAccess {
    pub unsafe fn SetIsdbCasRequest(&self, ulrequestid: u32, ulcbrequestbufferlen: u32, pbrequestbuffer: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIsdbCasRequest)(windows_core::Interface::as_raw(self), ulrequestid, ulcbrequestbufferlen, pbrequestbuffer) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_ISDBConditionalAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIsdbCasRequest: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8) -> windows_core::HRESULT,
}
pub trait IBDA_ISDBConditionalAccess_Impl: windows_core::IUnknownImpl {
    fn SetIsdbCasRequest(&self, ulrequestid: u32, ulcbrequestbufferlen: u32, pbrequestbuffer: *const u8) -> windows_core::Result<()>;
}
impl IBDA_ISDBConditionalAccess_Vtbl {
    pub const fn new<Identity: IBDA_ISDBConditionalAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIsdbCasRequest<Identity: IBDA_ISDBConditionalAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrequestid: u32, ulcbrequestbufferlen: u32, pbrequestbuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_ISDBConditionalAccess_Impl::SetIsdbCasRequest(this, core::mem::transmute_copy(&ulrequestid), core::mem::transmute_copy(&ulcbrequestbufferlen), core::mem::transmute_copy(&pbrequestbuffer)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetIsdbCasRequest: SetIsdbCasRequest::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_ISDBConditionalAccess as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_ISDBConditionalAccess {}
windows_core::imp::define_interface!(IBDA_LNBInfo, IBDA_LNBInfo_Vtbl, 0x992cf102_49f9_4719_a664_c4f23e2408f4);
windows_core::imp::interface_hierarchy!(IBDA_LNBInfo, windows_core::IUnknown);
impl IBDA_LNBInfo {
    pub unsafe fn put_LocalOscilatorFrequencyLowBand(&self, ulloflow: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LocalOscilatorFrequencyLowBand)(windows_core::Interface::as_raw(self), ulloflow) }
    }
    pub unsafe fn get_LocalOscilatorFrequencyLowBand(&self, pulloflow: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_LocalOscilatorFrequencyLowBand)(windows_core::Interface::as_raw(self), pulloflow as _) }
    }
    pub unsafe fn put_LocalOscilatorFrequencyHighBand(&self, ullofhigh: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LocalOscilatorFrequencyHighBand)(windows_core::Interface::as_raw(self), ullofhigh) }
    }
    pub unsafe fn get_LocalOscilatorFrequencyHighBand(&self, pullofhigh: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_LocalOscilatorFrequencyHighBand)(windows_core::Interface::as_raw(self), pullofhigh as _) }
    }
    pub unsafe fn put_HighLowSwitchFrequency(&self, ulswitchfrequency: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HighLowSwitchFrequency)(windows_core::Interface::as_raw(self), ulswitchfrequency) }
    }
    pub unsafe fn get_HighLowSwitchFrequency(&self, pulswitchfrequency: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_HighLowSwitchFrequency)(windows_core::Interface::as_raw(self), pulswitchfrequency as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_LNBInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub put_LocalOscilatorFrequencyLowBand: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_LocalOscilatorFrequencyLowBand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_LocalOscilatorFrequencyHighBand: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_LocalOscilatorFrequencyHighBand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_HighLowSwitchFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_HighLowSwitchFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IBDA_LNBInfo_Impl: windows_core::IUnknownImpl {
    fn put_LocalOscilatorFrequencyLowBand(&self, ulloflow: u32) -> windows_core::Result<()>;
    fn get_LocalOscilatorFrequencyLowBand(&self, pulloflow: *mut u32) -> windows_core::Result<()>;
    fn put_LocalOscilatorFrequencyHighBand(&self, ullofhigh: u32) -> windows_core::Result<()>;
    fn get_LocalOscilatorFrequencyHighBand(&self, pullofhigh: *mut u32) -> windows_core::Result<()>;
    fn put_HighLowSwitchFrequency(&self, ulswitchfrequency: u32) -> windows_core::Result<()>;
    fn get_HighLowSwitchFrequency(&self, pulswitchfrequency: *mut u32) -> windows_core::Result<()>;
}
impl IBDA_LNBInfo_Vtbl {
    pub const fn new<Identity: IBDA_LNBInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_LocalOscilatorFrequencyLowBand<Identity: IBDA_LNBInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloflow: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_LNBInfo_Impl::put_LocalOscilatorFrequencyLowBand(this, core::mem::transmute_copy(&ulloflow)).into()
            }
        }
        unsafe extern "system" fn get_LocalOscilatorFrequencyLowBand<Identity: IBDA_LNBInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulloflow: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_LNBInfo_Impl::get_LocalOscilatorFrequencyLowBand(this, core::mem::transmute_copy(&pulloflow)).into()
            }
        }
        unsafe extern "system" fn put_LocalOscilatorFrequencyHighBand<Identity: IBDA_LNBInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullofhigh: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_LNBInfo_Impl::put_LocalOscilatorFrequencyHighBand(this, core::mem::transmute_copy(&ullofhigh)).into()
            }
        }
        unsafe extern "system" fn get_LocalOscilatorFrequencyHighBand<Identity: IBDA_LNBInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullofhigh: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_LNBInfo_Impl::get_LocalOscilatorFrequencyHighBand(this, core::mem::transmute_copy(&pullofhigh)).into()
            }
        }
        unsafe extern "system" fn put_HighLowSwitchFrequency<Identity: IBDA_LNBInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulswitchfrequency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_LNBInfo_Impl::put_HighLowSwitchFrequency(this, core::mem::transmute_copy(&ulswitchfrequency)).into()
            }
        }
        unsafe extern "system" fn get_HighLowSwitchFrequency<Identity: IBDA_LNBInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulswitchfrequency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_LNBInfo_Impl::get_HighLowSwitchFrequency(this, core::mem::transmute_copy(&pulswitchfrequency)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            put_LocalOscilatorFrequencyLowBand: put_LocalOscilatorFrequencyLowBand::<Identity, OFFSET>,
            get_LocalOscilatorFrequencyLowBand: get_LocalOscilatorFrequencyLowBand::<Identity, OFFSET>,
            put_LocalOscilatorFrequencyHighBand: put_LocalOscilatorFrequencyHighBand::<Identity, OFFSET>,
            get_LocalOscilatorFrequencyHighBand: get_LocalOscilatorFrequencyHighBand::<Identity, OFFSET>,
            put_HighLowSwitchFrequency: put_HighLowSwitchFrequency::<Identity, OFFSET>,
            get_HighLowSwitchFrequency: get_HighLowSwitchFrequency::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_LNBInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_LNBInfo {}
windows_core::imp::define_interface!(IBDA_MUX, IBDA_MUX_Vtbl, 0x942aafec_4c05_4c74_b8eb_8706c2a4943f);
windows_core::imp::interface_hierarchy!(IBDA_MUX, windows_core::IUnknown);
impl IBDA_MUX {
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn SetPidList(&self, ulpidlistcount: u32, pbpidlistbuffer: *const super::bdatypes::BDA_MUX_PIDLISTITEM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPidList)(windows_core::Interface::as_raw(self), ulpidlistcount, pbpidlistbuffer) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn GetPidList(&self, pulpidlistcount: *mut u32, pbpidlistbuffer: *mut super::bdatypes::BDA_MUX_PIDLISTITEM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPidList)(windows_core::Interface::as_raw(self), pulpidlistcount as _, pbpidlistbuffer as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_MUX_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_bdatypes")]
    pub SetPidList: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::bdatypes::BDA_MUX_PIDLISTITEM) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    SetPidList: usize,
    #[cfg(feature = "Win32_bdatypes")]
    pub GetPidList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut super::bdatypes::BDA_MUX_PIDLISTITEM) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    GetPidList: usize,
}
#[cfg(feature = "Win32_bdatypes")]
pub trait IBDA_MUX_Impl: windows_core::IUnknownImpl {
    fn SetPidList(&self, ulpidlistcount: u32, pbpidlistbuffer: *const super::bdatypes::BDA_MUX_PIDLISTITEM) -> windows_core::Result<()>;
    fn GetPidList(&self, pulpidlistcount: *mut u32, pbpidlistbuffer: *mut super::bdatypes::BDA_MUX_PIDLISTITEM) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_bdatypes")]
impl IBDA_MUX_Vtbl {
    pub const fn new<Identity: IBDA_MUX_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPidList<Identity: IBDA_MUX_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulpidlistcount: u32, pbpidlistbuffer: *const super::bdatypes::BDA_MUX_PIDLISTITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_MUX_Impl::SetPidList(this, core::mem::transmute_copy(&ulpidlistcount), core::mem::transmute_copy(&pbpidlistbuffer)).into()
            }
        }
        unsafe extern "system" fn GetPidList<Identity: IBDA_MUX_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpidlistcount: *mut u32, pbpidlistbuffer: *mut super::bdatypes::BDA_MUX_PIDLISTITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_MUX_Impl::GetPidList(this, core::mem::transmute_copy(&pulpidlistcount), core::mem::transmute_copy(&pbpidlistbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPidList: SetPidList::<Identity, OFFSET>,
            GetPidList: GetPidList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_MUX as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_bdatypes")]
impl windows_core::RuntimeName for IBDA_MUX {}
windows_core::imp::define_interface!(IBDA_NameValueService, IBDA_NameValueService_Vtbl, 0x7f0b3150_7b81_4ad4_98e3_7e9097094301);
windows_core::imp::interface_hierarchy!(IBDA_NameValueService, windows_core::IUnknown);
impl IBDA_NameValueService {
    pub unsafe fn GetValueNameByIndex(&self, ulindex: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValueNameByIndex)(windows_core::Interface::as_raw(self), ulindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetValue(&self, bstrname: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrlanguage), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetValue(&self, uldialogrequest: u32, bstrlanguage: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR, ulreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), uldialogrequest, core::mem::transmute_copy(bstrlanguage), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrvalue), ulreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_NameValueService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetValueNameByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IBDA_NameValueService_Impl: windows_core::IUnknownImpl {
    fn GetValueNameByIndex(&self, ulindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetValue(&self, bstrname: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetValue(&self, uldialogrequest: u32, bstrlanguage: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR, ulreserved: u32) -> windows_core::Result<()>;
}
impl IBDA_NameValueService_Vtbl {
    pub const fn new<Identity: IBDA_NameValueService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetValueNameByIndex<Identity: IBDA_NameValueService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_NameValueService_Impl::GetValueNameByIndex(this, core::mem::transmute_copy(&ulindex)) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: IBDA_NameValueService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrlanguage: *mut core::ffi::c_void, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_NameValueService_Impl::GetValue(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrlanguage)) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IBDA_NameValueService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uldialogrequest: u32, bstrlanguage: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrvalue: *mut core::ffi::c_void, ulreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_NameValueService_Impl::SetValue(this, core::mem::transmute_copy(&uldialogrequest), core::mem::transmute(&bstrlanguage), core::mem::transmute(&bstrname), core::mem::transmute(&bstrvalue), core::mem::transmute_copy(&ulreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetValueNameByIndex: GetValueNameByIndex::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_NameValueService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_NameValueService {}
windows_core::imp::define_interface!(IBDA_NetworkProvider, IBDA_NetworkProvider_Vtbl, 0xfd501041_8ebe_11ce_8183_00aa00577da2);
windows_core::imp::interface_hierarchy!(IBDA_NetworkProvider, windows_core::IUnknown);
impl IBDA_NetworkProvider {
    pub unsafe fn PutSignalSource(&self, ulsignalsource: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutSignalSource)(windows_core::Interface::as_raw(self), ulsignalsource) }
    }
    pub unsafe fn GetSignalSource(&self, pulsignalsource: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSignalSource)(windows_core::Interface::as_raw(self), pulsignalsource as _) }
    }
    pub unsafe fn GetNetworkType(&self, pguidnetworktype: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNetworkType)(windows_core::Interface::as_raw(self), pguidnetworktype as _) }
    }
    pub unsafe fn PutTuningSpace(&self, guidtuningspace: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutTuningSpace)(windows_core::Interface::as_raw(self), guidtuningspace) }
    }
    pub unsafe fn GetTuningSpace(&self, pguidtuingspace: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTuningSpace)(windows_core::Interface::as_raw(self), pguidtuingspace as _) }
    }
    pub unsafe fn RegisterDeviceFilter<P0>(&self, punkfiltercontrol: P0, ppvregisitrationcontext: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterDeviceFilter)(windows_core::Interface::as_raw(self), punkfiltercontrol.param().abi(), ppvregisitrationcontext as _) }
    }
    pub unsafe fn UnRegisterDeviceFilter(&self, pvregistrationcontext: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterDeviceFilter)(windows_core::Interface::as_raw(self), pvregistrationcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_NetworkProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PutSignalSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetSignalSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNetworkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub PutTuningSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetTuningSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RegisterDeviceFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnRegisterDeviceFilter: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IBDA_NetworkProvider_Impl: windows_core::IUnknownImpl {
    fn PutSignalSource(&self, ulsignalsource: u32) -> windows_core::Result<()>;
    fn GetSignalSource(&self, pulsignalsource: *mut u32) -> windows_core::Result<()>;
    fn GetNetworkType(&self, pguidnetworktype: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn PutTuningSpace(&self, guidtuningspace: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetTuningSpace(&self, pguidtuingspace: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterDeviceFilter(&self, punkfiltercontrol: windows_core::Ref<windows_core::IUnknown>, ppvregisitrationcontext: *mut u32) -> windows_core::Result<()>;
    fn UnRegisterDeviceFilter(&self, pvregistrationcontext: u32) -> windows_core::Result<()>;
}
impl IBDA_NetworkProvider_Vtbl {
    pub const fn new<Identity: IBDA_NetworkProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PutSignalSource<Identity: IBDA_NetworkProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsignalsource: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_NetworkProvider_Impl::PutSignalSource(this, core::mem::transmute_copy(&ulsignalsource)).into()
            }
        }
        unsafe extern "system" fn GetSignalSource<Identity: IBDA_NetworkProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulsignalsource: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_NetworkProvider_Impl::GetSignalSource(this, core::mem::transmute_copy(&pulsignalsource)).into()
            }
        }
        unsafe extern "system" fn GetNetworkType<Identity: IBDA_NetworkProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidnetworktype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_NetworkProvider_Impl::GetNetworkType(this, core::mem::transmute_copy(&pguidnetworktype)).into()
            }
        }
        unsafe extern "system" fn PutTuningSpace<Identity: IBDA_NetworkProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtuningspace: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_NetworkProvider_Impl::PutTuningSpace(this, core::mem::transmute_copy(&guidtuningspace)).into()
            }
        }
        unsafe extern "system" fn GetTuningSpace<Identity: IBDA_NetworkProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidtuingspace: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_NetworkProvider_Impl::GetTuningSpace(this, core::mem::transmute_copy(&pguidtuingspace)).into()
            }
        }
        unsafe extern "system" fn RegisterDeviceFilter<Identity: IBDA_NetworkProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkfiltercontrol: *mut core::ffi::c_void, ppvregisitrationcontext: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_NetworkProvider_Impl::RegisterDeviceFilter(this, core::mem::transmute_copy(&punkfiltercontrol), core::mem::transmute_copy(&ppvregisitrationcontext)).into()
            }
        }
        unsafe extern "system" fn UnRegisterDeviceFilter<Identity: IBDA_NetworkProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvregistrationcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_NetworkProvider_Impl::UnRegisterDeviceFilter(this, core::mem::transmute_copy(&pvregistrationcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutSignalSource: PutSignalSource::<Identity, OFFSET>,
            GetSignalSource: GetSignalSource::<Identity, OFFSET>,
            GetNetworkType: GetNetworkType::<Identity, OFFSET>,
            PutTuningSpace: PutTuningSpace::<Identity, OFFSET>,
            GetTuningSpace: GetTuningSpace::<Identity, OFFSET>,
            RegisterDeviceFilter: RegisterDeviceFilter::<Identity, OFFSET>,
            UnRegisterDeviceFilter: UnRegisterDeviceFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_NetworkProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_NetworkProvider {}
windows_core::imp::define_interface!(IBDA_NullTransform, IBDA_NullTransform_Vtbl, 0xddf15b0d_bd25_11d2_9ca0_00c04f7971e0);
windows_core::imp::interface_hierarchy!(IBDA_NullTransform, windows_core::IUnknown);
impl IBDA_NullTransform {
    pub unsafe fn Start(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_NullTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBDA_NullTransform_Impl: windows_core::IUnknownImpl {
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
}
impl IBDA_NullTransform_Vtbl {
    pub const fn new<Identity: IBDA_NullTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IBDA_NullTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_NullTransform_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IBDA_NullTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_NullTransform_Impl::Stop(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Start: Start::<Identity, OFFSET>, Stop: Stop::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_NullTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_NullTransform {}
windows_core::imp::define_interface!(IBDA_PinControl, IBDA_PinControl_Vtbl, 0x0ded49d5_a8b7_4d5d_97a1_12b0c195874d);
windows_core::imp::interface_hierarchy!(IBDA_PinControl, windows_core::IUnknown);
impl IBDA_PinControl {
    pub unsafe fn GetPinID(&self, pulpinid: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPinID)(windows_core::Interface::as_raw(self), pulpinid as _) }
    }
    pub unsafe fn GetPinType(&self, pulpintype: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPinType)(windows_core::Interface::as_raw(self), pulpintype as _) }
    }
    pub unsafe fn RegistrationContext(&self, pulregistrationctx: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegistrationContext)(windows_core::Interface::as_raw(self), pulregistrationctx as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_PinControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPinID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPinType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RegistrationContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IBDA_PinControl_Impl: windows_core::IUnknownImpl {
    fn GetPinID(&self, pulpinid: *mut u32) -> windows_core::Result<()>;
    fn GetPinType(&self, pulpintype: *mut u32) -> windows_core::Result<()>;
    fn RegistrationContext(&self, pulregistrationctx: *mut u32) -> windows_core::Result<()>;
}
impl IBDA_PinControl_Vtbl {
    pub const fn new<Identity: IBDA_PinControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPinID<Identity: IBDA_PinControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpinid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_PinControl_Impl::GetPinID(this, core::mem::transmute_copy(&pulpinid)).into()
            }
        }
        unsafe extern "system" fn GetPinType<Identity: IBDA_PinControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpintype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_PinControl_Impl::GetPinType(this, core::mem::transmute_copy(&pulpintype)).into()
            }
        }
        unsafe extern "system" fn RegistrationContext<Identity: IBDA_PinControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulregistrationctx: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_PinControl_Impl::RegistrationContext(this, core::mem::transmute_copy(&pulregistrationctx)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPinID: GetPinID::<Identity, OFFSET>,
            GetPinType: GetPinType::<Identity, OFFSET>,
            RegistrationContext: RegistrationContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_PinControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_PinControl {}
windows_core::imp::define_interface!(IBDA_SignalProperties, IBDA_SignalProperties_Vtbl, 0xd2f1644b_b409_11d2_bc69_00a0c9ee9e16);
windows_core::imp::interface_hierarchy!(IBDA_SignalProperties, windows_core::IUnknown);
impl IBDA_SignalProperties {
    pub unsafe fn PutNetworkType(&self, guidnetworktype: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutNetworkType)(windows_core::Interface::as_raw(self), guidnetworktype) }
    }
    pub unsafe fn GetNetworkType(&self, pguidnetworktype: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNetworkType)(windows_core::Interface::as_raw(self), pguidnetworktype as _) }
    }
    pub unsafe fn PutSignalSource(&self, ulsignalsource: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutSignalSource)(windows_core::Interface::as_raw(self), ulsignalsource) }
    }
    pub unsafe fn GetSignalSource(&self, pulsignalsource: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSignalSource)(windows_core::Interface::as_raw(self), pulsignalsource as _) }
    }
    pub unsafe fn PutTuningSpace(&self, guidtuningspace: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutTuningSpace)(windows_core::Interface::as_raw(self), guidtuningspace) }
    }
    pub unsafe fn GetTuningSpace(&self, pguidtuingspace: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTuningSpace)(windows_core::Interface::as_raw(self), pguidtuingspace as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_SignalProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PutNetworkType: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetNetworkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub PutSignalSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetSignalSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PutTuningSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetTuningSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IBDA_SignalProperties_Impl: windows_core::IUnknownImpl {
    fn PutNetworkType(&self, guidnetworktype: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetNetworkType(&self, pguidnetworktype: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn PutSignalSource(&self, ulsignalsource: u32) -> windows_core::Result<()>;
    fn GetSignalSource(&self, pulsignalsource: *mut u32) -> windows_core::Result<()>;
    fn PutTuningSpace(&self, guidtuningspace: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetTuningSpace(&self, pguidtuingspace: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl IBDA_SignalProperties_Vtbl {
    pub const fn new<Identity: IBDA_SignalProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PutNetworkType<Identity: IBDA_SignalProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidnetworktype: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalProperties_Impl::PutNetworkType(this, core::mem::transmute_copy(&guidnetworktype)).into()
            }
        }
        unsafe extern "system" fn GetNetworkType<Identity: IBDA_SignalProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidnetworktype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalProperties_Impl::GetNetworkType(this, core::mem::transmute_copy(&pguidnetworktype)).into()
            }
        }
        unsafe extern "system" fn PutSignalSource<Identity: IBDA_SignalProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsignalsource: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalProperties_Impl::PutSignalSource(this, core::mem::transmute_copy(&ulsignalsource)).into()
            }
        }
        unsafe extern "system" fn GetSignalSource<Identity: IBDA_SignalProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulsignalsource: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalProperties_Impl::GetSignalSource(this, core::mem::transmute_copy(&pulsignalsource)).into()
            }
        }
        unsafe extern "system" fn PutTuningSpace<Identity: IBDA_SignalProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtuningspace: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalProperties_Impl::PutTuningSpace(this, core::mem::transmute_copy(&guidtuningspace)).into()
            }
        }
        unsafe extern "system" fn GetTuningSpace<Identity: IBDA_SignalProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidtuingspace: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalProperties_Impl::GetTuningSpace(this, core::mem::transmute_copy(&pguidtuingspace)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutNetworkType: PutNetworkType::<Identity, OFFSET>,
            GetNetworkType: GetNetworkType::<Identity, OFFSET>,
            PutSignalSource: PutSignalSource::<Identity, OFFSET>,
            GetSignalSource: GetSignalSource::<Identity, OFFSET>,
            PutTuningSpace: PutTuningSpace::<Identity, OFFSET>,
            GetTuningSpace: GetTuningSpace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_SignalProperties as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_SignalProperties {}
windows_core::imp::define_interface!(IBDA_SignalStatistics, IBDA_SignalStatistics_Vtbl, 0x1347d106_cf3a_428a_a5cb_ac0d9a2a4338);
windows_core::imp::interface_hierarchy!(IBDA_SignalStatistics, windows_core::IUnknown);
impl IBDA_SignalStatistics {
    pub unsafe fn put_SignalStrength(&self, ldbstrength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SignalStrength)(windows_core::Interface::as_raw(self), ldbstrength) }
    }
    pub unsafe fn get_SignalStrength(&self, pldbstrength: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SignalStrength)(windows_core::Interface::as_raw(self), pldbstrength as _) }
    }
    pub unsafe fn put_SignalQuality(&self, lpercentquality: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SignalQuality)(windows_core::Interface::as_raw(self), lpercentquality) }
    }
    pub unsafe fn get_SignalQuality(&self, plpercentquality: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SignalQuality)(windows_core::Interface::as_raw(self), plpercentquality as _) }
    }
    pub unsafe fn put_SignalPresent(&self, fpresent: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SignalPresent)(windows_core::Interface::as_raw(self), fpresent) }
    }
    pub unsafe fn get_SignalPresent(&self, pfpresent: *mut bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SignalPresent)(windows_core::Interface::as_raw(self), pfpresent as _) }
    }
    pub unsafe fn put_SignalLocked(&self, flocked: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SignalLocked)(windows_core::Interface::as_raw(self), flocked) }
    }
    pub unsafe fn get_SignalLocked(&self, pflocked: *mut bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SignalLocked)(windows_core::Interface::as_raw(self), pflocked as _) }
    }
    pub unsafe fn put_SampleTime(&self, lmssampletime: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SampleTime)(windows_core::Interface::as_raw(self), lmssampletime) }
    }
    pub unsafe fn get_SampleTime(&self, plmssampletime: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_SampleTime)(windows_core::Interface::as_raw(self), plmssampletime as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_SignalStatistics_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub put_SignalStrength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_SignalStrength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SignalQuality: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_SignalQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SignalPresent: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub get_SignalPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub put_SignalLocked: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub get_SignalLocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub put_SampleTime: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_SampleTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IBDA_SignalStatistics_Impl: windows_core::IUnknownImpl {
    fn put_SignalStrength(&self, ldbstrength: i32) -> windows_core::Result<()>;
    fn get_SignalStrength(&self, pldbstrength: *mut i32) -> windows_core::Result<()>;
    fn put_SignalQuality(&self, lpercentquality: i32) -> windows_core::Result<()>;
    fn get_SignalQuality(&self, plpercentquality: *mut i32) -> windows_core::Result<()>;
    fn put_SignalPresent(&self, fpresent: bool) -> windows_core::Result<()>;
    fn get_SignalPresent(&self, pfpresent: *mut bool) -> windows_core::Result<()>;
    fn put_SignalLocked(&self, flocked: bool) -> windows_core::Result<()>;
    fn get_SignalLocked(&self, pflocked: *mut bool) -> windows_core::Result<()>;
    fn put_SampleTime(&self, lmssampletime: i32) -> windows_core::Result<()>;
    fn get_SampleTime(&self, plmssampletime: *mut i32) -> windows_core::Result<()>;
}
impl IBDA_SignalStatistics_Vtbl {
    pub const fn new<Identity: IBDA_SignalStatistics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_SignalStrength<Identity: IBDA_SignalStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldbstrength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalStatistics_Impl::put_SignalStrength(this, core::mem::transmute_copy(&ldbstrength)).into()
            }
        }
        unsafe extern "system" fn get_SignalStrength<Identity: IBDA_SignalStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldbstrength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalStatistics_Impl::get_SignalStrength(this, core::mem::transmute_copy(&pldbstrength)).into()
            }
        }
        unsafe extern "system" fn put_SignalQuality<Identity: IBDA_SignalStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpercentquality: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalStatistics_Impl::put_SignalQuality(this, core::mem::transmute_copy(&lpercentquality)).into()
            }
        }
        unsafe extern "system" fn get_SignalQuality<Identity: IBDA_SignalStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpercentquality: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalStatistics_Impl::get_SignalQuality(this, core::mem::transmute_copy(&plpercentquality)).into()
            }
        }
        unsafe extern "system" fn put_SignalPresent<Identity: IBDA_SignalStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpresent: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalStatistics_Impl::put_SignalPresent(this, core::mem::transmute_copy(&fpresent)).into()
            }
        }
        unsafe extern "system" fn get_SignalPresent<Identity: IBDA_SignalStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfpresent: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalStatistics_Impl::get_SignalPresent(this, core::mem::transmute_copy(&pfpresent)).into()
            }
        }
        unsafe extern "system" fn put_SignalLocked<Identity: IBDA_SignalStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flocked: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalStatistics_Impl::put_SignalLocked(this, core::mem::transmute_copy(&flocked)).into()
            }
        }
        unsafe extern "system" fn get_SignalLocked<Identity: IBDA_SignalStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflocked: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalStatistics_Impl::get_SignalLocked(this, core::mem::transmute_copy(&pflocked)).into()
            }
        }
        unsafe extern "system" fn put_SampleTime<Identity: IBDA_SignalStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmssampletime: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalStatistics_Impl::put_SampleTime(this, core::mem::transmute_copy(&lmssampletime)).into()
            }
        }
        unsafe extern "system" fn get_SampleTime<Identity: IBDA_SignalStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmssampletime: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_SignalStatistics_Impl::get_SampleTime(this, core::mem::transmute_copy(&plmssampletime)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            put_SignalStrength: put_SignalStrength::<Identity, OFFSET>,
            get_SignalStrength: get_SignalStrength::<Identity, OFFSET>,
            put_SignalQuality: put_SignalQuality::<Identity, OFFSET>,
            get_SignalQuality: get_SignalQuality::<Identity, OFFSET>,
            put_SignalPresent: put_SignalPresent::<Identity, OFFSET>,
            get_SignalPresent: get_SignalPresent::<Identity, OFFSET>,
            put_SignalLocked: put_SignalLocked::<Identity, OFFSET>,
            get_SignalLocked: get_SignalLocked::<Identity, OFFSET>,
            put_SampleTime: put_SampleTime::<Identity, OFFSET>,
            get_SampleTime: get_SampleTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_SignalStatistics as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_SignalStatistics {}
windows_core::imp::define_interface!(IBDA_Topology, IBDA_Topology_Vtbl, 0x79b56888_7fea_4690_b45d_38fd3c7849be);
windows_core::imp::interface_hierarchy!(IBDA_Topology, windows_core::IUnknown);
impl IBDA_Topology {
    pub unsafe fn GetNodeTypes(&self, pulcnodetypes: *mut u32, ulcnodetypesmax: u32, rgulnodetypes: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNodeTypes)(windows_core::Interface::as_raw(self), pulcnodetypes as _, ulcnodetypesmax, rgulnodetypes as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn GetNodeDescriptors(&self, ulcnodedescriptors: *mut u32, ulcnodedescriptorsmax: u32, rgnodedescriptors: *mut super::bdatypes::BDANODE_DESCRIPTOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNodeDescriptors)(windows_core::Interface::as_raw(self), ulcnodedescriptors as _, ulcnodedescriptorsmax, rgnodedescriptors as _) }
    }
    pub unsafe fn GetNodeInterfaces(&self, ulnodetype: u32, pulcinterfaces: *mut u32, ulcinterfacesmax: u32, rgguidinterfaces: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNodeInterfaces)(windows_core::Interface::as_raw(self), ulnodetype, pulcinterfaces as _, ulcinterfacesmax, rgguidinterfaces as _) }
    }
    pub unsafe fn GetPinTypes(&self, pulcpintypes: *mut u32, ulcpintypesmax: u32, rgulpintypes: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPinTypes)(windows_core::Interface::as_raw(self), pulcpintypes as _, ulcpintypesmax, rgulpintypes as _) }
    }
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn GetTemplateConnections(&self, pulcconnections: *mut u32, ulcconnectionsmax: u32, rgconnections: *mut super::bdatypes::BDA_TEMPLATE_CONNECTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTemplateConnections)(windows_core::Interface::as_raw(self), pulcconnections as _, ulcconnectionsmax, rgconnections as _) }
    }
    pub unsafe fn CreatePin(&self, ulpintype: u32, pulpinid: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreatePin)(windows_core::Interface::as_raw(self), ulpintype, pulpinid as _) }
    }
    pub unsafe fn DeletePin(&self, ulpinid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeletePin)(windows_core::Interface::as_raw(self), ulpinid) }
    }
    #[cfg(feature = "Win32_strmif")]
    pub unsafe fn SetMediaType(&self, ulpinid: u32, pmediatype: *const super::strmif::AM_MEDIA_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMediaType)(windows_core::Interface::as_raw(self), ulpinid, core::mem::transmute(pmediatype)) }
    }
    #[cfg(feature = "Win32_strmif")]
    pub unsafe fn SetMedium(&self, ulpinid: u32, pmedium: *const super::strmif::REGPINMEDIUM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMedium)(windows_core::Interface::as_raw(self), ulpinid, pmedium) }
    }
    pub unsafe fn CreateTopology(&self, ulinputpinid: u32, uloutputpinid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateTopology)(windows_core::Interface::as_raw(self), ulinputpinid, uloutputpinid) }
    }
    pub unsafe fn GetControlNode(&self, ulinputpinid: u32, uloutputpinid: u32, ulnodetype: u32, ppcontrolnode: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetControlNode)(windows_core::Interface::as_raw(self), ulinputpinid, uloutputpinid, ulnodetype, core::mem::transmute(ppcontrolnode)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_Topology_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNodeTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_bdatypes")]
    pub GetNodeDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut super::bdatypes::BDANODE_DESCRIPTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    GetNodeDescriptors: usize,
    pub GetNodeInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetPinTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_bdatypes")]
    pub GetTemplateConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut super::bdatypes::BDA_TEMPLATE_CONNECTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    GetTemplateConnections: usize,
    pub CreatePin: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub DeletePin: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_strmif")]
    pub SetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::strmif::AM_MEDIA_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_strmif"))]
    SetMediaType: usize,
    #[cfg(feature = "Win32_strmif")]
    pub SetMedium: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::strmif::REGPINMEDIUM) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_strmif"))]
    SetMedium: usize,
    pub CreateTopology: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetControlNode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bdatypes", feature = "Win32_strmif"))]
pub trait IBDA_Topology_Impl: windows_core::IUnknownImpl {
    fn GetNodeTypes(&self, pulcnodetypes: *mut u32, ulcnodetypesmax: u32, rgulnodetypes: *mut u32) -> windows_core::Result<()>;
    fn GetNodeDescriptors(&self, ulcnodedescriptors: *mut u32, ulcnodedescriptorsmax: u32, rgnodedescriptors: *mut super::bdatypes::BDANODE_DESCRIPTOR) -> windows_core::Result<()>;
    fn GetNodeInterfaces(&self, ulnodetype: u32, pulcinterfaces: *mut u32, ulcinterfacesmax: u32, rgguidinterfaces: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetPinTypes(&self, pulcpintypes: *mut u32, ulcpintypesmax: u32, rgulpintypes: *mut u32) -> windows_core::Result<()>;
    fn GetTemplateConnections(&self, pulcconnections: *mut u32, ulcconnectionsmax: u32, rgconnections: *mut super::bdatypes::BDA_TEMPLATE_CONNECTION) -> windows_core::Result<()>;
    fn CreatePin(&self, ulpintype: u32, pulpinid: *mut u32) -> windows_core::Result<()>;
    fn DeletePin(&self, ulpinid: u32) -> windows_core::Result<()>;
    fn SetMediaType(&self, ulpinid: u32, pmediatype: *const super::strmif::AM_MEDIA_TYPE) -> windows_core::Result<()>;
    fn SetMedium(&self, ulpinid: u32, pmedium: *const super::strmif::REGPINMEDIUM) -> windows_core::Result<()>;
    fn CreateTopology(&self, ulinputpinid: u32, uloutputpinid: u32) -> windows_core::Result<()>;
    fn GetControlNode(&self, ulinputpinid: u32, uloutputpinid: u32, ulnodetype: u32, ppcontrolnode: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_bdatypes", feature = "Win32_strmif"))]
impl IBDA_Topology_Vtbl {
    pub const fn new<Identity: IBDA_Topology_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNodeTypes<Identity: IBDA_Topology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcnodetypes: *mut u32, ulcnodetypesmax: u32, rgulnodetypes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Topology_Impl::GetNodeTypes(this, core::mem::transmute_copy(&pulcnodetypes), core::mem::transmute_copy(&ulcnodetypesmax), core::mem::transmute_copy(&rgulnodetypes)).into()
            }
        }
        unsafe extern "system" fn GetNodeDescriptors<Identity: IBDA_Topology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcnodedescriptors: *mut u32, ulcnodedescriptorsmax: u32, rgnodedescriptors: *mut super::bdatypes::BDANODE_DESCRIPTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Topology_Impl::GetNodeDescriptors(this, core::mem::transmute_copy(&ulcnodedescriptors), core::mem::transmute_copy(&ulcnodedescriptorsmax), core::mem::transmute_copy(&rgnodedescriptors)).into()
            }
        }
        unsafe extern "system" fn GetNodeInterfaces<Identity: IBDA_Topology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulnodetype: u32, pulcinterfaces: *mut u32, ulcinterfacesmax: u32, rgguidinterfaces: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Topology_Impl::GetNodeInterfaces(this, core::mem::transmute_copy(&ulnodetype), core::mem::transmute_copy(&pulcinterfaces), core::mem::transmute_copy(&ulcinterfacesmax), core::mem::transmute_copy(&rgguidinterfaces)).into()
            }
        }
        unsafe extern "system" fn GetPinTypes<Identity: IBDA_Topology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcpintypes: *mut u32, ulcpintypesmax: u32, rgulpintypes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Topology_Impl::GetPinTypes(this, core::mem::transmute_copy(&pulcpintypes), core::mem::transmute_copy(&ulcpintypesmax), core::mem::transmute_copy(&rgulpintypes)).into()
            }
        }
        unsafe extern "system" fn GetTemplateConnections<Identity: IBDA_Topology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcconnections: *mut u32, ulcconnectionsmax: u32, rgconnections: *mut super::bdatypes::BDA_TEMPLATE_CONNECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Topology_Impl::GetTemplateConnections(this, core::mem::transmute_copy(&pulcconnections), core::mem::transmute_copy(&ulcconnectionsmax), core::mem::transmute_copy(&rgconnections)).into()
            }
        }
        unsafe extern "system" fn CreatePin<Identity: IBDA_Topology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulpintype: u32, pulpinid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Topology_Impl::CreatePin(this, core::mem::transmute_copy(&ulpintype), core::mem::transmute_copy(&pulpinid)).into()
            }
        }
        unsafe extern "system" fn DeletePin<Identity: IBDA_Topology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulpinid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Topology_Impl::DeletePin(this, core::mem::transmute_copy(&ulpinid)).into()
            }
        }
        unsafe extern "system" fn SetMediaType<Identity: IBDA_Topology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulpinid: u32, pmediatype: *const super::strmif::AM_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Topology_Impl::SetMediaType(this, core::mem::transmute_copy(&ulpinid), core::mem::transmute_copy(&pmediatype)).into()
            }
        }
        unsafe extern "system" fn SetMedium<Identity: IBDA_Topology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulpinid: u32, pmedium: *const super::strmif::REGPINMEDIUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Topology_Impl::SetMedium(this, core::mem::transmute_copy(&ulpinid), core::mem::transmute_copy(&pmedium)).into()
            }
        }
        unsafe extern "system" fn CreateTopology<Identity: IBDA_Topology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulinputpinid: u32, uloutputpinid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Topology_Impl::CreateTopology(this, core::mem::transmute_copy(&ulinputpinid), core::mem::transmute_copy(&uloutputpinid)).into()
            }
        }
        unsafe extern "system" fn GetControlNode<Identity: IBDA_Topology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulinputpinid: u32, uloutputpinid: u32, ulnodetype: u32, ppcontrolnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_Topology_Impl::GetControlNode(this, core::mem::transmute_copy(&ulinputpinid), core::mem::transmute_copy(&uloutputpinid), core::mem::transmute_copy(&ulnodetype), core::mem::transmute_copy(&ppcontrolnode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNodeTypes: GetNodeTypes::<Identity, OFFSET>,
            GetNodeDescriptors: GetNodeDescriptors::<Identity, OFFSET>,
            GetNodeInterfaces: GetNodeInterfaces::<Identity, OFFSET>,
            GetPinTypes: GetPinTypes::<Identity, OFFSET>,
            GetTemplateConnections: GetTemplateConnections::<Identity, OFFSET>,
            CreatePin: CreatePin::<Identity, OFFSET>,
            DeletePin: DeletePin::<Identity, OFFSET>,
            SetMediaType: SetMediaType::<Identity, OFFSET>,
            SetMedium: SetMedium::<Identity, OFFSET>,
            CreateTopology: CreateTopology::<Identity, OFFSET>,
            GetControlNode: GetControlNode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_Topology as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bdatypes", feature = "Win32_strmif"))]
impl windows_core::RuntimeName for IBDA_Topology {}
windows_core::imp::define_interface!(IBDA_TransportStreamInfo, IBDA_TransportStreamInfo_Vtbl, 0x8e882535_5f86_47ab_86cf_c281a72a0549);
windows_core::imp::interface_hierarchy!(IBDA_TransportStreamInfo, windows_core::IUnknown);
impl IBDA_TransportStreamInfo {
    pub unsafe fn get_PatTableTickCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PatTableTickCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_TransportStreamInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_PatTableTickCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IBDA_TransportStreamInfo_Impl: windows_core::IUnknownImpl {
    fn get_PatTableTickCount(&self) -> windows_core::Result<u32>;
}
impl IBDA_TransportStreamInfo_Vtbl {
    pub const fn new<Identity: IBDA_TransportStreamInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_PatTableTickCount<Identity: IBDA_TransportStreamInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppattickcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_TransportStreamInfo_Impl::get_PatTableTickCount(this) {
                    Ok(ok__) => {
                        ppattickcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), get_PatTableTickCount: get_PatTableTickCount::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_TransportStreamInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_TransportStreamInfo {}
windows_core::imp::define_interface!(IBDA_TransportStreamSelector, IBDA_TransportStreamSelector_Vtbl, 0x1dcfafe9_b45e_41b3_bb2a_561eb129ae98);
windows_core::imp::interface_hierarchy!(IBDA_TransportStreamSelector, windows_core::IUnknown);
impl IBDA_TransportStreamSelector {
    pub unsafe fn SetTSID(&self, ustsid: u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTSID)(windows_core::Interface::as_raw(self), ustsid) }
    }
    pub unsafe fn GetTSInformation(&self, pultsinformationbufferlen: *mut u32, pbtsinformationbuffer: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTSInformation)(windows_core::Interface::as_raw(self), pultsinformationbufferlen as _, pbtsinformationbuffer as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_TransportStreamSelector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTSID: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetTSInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u8) -> windows_core::HRESULT,
}
pub trait IBDA_TransportStreamSelector_Impl: windows_core::IUnknownImpl {
    fn SetTSID(&self, ustsid: u16) -> windows_core::Result<()>;
    fn GetTSInformation(&self, pultsinformationbufferlen: *mut u32, pbtsinformationbuffer: *mut u8) -> windows_core::Result<()>;
}
impl IBDA_TransportStreamSelector_Vtbl {
    pub const fn new<Identity: IBDA_TransportStreamSelector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTSID<Identity: IBDA_TransportStreamSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ustsid: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_TransportStreamSelector_Impl::SetTSID(this, core::mem::transmute_copy(&ustsid)).into()
            }
        }
        unsafe extern "system" fn GetTSInformation<Identity: IBDA_TransportStreamSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultsinformationbufferlen: *mut u32, pbtsinformationbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_TransportStreamSelector_Impl::GetTSInformation(this, core::mem::transmute_copy(&pultsinformationbufferlen), core::mem::transmute_copy(&pbtsinformationbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTSID: SetTSID::<Identity, OFFSET>,
            GetTSInformation: GetTSInformation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_TransportStreamSelector as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_TransportStreamSelector {}
windows_core::imp::define_interface!(IBDA_UserActivityService, IBDA_UserActivityService_Vtbl, 0x53b14189_e478_4b7a_a1ff_506db4b99dfe);
windows_core::imp::interface_hierarchy!(IBDA_UserActivityService, windows_core::IUnknown);
impl IBDA_UserActivityService {
    pub unsafe fn SetCurrentTunerUseReason(&self, dwusereason: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentTunerUseReason)(windows_core::Interface::as_raw(self), dwusereason) }
    }
    pub unsafe fn GetUserActivityInterval(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserActivityInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UserActivityDetected(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UserActivityDetected)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_UserActivityService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCurrentTunerUseReason: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetUserActivityInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UserActivityDetected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBDA_UserActivityService_Impl: windows_core::IUnknownImpl {
    fn SetCurrentTunerUseReason(&self, dwusereason: u32) -> windows_core::Result<()>;
    fn GetUserActivityInterval(&self) -> windows_core::Result<u32>;
    fn UserActivityDetected(&self) -> windows_core::Result<()>;
}
impl IBDA_UserActivityService_Vtbl {
    pub const fn new<Identity: IBDA_UserActivityService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCurrentTunerUseReason<Identity: IBDA_UserActivityService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwusereason: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_UserActivityService_Impl::SetCurrentTunerUseReason(this, core::mem::transmute_copy(&dwusereason)).into()
            }
        }
        unsafe extern "system" fn GetUserActivityInterval<Identity: IBDA_UserActivityService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwactivityinterval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_UserActivityService_Impl::GetUserActivityInterval(this) {
                    Ok(ok__) => {
                        pdwactivityinterval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserActivityDetected<Identity: IBDA_UserActivityService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_UserActivityService_Impl::UserActivityDetected(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCurrentTunerUseReason: SetCurrentTunerUseReason::<Identity, OFFSET>,
            GetUserActivityInterval: GetUserActivityInterval::<Identity, OFFSET>,
            UserActivityDetected: UserActivityDetected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_UserActivityService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_UserActivityService {}
windows_core::imp::define_interface!(IBDA_VoidTransform, IBDA_VoidTransform_Vtbl, 0x71985f46_1ca1_11d3_9cc8_00c04f7971e0);
windows_core::imp::interface_hierarchy!(IBDA_VoidTransform, windows_core::IUnknown);
impl IBDA_VoidTransform {
    pub unsafe fn Start(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_VoidTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBDA_VoidTransform_Impl: windows_core::IUnknownImpl {
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
}
impl IBDA_VoidTransform_Vtbl {
    pub const fn new<Identity: IBDA_VoidTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IBDA_VoidTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_VoidTransform_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IBDA_VoidTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_VoidTransform_Impl::Stop(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Start: Start::<Identity, OFFSET>, Stop: Stop::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_VoidTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_VoidTransform {}
windows_core::imp::define_interface!(IBDA_WMDRMSession, IBDA_WMDRMSession_Vtbl, 0x4be6fa3d_07cd_4139_8b80_8c18ba3aec88);
windows_core::imp::interface_hierarchy!(IBDA_WMDRMSession, windows_core::IUnknown);
impl IBDA_WMDRMSession {
    pub unsafe fn GetStatus(&self, maxcapturetoken: *mut u32, maxstreamingpid: *mut u32, maxlicense: *mut u32, minsecuritylevel: *mut u32, revinfosequencenumber: *mut u32, revinfoissuedtime: *mut u64, revinfottl: *mut u32, revlistversion: *mut u32, ulstate: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), maxcapturetoken as _, maxstreamingpid as _, maxlicense as _, minsecuritylevel as _, revinfosequencenumber as _, revinfoissuedtime as _, revinfottl as _, revlistversion as _, ulstate as _) }
    }
    pub unsafe fn SetRevInfo(&self, ulrevinfolen: u32, pbrevinfo: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRevInfo)(windows_core::Interface::as_raw(self), ulrevinfolen, pbrevinfo) }
    }
    pub unsafe fn SetCrl(&self, ulcrllen: u32, pbcrllen: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCrl)(windows_core::Interface::as_raw(self), ulcrllen, pbcrllen) }
    }
    pub unsafe fn TransactMessage(&self, ulcbrequest: u32, pbrequest: *const u8, pulcbresponse: *mut u32, pbresponse: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TransactMessage)(windows_core::Interface::as_raw(self), ulcbrequest, pbrequest, pulcbresponse as _, pbresponse as _) }
    }
    pub unsafe fn GetLicense(&self, uuidkey: *const windows_core::GUID, pulpackagelen: *mut u32, pbpackage: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLicense)(windows_core::Interface::as_raw(self), uuidkey, pulpackagelen as _, pbpackage as _) }
    }
    pub unsafe fn ReissueLicense(&self, uuidkey: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReissueLicense)(windows_core::Interface::as_raw(self), uuidkey) }
    }
    pub unsafe fn RenewLicense(&self, ulinxmrlicenselen: u32, pbinxmrlicense: *const u8, ulentitlementtokenlen: u32, pbentitlementtoken: *const u8, puldescramblestatus: *mut u32, puloutxmrlicenselen: *mut u32, pboutxmrlicense: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RenewLicense)(windows_core::Interface::as_raw(self), ulinxmrlicenselen, pbinxmrlicense, ulentitlementtokenlen, pbentitlementtoken, puldescramblestatus as _, puloutxmrlicenselen as _, pboutxmrlicense as _) }
    }
    pub unsafe fn GetKeyInfo(&self, pulkeyinfolen: *mut u32, pbkeyinfo: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetKeyInfo)(windows_core::Interface::as_raw(self), pulkeyinfolen as _, pbkeyinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_WMDRMSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u64, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetRevInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub SetCrl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub TransactMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub GetLicense: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub ReissueLicense: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RenewLicense: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *const u8, *mut u32, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub GetKeyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u8) -> windows_core::HRESULT,
}
pub trait IBDA_WMDRMSession_Impl: windows_core::IUnknownImpl {
    fn GetStatus(&self, maxcapturetoken: *mut u32, maxstreamingpid: *mut u32, maxlicense: *mut u32, minsecuritylevel: *mut u32, revinfosequencenumber: *mut u32, revinfoissuedtime: *mut u64, revinfottl: *mut u32, revlistversion: *mut u32, ulstate: *mut u32) -> windows_core::Result<()>;
    fn SetRevInfo(&self, ulrevinfolen: u32, pbrevinfo: *const u8) -> windows_core::Result<()>;
    fn SetCrl(&self, ulcrllen: u32, pbcrllen: *const u8) -> windows_core::Result<()>;
    fn TransactMessage(&self, ulcbrequest: u32, pbrequest: *const u8, pulcbresponse: *mut u32, pbresponse: *mut u8) -> windows_core::Result<()>;
    fn GetLicense(&self, uuidkey: *const windows_core::GUID, pulpackagelen: *mut u32, pbpackage: *mut u8) -> windows_core::Result<()>;
    fn ReissueLicense(&self, uuidkey: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RenewLicense(&self, ulinxmrlicenselen: u32, pbinxmrlicense: *const u8, ulentitlementtokenlen: u32, pbentitlementtoken: *const u8, puldescramblestatus: *mut u32, puloutxmrlicenselen: *mut u32, pboutxmrlicense: *mut u8) -> windows_core::Result<()>;
    fn GetKeyInfo(&self, pulkeyinfolen: *mut u32, pbkeyinfo: *mut u8) -> windows_core::Result<()>;
}
impl IBDA_WMDRMSession_Vtbl {
    pub const fn new<Identity: IBDA_WMDRMSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: IBDA_WMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxcapturetoken: *mut u32, maxstreamingpid: *mut u32, maxlicense: *mut u32, minsecuritylevel: *mut u32, revinfosequencenumber: *mut u32, revinfoissuedtime: *mut u64, revinfottl: *mut u32, revlistversion: *mut u32, ulstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMSession_Impl::GetStatus(this, core::mem::transmute_copy(&maxcapturetoken), core::mem::transmute_copy(&maxstreamingpid), core::mem::transmute_copy(&maxlicense), core::mem::transmute_copy(&minsecuritylevel), core::mem::transmute_copy(&revinfosequencenumber), core::mem::transmute_copy(&revinfoissuedtime), core::mem::transmute_copy(&revinfottl), core::mem::transmute_copy(&revlistversion), core::mem::transmute_copy(&ulstate)).into()
            }
        }
        unsafe extern "system" fn SetRevInfo<Identity: IBDA_WMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrevinfolen: u32, pbrevinfo: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMSession_Impl::SetRevInfo(this, core::mem::transmute_copy(&ulrevinfolen), core::mem::transmute_copy(&pbrevinfo)).into()
            }
        }
        unsafe extern "system" fn SetCrl<Identity: IBDA_WMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcrllen: u32, pbcrllen: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMSession_Impl::SetCrl(this, core::mem::transmute_copy(&ulcrllen), core::mem::transmute_copy(&pbcrllen)).into()
            }
        }
        unsafe extern "system" fn TransactMessage<Identity: IBDA_WMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcbrequest: u32, pbrequest: *const u8, pulcbresponse: *mut u32, pbresponse: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMSession_Impl::TransactMessage(this, core::mem::transmute_copy(&ulcbrequest), core::mem::transmute_copy(&pbrequest), core::mem::transmute_copy(&pulcbresponse), core::mem::transmute_copy(&pbresponse)).into()
            }
        }
        unsafe extern "system" fn GetLicense<Identity: IBDA_WMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uuidkey: *const windows_core::GUID, pulpackagelen: *mut u32, pbpackage: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMSession_Impl::GetLicense(this, core::mem::transmute_copy(&uuidkey), core::mem::transmute_copy(&pulpackagelen), core::mem::transmute_copy(&pbpackage)).into()
            }
        }
        unsafe extern "system" fn ReissueLicense<Identity: IBDA_WMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uuidkey: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMSession_Impl::ReissueLicense(this, core::mem::transmute_copy(&uuidkey)).into()
            }
        }
        unsafe extern "system" fn RenewLicense<Identity: IBDA_WMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulinxmrlicenselen: u32, pbinxmrlicense: *const u8, ulentitlementtokenlen: u32, pbentitlementtoken: *const u8, puldescramblestatus: *mut u32, puloutxmrlicenselen: *mut u32, pboutxmrlicense: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMSession_Impl::RenewLicense(this, core::mem::transmute_copy(&ulinxmrlicenselen), core::mem::transmute_copy(&pbinxmrlicense), core::mem::transmute_copy(&ulentitlementtokenlen), core::mem::transmute_copy(&pbentitlementtoken), core::mem::transmute_copy(&puldescramblestatus), core::mem::transmute_copy(&puloutxmrlicenselen), core::mem::transmute_copy(&pboutxmrlicense)).into()
            }
        }
        unsafe extern "system" fn GetKeyInfo<Identity: IBDA_WMDRMSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulkeyinfolen: *mut u32, pbkeyinfo: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMSession_Impl::GetKeyInfo(this, core::mem::transmute_copy(&pulkeyinfolen), core::mem::transmute_copy(&pbkeyinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            SetRevInfo: SetRevInfo::<Identity, OFFSET>,
            SetCrl: SetCrl::<Identity, OFFSET>,
            TransactMessage: TransactMessage::<Identity, OFFSET>,
            GetLicense: GetLicense::<Identity, OFFSET>,
            ReissueLicense: ReissueLicense::<Identity, OFFSET>,
            RenewLicense: RenewLicense::<Identity, OFFSET>,
            GetKeyInfo: GetKeyInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_WMDRMSession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_WMDRMSession {}
windows_core::imp::define_interface!(IBDA_WMDRMTuner, IBDA_WMDRMTuner_Vtbl, 0x86d979cf_a8a7_4f94_b5fb_14c0aca68fe6);
windows_core::imp::interface_hierarchy!(IBDA_WMDRMTuner, windows_core::IUnknown);
impl IBDA_WMDRMTuner {
    pub unsafe fn PurchaseEntitlement(&self, uldialogrequest: u32, bstrlanguage: &windows_core::BSTR, ulpurchasetokenlen: u32, pbpurchasetoken: *const u8, puldescramblestatus: *mut u32, pulcapturetokenlen: *mut u32, pbcapturetoken: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PurchaseEntitlement)(windows_core::Interface::as_raw(self), uldialogrequest, core::mem::transmute_copy(bstrlanguage), ulpurchasetokenlen, pbpurchasetoken, puldescramblestatus as _, pulcapturetokenlen as _, pbcapturetoken as _) }
    }
    pub unsafe fn CancelCaptureToken(&self, ulcapturetokenlen: u32, pbcapturetoken: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelCaptureToken)(windows_core::Interface::as_raw(self), ulcapturetokenlen, pbcapturetoken) }
    }
    pub unsafe fn SetPidProtection(&self, ulpid: u32, uuidkey: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPidProtection)(windows_core::Interface::as_raw(self), ulpid, uuidkey) }
    }
    pub unsafe fn GetPidProtection(&self, pulpid: u32) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPidProtection)(windows_core::Interface::as_raw(self), pulpid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSyncValue(&self, ulsyncvalue: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSyncValue)(windows_core::Interface::as_raw(self), ulsyncvalue) }
    }
    pub unsafe fn GetStartCodeProfile(&self, pulstartcodeprofilelen: *mut u32, pbstartcodeprofile: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStartCodeProfile)(windows_core::Interface::as_raw(self), pulstartcodeprofilelen as _, pbstartcodeprofile as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBDA_WMDRMTuner_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PurchaseEntitlement: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *const u8, *mut u32, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub CancelCaptureToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub SetPidProtection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetPidProtection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetSyncValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetStartCodeProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u8) -> windows_core::HRESULT,
}
pub trait IBDA_WMDRMTuner_Impl: windows_core::IUnknownImpl {
    fn PurchaseEntitlement(&self, uldialogrequest: u32, bstrlanguage: &windows_core::BSTR, ulpurchasetokenlen: u32, pbpurchasetoken: *const u8, puldescramblestatus: *mut u32, pulcapturetokenlen: *mut u32, pbcapturetoken: *mut u8) -> windows_core::Result<()>;
    fn CancelCaptureToken(&self, ulcapturetokenlen: u32, pbcapturetoken: *const u8) -> windows_core::Result<()>;
    fn SetPidProtection(&self, ulpid: u32, uuidkey: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetPidProtection(&self, pulpid: u32) -> windows_core::Result<windows_core::GUID>;
    fn SetSyncValue(&self, ulsyncvalue: u32) -> windows_core::Result<()>;
    fn GetStartCodeProfile(&self, pulstartcodeprofilelen: *mut u32, pbstartcodeprofile: *mut u8) -> windows_core::Result<()>;
}
impl IBDA_WMDRMTuner_Vtbl {
    pub const fn new<Identity: IBDA_WMDRMTuner_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PurchaseEntitlement<Identity: IBDA_WMDRMTuner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uldialogrequest: u32, bstrlanguage: *mut core::ffi::c_void, ulpurchasetokenlen: u32, pbpurchasetoken: *const u8, puldescramblestatus: *mut u32, pulcapturetokenlen: *mut u32, pbcapturetoken: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMTuner_Impl::PurchaseEntitlement(this, core::mem::transmute_copy(&uldialogrequest), core::mem::transmute(&bstrlanguage), core::mem::transmute_copy(&ulpurchasetokenlen), core::mem::transmute_copy(&pbpurchasetoken), core::mem::transmute_copy(&puldescramblestatus), core::mem::transmute_copy(&pulcapturetokenlen), core::mem::transmute_copy(&pbcapturetoken)).into()
            }
        }
        unsafe extern "system" fn CancelCaptureToken<Identity: IBDA_WMDRMTuner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcapturetokenlen: u32, pbcapturetoken: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMTuner_Impl::CancelCaptureToken(this, core::mem::transmute_copy(&ulcapturetokenlen), core::mem::transmute_copy(&pbcapturetoken)).into()
            }
        }
        unsafe extern "system" fn SetPidProtection<Identity: IBDA_WMDRMTuner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulpid: u32, uuidkey: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMTuner_Impl::SetPidProtection(this, core::mem::transmute_copy(&ulpid), core::mem::transmute_copy(&uuidkey)).into()
            }
        }
        unsafe extern "system" fn GetPidProtection<Identity: IBDA_WMDRMTuner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpid: u32, uuidkey: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBDA_WMDRMTuner_Impl::GetPidProtection(this, core::mem::transmute_copy(&pulpid)) {
                    Ok(ok__) => {
                        uuidkey.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSyncValue<Identity: IBDA_WMDRMTuner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsyncvalue: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMTuner_Impl::SetSyncValue(this, core::mem::transmute_copy(&ulsyncvalue)).into()
            }
        }
        unsafe extern "system" fn GetStartCodeProfile<Identity: IBDA_WMDRMTuner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulstartcodeprofilelen: *mut u32, pbstartcodeprofile: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBDA_WMDRMTuner_Impl::GetStartCodeProfile(this, core::mem::transmute_copy(&pulstartcodeprofilelen), core::mem::transmute_copy(&pbstartcodeprofile)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PurchaseEntitlement: PurchaseEntitlement::<Identity, OFFSET>,
            CancelCaptureToken: CancelCaptureToken::<Identity, OFFSET>,
            SetPidProtection: SetPidProtection::<Identity, OFFSET>,
            GetPidProtection: GetPidProtection::<Identity, OFFSET>,
            SetSyncValue: SetSyncValue::<Identity, OFFSET>,
            GetStartCodeProfile: GetStartCodeProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBDA_WMDRMTuner as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBDA_WMDRMTuner {}
windows_core::imp::define_interface!(IBroadcastEvent, IBroadcastEvent_Vtbl, 0x3b21263f_26e8_489d_aac4_924f7efd9511);
windows_core::imp::interface_hierarchy!(IBroadcastEvent, windows_core::IUnknown);
impl IBroadcastEvent {
    pub unsafe fn Fire(&self, eventid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Fire)(windows_core::Interface::as_raw(self), core::mem::transmute(eventid)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBroadcastEvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Fire: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IBroadcastEvent_Impl: windows_core::IUnknownImpl {
    fn Fire(&self, eventid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl IBroadcastEvent_Vtbl {
    pub const fn new<Identity: IBroadcastEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Fire<Identity: IBroadcastEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBroadcastEvent_Impl::Fire(this, core::mem::transmute(&eventid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Fire: Fire::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBroadcastEvent as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBroadcastEvent {}
windows_core::imp::define_interface!(IBroadcastEventEx, IBroadcastEventEx_Vtbl, 0x3d9e3887_1929_423f_8021_43682de95448);
impl core::ops::Deref for IBroadcastEventEx {
    type Target = IBroadcastEvent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBroadcastEventEx, windows_core::IUnknown, IBroadcastEvent);
impl IBroadcastEventEx {
    pub unsafe fn FireEx(&self, eventid: windows_core::GUID, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FireEx)(windows_core::Interface::as_raw(self), core::mem::transmute(eventid), param1, param2, param3, param4) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBroadcastEventEx_Vtbl {
    pub base__: IBroadcastEvent_Vtbl,
    pub FireEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, u32, u32, u32) -> windows_core::HRESULT,
}
pub trait IBroadcastEventEx_Impl: IBroadcastEvent_Impl {
    fn FireEx(&self, eventid: &windows_core::GUID, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::Result<()>;
}
impl IBroadcastEventEx_Vtbl {
    pub const fn new<Identity: IBroadcastEventEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FireEx<Identity: IBroadcastEventEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: windows_core::GUID, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBroadcastEventEx_Impl::FireEx(this, core::mem::transmute(&eventid), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        Self { base__: IBroadcastEvent_Vtbl::new::<Identity, OFFSET>(), FireEx: FireEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBroadcastEventEx as windows_core::Interface>::IID || iid == &<IBroadcastEvent as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBroadcastEventEx {}
windows_core::imp::define_interface!(ICCSubStreamFiltering, ICCSubStreamFiltering_Vtbl, 0x4b2bd7ea_8347_467b_8dbf_62f784929cc3);
windows_core::imp::interface_hierarchy!(ICCSubStreamFiltering, windows_core::IUnknown);
impl ICCSubStreamFiltering {
    pub unsafe fn SubstreamTypes(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubstreamTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSubstreamTypes(&self, types: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSubstreamTypes)(windows_core::Interface::as_raw(self), types) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICCSubStreamFiltering_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SubstreamTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSubstreamTypes: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait ICCSubStreamFiltering_Impl: windows_core::IUnknownImpl {
    fn SubstreamTypes(&self) -> windows_core::Result<i32>;
    fn SetSubstreamTypes(&self, types: i32) -> windows_core::Result<()>;
}
impl ICCSubStreamFiltering_Vtbl {
    pub const fn new<Identity: ICCSubStreamFiltering_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SubstreamTypes<Identity: ICCSubStreamFiltering_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICCSubStreamFiltering_Impl::SubstreamTypes(this) {
                    Ok(ok__) => {
                        ptypes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubstreamTypes<Identity: ICCSubStreamFiltering_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, types: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICCSubStreamFiltering_Impl::SetSubstreamTypes(this, core::mem::transmute_copy(&types)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SubstreamTypes: SubstreamTypes::<Identity, OFFSET>,
            SetSubstreamTypes: SetSubstreamTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICCSubStreamFiltering as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICCSubStreamFiltering {}
windows_core::imp::define_interface!(IESEvent, IESEvent_Vtbl, 0x1f0e5357_af43_44e6_8547_654c645145d2);
windows_core::imp::interface_hierarchy!(IESEvent, windows_core::IUnknown);
impl IESEvent {
    pub unsafe fn GetEventId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEventType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCompletionStatus(&self, dwresult: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCompletionStatus)(windows_core::Interface::as_raw(self), dwresult) }
    }
    #[cfg(feature = "Win32_oaidl")]
    pub unsafe fn GetData(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStringData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IESEvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEventId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetEventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetCompletionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_oaidl")]
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_oaidl"))]
    GetData: usize,
    pub GetStringData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_oaidl")]
pub trait IESEvent_Impl: windows_core::IUnknownImpl {
    fn GetEventId(&self) -> windows_core::Result<u32>;
    fn GetEventType(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetCompletionStatus(&self, dwresult: u32) -> windows_core::Result<()>;
    fn GetData(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn GetStringData(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_oaidl")]
impl IESEvent_Vtbl {
    pub const fn new<Identity: IESEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEventId<Identity: IESEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdweventid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IESEvent_Impl::GetEventId(this) {
                    Ok(ok__) => {
                        pdweventid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventType<Identity: IESEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguideventtype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IESEvent_Impl::GetEventType(this) {
                    Ok(ok__) => {
                        pguideventtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCompletionStatus<Identity: IESEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwresult: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IESEvent_Impl::SetCompletionStatus(this, core::mem::transmute_copy(&dwresult)).into()
            }
        }
        unsafe extern "system" fn GetData<Identity: IESEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IESEvent_Impl::GetData(this) {
                    Ok(ok__) => {
                        pbdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringData<Identity: IESEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IESEvent_Impl::GetStringData(this) {
                    Ok(ok__) => {
                        pbstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEventId: GetEventId::<Identity, OFFSET>,
            GetEventType: GetEventType::<Identity, OFFSET>,
            SetCompletionStatus: SetCompletionStatus::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            GetStringData: GetStringData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IESEvent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_oaidl")]
impl windows_core::RuntimeName for IESEvent {}
windows_core::imp::define_interface!(IESEvents, IESEvents_Vtbl, 0xabd414bf_cfe5_4e5e_af5b_4b4e49c5bfeb);
windows_core::imp::interface_hierarchy!(IESEvents, windows_core::IUnknown);
impl IESEvents {
    pub unsafe fn OnESEventReceived<P1>(&self, guideventtype: windows_core::GUID, pesevent: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IESEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnESEventReceived)(windows_core::Interface::as_raw(self), core::mem::transmute(guideventtype), pesevent.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IESEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnESEventReceived: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IESEvents_Impl: windows_core::IUnknownImpl {
    fn OnESEventReceived(&self, guideventtype: &windows_core::GUID, pesevent: windows_core::Ref<IESEvent>) -> windows_core::Result<()>;
}
impl IESEvents_Vtbl {
    pub const fn new<Identity: IESEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnESEventReceived<Identity: IESEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guideventtype: windows_core::GUID, pesevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IESEvents_Impl::OnESEventReceived(this, core::mem::transmute(&guideventtype), core::mem::transmute_copy(&pesevent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnESEventReceived: OnESEventReceived::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IESEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IESEvents {}
windows_core::imp::define_interface!(IEnumPIDMap, IEnumPIDMap_Vtbl, 0xafb6c2a2_2c41_11d3_8a60_0000f81e0e4a);
windows_core::imp::interface_hierarchy!(IEnumPIDMap, windows_core::IUnknown);
impl IEnumPIDMap {
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn Next(&self, crequest: u32, ppidmap: *mut super::bdatypes::PID_MAP, pcreceived: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), crequest, ppidmap as _, pcreceived as _) }
    }
    pub unsafe fn Skip(&self, crecords: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), crecords) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPIDMap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_bdatypes")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::bdatypes::PID_MAP, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_bdatypes")]
pub trait IEnumPIDMap_Impl: windows_core::IUnknownImpl {
    fn Next(&self, crequest: u32, ppidmap: *mut super::bdatypes::PID_MAP, pcreceived: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, crecords: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumPIDMap>;
}
#[cfg(feature = "Win32_bdatypes")]
impl IEnumPIDMap_Vtbl {
    pub const fn new<Identity: IEnumPIDMap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumPIDMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: u32, ppidmap: *mut super::bdatypes::PID_MAP, pcreceived: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPIDMap_Impl::Next(this, core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&ppidmap), core::mem::transmute_copy(&pcreceived)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumPIDMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crecords: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPIDMap_Impl::Skip(this, core::mem::transmute_copy(&crecords)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumPIDMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPIDMap_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumPIDMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienumpidmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumPIDMap_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienumpidmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumPIDMap as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_bdatypes")]
impl windows_core::RuntimeName for IEnumPIDMap {}
windows_core::imp::define_interface!(IFrequencyMap, IFrequencyMap_Vtbl, 0x06fb45c1_693c_4ea7_b79f_7a6a54d8def2);
windows_core::imp::interface_hierarchy!(IFrequencyMap, windows_core::IUnknown);
impl IFrequencyMap {
    pub unsafe fn get_FrequencyMapping(&self, ulcount: *mut u32, ppullist: *mut *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_FrequencyMapping)(windows_core::Interface::as_raw(self), ulcount as _, ppullist as _) }
    }
    pub unsafe fn put_FrequencyMapping(&self, ulcount: u32, plist: *const u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FrequencyMapping)(windows_core::Interface::as_raw(self), ulcount, plist) }
    }
    pub unsafe fn get_CountryCode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CountryCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_CountryCode(&self, ulcountrycode: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CountryCode)(windows_core::Interface::as_raw(self), ulcountrycode) }
    }
    pub unsafe fn get_DefaultFrequencyMapping(&self, ulcountrycode: u32, pulcount: *mut u32, ppullist: *mut *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_DefaultFrequencyMapping)(windows_core::Interface::as_raw(self), ulcountrycode, pulcount as _, ppullist as _) }
    }
    pub unsafe fn get_CountryCodeList(&self, pulcount: *mut u32, ppullist: *mut *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_CountryCodeList)(windows_core::Interface::as_raw(self), pulcount as _, ppullist as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrequencyMap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_FrequencyMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u32) -> windows_core::HRESULT,
    pub put_FrequencyMapping: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> windows_core::HRESULT,
    pub get_CountryCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_CountryCode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_DefaultFrequencyMapping: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut *mut u32) -> windows_core::HRESULT,
    pub get_CountryCodeList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u32) -> windows_core::HRESULT,
}
pub trait IFrequencyMap_Impl: windows_core::IUnknownImpl {
    fn get_FrequencyMapping(&self, ulcount: *mut u32, ppullist: *mut *mut u32) -> windows_core::Result<()>;
    fn put_FrequencyMapping(&self, ulcount: u32, plist: *const u32) -> windows_core::Result<()>;
    fn get_CountryCode(&self) -> windows_core::Result<u32>;
    fn put_CountryCode(&self, ulcountrycode: u32) -> windows_core::Result<()>;
    fn get_DefaultFrequencyMapping(&self, ulcountrycode: u32, pulcount: *mut u32, ppullist: *mut *mut u32) -> windows_core::Result<()>;
    fn get_CountryCodeList(&self, pulcount: *mut u32, ppullist: *mut *mut u32) -> windows_core::Result<()>;
}
impl IFrequencyMap_Vtbl {
    pub const fn new<Identity: IFrequencyMap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_FrequencyMapping<Identity: IFrequencyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: *mut u32, ppullist: *mut *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrequencyMap_Impl::get_FrequencyMapping(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppullist)).into()
            }
        }
        unsafe extern "system" fn put_FrequencyMapping<Identity: IFrequencyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, plist: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrequencyMap_Impl::put_FrequencyMapping(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&plist)).into()
            }
        }
        unsafe extern "system" fn get_CountryCode<Identity: IFrequencyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcountrycode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrequencyMap_Impl::get_CountryCode(this) {
                    Ok(ok__) => {
                        pulcountrycode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CountryCode<Identity: IFrequencyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcountrycode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrequencyMap_Impl::put_CountryCode(this, core::mem::transmute_copy(&ulcountrycode)).into()
            }
        }
        unsafe extern "system" fn get_DefaultFrequencyMapping<Identity: IFrequencyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcountrycode: u32, pulcount: *mut u32, ppullist: *mut *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrequencyMap_Impl::get_DefaultFrequencyMapping(this, core::mem::transmute_copy(&ulcountrycode), core::mem::transmute_copy(&pulcount), core::mem::transmute_copy(&ppullist)).into()
            }
        }
        unsafe extern "system" fn get_CountryCodeList<Identity: IFrequencyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32, ppullist: *mut *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrequencyMap_Impl::get_CountryCodeList(this, core::mem::transmute_copy(&pulcount), core::mem::transmute_copy(&ppullist)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_FrequencyMapping: get_FrequencyMapping::<Identity, OFFSET>,
            put_FrequencyMapping: put_FrequencyMapping::<Identity, OFFSET>,
            get_CountryCode: get_CountryCode::<Identity, OFFSET>,
            put_CountryCode: put_CountryCode::<Identity, OFFSET>,
            get_DefaultFrequencyMapping: get_DefaultFrequencyMapping::<Identity, OFFSET>,
            get_CountryCodeList: get_CountryCodeList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrequencyMap as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFrequencyMap {}
windows_core::imp::define_interface!(IMPEG2PIDMap, IMPEG2PIDMap_Vtbl, 0xafb6c2a1_2c41_11d3_8a60_0000f81e0e4a);
windows_core::imp::interface_hierarchy!(IMPEG2PIDMap, windows_core::IUnknown);
impl IMPEG2PIDMap {
    #[cfg(feature = "Win32_bdatypes")]
    pub unsafe fn MapPID(&self, culpid: u32, pulpid: *const u32, mediasamplecontent: super::bdatypes::MEDIA_SAMPLE_CONTENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MapPID)(windows_core::Interface::as_raw(self), culpid, pulpid, mediasamplecontent) }
    }
    pub unsafe fn UnmapPID(&self, culpid: u32, pulpid: *const u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnmapPID)(windows_core::Interface::as_raw(self), culpid, pulpid) }
    }
    pub unsafe fn EnumPIDMap(&self) -> windows_core::Result<IEnumPIDMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumPIDMap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMPEG2PIDMap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_bdatypes")]
    pub MapPID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, super::bdatypes::MEDIA_SAMPLE_CONTENT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bdatypes"))]
    MapPID: usize,
    pub UnmapPID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> windows_core::HRESULT,
    pub EnumPIDMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_bdatypes")]
pub trait IMPEG2PIDMap_Impl: windows_core::IUnknownImpl {
    fn MapPID(&self, culpid: u32, pulpid: *const u32, mediasamplecontent: super::bdatypes::MEDIA_SAMPLE_CONTENT) -> windows_core::Result<()>;
    fn UnmapPID(&self, culpid: u32, pulpid: *const u32) -> windows_core::Result<()>;
    fn EnumPIDMap(&self) -> windows_core::Result<IEnumPIDMap>;
}
#[cfg(feature = "Win32_bdatypes")]
impl IMPEG2PIDMap_Vtbl {
    pub const fn new<Identity: IMPEG2PIDMap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MapPID<Identity: IMPEG2PIDMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, culpid: u32, pulpid: *const u32, mediasamplecontent: super::bdatypes::MEDIA_SAMPLE_CONTENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMPEG2PIDMap_Impl::MapPID(this, core::mem::transmute_copy(&culpid), core::mem::transmute_copy(&pulpid), core::mem::transmute_copy(&mediasamplecontent)).into()
            }
        }
        unsafe extern "system" fn UnmapPID<Identity: IMPEG2PIDMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, culpid: u32, pulpid: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMPEG2PIDMap_Impl::UnmapPID(this, core::mem::transmute_copy(&culpid), core::mem::transmute_copy(&pulpid)).into()
            }
        }
        unsafe extern "system" fn EnumPIDMap<Identity: IMPEG2PIDMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pienumpidmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMPEG2PIDMap_Impl::EnumPIDMap(this) {
                    Ok(ok__) => {
                        pienumpidmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MapPID: MapPID::<Identity, OFFSET>,
            UnmapPID: UnmapPID::<Identity, OFFSET>,
            EnumPIDMap: EnumPIDMap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMPEG2PIDMap as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_bdatypes")]
impl windows_core::RuntimeName for IMPEG2PIDMap {}
pub type KSPROPERTY_IPSINK = i32;
pub const KSPROPERTY_IPSINK_ADAPTER_ADDRESS: KSPROPERTY_IPSINK = 2;
pub const KSPROPERTY_IPSINK_ADAPTER_DESCRIPTION: KSPROPERTY_IPSINK = 1;
pub const KSPROPERTY_IPSINK_MULTICASTLIST: KSPROPERTY_IPSINK = 0;
pub type LocationCodeSchemeType = i32;
pub const NotAssociated: SmartCardAssociationType = 0;
pub const NotEntitled: EntitlementType = 1;
pub const NotReady: UICloseReasonType = 0;
pub const PBDA_AUX_CONNECTOR_TYPE_Composite: windows_core::GUID = windows_core::GUID::from_u128(0xf6298b4c_c725_4d42_849b_410bbb14ea62);
pub const PBDA_AUX_CONNECTOR_TYPE_SVideo: windows_core::GUID = windows_core::GUID::from_u128(0xa0e905f4_24c9_4a54_b761_213355efc13a);
pub const PBDA_Encoder_Audio_AlgorithmType_AC3: u32 = 1;
pub const PBDA_Encoder_Audio_AlgorithmType_MPEG1LayerII: u32 = 0;
pub const PBDA_Encoder_BitrateMode_Average: u32 = 3;
pub const PBDA_Encoder_BitrateMode_Constant: u32 = 1;
pub const PBDA_Encoder_BitrateMode_Variable: u32 = 2;
pub const PBDA_Encoder_Video_AVC: u32 = 1;
pub const PBDA_Encoder_Video_H264: u32 = 1;
pub const PBDA_Encoder_Video_MPEG2PartII: u32 = 0;
pub const PBDA_Encoder_Video_MPEG4Part10: u32 = 1;
pub const SCTE_18: LocationCodeSchemeType = 0;
#[repr(C)]
#[cfg(feature = "Win32_bdatypes")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SmartCardApplication {
    pub ApplicationType: super::bdatypes::ApplicationTypeType,
    pub ApplicationVersion: u16,
    pub pbstrApplicationName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub pbstrApplicationURL: core::mem::ManuallyDrop<windows_core::BSTR>,
}
pub type SmartCardAssociationType = i32;
pub type SmartCardStatusType = i32;
pub const SystemClosed: UICloseReasonType = 2;
pub const TechnicalFailure: EntitlementType = 2;
pub type UICloseReasonType = i32;
pub const UserClosed: UICloseReasonType = 1;
