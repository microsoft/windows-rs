#![cfg(target_env = "msvc")]

use std::sync::atomic::*;
use test_ref_params::*;

#[implement(ITest)]
struct Test(AtomicI32);

impl ITest_Impl for Test_Impl {
    fn Input(&self, input: Ref<ITest>) -> Result<i32> {
        input.ok()?.Current()
    }
    fn Output(&self, value: i32, output: OutRef<ITest>) -> Result<()> {
        output.write(Some(Test(value.into()).into()))
    }
    fn Current(&self) -> Result<i32> {
        Ok(self.0.load(Ordering::Relaxed))
    }
    fn SetCurrent(&self, value: i32) -> Result<()> {
        self.0.store(value, Ordering::Relaxed);
        Ok(())
    }
}

fn test_interface(test: &ITest) -> Result<()> {
    assert_eq!(test.Current()?, 0);
    test.SetCurrent(-321)?;
    assert_eq!(test.Current()?, -321);

    let one_two_three: ITest = Test(123.into()).into();
    let four_five_six: ITest = Test(456.into()).into();

    assert_eq!(test.Input(&one_two_three)?, 123);
    assert_eq!(test.Input(&four_five_six)?, 456);
    assert_eq!(test.Input(None).unwrap_err().code(), HRESULT(-2147467261)); // E_POINTER

    let mut seven_eight_nine = None;
    test.Output(789, &mut seven_eight_nine)?;
    assert_eq!(seven_eight_nine.unwrap().Current()?, 789);

    Ok(())
}

#[test]
fn test_rust() -> Result<()> {
    let test: ITest = Test(0.into()).into();
    test_interface(&test)
}

#[test]
fn test_cpp() -> Result<()> {
    let test: ITest = Test(0.into()).into();
    consume(&test)?;

    let test: ITest = produce()?;
    test_interface(&test)
}
