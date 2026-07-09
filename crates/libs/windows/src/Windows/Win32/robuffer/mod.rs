#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn RoGetBufferMarshaler() -> windows_core::Result<super::objidlbase::IMarshal> {
    windows_core::link!("api-ms-win-core-winrt-robuffer-l1-1-0.dll" "system" fn RoGetBufferMarshaler(buffermarshaler : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetBufferMarshaler(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(IBufferByteAccess, IBufferByteAccess_Vtbl, 0x905a0fef_bc53_11df_8c49_001e4fc686da);
windows_core::imp::interface_hierarchy!(IBufferByteAccess, windows_core::IUnknown);
impl IBufferByteAccess {
    #[cfg(feature = "Win32_rpcndr")]
    pub unsafe fn Buffer(&self) -> windows_core::Result<*mut super::rpcndr::byte> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Buffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferByteAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_rpcndr")]
    pub Buffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::rpcndr::byte) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpcndr"))]
    Buffer: usize,
}
#[cfg(feature = "Win32_rpcndr")]
pub trait IBufferByteAccess_Impl: windows_core::IUnknownImpl {
    fn Buffer(&self) -> windows_core::Result<*mut super::rpcndr::byte>;
}
#[cfg(feature = "Win32_rpcndr")]
impl IBufferByteAccess_Vtbl {
    pub const fn new<Identity: IBufferByteAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Buffer<Identity: IBufferByteAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::rpcndr::byte) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBufferByteAccess_Impl::Buffer(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Buffer: Buffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBufferByteAccess as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_rpcndr")]
impl windows_core::RuntimeName for IBufferByteAccess {}
