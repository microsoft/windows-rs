#![cfg(target_env = "msvc")]

use test_noexcept::*;

#[implement(ITest)]
#[derive(Default)]
struct Test(std::sync::RwLock<(HSTRING, i32, Option<ITest>)>);

impl ITest_Impl for Test_Impl {
    fn MethodString(&self, test: &HSTRING) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.0 = test.clone();
        Ok(())
    }
    fn MethodInt32(&self, test: i32) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.1 = test;
        Ok(())
    }
    fn MethodTest(&self, test: Ref<ITest>) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.2 = test.cloned();
        Ok(())
    }
    fn String(&self) -> Result<HSTRING> {
        let this = self.0.read().unwrap();
        Ok(this.0.clone())
    }
    fn SetString(&self, value: &HSTRING) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.0 = value.clone();
        Ok(())
    }
    fn Int32(&self) -> Result<i32> {
        let this = self.0.read().unwrap();
        Ok(this.1)
    }
    fn SetInt32(&self, value: i32) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.1 = value;
        Ok(())
    }
    fn Test(&self) -> Result<ITest> {
        let this = self.0.read().unwrap();
        this.2.clone().ok_or_else(Error::empty)
    }
    fn SetTest(&self, value: Ref<ITest>) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.2 = value.cloned();
        Ok(())
    }

    fn MethodStringN(&self, test: &HSTRING) {
        let mut this = self.0.write().unwrap();
        this.0 = test.clone();
    }
    fn MethodInt32N(&self, test: i32) {
        let mut this = self.0.write().unwrap();
        this.1 = test;
    }
    fn MethodTestN(&self, test: Ref<ITest>) {
        let mut this = self.0.write().unwrap();
        this.2 = test.cloned();
    }
    fn StringN(&self) -> HSTRING {
        let this = self.0.read().unwrap();
        this.0.clone()
    }
    fn SetStringN(&self, value: &HSTRING) {
        let mut this = self.0.write().unwrap();
        this.0 = value.clone();
    }
    fn Int32N(&self) -> i32 {
        let this = self.0.read().unwrap();
        this.1
    }
    fn SetInt32N(&self, value: i32) {
        let mut this = self.0.write().unwrap();
        this.1 = value;
    }
    fn TestN(&self) -> Option<ITest> {
        let this = self.0.read().unwrap();
        this.2.clone()
    }
    fn SetTestN(&self, value: Ref<ITest>) {
        let mut this = self.0.write().unwrap();
        this.2 = value.cloned();
    }
}

fn test_except(test: &ITest) -> Result<()> {
    test.MethodString(h!("abc"))?;
    assert_eq!(test.String()?, "abc");

    test.MethodInt32(123)?;
    assert_eq!(test.Int32()?, 123);

    test.MethodTest(test)?;
    assert_eq!(&test.Test()?, test);

    Ok(())
}

fn test_noexcept(test: &ITest) {
    test.MethodStringN(h!("abc"));
    assert_eq!(test.StringN(), "abc");

    test.MethodInt32N(123);
    assert_eq!(test.Int32N(), 123);

    test.MethodTestN(test);
    assert_eq!(test.TestN().as_ref(), Some(test));
}

#[test]
fn test_rust() -> Result<()> {
    let test: ITest = Test::default().into();
    test_noexcept(&test);
    test_except(&test)
}

#[test]
fn test_cpp() -> Result<()> {
    let test: ITest = Test::default().into();
    consume(&test)?;

    let test: ITest = produce()?;
    test_noexcept(&test);
    test_except(&test)
}
