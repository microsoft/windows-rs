pub mod Test {
    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Base(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        Base,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl Base {
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
    impl windows_core::RuntimeType for Base {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_class::<Self, IBase>();
    }
    unsafe impl windows_core::Interface for Base {
        type Vtable = <IBase as windows_core::Interface>::Vtable;
        const IID: windows_core::GUID = <IBase as windows_core::Interface>::IID;
    }
    impl windows_core::RuntimeName for Base {
        const NAME: &'static str = "Test.Base";
    }
    windows_core::imp::define_interface!(IBase, IBase_Vtbl, 0xdf7d3fd2_cd33_50c6_b037_b80b8a5e31a0);
    impl windows_core::RuntimeType for IBase {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IBase_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub BaseMethod:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(ILeaf, ILeaf_Vtbl, 0xc0eb0b4d_9d13_54f9_ab6e_8642fd4f946e);
    impl windows_core::RuntimeType for ILeaf {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    impl windows_core::RuntimeName for ILeaf {
        const NAME: &'static str = "Test.ILeaf";
    }
    pub trait ILeaf_Impl: windows_core::IUnknownImpl {
        fn LeafMethod(&self) -> windows_result::Result<i32>;
    }
    impl ILeaf_Vtbl {
        pub const fn new<Identity: ILeaf_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn LeafMethod<Identity: ILeaf_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match ILeaf_Impl::LeafMethod(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, ILeaf, OFFSET>(),
                LeafMethod: LeafMethod::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<ILeaf as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct ILeaf_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub LeafMethod:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(
        IMiddle,
        IMiddle_Vtbl,
        0xe4b0462c_af0b_56ac_846e_b8dbc0c363a4
    );
    impl windows_core::RuntimeType for IMiddle {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IMiddle_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub MiddleMethod:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Leaf(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        Leaf,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    windows_core::imp::required_hierarchy!(Leaf, Middle, Base);
    impl Leaf {
        pub fn BaseMethod(&self) -> windows_result::Result<i32> {
            let this = &windows_core::Interface::cast::<IBase>(self)?;
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).BaseMethod)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
        pub fn LeafMethod(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).LeafMethod)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
        pub fn MiddleMethod(&self) -> windows_result::Result<i32> {
            let this = &windows_core::Interface::cast::<IMiddle>(self)?;
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).MiddleMethod)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    impl windows_core::RuntimeType for Leaf {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_class::<Self, ILeaf>();
    }
    unsafe impl windows_core::Interface for Leaf {
        type Vtable = <ILeaf as windows_core::Interface>::Vtable;
        const IID: windows_core::GUID = <ILeaf as windows_core::Interface>::IID;
    }
    impl windows_core::RuntimeName for Leaf {
        const NAME: &'static str = "Test.Leaf";
    }
    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Middle(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        Middle,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    windows_core::imp::required_hierarchy!(Middle, Base);
    impl Middle {
        pub fn BaseMethod(&self) -> windows_result::Result<i32> {
            let this = &windows_core::Interface::cast::<IBase>(self)?;
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).BaseMethod)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
        pub fn MiddleMethod(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).MiddleMethod)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    impl windows_core::RuntimeType for Middle {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_class::<Self, IMiddle>();
    }
    unsafe impl windows_core::Interface for Middle {
        type Vtable = <IMiddle as windows_core::Interface>::Vtable;
        const IID: windows_core::GUID = <IMiddle as windows_core::Interface>::IID;
    }
    impl windows_core::RuntimeName for Middle {
        const NAME: &'static str = "Test.Middle";
    }
}
