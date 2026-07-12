windows_core::imp::define_interface!(IInner, IInner_Vtbl);
#[repr(C)]
pub struct IInner_Vtbl {
    GetDeep: usize,
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
