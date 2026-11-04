windows_core::imp::define_interface!(
    NativeBase,
    NativeBase_Vtbl,
    0xe1570879_18fd_54b8_876b_31731b9c95ee
);
windows_core::imp::interface_hierarchy!(NativeBase, windows_core::IUnknown);
impl NativeBase {
    pub(crate) unsafe fn BaseValue(&self) -> Payload {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BaseValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
}
#[repr(C)]
pub struct NativeBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BaseValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Payload),
}
windows_core::imp::define_interface!(
    NativeDerived,
    NativeDerived_Vtbl,
    0x5bd7ac95_6d7d_5713_9169_de026bf2d89a
);
impl core::ops::Deref for NativeDerived {
    type Target = NativeBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(NativeDerived, windows_core::IUnknown, NativeBase);
impl NativeDerived {
    pub(crate) unsafe fn DerivedValue(&self) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).DerivedValue)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
}
#[repr(C)]
pub struct NativeDerived_Vtbl {
    pub base__: NativeBase_Vtbl,
    pub DerivedValue: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Payload {
    pub value: i32,
}
windows_core::imp::define_interface!(
    RuntimeBase,
    RuntimeBase_Vtbl,
    0x199fe443_608d_5dbf_b51a_e0048262a562
);
impl windows_core::RuntimeType for RuntimeBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    RuntimeBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl RuntimeBase {
    pub(crate) fn BaseValue(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BaseValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct RuntimeBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BaseValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    RuntimeDerived,
    RuntimeDerived_Vtbl,
    0x7897d8ae_6efa_599e_ad44_eb32981dcc12
);
impl windows_core::RuntimeType for RuntimeDerived {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    RuntimeDerived,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(RuntimeDerived, RuntimeBase);
impl RuntimeDerived {
    pub(crate) fn DerivedValue(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DerivedValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct RuntimeDerived_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DerivedValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(Scoped, Scoped_Vtbl);
impl Scoped {
    pub(crate) unsafe fn Value(&self) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct Scoped_Vtbl {
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
}
