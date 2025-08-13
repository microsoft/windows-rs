#![expect(non_snake_case)]

use windows_core::*;

#[interface("09428a59-5b40-4e4c-9175-e7a78514316d")]
unsafe trait ITest: IUnknown {
    unsafe fn Test(&self, result: &mut i32) -> Result<()>;
    unsafe fn TestOther(&self, other: Ref<ITest>, result: &mut i32) -> Result<()>;
}

#[implement(ITest)]
struct Test(i32);

impl ITest_Impl for Test_Impl {
    unsafe fn Test(&self, result: &mut i32) -> Result<()> {
        *result = self.0;
        Ok(())
    }
    unsafe fn TestOther(&self, other: Ref<ITest>, result: &mut i32) -> Result<()> {
        unsafe { other.ok()?.Test(result) }
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let a: ITest = Test(123).into();
        let b: ITest = Test(456).into();

        let mut result = 0;

        a.Test(&mut result)?;
        assert_eq!(result, 123);

        b.Test(&mut result)?;
        assert_eq!(result, 456);

        b.TestOther(&a, &mut result)?;
        assert_eq!(result, 123);

        a.TestOther(&b, &mut result)?;
        assert_eq!(result, 456);

        Ok(())
    }
}
