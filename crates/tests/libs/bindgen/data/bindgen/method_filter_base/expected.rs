pub mod Test {
    windows_core::imp::define_interface!(IBase, IBase_Vtbl, 0x44a36fa2_38aa_5099_9fbf_d16084cd5901);
    impl windows_core::RuntimeType for IBase {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IBase,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IBase {
        pub fn KeepBase(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).KeepBase)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    impl windows_core::RuntimeName for IBase {
        const NAME: &'static str = "Test.IBase";
    }
    pub trait IBase_Impl: windows_core::IUnknownImpl {
        fn KeepBase(&self) -> windows_result::Result<i32>;
    }
    impl IBase_Vtbl {
        pub const fn new<Identity: IBase_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn KeepBase<Identity: IBase_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IBase_Impl::KeepBase(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            #[allow(unused_variables)]
            unsafe extern "system" fn FilteredOut<Identity: IBase_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                windows_core::HRESULT(0x80004001_u32 as i32)
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IBase, OFFSET>(),
                KeepBase: KeepBase::<Identity, OFFSET>,
                FilteredOut: FilteredOut::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IBase as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IBase_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub KeepBase:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
        FilteredOut:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(
        IDerived,
        IDerived_Vtbl,
        0x3c47d60c_1a1b_5658_893b_70d761a8bba3
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
    windows_core::imp::required_hierarchy!(IDerived, IBase);
    impl IDerived {
        pub fn KeepDerived(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).KeepDerived)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
        pub fn KeepBase(&self) -> windows_result::Result<i32> {
            let this = &windows_core::Interface::cast::<IBase>(self)?;
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).KeepBase)(
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
    pub trait IDerived_Impl: IBase_Impl {
        fn KeepDerived(&self) -> windows_result::Result<i32>;
    }
    impl IDerived_Vtbl {
        pub const fn new<Identity: IDerived_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn KeepDerived<Identity: IDerived_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IDerived_Impl::KeepDerived(this) {
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
                KeepDerived: KeepDerived::<Identity, OFFSET>,
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
        pub KeepDerived:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
}
