pub mod Test {
    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Foo(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        Foo,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    windows_core::imp::required_hierarchy!(Foo, IBase);
    impl Foo {
        pub fn new() -> windows_result::Result<Self> {
            Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
        }
        fn IActivationFactory<
            R,
            F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_result::Result<R>,
        >(
            callback: F,
        ) -> windows_result::Result<R> {
            static SHARED: windows_core::imp::FactoryCache<
                Foo,
                windows_core::imp::IGenericFactory,
            > = windows_core::imp::FactoryCache::new();
            SHARED.call(callback)
        }
    }
    impl windows_core::RuntimeType for Foo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_class::<Self, IFoo>();
    }
    unsafe impl windows_core::Interface for Foo {
        type Vtable = <IFoo as windows_core::Interface>::Vtable;
        const IID: windows_core::GUID = <IFoo as windows_core::Interface>::IID;
    }
    impl windows_core::RuntimeName for Foo {
        const NAME: &'static str = "Test.Foo";
    }
    windows_core::imp::define_interface!(IBase, IBase_Vtbl, 0xdf7d3fd2_cd33_50c6_b037_b80b8a5e31a0);
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
        pub fn BaseMethod(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).BaseMethod)(
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
        fn BaseMethod(&self) -> windows_result::Result<i32>;
    }
    impl IBase_Vtbl {
        pub const fn new<Identity: IBase_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn BaseMethod<Identity: IBase_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IBase_Impl::BaseMethod(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IBase, OFFSET>(),
                BaseMethod: BaseMethod::<Identity, OFFSET>,
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
        pub BaseMethod:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0xf05f601f_33d2_54fc_adca_2df8c56fe621);
    impl windows_core::RuntimeType for IFoo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    impl IFoo {
        pub fn Direct(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Direct)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Direct:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
}
