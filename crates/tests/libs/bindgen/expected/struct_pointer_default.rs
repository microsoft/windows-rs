pub type Alias = Pointer;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstHandle(pub *const core::ffi::c_void);
impl Default for ConstHandle {
    fn default() -> Self {
        Self(core::ptr::null())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HandleDefault {
    pub mutable: MutableHandle,
    pub constant: ConstHandle,
}
windows_core::imp::define_interface!(IObject, IObject_Vtbl);
impl IObject {
    pub unsafe fn Method(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Method)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
pub struct IObject_Vtbl {
    pub Method: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IObject_Impl {
    fn Method(&self);
}
impl IObject_Vtbl {
    pub const fn new<Identity: IObject_Impl>() -> Self {
        unsafe extern "system" fn Method<Identity: IObject_Impl>(this: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IObject_Impl::Method(this);
            }
        }
        Self {
            Method: Method::<Identity>,
        }
    }
}
struct IObject_ImplVtbl<T: IObject_Impl>(core::marker::PhantomData<T>);
impl<T: IObject_Impl> IObject_ImplVtbl<T> {
    const VTABLE: IObject_Vtbl = IObject_Vtbl::new::<T>();
}
impl IObject {
    pub fn new<'a, T: IObject_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &IObject_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe {
            windows_core::ScopedInterface::new(<Self as windows_core::Interface>::from_raw(
                this as *mut _,
            ))
        }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InterfaceDefault {
    pub object: core::mem::ManuallyDrop<Option<IObject>>,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutableHandle(pub *mut core::ffi::c_void);
impl Default for MutableHandle {
    fn default() -> Self {
        Self(core::ptr::null_mut())
    }
}
pub type Pointer = *mut u8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PointerDefault {
    pub raw: *mut core::ffi::c_void,
    pub alias: Alias,
    pub constant: *const core::ffi::c_void,
}
impl Default for PointerDefault {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
