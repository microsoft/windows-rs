pub mod Test {
    windows_core::imp::define_interface!(
        IDerived,
        IDerived_Vtbl,
        0xd3fed0a4_8ba5_5d1a_ac8f_cf0cb08debba
    );
    impl windows_core::RuntimeType for IDerived {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IDerived,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    windows_core::imp::required_hierarchy!(IDerived, IRequired);
    impl IDerived {
        pub fn Value(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Value)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
        pub fn Stub(&self) -> windows_result::Result<i32> {
            let this = &windows_core::Interface::cast::<IRequired>(self)?;
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).Stub)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    impl windows_core::RuntimeName for IDerived {
        const NAME: &'static str = "Test.IDerived";
    }
    pub trait IDerived_Impl: IRequired_Impl {
        fn Value(&self) -> windows_result::Result<i32>;
    }
    impl IDerived_Vtbl {
        pub const fn new<Identity: IDerived_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn Value<Identity: IDerived_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IDerived_Impl::Value(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IDerived, OFFSET>(),
                Value: Value::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IDerived as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IDerived_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Value:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(
        IRequired,
        IRequired_Vtbl,
        0xfa61ccc9_5bec_5f57_bb33_4ad48dd7ffda
    );
    impl windows_core::RuntimeType for IRequired {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IRequired,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl windows_core::RuntimeName for IRequired {
        const NAME: &'static str = "Test.IRequired";
    }
    pub trait IRequired_Impl: windows_core::IUnknownImpl {
        fn Stub(&self) -> windows_result::Result<i32>;
    }
    impl IRequired_Vtbl {
        pub const fn new<Identity: IRequired_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn Stub<Identity: IRequired_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IRequired_Impl::Stub(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IRequired, OFFSET>(),
                Stub: Stub::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IRequired as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IRequired_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Stub:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
}
