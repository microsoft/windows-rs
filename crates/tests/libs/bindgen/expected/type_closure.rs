windows_core::imp::define_interface!(IInner, IInner_Vtbl);
impl IInner {
    pub unsafe fn Value(&self) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct IInner_Vtbl {
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
}
pub trait IInner_Impl {
    fn Value(&self) -> i32;
}
impl IInner_Vtbl {
    pub const fn new<Identity: IInner_Impl>() -> Self {
        unsafe extern "system" fn Value<Identity: IInner_Impl>(
            this: *mut core::ffi::c_void,
        ) -> i32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IInner_Impl::Value(this)
            }
        }
        Self {
            Value: Value::<Identity>,
        }
    }
}
struct IInner_ImplVtbl<T: IInner_Impl>(core::marker::PhantomData<T>);
impl<T: IInner_Impl> IInner_ImplVtbl<T> {
    const VTABLE: IInner_Vtbl = IInner_Vtbl::new::<T>();
}
impl IInner {
    pub fn new<'a, T: IInner_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &IInner_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IOuter, IOuter_Vtbl);
impl IOuter {
    pub unsafe fn GetInner(&self) -> Option<IInner> {
        unsafe {
            (windows_core::Interface::vtable(self).GetInner)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct IOuter_Vtbl {
    pub GetInner: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<IInner>,
}
pub trait IOuter_Impl {
    fn GetInner(&self) -> Option<IInner>;
}
impl IOuter_Vtbl {
    pub const fn new<Identity: IOuter_Impl>() -> Self {
        unsafe extern "system" fn GetInner<Identity: IOuter_Impl>(
            this: *mut core::ffi::c_void,
        ) -> Option<IInner> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IOuter_Impl::GetInner(this)
            }
        }
        Self {
            GetInner: GetInner::<Identity>,
        }
    }
}
struct IOuter_ImplVtbl<T: IOuter_Impl>(core::marker::PhantomData<T>);
impl<T: IOuter_Impl> IOuter_ImplVtbl<T> {
    const VTABLE: IOuter_Vtbl = IOuter_Vtbl::new::<T>();
}
impl IOuter {
    pub fn new<'a, T: IOuter_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &IOuter_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
