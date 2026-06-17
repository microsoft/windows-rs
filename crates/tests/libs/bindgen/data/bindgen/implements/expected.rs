pub mod Test {
    windows_core::imp::define_interface!(IBar, IBar_Vtbl);
    impl IBar {
        pub unsafe fn Bar(&self) -> i32 {
            unsafe {
                (windows_core::Interface::vtable(self).Bar)(windows_core::Interface::as_raw(self))
            }
        }
    }
    #[repr(C)]
    pub struct IBar_Vtbl {
        pub Bar: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    }
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl);
    impl IFoo {
        pub unsafe fn Foo(&self) -> i32 {
            unsafe {
                (windows_core::Interface::vtable(self).Foo)(windows_core::Interface::as_raw(self))
            }
        }
    }
    #[repr(C)]
    pub struct IFoo_Vtbl {
        pub Foo: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    }
    pub trait IFoo_Impl {
        fn Foo(&self) -> i32;
    }
    impl IFoo_Vtbl {
        pub const fn new<Identity: IFoo_Impl>() -> Self {
            unsafe extern "system" fn Foo<Identity: IFoo_Impl>(
                this: *mut core::ffi::c_void,
            ) -> i32 {
                unsafe {
                    let this =
                        (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                    let this = &*((*this).this as *const Identity);
                    IFoo_Impl::Foo(this)
                }
            }
            Self {
                Foo: Foo::<Identity>,
            }
        }
    }
    struct IFoo_ImplVtbl<T: IFoo_Impl>(core::marker::PhantomData<T>);
    impl<T: IFoo_Impl> IFoo_ImplVtbl<T> {
        const VTABLE: IFoo_Vtbl = IFoo_Vtbl::new::<T>();
    }
    impl IFoo {
        pub fn new<'a, T: IFoo_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
            let this = windows_core::ScopedHeap {
                vtable: &IFoo_ImplVtbl::<T>::VTABLE as *const _ as *const _,
                this: this as *const _ as *const _,
            };
            let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
            unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
        }
    }
}
