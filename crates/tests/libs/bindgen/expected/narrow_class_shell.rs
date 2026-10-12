windows_core::imp::define_interface!(
    ISource,
    ISource_Vtbl,
    0xe6a45de3_2aba_5aca_97cd_a70913696a97
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
    pub fn GetWidget(&self) -> windows_core::Result<Widget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWidget)(
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
    fn GetWidget(&self) -> windows_core::Result<Widget>;
}
impl ISource_Vtbl {
    pub const fn new<Identity: ISource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWidget<Identity: ISource_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISource_Impl::GetWidget(this) {
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
            GetWidget: GetWidget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISource as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct ISource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetWidget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidget,
    IWidget_Vtbl,
    0x05cda87c_844a_5686_8429_f335c8ee6276
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
impl windows_core::RuntimeName for IWidget {
    const NAME: &'static str = "Test.IWidget";
}
#[repr(C)]
pub struct IWidget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
