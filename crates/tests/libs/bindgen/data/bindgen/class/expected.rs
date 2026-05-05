pub mod Test {
    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Base(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        Base,
        windows_core::IUnknown,
        windows_core::IInspectable,
        IBase
    );
    impl Base {}
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
    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Derived(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        Derived,
        windows_core::IUnknown,
        windows_core::IInspectable,
        IDerived
    );
    windows_core::imp::required_hierarchy!(Derived, IBase, Base);
    impl Derived {}
    impl windows_core::RuntimeType for Derived {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_class::<Self, IDerived>();
    }
    unsafe impl windows_core::Interface for Derived {
        type Vtable = <IDerived as windows_core::Interface>::Vtable;
        const IID: windows_core::GUID = <IDerived as windows_core::Interface>::IID;
    }
    impl windows_core::RuntimeName for Derived {
        const NAME: &'static str = "Test.Derived";
    }
    windows_core::imp::define_interface!(IBase, IBase_Vtbl, 0x5fa5f341_e06d_51b0_b307_c25adde80bc2);
    impl windows_core::RuntimeType for IBase {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IBase,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl windows_core::RuntimeName for IBase {
        const NAME: &'static str = "Test.IBase";
    }
    pub trait IBase_Impl: windows_core::IUnknownImpl {}
    impl IBase_Vtbl {
        pub const fn new<Identity: IBase_Impl, const OFFSET: isize>() -> Self {
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IBase, OFFSET>(),
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
    }
    windows_core::imp::define_interface!(
        IDerived,
        IDerived_Vtbl,
        0x23cf7109_3cfc_56a0_88cb_06d8fd320cc6
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
    impl windows_core::RuntimeName for IDerived {
        const NAME: &'static str = "Test.IDerived";
    }
    pub trait IDerived_Impl: windows_core::IUnknownImpl {}
    impl IDerived_Vtbl {
        pub const fn new<Identity: IDerived_Impl, const OFFSET: isize>() -> Self {
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IDerived, OFFSET>(),
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
    }
}
