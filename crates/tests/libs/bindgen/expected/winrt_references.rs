windows_core::imp::define_interface!(
    References,
    References_Vtbl,
    0x5bd5266b_b3f8_5ccb_8c91_3cb9f3fdde43
);
impl windows_core::RuntimeType for References {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.References");
}
windows_core::imp::interface_hierarchy!(
    References,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl References {
    pub fn Read(&self, value: Value, result: &mut Value) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Read)(
                windows_core::Interface::as_raw(self),
                &value,
                result,
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for References {
    const NAME: &'static str = "Test.References";
}
pub trait References_Impl: windows_core::IUnknownImpl {
    fn Read(&self, value: &Value, result: &mut Value) -> windows_core::Result<()>;
}
impl References_Vtbl {
    pub const fn new<Identity: References_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: References_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: &Value,
            result: *mut Value,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                References_Impl::Read(
                    this,
                    core::mem::transmute_copy(&value),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, References, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<References as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct References_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Read: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        &Value,
        *mut Value,
    ) -> windows_core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Value {
    pub low: u32,
    pub high: i32,
}
impl windows_core::imp::TypeKind for Value {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Value {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.Value;u4;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Value");
}
