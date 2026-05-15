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
    pub trait IBase_Impl {}
    impl IBase_Vtbl {
        pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> Self
        where
            <Identity as windows_core::IUnknownImpl>::Impl: IBase_Impl,
        {
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
    windows_core::imp::define_interface!(ILeaf, ILeaf_Vtbl, 0xdd76df24_0960_586d_8083_e2e48f6d2790);
    impl windows_core::RuntimeType for ILeaf {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        ILeaf,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl windows_core::RuntimeName for ILeaf {
        const NAME: &'static str = "Test.ILeaf";
    }
    pub trait ILeaf_Impl {}
    impl ILeaf_Vtbl {
        pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> Self
        where
            <Identity as windows_core::IUnknownImpl>::Impl: ILeaf_Impl,
        {
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, ILeaf, OFFSET>(),
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
    }
    windows_core::imp::define_interface!(
        IMiddle,
        IMiddle_Vtbl,
        0xd45b6734_977a_576c_9ef0_7494cc9bdc37
    );
    impl windows_core::RuntimeType for IMiddle {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IMiddle,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl windows_core::RuntimeName for IMiddle {
        const NAME: &'static str = "Test.IMiddle";
    }
    pub trait IMiddle_Impl {}
    impl IMiddle_Vtbl {
        pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> Self
        where
            <Identity as windows_core::IUnknownImpl>::Impl: IMiddle_Impl,
        {
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IMiddle, OFFSET>(),
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IMiddle as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IMiddle_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
    }
    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Leaf(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        Leaf,
        windows_core::IUnknown,
        windows_core::IInspectable,
        ILeaf
    );
    windows_core::imp::required_hierarchy!(Leaf, IBase, IMiddle, Middle, Base);
    impl Leaf {}
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
        windows_core::IInspectable,
        IMiddle
    );
    windows_core::imp::required_hierarchy!(Middle, IBase, Base);
    impl Middle {}
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
