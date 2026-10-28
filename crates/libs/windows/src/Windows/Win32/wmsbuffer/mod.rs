pub const IID_INSSBuffer: windows_core::GUID = windows_core::GUID::from_u128(0xe1cd3524_03d7_11d2_9eed_006097d2d7cf);
pub const IID_INSSBuffer2: windows_core::GUID = windows_core::GUID::from_u128(0x4f528693_1035_43fe_b428_757561ad3a68);
pub const IID_INSSBuffer3: windows_core::GUID = windows_core::GUID::from_u128(0xc87ceaaf_75be_4bc4_84eb_ac2798507672);
pub const IID_INSSBuffer4: windows_core::GUID = windows_core::GUID::from_u128(0xb6b8fd5a_32e2_49d4_a910_c26cc85465ed);
pub const IID_IWMSBuffer: windows_core::GUID = windows_core::GUID::from_u128(0xe1cd3524_03d7_11d2_9eed_006097d2d7cf);
pub const IID_IWMSBufferAllocator: windows_core::GUID = windows_core::GUID::from_u128(0x61103ca4_2033_11d2_9ef1_006097d2d7cf);
windows_core::imp::define_interface!(INSSBuffer, INSSBuffer_Vtbl, 0xe1cd3524_03d7_11d2_9eed_006097d2d7cf);
windows_core::imp::interface_hierarchy!(INSSBuffer, windows_core::IUnknown);
impl INSSBuffer {
    #[cfg(feature = "shared")]
    pub unsafe fn GetLength(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetLength(&self, dwlength: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLength)(windows_core::Interface::as_raw(self), dwlength) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaxLength(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBuffer(&self) -> windows_core::Result<*mut super::BYTE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut super::BYTE, pdwlength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBufferAndLength)(windows_core::Interface::as_raw(self), ppdwbuffer as _, pdwlength as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLength: usize,
    #[cfg(feature = "shared")]
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetLength: usize,
    #[cfg(feature = "shared")]
    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaxLength: usize,
    #[cfg(feature = "shared")]
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBuffer: usize,
    #[cfg(feature = "shared")]
    pub GetBufferAndLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBufferAndLength: usize,
}
#[cfg(feature = "shared")]
pub trait INSSBuffer_Impl: windows_core::IUnknownImpl {
    fn GetLength(&self) -> windows_core::Result<super::DWORD>;
    fn SetLength(&self, dwlength: super::DWORD) -> windows_core::Result<()>;
    fn GetMaxLength(&self) -> windows_core::Result<super::DWORD>;
    fn GetBuffer(&self) -> windows_core::Result<*mut super::BYTE>;
    fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut super::BYTE, pdwlength: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl INSSBuffer_Vtbl {
    pub const fn new<Identity: INSSBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLength<Identity: INSSBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetLength<Identity: INSSBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlength: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INSSBuffer_Impl::SetLength(this, core::mem::transmute_copy(&dwlength)).into()
            }
        }
        unsafe extern "system" fn GetMaxLength<Identity: INSSBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetBuffer<Identity: INSSBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdwbuffer: *mut *mut super::BYTE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetBufferAndLength<Identity: INSSBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdwbuffer: *mut *mut super::BYTE, pdwlength: *mut super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetSampleProperties(&self, cbproperties: super::DWORD) -> windows_core::Result<super::BYTE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSampleProperties)(windows_core::Interface::as_raw(self), cbproperties, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSampleProperties(&self, cbproperties: super::DWORD, pbproperties: *const super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSampleProperties)(windows_core::Interface::as_raw(self), cbproperties, pbproperties) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer2_Vtbl {
    pub base__: INSSBuffer_Vtbl,
    #[cfg(feature = "shared")]
    pub GetSampleProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSampleProperties: usize,
    #[cfg(feature = "shared")]
    pub SetSampleProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSampleProperties: usize,
}
#[cfg(feature = "shared")]
pub trait INSSBuffer2_Impl: INSSBuffer_Impl {
    fn GetSampleProperties(&self, cbproperties: super::DWORD) -> windows_core::Result<super::BYTE>;
    fn SetSampleProperties(&self, cbproperties: super::DWORD, pbproperties: *const super::BYTE) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl INSSBuffer2_Vtbl {
    pub const fn new<Identity: INSSBuffer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSampleProperties<Identity: INSSBuffer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbproperties: super::DWORD, pbproperties: *mut super::BYTE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetSampleProperties<Identity: INSSBuffer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbproperties: super::DWORD, pbproperties: *const super::BYTE) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
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
    #[cfg(feature = "shared")]
    pub unsafe fn SetProperty(&self, guidbufferproperty: windows_core::GUID, pvbufferproperty: *const core::ffi::c_void, dwbufferpropertysize: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), guidbufferproperty, pvbufferproperty, dwbufferpropertysize) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetProperty(&self, guidbufferproperty: windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), guidbufferproperty, pvbufferproperty as _, pdwbufferpropertysize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer3_Vtbl {
    pub base__: INSSBuffer2_Vtbl,
    #[cfg(feature = "shared")]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetProperty: usize,
    #[cfg(feature = "shared")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetProperty: usize,
}
#[cfg(feature = "shared")]
pub trait INSSBuffer3_Impl: INSSBuffer2_Impl {
    fn SetProperty(&self, guidbufferproperty: &windows_core::GUID, pvbufferproperty: *const core::ffi::c_void, dwbufferpropertysize: super::DWORD) -> windows_core::Result<()>;
    fn GetProperty(&self, guidbufferproperty: &windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl INSSBuffer3_Vtbl {
    pub const fn new<Identity: INSSBuffer3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProperty<Identity: INSSBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidbufferproperty: windows_core::GUID, pvbufferproperty: *const core::ffi::c_void, dwbufferpropertysize: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INSSBuffer3_Impl::SetProperty(this, core::mem::transmute(&guidbufferproperty), core::mem::transmute_copy(&pvbufferproperty), core::mem::transmute_copy(&dwbufferpropertysize)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: INSSBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidbufferproperty: windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetPropertyCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPropertyByIndex(&self, dwbufferpropertyindex: super::DWORD, pguidbufferproperty: *mut windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyByIndex)(windows_core::Interface::as_raw(self), dwbufferpropertyindex, pguidbufferproperty as _, pvbufferproperty as _, pdwbufferpropertysize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer4_Vtbl {
    pub base__: INSSBuffer3_Vtbl,
    #[cfg(feature = "shared")]
    pub GetPropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPropertyCount: usize,
    #[cfg(feature = "shared")]
    pub GetPropertyByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::GUID, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPropertyByIndex: usize,
}
#[cfg(feature = "shared")]
pub trait INSSBuffer4_Impl: INSSBuffer3_Impl {
    fn GetPropertyCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetPropertyByIndex(&self, dwbufferpropertyindex: super::DWORD, pguidbufferproperty: *mut windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl INSSBuffer4_Vtbl {
    pub const fn new<Identity: INSSBuffer4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyCount<Identity: INSSBuffer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbufferproperties: *mut super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetPropertyByIndex<Identity: INSSBuffer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbufferpropertyindex: super::DWORD, pguidbufferproperty: *mut windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for INSSBuffer4 {}
windows_core::imp::define_interface!(IWMSBufferAllocator, IWMSBufferAllocator_Vtbl, 0x61103ca4_2033_11d2_9ef1_006097d2d7cf);
windows_core::imp::interface_hierarchy!(IWMSBufferAllocator, windows_core::IUnknown);
impl IWMSBufferAllocator {
    #[cfg(feature = "shared")]
    pub unsafe fn AllocateBuffer(&self, dwmaxbuffersize: super::DWORD) -> windows_core::Result<INSSBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllocateBuffer)(windows_core::Interface::as_raw(self), dwmaxbuffersize, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: super::DWORD) -> windows_core::Result<INSSBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllocatePageSizeBuffer)(windows_core::Interface::as_raw(self), dwmaxbuffersize, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSBufferAllocator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub AllocateBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AllocateBuffer: usize,
    #[cfg(feature = "shared")]
    pub AllocatePageSizeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AllocatePageSizeBuffer: usize,
}
#[cfg(feature = "shared")]
pub trait IWMSBufferAllocator_Impl: windows_core::IUnknownImpl {
    fn AllocateBuffer(&self, dwmaxbuffersize: super::DWORD) -> windows_core::Result<INSSBuffer>;
    fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: super::DWORD) -> windows_core::Result<INSSBuffer>;
}
#[cfg(feature = "shared")]
impl IWMSBufferAllocator_Vtbl {
    pub const fn new<Identity: IWMSBufferAllocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllocateBuffer<Identity: IWMSBufferAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxbuffersize: super::DWORD, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AllocatePageSizeBuffer<Identity: IWMSBufferAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxbuffersize: super::DWORD, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMSBufferAllocator {}
