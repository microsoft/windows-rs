windows_core::imp::define_interface!(IEnumPortableDeviceObjectIDs, IEnumPortableDeviceObjectIDs_Vtbl, 0x10ece955_cf41_4728_bfa0_41eedf1bbf19);
windows_core::imp::interface_hierarchy!(IEnumPortableDeviceObjectIDs, windows_core::IUnknown);
impl IEnumPortableDeviceObjectIDs {
    pub unsafe fn Next(&self, cobjects: u32, pobjids: *mut windows_core::PWSTR, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cobjects, pobjids as _, pcfetched as _) }
    }
    pub unsafe fn Skip(&self, cobjects: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cobjects) }
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
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPortableDeviceObjectIDs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumPortableDeviceObjectIDs_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cobjects: u32, pobjids: *mut windows_core::PWSTR, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cobjects: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumPortableDeviceObjectIDs>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl IEnumPortableDeviceObjectIDs_Vtbl {
    pub const fn new<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjects: u32, pobjids: *mut windows_core::PWSTR, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPortableDeviceObjectIDs_Impl::Next(this, core::mem::transmute_copy(&cobjects), core::mem::transmute_copy(&pobjids), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjects: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPortableDeviceObjectIDs_Impl::Skip(this, core::mem::transmute_copy(&cobjects)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPortableDeviceObjectIDs_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumPortableDeviceObjectIDs_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPortableDeviceObjectIDs_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumPortableDeviceObjectIDs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumPortableDeviceObjectIDs {}
windows_core::imp::define_interface!(IPortableDevice, IPortableDevice_Vtbl, 0x625e2df8_6392_4cf0_9ad1_3cfa5f17775c);
windows_core::imp::interface_hierarchy!(IPortableDevice, windows_core::IUnknown);
impl IPortableDevice {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn Open<P0, P1>(&self, pszpnpdeviceid: P0, pclientinfo: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), pclientinfo.param().abi()) }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn SendCommand<P1>(&self, dwflags: u32, pparameters: P1) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>
    where
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendCommand)(windows_core::Interface::as_raw(self), dwflags, pparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Content(&self) -> windows_core::Result<IPortableDeviceContent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Capabilities(&self) -> windows_core::Result<IPortableDeviceCapabilities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Capabilities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn Advise<P1, P2>(&self, dwflags: u32, pcallback: P1, pparameters: P2) -> windows_core::Result<windows_core::PWSTR>
    where
        P1: windows_core::Param<IPortableDeviceEventCallback>,
        P2: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), dwflags, pcallback.param().abi(), pparameters.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise<P0>(&self, pszcookie: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), pszcookie.param().abi()) }
    }
    pub unsafe fn GetPnPDeviceID(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPnPDeviceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    Open: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub SendCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    SendCommand: usize,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "portabledevicetypes")]
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPnPDeviceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "portabledevicetypes")]
pub trait IPortableDevice_Impl: windows_core::IUnknownImpl {
    fn Open(&self, pszpnpdeviceid: &windows_core::PCWSTR, pclientinfo: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<()>;
    fn SendCommand(&self, dwflags: u32, pparameters: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn Content(&self) -> windows_core::Result<IPortableDeviceContent>;
    fn Capabilities(&self) -> windows_core::Result<IPortableDeviceCapabilities>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Advise(&self, dwflags: u32, pcallback: windows_core::Ref<IPortableDeviceEventCallback>, pparameters: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<windows_core::PWSTR>;
    fn Unadvise(&self, pszcookie: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPnPDeviceID(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "portabledevicetypes")]
impl IPortableDevice_Vtbl {
    pub const fn new<Identity: IPortableDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pclientinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevice_Impl::Open(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute_copy(&pclientinfo)).into()
            }
        }
        unsafe extern "system" fn SendCommand<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pparameters: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevice_Impl::SendCommand(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pparameters)) {
                    Ok(ok__) => {
                        ppresults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Content<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevice_Impl::Content(this) {
                    Ok(ok__) => {
                        ppcontent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Capabilities<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevice_Impl::Capabilities(this) {
                    Ok(ok__) => {
                        ppcapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevice_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevice_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Advise<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pcallback: *mut core::ffi::c_void, pparameters: *mut core::ffi::c_void, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevice_Impl::Advise(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pparameters)) {
                    Ok(ok__) => {
                        ppszcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcookie: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevice_Impl::Unadvise(this, core::mem::transmute(&pszcookie)).into()
            }
        }
        unsafe extern "system" fn GetPnPDeviceID<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpnpdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevice_Impl::GetPnPDeviceID(this) {
                    Ok(ok__) => {
                        ppszpnpdeviceid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            SendCommand: SendCommand::<Identity, OFFSET>,
            Content: Content::<Identity, OFFSET>,
            Capabilities: Capabilities::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            GetPnPDeviceID: GetPnPDeviceID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "portabledevicetypes")]
impl windows_core::RuntimeName for IPortableDevice {}
windows_core::imp::define_interface!(IPortableDeviceCapabilities, IPortableDeviceCapabilities_Vtbl, 0x2c8c6dbf_e3dc_4061_becc_8542e810d126);
windows_core::imp::interface_hierarchy!(IPortableDeviceCapabilities, windows_core::IUnknown);
impl IPortableDeviceCapabilities {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedCommands(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedCommands)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub unsafe fn GetCommandOptions(&self, command: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCommandOptions)(windows_core::Interface::as_raw(self), command, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetFunctionalCategories(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFunctionalCategories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetFunctionalObjects(&self, category: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFunctionalObjects)(windows_core::Interface::as_raw(self), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedContentTypes(&self, category: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedContentTypes)(windows_core::Interface::as_raw(self), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedFormats(&self, contenttype: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedFormats)(windows_core::Interface::as_raw(self), contenttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedFormatProperties)(windows_core::Interface::as_raw(self), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub unsafe fn GetFixedPropertyAttributes(&self, format: *const windows_core::GUID, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFixedPropertyAttributes)(windows_core::Interface::as_raw(self), format, key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedEvents(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedEvents)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetEventOptions(&self, event: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventOptions)(windows_core::Interface::as_raw(self), event, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceCapabilities_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedCommands: usize,
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub GetCommandOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "portabledevicetypes", feature = "wtypes")))]
    GetCommandOptions: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetFunctionalCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetFunctionalCategories: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetFunctionalObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetFunctionalObjects: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedContentTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedContentTypes: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedFormats: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedFormatProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedFormatProperties: usize,
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub GetFixedPropertyAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::wtypes::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "portabledevicetypes", feature = "wtypes")))]
    GetFixedPropertyAttributes: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedEvents: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetEventOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetEventOptions: usize,
}
#[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
pub trait IPortableDeviceCapabilities_Impl: windows_core::IUnknownImpl {
    fn GetSupportedCommands(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection>;
    fn GetCommandOptions(&self, command: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn GetFunctionalCategories(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>;
    fn GetFunctionalObjects(&self, category: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>;
    fn GetSupportedContentTypes(&self, category: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>;
    fn GetSupportedFormats(&self, contenttype: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>;
    fn GetSupportedFormatProperties(&self, format: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection>;
    fn GetFixedPropertyAttributes(&self, format: *const windows_core::GUID, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn GetSupportedEvents(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>;
    fn GetEventOptions(&self, event: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
}
#[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
impl IPortableDeviceCapabilities_Vtbl {
    pub const fn new<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedCommands<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcommands: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetSupportedCommands(this) {
                    Ok(ok__) => {
                        ppcommands.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCommandOptions<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, command: *const super::wtypes::PROPERTYKEY, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetCommandOptions(this, core::mem::transmute_copy(&command)) {
                    Ok(ok__) => {
                        ppoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionalCategories<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcategories: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetFunctionalCategories(this) {
                    Ok(ok__) => {
                        ppcategories.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionalObjects<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: *const windows_core::GUID, ppobjectids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetFunctionalObjects(this, core::mem::transmute_copy(&category)) {
                    Ok(ok__) => {
                        ppobjectids.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedContentTypes<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: *const windows_core::GUID, ppcontenttypes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetSupportedContentTypes(this, core::mem::transmute_copy(&category)) {
                    Ok(ok__) => {
                        ppcontenttypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *const windows_core::GUID, ppformats: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetSupportedFormats(this, core::mem::transmute_copy(&contenttype)) {
                    Ok(ok__) => {
                        ppformats.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetSupportedFormatProperties(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        ppkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFixedPropertyAttributes<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, key: *const super::wtypes::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetFixedPropertyAttributes(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceCapabilities_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn GetSupportedEvents<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppevents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetSupportedEvents(this) {
                    Ok(ok__) => {
                        ppevents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventOptions<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const windows_core::GUID, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetEventOptions(this, core::mem::transmute_copy(&event)) {
                    Ok(ok__) => {
                        ppoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedCommands: GetSupportedCommands::<Identity, OFFSET>,
            GetCommandOptions: GetCommandOptions::<Identity, OFFSET>,
            GetFunctionalCategories: GetFunctionalCategories::<Identity, OFFSET>,
            GetFunctionalObjects: GetFunctionalObjects::<Identity, OFFSET>,
            GetSupportedContentTypes: GetSupportedContentTypes::<Identity, OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Identity, OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Identity, OFFSET>,
            GetFixedPropertyAttributes: GetFixedPropertyAttributes::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Identity, OFFSET>,
            GetEventOptions: GetEventOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
impl windows_core::RuntimeName for IPortableDeviceCapabilities {}
windows_core::imp::define_interface!(IPortableDeviceContent, IPortableDeviceContent_Vtbl, 0x6a96ed84_7c73_4480_9938_bf5af477d426);
windows_core::imp::interface_hierarchy!(IPortableDeviceContent, windows_core::IUnknown);
impl IPortableDeviceContent {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn EnumObjects<P1, P2>(&self, dwflags: u32, pszparentobjectid: P1, pfilter: P2) -> windows_core::Result<IEnumPortableDeviceObjectIDs>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumObjects)(windows_core::Interface::as_raw(self), dwflags, pszparentobjectid.param().abi(), pfilter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<IPortableDeviceProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Transfer(&self) -> windows_core::Result<IPortableDeviceResources> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Transfer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn CreateObjectWithPropertiesOnly<P0>(&self, pvalues: P0, ppszobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateObjectWithPropertiesOnly)(windows_core::Interface::as_raw(self), pvalues.param().abi(), ppszobjectid as _) }
    }
    #[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
    pub unsafe fn CreateObjectWithPropertiesAndData<P0>(&self, pvalues: P0, ppdata: *mut Option<super::objidlbase::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateObjectWithPropertiesAndData)(windows_core::Interface::as_raw(self), pvalues.param().abi(), core::mem::transmute(ppdata), pdwoptimalwritebuffersize as _, ppszcookie as _) }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn Delete<P1>(&self, dwoptions: u32, pobjectids: P1, ppresults: *mut Option<super::portabledevicetypes::IPortableDevicePropVariantCollection>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::portabledevicetypes::IPortableDevicePropVariantCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), dwoptions, pobjectids.param().abi(), core::mem::transmute(ppresults)) }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetObjectIDsFromPersistentUniqueIDs<P0>(&self, ppersistentuniqueids: P0) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDevicePropVariantCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectIDsFromPersistentUniqueIDs)(windows_core::Interface::as_raw(self), ppersistentuniqueids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn Move<P0, P1>(&self, pobjectids: P0, pszdestinationfolderobjectid: P1, ppresults: *mut Option<super::portabledevicetypes::IPortableDevicePropVariantCollection>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDevicePropVariantCollection>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), pobjectids.param().abi(), pszdestinationfolderobjectid.param().abi(), core::mem::transmute(ppresults)) }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn Copy<P0, P1>(&self, pobjectids: P0, pszdestinationfolderobjectid: P1, ppresults: *mut Option<super::portabledevicetypes::IPortableDevicePropVariantCollection>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDevicePropVariantCollection>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), pobjectids.param().abi(), pszdestinationfolderobjectid.param().abi(), core::mem::transmute(ppresults)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub EnumObjects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    EnumObjects: usize,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Transfer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "portabledevicetypes")]
    pub CreateObjectWithPropertiesOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    CreateObjectWithPropertiesOnly: usize,
    #[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
    pub CreateObjectWithPropertiesAndData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "portabledevicetypes")))]
    CreateObjectWithPropertiesAndData: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    Delete: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetObjectIDsFromPersistentUniqueIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetObjectIDsFromPersistentUniqueIDs: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "portabledevicetypes")]
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    Move: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    Copy: usize,
}
#[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
pub trait IPortableDeviceContent_Impl: windows_core::IUnknownImpl {
    fn EnumObjects(&self, dwflags: u32, pszparentobjectid: &windows_core::PCWSTR, pfilter: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<IEnumPortableDeviceObjectIDs>;
    fn Properties(&self) -> windows_core::Result<IPortableDeviceProperties>;
    fn Transfer(&self) -> windows_core::Result<IPortableDeviceResources>;
    fn CreateObjectWithPropertiesOnly(&self, pvalues: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>, ppszobjectid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn CreateObjectWithPropertiesAndData(&self, pvalues: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>, ppdata: windows_core::OutRef<super::objidlbase::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn Delete(&self, dwoptions: u32, pobjectids: windows_core::Ref<super::portabledevicetypes::IPortableDevicePropVariantCollection>, ppresults: windows_core::OutRef<super::portabledevicetypes::IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
    fn GetObjectIDsFromPersistentUniqueIDs(&self, ppersistentuniqueids: windows_core::Ref<super::portabledevicetypes::IPortableDevicePropVariantCollection>) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Move(&self, pobjectids: windows_core::Ref<super::portabledevicetypes::IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: &windows_core::PCWSTR, ppresults: windows_core::OutRef<super::portabledevicetypes::IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
    fn Copy(&self, pobjectids: windows_core::Ref<super::portabledevicetypes::IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: &windows_core::PCWSTR, ppresults: windows_core::OutRef<super::portabledevicetypes::IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
impl IPortableDeviceContent_Vtbl {
    pub const fn new<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumObjects<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszparentobjectid: windows_core::PCWSTR, pfilter: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceContent_Impl::EnumObjects(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszparentobjectid), core::mem::transmute_copy(&pfilter)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceContent_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Transfer<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceContent_Impl::Transfer(this) {
                    Ok(ok__) => {
                        ppresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateObjectWithPropertiesOnly<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalues: *mut core::ffi::c_void, ppszobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::CreateObjectWithPropertiesOnly(this, core::mem::transmute_copy(&pvalues), core::mem::transmute_copy(&ppszobjectid)).into()
            }
        }
        unsafe extern "system" fn CreateObjectWithPropertiesAndData<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalues: *mut core::ffi::c_void, ppdata: *mut *mut core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::CreateObjectWithPropertiesAndData(this, core::mem::transmute_copy(&pvalues), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pdwoptimalwritebuffersize), core::mem::transmute_copy(&ppszcookie)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: u32, pobjectids: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::Delete(this, core::mem::transmute_copy(&dwoptions), core::mem::transmute_copy(&pobjectids), core::mem::transmute_copy(&ppresults)).into()
            }
        }
        unsafe extern "system" fn GetObjectIDsFromPersistentUniqueIDs<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppersistentuniqueids: *mut core::ffi::c_void, ppobjectids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceContent_Impl::GetObjectIDsFromPersistentUniqueIDs(this, core::mem::transmute_copy(&ppersistentuniqueids)) {
                    Ok(ok__) => {
                        ppobjectids.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Move<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectids: *mut core::ffi::c_void, pszdestinationfolderobjectid: windows_core::PCWSTR, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::Move(this, core::mem::transmute_copy(&pobjectids), core::mem::transmute(&pszdestinationfolderobjectid), core::mem::transmute_copy(&ppresults)).into()
            }
        }
        unsafe extern "system" fn Copy<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectids: *mut core::ffi::c_void, pszdestinationfolderobjectid: windows_core::PCWSTR, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::Copy(this, core::mem::transmute_copy(&pobjectids), core::mem::transmute(&pszdestinationfolderobjectid), core::mem::transmute_copy(&ppresults)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Transfer: Transfer::<Identity, OFFSET>,
            CreateObjectWithPropertiesOnly: CreateObjectWithPropertiesOnly::<Identity, OFFSET>,
            CreateObjectWithPropertiesAndData: CreateObjectWithPropertiesAndData::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetObjectIDsFromPersistentUniqueIDs: GetObjectIDsFromPersistentUniqueIDs::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceContent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
impl windows_core::RuntimeName for IPortableDeviceContent {}
windows_core::imp::define_interface!(IPortableDeviceContent2, IPortableDeviceContent2_Vtbl, 0x9b4add96_f6bf_4034_8708_eca72bf10554);
impl core::ops::Deref for IPortableDeviceContent2 {
    type Target = IPortableDeviceContent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPortableDeviceContent2, windows_core::IUnknown, IPortableDeviceContent);
impl IPortableDeviceContent2 {
    #[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
    pub unsafe fn UpdateObjectWithPropertiesAndData<P0, P1>(&self, pszobjectid: P0, pproperties: P1, ppdata: *mut Option<super::objidlbase::IStream>, pdwoptimalwritebuffersize: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateObjectWithPropertiesAndData)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), pproperties.param().abi(), core::mem::transmute(ppdata), pdwoptimalwritebuffersize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContent2_Vtbl {
    pub base__: IPortableDeviceContent_Vtbl,
    #[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
    pub UpdateObjectWithPropertiesAndData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "portabledevicetypes")))]
    UpdateObjectWithPropertiesAndData: usize,
}
#[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
pub trait IPortableDeviceContent2_Impl: IPortableDeviceContent_Impl {
    fn UpdateObjectWithPropertiesAndData(&self, pszobjectid: &windows_core::PCWSTR, pproperties: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>, ppdata: windows_core::OutRef<super::objidlbase::IStream>, pdwoptimalwritebuffersize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
impl IPortableDeviceContent2_Vtbl {
    pub const fn new<Identity: IPortableDeviceContent2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateObjectWithPropertiesAndData<Identity: IPortableDeviceContent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pproperties: *mut core::ffi::c_void, ppdata: *mut *mut core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent2_Impl::UpdateObjectWithPropertiesAndData(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&pproperties), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pdwoptimalwritebuffersize)).into()
            }
        }
        Self {
            base__: IPortableDeviceContent_Vtbl::new::<Identity, OFFSET>(),
            UpdateObjectWithPropertiesAndData: UpdateObjectWithPropertiesAndData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceContent2 as windows_core::Interface>::IID || iid == &<IPortableDeviceContent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
impl windows_core::RuntimeName for IPortableDeviceContent2 {}
#[cfg(feature = "objidlbase")]
windows_core::imp::define_interface!(IPortableDeviceDataStream, IPortableDeviceDataStream_Vtbl, 0x88e04db3_1012_4d64_9996_f703a950d3f4);
#[cfg(feature = "objidlbase")]
impl core::ops::Deref for IPortableDeviceDataStream {
    type Target = super::objidlbase::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidlbase")]
windows_core::imp::interface_hierarchy!(IPortableDeviceDataStream, windows_core::IUnknown, super::objidlbase::ISequentialStream, super::objidlbase::IStream);
#[cfg(feature = "objidlbase")]
impl IPortableDeviceDataStream {
    pub unsafe fn GetObjectID(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "objidlbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDataStream_Vtbl {
    pub base__: super::objidlbase::IStream_Vtbl,
    pub GetObjectID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
pub trait IPortableDeviceDataStream_Impl: super::objidlbase::IStream_Impl {
    fn GetObjectID(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl IPortableDeviceDataStream_Vtbl {
    pub const fn new<Identity: IPortableDeviceDataStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectID<Identity: IPortableDeviceDataStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceDataStream_Impl::GetObjectID(this) {
                    Ok(ok__) => {
                        ppszobjectid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceDataStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceDataStream_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: super::objidlbase::IStream_Vtbl::new::<Identity, OFFSET>(),
            GetObjectID: GetObjectID::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceDataStream as windows_core::Interface>::IID || iid == &<super::objidlbase::ISequentialStream as windows_core::Interface>::IID || iid == &<super::objidlbase::IStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl windows_core::RuntimeName for IPortableDeviceDataStream {}
windows_core::imp::define_interface!(IPortableDeviceDispatchFactory, IPortableDeviceDispatchFactory_Vtbl, 0x5e1eafc3_e3d7_4132_96fa_759c0f9d1e0f);
windows_core::imp::interface_hierarchy!(IPortableDeviceDispatchFactory, windows_core::IUnknown);
impl IPortableDeviceDispatchFactory {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetDeviceDispatch<P0>(&self, pszpnpdeviceid: P0) -> windows_core::Result<super::oaidl::IDispatch>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceDispatch)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDispatchFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub GetDeviceDispatch: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetDeviceDispatch: usize,
}
#[cfg(feature = "oaidl")]
pub trait IPortableDeviceDispatchFactory_Impl: windows_core::IUnknownImpl {
    fn GetDeviceDispatch(&self, pszpnpdeviceid: &windows_core::PCWSTR) -> windows_core::Result<super::oaidl::IDispatch>;
}
#[cfg(feature = "oaidl")]
impl IPortableDeviceDispatchFactory_Vtbl {
    pub const fn new<Identity: IPortableDeviceDispatchFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceDispatch<Identity: IPortableDeviceDispatchFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, ppdevicedispatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceDispatchFactory_Impl::GetDeviceDispatch(this, core::mem::transmute(&pszpnpdeviceid)) {
                    Ok(ok__) => {
                        ppdevicedispatch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDeviceDispatch: GetDeviceDispatch::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceDispatchFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IPortableDeviceDispatchFactory {}
windows_core::imp::define_interface!(IPortableDeviceEventCallback, IPortableDeviceEventCallback_Vtbl, 0xa8792a31_f385_493c_a893_40f64eb45f6e);
windows_core::imp::interface_hierarchy!(IPortableDeviceEventCallback, windows_core::IUnknown);
impl IPortableDeviceEventCallback {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn OnEvent<P0>(&self, peventparameters: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), peventparameters.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceEventCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    OnEvent: usize,
}
#[cfg(feature = "portabledevicetypes")]
pub trait IPortableDeviceEventCallback_Impl: windows_core::IUnknownImpl {
    fn OnEvent(&self, peventparameters: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<()>;
}
#[cfg(feature = "portabledevicetypes")]
impl IPortableDeviceEventCallback_Vtbl {
    pub const fn new<Identity: IPortableDeviceEventCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnEvent<Identity: IPortableDeviceEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceEventCallback_Impl::OnEvent(this, core::mem::transmute_copy(&peventparameters)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceEventCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "portabledevicetypes")]
impl windows_core::RuntimeName for IPortableDeviceEventCallback {}
windows_core::imp::define_interface!(IPortableDeviceManager, IPortableDeviceManager_Vtbl, 0xa1567595_4c2f_4574_a6fa_ecef917b9a40);
windows_core::imp::interface_hierarchy!(IPortableDeviceManager, windows_core::IUnknown);
impl IPortableDeviceManager {
    pub unsafe fn GetDevices(&self, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevices)(windows_core::Interface::as_raw(self), ppnpdeviceids as _, pcpnpdeviceids as _) }
    }
    pub unsafe fn RefreshDeviceList(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RefreshDeviceList)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDeviceFriendlyName<P0>(&self, pszpnpdeviceid: P0, pdevicefriendlyname: *mut u16, pcchdevicefriendlyname: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceFriendlyName)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), pdevicefriendlyname as _, pcchdevicefriendlyname as _) }
    }
    pub unsafe fn GetDeviceDescription<P0>(&self, pszpnpdeviceid: P0, pdevicedescription: *mut u16, pcchdevicedescription: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceDescription)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), pdevicedescription as _, pcchdevicedescription as _) }
    }
    pub unsafe fn GetDeviceManufacturer<P0>(&self, pszpnpdeviceid: P0, pdevicemanufacturer: *mut u16, pcchdevicemanufacturer: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceManufacturer)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), pdevicemanufacturer as _, pcchdevicemanufacturer as _) }
    }
    pub unsafe fn GetDeviceProperty<P0, P1>(&self, pszpnpdeviceid: P0, pszdevicepropertyname: P1, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceProperty)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), pszdevicepropertyname.param().abi(), pdata as _, pcbdata as _, pdwtype as _) }
    }
    pub unsafe fn GetPrivateDevices(&self, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateDevices)(windows_core::Interface::as_raw(self), ppnpdeviceids as _, pcpnpdeviceids as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub RefreshDeviceList: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut u8, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetPrivateDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
pub trait IPortableDeviceManager_Impl: windows_core::IUnknownImpl {
    fn GetDevices(&self, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::Result<()>;
    fn RefreshDeviceList(&self) -> windows_core::Result<()>;
    fn GetDeviceFriendlyName(&self, pszpnpdeviceid: &windows_core::PCWSTR, pdevicefriendlyname: *mut u16, pcchdevicefriendlyname: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceDescription(&self, pszpnpdeviceid: &windows_core::PCWSTR, pdevicedescription: *mut u16, pcchdevicedescription: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceManufacturer(&self, pszpnpdeviceid: &windows_core::PCWSTR, pdevicemanufacturer: *mut u16, pcchdevicemanufacturer: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceProperty(&self, pszpnpdeviceid: &windows_core::PCWSTR, pszdevicepropertyname: &windows_core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> windows_core::Result<()>;
    fn GetPrivateDevices(&self, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::Result<()>;
}
impl IPortableDeviceManager_Vtbl {
    pub const fn new<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevices<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetDevices(this, core::mem::transmute_copy(&ppnpdeviceids), core::mem::transmute_copy(&pcpnpdeviceids)).into()
            }
        }
        unsafe extern "system" fn RefreshDeviceList<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::RefreshDeviceList(this).into()
            }
        }
        unsafe extern "system" fn GetDeviceFriendlyName<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pdevicefriendlyname: *mut u16, pcchdevicefriendlyname: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetDeviceFriendlyName(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute_copy(&pdevicefriendlyname), core::mem::transmute_copy(&pcchdevicefriendlyname)).into()
            }
        }
        unsafe extern "system" fn GetDeviceDescription<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pdevicedescription: *mut u16, pcchdevicedescription: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetDeviceDescription(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute_copy(&pdevicedescription), core::mem::transmute_copy(&pcchdevicedescription)).into()
            }
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pdevicemanufacturer: *mut u16, pcchdevicemanufacturer: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetDeviceManufacturer(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute_copy(&pdevicemanufacturer), core::mem::transmute_copy(&pcchdevicemanufacturer)).into()
            }
        }
        unsafe extern "system" fn GetDeviceProperty<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pszdevicepropertyname: windows_core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetDeviceProperty(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute(&pszdevicepropertyname), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&pdwtype)).into()
            }
        }
        unsafe extern "system" fn GetPrivateDevices<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetPrivateDevices(this, core::mem::transmute_copy(&ppnpdeviceids), core::mem::transmute_copy(&pcpnpdeviceids)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevices: GetDevices::<Identity, OFFSET>,
            RefreshDeviceList: RefreshDeviceList::<Identity, OFFSET>,
            GetDeviceFriendlyName: GetDeviceFriendlyName::<Identity, OFFSET>,
            GetDeviceDescription: GetDeviceDescription::<Identity, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, OFFSET>,
            GetDeviceProperty: GetDeviceProperty::<Identity, OFFSET>,
            GetPrivateDevices: GetPrivateDevices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceManager {}
windows_core::imp::define_interface!(IPortableDeviceProperties, IPortableDeviceProperties_Vtbl, 0x7f6d695c_03df_4439_a809_59266beee3a6);
windows_core::imp::interface_hierarchy!(IPortableDeviceProperties, windows_core::IUnknown);
impl IPortableDeviceProperties {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedProperties<P0>(&self, pszobjectid: P0) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedProperties)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub unsafe fn GetPropertyAttributes<P0>(&self, pszobjectid: P0, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyAttributes)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetValues<P0, P1>(&self, pszobjectid: P0, pkeys: P1) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceKeyCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValues)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), pkeys.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn SetValues<P0, P1>(&self, pszobjectid: P0, pvalues: P1) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetValues)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), pvalues.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn Delete<P0, P1>(&self, pszobjectid: P0, pkeys: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceKeyCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), pkeys.param().abi()) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedProperties: usize,
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub GetPropertyAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::wtypes::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "portabledevicetypes", feature = "wtypes")))]
    GetPropertyAttributes: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetValues: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetValues: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub SetValues: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    SetValues: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    Delete: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
pub trait IPortableDeviceProperties_Impl: windows_core::IUnknownImpl {
    fn GetSupportedProperties(&self, pszobjectid: &windows_core::PCWSTR) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection>;
    fn GetPropertyAttributes(&self, pszobjectid: &windows_core::PCWSTR, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn GetValues(&self, pszobjectid: &windows_core::PCWSTR, pkeys: windows_core::Ref<super::portabledevicetypes::IPortableDeviceKeyCollection>) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn SetValues(&self, pszobjectid: &windows_core::PCWSTR, pvalues: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn Delete(&self, pszobjectid: &windows_core::PCWSTR, pkeys: windows_core::Ref<super::portabledevicetypes::IPortableDeviceKeyCollection>) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
impl IPortableDeviceProperties_Vtbl {
    pub const fn new<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedProperties<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceProperties_Impl::GetSupportedProperties(this, core::mem::transmute(&pszobjectid)) {
                    Ok(ok__) => {
                        ppkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyAttributes<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, key: *const super::wtypes::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceProperties_Impl::GetPropertyAttributes(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValues<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pkeys: *mut core::ffi::c_void, ppvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceProperties_Impl::GetValues(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&pkeys)) {
                    Ok(ok__) => {
                        ppvalues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValues<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pvalues: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceProperties_Impl::SetValues(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&pvalues)) {
                    Ok(ok__) => {
                        ppresults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pkeys: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceProperties_Impl::Delete(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&pkeys)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceProperties_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedProperties: GetSupportedProperties::<Identity, OFFSET>,
            GetPropertyAttributes: GetPropertyAttributes::<Identity, OFFSET>,
            GetValues: GetValues::<Identity, OFFSET>,
            SetValues: SetValues::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceProperties as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
impl windows_core::RuntimeName for IPortableDeviceProperties {}
windows_core::imp::define_interface!(IPortableDevicePropertiesBulk, IPortableDevicePropertiesBulk_Vtbl, 0x482b05c0_4056_44ed_9e0f_5e23b009da93);
windows_core::imp::interface_hierarchy!(IPortableDevicePropertiesBulk, windows_core::IUnknown);
impl IPortableDevicePropertiesBulk {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn QueueGetValuesByObjectList<P0, P1, P2>(&self, pobjectids: P0, pkeys: P1, pcallback: P2) -> windows_core::Result<windows_core::GUID>
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDevicePropVariantCollection>,
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceKeyCollection>,
        P2: windows_core::Param<IPortableDevicePropertiesBulkCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueGetValuesByObjectList)(windows_core::Interface::as_raw(self), pobjectids.param().abi(), pkeys.param().abi(), pcallback.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn QueueGetValuesByObjectFormat<P1, P3, P4>(&self, pguidobjectformat: *const windows_core::GUID, pszparentobjectid: P1, dwdepth: u32, pkeys: P3, pcallback: P4) -> windows_core::Result<windows_core::GUID>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::portabledevicetypes::IPortableDeviceKeyCollection>,
        P4: windows_core::Param<IPortableDevicePropertiesBulkCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueGetValuesByObjectFormat)(windows_core::Interface::as_raw(self), pguidobjectformat, pszparentobjectid.param().abi(), dwdepth, pkeys.param().abi(), pcallback.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn QueueSetValuesByObjectList<P0, P1>(&self, pobjectvalues: P0, pcallback: P1) -> windows_core::Result<windows_core::GUID>
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDeviceValuesCollection>,
        P1: windows_core::Param<IPortableDevicePropertiesBulkCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueSetValuesByObjectList)(windows_core::Interface::as_raw(self), pobjectvalues.param().abi(), pcallback.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Start(&self, pcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), pcontext) }
    }
    pub unsafe fn Cancel(&self, pcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), pcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulk_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub QueueGetValuesByObjectList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    QueueGetValuesByObjectList: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub QueueGetValuesByObjectFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    QueueGetValuesByObjectFormat: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub QueueSetValuesByObjectList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    QueueSetValuesByObjectList: usize,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "portabledevicetypes")]
pub trait IPortableDevicePropertiesBulk_Impl: windows_core::IUnknownImpl {
    fn QueueGetValuesByObjectList(&self, pobjectids: windows_core::Ref<super::portabledevicetypes::IPortableDevicePropVariantCollection>, pkeys: windows_core::Ref<super::portabledevicetypes::IPortableDeviceKeyCollection>, pcallback: windows_core::Ref<IPortableDevicePropertiesBulkCallback>) -> windows_core::Result<windows_core::GUID>;
    fn QueueGetValuesByObjectFormat(&self, pguidobjectformat: *const windows_core::GUID, pszparentobjectid: &windows_core::PCWSTR, dwdepth: u32, pkeys: windows_core::Ref<super::portabledevicetypes::IPortableDeviceKeyCollection>, pcallback: windows_core::Ref<IPortableDevicePropertiesBulkCallback>) -> windows_core::Result<windows_core::GUID>;
    fn QueueSetValuesByObjectList(&self, pobjectvalues: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValuesCollection>, pcallback: windows_core::Ref<IPortableDevicePropertiesBulkCallback>) -> windows_core::Result<windows_core::GUID>;
    fn Start(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Cancel(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "portabledevicetypes")]
impl IPortableDevicePropertiesBulk_Vtbl {
    pub const fn new<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueueGetValuesByObjectList<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectids: *mut core::ffi::c_void, pkeys: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pcontext: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevicePropertiesBulk_Impl::QueueGetValuesByObjectList(this, core::mem::transmute_copy(&pobjectids), core::mem::transmute_copy(&pkeys), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        pcontext.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueueGetValuesByObjectFormat<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidobjectformat: *const windows_core::GUID, pszparentobjectid: windows_core::PCWSTR, dwdepth: u32, pkeys: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pcontext: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevicePropertiesBulk_Impl::QueueGetValuesByObjectFormat(this, core::mem::transmute_copy(&pguidobjectformat), core::mem::transmute(&pszparentobjectid), core::mem::transmute_copy(&dwdepth), core::mem::transmute_copy(&pkeys), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        pcontext.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueueSetValuesByObjectList<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectvalues: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pcontext: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevicePropertiesBulk_Impl::QueueSetValuesByObjectList(this, core::mem::transmute_copy(&pobjectvalues), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        pcontext.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Start<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropertiesBulk_Impl::Start(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropertiesBulk_Impl::Cancel(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueueGetValuesByObjectList: QueueGetValuesByObjectList::<Identity, OFFSET>,
            QueueGetValuesByObjectFormat: QueueGetValuesByObjectFormat::<Identity, OFFSET>,
            QueueSetValuesByObjectList: QueueSetValuesByObjectList::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulk as windows_core::Interface>::IID
    }
}
#[cfg(feature = "portabledevicetypes")]
impl windows_core::RuntimeName for IPortableDevicePropertiesBulk {}
windows_core::imp::define_interface!(IPortableDevicePropertiesBulkCallback, IPortableDevicePropertiesBulkCallback_Vtbl, 0x9deacb80_11e8_40e3_a9f3_f557986a7845);
windows_core::imp::interface_hierarchy!(IPortableDevicePropertiesBulkCallback, windows_core::IUnknown);
impl IPortableDevicePropertiesBulkCallback {
    pub unsafe fn OnStart(&self, pcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStart)(windows_core::Interface::as_raw(self), pcontext) }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn OnProgress<P1>(&self, pcontext: *const windows_core::GUID, presults: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceValuesCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), pcontext, presults.param().abi()) }
    }
    pub unsafe fn OnEnd(&self, pcontext: *const windows_core::GUID, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnEnd)(windows_core::Interface::as_raw(self), pcontext, hrstatus) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulkCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "portabledevicetypes")]
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    OnProgress: usize,
    pub OnEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "portabledevicetypes")]
pub trait IPortableDevicePropertiesBulkCallback_Impl: windows_core::IUnknownImpl {
    fn OnStart(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnProgress(&self, pcontext: *const windows_core::GUID, presults: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValuesCollection>) -> windows_core::Result<()>;
    fn OnEnd(&self, pcontext: *const windows_core::GUID, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "portabledevicetypes")]
impl IPortableDevicePropertiesBulkCallback_Vtbl {
    pub const fn new<Identity: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStart<Identity: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropertiesBulkCallback_Impl::OnStart(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn OnProgress<Identity: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID, presults: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropertiesBulkCallback_Impl::OnProgress(this, core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&presults)).into()
            }
        }
        unsafe extern "system" fn OnEnd<Identity: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropertiesBulkCallback_Impl::OnEnd(this, core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStart: OnStart::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
            OnEnd: OnEnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulkCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "portabledevicetypes")]
impl windows_core::RuntimeName for IPortableDevicePropertiesBulkCallback {}
windows_core::imp::define_interface!(IPortableDeviceResources, IPortableDeviceResources_Vtbl, 0xfd8878ac_d841_4d17_891c_e6829cdb6934);
windows_core::imp::interface_hierarchy!(IPortableDeviceResources, windows_core::IUnknown);
impl IPortableDeviceResources {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedResources<P0>(&self, pszobjectid: P0) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedResources)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub unsafe fn GetResourceAttributes<P0>(&self, pszobjectid: P0, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourceAttributes)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "objidlbase", feature = "wtypes"))]
    pub unsafe fn GetStream<P0>(&self, pszobjectid: P0, key: *const super::wtypes::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut Option<super::objidlbase::IStream>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), key, dwmode, pdwoptimalbuffersize as _, core::mem::transmute(ppstream)) }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn Delete<P0, P1>(&self, pszobjectid: P0, pkeys: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceKeyCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), pkeys.param().abi()) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
    pub unsafe fn CreateResource<P0>(&self, presourceattributes: P0, ppdata: *mut Option<super::objidlbase::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateResource)(windows_core::Interface::as_raw(self), presourceattributes.param().abi(), core::mem::transmute(ppdata), pdwoptimalwritebuffersize as _, ppszcookie as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceResources_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedResources: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedResources: usize,
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub GetResourceAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::wtypes::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "portabledevicetypes", feature = "wtypes")))]
    GetResourceAttributes: usize,
    #[cfg(all(feature = "objidlbase", feature = "wtypes"))]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::wtypes::PROPERTYKEY, u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "wtypes")))]
    GetStream: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    Delete: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidlbase", feature = "portabledevicetypes"))]
    pub CreateResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "portabledevicetypes")))]
    CreateResource: usize,
}
#[cfg(all(feature = "objidlbase", feature = "portabledevicetypes", feature = "wtypes"))]
pub trait IPortableDeviceResources_Impl: windows_core::IUnknownImpl {
    fn GetSupportedResources(&self, pszobjectid: &windows_core::PCWSTR) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection>;
    fn GetResourceAttributes(&self, pszobjectid: &windows_core::PCWSTR, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn GetStream(&self, pszobjectid: &windows_core::PCWSTR, key: *const super::wtypes::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: windows_core::OutRef<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn Delete(&self, pszobjectid: &windows_core::PCWSTR, pkeys: windows_core::Ref<super::portabledevicetypes::IPortableDeviceKeyCollection>) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn CreateResource(&self, presourceattributes: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>, ppdata: windows_core::OutRef<super::objidlbase::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "portabledevicetypes", feature = "wtypes"))]
impl IPortableDeviceResources_Vtbl {
    pub const fn new<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedResources<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceResources_Impl::GetSupportedResources(this, core::mem::transmute(&pszobjectid)) {
                    Ok(ok__) => {
                        ppkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResourceAttributes<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, key: *const super::wtypes::PROPERTYKEY, ppresourceattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceResources_Impl::GetResourceAttributes(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppresourceattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStream<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, key: *const super::wtypes::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceResources_Impl::GetStream(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&key), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pdwoptimalbuffersize), core::mem::transmute_copy(&ppstream)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pkeys: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceResources_Impl::Delete(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&pkeys)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceResources_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn CreateResource<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourceattributes: *mut core::ffi::c_void, ppdata: *mut *mut core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceResources_Impl::CreateResource(this, core::mem::transmute_copy(&presourceattributes), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pdwoptimalwritebuffersize), core::mem::transmute_copy(&ppszcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedResources: GetSupportedResources::<Identity, OFFSET>,
            GetResourceAttributes: GetResourceAttributes::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            CreateResource: CreateResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceResources as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "portabledevicetypes", feature = "wtypes"))]
impl windows_core::RuntimeName for IPortableDeviceResources {}
windows_core::imp::define_interface!(IPortableDeviceService, IPortableDeviceService_Vtbl, 0xd3bd3a44_d7b5_40a9_98b7_2fa4d01dec08);
windows_core::imp::interface_hierarchy!(IPortableDeviceService, windows_core::IUnknown);
impl IPortableDeviceService {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn Open<P0, P1>(&self, pszpnpserviceid: P0, pclientinfo: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pszpnpserviceid.param().abi(), pclientinfo.param().abi()) }
    }
    pub unsafe fn Capabilities(&self) -> windows_core::Result<IPortableDeviceServiceCapabilities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Capabilities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Content(&self) -> windows_core::Result<IPortableDeviceContent2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Methods(&self) -> windows_core::Result<IPortableDeviceServiceMethods> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Methods)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetServiceObjectID(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceObjectID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPnPServiceID(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPnPServiceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn Advise<P1, P2>(&self, dwflags: u32, pcallback: P1, pparameters: P2) -> windows_core::Result<windows_core::PWSTR>
    where
        P1: windows_core::Param<IPortableDeviceEventCallback>,
        P2: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), dwflags, pcallback.param().abi(), pparameters.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise<P0>(&self, pszcookie: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), pszcookie.param().abi()) }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn SendCommand<P1>(&self, dwflags: u32, pparameters: P1) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>
    where
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendCommand)(windows_core::Interface::as_raw(self), dwflags, pparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    Open: usize,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Methods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetServiceObjectID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPnPServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "portabledevicetypes")]
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "portabledevicetypes")]
    pub SendCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    SendCommand: usize,
}
#[cfg(feature = "portabledevicetypes")]
pub trait IPortableDeviceService_Impl: windows_core::IUnknownImpl {
    fn Open(&self, pszpnpserviceid: &windows_core::PCWSTR, pclientinfo: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<()>;
    fn Capabilities(&self) -> windows_core::Result<IPortableDeviceServiceCapabilities>;
    fn Content(&self) -> windows_core::Result<IPortableDeviceContent2>;
    fn Methods(&self) -> windows_core::Result<IPortableDeviceServiceMethods>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetServiceObjectID(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPnPServiceID(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Advise(&self, dwflags: u32, pcallback: windows_core::Ref<IPortableDeviceEventCallback>, pparameters: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<windows_core::PWSTR>;
    fn Unadvise(&self, pszcookie: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SendCommand(&self, dwflags: u32, pparameters: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
}
#[cfg(feature = "portabledevicetypes")]
impl IPortableDeviceService_Vtbl {
    pub const fn new<Identity: IPortableDeviceService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpserviceid: windows_core::PCWSTR, pclientinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceService_Impl::Open(this, core::mem::transmute(&pszpnpserviceid), core::mem::transmute_copy(&pclientinfo)).into()
            }
        }
        unsafe extern "system" fn Capabilities<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::Capabilities(this) {
                    Ok(ok__) => {
                        ppcapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Content<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::Content(this) {
                    Ok(ok__) => {
                        ppcontent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Methods<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmethods: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::Methods(this) {
                    Ok(ok__) => {
                        ppmethods.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceService_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceService_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn GetServiceObjectID<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszserviceobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::GetServiceObjectID(this) {
                    Ok(ok__) => {
                        ppszserviceobjectid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPnPServiceID<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpnpserviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::GetPnPServiceID(this) {
                    Ok(ok__) => {
                        ppszpnpserviceid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Advise<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pcallback: *mut core::ffi::c_void, pparameters: *mut core::ffi::c_void, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::Advise(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pparameters)) {
                    Ok(ok__) => {
                        ppszcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcookie: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceService_Impl::Unadvise(this, core::mem::transmute(&pszcookie)).into()
            }
        }
        unsafe extern "system" fn SendCommand<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pparameters: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::SendCommand(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pparameters)) {
                    Ok(ok__) => {
                        ppresults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Capabilities: Capabilities::<Identity, OFFSET>,
            Content: Content::<Identity, OFFSET>,
            Methods: Methods::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetServiceObjectID: GetServiceObjectID::<Identity, OFFSET>,
            GetPnPServiceID: GetPnPServiceID::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            SendCommand: SendCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "portabledevicetypes")]
impl windows_core::RuntimeName for IPortableDeviceService {}
windows_core::imp::define_interface!(IPortableDeviceServiceActivation, IPortableDeviceServiceActivation_Vtbl, 0xe56b0534_d9b9_425c_9b99_75f97cb3d7c8);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceActivation, windows_core::IUnknown);
impl IPortableDeviceServiceActivation {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn OpenAsync<P0, P1, P2>(&self, pszpnpserviceid: P0, pclientinfo: P1, pcallback: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
        P2: windows_core::Param<IPortableDeviceServiceOpenCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenAsync)(windows_core::Interface::as_raw(self), pszpnpserviceid.param().abi(), pclientinfo.param().abi(), pcallback.param().abi()) }
    }
    pub unsafe fn CancelOpenAsync(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelOpenAsync)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceActivation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub OpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    OpenAsync: usize,
    pub CancelOpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "portabledevicetypes")]
pub trait IPortableDeviceServiceActivation_Impl: windows_core::IUnknownImpl {
    fn OpenAsync(&self, pszpnpserviceid: &windows_core::PCWSTR, pclientinfo: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>, pcallback: windows_core::Ref<IPortableDeviceServiceOpenCallback>) -> windows_core::Result<()>;
    fn CancelOpenAsync(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "portabledevicetypes")]
impl IPortableDeviceServiceActivation_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceActivation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenAsync<Identity: IPortableDeviceServiceActivation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpserviceid: windows_core::PCWSTR, pclientinfo: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceActivation_Impl::OpenAsync(this, core::mem::transmute(&pszpnpserviceid), core::mem::transmute_copy(&pclientinfo), core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn CancelOpenAsync<Identity: IPortableDeviceServiceActivation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceActivation_Impl::CancelOpenAsync(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenAsync: OpenAsync::<Identity, OFFSET>,
            CancelOpenAsync: CancelOpenAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceActivation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "portabledevicetypes")]
impl windows_core::RuntimeName for IPortableDeviceServiceActivation {}
windows_core::imp::define_interface!(IPortableDeviceServiceCapabilities, IPortableDeviceServiceCapabilities_Vtbl, 0x24dbd89d_413e_43e0_bd5b_197f3c56c886);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceCapabilities, windows_core::IUnknown);
impl IPortableDeviceServiceCapabilities {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedMethods(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedMethods)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedMethodsByFormat(&self, format: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedMethodsByFormat)(windows_core::Interface::as_raw(self), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetMethodAttributes(&self, method: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMethodAttributes)(windows_core::Interface::as_raw(self), method, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub unsafe fn GetMethodParameterAttributes(&self, method: *const windows_core::GUID, parameter: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMethodParameterAttributes)(windows_core::Interface::as_raw(self), method, parameter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedFormats(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedFormats)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetFormatAttributes(&self, format: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormatAttributes)(windows_core::Interface::as_raw(self), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedFormatProperties)(windows_core::Interface::as_raw(self), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub unsafe fn GetFormatPropertyAttributes(&self, format: *const windows_core::GUID, property: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormatPropertyAttributes)(windows_core::Interface::as_raw(self), format, property, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedEvents(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedEvents)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetEventAttributes(&self, event: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventAttributes)(windows_core::Interface::as_raw(self), event, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub unsafe fn GetEventParameterAttributes(&self, event: *const windows_core::GUID, parameter: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventParameterAttributes)(windows_core::Interface::as_raw(self), event, parameter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetInheritedServices(&self, dwinheritancetype: u32) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInheritedServices)(windows_core::Interface::as_raw(self), dwinheritancetype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetFormatRenderingProfiles(&self, format: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValuesCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormatRenderingProfiles)(windows_core::Interface::as_raw(self), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn GetSupportedCommands(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedCommands)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub unsafe fn GetCommandOptions(&self, command: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCommandOptions)(windows_core::Interface::as_raw(self), command, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceCapabilities_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedMethods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedMethods: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedMethodsByFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedMethodsByFormat: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetMethodAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetMethodAttributes: usize,
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub GetMethodParameterAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::wtypes::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "portabledevicetypes", feature = "wtypes")))]
    GetMethodParameterAttributes: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedFormats: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetFormatAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetFormatAttributes: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedFormatProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedFormatProperties: usize,
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub GetFormatPropertyAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::wtypes::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "portabledevicetypes", feature = "wtypes")))]
    GetFormatPropertyAttributes: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedEvents: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetEventAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetEventAttributes: usize,
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub GetEventParameterAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::wtypes::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "portabledevicetypes", feature = "wtypes")))]
    GetEventParameterAttributes: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetInheritedServices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetInheritedServices: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetFormatRenderingProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetFormatRenderingProfiles: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub GetSupportedCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    GetSupportedCommands: usize,
    #[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
    pub GetCommandOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "portabledevicetypes", feature = "wtypes")))]
    GetCommandOptions: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
pub trait IPortableDeviceServiceCapabilities_Impl: windows_core::IUnknownImpl {
    fn GetSupportedMethods(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>;
    fn GetSupportedMethodsByFormat(&self, format: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>;
    fn GetMethodAttributes(&self, method: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn GetMethodParameterAttributes(&self, method: *const windows_core::GUID, parameter: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn GetSupportedFormats(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>;
    fn GetFormatAttributes(&self, format: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn GetSupportedFormatProperties(&self, format: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection>;
    fn GetFormatPropertyAttributes(&self, format: *const windows_core::GUID, property: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn GetSupportedEvents(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>;
    fn GetEventAttributes(&self, event: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn GetEventParameterAttributes(&self, event: *const windows_core::GUID, parameter: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn GetInheritedServices(&self, dwinheritancetype: u32) -> windows_core::Result<super::portabledevicetypes::IPortableDevicePropVariantCollection>;
    fn GetFormatRenderingProfiles(&self, format: *const windows_core::GUID) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValuesCollection>;
    fn GetSupportedCommands(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection>;
    fn GetCommandOptions(&self, command: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
impl IPortableDeviceServiceCapabilities_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedMethods<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmethods: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedMethods(this) {
                    Ok(ok__) => {
                        ppmethods.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedMethodsByFormat<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppmethods: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedMethodsByFormat(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        ppmethods.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMethodAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetMethodAttributes(this, core::mem::transmute_copy(&method)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMethodParameterAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, parameter: *const super::wtypes::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetMethodParameterAttributes(this, core::mem::transmute_copy(&method), core::mem::transmute_copy(&parameter)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppformats: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedFormats(this) {
                    Ok(ok__) => {
                        ppformats.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormatAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetFormatAttributes(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedFormatProperties(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        ppkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormatPropertyAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, property: *const super::wtypes::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetFormatPropertyAttributes(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&property)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedEvents<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppevents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedEvents(this) {
                    Ok(ok__) => {
                        ppevents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const windows_core::GUID, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetEventAttributes(this, core::mem::transmute_copy(&event)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventParameterAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const windows_core::GUID, parameter: *const super::wtypes::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetEventParameterAttributes(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&parameter)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInheritedServices<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinheritancetype: u32, ppservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetInheritedServices(this, core::mem::transmute_copy(&dwinheritancetype)) {
                    Ok(ok__) => {
                        ppservices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormatRenderingProfiles<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, pprenderingprofiles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetFormatRenderingProfiles(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        pprenderingprofiles.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedCommands<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcommands: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedCommands(this) {
                    Ok(ok__) => {
                        ppcommands.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCommandOptions<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, command: *const super::wtypes::PROPERTYKEY, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetCommandOptions(this, core::mem::transmute_copy(&command)) {
                    Ok(ok__) => {
                        ppoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceCapabilities_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedMethods: GetSupportedMethods::<Identity, OFFSET>,
            GetSupportedMethodsByFormat: GetSupportedMethodsByFormat::<Identity, OFFSET>,
            GetMethodAttributes: GetMethodAttributes::<Identity, OFFSET>,
            GetMethodParameterAttributes: GetMethodParameterAttributes::<Identity, OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Identity, OFFSET>,
            GetFormatAttributes: GetFormatAttributes::<Identity, OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Identity, OFFSET>,
            GetFormatPropertyAttributes: GetFormatPropertyAttributes::<Identity, OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Identity, OFFSET>,
            GetEventAttributes: GetEventAttributes::<Identity, OFFSET>,
            GetEventParameterAttributes: GetEventParameterAttributes::<Identity, OFFSET>,
            GetInheritedServices: GetInheritedServices::<Identity, OFFSET>,
            GetFormatRenderingProfiles: GetFormatRenderingProfiles::<Identity, OFFSET>,
            GetSupportedCommands: GetSupportedCommands::<Identity, OFFSET>,
            GetCommandOptions: GetCommandOptions::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "portabledevicetypes", feature = "wtypes"))]
impl windows_core::RuntimeName for IPortableDeviceServiceCapabilities {}
windows_core::imp::define_interface!(IPortableDeviceServiceManager, IPortableDeviceServiceManager_Vtbl, 0xa8abc4e9_a84a_47a9_80b3_c5d9b172a961);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceManager, windows_core::IUnknown);
impl IPortableDeviceServiceManager {
    pub unsafe fn GetDeviceServices<P0>(&self, pszpnpdeviceid: P0, guidservicecategory: *const windows_core::GUID, pservices: *mut windows_core::PWSTR, pcservices: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceServices)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), guidservicecategory, pservices as _, pcservices as _) }
    }
    pub unsafe fn GetDeviceForService<P0>(&self, pszpnpserviceid: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceForService)(windows_core::Interface::as_raw(self), pszpnpserviceid.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeviceServices: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceForService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IPortableDeviceServiceManager_Impl: windows_core::IUnknownImpl {
    fn GetDeviceServices(&self, pszpnpdeviceid: &windows_core::PCWSTR, guidservicecategory: *const windows_core::GUID, pservices: *mut windows_core::PWSTR, pcservices: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceForService(&self, pszpnpserviceid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl IPortableDeviceServiceManager_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceServices<Identity: IPortableDeviceServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, guidservicecategory: *const windows_core::GUID, pservices: *mut windows_core::PWSTR, pcservices: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceManager_Impl::GetDeviceServices(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute_copy(&guidservicecategory), core::mem::transmute_copy(&pservices), core::mem::transmute_copy(&pcservices)).into()
            }
        }
        unsafe extern "system" fn GetDeviceForService<Identity: IPortableDeviceServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpserviceid: windows_core::PCWSTR, ppszpnpdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceManager_Impl::GetDeviceForService(this, core::mem::transmute(&pszpnpserviceid)) {
                    Ok(ok__) => {
                        ppszpnpdeviceid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceServices: GetDeviceServices::<Identity, OFFSET>,
            GetDeviceForService: GetDeviceForService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceServiceManager {}
windows_core::imp::define_interface!(IPortableDeviceServiceMethodCallback, IPortableDeviceServiceMethodCallback_Vtbl, 0xc424233c_afce_4828_a756_7ed7a2350083);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceMethodCallback, windows_core::IUnknown);
impl IPortableDeviceServiceMethodCallback {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn OnComplete<P1>(&self, hrstatus: windows_core::HRESULT, presults: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnComplete)(windows_core::Interface::as_raw(self), hrstatus, presults.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethodCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub OnComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    OnComplete: usize,
}
#[cfg(feature = "portabledevicetypes")]
pub trait IPortableDeviceServiceMethodCallback_Impl: windows_core::IUnknownImpl {
    fn OnComplete(&self, hrstatus: windows_core::HRESULT, presults: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<()>;
}
#[cfg(feature = "portabledevicetypes")]
impl IPortableDeviceServiceMethodCallback_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceMethodCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnComplete<Identity: IPortableDeviceServiceMethodCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, presults: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceMethodCallback_Impl::OnComplete(this, core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&presults)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethodCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "portabledevicetypes")]
impl windows_core::RuntimeName for IPortableDeviceServiceMethodCallback {}
windows_core::imp::define_interface!(IPortableDeviceServiceMethods, IPortableDeviceServiceMethods_Vtbl, 0xe20333c9_fd34_412d_a381_cc6f2d820df7);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceMethods, windows_core::IUnknown);
impl IPortableDeviceServiceMethods {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn Invoke<P1>(&self, method: *const windows_core::GUID, pparameters: P1, ppresults: *mut Option<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), method, pparameters.param().abi(), core::mem::transmute(ppresults)) }
    }
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn InvokeAsync<P1, P2>(&self, method: *const windows_core::GUID, pparameters: P1, pcallback: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
        P2: windows_core::Param<IPortableDeviceServiceMethodCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).InvokeAsync)(windows_core::Interface::as_raw(self), method, pparameters.param().abi(), pcallback.param().abi()) }
    }
    pub unsafe fn Cancel<P0>(&self, pcallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPortableDeviceServiceMethodCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), pcallback.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethods_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    Invoke: usize,
    #[cfg(feature = "portabledevicetypes")]
    pub InvokeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    InvokeAsync: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "portabledevicetypes")]
pub trait IPortableDeviceServiceMethods_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, method: *const windows_core::GUID, pparameters: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>, ppresults: windows_core::OutRef<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<()>;
    fn InvokeAsync(&self, method: *const windows_core::GUID, pparameters: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>, pcallback: windows_core::Ref<IPortableDeviceServiceMethodCallback>) -> windows_core::Result<()>;
    fn Cancel(&self, pcallback: windows_core::Ref<IPortableDeviceServiceMethodCallback>) -> windows_core::Result<()>;
}
#[cfg(feature = "portabledevicetypes")]
impl IPortableDeviceServiceMethods_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, pparameters: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceMethods_Impl::Invoke(this, core::mem::transmute_copy(&method), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&ppresults)).into()
            }
        }
        unsafe extern "system" fn InvokeAsync<Identity: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, pparameters: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceMethods_Impl::InvokeAsync(this, core::mem::transmute_copy(&method), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceMethods_Impl::Cancel(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
            InvokeAsync: InvokeAsync::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethods as windows_core::Interface>::IID
    }
}
#[cfg(feature = "portabledevicetypes")]
impl windows_core::RuntimeName for IPortableDeviceServiceMethods {}
windows_core::imp::define_interface!(IPortableDeviceServiceOpenCallback, IPortableDeviceServiceOpenCallback_Vtbl, 0xbced49c8_8efe_41ed_960b_61313abd47a9);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceOpenCallback, windows_core::IUnknown);
impl IPortableDeviceServiceOpenCallback {
    pub unsafe fn OnComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnComplete)(windows_core::Interface::as_raw(self), hrstatus) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceOpenCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IPortableDeviceServiceOpenCallback_Impl: windows_core::IUnknownImpl {
    fn OnComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IPortableDeviceServiceOpenCallback_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceOpenCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnComplete<Identity: IPortableDeviceServiceOpenCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceOpenCallback_Impl::OnComplete(this, core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceOpenCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceServiceOpenCallback {}
windows_core::imp::define_interface!(IPortableDeviceUnitsStream, IPortableDeviceUnitsStream_Vtbl, 0x5e98025f_bfc4_47a2_9a5f_bc900a507c67);
windows_core::imp::interface_hierarchy!(IPortableDeviceUnitsStream, windows_core::IUnknown);
impl IPortableDeviceUnitsStream {
    #[cfg(feature = "portabledevicetypes")]
    pub unsafe fn SeekInUnits(&self, dlibmove: i64, units: super::portabledevicetypes::WPD_STREAM_UNITS, dworigin: u32, plibnewposition: Option<*mut u64>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SeekInUnits)(windows_core::Interface::as_raw(self), dlibmove, units, dworigin, plibnewposition.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceUnitsStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "portabledevicetypes")]
    pub SeekInUnits: unsafe extern "system" fn(*mut core::ffi::c_void, i64, super::portabledevicetypes::WPD_STREAM_UNITS, u32, *mut u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "portabledevicetypes"))]
    SeekInUnits: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "portabledevicetypes")]
pub trait IPortableDeviceUnitsStream_Impl: windows_core::IUnknownImpl {
    fn SeekInUnits(&self, dlibmove: i64, units: super::portabledevicetypes::WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "portabledevicetypes")]
impl IPortableDeviceUnitsStream_Vtbl {
    pub const fn new<Identity: IPortableDeviceUnitsStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SeekInUnits<Identity: IPortableDeviceUnitsStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dlibmove: i64, units: super::portabledevicetypes::WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceUnitsStream_Impl::SeekInUnits(this, core::mem::transmute_copy(&dlibmove), core::mem::transmute_copy(&units), core::mem::transmute_copy(&dworigin), core::mem::transmute_copy(&plibnewposition)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceUnitsStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceUnitsStream_Impl::Cancel(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SeekInUnits: SeekInUnits::<Identity, OFFSET>, Cancel: Cancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceUnitsStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "portabledevicetypes")]
impl windows_core::RuntimeName for IPortableDeviceUnitsStream {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IPortableDeviceWebControl, IPortableDeviceWebControl_Vtbl, 0x94fc7953_5ca1_483a_8aee_df52e7747d00);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IPortableDeviceWebControl {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IPortableDeviceWebControl, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IPortableDeviceWebControl {
    pub unsafe fn GetDeviceFromId(&self, deviceid: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceFromId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDeviceFromIdAsync<P1, P2>(&self, deviceid: &windows_core::BSTR, pcompletionhandler: P1, perrorhandler: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::oaidl::IDispatch>,
        P2: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceFromIdAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(deviceid), pcompletionhandler.param().abi(), perrorhandler.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceWebControl_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub GetDeviceFromId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceFromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPortableDeviceWebControl_Impl: super::oaidl::IDispatch_Impl {
    fn GetDeviceFromId(&self, deviceid: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch>;
    fn GetDeviceFromIdAsync(&self, deviceid: &windows_core::BSTR, pcompletionhandler: windows_core::Ref<super::oaidl::IDispatch>, perrorhandler: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPortableDeviceWebControl_Vtbl {
    pub const fn new<Identity: IPortableDeviceWebControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceFromId<Identity: IPortableDeviceWebControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceWebControl_Impl::GetDeviceFromId(this, core::mem::transmute(&deviceid)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceFromIdAsync<Identity: IPortableDeviceWebControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: *mut core::ffi::c_void, pcompletionhandler: *mut core::ffi::c_void, perrorhandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceWebControl_Impl::GetDeviceFromIdAsync(this, core::mem::transmute(&deviceid), core::mem::transmute_copy(&pcompletionhandler), core::mem::transmute_copy(&perrorhandler)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceFromId: GetDeviceFromId::<Identity, OFFSET>,
            GetDeviceFromIdAsync: GetDeviceFromIdAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceWebControl as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPortableDeviceWebControl {}
pub const PortableDevice: windows_core::GUID = windows_core::GUID::from_u128(0x728a21c5_3d9e_48d7_9810_864848f0f404);
pub const PortableDeviceDispatchFactory: windows_core::GUID = windows_core::GUID::from_u128(0x43232233_8338_4658_ae01_0b4ae830b6b0);
pub const PortableDeviceFTM: windows_core::GUID = windows_core::GUID::from_u128(0xf7c0039a_4762_488a_b4b3_760ef9a1ba9b);
pub const PortableDeviceManager: windows_core::GUID = windows_core::GUID::from_u128(0x0af10cec_2ecd_4b92_9581_34f6ae0637f3);
pub const PortableDeviceService: windows_core::GUID = windows_core::GUID::from_u128(0xef5db4c2_9312_422c_9152_411cd9c4dd84);
pub const PortableDeviceServiceFTM: windows_core::GUID = windows_core::GUID::from_u128(0x1649b154_c794_497a_9b03_f3f0121302f3);
pub const PortableDeviceWebControl: windows_core::GUID = windows_core::GUID::from_u128(0x186dd02c_2dec_41b5_a7d4_b59056fade51);
