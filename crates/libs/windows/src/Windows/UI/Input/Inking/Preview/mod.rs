windows_core::imp::define_interface!(IPalmRejectionDelayZonePreview, IPalmRejectionDelayZonePreview_Vtbl, 0x62b496cb_539d_5343_a65f_41f5300ec70c);
impl windows_core::RuntimeType for IPalmRejectionDelayZonePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreview");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPalmRejectionDelayZonePreviewStatics, IPalmRejectionDelayZonePreviewStatics_Vtbl, 0xcdef5ee0_93d0_53a9_8f0e_9a379f8f7530);
impl windows_core::RuntimeType for IPalmRejectionDelayZonePreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreviewStatics");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PalmRejectionDelayZonePreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PalmRejectionDelayZonePreview, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PalmRejectionDelayZonePreview, super::super::super::super::Foundation::IClosable);
impl PalmRejectionDelayZonePreview {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    fn IPalmRejectionDelayZonePreviewStatics<R, F: FnOnce(&IPalmRejectionDelayZonePreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PalmRejectionDelayZonePreview, IPalmRejectionDelayZonePreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PalmRejectionDelayZonePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPalmRejectionDelayZonePreview>();
}
unsafe impl windows_core::Interface for PalmRejectionDelayZonePreview {
    type Vtable = <IPalmRejectionDelayZonePreview as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPalmRejectionDelayZonePreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PalmRejectionDelayZonePreview {
    const NAME: &'static str = "Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview";
}
unsafe impl Send for PalmRejectionDelayZonePreview {}
unsafe impl Sync for PalmRejectionDelayZonePreview {}
