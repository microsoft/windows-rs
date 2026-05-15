pub mod Test {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Baz {
        pub Width: f32,
        pub Height: f32,
    }
    impl windows_core::TypeKind for Baz {
        type TypeKind = windows_core::CopyType;
    }
    impl windows_core::RuntimeType for Baz {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"struct(Test.Baz;f4;f4)");
    }
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0x5dcafa36_d2c1_5ce7_ac1f_f53c18e6d978);
    impl windows_core::RuntimeType for IFoo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IFoo,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IFoo {
        pub fn Keep(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Keep)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    impl windows_core::RuntimeName for IFoo {
        const NAME: &'static str = "Test.IFoo";
    }
    pub trait IFoo_Impl: windows_core::IUnknownImpl {
        fn Keep(&self) -> windows_result::Result<i32>;
    }
    impl IFoo_Vtbl {
        pub const fn new<Identity: IFoo_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn Keep<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IFoo_Impl::Keep(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            #[allow(unused_variables)]
            unsafe extern "system" fn GetBaz<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut Baz,
            ) -> windows_core::HRESULT {
                windows_core::HRESULT(0x80004001_u32 as i32)
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IFoo, OFFSET>(),
                Keep: Keep::<Identity, OFFSET>,
                GetBaz: GetBaz::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IFoo as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Keep:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
        GetBaz:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut Baz) -> windows_result::HRESULT,
    }
}
