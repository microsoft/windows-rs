windows_core::imp::define_interface!(
    IWidget,
    IWidget_Vtbl,
    0x303e6368_bcf1_5a25_8cfc_bf711e673f94
);
impl windows_core::RuntimeType for IWidget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IWidget");
}
windows_core::imp::interface_hierarchy!(
    IWidget,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidget {
    pub fn Read(&self) -> windows_core::Result<i32> {
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
impl windows_core::RuntimeName for IWidget {
    const NAME: &'static str = "Test.IWidget";
}
#[repr(C)]
pub struct IWidget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Widget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Widget,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IWidget
);
impl Widget {
    pub fn Read(&self) -> windows_core::Result<i32> {
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
impl windows_core::RuntimeType for Widget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidget>();
}
unsafe impl windows_core::Interface for Widget {
    type Vtable = <IWidget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidget as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Widget {
    const NAME: &'static str = "Test.Widget";
}
