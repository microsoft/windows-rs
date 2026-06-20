windows_core::imp::define_interface!(
    Factory,
    Factory_Vtbl,
    0x8722b2b2_1d05_5551_87c8_d46311b7873d
);
impl windows_core::RuntimeType for Factory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    Factory,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Factory {
    pub fn Create(&self) -> windows_core::Result<Widget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for Factory {
    const NAME: &'static str = "Test.Factory";
}
pub trait Factory_Impl: windows_core::IUnknownImpl {
    fn Create(&self) -> windows_core::Result<Widget>;
}
impl Factory_Vtbl {
    pub const fn new<Identity: Factory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: Factory_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Factory_Impl::Create(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, Factory, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Factory as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct Factory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidget,
    IWidget_Vtbl,
    0x62d365dd_b506_534c_a235_93129deef881
);
impl windows_core::RuntimeType for IWidget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWidget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Widget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Widget, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for Widget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidget>();
}
unsafe impl windows_core::Interface for Widget {
    type Vtable = <IWidget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidget as windows_core::Interface>::IID;
}
impl core::ops::Deref for Widget {
    type Target = IWidget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Widget {
    const NAME: &'static str = "Test.Widget";
}
