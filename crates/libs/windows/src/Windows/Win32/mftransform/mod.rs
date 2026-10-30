#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateTransformActivate(ppactivate: *mut Option<super::IMFActivate>) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFCreateTransformActivate(ppactivate : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFCreateTransformActivate(core::mem::transmute(ppactivate)) }
}
pub type DeviceStreamState = i32;
pub const DeviceStreamState_Disabled: DeviceStreamState = 3;
pub const DeviceStreamState_Pause: DeviceStreamState = 1;
pub const DeviceStreamState_Run: DeviceStreamState = 2;
pub const DeviceStreamState_Stop: DeviceStreamState = 0;
windows_core::imp::define_interface!(IMFDeviceTransform, IMFDeviceTransform_Vtbl, 0xd818fbd8_fc46_42f2_87ac_1ea2d1f9bf32);
windows_core::imp::interface_hierarchy!(IMFDeviceTransform, windows_core::IUnknown);
impl IMFDeviceTransform {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn InitializeTransform<P0>(&self, pattributes: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeTransform)(windows_core::Interface::as_raw(self), pattributes.param().abi()) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetInputAvailableType(&self, dwinputstreamid: super::DWORD, dwtypeindex: super::DWORD) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputAvailableType)(windows_core::Interface::as_raw(self), dwinputstreamid, dwtypeindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamid: super::DWORD) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputCurrentType)(windows_core::Interface::as_raw(self), dwinputstreamid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetInputStreamAttributes(&self, dwinputstreamid: super::DWORD) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamAttributes)(windows_core::Interface::as_raw(self), dwinputstreamid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetOutputAvailableType(&self, dwoutputstreamid: super::DWORD, dwtypeindex: super::DWORD) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputAvailableType)(windows_core::Interface::as_raw(self), dwoutputstreamid, dwtypeindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamid: super::DWORD) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputCurrentType)(windows_core::Interface::as_raw(self), dwoutputstreamid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetOutputStreamAttributes(&self, dwoutputstreamid: super::DWORD) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamAttributes)(windows_core::Interface::as_raw(self), dwoutputstreamid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut super::DWORD, pcoutputstreams: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), pcinputstreams as _, pcoutputstreams as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamIDs(&self, dwinputidarraysize: super::DWORD, pdwinputstreamids: *mut super::DWORD, dwoutputidarraysize: super::DWORD, pdwoutputstreamids: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamIDs)(windows_core::Interface::as_raw(self), dwinputidarraysize, pdwinputstreamids as _, dwoutputidarraysize, pdwoutputstreamids as _) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn ProcessEvent<P1>(&self, dwinputstreamid: super::DWORD, pevent: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessEvent)(windows_core::Interface::as_raw(self), dwinputstreamid, pevent.param().abi()) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn ProcessInput<P1>(&self, dwinputstreamid: super::DWORD, psample: P1, dwflags: super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessInput)(windows_core::Interface::as_raw(self), dwinputstreamid, psample.param().abi(), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ProcessMessage(&self, emessage: MFT_MESSAGE_TYPE, ulparam: super::ULONG_PTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessMessage)(windows_core::Interface::as_raw(self), emessage, ulparam) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn ProcessOutput(&self, dwflags: super::DWORD, coutputbuffercount: super::DWORD, poutputsample: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessOutput)(windows_core::Interface::as_raw(self), dwflags, coutputbuffercount, poutputsample, pdwstatus as _) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn SetInputStreamState<P1>(&self, dwstreamid: super::DWORD, pmediatype: P1, value: DeviceStreamState, dwflags: super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputStreamState)(windows_core::Interface::as_raw(self), dwstreamid, pmediatype.param().abi(), value, dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetInputStreamState(&self, dwstreamid: super::DWORD, value: *mut DeviceStreamState) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputStreamState)(windows_core::Interface::as_raw(self), dwstreamid, value as _) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn SetOutputStreamState<P1>(&self, dwstreamid: super::DWORD, pmediatype: P1, value: DeviceStreamState, dwflags: super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputStreamState)(windows_core::Interface::as_raw(self), dwstreamid, pmediatype.param().abi(), value, dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputStreamState(&self, dwstreamid: super::DWORD, value: *mut DeviceStreamState) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputStreamState)(windows_core::Interface::as_raw(self), dwstreamid, value as _) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetInputStreamPreferredState(&self, dwstreamid: super::DWORD, value: *mut DeviceStreamState) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamPreferredState)(windows_core::Interface::as_raw(self), dwstreamid, value as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn FlushInputStream(&self, dwstreamindex: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FlushInputStream)(windows_core::Interface::as_raw(self), dwstreamindex, dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn FlushOutputStream(&self, dwstreamindex: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FlushOutputStream)(windows_core::Interface::as_raw(self), dwstreamindex, dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDeviceTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub InitializeTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    InitializeTransform: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetInputAvailableType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetInputAvailableType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetInputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetInputCurrentType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetInputStreamAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetInputStreamAttributes: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetOutputAvailableType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetOutputAvailableType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetOutputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetOutputCurrentType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetOutputStreamAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetOutputStreamAttributes: usize,
    #[cfg(feature = "shared")]
    pub GetStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamCount: usize,
    #[cfg(feature = "shared")]
    pub GetStreamIDs: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamIDs: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub ProcessEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    ProcessEvent: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub ProcessInput: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    ProcessInput: usize,
    #[cfg(feature = "shared")]
    pub ProcessMessage: unsafe extern "system" fn(*mut core::ffi::c_void, MFT_MESSAGE_TYPE, super::ULONG_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ProcessMessage: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub ProcessOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut MFT_OUTPUT_DATA_BUFFER, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    ProcessOutput: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub SetInputStreamState: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, DeviceStreamState, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    SetInputStreamState: usize,
    #[cfg(feature = "shared")]
    pub GetInputStreamState: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut DeviceStreamState) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInputStreamState: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub SetOutputStreamState: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, DeviceStreamState, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    SetOutputStreamState: usize,
    #[cfg(feature = "shared")]
    pub GetOutputStreamState: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut DeviceStreamState) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputStreamState: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetInputStreamPreferredState: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut DeviceStreamState, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetInputStreamPreferredState: usize,
    #[cfg(feature = "shared")]
    pub FlushInputStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    FlushInputStream: usize,
    #[cfg(feature = "shared")]
    pub FlushOutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    FlushOutputStream: usize,
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
pub trait IMFDeviceTransform_Impl: windows_core::IUnknownImpl {
    fn InitializeTransform(&self, pattributes: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<()>;
    fn GetInputAvailableType(&self, dwinputstreamid: super::DWORD, dwtypeindex: super::DWORD) -> windows_core::Result<super::IMFMediaType>;
    fn GetInputCurrentType(&self, dwinputstreamid: super::DWORD) -> windows_core::Result<super::IMFMediaType>;
    fn GetInputStreamAttributes(&self, dwinputstreamid: super::DWORD) -> windows_core::Result<super::IMFAttributes>;
    fn GetOutputAvailableType(&self, dwoutputstreamid: super::DWORD, dwtypeindex: super::DWORD) -> windows_core::Result<super::IMFMediaType>;
    fn GetOutputCurrentType(&self, dwoutputstreamid: super::DWORD) -> windows_core::Result<super::IMFMediaType>;
    fn GetOutputStreamAttributes(&self, dwoutputstreamid: super::DWORD) -> windows_core::Result<super::IMFAttributes>;
    fn GetStreamCount(&self, pcinputstreams: *mut super::DWORD, pcoutputstreams: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetStreamIDs(&self, dwinputidarraysize: super::DWORD, pdwinputstreamids: *mut super::DWORD, dwoutputidarraysize: super::DWORD, pdwoutputstreamids: *mut super::DWORD) -> windows_core::Result<()>;
    fn ProcessEvent(&self, dwinputstreamid: super::DWORD, pevent: windows_core::Ref<super::IMFMediaEvent>) -> windows_core::Result<()>;
    fn ProcessInput(&self, dwinputstreamid: super::DWORD, psample: windows_core::Ref<super::IMFSample>, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn ProcessMessage(&self, emessage: MFT_MESSAGE_TYPE, ulparam: super::ULONG_PTR) -> windows_core::Result<()>;
    fn ProcessOutput(&self, dwflags: super::DWORD, coutputbuffercount: super::DWORD, poutputsample: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetInputStreamState(&self, dwstreamid: super::DWORD, pmediatype: windows_core::Ref<super::IMFMediaType>, value: DeviceStreamState, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn GetInputStreamState(&self, dwstreamid: super::DWORD, value: *mut DeviceStreamState) -> windows_core::Result<()>;
    fn SetOutputStreamState(&self, dwstreamid: super::DWORD, pmediatype: windows_core::Ref<super::IMFMediaType>, value: DeviceStreamState, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn GetOutputStreamState(&self, dwstreamid: super::DWORD, value: *mut DeviceStreamState) -> windows_core::Result<()>;
    fn GetInputStreamPreferredState(&self, dwstreamid: super::DWORD, value: *mut DeviceStreamState) -> windows_core::Result<super::IMFMediaType>;
    fn FlushInputStream(&self, dwstreamindex: super::DWORD, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn FlushOutputStream(&self, dwstreamindex: super::DWORD, dwflags: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl IMFDeviceTransform_Vtbl {
    pub const fn new<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeTransform<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::InitializeTransform(this, core::mem::transmute_copy(&pattributes)).into()
            }
        }
        unsafe extern "system" fn GetInputAvailableType<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, dwtypeindex: super::DWORD, pmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetInputAvailableType(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&dwtypeindex)) {
                    Ok(ok__) => {
                        pmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        pmediatype.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetInputCurrentType<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, pmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetInputCurrentType(this, core::mem::transmute_copy(&dwinputstreamid)) {
                    Ok(ok__) => {
                        pmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        pmediatype.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetInputStreamAttributes<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetInputStreamAttributes(this, core::mem::transmute_copy(&dwinputstreamid)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppattributes.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetOutputAvailableType<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: super::DWORD, dwtypeindex: super::DWORD, pmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetOutputAvailableType(this, core::mem::transmute_copy(&dwoutputstreamid), core::mem::transmute_copy(&dwtypeindex)) {
                    Ok(ok__) => {
                        pmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        pmediatype.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: super::DWORD, pmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetOutputCurrentType(this, core::mem::transmute_copy(&dwoutputstreamid)) {
                    Ok(ok__) => {
                        pmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        pmediatype.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetOutputStreamAttributes<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: super::DWORD, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetOutputStreamAttributes(this, core::mem::transmute_copy(&dwoutputstreamid)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppattributes.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetStreamCount<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinputstreams: *mut super::DWORD, pcoutputstreams: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::GetStreamCount(this, core::mem::transmute_copy(&pcinputstreams), core::mem::transmute_copy(&pcoutputstreams)).into()
            }
        }
        unsafe extern "system" fn GetStreamIDs<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputidarraysize: super::DWORD, pdwinputstreamids: *mut super::DWORD, dwoutputidarraysize: super::DWORD, pdwoutputstreamids: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::GetStreamIDs(this, core::mem::transmute_copy(&dwinputidarraysize), core::mem::transmute_copy(&pdwinputstreamids), core::mem::transmute_copy(&dwoutputidarraysize), core::mem::transmute_copy(&pdwoutputstreamids)).into()
            }
        }
        unsafe extern "system" fn ProcessEvent<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::ProcessEvent(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&pevent)).into()
            }
        }
        unsafe extern "system" fn ProcessInput<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, psample: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::ProcessInput(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&psample), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn ProcessMessage<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emessage: MFT_MESSAGE_TYPE, ulparam: super::ULONG_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::ProcessMessage(this, core::mem::transmute_copy(&emessage), core::mem::transmute_copy(&ulparam)).into()
            }
        }
        unsafe extern "system" fn ProcessOutput<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, coutputbuffercount: super::DWORD, poutputsample: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::ProcessOutput(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&coutputbuffercount), core::mem::transmute_copy(&poutputsample), core::mem::transmute_copy(&pdwstatus)).into()
            }
        }
        unsafe extern "system" fn SetInputStreamState<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: super::DWORD, pmediatype: *mut core::ffi::c_void, value: DeviceStreamState, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::SetInputStreamState(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&pmediatype), core::mem::transmute_copy(&value), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetInputStreamState<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: super::DWORD, value: *mut DeviceStreamState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::GetInputStreamState(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetOutputStreamState<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: super::DWORD, pmediatype: *mut core::ffi::c_void, value: DeviceStreamState, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::SetOutputStreamState(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&pmediatype), core::mem::transmute_copy(&value), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetOutputStreamState<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: super::DWORD, value: *mut DeviceStreamState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::GetOutputStreamState(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetInputStreamPreferredState<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: super::DWORD, value: *mut DeviceStreamState, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetInputStreamPreferredState(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        ppmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppmediatype.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn FlushInputStream<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::FlushInputStream(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn FlushOutputStream<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::FlushOutputStream(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeTransform: InitializeTransform::<Identity, OFFSET>,
            GetInputAvailableType: GetInputAvailableType::<Identity, OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Identity, OFFSET>,
            GetInputStreamAttributes: GetInputStreamAttributes::<Identity, OFFSET>,
            GetOutputAvailableType: GetOutputAvailableType::<Identity, OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Identity, OFFSET>,
            GetOutputStreamAttributes: GetOutputStreamAttributes::<Identity, OFFSET>,
            GetStreamCount: GetStreamCount::<Identity, OFFSET>,
            GetStreamIDs: GetStreamIDs::<Identity, OFFSET>,
            ProcessEvent: ProcessEvent::<Identity, OFFSET>,
            ProcessInput: ProcessInput::<Identity, OFFSET>,
            ProcessMessage: ProcessMessage::<Identity, OFFSET>,
            ProcessOutput: ProcessOutput::<Identity, OFFSET>,
            SetInputStreamState: SetInputStreamState::<Identity, OFFSET>,
            GetInputStreamState: GetInputStreamState::<Identity, OFFSET>,
            SetOutputStreamState: SetOutputStreamState::<Identity, OFFSET>,
            GetOutputStreamState: GetOutputStreamState::<Identity, OFFSET>,
            GetInputStreamPreferredState: GetInputStreamPreferredState::<Identity, OFFSET>,
            FlushInputStream: FlushInputStream::<Identity, OFFSET>,
            FlushOutputStream: FlushOutputStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFDeviceTransform as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl windows_core::RuntimeName for IMFDeviceTransform {}
windows_core::imp::define_interface!(IMFDeviceTransform2, IMFDeviceTransform2_Vtbl, 0xf5980fed_b521_488f_909f_1a5fcecedb14);
impl core::ops::Deref for IMFDeviceTransform2 {
    type Target = IMFDeviceTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFDeviceTransform2, windows_core::IUnknown, IMFDeviceTransform);
impl IMFDeviceTransform2 {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetTransformAttributes(&self) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDeviceTransform2_Vtbl {
    pub base__: IMFDeviceTransform_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub GetTransformAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetTransformAttributes: usize,
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
pub trait IMFDeviceTransform2_Impl: IMFDeviceTransform_Impl {
    fn GetTransformAttributes(&self) -> windows_core::Result<super::IMFAttributes>;
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl IMFDeviceTransform2_Vtbl {
    pub const fn new<Identity: IMFDeviceTransform2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTransformAttributes<Identity: IMFDeviceTransform2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform2_Impl::GetTransformAttributes(this) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppattributes.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        Self { base__: IMFDeviceTransform_Vtbl::new::<Identity, OFFSET>(), GetTransformAttributes: GetTransformAttributes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFDeviceTransform2 as windows_core::Interface>::IID || iid == &<IMFDeviceTransform as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl windows_core::RuntimeName for IMFDeviceTransform2 {}
windows_core::imp::define_interface!(IMFDeviceTransformCallback, IMFDeviceTransformCallback_Vtbl, 0x6d5cb646_29ec_41fb_8179_8c4c6d750811);
windows_core::imp::interface_hierarchy!(IMFDeviceTransformCallback, windows_core::IUnknown);
impl IMFDeviceTransformCallback {
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn OnBufferSent<P0>(&self, pcallbackattributes: P0, pinid: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnBufferSent)(windows_core::Interface::as_raw(self), pcallbackattributes.param().abi(), pinid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDeviceTransformCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub OnBufferSent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    OnBufferSent: usize,
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
pub trait IMFDeviceTransformCallback_Impl: windows_core::IUnknownImpl {
    fn OnBufferSent(&self, pcallbackattributes: windows_core::Ref<super::IMFAttributes>, pinid: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl IMFDeviceTransformCallback_Vtbl {
    pub const fn new<Identity: IMFDeviceTransformCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnBufferSent<Identity: IMFDeviceTransformCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallbackattributes: *mut core::ffi::c_void, pinid: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransformCallback_Impl::OnBufferSent(this, core::mem::transmute_copy(&pcallbackattributes), core::mem::transmute_copy(&pinid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnBufferSent: OnBufferSent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFDeviceTransformCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl windows_core::RuntimeName for IMFDeviceTransformCallback {}
windows_core::imp::define_interface!(IMFTransform, IMFTransform_Vtbl, 0xbf94c121_5b05_4e6f_8000_ba598961414d);
windows_core::imp::interface_hierarchy!(IMFTransform, windows_core::IUnknown);
impl IMFTransform {
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamLimits(&self, pdwinputminimum: *mut super::DWORD, pdwinputmaximum: *mut super::DWORD, pdwoutputminimum: *mut super::DWORD, pdwoutputmaximum: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamLimits)(windows_core::Interface::as_raw(self), pdwinputminimum as _, pdwinputmaximum as _, pdwoutputminimum as _, pdwoutputmaximum as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut super::DWORD, pcoutputstreams: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), pcinputstreams as _, pcoutputstreams as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamIDs(&self, dwinputidarraysize: super::DWORD, pdwinputids: *mut super::DWORD, dwoutputidarraysize: super::DWORD, pdwoutputids: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamIDs)(windows_core::Interface::as_raw(self), dwinputidarraysize, pdwinputids as _, dwoutputidarraysize, pdwoutputids as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetInputStreamInfo(&self, dwinputstreamid: super::DWORD, pstreaminfo: *mut MFT_INPUT_STREAM_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputStreamInfo)(windows_core::Interface::as_raw(self), dwinputstreamid, pstreaminfo as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputStreamInfo(&self, dwoutputstreamid: super::DWORD, pstreaminfo: *mut MFT_OUTPUT_STREAM_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputStreamInfo)(windows_core::Interface::as_raw(self), dwoutputstreamid, pstreaminfo as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetAttributes(&self) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetInputStreamAttributes(&self, dwinputstreamid: super::DWORD) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamAttributes)(windows_core::Interface::as_raw(self), dwinputstreamid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetOutputStreamAttributes(&self, dwoutputstreamid: super::DWORD) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamAttributes)(windows_core::Interface::as_raw(self), dwoutputstreamid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteInputStream(&self, dwstreamid: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteInputStream)(windows_core::Interface::as_raw(self), dwstreamid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddInputStreams(&self, cstreams: super::DWORD, adwstreamids: *const super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddInputStreams)(windows_core::Interface::as_raw(self), cstreams, adwstreamids) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetInputAvailableType(&self, dwinputstreamid: super::DWORD, dwtypeindex: super::DWORD) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputAvailableType)(windows_core::Interface::as_raw(self), dwinputstreamid, dwtypeindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetOutputAvailableType(&self, dwoutputstreamid: super::DWORD, dwtypeindex: super::DWORD) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputAvailableType)(windows_core::Interface::as_raw(self), dwoutputstreamid, dwtypeindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn SetInputType<P1>(&self, dwinputstreamid: super::DWORD, ptype: P1, dwflags: super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputType)(windows_core::Interface::as_raw(self), dwinputstreamid, ptype.param().abi(), dwflags) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn SetOutputType<P1>(&self, dwoutputstreamid: super::DWORD, ptype: P1, dwflags: super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputType)(windows_core::Interface::as_raw(self), dwoutputstreamid, ptype.param().abi(), dwflags) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamid: super::DWORD) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputCurrentType)(windows_core::Interface::as_raw(self), dwinputstreamid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamid: super::DWORD) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputCurrentType)(windows_core::Interface::as_raw(self), dwoutputstreamid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetInputStatus(&self, dwinputstreamid: super::DWORD, pdwflags: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputStatus)(windows_core::Interface::as_raw(self), dwinputstreamid, pdwflags as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputStatus(&self, pdwflags: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputStatus)(windows_core::Interface::as_raw(self), pdwflags as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetOutputBounds(&self, hnslowerbound: super::LONGLONG, hnsupperbound: super::LONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOutputBounds)(windows_core::Interface::as_raw(self), hnslowerbound, hnsupperbound) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn ProcessEvent<P1>(&self, dwinputstreamid: super::DWORD, pevent: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessEvent)(windows_core::Interface::as_raw(self), dwinputstreamid, pevent.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ProcessMessage(&self, emessage: MFT_MESSAGE_TYPE, ulparam: super::ULONG_PTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessMessage)(windows_core::Interface::as_raw(self), emessage, ulparam) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn ProcessInput<P1>(&self, dwinputstreamid: super::DWORD, psample: P1, dwflags: super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessInput)(windows_core::Interface::as_raw(self), dwinputstreamid, psample.param().abi(), dwflags) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn ProcessOutput(&self, dwflags: super::DWORD, coutputbuffercount: super::DWORD, poutputsamples: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessOutput)(windows_core::Interface::as_raw(self), dwflags, coutputbuffercount, poutputsamples, pdwstatus as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetStreamLimits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::DWORD, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamLimits: usize,
    #[cfg(feature = "shared")]
    pub GetStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamCount: usize,
    #[cfg(feature = "shared")]
    pub GetStreamIDs: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamIDs: usize,
    #[cfg(feature = "shared")]
    pub GetInputStreamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut MFT_INPUT_STREAM_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInputStreamInfo: usize,
    #[cfg(feature = "shared")]
    pub GetOutputStreamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut MFT_OUTPUT_STREAM_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputStreamInfo: usize,
    #[cfg(feature = "mfobjects")]
    pub GetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetAttributes: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetInputStreamAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetInputStreamAttributes: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetOutputStreamAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetOutputStreamAttributes: usize,
    #[cfg(feature = "shared")]
    pub DeleteInputStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteInputStream: usize,
    #[cfg(feature = "shared")]
    pub AddInputStreams: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddInputStreams: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetInputAvailableType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetInputAvailableType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetOutputAvailableType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetOutputAvailableType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub SetInputType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    SetInputType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub SetOutputType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    SetOutputType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetInputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetInputCurrentType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetOutputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetOutputCurrentType: usize,
    #[cfg(feature = "shared")]
    pub GetInputStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInputStatus: usize,
    #[cfg(feature = "shared")]
    pub GetOutputStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputStatus: usize,
    #[cfg(feature = "shared")]
    pub SetOutputBounds: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONGLONG, super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetOutputBounds: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub ProcessEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    ProcessEvent: usize,
    #[cfg(feature = "shared")]
    pub ProcessMessage: unsafe extern "system" fn(*mut core::ffi::c_void, MFT_MESSAGE_TYPE, super::ULONG_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ProcessMessage: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub ProcessInput: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    ProcessInput: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub ProcessOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut MFT_OUTPUT_DATA_BUFFER, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    ProcessOutput: usize,
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
pub trait IMFTransform_Impl: windows_core::IUnknownImpl {
    fn GetStreamLimits(&self, pdwinputminimum: *mut super::DWORD, pdwinputmaximum: *mut super::DWORD, pdwoutputminimum: *mut super::DWORD, pdwoutputmaximum: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetStreamCount(&self, pcinputstreams: *mut super::DWORD, pcoutputstreams: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetStreamIDs(&self, dwinputidarraysize: super::DWORD, pdwinputids: *mut super::DWORD, dwoutputidarraysize: super::DWORD, pdwoutputids: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetInputStreamInfo(&self, dwinputstreamid: super::DWORD, pstreaminfo: *mut MFT_INPUT_STREAM_INFO) -> windows_core::Result<()>;
    fn GetOutputStreamInfo(&self, dwoutputstreamid: super::DWORD, pstreaminfo: *mut MFT_OUTPUT_STREAM_INFO) -> windows_core::Result<()>;
    fn GetAttributes(&self) -> windows_core::Result<super::IMFAttributes>;
    fn GetInputStreamAttributes(&self, dwinputstreamid: super::DWORD) -> windows_core::Result<super::IMFAttributes>;
    fn GetOutputStreamAttributes(&self, dwoutputstreamid: super::DWORD) -> windows_core::Result<super::IMFAttributes>;
    fn DeleteInputStream(&self, dwstreamid: super::DWORD) -> windows_core::Result<()>;
    fn AddInputStreams(&self, cstreams: super::DWORD, adwstreamids: *const super::DWORD) -> windows_core::Result<()>;
    fn GetInputAvailableType(&self, dwinputstreamid: super::DWORD, dwtypeindex: super::DWORD) -> windows_core::Result<super::IMFMediaType>;
    fn GetOutputAvailableType(&self, dwoutputstreamid: super::DWORD, dwtypeindex: super::DWORD) -> windows_core::Result<super::IMFMediaType>;
    fn SetInputType(&self, dwinputstreamid: super::DWORD, ptype: windows_core::Ref<super::IMFMediaType>, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn SetOutputType(&self, dwoutputstreamid: super::DWORD, ptype: windows_core::Ref<super::IMFMediaType>, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn GetInputCurrentType(&self, dwinputstreamid: super::DWORD) -> windows_core::Result<super::IMFMediaType>;
    fn GetOutputCurrentType(&self, dwoutputstreamid: super::DWORD) -> windows_core::Result<super::IMFMediaType>;
    fn GetInputStatus(&self, dwinputstreamid: super::DWORD, pdwflags: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetOutputStatus(&self, pdwflags: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetOutputBounds(&self, hnslowerbound: super::LONGLONG, hnsupperbound: super::LONGLONG) -> windows_core::Result<()>;
    fn ProcessEvent(&self, dwinputstreamid: super::DWORD, pevent: windows_core::Ref<super::IMFMediaEvent>) -> windows_core::Result<()>;
    fn ProcessMessage(&self, emessage: MFT_MESSAGE_TYPE, ulparam: super::ULONG_PTR) -> windows_core::Result<()>;
    fn ProcessInput(&self, dwinputstreamid: super::DWORD, psample: windows_core::Ref<super::IMFSample>, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn ProcessOutput(&self, dwflags: super::DWORD, coutputbuffercount: super::DWORD, poutputsamples: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl IMFTransform_Vtbl {
    pub const fn new<Identity: IMFTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamLimits<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwinputminimum: *mut super::DWORD, pdwinputmaximum: *mut super::DWORD, pdwoutputminimum: *mut super::DWORD, pdwoutputmaximum: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetStreamLimits(this, core::mem::transmute_copy(&pdwinputminimum), core::mem::transmute_copy(&pdwinputmaximum), core::mem::transmute_copy(&pdwoutputminimum), core::mem::transmute_copy(&pdwoutputmaximum)).into()
            }
        }
        unsafe extern "system" fn GetStreamCount<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinputstreams: *mut super::DWORD, pcoutputstreams: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetStreamCount(this, core::mem::transmute_copy(&pcinputstreams), core::mem::transmute_copy(&pcoutputstreams)).into()
            }
        }
        unsafe extern "system" fn GetStreamIDs<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputidarraysize: super::DWORD, pdwinputids: *mut super::DWORD, dwoutputidarraysize: super::DWORD, pdwoutputids: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetStreamIDs(this, core::mem::transmute_copy(&dwinputidarraysize), core::mem::transmute_copy(&pdwinputids), core::mem::transmute_copy(&dwoutputidarraysize), core::mem::transmute_copy(&pdwoutputids)).into()
            }
        }
        unsafe extern "system" fn GetInputStreamInfo<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, pstreaminfo: *mut MFT_INPUT_STREAM_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetInputStreamInfo(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&pstreaminfo)).into()
            }
        }
        unsafe extern "system" fn GetOutputStreamInfo<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: super::DWORD, pstreaminfo: *mut MFT_OUTPUT_STREAM_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetOutputStreamInfo(this, core::mem::transmute_copy(&dwoutputstreamid), core::mem::transmute_copy(&pstreaminfo)).into()
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetAttributes(this) {
                    Ok(ok__) => {
                        pattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputStreamAttributes<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, pattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetInputStreamAttributes(this, core::mem::transmute_copy(&dwinputstreamid)) {
                    Ok(ok__) => {
                        pattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputStreamAttributes<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: super::DWORD, pattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetOutputStreamAttributes(this, core::mem::transmute_copy(&dwoutputstreamid)) {
                    Ok(ok__) => {
                        pattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteInputStream<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::DeleteInputStream(this, core::mem::transmute_copy(&dwstreamid)).into()
            }
        }
        unsafe extern "system" fn AddInputStreams<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cstreams: super::DWORD, adwstreamids: *const super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::AddInputStreams(this, core::mem::transmute_copy(&cstreams), core::mem::transmute_copy(&adwstreamids)).into()
            }
        }
        unsafe extern "system" fn GetInputAvailableType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, dwtypeindex: super::DWORD, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetInputAvailableType(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&dwtypeindex)) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputAvailableType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: super::DWORD, dwtypeindex: super::DWORD, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetOutputAvailableType(this, core::mem::transmute_copy(&dwoutputstreamid), core::mem::transmute_copy(&dwtypeindex)) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInputType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, ptype: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::SetInputType(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SetOutputType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: super::DWORD, ptype: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::SetOutputType(this, core::mem::transmute_copy(&dwoutputstreamid), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetInputCurrentType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetInputCurrentType(this, core::mem::transmute_copy(&dwinputstreamid)) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: super::DWORD, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetOutputCurrentType(this, core::mem::transmute_copy(&dwoutputstreamid)) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputStatus<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, pdwflags: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetInputStatus(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&pdwflags)).into()
            }
        }
        unsafe extern "system" fn GetOutputStatus<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetOutputStatus(this, core::mem::transmute_copy(&pdwflags)).into()
            }
        }
        unsafe extern "system" fn SetOutputBounds<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnslowerbound: super::LONGLONG, hnsupperbound: super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::SetOutputBounds(this, core::mem::transmute_copy(&hnslowerbound), core::mem::transmute_copy(&hnsupperbound)).into()
            }
        }
        unsafe extern "system" fn ProcessEvent<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::ProcessEvent(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&pevent)).into()
            }
        }
        unsafe extern "system" fn ProcessMessage<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emessage: MFT_MESSAGE_TYPE, ulparam: super::ULONG_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::ProcessMessage(this, core::mem::transmute_copy(&emessage), core::mem::transmute_copy(&ulparam)).into()
            }
        }
        unsafe extern "system" fn ProcessInput<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: super::DWORD, psample: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::ProcessInput(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&psample), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn ProcessOutput<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, coutputbuffercount: super::DWORD, poutputsamples: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::ProcessOutput(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&coutputbuffercount), core::mem::transmute_copy(&poutputsamples), core::mem::transmute_copy(&pdwstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamLimits: GetStreamLimits::<Identity, OFFSET>,
            GetStreamCount: GetStreamCount::<Identity, OFFSET>,
            GetStreamIDs: GetStreamIDs::<Identity, OFFSET>,
            GetInputStreamInfo: GetInputStreamInfo::<Identity, OFFSET>,
            GetOutputStreamInfo: GetOutputStreamInfo::<Identity, OFFSET>,
            GetAttributes: GetAttributes::<Identity, OFFSET>,
            GetInputStreamAttributes: GetInputStreamAttributes::<Identity, OFFSET>,
            GetOutputStreamAttributes: GetOutputStreamAttributes::<Identity, OFFSET>,
            DeleteInputStream: DeleteInputStream::<Identity, OFFSET>,
            AddInputStreams: AddInputStreams::<Identity, OFFSET>,
            GetInputAvailableType: GetInputAvailableType::<Identity, OFFSET>,
            GetOutputAvailableType: GetOutputAvailableType::<Identity, OFFSET>,
            SetInputType: SetInputType::<Identity, OFFSET>,
            SetOutputType: SetOutputType::<Identity, OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Identity, OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Identity, OFFSET>,
            GetInputStatus: GetInputStatus::<Identity, OFFSET>,
            GetOutputStatus: GetOutputStatus::<Identity, OFFSET>,
            SetOutputBounds: SetOutputBounds::<Identity, OFFSET>,
            ProcessEvent: ProcessEvent::<Identity, OFFSET>,
            ProcessMessage: ProcessMessage::<Identity, OFFSET>,
            ProcessInput: ProcessInput::<Identity, OFFSET>,
            ProcessOutput: ProcessOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTransform as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl windows_core::RuntimeName for IMFTransform {}
pub const MEDeviceStreamCreated: windows_core::GUID = windows_core::GUID::from_u128(0x0252a1cf_3540_43b4_9164_d72eb405fa40);
pub type MF3DVideoOutputType = i32;
pub const MF3DVideoOutputType_BaseView: MF3DVideoOutputType = 0;
pub const MF3DVideoOutputType_Stereo: MF3DVideoOutputType = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFAudioDecoderDegradationInfo {
    pub eDegradationReason: MFT_AUDIO_DECODER_DEGRADATION_REASON,
    pub eType: MFT_AUDIO_DECODER_DEGRADATION_TYPE,
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CATEGORY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xc57a84c0_1a80_40a3_97b5_9272a403c8ae), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CLSID: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xc57a84c0_1a80_40a3_97b5_9272a403c8ae), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_EXATTRIBUTE_SUPPORTED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x456fe843_3c87_40c0_949d_1409c97dab2c), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MULTICHANNEL_CHANNEL_MASK: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58bdaf8c_3224_4692_86d0_44d65c5bf82b), pid: 1 };
pub const MFT_AUDIO_DECODER_AUDIO_ENDPOINT_FORMFACTOR: windows_core::GUID = windows_core::GUID::from_u128(0x8d574310_909a_433a_ace7_eee74719f901);
pub const MFT_AUDIO_DECODER_AUDIO_ENDPOINT_ID: windows_core::GUID = windows_core::GUID::from_u128(0xc7ccdd6e_5398_4695_8be7_51b3e95111bd);
pub const MFT_AUDIO_DECODER_AUDIO_ENDPOINT_IS_DIGITAL_STEREO_ONLY: windows_core::GUID = windows_core::GUID::from_u128(0x26e5a90d_4ad1_4f8c_b8af_adf14d2178f1);
pub const MFT_AUDIO_DECODER_DEGRADATION_INFO_ATTRIBUTE: windows_core::GUID = windows_core::GUID::from_u128(0x6c3386ad_ec20_430d_b2a5_505c7178d9c4);
pub type MFT_AUDIO_DECODER_DEGRADATION_REASON = i32;
pub const MFT_AUDIO_DECODER_DEGRADATION_REASON_LICENSING_REQUIREMENT: MFT_AUDIO_DECODER_DEGRADATION_REASON = 1;
pub const MFT_AUDIO_DECODER_DEGRADATION_REASON_NONE: MFT_AUDIO_DECODER_DEGRADATION_REASON = 0;
pub type MFT_AUDIO_DECODER_DEGRADATION_TYPE = i32;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX2CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 1;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX6CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 2;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX8CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 3;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_NONE: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 0;
pub const MFT_AUDIO_DECODER_SPATIAL_METADATA_CLIENT: windows_core::GUID = windows_core::GUID::from_u128(0x05987df4_1270_4999_925f_8e939a7c0af7);
pub const MFT_CODEC_MERIT_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x88a7cb15_7b07_4a34_9128_e64c6703c4d3);
pub const MFT_CONNECTED_STREAM_ATTRIBUTE: windows_core::GUID = windows_core::GUID::from_u128(0x71eeb820_a59f_4de2_bcec_38db1dd611a4);
pub const MFT_CONNECTED_TO_HW_STREAM: windows_core::GUID = windows_core::GUID::from_u128(0x34e6e728_06d6_4491_a553_4795650db912);
pub const MFT_DECODER_AUTOMATIC_SOFTWARE_FALLBACK: windows_core::GUID = windows_core::GUID::from_u128(0x41f34f53_1bf6_49ed_b95d_02d2a1d7115a);
pub const MFT_DECODER_EXPOSE_OUTPUT_TYPES_IN_NATIVE_ORDER: windows_core::GUID = windows_core::GUID::from_u128(0xef80833f_f8fa_44d9_80d8_41ed6232670c);
pub const MFT_DECODER_FINAL_VIDEO_RESOLUTION_HINT: windows_core::GUID = windows_core::GUID::from_u128(0xdc2f8496_15c4_407a_b6f0_1b66ab5fbf53);
pub const MFT_DECODER_OPERATING_POINT: windows_core::GUID = windows_core::GUID::from_u128(0xa1230334_55d4_4d97_82a7_26d3e6456725);
pub const MFT_DECODER_QUALITY_MANAGEMENT_CUSTOM_CONTROL: windows_core::GUID = windows_core::GUID::from_u128(0xa24e30d7_de25_4558_bbfb_71070a2d332e);
pub const MFT_DECODER_QUALITY_MANAGEMENT_RECOVERY_WITHOUT_ARTIFACTS: windows_core::GUID = windows_core::GUID::from_u128(0xd8980deb_0a48_425f_8623_611db41d3810);
pub const MFT_DRAIN_NO_TAILS: _MFT_DRAIN_TYPE = 1;
pub const MFT_DRAIN_PRODUCE_TAILS: _MFT_DRAIN_TYPE = 0;
pub const MFT_ENCODER_SUPPORTS_CONFIG_EVENT: windows_core::GUID = windows_core::GUID::from_u128(0x86a355ae_3a77_4ec4_9f31_01149a4e92de);
pub const MFT_END_STREAMING_AWARE: windows_core::GUID = windows_core::GUID::from_u128(0x70fbc845_b07e_4089_b064_399dc6110f29);
pub const MFT_ENUM_HARDWARE_URL_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x2fb866ac_b078_4942_ab6c_003d05cda674);
pub const MFT_ENUM_HARDWARE_VENDOR_ID_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x3aecb0cc_035b_4bcc_8185_2b8d551ef3af);
pub const MFT_ENUM_TRANSCODE_ONLY_ATTRIBUTE: windows_core::GUID = windows_core::GUID::from_u128(0x111ea8cd_b62a_4bdb_89f6_67ffcdc2458b);
pub const MFT_FIELDOFUSE_UNLOCK_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x8ec2e9fd_9148_410d_831e_702439461a8e);
pub const MFT_FRIENDLY_NAME_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x314ffbae_5b41_4c95_9c19_4e7d586face3);
pub const MFT_HW_TIMESTAMP_WITH_QPC_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x8d030fb8_cc43_4258_a22e_9210bef89be4);
pub const MFT_INPUT_DATA_BUFFER_PLACEHOLDER: _MFT_INPUT_DATA_BUFFER_FLAGS = -1;
pub const MFT_INPUT_STATUS_ACCEPT_DATA: _MFT_INPUT_STATUS_FLAGS = 1;
pub const MFT_INPUT_STREAM_DOES_NOT_ADDREF: _MFT_INPUT_STREAM_INFO_FLAGS = 256;
pub const MFT_INPUT_STREAM_FIXED_SAMPLE_SIZE: _MFT_INPUT_STREAM_INFO_FLAGS = 4;
pub const MFT_INPUT_STREAM_HOLDS_BUFFERS: _MFT_INPUT_STREAM_INFO_FLAGS = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFT_INPUT_STREAM_INFO {
    pub hnsMaxLatency: super::LONGLONG,
    pub dwFlags: super::DWORD,
    pub cbSize: super::DWORD,
    pub cbMaxLookahead: super::DWORD,
    pub cbAlignment: super::DWORD,
}
pub const MFT_INPUT_STREAM_OPTIONAL: _MFT_INPUT_STREAM_INFO_FLAGS = 1024;
pub const MFT_INPUT_STREAM_PROCESSES_IN_PLACE: _MFT_INPUT_STREAM_INFO_FLAGS = 2048;
pub const MFT_INPUT_STREAM_REMOVABLE: _MFT_INPUT_STREAM_INFO_FLAGS = 512;
pub const MFT_INPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER: _MFT_INPUT_STREAM_INFO_FLAGS = 2;
pub const MFT_INPUT_STREAM_WHOLE_SAMPLES: _MFT_INPUT_STREAM_INFO_FLAGS = 1;
pub const MFT_INPUT_TYPES_Attributes: windows_core::GUID = windows_core::GUID::from_u128(0x4276c9b1_759d_4bf3_9cd0_0d723d138f96);
pub const MFT_MESSAGE_COMMAND_DRAIN: MFT_MESSAGE_TYPE = 1;
pub const MFT_MESSAGE_COMMAND_FLUSH: MFT_MESSAGE_TYPE = 0;
pub const MFT_MESSAGE_COMMAND_FLUSH_OUTPUT_STREAM: MFT_MESSAGE_TYPE = 268435464;
pub const MFT_MESSAGE_COMMAND_MARKER: MFT_MESSAGE_TYPE = 536870912;
pub const MFT_MESSAGE_COMMAND_SET_OUTPUT_STREAM_STATE: MFT_MESSAGE_TYPE = 268435463;
pub const MFT_MESSAGE_COMMAND_TICK: MFT_MESSAGE_TYPE = 4;
pub const MFT_MESSAGE_DROP_SAMPLES: MFT_MESSAGE_TYPE = 3;
pub const MFT_MESSAGE_NOTIFY_BEGIN_STREAMING: MFT_MESSAGE_TYPE = 268435456;
pub const MFT_MESSAGE_NOTIFY_END_OF_STREAM: MFT_MESSAGE_TYPE = 268435458;
pub const MFT_MESSAGE_NOTIFY_END_STREAMING: MFT_MESSAGE_TYPE = 268435457;
pub const MFT_MESSAGE_NOTIFY_EVENT: MFT_MESSAGE_TYPE = 268435462;
pub const MFT_MESSAGE_NOTIFY_REACQUIRE_RESOURCES: MFT_MESSAGE_TYPE = 268435461;
pub const MFT_MESSAGE_NOTIFY_RELEASE_RESOURCES: MFT_MESSAGE_TYPE = 268435460;
pub const MFT_MESSAGE_NOTIFY_START_OF_STREAM: MFT_MESSAGE_TYPE = 268435459;
pub const MFT_MESSAGE_SET_D3D_MANAGER: MFT_MESSAGE_TYPE = 2;
pub type MFT_MESSAGE_TYPE = i32;
#[cfg(feature = "shared")]
pub const MFT_OUTPUT_BOUND_LOWER_UNBOUNDED: super::LONGLONG = -9223372036854775808;
pub const MFT_OUTPUT_BOUND_UPPER_UNBOUNDED: i64 = 9223372036854775807;
#[repr(C)]
#[cfg(all(feature = "mfobjects", feature = "shared"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFT_OUTPUT_DATA_BUFFER {
    pub dwStreamID: super::DWORD,
    pub pSample: core::mem::ManuallyDrop<Option<super::IMFSample>>,
    pub dwStatus: super::DWORD,
    pub pEvents: core::mem::ManuallyDrop<Option<super::IMFCollection>>,
}
pub const MFT_OUTPUT_DATA_BUFFER_FORMAT_CHANGE: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 256;
pub const MFT_OUTPUT_DATA_BUFFER_INCOMPLETE: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 16777216;
pub const MFT_OUTPUT_DATA_BUFFER_NO_SAMPLE: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 768;
pub const MFT_OUTPUT_DATA_BUFFER_STREAM_END: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 512;
pub const MFT_OUTPUT_STATUS_SAMPLE_READY: _MFT_OUTPUT_STATUS_FLAGS = 1;
pub const MFT_OUTPUT_STREAM_CAN_PROVIDE_SAMPLES: _MFT_OUTPUT_STREAM_INFO_FLAGS = 512;
pub const MFT_OUTPUT_STREAM_DISCARDABLE: _MFT_OUTPUT_STREAM_INFO_FLAGS = 8;
pub const MFT_OUTPUT_STREAM_FIXED_SAMPLE_SIZE: _MFT_OUTPUT_STREAM_INFO_FLAGS = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFT_OUTPUT_STREAM_INFO {
    pub dwFlags: super::DWORD,
    pub cbSize: super::DWORD,
    pub cbAlignment: super::DWORD,
}
pub const MFT_OUTPUT_STREAM_LAZY_READ: _MFT_OUTPUT_STREAM_INFO_FLAGS = 1024;
pub const MFT_OUTPUT_STREAM_OPTIONAL: _MFT_OUTPUT_STREAM_INFO_FLAGS = 16;
pub const MFT_OUTPUT_STREAM_PROVIDES_SAMPLES: _MFT_OUTPUT_STREAM_INFO_FLAGS = 256;
pub const MFT_OUTPUT_STREAM_REMOVABLE: _MFT_OUTPUT_STREAM_INFO_FLAGS = 2048;
pub const MFT_OUTPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER: _MFT_OUTPUT_STREAM_INFO_FLAGS = 2;
pub const MFT_OUTPUT_STREAM_WHOLE_SAMPLES: _MFT_OUTPUT_STREAM_INFO_FLAGS = 1;
pub const MFT_OUTPUT_TYPES_Attributes: windows_core::GUID = windows_core::GUID::from_u128(0x8eae8cf3_a44f_4306_ba5c_bf5dda242818);
pub const MFT_POLICY_SET_AWARE: windows_core::GUID = windows_core::GUID::from_u128(0x5a633b19_cc39_4fa8_8ca5_59981b7a0018);
pub const MFT_PREFERRED_ENCODER_PROFILE: windows_core::GUID = windows_core::GUID::from_u128(0x53004909_1ef5_46d7_a18e_5a75f8b5905f);
pub const MFT_PREFERRED_OUTPUTTYPE_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x7e700499_396a_49ee_b1b4_f628021e8c9d);
pub const MFT_PROCESS_LOCAL_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x543186e4_4649_4e65_b588_4aa352aff379);
pub const MFT_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _MFT_PROCESS_OUTPUT_FLAGS = 1;
pub const MFT_PROCESS_OUTPUT_REGENERATE_LAST_OUTPUT: _MFT_PROCESS_OUTPUT_FLAGS = 2;
pub const MFT_PROCESS_OUTPUT_STATUS_NEW_STREAMS: _MFT_PROCESS_OUTPUT_STATUS = 256;
pub const MFT_REMUX_MARK_I_PICTURE_AS_CLEAN_POINT: windows_core::GUID = windows_core::GUID::from_u128(0x364e8f85_3f2e_436c_b2a2_4440a012a9e8);
pub const MFT_SET_TYPE_TEST_ONLY: _MFT_SET_TYPE_FLAGS = 1;
pub const MFT_STREAMS_UNLIMITED: u32 = 4294967295;
#[repr(C)]
#[cfg(all(feature = "mfobjects", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFT_STREAM_STATE_PARAM {
    pub StreamId: super::DWORD,
    pub State: super::MF_STREAM_STATE,
}
pub const MFT_SUPPORT_3DVIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x093f81b1_4f2e_4631_8168_7934032a01d3);
pub const MFT_TRANSFORM_CLSID_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x6821c42b_65a4_4e82_99bc_9a88205ecd0c);
pub const MFT_USING_HARDWARE_DRM: windows_core::GUID = windows_core::GUID::from_u128(0x34faa77d_d79e_4957_b8ce_362b2684996c);
pub const MF_DMFT_FRAME_BUFFER_INFO: windows_core::GUID = windows_core::GUID::from_u128(0x396ce1c9_67a9_454c_8797_95a45799d804);
pub const MF_ENABLE_3DVIDEO_OUTPUT: windows_core::GUID = windows_core::GUID::from_u128(0xbdad7bca_0e5f_4b10_ab16_26de381b6293);
pub const MF_SA_AUDIO_ENDPOINT_AWARE: windows_core::GUID = windows_core::GUID::from_u128(0xc0381701_805c_42b2_ac8d_e2b4bf21f4f8);
pub const MF_SA_BUFFERS_PER_SAMPLE: windows_core::GUID = windows_core::GUID::from_u128(0x873c5171_1e3d_4e25_988d_b433ce041983);
pub const MF_SA_D3D11_ALLOCATE_DISPLAYABLE_RESOURCES: windows_core::GUID = windows_core::GUID::from_u128(0xeeface6d_2ea9_4adf_bbdf_7bbc482a1b6d);
pub const MF_SA_D3D11_ALLOW_DYNAMIC_YUV_TEXTURE: windows_core::GUID = windows_core::GUID::from_u128(0xce06d49f_0613_4b9d_86a6_d8c4f9c10075);
pub const MF_SA_D3D11_AWARE: windows_core::GUID = windows_core::GUID::from_u128(0x206b4fc8_fcf9_4c51_afe3_9764369e33a0);
pub const MF_SA_D3D11_BINDFLAGS: windows_core::GUID = windows_core::GUID::from_u128(0xeacf97ad_065c_4408_bee3_fdcbfd128be2);
pub const MF_SA_D3D11_HW_PROTECTED: windows_core::GUID = windows_core::GUID::from_u128(0x3a8ba9d9_92ca_4307_a391_6999dbf3b6ce);
pub const MF_SA_D3D11_SHARED: windows_core::GUID = windows_core::GUID::from_u128(0x7b8f32c3_6d96_4b89_9203_dd38b61414f3);
pub const MF_SA_D3D11_SHARED_WITHOUT_MUTEX: windows_core::GUID = windows_core::GUID::from_u128(0x39dbd44d_2e44_4931_a4c8_352d3dc42115);
pub const MF_SA_D3D11_USAGE: windows_core::GUID = windows_core::GUID::from_u128(0xe85fe442_2ca3_486e_a9c7_109dda609880);
pub const MF_SA_D3D_AWARE: windows_core::GUID = windows_core::GUID::from_u128(0xeaa35c29_775e_488e_9b61_b3283e49583b);
pub const MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x851745d5_c3d6_476d_9527_498ef2d10d18);
pub const MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT_PROGRESSIVE: windows_core::GUID = windows_core::GUID::from_u128(0x0f5523a5_1cb2_47c5_a550_2eeb84b4d14a);
pub const MF_SA_REQUIRED_SAMPLE_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x18802c61_324b_4952_abd0_176ff5c696ff);
pub const MF_SA_REQUIRED_SAMPLE_COUNT_PROGRESSIVE: windows_core::GUID = windows_core::GUID::from_u128(0xb172d58e_fa77_4e48_8d2a_1df2d850eac2);
pub const MF_TRANSFORM_ASYNC: windows_core::GUID = windows_core::GUID::from_u128(0xf81a699a_649a_497d_8c73_29f8fed6ad7a);
pub const MF_TRANSFORM_ASYNC_UNLOCK: windows_core::GUID = windows_core::GUID::from_u128(0xe5666d6b_3422_4eb6_a421_da7db1f8e207);
pub const MF_TRANSFORM_CATEGORY_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0xceabba49_506d_4757_a6ff_66c184987e4e);
pub const MF_TRANSFORM_FLAGS_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x9359bb7e_6275_46c4_a025_1c01e45f1a86);
pub type PDeviceStreamState = *mut DeviceStreamState;
#[cfg(all(feature = "mfobjects", feature = "shared"))]
pub type PMFT_OUTPUT_DATA_BUFFER = *mut MFT_OUTPUT_DATA_BUFFER;
#[cfg(all(feature = "mfobjects", feature = "shared"))]
pub type PMFT_STREAM_STATE_PARAM = *mut MFT_STREAM_STATE_PARAM;
#[cfg(feature = "shared")]
pub type PSTREAM_MEDIUM = *mut STREAM_MEDIUM;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STREAM_MEDIUM {
    pub gidMedium: windows_core::GUID,
    pub unMediumInstance: super::UINT32,
}
pub type _MFT_DRAIN_TYPE = i32;
pub type _MFT_INPUT_DATA_BUFFER_FLAGS = i32;
pub type _MFT_INPUT_STATUS_FLAGS = i32;
pub type _MFT_INPUT_STREAM_INFO_FLAGS = i32;
pub type _MFT_OUTPUT_DATA_BUFFER_FLAGS = i32;
pub type _MFT_OUTPUT_STATUS_FLAGS = i32;
pub type _MFT_OUTPUT_STREAM_INFO_FLAGS = i32;
pub type _MFT_PROCESS_OUTPUT_FLAGS = i32;
pub type _MFT_PROCESS_OUTPUT_STATUS = i32;
pub type _MFT_SET_TYPE_FLAGS = i32;
