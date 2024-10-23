#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub mod Windows {
    pub mod Foundation {
        windows_core::imp::define_interface!(
            IWwwFormUrlDecoderRuntimeClass,
            IWwwFormUrlDecoderRuntimeClass_Vtbl,
            0xd45a0451_f225_4542_9296_0e1df5d254df
        );
        windows_core::imp::interface_hierarchy!(
            IWwwFormUrlDecoderRuntimeClass,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        impl core::ops::Deref for IWwwFormUrlDecoderRuntimeClass {
            type Target = windows_core::IInspectable;
            fn deref(&self) -> &Self::Target {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl windows_core::RuntimeType for IWwwFormUrlDecoderRuntimeClass {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        #[repr(C)]
        pub struct IWwwFormUrlDecoderRuntimeClass_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
        }
        windows_core::imp::define_interface!(
            IWwwFormUrlDecoderRuntimeClassFactory,
            IWwwFormUrlDecoderRuntimeClassFactory_Vtbl,
            0x5b8c6b3d_24ae_41b5_a1bf_f0c3d544845b
        );
        windows_core::imp::interface_hierarchy!(
            IWwwFormUrlDecoderRuntimeClassFactory,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        impl core::ops::Deref for IWwwFormUrlDecoderRuntimeClassFactory {
            type Target = windows_core::IInspectable;
            fn deref(&self) -> &Self::Target {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl windows_core::RuntimeType for IWwwFormUrlDecoderRuntimeClassFactory {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        #[repr(C)]
        pub struct IWwwFormUrlDecoderRuntimeClassFactory_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
        }
        #[repr(transparent)]
        #[derive(PartialEq, Eq, Debug, Clone)]
        pub struct WwwFormUrlDecoder(windows_core::IUnknown);
        windows_core::imp::interface_hierarchy!(
            WwwFormUrlDecoder,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        impl WwwFormUrlDecoder {
            pub fn Size(&self) {}
        }
        impl windows_core::RuntimeType for WwwFormUrlDecoder {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_class::<Self, IWwwFormUrlDecoderRuntimeClass>();
        }
        unsafe impl windows_core::Interface for WwwFormUrlDecoder {
            type Vtable = <IWwwFormUrlDecoderRuntimeClass as windows_core::Interface>::Vtable;
            const IID: windows_core::GUID =
                <IWwwFormUrlDecoderRuntimeClass as windows_core::Interface>::IID;
        }
        impl windows_core::RuntimeName for WwwFormUrlDecoder {
            const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoder";
        }
        pub mod Collections {
            #[repr(transparent)]
            #[derive(PartialEq, Eq, Debug, Clone)]
            pub struct IIterable<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
            where
                T: windows_core::RuntimeType + 'static;
            impl<T: windows_core::RuntimeType + 'static>
                windows_core::imp::CanInto<windows_core::IUnknown> for IIterable<T>
            {
            }
            impl<T: windows_core::RuntimeType + 'static>
                windows_core::imp::CanInto<windows_core::IInspectable> for IIterable<T>
            {
            }
            unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterable<T> {
                type Vtable = IIterable_Vtbl;
                const IID: windows_core::GUID =
                    windows_core::GUID::from_u128(0xfaa585ea_6214_4217_afda_7f46de5869b3);
            }
            impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IIterable<T> {
                type Target = windows_core::IInspectable;
                fn deref(&self) -> &Self::Target {
                    unsafe { core::mem::transmute(self) }
                }
            }
            impl<T: windows_core::RuntimeType + 'static> IIterable<T> {
                pub fn First(&self) {}
            }
            impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterable<T> {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            pub struct IIterable_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            #[repr(transparent)]
            #[derive(PartialEq, Eq, Debug, Clone)]
            pub struct IVectorView<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
            where
                T: windows_core::RuntimeType + 'static;
            impl<T: windows_core::RuntimeType + 'static>
                windows_core::imp::CanInto<windows_core::IUnknown> for IVectorView<T>
            {
            }
            impl<T: windows_core::RuntimeType + 'static>
                windows_core::imp::CanInto<windows_core::IInspectable> for IVectorView<T>
            {
            }
            impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>>
                for IVectorView<T>
            {
                const QUERY: bool = true;
            }
            unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVectorView<T> {
                type Vtable = IVectorView_Vtbl;
                const IID: windows_core::GUID =
                    windows_core::GUID::from_u128(0xbbe1fa4c_b0e3_4583_baef_1f1b2e483e56);
            }
            impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IVectorView<T> {
                type Target = windows_core::IInspectable;
                fn deref(&self) -> &Self::Target {
                    unsafe { core::mem::transmute(self) }
                }
            }
            impl<T: windows_core::RuntimeType + 'static> IVectorView<T> {
                pub fn GetAt(&self) {}
                pub fn Size(&self) {}
                pub fn IndexOf(&self) {}
                pub fn GetMany(&self) {}
            }
            impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVectorView<T> {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            pub struct IVectorView_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
        }
    }
}
