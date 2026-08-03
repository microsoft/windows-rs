#[inline]
pub unsafe fn InvalidFunction(count: u32, values: *const u32) {
    windows_core::link!("test.dll" "system" fn InvalidFunction(count : u32, values : *const u32));
    unsafe { InvalidFunction(count, values) }
}
#[inline]
pub unsafe fn ValidFunction(values: &[u32]) {
    windows_core::link!("test.dll" "system" fn ValidFunction(count : u32, values : *const u32));
    unsafe { ValidFunction(values.len().try_into().unwrap(), values.as_ptr()) }
}
windows_core::imp::define_interface!(Interface, Interface_Vtbl);
impl Interface {
    pub unsafe fn ValidElements(&self, values: &[u32]) {
        unsafe {
            (windows_core::Interface::vtable(self).ValidElements)(
                windows_core::Interface::as_raw(self),
                values.len().try_into().unwrap(),
                values.as_ptr(),
            );
        }
    }
    pub unsafe fn NegativeElements(&self, count: u32, values: *const u32) {
        unsafe {
            (windows_core::Interface::vtable(self).NegativeElements)(
                windows_core::Interface::as_raw(self),
                count,
                values,
            );
        }
    }
    pub unsafe fn OutOfRangeElements(&self, count: u32, values: *const u32) {
        unsafe {
            (windows_core::Interface::vtable(self).OutOfRangeElements)(
                windows_core::Interface::as_raw(self),
                count,
                values,
            );
        }
    }
    pub unsafe fn SelfElements(&self, values: *const u32) {
        unsafe {
            (windows_core::Interface::vtable(self).SelfElements)(
                windows_core::Interface::as_raw(self),
                values,
            );
        }
    }
    pub unsafe fn SharedCount(&self, count: u32, left: *const u32, right: *const u32) {
        unsafe {
            (windows_core::Interface::vtable(self).SharedCount)(
                windows_core::Interface::as_raw(self),
                count,
                left,
                right,
            );
        }
    }
    pub unsafe fn ValidBytes(&self, values: &[u8]) {
        unsafe {
            (windows_core::Interface::vtable(self).ValidBytes)(
                windows_core::Interface::as_raw(self),
                values.len().try_into().unwrap(),
                values.as_ptr(),
            );
        }
    }
    pub unsafe fn NegativeBytes(&self, count: u32, values: *const u8) {
        unsafe {
            (windows_core::Interface::vtable(self).NegativeBytes)(
                windows_core::Interface::as_raw(self),
                count,
                values,
            );
        }
    }
    pub unsafe fn OutOfRangeBytes(&self, count: u32, values: *const u8) {
        unsafe {
            (windows_core::Interface::vtable(self).OutOfRangeBytes)(
                windows_core::Interface::as_raw(self),
                count,
                values,
            );
        }
    }
    pub unsafe fn SelfBytes(&self, values: *const u8) {
        unsafe {
            (windows_core::Interface::vtable(self).SelfBytes)(
                windows_core::Interface::as_raw(self),
                values,
            );
        }
    }
    pub unsafe fn ValidConstant(&self, values: &[u32; 4]) {
        unsafe {
            (windows_core::Interface::vtable(self).ValidConstant)(
                windows_core::Interface::as_raw(self),
                values.as_ptr(),
            );
        }
    }
    pub unsafe fn NegativeConstant(&self, values: *const u32) {
        unsafe {
            (windows_core::Interface::vtable(self).NegativeConstant)(
                windows_core::Interface::as_raw(self),
                values,
            );
        }
    }
    pub unsafe fn OutOfRangeConstant(&self, values: *const u32) {
        unsafe {
            (windows_core::Interface::vtable(self).OutOfRangeConstant)(
                windows_core::Interface::as_raw(self),
                values,
            );
        }
    }
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub ValidElements: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32),
    pub NegativeElements: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32),
    pub OutOfRangeElements: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32),
    pub SelfElements: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32),
    pub SharedCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *const u32),
    pub ValidBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8),
    pub NegativeBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8),
    pub OutOfRangeBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8),
    pub SelfBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8),
    pub ValidConstant: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32),
    pub NegativeConstant: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32),
    pub OutOfRangeConstant: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32),
}
pub trait Interface_Impl {
    fn ValidElements(&self, count: u32, values: *const u32);
    fn NegativeElements(&self, count: u32, values: *const u32);
    fn OutOfRangeElements(&self, count: u32, values: *const u32);
    fn SelfElements(&self, values: *const u32);
    fn SharedCount(&self, count: u32, left: *const u32, right: *const u32);
    fn ValidBytes(&self, count: u32, values: *const u8);
    fn NegativeBytes(&self, count: u32, values: *const u8);
    fn OutOfRangeBytes(&self, count: u32, values: *const u8);
    fn SelfBytes(&self, values: *const u8);
    fn ValidConstant(&self, values: *const u32);
    fn NegativeConstant(&self, values: *const u32);
    fn OutOfRangeConstant(&self, values: *const u32);
}
impl Interface_Vtbl {
    pub const fn new<Identity: Interface_Impl>() -> Self {
        unsafe extern "system" fn ValidElements<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::ValidElements(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn NegativeElements<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::NegativeElements(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn OutOfRangeElements<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::OutOfRangeElements(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn SelfElements<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::SelfElements(this, core::mem::transmute_copy(&values));
            }
        }
        unsafe extern "system" fn SharedCount<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            left: *const u32,
            right: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::SharedCount(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&left),
                    core::mem::transmute_copy(&right),
                );
            }
        }
        unsafe extern "system" fn ValidBytes<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *const u8,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::ValidBytes(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn NegativeBytes<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *const u8,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::NegativeBytes(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn OutOfRangeBytes<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *const u8,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::OutOfRangeBytes(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn SelfBytes<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            values: *const u8,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::SelfBytes(this, core::mem::transmute_copy(&values));
            }
        }
        unsafe extern "system" fn ValidConstant<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::ValidConstant(this, core::mem::transmute_copy(&values));
            }
        }
        unsafe extern "system" fn NegativeConstant<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::NegativeConstant(this, core::mem::transmute_copy(&values));
            }
        }
        unsafe extern "system" fn OutOfRangeConstant<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::OutOfRangeConstant(this, core::mem::transmute_copy(&values));
            }
        }
        Self {
            ValidElements: ValidElements::<Identity>,
            NegativeElements: NegativeElements::<Identity>,
            OutOfRangeElements: OutOfRangeElements::<Identity>,
            SelfElements: SelfElements::<Identity>,
            SharedCount: SharedCount::<Identity>,
            ValidBytes: ValidBytes::<Identity>,
            NegativeBytes: NegativeBytes::<Identity>,
            OutOfRangeBytes: OutOfRangeBytes::<Identity>,
            SelfBytes: SelfBytes::<Identity>,
            ValidConstant: ValidConstant::<Identity>,
            NegativeConstant: NegativeConstant::<Identity>,
            OutOfRangeConstant: OutOfRangeConstant::<Identity>,
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
