windows_core::imp::define_interface!(ISpatialAudioMetadataClient, ISpatialAudioMetadataClient_Vtbl, 0x777d4a3b_f6ff_4a26_85dc_68d7cdeda1d4);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataClient, windows_core::IUnknown);
impl ISpatialAudioMetadataClient {
    pub unsafe fn ActivateSpatialAudioMetadataItems(&self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut Option<ISpatialAudioMetadataItemsBuffer>, metadataitems: *mut Option<ISpatialAudioMetadataItems>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataItems)(windows_core::Interface::as_raw(self), maxitemcount, framecount, core::mem::transmute(metadataitemsbuffer), core::mem::transmute(metadataitems)) }
    }
    pub unsafe fn GetSpatialAudioMetadataItemsBufferLength(&self, maxitemcount: u16) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpatialAudioMetadataItemsBufferLength)(windows_core::Interface::as_raw(self), maxitemcount, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ActivateSpatialAudioMetadataWriter(&self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> windows_core::Result<ISpatialAudioMetadataWriter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataWriter)(windows_core::Interface::as_raw(self), overflowmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ActivateSpatialAudioMetadataCopier(&self) -> windows_core::Result<ISpatialAudioMetadataCopier> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataCopier)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ActivateSpatialAudioMetadataReader(&self) -> windows_core::Result<ISpatialAudioMetadataReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataReader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ActivateSpatialAudioMetadataItems: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpatialAudioMetadataItemsBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u32) -> windows_core::HRESULT,
    pub ActivateSpatialAudioMetadataWriter: unsafe extern "system" fn(*mut core::ffi::c_void, SpatialAudioMetadataWriterOverflowMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateSpatialAudioMetadataCopier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateSpatialAudioMetadataReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataClient_Impl: windows_core::IUnknownImpl {
    fn ActivateSpatialAudioMetadataItems(&self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: windows_core::OutRef<ISpatialAudioMetadataItemsBuffer>, metadataitems: windows_core::OutRef<ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn GetSpatialAudioMetadataItemsBufferLength(&self, maxitemcount: u16) -> windows_core::Result<u32>;
    fn ActivateSpatialAudioMetadataWriter(&self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> windows_core::Result<ISpatialAudioMetadataWriter>;
    fn ActivateSpatialAudioMetadataCopier(&self) -> windows_core::Result<ISpatialAudioMetadataCopier>;
    fn ActivateSpatialAudioMetadataReader(&self) -> windows_core::Result<ISpatialAudioMetadataReader>;
}
impl ISpatialAudioMetadataClient_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateSpatialAudioMetadataItems<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut *mut core::ffi::c_void, metadataitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataItems(this, core::mem::transmute_copy(&maxitemcount), core::mem::transmute_copy(&framecount), core::mem::transmute_copy(&metadataitemsbuffer), core::mem::transmute_copy(&metadataitems)).into()
            }
        }
        unsafe extern "system" fn GetSpatialAudioMetadataItemsBufferLength<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxitemcount: u16, bufferlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataClient_Impl::GetSpatialAudioMetadataItemsBufferLength(this, core::mem::transmute_copy(&maxitemcount)) {
                    Ok(ok__) => {
                        bufferlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataWriter<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overflowmode: SpatialAudioMetadataWriterOverflowMode, metadatawriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataWriter(this, core::mem::transmute_copy(&overflowmode)) {
                    Ok(ok__) => {
                        metadatawriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataCopier<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadatacopier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataCopier(this) {
                    Ok(ok__) => {
                        metadatacopier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataReader<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadatareader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataReader(this) {
                    Ok(ok__) => {
                        metadatareader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ActivateSpatialAudioMetadataItems: ActivateSpatialAudioMetadataItems::<Identity, OFFSET>,
            GetSpatialAudioMetadataItemsBufferLength: GetSpatialAudioMetadataItemsBufferLength::<Identity, OFFSET>,
            ActivateSpatialAudioMetadataWriter: ActivateSpatialAudioMetadataWriter::<Identity, OFFSET>,
            ActivateSpatialAudioMetadataCopier: ActivateSpatialAudioMetadataCopier::<Identity, OFFSET>,
            ActivateSpatialAudioMetadataReader: ActivateSpatialAudioMetadataReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataClient as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioMetadataClient {}
windows_core::imp::define_interface!(ISpatialAudioMetadataCopier, ISpatialAudioMetadataCopier_Vtbl, 0xd224b233_e251_4fd0_9ca2_d5ecf9a68404);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataCopier, windows_core::IUnknown);
impl ISpatialAudioMetadataCopier {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), metadataitems.param().abi()) }
    }
    pub unsafe fn CopyMetadataForFrames<P2>(&self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: P2) -> windows_core::Result<u16>
    where
        P2: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyMetadataForFrames)(windows_core::Interface::as_raw(self), copyframecount, copymode, dstmetadataitems.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataCopier_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyMetadataForFrames: unsafe extern "system" fn(*mut core::ffi::c_void, u16, SpatialAudioMetadataCopyMode, *mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataCopier_Impl: windows_core::IUnknownImpl {
    fn Open(&self, metadataitems: windows_core::Ref<ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn CopyMetadataForFrames(&self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: windows_core::Ref<ISpatialAudioMetadataItems>) -> windows_core::Result<u16>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl ISpatialAudioMetadataCopier_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataCopier_Impl::Open(this, core::mem::transmute_copy(&metadataitems)).into()
            }
        }
        unsafe extern "system" fn CopyMetadataForFrames<Identity: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: *mut core::ffi::c_void, itemscopied: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataCopier_Impl::CopyMetadataForFrames(this, core::mem::transmute_copy(&copyframecount), core::mem::transmute_copy(&copymode), core::mem::transmute_copy(&dstmetadataitems)) {
                    Ok(ok__) => {
                        itemscopied.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataCopier_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            CopyMetadataForFrames: CopyMetadataForFrames::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataCopier as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioMetadataCopier {}
windows_core::imp::define_interface!(ISpatialAudioMetadataItems, ISpatialAudioMetadataItems_Vtbl, 0xbcd7c78f_3098_4f22_b547_a2f25a381269);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataItems, windows_core::IUnknown);
impl ISpatialAudioMetadataItems {
    pub unsafe fn GetFrameCount(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetItemCount(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxItemCount(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxValueBufferLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxValueBufferLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInfo(&self) -> windows_core::Result<SpatialAudioMetadataItemsInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetMaxItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetMaxValueBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialAudioMetadataItemsInfo) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataItems_Impl: windows_core::IUnknownImpl {
    fn GetFrameCount(&self) -> windows_core::Result<u16>;
    fn GetItemCount(&self) -> windows_core::Result<u16>;
    fn GetMaxItemCount(&self) -> windows_core::Result<u16>;
    fn GetMaxValueBufferLength(&self) -> windows_core::Result<u32>;
    fn GetInfo(&self) -> windows_core::Result<SpatialAudioMetadataItemsInfo>;
}
impl ISpatialAudioMetadataItems_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFrameCount<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framecount: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataItems_Impl::GetFrameCount(this) {
                    Ok(ok__) => {
                        framecount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemCount<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemcount: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataItems_Impl::GetItemCount(this) {
                    Ok(ok__) => {
                        itemcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxItemCount<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxitemcount: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataItems_Impl::GetMaxItemCount(this) {
                    Ok(ok__) => {
                        maxitemcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxValueBufferLength<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxvaluebufferlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataItems_Impl::GetMaxValueBufferLength(this) {
                    Ok(ok__) => {
                        maxvaluebufferlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInfo<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, info: *mut SpatialAudioMetadataItemsInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataItems_Impl::GetInfo(this) {
                    Ok(ok__) => {
                        info.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameCount: GetFrameCount::<Identity, OFFSET>,
            GetItemCount: GetItemCount::<Identity, OFFSET>,
            GetMaxItemCount: GetMaxItemCount::<Identity, OFFSET>,
            GetMaxValueBufferLength: GetMaxValueBufferLength::<Identity, OFFSET>,
            GetInfo: GetInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataItems as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioMetadataItems {}
windows_core::imp::define_interface!(ISpatialAudioMetadataItemsBuffer, ISpatialAudioMetadataItemsBuffer_Vtbl, 0x42640a16_e1bd_42d9_9ff6_031ab71a2dba);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataItemsBuffer, windows_core::IUnknown);
impl ISpatialAudioMetadataItemsBuffer {
    pub unsafe fn AttachToBuffer(&self, buffer: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AttachToBuffer)(windows_core::Interface::as_raw(self), buffer.as_mut_ptr(), buffer.len().try_into().unwrap()) }
    }
    pub unsafe fn AttachToPopulatedBuffer(&self, buffer: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AttachToPopulatedBuffer)(windows_core::Interface::as_raw(self), buffer.as_mut_ptr(), buffer.len().try_into().unwrap()) }
    }
    pub unsafe fn DetachBuffer(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DetachBuffer)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItemsBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AttachToBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
    pub AttachToPopulatedBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
    pub DetachBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataItemsBuffer_Impl: windows_core::IUnknownImpl {
    fn AttachToBuffer(&self, buffer: *mut u8, bufferlength: u32) -> windows_core::Result<()>;
    fn AttachToPopulatedBuffer(&self, buffer: *mut u8, bufferlength: u32) -> windows_core::Result<()>;
    fn DetachBuffer(&self) -> windows_core::Result<()>;
}
impl ISpatialAudioMetadataItemsBuffer_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttachToBuffer<Identity: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataItemsBuffer_Impl::AttachToBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&bufferlength)).into()
            }
        }
        unsafe extern "system" fn AttachToPopulatedBuffer<Identity: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataItemsBuffer_Impl::AttachToPopulatedBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&bufferlength)).into()
            }
        }
        unsafe extern "system" fn DetachBuffer<Identity: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataItemsBuffer_Impl::DetachBuffer(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AttachToBuffer: AttachToBuffer::<Identity, OFFSET>,
            AttachToPopulatedBuffer: AttachToPopulatedBuffer::<Identity, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataItemsBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioMetadataItemsBuffer {}
windows_core::imp::define_interface!(ISpatialAudioMetadataReader, ISpatialAudioMetadataReader_Vtbl, 0xb78e86a2_31d9_4c32_94d2_7df40fc7ebec);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataReader, windows_core::IUnknown);
impl ISpatialAudioMetadataReader {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), metadataitems.param().abi()) }
    }
    pub unsafe fn ReadNextItem(&self, commandcount: *mut u8, frameoffset: *mut u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadNextItem)(windows_core::Interface::as_raw(self), commandcount as _, frameoffset as _) }
    }
    pub unsafe fn ReadNextItemCommand(&self, commandid: *mut u8, valuebuffer: *mut core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadNextItemCommand)(windows_core::Interface::as_raw(self), commandid as _, valuebuffer as _, maxvaluebufferlength, valuebufferlength as _) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadNextItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u16) -> windows_core::HRESULT,
    pub ReadNextItemCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataReader_Impl: windows_core::IUnknownImpl {
    fn Open(&self, metadataitems: windows_core::Ref<ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn ReadNextItem(&self, commandcount: *mut u8, frameoffset: *mut u16) -> windows_core::Result<()>;
    fn ReadNextItemCommand(&self, commandid: *mut u8, valuebuffer: *mut core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl ISpatialAudioMetadataReader_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataReader_Impl::Open(this, core::mem::transmute_copy(&metadataitems)).into()
            }
        }
        unsafe extern "system" fn ReadNextItem<Identity: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandcount: *mut u8, frameoffset: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataReader_Impl::ReadNextItem(this, core::mem::transmute_copy(&commandcount), core::mem::transmute_copy(&frameoffset)).into()
            }
        }
        unsafe extern "system" fn ReadNextItemCommand<Identity: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: *mut u8, valuebuffer: *mut core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataReader_Impl::ReadNextItemCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&valuebuffer), core::mem::transmute_copy(&maxvaluebufferlength), core::mem::transmute_copy(&valuebufferlength)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataReader_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            ReadNextItem: ReadNextItem::<Identity, OFFSET>,
            ReadNextItemCommand: ReadNextItemCommand::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioMetadataReader {}
windows_core::imp::define_interface!(ISpatialAudioMetadataWriter, ISpatialAudioMetadataWriter_Vtbl, 0x1b17ca01_2955_444d_a430_537dc589a844);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataWriter, windows_core::IUnknown);
impl ISpatialAudioMetadataWriter {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), metadataitems.param().abi()) }
    }
    pub unsafe fn WriteNextItem(&self, frameoffset: u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteNextItem)(windows_core::Interface::as_raw(self), frameoffset) }
    }
    pub unsafe fn WriteNextItemCommand(&self, commandid: u8, valuebuffer: Option<*const core::ffi::c_void>, valuebufferlength: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteNextItemCommand)(windows_core::Interface::as_raw(self), commandid, valuebuffer.unwrap_or(core::mem::zeroed()) as _, valuebufferlength) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteNextItem: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub WriteNextItemCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataWriter_Impl: windows_core::IUnknownImpl {
    fn Open(&self, metadataitems: windows_core::Ref<ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn WriteNextItem(&self, frameoffset: u16) -> windows_core::Result<()>;
    fn WriteNextItemCommand(&self, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl ISpatialAudioMetadataWriter_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataWriter_Impl::Open(this, core::mem::transmute_copy(&metadataitems)).into()
            }
        }
        unsafe extern "system" fn WriteNextItem<Identity: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frameoffset: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataWriter_Impl::WriteNextItem(this, core::mem::transmute_copy(&frameoffset)).into()
            }
        }
        unsafe extern "system" fn WriteNextItemCommand<Identity: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataWriter_Impl::WriteNextItemCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&valuebuffer), core::mem::transmute_copy(&valuebufferlength)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataWriter_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            WriteNextItem: WriteNextItem::<Identity, OFFSET>,
            WriteNextItemCommand: WriteNextItemCommand::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataWriter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioMetadataWriter {}
#[cfg(feature = "spatialaudioclient")]
windows_core::imp::define_interface!(ISpatialAudioObjectForMetadataCommands, ISpatialAudioObjectForMetadataCommands_Vtbl, 0x0df2c94b_f5f9_472d_af6b_c46e0ac9cd05);
#[cfg(feature = "spatialaudioclient")]
impl core::ops::Deref for ISpatialAudioObjectForMetadataCommands {
    type Target = super::spatialaudioclient::ISpatialAudioObjectBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "spatialaudioclient")]
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectForMetadataCommands, windows_core::IUnknown, super::spatialaudioclient::ISpatialAudioObjectBase);
#[cfg(feature = "spatialaudioclient")]
impl ISpatialAudioObjectForMetadataCommands {
    pub unsafe fn WriteNextMetadataCommand(&self, commandid: u8, valuebuffer: Option<*const core::ffi::c_void>, valuebufferlength: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteNextMetadataCommand)(windows_core::Interface::as_raw(self), commandid, valuebuffer.unwrap_or(core::mem::zeroed()) as _, valuebufferlength) }
    }
}
#[cfg(feature = "spatialaudioclient")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataCommands_Vtbl {
    pub base__: super::spatialaudioclient::ISpatialAudioObjectBase_Vtbl,
    pub WriteNextMetadataCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "spatialaudioclient")]
pub trait ISpatialAudioObjectForMetadataCommands_Impl: super::spatialaudioclient::ISpatialAudioObjectBase_Impl {
    fn WriteNextMetadataCommand(&self, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "spatialaudioclient")]
impl ISpatialAudioObjectForMetadataCommands_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectForMetadataCommands_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteNextMetadataCommand<Identity: ISpatialAudioObjectForMetadataCommands_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectForMetadataCommands_Impl::WriteNextMetadataCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&valuebuffer), core::mem::transmute_copy(&valuebufferlength)).into()
            }
        }
        Self {
            base__: super::spatialaudioclient::ISpatialAudioObjectBase_Vtbl::new::<Identity, OFFSET>(),
            WriteNextMetadataCommand: WriteNextMetadataCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForMetadataCommands as windows_core::Interface>::IID || iid == &<super::spatialaudioclient::ISpatialAudioObjectBase as windows_core::Interface>::IID
    }
}
#[cfg(feature = "spatialaudioclient")]
impl windows_core::RuntimeName for ISpatialAudioObjectForMetadataCommands {}
#[cfg(feature = "spatialaudioclient")]
windows_core::imp::define_interface!(ISpatialAudioObjectForMetadataItems, ISpatialAudioObjectForMetadataItems_Vtbl, 0xddea49ff_3bc0_4377_8aad_9fbcfd808566);
#[cfg(feature = "spatialaudioclient")]
impl core::ops::Deref for ISpatialAudioObjectForMetadataItems {
    type Target = super::spatialaudioclient::ISpatialAudioObjectBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "spatialaudioclient")]
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectForMetadataItems, windows_core::IUnknown, super::spatialaudioclient::ISpatialAudioObjectBase);
#[cfg(feature = "spatialaudioclient")]
impl ISpatialAudioObjectForMetadataItems {
    pub unsafe fn GetSpatialAudioMetadataItems(&self) -> windows_core::Result<ISpatialAudioMetadataItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpatialAudioMetadataItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "spatialaudioclient")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataItems_Vtbl {
    pub base__: super::spatialaudioclient::ISpatialAudioObjectBase_Vtbl,
    pub GetSpatialAudioMetadataItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "spatialaudioclient")]
pub trait ISpatialAudioObjectForMetadataItems_Impl: super::spatialaudioclient::ISpatialAudioObjectBase_Impl {
    fn GetSpatialAudioMetadataItems(&self) -> windows_core::Result<ISpatialAudioMetadataItems>;
}
#[cfg(feature = "spatialaudioclient")]
impl ISpatialAudioObjectForMetadataItems_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectForMetadataItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSpatialAudioMetadataItems<Identity: ISpatialAudioObjectForMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectForMetadataItems_Impl::GetSpatialAudioMetadataItems(this) {
                    Ok(ok__) => {
                        metadataitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::spatialaudioclient::ISpatialAudioObjectBase_Vtbl::new::<Identity, OFFSET>(),
            GetSpatialAudioMetadataItems: GetSpatialAudioMetadataItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForMetadataItems as windows_core::Interface>::IID || iid == &<super::spatialaudioclient::ISpatialAudioObjectBase as windows_core::Interface>::IID
    }
}
#[cfg(feature = "spatialaudioclient")]
impl windows_core::RuntimeName for ISpatialAudioObjectForMetadataItems {}
#[cfg(feature = "spatialaudioclient")]
windows_core::imp::define_interface!(ISpatialAudioObjectRenderStreamForMetadata, ISpatialAudioObjectRenderStreamForMetadata_Vtbl, 0xbbc9c907_48d5_4a2e_a0c7_f7f0d67c1fb1);
#[cfg(feature = "spatialaudioclient")]
impl core::ops::Deref for ISpatialAudioObjectRenderStreamForMetadata {
    type Target = super::spatialaudioclient::ISpatialAudioObjectRenderStreamBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "spatialaudioclient")]
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamForMetadata, windows_core::IUnknown, super::spatialaudioclient::ISpatialAudioObjectRenderStreamBase);
#[cfg(feature = "spatialaudioclient")]
impl ISpatialAudioObjectRenderStreamForMetadata {
    pub unsafe fn ActivateSpatialAudioObjectForMetadataCommands(&self, r#type: super::spatialaudioclient::AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForMetadataCommands> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioObjectForMetadataCommands)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ActivateSpatialAudioObjectForMetadataItems(&self, r#type: super::spatialaudioclient::AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForMetadataItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioObjectForMetadataItems)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "spatialaudioclient")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
    pub base__: super::spatialaudioclient::ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObjectForMetadataCommands: unsafe extern "system" fn(*mut core::ffi::c_void, super::spatialaudioclient::AudioObjectType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateSpatialAudioObjectForMetadataItems: unsafe extern "system" fn(*mut core::ffi::c_void, super::spatialaudioclient::AudioObjectType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "spatialaudioclient")]
pub trait ISpatialAudioObjectRenderStreamForMetadata_Impl: super::spatialaudioclient::ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObjectForMetadataCommands(&self, r#type: super::spatialaudioclient::AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForMetadataCommands>;
    fn ActivateSpatialAudioObjectForMetadataItems(&self, r#type: super::spatialaudioclient::AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForMetadataItems>;
}
#[cfg(feature = "spatialaudioclient")]
impl ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataCommands<Identity: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: super::spatialaudioclient::AudioObjectType, audioobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectRenderStreamForMetadata_Impl::ActivateSpatialAudioObjectForMetadataCommands(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        audioobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataItems<Identity: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: super::spatialaudioclient::AudioObjectType, audioobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectRenderStreamForMetadata_Impl::ActivateSpatialAudioObjectForMetadataItems(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        audioobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::spatialaudioclient::ISpatialAudioObjectRenderStreamBase_Vtbl::new::<Identity, OFFSET>(),
            ActivateSpatialAudioObjectForMetadataCommands: ActivateSpatialAudioObjectForMetadataCommands::<Identity, OFFSET>,
            ActivateSpatialAudioObjectForMetadataItems: ActivateSpatialAudioObjectForMetadataItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamForMetadata as windows_core::Interface>::IID || iid == &<super::spatialaudioclient::ISpatialAudioObjectRenderStreamBase as windows_core::Interface>::IID
    }
}
#[cfg(feature = "spatialaudioclient")]
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStreamForMetadata {}
pub const SPATIAL_AUDIO_POSITION: u32 = 200;
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200;
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: i32 = -2004286956;
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: i32 = -2004286969;
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: i32 = -2004286968;
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: i32 = -2004286940;
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: i32 = -2004286942;
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: i32 = -2004286976;
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: i32 = -2004286955;
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: i32 = -2004286941;
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: i32 = -2004286967;
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: i32 = -2004286952;
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: i32 = -2004286974;
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: i32 = -2004286957;
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: i32 = -2004286939;
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: i32 = -2004286959;
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: i32 = -2004286951;
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: i32 = -2004286971;
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: i32 = -2004286973;
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: i32 = -2004286954;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: i32 = -2004286944;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: i32 = -2004286960;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: i32 = -2004286958;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: i32 = -2004286943;
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: i32 = -2004286970;
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: i32 = -2004286953;
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: i32 = -2004286975;
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: i32 = -2004286972;
pub type SpatialAudioMetadataCopyMode = i32;
pub const SpatialAudioMetadataCopy_Append: SpatialAudioMetadataCopyMode = 1;
pub const SpatialAudioMetadataCopy_AppendMergeWithFirst: SpatialAudioMetadataCopyMode = 3;
pub const SpatialAudioMetadataCopy_AppendMergeWithLast: SpatialAudioMetadataCopyMode = 2;
pub const SpatialAudioMetadataCopy_Overwrite: SpatialAudioMetadataCopyMode = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioMetadataItemsInfo {
    pub FrameCount: u16,
    pub ItemCount: u16,
    pub MaxItemCount: u16,
    pub MaxValueBufferLength: u32,
}
pub type SpatialAudioMetadataWriterOverflowMode = i32;
pub const SpatialAudioMetadataWriterOverflow_Fail: SpatialAudioMetadataWriterOverflowMode = 0;
pub const SpatialAudioMetadataWriterOverflow_MergeWithLast: SpatialAudioMetadataWriterOverflowMode = 2;
pub const SpatialAudioMetadataWriterOverflow_MergeWithNew: SpatialAudioMetadataWriterOverflowMode = 1;
#[repr(C, packed(1))]
#[cfg(all(feature = "audiosessiontypes", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "spatialaudioclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: *const super::mmeapi::WAVEFORMATEX,
    pub StaticObjectTypeMask: super::spatialaudioclient::AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::winnt::HANDLE,
    pub MetadataFormatId: windows_core::GUID,
    pub MaxMetadataItemCount: u16,
    pub MetadataActivationParams: *const super::propidlbase::PROPVARIANT,
    pub NotifyObject: core::mem::ManuallyDrop<Option<super::spatialaudioclient::ISpatialAudioObjectRenderStreamNotify>>,
}
#[cfg(all(feature = "audiosessiontypes", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "spatialaudioclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "audiosessiontypes", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "spatialaudioclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: *const super::mmeapi::WAVEFORMATEX,
    pub StaticObjectTypeMask: super::spatialaudioclient::AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::winnt::HANDLE,
    pub MetadataFormatId: windows_core::GUID,
    pub MaxMetadataItemCount: u32,
    pub MetadataActivationParams: *const super::propidlbase::PROPVARIANT,
    pub NotifyObject: core::mem::ManuallyDrop<Option<super::spatialaudioclient::ISpatialAudioObjectRenderStreamNotify>>,
    pub Options: super::spatialaudioclient::SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(all(feature = "audiosessiontypes", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "spatialaudioclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
