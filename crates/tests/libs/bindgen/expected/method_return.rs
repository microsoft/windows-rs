windows_core::imp::define_interface!(Interface, Interface_Vtbl);
impl Interface {
    pub unsafe fn ResultVoid(&self, value: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ResultVoid)(
                windows_core::Interface::as_raw(self),
                value,
            )
        }
    }
    pub unsafe fn ResultValue(&self, value: u32) -> windows_core::Result<Struct> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultValue)(
                windows_core::Interface::as_raw(self),
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn ReturnStruct(&self, value: u32) -> Struct {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReturnStruct)(
                windows_core::Interface::as_raw(self),
                value,
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn ReturnValue(&self, value: u32) -> Struct {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReturnValue)(
                windows_core::Interface::as_raw(self),
                value,
                &mut result__,
            );
            result__
        }
    }
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub ResultVoid: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ResultValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut Struct,
    ) -> windows_core::HRESULT,
    pub ReturnStruct: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut Struct),
    pub ReturnValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut Struct),
}
pub trait Interface_Impl {
    fn ResultVoid(&self, value: u32) -> windows_core::Result<()>;
    fn ResultValue(&self, value: u32) -> windows_core::Result<Struct>;
    fn ReturnStruct(&self, value: u32, result: *mut Struct);
    fn ReturnValue(&self, value: u32, result: *mut Struct);
}
impl Interface_Vtbl {
    pub const fn new<Identity: Interface_Impl>() -> Self {
        unsafe extern "system" fn ResultVoid<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::ResultVoid(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ResultValue<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: u32,
            result: *mut Struct,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match Interface_Impl::ResultValue(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        result.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReturnStruct<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: u32,
            result: *mut Struct,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::ReturnStruct(
                    this,
                    core::mem::transmute_copy(&value),
                    core::mem::transmute_copy(&result),
                );
            }
        }
        unsafe extern "system" fn ReturnValue<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: u32,
            result: *mut Struct,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::ReturnValue(
                    this,
                    core::mem::transmute_copy(&value),
                    core::mem::transmute_copy(&result),
                );
            }
        }
        Self {
            ResultVoid: ResultVoid::<Identity>,
            ResultValue: ResultValue::<Identity>,
            ReturnStruct: ReturnStruct::<Identity>,
            ReturnValue: ReturnValue::<Identity>,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Struct {
    pub x: i32,
    pub y: i32,
}
