// mod composable;
 mod activatable;

// use windows::{core::*, Win32::System::WinRT::*, Win32::Foundation::*};

// #[no_mangle]
// unsafe extern "system" fn DllGetActivationFactory(
//     name: Ref<HSTRING>,
//     factory: OutRef<IActivationFactory>,
// ) -> HRESULT {
//     if *name == "Namespace.ActivatableType" {
//         factory.write(Some(ActivatableTypeFactory.into())).into()
//     } else {
//         _ = factory.write(None);
//         CLASS_E_CLASSNOTAVAILABLE
//     }
// }

// #[implement(IActivationFactory)]
// struct ActivatableTypeFactory;

// impl IActivationFactory_Impl for ActivatableTypeFactory_Impl {
//     fn ActivateInstance(&self) -> Result<IInspectable> {
//         Ok(ActivatableType::new().into())
//     }
// }

// #[implement(activatable::ActivatableType)]
// struct ActivatableType(i32);

// // impl IActivatableType_Impl for ActivatableType_Impl {
// //     fn Value(&self) -> Result<i32> {
// //         Ok(self.0)
// //     }
// // }

// impl ActivatableType {
//     fn new() -> Self {
//         Self(0)
//     }
// }