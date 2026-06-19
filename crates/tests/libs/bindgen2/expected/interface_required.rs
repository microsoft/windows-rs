windows_core::imp::define_interface!(IBase, IBase_Vtbl);
impl IBase {
    pub unsafe fn BaseMethod(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).BaseMethod)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
}
#[repr(C)]
pub struct IBase_Vtbl {
    pub BaseMethod: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBase_Impl {
    fn BaseMethod(&self) -> windows_core::Result<()>;
}
impl IBase_Vtbl {
    pub const fn new<Identity: IBase_Impl>() -> Self {
        unsafe extern "system" fn BaseMethod<Identity: IBase_Impl>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IBase_Impl::BaseMethod(this).into()
            }
        }
        Self {
            BaseMethod: BaseMethod::<Identity>,
        }
    }
}
struct IBase_ImplVtbl<T: IBase_Impl>(core::marker::PhantomData<T>);
impl<T: IBase_Impl> IBase_ImplVtbl<T> {
    const VTABLE: IBase_Vtbl = IBase_Vtbl::new::<T>();
}
impl IBase {
    pub fn new<'a, T: IBase_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &IBase_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IInterface, IInterface_Vtbl);
impl core::ops::Deref for IInterface {
    type Target = IBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInterface, IBase);
impl IInterface {
    pub unsafe fn Method(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Method)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct IInterface_Vtbl {
    pub base__: IBase_Vtbl,
    pub Method: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInterface_Impl: IBase_Impl {
    fn Method(&self) -> windows_core::Result<()>;
}
impl IInterface_Vtbl {
    pub const fn new<Identity: IInterface_Impl>() -> Self {
        unsafe extern "system" fn Method<Identity: IInterface_Impl>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IInterface_Impl::Method(this).into()
            }
        }
        Self {
            base__: IBase_Vtbl::new::<Identity>(),
            Method: Method::<Identity>,
        }
    }
}
struct IInterface_ImplVtbl<T: IInterface_Impl>(core::marker::PhantomData<T>);
impl<T: IInterface_Impl> IInterface_ImplVtbl<T> {
    const VTABLE: IInterface_Vtbl = IInterface_Vtbl::new::<T>();
}
impl IInterface {
    pub fn new<'a, T: IInterface_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &IInterface_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
