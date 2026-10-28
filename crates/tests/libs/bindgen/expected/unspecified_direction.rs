pub type Pointer = *mut u32;
windows_core::imp::define_interface!(Unknown, Unknown_Vtbl);
impl Unknown {
    pub unsafe fn Scalar(&self, value: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Scalar)(
                windows_core::Interface::as_raw(self),
                value,
            )
        }
    }
    pub unsafe fn Raw(&self, value: *mut u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Raw)(
                windows_core::Interface::as_raw(self),
                value as _,
            )
        }
    }
    pub unsafe fn Alias(&self, value: Pointer) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Alias)(
                windows_core::Interface::as_raw(self),
                value as _,
            )
        }
    }
    pub unsafe fn Buffer(&self, count: u32, value: *mut u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Buffer)(
                windows_core::Interface::as_raw(self),
                count,
                value as _,
            )
        }
    }
    pub unsafe fn Query(
        &self,
        riid: *const windows_core::GUID,
        value: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Query)(
                windows_core::Interface::as_raw(self),
                riid,
                value as _,
            )
        }
    }
    pub unsafe fn Interface(&self, value: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Interface)(
                windows_core::Interface::as_raw(self),
                value as _,
            )
        }
    }
    pub unsafe fn Input(&self, value: *const u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Input)(
                windows_core::Interface::as_raw(self),
                value,
            )
        }
    }
    pub unsafe fn Output(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Output)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct Unknown_Vtbl {
    pub Scalar: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Raw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Alias: unsafe extern "system" fn(*mut core::ffi::c_void, Pointer) -> windows_core::HRESULT,
    pub Buffer:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Interface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Input:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub Output:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait Unknown_Impl {
    fn Scalar(&self, value: u32) -> windows_core::Result<()>;
    fn Raw(&self, value: *mut u32) -> windows_core::Result<()>;
    fn Alias(&self, value: Pointer) -> windows_core::Result<()>;
    fn Buffer(&self, count: u32, value: *mut u32) -> windows_core::Result<()>;
    fn Query(
        &self,
        riid: *const windows_core::GUID,
        value: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn Interface(&self, value: *mut Option<Unknown>) -> windows_core::Result<()>;
    fn Input(&self, value: *const u32) -> windows_core::Result<()>;
    fn Output(&self) -> windows_core::Result<u32>;
}
impl Unknown_Vtbl {
    pub const fn new<Identity: Unknown_Impl>() -> Self {
        unsafe extern "system" fn Scalar<Identity: Unknown_Impl>(
            this: *mut core::ffi::c_void,
            value: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Unknown_Impl::Scalar(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Raw<Identity: Unknown_Impl>(
            this: *mut core::ffi::c_void,
            value: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Unknown_Impl::Raw(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Alias<Identity: Unknown_Impl>(
            this: *mut core::ffi::c_void,
            value: Pointer,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Unknown_Impl::Alias(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Buffer<Identity: Unknown_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            value: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Unknown_Impl::Buffer(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&value),
                )
                .into()
            }
        }
        unsafe extern "system" fn Query<Identity: Unknown_Impl>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Unknown_Impl::Query(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&value),
                )
                .into()
            }
        }
        unsafe extern "system" fn Interface<Identity: Unknown_Impl>(
            this: *mut core::ffi::c_void,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Unknown_Impl::Interface(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Input<Identity: Unknown_Impl>(
            this: *mut core::ffi::c_void,
            value: *const u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Unknown_Impl::Input(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Output<Identity: Unknown_Impl>(
            this: *mut core::ffi::c_void,
            value: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match Unknown_Impl::Output(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            Scalar: Scalar::<Identity>,
            Raw: Raw::<Identity>,
            Alias: Alias::<Identity>,
            Buffer: Buffer::<Identity>,
            Query: Query::<Identity>,
            Interface: Interface::<Identity>,
            Input: Input::<Identity>,
            Output: Output::<Identity>,
        }
    }
}
struct Unknown_ImplVtbl<T: Unknown_Impl>(core::marker::PhantomData<T>);
impl<T: Unknown_Impl> Unknown_ImplVtbl<T> {
    const VTABLE: Unknown_Vtbl = Unknown_Vtbl::new::<T>();
}
impl Unknown {
    pub fn new<'a, T: Unknown_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &Unknown_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
