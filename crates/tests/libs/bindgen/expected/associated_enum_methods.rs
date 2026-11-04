pub mod Windows {
    pub mod Win32 {
        pub mod Api {
            windows_core::imp::define_interface!(I, I_Vtbl);
            impl I {
                pub unsafe fn Plain(&self) {
                    unsafe {
                        (windows_core::Interface::vtable(self).Plain)(
                            windows_core::Interface::as_raw(self),
                        );
                    }
                }
                pub unsafe fn Annotated(&self, value: u32) {
                    unsafe {
                        (windows_core::Interface::vtable(self).Annotated)(
                            windows_core::Interface::as_raw(self),
                            value,
                        );
                    }
                }
            }
            #[repr(C)]
            pub struct I_Vtbl {
                pub Plain: unsafe extern "system" fn(*mut core::ffi::c_void),
                pub Annotated: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
            }
            pub trait I_Impl {
                fn Plain(&self);
                fn Annotated(&self, value: u32);
            }
            impl I_Vtbl {
                pub const fn new<Identity: I_Impl>() -> Self {
                    unsafe extern "system" fn Plain<Identity: I_Impl>(
                        this: *mut core::ffi::c_void,
                    ) {
                        unsafe {
                            let this = (this as *mut *mut core::ffi::c_void)
                                as *const windows_core::ScopedHeap;
                            let this = &*((*this).this as *const Identity);
                            I_Impl::Plain(this);
                        }
                    }
                    unsafe extern "system" fn Annotated<Identity: I_Impl>(
                        this: *mut core::ffi::c_void,
                        value: u32,
                    ) {
                        unsafe {
                            let this = (this as *mut *mut core::ffi::c_void)
                                as *const windows_core::ScopedHeap;
                            let this = &*((*this).this as *const Identity);
                            I_Impl::Annotated(this, core::mem::transmute_copy(&value));
                        }
                    }
                    Self {
                        Plain: Plain::<Identity>,
                        Annotated: Annotated::<Identity>,
                    }
                }
            }
            struct I_ImplVtbl<T: I_Impl>(core::marker::PhantomData<T>);
            impl<T: I_Impl> I_ImplVtbl<T> {
                const VTABLE: I_Vtbl = I_Vtbl::new::<T>();
            }
            impl I {
                pub fn new<'a, T: I_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
                    let this = windows_core::ScopedHeap {
                        vtable: &I_ImplVtbl::<T>::VTABLE as *const _ as *const _,
                        this: this as *const _ as *const _,
                    };
                    let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
                    unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
                }
            }
        }
        pub mod Values {
            pub const One: Options = 1;
            pub type Options = u32;
        }
    }
}
