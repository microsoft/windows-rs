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
pub trait IStringAliases_Impl {
    fn get_Language(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn put_Language(&self, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn get_Ansi(&self) -> windows_core::Result<windows_core::PSTR>;
    fn put_Ansi(&self, value: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn Buffer(&self, value: *const u16) -> windows_core::Result<()>;
}
impl IStringAliases_Vtbl {
    pub const fn new<Identity: IStringAliases_Impl>() -> Self {
        unsafe extern "system" fn get_Language<Identity: IStringAliases_Impl>(
            this: *mut core::ffi::c_void,
            value: *mut windows_core::PWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match IStringAliases_Impl::get_Language(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Language<Identity: IStringAliases_Impl>(
            this: *mut core::ffi::c_void,
            value: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IStringAliases_Impl::put_Language(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_Ansi<Identity: IStringAliases_Impl>(
            this: *mut core::ffi::c_void,
            value: *mut windows_core::PSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match IStringAliases_Impl::get_Ansi(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Ansi<Identity: IStringAliases_Impl>(
            this: *mut core::ffi::c_void,
            value: windows_core::PCSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IStringAliases_Impl::put_Ansi(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Buffer<Identity: IStringAliases_Impl>(
            this: *mut core::ffi::c_void,
            value: *const u16,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IStringAliases_Impl::Buffer(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            get_Language: get_Language::<Identity>,
            put_Language: put_Language::<Identity>,
            get_Ansi: get_Ansi::<Identity>,
            put_Ansi: put_Ansi::<Identity>,
            Buffer: Buffer::<Identity>,
        }
    }
}
struct IStringAliases_ImplVtbl<T: IStringAliases_Impl>(core::marker::PhantomData<T>);
impl<T: IStringAliases_Impl> IStringAliases_ImplVtbl<T> {
    const VTABLE: IStringAliases_Vtbl = IStringAliases_Vtbl::new::<T>();
}
impl IStringAliases {
    pub fn new<'a, T: IStringAliases_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &IStringAliases_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
pub type LPCWSTR = *const u16;
