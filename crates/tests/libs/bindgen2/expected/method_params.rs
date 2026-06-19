#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct BOOL(pub i32);
windows_core::imp::define_interface!(Interface, Interface_Vtbl);
impl Interface {
    pub unsafe fn IntoParam(&self, value: *const Struct) {
        unsafe {
            (windows_core::Interface::vtable(self).IntoParam)(
                windows_core::Interface::as_raw(self),
                value,
            );
        }
    }
    pub unsafe fn Optional(&self, value: Option<*const Struct>) {
        unsafe {
            (windows_core::Interface::vtable(self).Optional)(
                windows_core::Interface::as_raw(self),
                value.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn BoolParam(&self, value: BOOL) {
        unsafe {
            (windows_core::Interface::vtable(self).BoolParam)(
                windows_core::Interface::as_raw(self),
                value,
            );
        }
    }
    pub unsafe fn ValueType(&self, value: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).ValueType)(
                windows_core::Interface::as_raw(self),
                value,
            );
        }
    }
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub IntoParam: unsafe extern "system" fn(*mut core::ffi::c_void, *const Struct),
    pub Optional: unsafe extern "system" fn(*mut core::ffi::c_void, *const Struct),
    pub BoolParam: unsafe extern "system" fn(*mut core::ffi::c_void, BOOL),
    pub ValueType: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
pub trait Interface_Impl {
    fn IntoParam(&self, value: *const Struct);
    fn Optional(&self, value: *const Struct);
    fn BoolParam(&self, value: BOOL);
    fn ValueType(&self, value: u32);
}
impl Interface_Vtbl {
    pub const fn new<Identity: Interface_Impl>() -> Self {
        unsafe extern "system" fn IntoParam<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: *const Struct,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::IntoParam(this, core::mem::transmute_copy(&value));
            }
        }
        unsafe extern "system" fn Optional<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: *const Struct,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::Optional(this, core::mem::transmute_copy(&value));
            }
        }
        unsafe extern "system" fn BoolParam<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: BOOL,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::BoolParam(this, core::mem::transmute_copy(&value));
            }
        }
        unsafe extern "system" fn ValueType<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::ValueType(this, core::mem::transmute_copy(&value));
            }
        }
        Self {
            IntoParam: IntoParam::<Identity>,
            Optional: Optional::<Identity>,
            BoolParam: BoolParam::<Identity>,
            ValueType: ValueType::<Identity>,
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Struct {
    pub x: i32,
    pub y: i32,
}
