#[inline]
pub unsafe fn WSDCreateOutboundAttachment() -> windows_core::Result<IWSDOutboundAttachment> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateOutboundAttachment(ppattachment : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateOutboundAttachment(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(IWSDAttachment, IWSDAttachment_Vtbl, 0x5d55a616_9df8_4b09_b156_9ba351a48b76);
windows_core::imp::interface_hierarchy!(IWSDAttachment, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAttachment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IWSDAttachment_Impl: windows_core::IUnknownImpl {}
impl IWSDAttachment_Vtbl {
    pub const fn new<Identity: IWSDAttachment_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDAttachment {}
windows_core::imp::define_interface!(IWSDInboundAttachment, IWSDInboundAttachment_Vtbl, 0x5bd6ca65_233c_4fb8_9f7a_2641619655c9);
impl core::ops::Deref for IWSDInboundAttachment {
    type Target = IWSDAttachment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDInboundAttachment, windows_core::IUnknown, IWSDAttachment);
impl IWSDInboundAttachment {
    pub unsafe fn Read(&self, pbuffer: &mut [u8], pdwnumberofbytesread: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), pdwnumberofbytesread as _) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDInboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWSDInboundAttachment_Impl: IWSDAttachment_Impl {
    fn Read(&self, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IWSDInboundAttachment_Vtbl {
    pub const fn new<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDInboundAttachment_Impl::Read(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwbytestoread), core::mem::transmute_copy(&pdwnumberofbytesread)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDInboundAttachment_Impl::Close(this).into()
            }
        }
        Self { base__: IWSDAttachment_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDInboundAttachment as windows_core::Interface>::IID || iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDInboundAttachment {}
windows_core::imp::define_interface!(IWSDOutboundAttachment, IWSDOutboundAttachment_Vtbl, 0xaa302f8d_5a22_4ba5_b392_aa8486f4c15d);
impl core::ops::Deref for IWSDOutboundAttachment {
    type Target = IWSDAttachment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDOutboundAttachment, windows_core::IUnknown, IWSDAttachment);
impl IWSDOutboundAttachment {
    pub unsafe fn Write(&self, pbuffer: &[u8]) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDOutboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWSDOutboundAttachment_Impl: IWSDAttachment_Impl {
    fn Write(&self, pbuffer: *const u8, dwbytestowrite: u32) -> windows_core::Result<u32>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
}
impl IWSDOutboundAttachment_Vtbl {
    pub const fn new<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Write<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDOutboundAttachment_Impl::Write(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwbytestowrite)) {
                    Ok(ok__) => {
                        pdwnumberofbyteswritten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDOutboundAttachment_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDOutboundAttachment_Impl::Abort(this).into()
            }
        }
        Self {
            base__: IWSDAttachment_Vtbl::new::<Identity, OFFSET>(),
            Write: Write::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDOutboundAttachment as windows_core::Interface>::IID || iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDOutboundAttachment {}
