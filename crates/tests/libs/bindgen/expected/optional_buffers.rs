#[inline]
pub unsafe fn OptionalBytes(mut values: Option<&mut [u8]>) -> u32 {
    windows_core::link!("test.dll" "system" fn OptionalBytes(values : PSCHAR, count : u32) -> u32);
    unsafe {
        OptionalBytes(
            core::mem::transmute(
                values
                    .as_mut()
                    .map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()),
            ),
            values
                .as_deref()
                .map_or(0, |slice| slice.len().try_into().unwrap()),
        )
    }
}
#[inline]
pub unsafe fn OptionalElementsAfter(mut values: Option<&mut [u32]>) -> u32 {
    windows_core::link!("test.dll" "system" fn OptionalElementsAfter(values : *mut u32, count : u32) -> u32);
    unsafe {
        OptionalElementsAfter(
            values
                .as_mut()
                .map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()),
            values
                .as_deref()
                .map_or(0, |slice| slice.len().try_into().unwrap()),
        )
    }
}
#[inline]
pub unsafe fn OptionalElementsBefore(mut values: Option<&mut [u32]>) -> u32 {
    windows_core::link!("test.dll" "system" fn OptionalElementsBefore(count : u32, values : *mut u32) -> u32);
    unsafe {
        OptionalElementsBefore(
            values
                .as_deref()
                .map_or(0, |slice| slice.len().try_into().unwrap()),
            values
                .as_mut()
                .map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()),
        )
    }
}
#[inline]
pub unsafe fn OptionalFixed(mut values: Option<&mut [u32; 4]>) -> u32 {
    windows_core::link!("test.dll" "system" fn OptionalFixed(values : *mut u32) -> u32);
    unsafe {
        OptionalFixed(
            values
                .as_mut()
                .map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()),
        )
    }
}
windows_core::imp::define_interface!(Buffers, Buffers_Vtbl);
impl Buffers {
    pub unsafe fn ElementsAfter(&self, mut values: Option<&mut [u32]>) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).ElementsAfter)(
                windows_core::Interface::as_raw(self),
                values
                    .as_mut()
                    .map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()),
                values
                    .as_deref()
                    .map_or(0, |slice| slice.len().try_into().unwrap()),
            )
        }
    }
    pub unsafe fn ElementsBefore(&self, mut values: Option<&mut [u32]>) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).ElementsBefore)(
                windows_core::Interface::as_raw(self),
                values
                    .as_deref()
                    .map_or(0, |slice| slice.len().try_into().unwrap()),
                values
                    .as_mut()
                    .map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()),
            )
        }
    }
    pub unsafe fn Bytes(&self, mut values: Option<&mut [u8]>) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).Bytes)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(
                    values
                        .as_mut()
                        .map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()),
                ),
                values
                    .as_deref()
                    .map_or(0, |slice| slice.len().try_into().unwrap()),
            )
        }
    }
    pub unsafe fn Fixed(&self, mut values: Option<&mut [u32; 4]>) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).Fixed)(
                windows_core::Interface::as_raw(self),
                values
                    .as_mut()
                    .map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()),
            )
        }
    }
    pub unsafe fn Modern(&self, mut values: Option<&mut [u32]>) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).Modern)(
                windows_core::Interface::as_raw(self),
                values
                    .as_mut()
                    .map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()),
                values
                    .as_deref()
                    .map_or(0, |slice| slice.len().try_into().unwrap()),
            )
        }
    }
    pub unsafe fn Input(&self, values: Option<&[u32]>) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).Input)(
                windows_core::Interface::as_raw(self),
                values.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                values.map_or(0, |slice| slice.len().try_into().unwrap()),
            )
        }
    }
    pub unsafe fn Output(&self, values: Option<*mut u32>, count: u32) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).Output)(
                windows_core::Interface::as_raw(self),
                values.unwrap_or(core::mem::zeroed()) as _,
                count,
            )
        }
    }
    pub unsafe fn Required(&self, values: &mut [u32]) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).Required)(
                windows_core::Interface::as_raw(self),
                values.as_mut_ptr(),
                values.len().try_into().unwrap(),
            )
        }
    }
}
#[repr(C)]
pub struct Buffers_Vtbl {
    pub ElementsAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32) -> u32,
    pub ElementsBefore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> u32,
    pub Bytes: unsafe extern "system" fn(*mut core::ffi::c_void, PSCHAR, u32) -> u32,
    pub Fixed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> u32,
    pub Modern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32) -> u32,
    pub Input: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32) -> u32,
    pub Output: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32) -> u32,
    pub Required: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32) -> u32,
}
pub trait Buffers_Impl {
    fn ElementsAfter(&self, values: *mut u32, count: u32) -> u32;
    fn ElementsBefore(&self, count: u32, values: *mut u32) -> u32;
    fn Bytes(&self, values: PSCHAR, count: u32) -> u32;
    fn Fixed(&self, values: *mut u32) -> u32;
    fn Modern(&self, values: *mut u32, count: u32) -> u32;
    fn Input(&self, values: *const u32, count: u32) -> u32;
    fn Output(&self, values: *mut u32, count: u32) -> u32;
    fn Required(&self, values: *mut u32, count: u32) -> u32;
}
impl Buffers_Vtbl {
    pub const fn new<Identity: Buffers_Impl>() -> Self {
        unsafe extern "system" fn ElementsAfter<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *mut u32,
            count: u32,
        ) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::ElementsAfter(
                    this,
                    core::mem::transmute_copy(&values),
                    core::mem::transmute_copy(&count),
                )
            }
        }
        unsafe extern "system" fn ElementsBefore<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *mut u32,
        ) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::ElementsBefore(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                )
            }
        }
        unsafe extern "system" fn Bytes<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: PSCHAR,
            count: u32,
        ) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Bytes(
                    this,
                    core::mem::transmute_copy(&values),
                    core::mem::transmute_copy(&count),
                )
            }
        }
        unsafe extern "system" fn Fixed<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *mut u32,
        ) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Fixed(this, core::mem::transmute_copy(&values))
            }
        }
        unsafe extern "system" fn Modern<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *mut u32,
            count: u32,
        ) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Modern(
                    this,
                    core::mem::transmute_copy(&values),
                    core::mem::transmute_copy(&count),
                )
            }
        }
        unsafe extern "system" fn Input<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *const u32,
            count: u32,
        ) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Input(
                    this,
                    core::mem::transmute_copy(&values),
                    core::mem::transmute_copy(&count),
                )
            }
        }
        unsafe extern "system" fn Output<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *mut u32,
            count: u32,
        ) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Output(
                    this,
                    core::mem::transmute_copy(&values),
                    core::mem::transmute_copy(&count),
                )
            }
        }
        unsafe extern "system" fn Required<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *mut u32,
            count: u32,
        ) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Required(
                    this,
                    core::mem::transmute_copy(&values),
                    core::mem::transmute_copy(&count),
                )
            }
        }
        Self {
            ElementsAfter: ElementsAfter::<Identity>,
            ElementsBefore: ElementsBefore::<Identity>,
            Bytes: Bytes::<Identity>,
            Fixed: Fixed::<Identity>,
            Modern: Modern::<Identity>,
            Input: Input::<Identity>,
            Output: Output::<Identity>,
            Required: Required::<Identity>,
        }
    }
}
struct Buffers_ImplVtbl<T: Buffers_Impl>(core::marker::PhantomData<T>);
impl<T: Buffers_Impl> Buffers_ImplVtbl<T> {
    const VTABLE: Buffers_Vtbl = Buffers_Vtbl::new::<T>();
}
impl Buffers {
    pub fn new<'a, T: Buffers_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &Buffers_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
pub type PSCHAR = *mut SCHAR;
pub type SCHAR = i8;
windows_core::imp::define_interface!(Scoped, Scoped_Vtbl);
impl Scoped {
    pub unsafe fn Value(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct Scoped_Vtbl {
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait Scoped_Impl {
    fn Value(&self) -> u32;
}
impl Scoped_Vtbl {
    pub const fn new<Identity: Scoped_Impl>() -> Self {
        unsafe extern "system" fn Value<Identity: Scoped_Impl>(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Scoped_Impl::Value(this)
            }
        }
        Self {
            Value: Value::<Identity>,
        }
    }
}
struct Scoped_ImplVtbl<T: Scoped_Impl>(core::marker::PhantomData<T>);
impl<T: Scoped_Impl> Scoped_ImplVtbl<T> {
    const VTABLE: Scoped_Vtbl = Scoped_Vtbl::new::<T>();
}
impl Scoped {
    pub fn new<'a, T: Scoped_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &Scoped_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
