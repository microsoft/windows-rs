windows_core::imp::define_interface!(IStringAliases, IStringAliases_Vtbl);
impl IStringAliases {
    pub unsafe fn get_Language(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Language)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn put_Language<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Language)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
        }
    }
    pub unsafe fn get_Ansi(&self) -> windows_core::Result<windows_core::PSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Ansi)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn put_Ansi<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Ansi)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
        }
    }
    pub unsafe fn Buffer(&self, value: *const u16) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Buffer)(
                windows_core::Interface::as_raw(self),
                value,
            )
        }
    }
}
#[repr(C)]
pub struct IStringAliases_Vtbl {
    pub get_Language: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
    pub put_Language: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub get_Ansi: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::PSTR,
    ) -> windows_core::HRESULT,
    pub put_Ansi: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCSTR,
    ) -> windows_core::HRESULT,
    pub Buffer:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
}
pub type LPCWSTR = *const u16;
