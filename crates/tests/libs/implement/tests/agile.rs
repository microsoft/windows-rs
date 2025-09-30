use windows::core::*;
use windows::Win32::System::Com::{Marshal::*, *};

#[interface("e9f8ddc5-7006-4c7f-9c1e-5a08e240cb13")]
unsafe trait ITest: IUnknown {}

#[implement(ITest)]
struct DefaultAgile;
impl ITest_Impl for DefaultAgile_Impl {}

#[implement(ITest, Agile = false)]
struct AgileFalse;
impl ITest_Impl for AgileFalse_Impl {}

#[implement(ITest, Agile = true)]
struct AgileTrue;
impl ITest_Impl for AgileTrue_Impl {}

#[implement(ITest, Agile = false, IAgileObject)]
struct ExplicitAgile;
impl ITest_Impl for ExplicitAgile_Impl {}
impl IAgileObject_Impl for ExplicitAgile_Impl {}

#[test]
fn test_default() {
    let test: ITest = DefaultAgile.into();
    test.cast::<ITest>().unwrap();
    test.cast::<IAgileObject>().unwrap();
    test.cast::<IMarshal>().unwrap();
}

#[test]
fn test_agile_false() {
    let test: ITest = AgileFalse.into();
    test.cast::<ITest>().unwrap();
    test.cast::<IAgileObject>().unwrap_err();
    test.cast::<IMarshal>().unwrap_err();
}

#[test]
fn test_agile_true() {
    let test: ITest = DefaultAgile.into();
    test.cast::<ITest>().unwrap();
    test.cast::<IAgileObject>().unwrap();
    test.cast::<IMarshal>().unwrap();
}

#[test]
fn test_explicit_agile() {
    let test: ITest = ExplicitAgile.into();
    test.cast::<ITest>().unwrap();
    test.cast::<IAgileObject>().unwrap();
    test.cast::<IMarshal>().unwrap_err();
}
