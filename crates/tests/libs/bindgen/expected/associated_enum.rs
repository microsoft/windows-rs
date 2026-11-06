#[inline]
pub unsafe fn Duplicate(options: u32) {
    windows_core::link!("fixture.dll" "C" fn Duplicate(options : u32));
    unsafe { Duplicate(options) }
}
#[inline]
pub unsafe fn GetFamily() -> i32 {
    windows_core::link!("fixture.dll" "C" fn GetFamily() -> i32);
    unsafe { GetFamily() }
}
#[inline]
pub unsafe fn Socket(family: i32) {
    windows_core::link!("fixture.dll" "C" fn Socket(family : i32));
    unsafe { Socket(family) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Arguments {
    pub family: u16,
}
pub type Callback = Option<unsafe extern "C" fn(options: u32)>;
pub const CloseSource: Options = 1;
pub type Family = u16;
pub const HighBit: Options = 2147483648;
windows_core::imp::define_interface!(IOptions, IOptions_Vtbl);
impl IOptions {
    pub unsafe fn Apply(&self, options: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Apply)(
                windows_core::Interface::as_raw(self),
                options,
            );
        }
    }
}
#[repr(C)]
pub struct IOptions_Vtbl {
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
pub trait IOptions_Impl {
    fn Apply(&self, options: u32);
}
impl IOptions_Vtbl {
    pub const fn new<Identity: IOptions_Impl>() -> Self {
        unsafe extern "system" fn Apply<Identity: IOptions_Impl>(
            this: *mut core::ffi::c_void,
            options: u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IOptions_Impl::Apply(this, core::mem::transmute_copy(&options));
            }
        }
        Self {
            Apply: Apply::<Identity>,
        }
    }
}
struct IOptions_ImplVtbl<T: IOptions_Impl>(core::marker::PhantomData<T>);
impl<T: IOptions_Impl> IOptions_ImplVtbl<T> {
    const VTABLE: IOptions_Vtbl = IOptions_Vtbl::new::<T>();
}
impl IOptions {
    pub fn new<'a, T: IOptions_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &IOptions_ImplVtbl::<T>::VTABLE as *const _ as *const _,
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
pub const Inet: Family = 2;
pub const Negative: Signed = -100;
pub type Options = u32;
pub const SameAccess: Options = 2;
pub type Signed = i16;
