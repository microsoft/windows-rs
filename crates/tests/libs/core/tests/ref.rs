#![allow(non_camel_case_types)]

use windows::Win32::Foundation::E_POINTER;
use windows_core::*;

#[interface("6120f260-090f-4b55-a09f-3cce3ffb35a4")]
unsafe trait ITest: IUnknown {
    fn get(&self) -> usize;
}

#[implement(ITest)]
struct Test(usize);

impl Drop for Test {
    fn drop(&mut self) {
        println!("Drop for Test({})", self.0);
    }
}

impl ITest_Impl for Test_Impl {
    unsafe fn get(&self) -> usize {
        self.0
    }
}

fn call_hstring(input: Ref<HSTRING>) -> HSTRING {
    input.clone()
}

fn call_object(input: Ref<ITest>) -> Option<ITest> {
    input.clone()
}

#[test]
fn test_ref() {
    assert_eq!(call_hstring(h!("test").into()), "test");

    // Drop lifetime test
    {
        let test: ITest = Test(12).into();
        let clone = call_object((&test).into());
        drop(test);
        assert_eq!(unsafe { clone.unwrap().get() }, 12);
    }

    // Inline test (`get` is the only unsafe call)
    unsafe {
        assert_eq!(
            call_object((&ITest::from(Test(23))).into()).unwrap().get(),
            23
        );
    }

    // From &T
    {
        let test: ITest = Test(34).into();
        let test = call_object((&test).into());
        assert_eq!(unsafe { test.unwrap().get() }, 34);
    }

    // From &Option<T>
    {
        let test: Option<ITest> = Some(Test(45).into());
        let test = call_object((&test).into());
        assert_eq!(unsafe { test.unwrap().get() }, 45);
    }

    // From Option<&T>
    {
        let test: ITest = Test(56).into();
        let test = call_object(Some(&test).into());
        assert_eq!(unsafe { test.unwrap().get() }, 56);
    }
}

fn return_hstring(input: Ref<HSTRING>, output: OutRef<HSTRING>) -> Result<()> {
    output.write(input.clone())
}

fn return_object(input: Ref<ITest>, output: OutRef<ITest>) -> Result<()> {
    output.write(input.clone())
}

#[test]
fn test_out_ref() {
    let mut result = HSTRING::new();
    return_hstring(h!("test").into(), (&mut result).into()).unwrap();
    assert_eq!(result, "test");

    // input and output
    let input: ITest = Test(11).into();
    let mut output = None;
    return_object((&input).into(), (&mut output).into()).unwrap();
    drop(input);
    assert!(output.is_some());
    assert_eq!(unsafe { output.unwrap().get() }, 11);

    // "None" input and output
    let mut output = None;
    return_object((&None).into(), (&mut output).into()).unwrap();
    assert!(output.is_none());

    // default (optional) input and output
    let error = return_object(Ref::default(), OutRef::default()).unwrap_err();
    assert_eq!(error.code(), E_POINTER);
}
