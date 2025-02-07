mod bindings;

use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "test_overloads.A" {
        factory.write(Some(FA.into())).into()
    } else if *name == "test_overloads.B" {
        factory.write(Some(FB.into())).into()
    } else if *name == "test_overloads.C" {
        factory.write(Some(FC.into())).into()
    } else if *name == "test_overloads.D" {
        factory.write(Some(FD.into())).into()
    } else if *name == "test_overloads.E" {
        factory.write(Some(FE.into())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[implement(IActivationFactory)]
struct FA;
#[implement(bindings::A)]
struct A;
impl IActivationFactory_Impl for FA_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(A.into())
    }
}
impl bindings::IA_Impl for A_Impl {
    fn Method(&self) -> Result<i32> {
        Ok(1)
    }
    fn Method2(&self, _: i32) -> Result<i32> {
        Ok(2)
    }
}

#[implement(IActivationFactory)]
struct FB;
#[implement(bindings::B)]
struct B;
impl IActivationFactory_Impl for FB_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(B.into())
    }
}
impl bindings::IB_Impl for B_Impl {
    fn MethodOne(&self) -> Result<i32> {
        Ok(3)
    }
    fn MethodTwo(&self, _: i32) -> Result<i32> {
        Ok(4)
    }
}

#[implement(IActivationFactory)]
struct FC;
#[implement(bindings::C)]
struct C;
impl IActivationFactory_Impl for FC_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(C.into())
    }
}
impl bindings::IC_Impl for C_Impl {
    fn Method(&self) -> Result<i32> {
        Ok(5)
    }
    fn Method2(&self, _: i32) -> Result<i32> {
        Ok(6)
    }
}

#[implement(IActivationFactory)]
struct FD;
#[implement(bindings::D, bindings::ID2)] // TODO: https://github.com/microsoft/windows-rs/issues/3258
struct D;
impl IActivationFactory_Impl for FD_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(D.into())
    }
}
impl bindings::ID_Impl for D_Impl {
    fn Method(&self) -> Result<i32> {
        Ok(7)
    }
    fn Method2(&self, _: i32) -> Result<i32> {
        Ok(8)
    }
}
impl bindings::ID2_Impl for D_Impl {
    fn Method(&self, _: i32, _: i32) -> Result<i32> {
        Ok(9)
    }
    fn Method2(&self, _: i32, _: i32, _: i32) -> Result<i32> {
        Ok(10)
    }
}

#[implement(IActivationFactory)]
struct FE;
#[implement(bindings::E, bindings::IE2)] // TODO: https://github.com/microsoft/windows-rs/issues/3258
struct E;
impl IActivationFactory_Impl for FE_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(E.into())
    }
}
impl bindings::IE_Impl for E_Impl {
    fn MethodOne(&self) -> Result<i32> {
        Ok(11)
    }
    fn MethodTwo(&self, _: i32) -> Result<i32> {
        Ok(12)
    }
}
impl bindings::IE2_Impl for E_Impl {
    fn MethodThree(&self, _: i32, _: i32) -> Result<i32> {
        Ok(13)
    }
    fn MethodFour(&self, _: i32, _: i32, _: i32) -> Result<i32> {
        Ok(14)
    }
}
