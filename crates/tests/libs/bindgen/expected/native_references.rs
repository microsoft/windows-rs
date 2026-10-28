#[inline]
pub unsafe fn Mutate(value: *mut Alias) {
    windows_core::link!("test.dll" "system" fn Mutate(value : *mut Alias));
    unsafe { Mutate(value) }
}
#[inline]
pub unsafe fn Read(value: *const Alias, result: *mut u32) {
    windows_core::link!("test.dll" "system" fn Read(value : *const Alias, result : *mut u32));
    unsafe { Read(value, result as _) }
}
#[inline]
pub unsafe fn ReadOut(value: *const Alias, result: *mut u32) {
    windows_core::link!("test.dll" "system" fn ReadOut(value : *const Alias, result : *mut u32));
    unsafe { ReadOut(value, result as _) }
}
pub type Alias = Value;
pub type Callback = Option<unsafe extern "system" fn(value: *const Alias, result: *mut u32)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Callbacks {
    pub callback: Option<unsafe extern "system" fn(*const Alias, *mut u32)>,
}
windows_core::imp::define_interface!(References, References_Vtbl);
impl References {
    pub unsafe fn Read(&self, value: *const Alias, result: *mut u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Read)(
                windows_core::Interface::as_raw(self),
                value,
                result as _,
            );
        }
    }
    pub unsafe fn ReadOut(&self, value: *const Alias, result: *mut u32) {
        unsafe {
            (windows_core::Interface::vtable(self).ReadOut)(
                windows_core::Interface::as_raw(self),
                value,
                result as _,
            );
        }
    }
}
#[repr(C)]
pub struct References_Vtbl {
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *const Alias, *mut u32),
    pub ReadOut: unsafe extern "system" fn(*mut core::ffi::c_void, *const Alias, *mut u32),
}
pub trait References_Impl {
    fn Read(&self, value: *const Alias, result: *mut u32);
    fn ReadOut(&self, value: *const Alias, result: *mut u32);
}
impl References_Vtbl {
    pub const fn new<Identity: References_Impl>() -> Self {
        unsafe extern "system" fn Read<Identity: References_Impl>(
            this: *mut core::ffi::c_void,
            value: *const Alias,
            result: *mut u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                References_Impl::Read(
                    this,
                    core::mem::transmute_copy(&value),
                    core::mem::transmute_copy(&result),
                );
            }
        }
        unsafe extern "system" fn ReadOut<Identity: References_Impl>(
            this: *mut core::ffi::c_void,
            value: *const Alias,
            result: *mut u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                References_Impl::ReadOut(
                    this,
                    core::mem::transmute_copy(&value),
                    core::mem::transmute_copy(&result),
                );
            }
        }
        Self {
            Read: Read::<Identity>,
            ReadOut: ReadOut::<Identity>,
        }
    }
}
struct References_ImplVtbl<T: References_Impl>(core::marker::PhantomData<T>);
impl<T: References_Impl> References_ImplVtbl<T> {
    const VTABLE: References_Vtbl = References_Vtbl::new::<T>();
}
impl References {
    pub fn new<'a, T: References_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &References_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Value {
    pub low: u32,
    pub high: i32,
}
