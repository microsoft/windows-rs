windows_core::imp::define_interface!(Interface, Interface_Vtbl);
impl Interface {
    pub unsafe fn OutFixed(&self, buffer: &mut [u32; 4]) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutFixed)(
                windows_core::Interface::as_raw(self),
                buffer.as_mut_ptr(),
            )
        }
    }
    pub unsafe fn OutLen(&self, buffer: &mut [u32]) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutLen)(
                windows_core::Interface::as_raw(self),
                buffer.len().try_into().unwrap(),
                buffer.as_mut_ptr(),
            )
        }
    }
    pub unsafe fn OutOpt(&self, buffer: Option<&mut [u32]>) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutOpt)(
                windows_core::Interface::as_raw(self),
                buffer
                    .as_deref()
                    .map_or(0, |slice| slice.len().try_into().unwrap()),
                buffer
                    .as_deref()
                    .map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
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
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub OutFixed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> i32,
    pub OutLen: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> i32,
    pub OutOpt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> i32,
    pub InLen: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> i32,
}
pub trait Interface_Impl {
    fn OutFixed(&self, buffer: *mut u32) -> i32;
    fn OutLen(&self, count: u32, buffer: *mut u32) -> i32;
    fn OutOpt(&self, count: u32, buffer: *mut u32) -> i32;
    fn InLen(&self, count: u32, buffer: *const u32) -> i32;
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
        Self {
            OutFixed: OutFixed::<Identity>,
            OutLen: OutLen::<Identity>,
            OutOpt: OutOpt::<Identity>,
            InLen: InLen::<Identity>,
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
