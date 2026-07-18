#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateTransformActivate() -> windows_core::Result<super::IMFActivate> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateTransformActivate(ppactivate : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateTransformActivate(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
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
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetInputAvailableType(&self, dwinputstreamid: u32, dwtypeindex: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputAvailableType)(windows_core::Interface::as_raw(self), dwinputstreamid, dwtypeindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamid: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputCurrentType)(windows_core::Interface::as_raw(self), dwinputstreamid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetInputStreamAttributes(&self, dwinputstreamid: u32) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamAttributes)(windows_core::Interface::as_raw(self), dwinputstreamid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetOutputAvailableType(&self, dwoutputstreamid: u32, dwtypeindex: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputAvailableType)(windows_core::Interface::as_raw(self), dwoutputstreamid, dwtypeindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamid: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputCurrentType)(windows_core::Interface::as_raw(self), dwoutputstreamid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetOutputStreamAttributes(&self, dwoutputstreamid: u32) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamAttributes)(windows_core::Interface::as_raw(self), dwoutputstreamid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), pcinputstreams as _, pcoutputstreams as _) }
    }
    pub unsafe fn GetStreamIDs(&self, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamIDs)(windows_core::Interface::as_raw(self), dwinputidarraysize, pdwinputstreamids as _, dwoutputidarraysize, pdwoutputstreamids as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn ProcessEvent<P1>(&self, dwinputstreamid: u32, pevent: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessEvent)(windows_core::Interface::as_raw(self), dwinputstreamid, pevent.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn ProcessInput<P1>(&self, dwinputstreamid: u32, psample: P1, dwflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessInput)(windows_core::Interface::as_raw(self), dwinputstreamid, psample.param().abi(), dwflags) }
    }
    pub unsafe fn ProcessMessage(&self, emessage: MFT_MESSAGE_TYPE, ulparam: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessMessage)(windows_core::Interface::as_raw(self), emessage, ulparam) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessOutput)(windows_core::Interface::as_raw(self), dwflags, coutputbuffercount, poutputsample, pdwstatus as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetInputStreamState<P1>(&self, dwstreamid: u32, pmediatype: P1, value: DeviceStreamState, dwflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputStreamState)(windows_core::Interface::as_raw(self), dwstreamid, pmediatype.param().abi(), value, dwflags) }
    }
    pub unsafe fn GetInputStreamState(&self, dwstreamid: u32) -> windows_core::Result<DeviceStreamState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamState)(windows_core::Interface::as_raw(self), dwstreamid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetOutputStreamState<P1>(&self, dwstreamid: u32, pmediatype: P1, value: DeviceStreamState, dwflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputStreamState)(windows_core::Interface::as_raw(self), dwstreamid, pmediatype.param().abi(), value, dwflags) }
    }
    pub unsafe fn GetOutputStreamState(&self, dwstreamid: u32) -> windows_core::Result<DeviceStreamState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamState)(windows_core::Interface::as_raw(self), dwstreamid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetInputStreamPreferredState(&self, dwstreamid: u32, value: *mut DeviceStreamState, ppmediatype: *mut Option<super::IMFMediaType>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputStreamPreferredState)(windows_core::Interface::as_raw(self), dwstreamid, value as _, core::mem::transmute(ppmediatype)) }
    }
    pub unsafe fn FlushInputStream(&self, dwstreamindex: u32, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FlushInputStream)(windows_core::Interface::as_raw(self), dwstreamindex, dwflags) }
    }
    pub unsafe fn FlushOutputStream(&self, dwstreamindex: u32, dwflags: u32) -> windows_core::HRESULT {
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
    #[cfg(feature = "mfobjects")]
    pub GetInputAvailableType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetInputAvailableType: usize,
    #[cfg(feature = "mfobjects")]
    pub GetInputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetInputCurrentType: usize,
    #[cfg(feature = "mfobjects")]
    pub GetInputStreamAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetInputStreamAttributes: usize,
    #[cfg(feature = "mfobjects")]
    pub GetOutputAvailableType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetOutputAvailableType: usize,
    #[cfg(feature = "mfobjects")]
    pub GetOutputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetOutputCurrentType: usize,
    #[cfg(feature = "mfobjects")]
    pub GetOutputStreamAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetOutputStreamAttributes: usize,
    pub GetStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetStreamIDs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub ProcessEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    ProcessEvent: usize,
    #[cfg(feature = "mfobjects")]
    pub ProcessInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    ProcessInput: usize,
    pub ProcessMessage: unsafe extern "system" fn(*mut core::ffi::c_void, MFT_MESSAGE_TYPE, usize) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub ProcessOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut MFT_OUTPUT_DATA_BUFFER, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    ProcessOutput: usize,
    #[cfg(feature = "mfobjects")]
    pub SetInputStreamState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, DeviceStreamState, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetInputStreamState: usize,
    pub GetInputStreamState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DeviceStreamState) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub SetOutputStreamState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, DeviceStreamState, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetOutputStreamState: usize,
    pub GetOutputStreamState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DeviceStreamState) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetInputStreamPreferredState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DeviceStreamState, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetInputStreamPreferredState: usize,
    pub FlushInputStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub FlushOutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFDeviceTransform_Impl: windows_core::IUnknownImpl {
    fn InitializeTransform(&self, pattributes: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<()>;
    fn GetInputAvailableType(&self, dwinputstreamid: u32, dwtypeindex: u32) -> windows_core::Result<super::IMFMediaType>;
    fn GetInputCurrentType(&self, dwinputstreamid: u32) -> windows_core::Result<super::IMFMediaType>;
    fn GetInputStreamAttributes(&self, dwinputstreamid: u32) -> windows_core::Result<super::IMFAttributes>;
    fn GetOutputAvailableType(&self, dwoutputstreamid: u32, dwtypeindex: u32) -> windows_core::Result<super::IMFMediaType>;
    fn GetOutputCurrentType(&self, dwoutputstreamid: u32) -> windows_core::Result<super::IMFMediaType>;
    fn GetOutputStreamAttributes(&self, dwoutputstreamid: u32) -> windows_core::Result<super::IMFAttributes>;
    fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::Result<()>;
    fn GetStreamIDs(&self, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> windows_core::Result<()>;
    fn ProcessEvent(&self, dwinputstreamid: u32, pevent: windows_core::Ref<super::IMFMediaEvent>) -> windows_core::Result<()>;
    fn ProcessInput(&self, dwinputstreamid: u32, psample: windows_core::Ref<super::IMFSample>, dwflags: u32) -> windows_core::Result<()>;
    fn ProcessMessage(&self, emessage: MFT_MESSAGE_TYPE, ulparam: usize) -> windows_core::Result<()>;
    fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::Result<()>;
    fn SetInputStreamState(&self, dwstreamid: u32, pmediatype: windows_core::Ref<super::IMFMediaType>, value: DeviceStreamState, dwflags: u32) -> windows_core::Result<()>;
    fn GetInputStreamState(&self, dwstreamid: u32) -> windows_core::Result<DeviceStreamState>;
    fn SetOutputStreamState(&self, dwstreamid: u32, pmediatype: windows_core::Ref<super::IMFMediaType>, value: DeviceStreamState, dwflags: u32) -> windows_core::Result<()>;
    fn GetOutputStreamState(&self, dwstreamid: u32) -> windows_core::Result<DeviceStreamState>;
    fn GetInputStreamPreferredState(&self, dwstreamid: u32, value: *mut DeviceStreamState, ppmediatype: windows_core::OutRef<super::IMFMediaType>) -> windows_core::Result<()>;
    fn FlushInputStream(&self, dwstreamindex: u32, dwflags: u32) -> windows_core::Result<()>;
    fn FlushOutputStream(&self, dwstreamindex: u32, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFDeviceTransform_Vtbl {
    pub const fn new<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeTransform<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::InitializeTransform(this, core::mem::transmute_copy(&pattributes)).into()
            }
        }
        unsafe extern "system" fn GetInputAvailableType<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, dwtypeindex: u32, pmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetInputAvailableType(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&dwtypeindex)) {
                    Ok(ok__) => {
                        pmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputCurrentType<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, pmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetInputCurrentType(this, core::mem::transmute_copy(&dwinputstreamid)) {
                    Ok(ok__) => {
                        pmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputStreamAttributes<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetInputStreamAttributes(this, core::mem::transmute_copy(&dwinputstreamid)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputAvailableType<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: u32, dwtypeindex: u32, pmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetOutputAvailableType(this, core::mem::transmute_copy(&dwoutputstreamid), core::mem::transmute_copy(&dwtypeindex)) {
                    Ok(ok__) => {
                        pmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: u32, pmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetOutputCurrentType(this, core::mem::transmute_copy(&dwoutputstreamid)) {
                    Ok(ok__) => {
                        pmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputStreamAttributes<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: u32, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetOutputStreamAttributes(this, core::mem::transmute_copy(&dwoutputstreamid)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamCount<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::GetStreamCount(this, core::mem::transmute_copy(&pcinputstreams), core::mem::transmute_copy(&pcoutputstreams)).into()
            }
        }
        unsafe extern "system" fn GetStreamIDs<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::GetStreamIDs(this, core::mem::transmute_copy(&dwinputidarraysize), core::mem::transmute_copy(&pdwinputstreamids), core::mem::transmute_copy(&dwoutputidarraysize), core::mem::transmute_copy(&pdwoutputstreamids)).into()
            }
        }
        unsafe extern "system" fn ProcessEvent<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::ProcessEvent(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&pevent)).into()
            }
        }
        unsafe extern "system" fn ProcessInput<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, psample: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::ProcessInput(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&psample), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn ProcessMessage<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emessage: MFT_MESSAGE_TYPE, ulparam: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::ProcessMessage(this, core::mem::transmute_copy(&emessage), core::mem::transmute_copy(&ulparam)).into()
            }
        }
        unsafe extern "system" fn ProcessOutput<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::ProcessOutput(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&coutputbuffercount), core::mem::transmute_copy(&poutputsample), core::mem::transmute_copy(&pdwstatus)).into()
            }
        }
        unsafe extern "system" fn SetInputStreamState<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32, pmediatype: *mut core::ffi::c_void, value: DeviceStreamState, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::SetInputStreamState(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&pmediatype), core::mem::transmute_copy(&value), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetInputStreamState<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32, value: *mut DeviceStreamState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetInputStreamState(this, core::mem::transmute_copy(&dwstreamid)) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutputStreamState<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32, pmediatype: *mut core::ffi::c_void, value: DeviceStreamState, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::SetOutputStreamState(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&pmediatype), core::mem::transmute_copy(&value), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetOutputStreamState<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32, value: *mut DeviceStreamState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDeviceTransform_Impl::GetOutputStreamState(this, core::mem::transmute_copy(&dwstreamid)) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputStreamPreferredState<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32, value: *mut DeviceStreamState, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::GetInputStreamPreferredState(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&value), core::mem::transmute_copy(&ppmediatype)).into()
            }
        }
        unsafe extern "system" fn FlushInputStream<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDeviceTransform_Impl::FlushInputStream(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn FlushOutputStream<Identity: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> windows_core::HRESULT {
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
#[cfg(feature = "mfobjects")]
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
            (windows_core::Interface::vtable(self).GetTransformAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
#[cfg(feature = "mfobjects")]
pub trait IMFDeviceTransform2_Impl: IMFDeviceTransform_Impl {
    fn GetTransformAttributes(&self) -> windows_core::Result<super::IMFAttributes>;
}
#[cfg(feature = "mfobjects")]
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
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IMFDeviceTransform_Vtbl::new::<Identity, OFFSET>(), GetTransformAttributes: GetTransformAttributes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFDeviceTransform2 as windows_core::Interface>::IID || iid == &<IMFDeviceTransform as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFDeviceTransform2 {}
windows_core::imp::define_interface!(IMFDeviceTransformCallback, IMFDeviceTransformCallback_Vtbl, 0x6d5cb646_29ec_41fb_8179_8c4c6d750811);
windows_core::imp::interface_hierarchy!(IMFDeviceTransformCallback, windows_core::IUnknown);
impl IMFDeviceTransformCallback {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn OnBufferSent<P0>(&self, pcallbackattributes: P0, pinid: u32) -> windows_core::HRESULT
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
    #[cfg(feature = "mfobjects")]
    pub OnBufferSent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    OnBufferSent: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFDeviceTransformCallback_Impl: windows_core::IUnknownImpl {
    fn OnBufferSent(&self, pcallbackattributes: windows_core::Ref<super::IMFAttributes>, pinid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFDeviceTransformCallback_Vtbl {
    pub const fn new<Identity: IMFDeviceTransformCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnBufferSent<Identity: IMFDeviceTransformCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallbackattributes: *mut core::ffi::c_void, pinid: u32) -> windows_core::HRESULT {
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
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFDeviceTransformCallback {}
windows_core::imp::define_interface!(IMFTransform, IMFTransform_Vtbl, 0xbf94c121_5b05_4e6f_8000_ba598961414d);
windows_core::imp::interface_hierarchy!(IMFTransform, windows_core::IUnknown);
impl IMFTransform {
    pub unsafe fn GetStreamLimits(&self, pdwinputminimum: *mut u32, pdwinputmaximum: *mut u32, pdwoutputminimum: *mut u32, pdwoutputmaximum: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamLimits)(windows_core::Interface::as_raw(self), pdwinputminimum as _, pdwinputmaximum as _, pdwoutputminimum as _, pdwoutputmaximum as _) }
    }
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), pcinputstreams as _, pcoutputstreams as _) }
    }
    pub unsafe fn GetStreamIDs(&self, dwinputidarraysize: u32, pdwinputids: *mut u32, dwoutputidarraysize: u32, pdwoutputids: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamIDs)(windows_core::Interface::as_raw(self), dwinputidarraysize, pdwinputids as _, dwoutputidarraysize, pdwoutputids as _) }
    }
    pub unsafe fn GetInputStreamInfo(&self, dwinputstreamid: u32, pstreaminfo: *mut MFT_INPUT_STREAM_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputStreamInfo)(windows_core::Interface::as_raw(self), dwinputstreamid, pstreaminfo as _) }
    }
    pub unsafe fn GetOutputStreamInfo(&self, dwoutputstreamid: u32) -> windows_core::Result<MFT_OUTPUT_STREAM_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamInfo)(windows_core::Interface::as_raw(self), dwoutputstreamid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetAttributes(&self) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetInputStreamAttributes(&self, dwinputstreamid: u32) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamAttributes)(windows_core::Interface::as_raw(self), dwinputstreamid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetOutputStreamAttributes(&self, dwoutputstreamid: u32) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamAttributes)(windows_core::Interface::as_raw(self), dwoutputstreamid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteInputStream(&self, dwstreamid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteInputStream)(windows_core::Interface::as_raw(self), dwstreamid) }
    }
    pub unsafe fn AddInputStreams(&self, cstreams: u32, adwstreamids: *const u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddInputStreams)(windows_core::Interface::as_raw(self), cstreams, adwstreamids) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetInputAvailableType(&self, dwinputstreamid: u32, dwtypeindex: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputAvailableType)(windows_core::Interface::as_raw(self), dwinputstreamid, dwtypeindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetOutputAvailableType(&self, dwoutputstreamid: u32, dwtypeindex: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputAvailableType)(windows_core::Interface::as_raw(self), dwoutputstreamid, dwtypeindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetInputType<P1>(&self, dwinputstreamid: u32, ptype: P1, dwflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputType)(windows_core::Interface::as_raw(self), dwinputstreamid, ptype.param().abi(), dwflags) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetOutputType<P1>(&self, dwoutputstreamid: u32, ptype: P1, dwflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputType)(windows_core::Interface::as_raw(self), dwoutputstreamid, ptype.param().abi(), dwflags) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamid: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputCurrentType)(windows_core::Interface::as_raw(self), dwinputstreamid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamid: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputCurrentType)(windows_core::Interface::as_raw(self), dwoutputstreamid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetInputStatus(&self, dwinputstreamid: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStatus)(windows_core::Interface::as_raw(self), dwinputstreamid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOutputStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOutputBounds(&self, hnslowerbound: i64, hnsupperbound: i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOutputBounds)(windows_core::Interface::as_raw(self), hnslowerbound, hnsupperbound) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn ProcessEvent<P1>(&self, dwinputstreamid: u32, pevent: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessEvent)(windows_core::Interface::as_raw(self), dwinputstreamid, pevent.param().abi()) }
    }
    pub unsafe fn ProcessMessage(&self, emessage: MFT_MESSAGE_TYPE, ulparam: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessMessage)(windows_core::Interface::as_raw(self), emessage, ulparam) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn ProcessInput<P1>(&self, dwinputstreamid: u32, psample: P1, dwflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessInput)(windows_core::Interface::as_raw(self), dwinputstreamid, psample.param().abi(), dwflags) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputsamples: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessOutput)(windows_core::Interface::as_raw(self), dwflags, coutputbuffercount, poutputsamples, pdwstatus as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStreamLimits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetStreamIDs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetInputStreamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MFT_INPUT_STREAM_INFO) -> windows_core::HRESULT,
    pub GetOutputStreamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MFT_OUTPUT_STREAM_INFO) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetAttributes: usize,
    #[cfg(feature = "mfobjects")]
    pub GetInputStreamAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetInputStreamAttributes: usize,
    #[cfg(feature = "mfobjects")]
    pub GetOutputStreamAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetOutputStreamAttributes: usize,
    pub DeleteInputStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub AddInputStreams: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetInputAvailableType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetInputAvailableType: usize,
    #[cfg(feature = "mfobjects")]
    pub GetOutputAvailableType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetOutputAvailableType: usize,
    #[cfg(feature = "mfobjects")]
    pub SetInputType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetInputType: usize,
    #[cfg(feature = "mfobjects")]
    pub SetOutputType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetOutputType: usize,
    #[cfg(feature = "mfobjects")]
    pub GetInputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetInputCurrentType: usize,
    #[cfg(feature = "mfobjects")]
    pub GetOutputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetOutputCurrentType: usize,
    pub GetInputStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetOutputStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetOutputBounds: unsafe extern "system" fn(*mut core::ffi::c_void, i64, i64) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub ProcessEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    ProcessEvent: usize,
    pub ProcessMessage: unsafe extern "system" fn(*mut core::ffi::c_void, MFT_MESSAGE_TYPE, usize) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub ProcessInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    ProcessInput: usize,
    #[cfg(feature = "mfobjects")]
    pub ProcessOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut MFT_OUTPUT_DATA_BUFFER, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    ProcessOutput: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFTransform_Impl: windows_core::IUnknownImpl {
    fn GetStreamLimits(&self, pdwinputminimum: *mut u32, pdwinputmaximum: *mut u32, pdwoutputminimum: *mut u32, pdwoutputmaximum: *mut u32) -> windows_core::Result<()>;
    fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::Result<()>;
    fn GetStreamIDs(&self, dwinputidarraysize: u32, pdwinputids: *mut u32, dwoutputidarraysize: u32, pdwoutputids: *mut u32) -> windows_core::Result<()>;
    fn GetInputStreamInfo(&self, dwinputstreamid: u32, pstreaminfo: *mut MFT_INPUT_STREAM_INFO) -> windows_core::Result<()>;
    fn GetOutputStreamInfo(&self, dwoutputstreamid: u32) -> windows_core::Result<MFT_OUTPUT_STREAM_INFO>;
    fn GetAttributes(&self) -> windows_core::Result<super::IMFAttributes>;
    fn GetInputStreamAttributes(&self, dwinputstreamid: u32) -> windows_core::Result<super::IMFAttributes>;
    fn GetOutputStreamAttributes(&self, dwoutputstreamid: u32) -> windows_core::Result<super::IMFAttributes>;
    fn DeleteInputStream(&self, dwstreamid: u32) -> windows_core::Result<()>;
    fn AddInputStreams(&self, cstreams: u32, adwstreamids: *const u32) -> windows_core::Result<()>;
    fn GetInputAvailableType(&self, dwinputstreamid: u32, dwtypeindex: u32) -> windows_core::Result<super::IMFMediaType>;
    fn GetOutputAvailableType(&self, dwoutputstreamid: u32, dwtypeindex: u32) -> windows_core::Result<super::IMFMediaType>;
    fn SetInputType(&self, dwinputstreamid: u32, ptype: windows_core::Ref<super::IMFMediaType>, dwflags: u32) -> windows_core::Result<()>;
    fn SetOutputType(&self, dwoutputstreamid: u32, ptype: windows_core::Ref<super::IMFMediaType>, dwflags: u32) -> windows_core::Result<()>;
    fn GetInputCurrentType(&self, dwinputstreamid: u32) -> windows_core::Result<super::IMFMediaType>;
    fn GetOutputCurrentType(&self, dwoutputstreamid: u32) -> windows_core::Result<super::IMFMediaType>;
    fn GetInputStatus(&self, dwinputstreamid: u32) -> windows_core::Result<u32>;
    fn GetOutputStatus(&self) -> windows_core::Result<u32>;
    fn SetOutputBounds(&self, hnslowerbound: i64, hnsupperbound: i64) -> windows_core::Result<()>;
    fn ProcessEvent(&self, dwinputstreamid: u32, pevent: windows_core::Ref<super::IMFMediaEvent>) -> windows_core::Result<()>;
    fn ProcessMessage(&self, emessage: MFT_MESSAGE_TYPE, ulparam: usize) -> windows_core::Result<()>;
    fn ProcessInput(&self, dwinputstreamid: u32, psample: windows_core::Ref<super::IMFSample>, dwflags: u32) -> windows_core::Result<()>;
    fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputsamples: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFTransform_Vtbl {
    pub const fn new<Identity: IMFTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamLimits<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwinputminimum: *mut u32, pdwinputmaximum: *mut u32, pdwoutputminimum: *mut u32, pdwoutputmaximum: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetStreamLimits(this, core::mem::transmute_copy(&pdwinputminimum), core::mem::transmute_copy(&pdwinputmaximum), core::mem::transmute_copy(&pdwoutputminimum), core::mem::transmute_copy(&pdwoutputmaximum)).into()
            }
        }
        unsafe extern "system" fn GetStreamCount<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetStreamCount(this, core::mem::transmute_copy(&pcinputstreams), core::mem::transmute_copy(&pcoutputstreams)).into()
            }
        }
        unsafe extern "system" fn GetStreamIDs<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputidarraysize: u32, pdwinputids: *mut u32, dwoutputidarraysize: u32, pdwoutputids: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetStreamIDs(this, core::mem::transmute_copy(&dwinputidarraysize), core::mem::transmute_copy(&pdwinputids), core::mem::transmute_copy(&dwoutputidarraysize), core::mem::transmute_copy(&pdwoutputids)).into()
            }
        }
        unsafe extern "system" fn GetInputStreamInfo<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, pstreaminfo: *mut MFT_INPUT_STREAM_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetInputStreamInfo(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&pstreaminfo)).into()
            }
        }
        unsafe extern "system" fn GetOutputStreamInfo<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: u32, pstreaminfo: *mut MFT_OUTPUT_STREAM_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetOutputStreamInfo(this, core::mem::transmute_copy(&dwoutputstreamid)) {
                    Ok(ok__) => {
                        pstreaminfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
        unsafe extern "system" fn GetInputStreamAttributes<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, pattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetOutputStreamAttributes<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: u32, pattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn DeleteInputStream<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::DeleteInputStream(this, core::mem::transmute_copy(&dwstreamid)).into()
            }
        }
        unsafe extern "system" fn AddInputStreams<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cstreams: u32, adwstreamids: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::AddInputStreams(this, core::mem::transmute_copy(&cstreams), core::mem::transmute_copy(&adwstreamids)).into()
            }
        }
        unsafe extern "system" fn GetInputAvailableType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, dwtypeindex: u32, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetOutputAvailableType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: u32, dwtypeindex: u32, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetInputType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, ptype: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::SetInputType(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SetOutputType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: u32, ptype: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::SetOutputType(this, core::mem::transmute_copy(&dwoutputstreamid), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetInputCurrentType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetOutputCurrentType<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: u32, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetInputStatus<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetInputStatus(this, core::mem::transmute_copy(&dwinputstreamid)) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputStatus<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetOutputStatus(this) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutputBounds<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnslowerbound: i64, hnsupperbound: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::SetOutputBounds(this, core::mem::transmute_copy(&hnslowerbound), core::mem::transmute_copy(&hnsupperbound)).into()
            }
        }
        unsafe extern "system" fn ProcessEvent<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::ProcessEvent(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&pevent)).into()
            }
        }
        unsafe extern "system" fn ProcessMessage<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emessage: MFT_MESSAGE_TYPE, ulparam: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::ProcessMessage(this, core::mem::transmute_copy(&emessage), core::mem::transmute_copy(&ulparam)).into()
            }
        }
        unsafe extern "system" fn ProcessInput<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamid: u32, psample: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::ProcessInput(this, core::mem::transmute_copy(&dwinputstreamid), core::mem::transmute_copy(&psample), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn ProcessOutput<Identity: IMFTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputsamples: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::HRESULT {
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
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFTransform {}
pub type MF3DVideoOutputType = i32;
pub const MF3DVideoOutputType_BaseView: MF3DVideoOutputType = 0;
pub const MF3DVideoOutputType_Stereo: MF3DVideoOutputType = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFAudioDecoderDegradationInfo {
    pub eDegradationReason: MFT_AUDIO_DECODER_DEGRADATION_REASON,
    pub eType: MFT_AUDIO_DECODER_DEGRADATION_TYPE,
}
pub type MFT_AUDIO_DECODER_DEGRADATION_REASON = i32;
pub const MFT_AUDIO_DECODER_DEGRADATION_REASON_LICENSING_REQUIREMENT: MFT_AUDIO_DECODER_DEGRADATION_REASON = 1;
pub const MFT_AUDIO_DECODER_DEGRADATION_REASON_NONE: MFT_AUDIO_DECODER_DEGRADATION_REASON = 0;
pub type MFT_AUDIO_DECODER_DEGRADATION_TYPE = i32;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX2CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 1;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX6CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 2;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX8CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 3;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_NONE: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 0;
pub const MFT_DRAIN_NO_TAILS: _MFT_DRAIN_TYPE = 1;
pub const MFT_DRAIN_PRODUCE_TAILS: _MFT_DRAIN_TYPE = 0;
pub const MFT_INPUT_DATA_BUFFER_PLACEHOLDER: _MFT_INPUT_DATA_BUFFER_FLAGS = -1;
pub const MFT_INPUT_STATUS_ACCEPT_DATA: _MFT_INPUT_STATUS_FLAGS = 1;
pub const MFT_INPUT_STREAM_DOES_NOT_ADDREF: _MFT_INPUT_STREAM_INFO_FLAGS = 256;
pub const MFT_INPUT_STREAM_FIXED_SAMPLE_SIZE: _MFT_INPUT_STREAM_INFO_FLAGS = 4;
pub const MFT_INPUT_STREAM_HOLDS_BUFFERS: _MFT_INPUT_STREAM_INFO_FLAGS = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFT_INPUT_STREAM_INFO {
    pub hnsMaxLatency: i64,
    pub dwFlags: u32,
    pub cbSize: u32,
    pub cbMaxLookahead: u32,
    pub cbAlignment: u32,
}
pub const MFT_INPUT_STREAM_OPTIONAL: _MFT_INPUT_STREAM_INFO_FLAGS = 1024;
pub const MFT_INPUT_STREAM_PROCESSES_IN_PLACE: _MFT_INPUT_STREAM_INFO_FLAGS = 2048;
pub const MFT_INPUT_STREAM_REMOVABLE: _MFT_INPUT_STREAM_INFO_FLAGS = 512;
pub const MFT_INPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER: _MFT_INPUT_STREAM_INFO_FLAGS = 2;
pub const MFT_INPUT_STREAM_WHOLE_SAMPLES: _MFT_INPUT_STREAM_INFO_FLAGS = 1;
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
pub const MFT_OUTPUT_BOUND_LOWER_UNBOUNDED: u32 = 0;
pub const MFT_OUTPUT_BOUND_UPPER_UNBOUNDED: i32 = -1;
#[repr(C)]
#[cfg(feature = "mfobjects")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFT_OUTPUT_DATA_BUFFER {
    pub dwStreamID: u32,
    pub pSample: core::mem::ManuallyDrop<Option<super::IMFSample>>,
    pub dwStatus: u32,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFT_OUTPUT_STREAM_INFO {
    pub dwFlags: u32,
    pub cbSize: u32,
    pub cbAlignment: u32,
}
pub const MFT_OUTPUT_STREAM_LAZY_READ: _MFT_OUTPUT_STREAM_INFO_FLAGS = 1024;
pub const MFT_OUTPUT_STREAM_OPTIONAL: _MFT_OUTPUT_STREAM_INFO_FLAGS = 16;
pub const MFT_OUTPUT_STREAM_PROVIDES_SAMPLES: _MFT_OUTPUT_STREAM_INFO_FLAGS = 256;
pub const MFT_OUTPUT_STREAM_REMOVABLE: _MFT_OUTPUT_STREAM_INFO_FLAGS = 2048;
pub const MFT_OUTPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER: _MFT_OUTPUT_STREAM_INFO_FLAGS = 2;
pub const MFT_OUTPUT_STREAM_WHOLE_SAMPLES: _MFT_OUTPUT_STREAM_INFO_FLAGS = 1;
pub const MFT_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _MFT_PROCESS_OUTPUT_FLAGS = 1;
pub const MFT_PROCESS_OUTPUT_REGENERATE_LAST_OUTPUT: _MFT_PROCESS_OUTPUT_FLAGS = 2;
pub const MFT_PROCESS_OUTPUT_STATUS_NEW_STREAMS: _MFT_PROCESS_OUTPUT_STATUS = 256;
pub const MFT_SET_TYPE_TEST_ONLY: _MFT_SET_TYPE_FLAGS = 1;
pub const MFT_STREAMS_UNLIMITED: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "mfobjects")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFT_STREAM_STATE_PARAM {
    pub StreamId: u32,
    pub State: super::MF_STREAM_STATE,
}
pub type PDeviceStreamState = *mut DeviceStreamState;
#[cfg(feature = "mfobjects")]
pub type PMFT_OUTPUT_DATA_BUFFER = *mut MFT_OUTPUT_DATA_BUFFER;
#[cfg(feature = "mfobjects")]
pub type PMFT_STREAM_STATE_PARAM = *mut MFT_STREAM_STATE_PARAM;
pub type PSTREAM_MEDIUM = *mut STREAM_MEDIUM;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STREAM_MEDIUM {
    pub gidMedium: windows_core::GUID,
    pub unMediumInstance: u32,
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
