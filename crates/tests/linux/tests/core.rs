use windows_core::*;

#[interface("d888acaa-fb67-46a4-bb35-87cb37db5830")]
unsafe trait ITest: IUnknown {}

#[implement(ITest)]
struct Test;

impl ITest_Impl for Test_Impl {}

#[test]
fn test() {
    let object = ComObject::new(Test);
    let unknown: IUnknown = object.to_interface();
    let _test: ITest = unknown.cast().expect("QueryInterface for ITest");
}
