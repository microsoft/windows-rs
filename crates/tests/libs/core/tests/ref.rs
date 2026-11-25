#![cfg(windows)]
#![allow(non_camel_case_types)]

use test_core::E_POINTER;
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
    let mut result = HSTRING::from("will be dropped");
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
    let error = return_object(InRef::default(), OutRef::default()).unwrap_err();
    assert_eq!(error.code(), E_POINTER);
}

#[test]
fn bstr_borrowed_abi_and_output() {
    let input = BSTR::from("borrowed\0text");
    let borrowed: ParamValue<BSTR> = unsafe { (&input).param() };
    let retained = borrowed.borrow().cloned().unwrap();
    drop(borrowed);
    assert_eq!(&*input, &*retained);
    assert_ne!(input.as_ptr(), retained.as_ptr());
    drop(input);
    assert_eq!(retained.display().to_string(), "borrowed\0text");

    let mut output = BSTR::from("old");
    OutRef::<BSTR>::from(&mut output).write(retained).unwrap();
    assert_eq!(output.display().to_string(), "borrowed\0text");
    assert_eq!(
        OutRef::<BSTR>::default()
            .write(BSTR::from("rejected"))
            .unwrap_err()
            .code(),
        E_POINTER
    );
}

#[test]
fn borrowed_null_is_not_an_empty_value() {
    let empty = HSTRING::new();
    let borrowed = InRef::from(&empty);
    assert!(!borrowed.is_null());
    assert_eq!(borrowed.as_ref(), Some(&empty));

    let borrowed = InRef::from(&false);
    assert!(!borrowed.is_null());
    assert_eq!(borrowed.as_ref(), Some(&false));

    let borrowed = InRef::<ITest>::default();
    assert!(borrowed.is_null());
    assert!(borrowed.as_ref().is_none());
    assert_eq!(borrowed.ok().unwrap_err().code(), E_POINTER);
    assert_eq!(
        <ITest as imp::Type>::from_default(&None)
            .unwrap_err()
            .code(),
        HRESULT(0)
    );
}

#[test]
fn borrowed_view_clones_only_when_retained() {
    use std::sync::Arc;

    #[derive(Clone)]
    struct Value(Arc<()>);

    impl imp::TypeKind for Value {
        type TypeKind = imp::CloneType;
    }

    let value = Value(Arc::new(()));
    let retained = {
        let borrowed = InRef::from(&value);
        let view = borrowed.as_ref().unwrap();
        assert_eq!(Arc::strong_count(&view.0), 1);
        <Value as imp::Type>::from_default(&borrowed).unwrap()
    };
    assert_eq!(Arc::strong_count(&value.0), 2);
    drop(value);
    assert_eq!(Arc::strong_count(&retained.0), 1);
}
