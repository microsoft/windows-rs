windows_core::imp::define_interface!(IItems, IItems_Vtbl, 0x8dfef4bd_8c6e_5d6d_9614_0aca51e6943b);
impl windows_core::RuntimeType for IItems {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IItems");
}
windows_core::imp::interface_hierarchy!(IItems, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IItems {
    const NAME: &'static str = "Test.IItems";
}
#[repr(C)]
pub struct IItems_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ISource,
    ISource_Vtbl,
    0x4b03722d_bb0f_59cf_9516_4b0e089ddf6a
);
impl windows_core::RuntimeType for ISource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.ISource");
}
windows_core::imp::interface_hierarchy!(
    ISource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ISource {
    pub fn GetItems(&self) -> windows_core::Result<IItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for ISource {
    const NAME: &'static str = "Test.ISource";
}
pub trait ISource_Impl: windows_core::IUnknownImpl {
    fn GetItems(&self) -> windows_core::Result<IItems>;
}
impl ISource_Vtbl {
    pub const fn new<Identity: ISource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItems<Identity: ISource_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISource_Impl::GetItems(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISource, OFFSET>(),
            GetItems: GetItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISource as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct ISource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
