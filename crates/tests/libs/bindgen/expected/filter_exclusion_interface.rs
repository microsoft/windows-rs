windows_core::imp::define_interface!(IKept, IKept_Vtbl, 0x694faf18_5fa9_5673_9e69_deec743d0692);
impl windows_core::RuntimeType for IKept {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IKept");
}
windows_core::imp::interface_hierarchy!(IKept, windows_core::IUnknown, windows_core::IInspectable);
impl IKept {
    pub fn Read(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Read)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IKept {
    const NAME: &'static str = "Test.IKept";
}
pub trait IKept_Impl: windows_core::IUnknownImpl {
    fn Read(&self) -> windows_core::Result<u32>;
}
impl IKept_Vtbl {
    pub const fn new<Identity: IKept_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IKept_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKept_Impl::Read(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IKept, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKept as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IKept_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
