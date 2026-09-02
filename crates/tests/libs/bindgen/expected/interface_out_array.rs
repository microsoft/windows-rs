windows_core::imp::define_interface!(Interface, Interface_Vtbl);
impl Interface {
    pub unsafe fn OutFixed(&self, buffer: *mut u32) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutFixed)(
                windows_core::Interface::as_raw(self),
                buffer as _,
            )
        }
    }
    pub unsafe fn OutLen(&self, count: u32, buffer: *mut u32) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutLen)(
                windows_core::Interface::as_raw(self),
                count,
                buffer as _,
            )
        }
    }
    pub unsafe fn OutOpt(&self, count: u32, buffer: Option<*mut u32>) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutOpt)(
                windows_core::Interface::as_raw(self),
                count,
                buffer.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn OutBytes(&self, size: u32, buffer: *mut u8) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutBytes)(
                windows_core::Interface::as_raw(self),
                size,
                buffer as _,
            )
        }
    }
    pub unsafe fn OutInterfaces(
        &self,
        count: u32,
        values: *mut Option<windows_core::IInspectable>,
    ) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutInterfaces)(
                windows_core::Interface::as_raw(self),
                count,
                core::mem::transmute(values),
            )
        }
    }
    pub unsafe fn InOutLen(&self, buffer: &mut [u32]) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).InOutLen)(
                windows_core::Interface::as_raw(self),
                buffer.len().try_into().unwrap(),
                buffer.as_mut_ptr(),
            )
        }
    }
    pub unsafe fn SharedCount(&self, count: u32, input: *const u32, output: *mut u32) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).SharedCount)(
                windows_core::Interface::as_raw(self),
                count,
                input,
                output as _,
            )
        }
    }
    pub unsafe fn InLen(&self, buffer: &[u32]) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).InLen)(
                windows_core::Interface::as_raw(self),
                buffer.len().try_into().unwrap(),
                buffer.as_ptr(),
            )
        }
    }
    pub unsafe fn InSignedLen(&self, count: i32, buffer: Option<*const u32>) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).InSignedLen)(
                windows_core::Interface::as_raw(self),
                count,
                buffer.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub OutFixed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> i32,
    pub OutLen: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> i32,
    pub OutOpt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> i32,
    pub OutBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> i32,
    pub OutInterfaces:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> i32,
    pub InOutLen: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> i32,
    pub SharedCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *mut u32) -> i32,
    pub InLen: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> i32,
    pub InSignedLen: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u32) -> i32,
}
pub trait Interface_Impl {
    fn OutFixed(&self, buffer: *mut u32) -> i32;
    fn OutLen(&self, count: u32, buffer: *mut u32) -> i32;
    fn OutOpt(&self, count: u32, buffer: *mut u32) -> i32;
    fn OutBytes(&self, size: u32, buffer: *mut u8) -> i32;
    fn OutInterfaces(&self, count: u32, values: *mut Option<windows_core::IInspectable>) -> i32;
    fn InOutLen(&self, count: u32, buffer: *mut u32) -> i32;
    fn SharedCount(&self, count: u32, input: *const u32, output: *mut u32) -> i32;
    fn InLen(&self, count: u32, buffer: *const u32) -> i32;
    fn InSignedLen(&self, count: i32, buffer: *const u32) -> i32;
}
impl Interface_Vtbl {
    pub const fn new<Identity: Interface_Impl>() -> Self {
        unsafe extern "system" fn OutFixed<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            buffer: *mut u32,
        ) -> i32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::OutFixed(this, core::mem::transmute_copy(&buffer))
            }
        }
        unsafe extern "system" fn OutLen<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            buffer: *mut u32,
        ) -> i32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::OutLen(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&buffer),
                )
            }
        }
        unsafe extern "system" fn OutOpt<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            buffer: *mut u32,
        ) -> i32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::OutOpt(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&buffer),
                )
            }
        }
        unsafe extern "system" fn OutBytes<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            size: u32,
            buffer: *mut u8,
        ) -> i32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::OutBytes(
                    this,
                    core::mem::transmute_copy(&size),
                    core::mem::transmute_copy(&buffer),
                )
            }
        }
        unsafe extern "system" fn OutInterfaces<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *mut *mut core::ffi::c_void,
        ) -> i32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::OutInterfaces(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                )
            }
        }
        unsafe extern "system" fn InOutLen<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            buffer: *mut u32,
        ) -> i32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::InOutLen(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&buffer),
                )
            }
        }
        unsafe extern "system" fn SharedCount<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            input: *const u32,
            output: *mut u32,
        ) -> i32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::SharedCount(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&input),
                    core::mem::transmute_copy(&output),
                )
            }
        }
        unsafe extern "system" fn InLen<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            buffer: *const u32,
        ) -> i32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::InLen(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&buffer),
                )
            }
        }
        unsafe extern "system" fn InSignedLen<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: i32,
            buffer: *const u32,
        ) -> i32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::InSignedLen(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&buffer),
                )
            }
        }
        Self {
            OutFixed: OutFixed::<Identity>,
            OutLen: OutLen::<Identity>,
            OutOpt: OutOpt::<Identity>,
            OutBytes: OutBytes::<Identity>,
            OutInterfaces: OutInterfaces::<Identity>,
            InOutLen: InOutLen::<Identity>,
            SharedCount: SharedCount::<Identity>,
            InLen: InLen::<Identity>,
            InSignedLen: InSignedLen::<Identity>,
        }
    }
}
struct Interface_ImplVtbl<T: Interface_Impl>(core::marker::PhantomData<T>);
impl<T: Interface_Impl> Interface_ImplVtbl<T> {
    const VTABLE: Interface_Vtbl = Interface_Vtbl::new::<T>();
}
impl Interface {
    pub fn new<'a, T: Interface_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &Interface_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(Object, Object_Vtbl);
#[repr(C)]
pub struct Object_Vtbl {}
pub trait Object_Impl {}
impl Object_Vtbl {
    pub const fn new<Identity: Object_Impl>() -> Self {
        Self {}
    }
}
struct Object_ImplVtbl<T: Object_Impl>(core::marker::PhantomData<T>);
impl<T: Object_Impl> Object_ImplVtbl<T> {
    const VTABLE: Object_Vtbl = Object_Vtbl::new::<T>();
}
impl Object {
    pub fn new<'a, T: Object_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &Object_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
