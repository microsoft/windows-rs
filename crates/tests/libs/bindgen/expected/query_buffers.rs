windows_core::imp::define_interface!(Factory, Factory_Vtbl);
impl Factory {
    pub unsafe fn Reference<T>(&self, filters: &[windows_core::GUID]) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).Reference)(
                windows_core::Interface::as_raw(self),
                filters.len().try_into().unwrap(),
                filters.as_ptr(),
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn BufferAfterId<T>(&self, filters: &[windows_core::GUID]) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).BufferAfterId)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                filters.len().try_into().unwrap(),
                filters.as_ptr(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn BufferOnly(
        &self,
        filters: &[windows_core::GUID],
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).BufferOnly)(
                windows_core::Interface::as_raw(self),
                filters.len().try_into().unwrap(),
                filters.as_ptr(),
                result as _,
            )
        }
    }
    pub unsafe fn ByteBuffer(
        &self,
        count: u32,
        filters: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ByteBuffer)(
                windows_core::Interface::as_raw(self),
                count,
                filters,
                result as _,
            )
        }
    }
    pub unsafe fn FixedBuffer(
        &self,
        filters: &[windows_core::GUID; 1],
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).FixedBuffer)(
                windows_core::Interface::as_raw(self),
                filters.as_ptr(),
                result as _,
            )
        }
    }
    pub unsafe fn OutputBuffer(
        &self,
        riid: *const windows_core::GUID,
        count: u32,
        results: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).OutputBuffer)(
                windows_core::Interface::as_raw(self),
                riid,
                count,
                results as _,
            )
        }
    }
}
#[repr(C)]
pub struct Factory_Vtbl {
    pub Reference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        *const IID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub BufferAfterId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub BufferOnly: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ByteBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FixedBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OutputBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait Factory_Impl {
    fn Reference(
        &self,
        count: u32,
        filters: *const windows_core::GUID,
        riid: *const IID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn BufferAfterId(
        &self,
        riid: *const windows_core::GUID,
        count: u32,
        filters: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn BufferOnly(
        &self,
        count: u32,
        filters: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn ByteBuffer(
        &self,
        count: u32,
        filters: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn FixedBuffer(
        &self,
        filters: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn OutputBuffer(
        &self,
        riid: *const windows_core::GUID,
        count: u32,
        results: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
}
impl Factory_Vtbl {
    pub const fn new<Identity: Factory_Impl>() -> Self {
        unsafe extern "system" fn Reference<Identity: Factory_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            filters: *const windows_core::GUID,
            riid: *const IID,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Factory_Impl::Reference(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&filters),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        unsafe extern "system" fn BufferAfterId<Identity: Factory_Impl>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            count: u32,
            filters: *const windows_core::GUID,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Factory_Impl::BufferAfterId(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&filters),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        unsafe extern "system" fn BufferOnly<Identity: Factory_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            filters: *const windows_core::GUID,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Factory_Impl::BufferOnly(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&filters),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        unsafe extern "system" fn ByteBuffer<Identity: Factory_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            filters: *const windows_core::GUID,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Factory_Impl::ByteBuffer(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&filters),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        unsafe extern "system" fn FixedBuffer<Identity: Factory_Impl>(
            this: *mut core::ffi::c_void,
            filters: *const windows_core::GUID,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Factory_Impl::FixedBuffer(
                    this,
                    core::mem::transmute_copy(&filters),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        unsafe extern "system" fn OutputBuffer<Identity: Factory_Impl>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            count: u32,
            results: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Factory_Impl::OutputBuffer(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&results),
                )
                .into()
            }
        }
        Self {
            Reference: Reference::<Identity>,
            BufferAfterId: BufferAfterId::<Identity>,
            BufferOnly: BufferOnly::<Identity>,
            ByteBuffer: ByteBuffer::<Identity>,
            FixedBuffer: FixedBuffer::<Identity>,
            OutputBuffer: OutputBuffer::<Identity>,
        }
    }
}
struct Factory_ImplVtbl<T: Factory_Impl>(core::marker::PhantomData<T>);
impl<T: Factory_Impl> Factory_ImplVtbl<T> {
    const VTABLE: Factory_Vtbl = Factory_Vtbl::new::<T>();
}
impl Factory {
    pub fn new<'a, T: Factory_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &Factory_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub type IID = windows_core::GUID;
