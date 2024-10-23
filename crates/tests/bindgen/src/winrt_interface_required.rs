#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IAsyncAction,
    IAsyncAction_Vtbl,
    0x5a648006_843a_4da9_865b_9d26e5dfad7b
);
windows_core::imp::interface_hierarchy!(
    IAsyncAction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IAsyncAction, IAsyncInfo);
impl core::ops::Deref for IAsyncAction {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl IAsyncAction {
    pub fn GetResults(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
impl windows_core::RuntimeType for IAsyncAction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAsyncAction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IAsyncInfo,
    IAsyncInfo_Vtbl,
    0x00000036_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(
    IAsyncInfo,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IAsyncInfo {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl IAsyncInfo {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
impl windows_core::RuntimeType for IAsyncInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAsyncInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
