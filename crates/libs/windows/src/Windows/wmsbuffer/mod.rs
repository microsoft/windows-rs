windows_core::imp::define_interface!(INSSBuffer, INSSBuffer_Vtbl, 0xe1cd3524_03d7_11d2_9eed_006097d2d7cf);
windows_core::imp::interface_hierarchy!(INSSBuffer, windows_core::IUnknown);
impl INSSBuffer {
    pub unsafe fn GetLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLength)(windows_core::Interface::as_raw(self), dwlength) }
    }
    pub unsafe fn GetMaxLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBuffer(&self) -> windows_core::Result<*mut u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBufferAndLength)(windows_core::Interface::as_raw(self), ppdwbuffer as _, pdwlength as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait INSSBuffer_Impl: windows_core::IUnknownImpl {
    fn GetLength(&self) -> windows_core::Result<u32>;
    fn SetLength(&self, dwlength: u32) -> windows_core::Result<()>;
    fn GetMaxLength(&self) -> windows_core::Result<u32>;
    fn GetBuffer(&self) -> windows_core::Result<*mut u8>;
    fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> windows_core::Result<()>;
}
impl INSSBuffer_Vtbl {
    pub const fn new<Identity: INSSBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLength<Identity: INSSBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INSSBuffer_Impl::GetLength(this) {
                    Ok(ok__) => {
                        pdwlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLength<Identity: INSSBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INSSBuffer_Impl::SetLength(this, core::mem::transmute_copy(&dwlength)).into()
            }
        }
        unsafe extern "system" fn GetMaxLength<Identity: INSSBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INSSBuffer_Impl::GetMaxLength(this) {
                    Ok(ok__) => {
                        pdwlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: INSSBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INSSBuffer_Impl::GetBuffer(this) {
                    Ok(ok__) => {
                        ppdwbuffer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Identity: INSSBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INSSBuffer_Impl::GetBufferAndLength(this, core::mem::transmute_copy(&ppdwbuffer), core::mem::transmute_copy(&pdwlength)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, OFFSET>,
            SetLength: SetLength::<Identity, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, OFFSET>,
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INSSBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INSSBuffer {}
windows_core::imp::define_interface!(INSSBuffer2, INSSBuffer2_Vtbl, 0x4f528693_1035_43fe_b428_757561ad3a68);
impl core::ops::Deref for INSSBuffer2 {
    type Target = INSSBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INSSBuffer2, windows_core::IUnknown, INSSBuffer);
impl INSSBuffer2 {
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSampleProperties)(windows_core::Interface::as_raw(self), cbproperties, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSampleProperties)(windows_core::Interface::as_raw(self), cbproperties, pbproperties) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer2_Vtbl {
    pub base__: INSSBuffer_Vtbl,
    pub GetSampleProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub SetSampleProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
pub trait INSSBuffer2_Impl: INSSBuffer_Impl {
    fn GetSampleProperties(&self, cbproperties: u32) -> windows_core::Result<u8>;
    fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> windows_core::Result<()>;
}
impl INSSBuffer2_Vtbl {
    pub const fn new<Identity: INSSBuffer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSampleProperties<Identity: INSSBuffer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INSSBuffer2_Impl::GetSampleProperties(this, core::mem::transmute_copy(&cbproperties)) {
                    Ok(ok__) => {
                        pbproperties.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSampleProperties<Identity: INSSBuffer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INSSBuffer2_Impl::SetSampleProperties(this, core::mem::transmute_copy(&cbproperties), core::mem::transmute_copy(&pbproperties)).into()
            }
        }
        Self {
            base__: INSSBuffer_Vtbl::new::<Identity, OFFSET>(),
            GetSampleProperties: GetSampleProperties::<Identity, OFFSET>,
            SetSampleProperties: SetSampleProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INSSBuffer2 as windows_core::Interface>::IID || iid == &<INSSBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INSSBuffer2 {}
windows_core::imp::define_interface!(INSSBuffer3, INSSBuffer3_Vtbl, 0xc87ceaaf_75be_4bc4_84eb_ac2798507672);
impl core::ops::Deref for INSSBuffer3 {
    type Target = INSSBuffer2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INSSBuffer3, windows_core::IUnknown, INSSBuffer, INSSBuffer2);
impl INSSBuffer3 {
    pub unsafe fn SetProperty(&self, guidbufferproperty: windows_core::GUID, pvbufferproperty: *const core::ffi::c_void, dwbufferpropertysize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), guidbufferproperty, pvbufferproperty, dwbufferpropertysize) }
    }
    pub unsafe fn GetProperty(&self, guidbufferproperty: windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), guidbufferproperty, pvbufferproperty as _, pdwbufferpropertysize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer3_Vtbl {
    pub base__: INSSBuffer2_Vtbl,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait INSSBuffer3_Impl: INSSBuffer2_Impl {
    fn SetProperty(&self, guidbufferproperty: &windows_core::GUID, pvbufferproperty: *const core::ffi::c_void, dwbufferpropertysize: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, guidbufferproperty: &windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::Result<()>;
}
impl INSSBuffer3_Vtbl {
    pub const fn new<Identity: INSSBuffer3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProperty<Identity: INSSBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidbufferproperty: windows_core::GUID, pvbufferproperty: *const core::ffi::c_void, dwbufferpropertysize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INSSBuffer3_Impl::SetProperty(this, core::mem::transmute(&guidbufferproperty), core::mem::transmute_copy(&pvbufferproperty), core::mem::transmute_copy(&dwbufferpropertysize)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: INSSBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidbufferproperty: windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INSSBuffer3_Impl::GetProperty(this, core::mem::transmute(&guidbufferproperty), core::mem::transmute_copy(&pvbufferproperty), core::mem::transmute_copy(&pdwbufferpropertysize)).into()
            }
        }
        Self { base__: INSSBuffer2_Vtbl::new::<Identity, OFFSET>(), SetProperty: SetProperty::<Identity, OFFSET>, GetProperty: GetProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INSSBuffer3 as windows_core::Interface>::IID || iid == &<INSSBuffer as windows_core::Interface>::IID || iid == &<INSSBuffer2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INSSBuffer3 {}
windows_core::imp::define_interface!(INSSBuffer4, INSSBuffer4_Vtbl, 0xb6b8fd5a_32e2_49d4_a910_c26cc85465ed);
impl core::ops::Deref for INSSBuffer4 {
    type Target = INSSBuffer3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INSSBuffer4, windows_core::IUnknown, INSSBuffer, INSSBuffer2, INSSBuffer3);
impl INSSBuffer4 {
    pub unsafe fn GetPropertyCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPropertyByIndex(&self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyByIndex)(windows_core::Interface::as_raw(self), dwbufferpropertyindex, pguidbufferproperty as _, pvbufferproperty as _, pdwbufferpropertysize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer4_Vtbl {
    pub base__: INSSBuffer3_Vtbl,
    pub GetPropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPropertyByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait INSSBuffer4_Impl: INSSBuffer3_Impl {
    fn GetPropertyCount(&self) -> windows_core::Result<u32>;
    fn GetPropertyByIndex(&self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::Result<()>;
}
impl INSSBuffer4_Vtbl {
    pub const fn new<Identity: INSSBuffer4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyCount<Identity: INSSBuffer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbufferproperties: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INSSBuffer4_Impl::GetPropertyCount(this) {
                    Ok(ok__) => {
                        pcbufferproperties.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyByIndex<Identity: INSSBuffer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INSSBuffer4_Impl::GetPropertyByIndex(this, core::mem::transmute_copy(&dwbufferpropertyindex), core::mem::transmute_copy(&pguidbufferproperty), core::mem::transmute_copy(&pvbufferproperty), core::mem::transmute_copy(&pdwbufferpropertysize)).into()
            }
        }
        Self {
            base__: INSSBuffer3_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Identity, OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INSSBuffer4 as windows_core::Interface>::IID || iid == &<INSSBuffer as windows_core::Interface>::IID || iid == &<INSSBuffer2 as windows_core::Interface>::IID || iid == &<INSSBuffer3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INSSBuffer4 {}
windows_core::imp::define_interface!(IWMSBufferAllocator, IWMSBufferAllocator_Vtbl, 0x61103ca4_2033_11d2_9ef1_006097d2d7cf);
windows_core::imp::interface_hierarchy!(IWMSBufferAllocator, windows_core::IUnknown);
impl IWMSBufferAllocator {
    pub unsafe fn AllocateBuffer(&self, dwmaxbuffersize: u32) -> windows_core::Result<INSSBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllocateBuffer)(windows_core::Interface::as_raw(self), dwmaxbuffersize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: u32) -> windows_core::Result<INSSBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllocatePageSizeBuffer)(windows_core::Interface::as_raw(self), dwmaxbuffersize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSBufferAllocator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllocateBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllocatePageSizeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMSBufferAllocator_Impl: windows_core::IUnknownImpl {
    fn AllocateBuffer(&self, dwmaxbuffersize: u32) -> windows_core::Result<INSSBuffer>;
    fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: u32) -> windows_core::Result<INSSBuffer>;
}
impl IWMSBufferAllocator_Vtbl {
    pub const fn new<Identity: IWMSBufferAllocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllocateBuffer<Identity: IWMSBufferAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSBufferAllocator_Impl::AllocateBuffer(this, core::mem::transmute_copy(&dwmaxbuffersize)) {
                    Ok(ok__) => {
                        ppbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AllocatePageSizeBuffer<Identity: IWMSBufferAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSBufferAllocator_Impl::AllocatePageSizeBuffer(this, core::mem::transmute_copy(&dwmaxbuffersize)) {
                    Ok(ok__) => {
                        ppbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AllocateBuffer: AllocateBuffer::<Identity, OFFSET>,
            AllocatePageSizeBuffer: AllocatePageSizeBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMSBufferAllocator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMSBufferAllocator {}
