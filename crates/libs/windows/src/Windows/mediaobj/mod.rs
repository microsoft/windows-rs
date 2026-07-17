pub const DMO_INPLACE_NORMAL: _DMO_INPLACE_PROCESS_FLAGS = 0;
pub const DMO_INPLACE_ZERO: _DMO_INPLACE_PROCESS_FLAGS = 1;
pub const DMO_INPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_INPUT_DATA_BUFFER_FLAGS = 8;
pub const DMO_INPUT_DATA_BUFFERF_SYNCPOINT: _DMO_INPUT_DATA_BUFFER_FLAGS = 1;
pub const DMO_INPUT_DATA_BUFFERF_TIME: _DMO_INPUT_DATA_BUFFER_FLAGS = 2;
pub const DMO_INPUT_DATA_BUFFERF_TIMELENGTH: _DMO_INPUT_DATA_BUFFER_FLAGS = 4;
pub const DMO_INPUT_STATUSF_ACCEPT_DATA: _DMO_INPUT_STATUS_FLAGS = 1;
pub const DMO_INPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_INPUT_STREAM_INFO_FLAGS = 4;
pub const DMO_INPUT_STREAMF_HOLDS_BUFFERS: _DMO_INPUT_STREAM_INFO_FLAGS = 8;
pub const DMO_INPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_INPUT_STREAM_INFO_FLAGS = 2;
pub const DMO_INPUT_STREAMF_WHOLE_SAMPLES: _DMO_INPUT_STREAM_INFO_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DMO_MEDIA_TYPE {
    pub majortype: windows_core::GUID,
    pub subtype: windows_core::GUID,
    pub bFixedSizeSamples: windows_core::BOOL,
    pub bTemporalCompression: windows_core::BOOL,
    pub lSampleSize: u32,
    pub formattype: windows_core::GUID,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
impl Default for DMO_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DMO_OUTPUT_DATA_BUFFER {
    pub pBuffer: core::mem::ManuallyDrop<Option<IMediaBuffer>>,
    pub dwStatus: u32,
    pub rtTimestamp: super::ksmedia::REFERENCE_TIME,
    pub rtTimelength: super::ksmedia::REFERENCE_TIME,
}
pub const DMO_OUTPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 8;
pub const DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 16777216;
pub const DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 1;
pub const DMO_OUTPUT_DATA_BUFFERF_TIME: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 2;
pub const DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 4;
pub const DMO_OUTPUT_STREAMF_DISCARDABLE: _DMO_OUTPUT_STREAM_INFO_FLAGS = 8;
pub const DMO_OUTPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_OUTPUT_STREAM_INFO_FLAGS = 4;
pub const DMO_OUTPUT_STREAMF_OPTIONAL: _DMO_OUTPUT_STREAM_INFO_FLAGS = 16;
pub const DMO_OUTPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_OUTPUT_STREAM_INFO_FLAGS = 2;
pub const DMO_OUTPUT_STREAMF_WHOLE_SAMPLES: _DMO_OUTPUT_STREAM_INFO_FLAGS = 1;
pub const DMO_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _DMO_PROCESS_OUTPUT_FLAGS = 1;
pub const DMO_QUALITY_STATUS_ENABLED: _DMO_QUALITY_STATUS_FLAGS = 1;
pub const DMO_SET_TYPEF_CLEAR: _DMO_SET_TYPE_FLAGS = 2;
pub const DMO_SET_TYPEF_TEST_ONLY: _DMO_SET_TYPE_FLAGS = 1;
pub const DMO_VOSF_NEEDS_PREVIOUS_SAMPLE: _DMO_VIDEO_OUTPUT_STREAM_FLAGS = 1;
windows_core::imp::define_interface!(IDMOQualityControl, IDMOQualityControl_Vtbl, 0x65abea96_cf36_453f_af8a_705e98f16260);
windows_core::imp::interface_hierarchy!(IDMOQualityControl, windows_core::IUnknown);
impl IDMOQualityControl {
    #[cfg(feature = "ksmedia")]
    pub unsafe fn SetNow(&self, rtnow: super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNow)(windows_core::Interface::as_raw(self), rtnow) }
    }
    pub unsafe fn SetStatus(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOQualityControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ksmedia")]
    pub SetNow: unsafe extern "system" fn(*mut core::ffi::c_void, super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    SetNow: usize,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "ksmedia")]
pub trait IDMOQualityControl_Impl: windows_core::IUnknownImpl {
    fn SetNow(&self, rtnow: super::ksmedia::REFERENCE_TIME) -> windows_core::Result<()>;
    fn SetStatus(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "ksmedia")]
impl IDMOQualityControl_Vtbl {
    pub const fn new<Identity: IDMOQualityControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNow<Identity: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtnow: super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDMOQualityControl_Impl::SetNow(this, core::mem::transmute_copy(&rtnow)).into()
            }
        }
        unsafe extern "system" fn SetStatus<Identity: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDMOQualityControl_Impl::SetStatus(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDMOQualityControl_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetNow: SetNow::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMOQualityControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ksmedia")]
impl windows_core::RuntimeName for IDMOQualityControl {}
windows_core::imp::define_interface!(IDMOVideoOutputOptimizations, IDMOVideoOutputOptimizations_Vtbl, 0xbe8f4f4e_5b16_4d29_b350_7f6b5d9298ac);
windows_core::imp::interface_hierarchy!(IDMOVideoOutputOptimizations, windows_core::IUnknown);
impl IDMOVideoOutputOptimizations {
    pub unsafe fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryOperationModePreferences)(windows_core::Interface::as_raw(self), uloutputstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOperationMode)(windows_core::Interface::as_raw(self), uloutputstreamindex, dwenabledfeatures) }
    }
    pub unsafe fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentOperationMode)(windows_core::Interface::as_raw(self), uloutputstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSampleRequirements)(windows_core::Interface::as_raw(self), uloutputstreamindex, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOVideoOutputOptimizations_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryOperationModePreferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetOperationMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetCurrentOperationMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentSampleRequirements: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IDMOVideoOutputOptimizations_Impl: windows_core::IUnknownImpl {
    fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> windows_core::Result<u32>;
    fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> windows_core::Result<()>;
    fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> windows_core::Result<u32>;
    fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> windows_core::Result<u32>;
}
impl IDMOVideoOutputOptimizations_Vtbl {
    pub const fn new<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryOperationModePreferences<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDMOVideoOutputOptimizations_Impl::QueryOperationModePreferences(this, core::mem::transmute_copy(&uloutputstreamindex)) {
                    Ok(ok__) => {
                        pdwrequestedcapabilities.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOperationMode<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, dwenabledfeatures: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDMOVideoOutputOptimizations_Impl::SetOperationMode(this, core::mem::transmute_copy(&uloutputstreamindex), core::mem::transmute_copy(&dwenabledfeatures)).into()
            }
        }
        unsafe extern "system" fn GetCurrentOperationMode<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDMOVideoOutputOptimizations_Impl::GetCurrentOperationMode(this, core::mem::transmute_copy(&uloutputstreamindex)) {
                    Ok(ok__) => {
                        pdwenabledfeatures.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentSampleRequirements<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDMOVideoOutputOptimizations_Impl::GetCurrentSampleRequirements(this, core::mem::transmute_copy(&uloutputstreamindex)) {
                    Ok(ok__) => {
                        pdwrequestedfeatures.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryOperationModePreferences: QueryOperationModePreferences::<Identity, OFFSET>,
            SetOperationMode: SetOperationMode::<Identity, OFFSET>,
            GetCurrentOperationMode: GetCurrentOperationMode::<Identity, OFFSET>,
            GetCurrentSampleRequirements: GetCurrentSampleRequirements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMOVideoOutputOptimizations as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDMOVideoOutputOptimizations {}
windows_core::imp::define_interface!(IEnumDMO, IEnumDMO_Vtbl, 0x2c3cd98a_2bfa_4a53_9c27_5249ba64ba0f);
windows_core::imp::interface_hierarchy!(IEnumDMO, windows_core::IUnknown);
impl IEnumDMO {
    pub unsafe fn Next(&self, citemstofetch: u32, pclsid: *mut windows_core::GUID, names: *mut windows_core::PWSTR, pcitemsfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), citemstofetch, pclsid as _, names as _, pcitemsfetched as _) }
    }
    pub unsafe fn Skip(&self, citemstoskip: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), citemstoskip) }
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
pub struct IEnumDMO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumDMO_Impl: windows_core::IUnknownImpl {
    fn Next(&self, citemstofetch: u32, pclsid: *mut windows_core::GUID, names: *mut windows_core::PWSTR, pcitemsfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, citemstoskip: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDMO>;
}
impl IEnumDMO_Vtbl {
    pub const fn new<Identity: IEnumDMO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, citemstofetch: u32, pclsid: *mut windows_core::GUID, names: *mut windows_core::PWSTR, pcitemsfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDMO_Impl::Next(this, core::mem::transmute_copy(&citemstofetch), core::mem::transmute_copy(&pclsid), core::mem::transmute_copy(&names), core::mem::transmute_copy(&pcitemsfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, citemstoskip: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDMO_Impl::Skip(this, core::mem::transmute_copy(&citemstoskip)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDMO_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDMO_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
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
        iid == &<IEnumDMO as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumDMO {}
windows_core::imp::define_interface!(IMediaBuffer, IMediaBuffer_Vtbl, 0x59eff8b9_938c_4a26_82f2_95cb84cdc837);
windows_core::imp::interface_hierarchy!(IMediaBuffer, windows_core::IUnknown);
impl IMediaBuffer {
    pub unsafe fn SetLength(&self, cblength: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLength)(windows_core::Interface::as_raw(self), cblength) }
    }
    pub unsafe fn GetMaxLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBufferAndLength(&self, ppbuffer: *mut *mut u8, pcblength: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBufferAndLength)(windows_core::Interface::as_raw(self), ppbuffer as _, pcblength.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IMediaBuffer_Impl: windows_core::IUnknownImpl {
    fn SetLength(&self, cblength: u32) -> windows_core::Result<()>;
    fn GetMaxLength(&self) -> windows_core::Result<u32>;
    fn GetBufferAndLength(&self, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> windows_core::Result<()>;
}
impl IMediaBuffer_Vtbl {
    pub const fn new<Identity: IMediaBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLength<Identity: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cblength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaBuffer_Impl::SetLength(this, core::mem::transmute_copy(&cblength)).into()
            }
        }
        unsafe extern "system" fn GetMaxLength<Identity: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbmaxlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaBuffer_Impl::GetMaxLength(this) {
                    Ok(ok__) => {
                        pcbmaxlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Identity: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaBuffer_Impl::GetBufferAndLength(this, core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLength: SetLength::<Identity, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMediaBuffer {}
windows_core::imp::define_interface!(IMediaObject, IMediaObject_Vtbl, 0xd8ad0f58_5494_4102_97c5_ec798e59bcf4);
windows_core::imp::interface_hierarchy!(IMediaObject, windows_core::IUnknown);
impl IMediaObject {
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), pcinputstreams as _, pcoutputstreams as _) }
    }
    pub unsafe fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamInfo)(windows_core::Interface::as_raw(self), dwinputstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamInfo)(windows_core::Interface::as_raw(self), dwoutputstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32, pmt: Option<*mut DMO_MEDIA_TYPE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputType)(windows_core::Interface::as_raw(self), dwinputstreamindex, dwtypeindex, pmt.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: Option<*mut DMO_MEDIA_TYPE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputType)(windows_core::Interface::as_raw(self), dwoutputstreamindex, dwtypeindex, pmt.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetInputType(&self, dwinputstreamindex: u32, pmt: Option<*const DMO_MEDIA_TYPE>, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInputType)(windows_core::Interface::as_raw(self), dwinputstreamindex, pmt.unwrap_or(core::mem::zeroed()) as _, dwflags) }
    }
    pub unsafe fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: Option<*const DMO_MEDIA_TYPE>, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOutputType)(windows_core::Interface::as_raw(self), dwoutputstreamindex, pmt.unwrap_or(core::mem::zeroed()) as _, dwflags) }
    }
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputCurrentType)(windows_core::Interface::as_raw(self), dwinputstreamindex, pmt) }
    }
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputCurrentType)(windows_core::Interface::as_raw(self), dwoutputstreamindex, pmt) }
    }
    pub unsafe fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputSizeInfo)(windows_core::Interface::as_raw(self), dwinputstreamindex, pcbsize as _, pcbmaxlookahead as _, pcbalignment as _) }
    }
    pub unsafe fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputSizeInfo)(windows_core::Interface::as_raw(self), dwoutputstreamindex, pcbsize as _, pcbalignment as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> windows_core::Result<super::ksmedia::REFERENCE_TIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputMaxLatency)(windows_core::Interface::as_raw(self), dwinputstreamindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn SetInputMaxLatency(&self, dwinputstreamindex: u32, rtmaxlatency: super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInputMaxLatency)(windows_core::Interface::as_raw(self), dwinputstreamindex, rtmaxlatency) }
    }
    pub unsafe fn Flush(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Discontinuity(&self, dwinputstreamindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Discontinuity)(windows_core::Interface::as_raw(self), dwinputstreamindex) }
    }
    pub unsafe fn AllocateStreamingResources(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AllocateStreamingResources)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FreeStreamingResources(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeStreamingResources)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetInputStatus(&self, dwinputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStatus)(windows_core::Interface::as_raw(self), dwinputstreamindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn ProcessInput<P1>(&self, dwinputstreamindex: u32, pbuffer: P1, dwflags: u32, rttimestamp: super::ksmedia::REFERENCE_TIME, rttimelength: super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IMediaBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessInput)(windows_core::Interface::as_raw(self), dwinputstreamindex, pbuffer.param().abi(), dwflags, rttimestamp, rttimelength) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn ProcessOutput(&self, dwflags: u32, poutputbuffers: &mut [DMO_OUTPUT_DATA_BUFFER], pdwstatus: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessOutput)(windows_core::Interface::as_raw(self), dwflags, poutputbuffers.len().try_into().unwrap(), poutputbuffers.as_mut_ptr(), pdwstatus as _) }
    }
    pub unsafe fn Lock(&self, block: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), block) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetInputStreamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetOutputStreamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetInputType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT,
    pub GetOutputType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT,
    pub SetInputType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DMO_MEDIA_TYPE, u32) -> windows_core::HRESULT,
    pub SetOutputType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DMO_MEDIA_TYPE, u32) -> windows_core::HRESULT,
    pub GetInputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT,
    pub GetOutputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT,
    pub GetInputSizeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetOutputSizeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetInputMaxLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetInputMaxLatency: usize,
    #[cfg(feature = "ksmedia")]
    pub SetInputMaxLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    SetInputMaxLatency: usize,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Discontinuity: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub AllocateStreamingResources: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FreeStreamingResources: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInputStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub ProcessInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, super::ksmedia::REFERENCE_TIME, super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    ProcessInput: usize,
    #[cfg(feature = "ksmedia")]
    pub ProcessOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DMO_OUTPUT_DATA_BUFFER, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    ProcessOutput: usize,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "ksmedia")]
pub trait IMediaObject_Impl: windows_core::IUnknownImpl {
    fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::Result<()>;
    fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> windows_core::Result<u32>;
    fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> windows_core::Result<u32>;
    fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn SetInputType(&self, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::Result<()>;
    fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::Result<()>;
    fn GetInputCurrentType(&self, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn GetOutputCurrentType(&self, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> windows_core::Result<()>;
    fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> windows_core::Result<()>;
    fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> windows_core::Result<super::ksmedia::REFERENCE_TIME>;
    fn SetInputMaxLatency(&self, dwinputstreamindex: u32, rtmaxlatency: super::ksmedia::REFERENCE_TIME) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
    fn Discontinuity(&self, dwinputstreamindex: u32) -> windows_core::Result<()>;
    fn AllocateStreamingResources(&self) -> windows_core::Result<()>;
    fn FreeStreamingResources(&self) -> windows_core::Result<()>;
    fn GetInputStatus(&self, dwinputstreamindex: u32) -> windows_core::Result<u32>;
    fn ProcessInput(&self, dwinputstreamindex: u32, pbuffer: windows_core::Ref<IMediaBuffer>, dwflags: u32, rttimestamp: super::ksmedia::REFERENCE_TIME, rttimelength: super::ksmedia::REFERENCE_TIME) -> windows_core::Result<()>;
    fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::Result<()>;
    fn Lock(&self, block: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "ksmedia")]
impl IMediaObject_Vtbl {
    pub const fn new<Identity: IMediaObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamCount<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetStreamCount(this, core::mem::transmute_copy(&pcinputstreams), core::mem::transmute_copy(&pcoutputstreams)).into()
            }
        }
        unsafe extern "system" fn GetInputStreamInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObject_Impl::GetInputStreamInfo(this, core::mem::transmute_copy(&dwinputstreamindex)) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputStreamInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObject_Impl::GetOutputStreamInfo(this, core::mem::transmute_copy(&dwoutputstreamindex)) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetInputType(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&dwtypeindex), core::mem::transmute_copy(&pmt)).into()
            }
        }
        unsafe extern "system" fn GetOutputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetOutputType(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&dwtypeindex), core::mem::transmute_copy(&pmt)).into()
            }
        }
        unsafe extern "system" fn SetInputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::SetInputType(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pmt), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SetOutputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::SetOutputType(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&pmt), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetInputCurrentType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetInputCurrentType(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pmt)).into()
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetOutputCurrentType(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&pmt)).into()
            }
        }
        unsafe extern "system" fn GetInputSizeInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetInputSizeInfo(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pcbsize), core::mem::transmute_copy(&pcbmaxlookahead), core::mem::transmute_copy(&pcbalignment)).into()
            }
        }
        unsafe extern "system" fn GetOutputSizeInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetOutputSizeInfo(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&pcbsize), core::mem::transmute_copy(&pcbalignment)).into()
            }
        }
        unsafe extern "system" fn GetInputMaxLatency<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, prtmaxlatency: *mut super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObject_Impl::GetInputMaxLatency(this, core::mem::transmute_copy(&dwinputstreamindex)) {
                    Ok(ok__) => {
                        prtmaxlatency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInputMaxLatency<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, rtmaxlatency: super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::SetInputMaxLatency(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&rtmaxlatency)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::Flush(this).into()
            }
        }
        unsafe extern "system" fn Discontinuity<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::Discontinuity(this, core::mem::transmute_copy(&dwinputstreamindex)).into()
            }
        }
        unsafe extern "system" fn AllocateStreamingResources<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::AllocateStreamingResources(this).into()
            }
        }
        unsafe extern "system" fn FreeStreamingResources<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::FreeStreamingResources(this).into()
            }
        }
        unsafe extern "system" fn GetInputStatus<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, dwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObject_Impl::GetInputStatus(this, core::mem::transmute_copy(&dwinputstreamindex)) {
                    Ok(ok__) => {
                        dwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProcessInput<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pbuffer: *mut core::ffi::c_void, dwflags: u32, rttimestamp: super::ksmedia::REFERENCE_TIME, rttimelength: super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::ProcessInput(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&rttimestamp), core::mem::transmute_copy(&rttimelength)).into()
            }
        }
        unsafe extern "system" fn ProcessOutput<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::ProcessOutput(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&coutputbuffercount), core::mem::transmute_copy(&poutputbuffers), core::mem::transmute_copy(&pdwstatus)).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::Lock(this, core::mem::transmute_copy(&block)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamCount: GetStreamCount::<Identity, OFFSET>,
            GetInputStreamInfo: GetInputStreamInfo::<Identity, OFFSET>,
            GetOutputStreamInfo: GetOutputStreamInfo::<Identity, OFFSET>,
            GetInputType: GetInputType::<Identity, OFFSET>,
            GetOutputType: GetOutputType::<Identity, OFFSET>,
            SetInputType: SetInputType::<Identity, OFFSET>,
            SetOutputType: SetOutputType::<Identity, OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Identity, OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Identity, OFFSET>,
            GetInputSizeInfo: GetInputSizeInfo::<Identity, OFFSET>,
            GetOutputSizeInfo: GetOutputSizeInfo::<Identity, OFFSET>,
            GetInputMaxLatency: GetInputMaxLatency::<Identity, OFFSET>,
            SetInputMaxLatency: SetInputMaxLatency::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            Discontinuity: Discontinuity::<Identity, OFFSET>,
            AllocateStreamingResources: AllocateStreamingResources::<Identity, OFFSET>,
            FreeStreamingResources: FreeStreamingResources::<Identity, OFFSET>,
            GetInputStatus: GetInputStatus::<Identity, OFFSET>,
            ProcessInput: ProcessInput::<Identity, OFFSET>,
            ProcessOutput: ProcessOutput::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ksmedia")]
impl windows_core::RuntimeName for IMediaObject {}
windows_core::imp::define_interface!(IMediaObjectInPlace, IMediaObjectInPlace_Vtbl, 0x651b9ad0_0fc7_4aa9_9538_d89931010741);
windows_core::imp::interface_hierarchy!(IMediaObjectInPlace, windows_core::IUnknown);
impl IMediaObjectInPlace {
    #[cfg(feature = "ksmedia")]
    pub unsafe fn Process(&self, pdata: &mut [u8], reftimestart: super::ksmedia::REFERENCE_TIME, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), pdata.len().try_into().unwrap(), pdata.as_mut_ptr(), reftimestart, dwflags) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetLatency(&self) -> windows_core::Result<super::ksmedia::REFERENCE_TIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObjectInPlace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ksmedia")]
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, super::ksmedia::REFERENCE_TIME, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    Process: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetLatency: usize,
}
#[cfg(feature = "ksmedia")]
pub trait IMediaObjectInPlace_Impl: windows_core::IUnknownImpl {
    fn Process(&self, ulsize: u32, pdata: *mut u8, reftimestart: super::ksmedia::REFERENCE_TIME, dwflags: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IMediaObjectInPlace>;
    fn GetLatency(&self) -> windows_core::Result<super::ksmedia::REFERENCE_TIME>;
}
#[cfg(feature = "ksmedia")]
impl IMediaObjectInPlace_Vtbl {
    pub const fn new<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Process<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u32, pdata: *mut u8, reftimestart: super::ksmedia::REFERENCE_TIME, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObjectInPlace_Impl::Process(this, core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&reftimestart), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediaobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObjectInPlace_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppmediaobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLatency<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, platencytime: *mut super::ksmedia::REFERENCE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObjectInPlace_Impl::GetLatency(this) {
                    Ok(ok__) => {
                        platencytime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Process: Process::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetLatency: GetLatency::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaObjectInPlace as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ksmedia")]
impl windows_core::RuntimeName for IMediaObjectInPlace {}
#[cfg(feature = "ksmedia")]
pub type PDMO_OUTPUT_DATA_BUFFER = *mut DMO_OUTPUT_DATA_BUFFER;
pub type _DMO_INPLACE_PROCESS_FLAGS = i32;
pub type _DMO_INPUT_DATA_BUFFER_FLAGS = i32;
pub type _DMO_INPUT_STATUS_FLAGS = i32;
pub type _DMO_INPUT_STREAM_INFO_FLAGS = i32;
pub type _DMO_OUTPUT_DATA_BUFFER_FLAGS = i32;
pub type _DMO_OUTPUT_STREAM_INFO_FLAGS = i32;
pub type _DMO_PROCESS_OUTPUT_FLAGS = i32;
pub type _DMO_QUALITY_STATUS_FLAGS = i32;
pub type _DMO_SET_TYPE_FLAGS = i32;
pub type _DMO_VIDEO_OUTPUT_STREAM_FLAGS = i32;
