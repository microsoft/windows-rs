use windows_core::*;

pub const S_OK: HRESULT = HRESULT(0);
pub const S_FALSE: HRESULT = HRESULT(1);
pub const E_INVALIDARG: HRESULT = HRESULT(0x80070057_u32 as _);
pub const E_POINTER: HRESULT = HRESULT(0x80004003_u32 as _);

#[interface("09428a59-5b40-4e4c-9175-e7a78514316d")]
unsafe trait ITest: IUnknown {
    // TODO: compile error if param type is not Ref/OutRef and is not Copy

    unsafe fn usize(&self, input: usize, output: OutRef<usize>) -> HRESULT;
    unsafe fn hstring(&self, input: Ref<HSTRING>, output: OutRef<HSTRING>) -> HRESULT;
    unsafe fn interface(&self, input: Ref<ITest>, output: OutRef<ITest>) -> HRESULT;
    unsafe fn required_input(&self, input: Ref<ITest>, output: OutRef<ITest>) -> HRESULT;
    unsafe fn optional_output(&self, input: Ref<ITest>, output: OutRef<ITest>) -> HRESULT;

    unsafe fn result_usize(&self, input: usize, output: OutRef<usize>) -> Result<()>;
    unsafe fn result_hstring(&self, input: Ref<HSTRING>, output: OutRef<HSTRING>) -> Result<()>;
    unsafe fn result_interface(&self, input: Ref<ITest>, output: OutRef<ITest>) -> Result<()>;
    unsafe fn result_required_input(&self, input: Ref<ITest>, output: OutRef<ITest>) -> Result<()>;
}

#[implement(ITest)]
struct Test;

impl ITest_Impl for Test_Impl {
    unsafe fn usize(&self, input: usize, output: OutRef<usize>) -> HRESULT {
        output.write(input).into()
    }
    unsafe fn hstring(&self, input: Ref<HSTRING>, output: OutRef<HSTRING>) -> HRESULT {
        output.write(input.clone()).into()
    }
    unsafe fn interface(&self, input: Ref<ITest>, output: OutRef<ITest>) -> HRESULT {
        output.write(input.clone()).into()
    }
    unsafe fn required_input(&self, input: Ref<ITest>, output: OutRef<ITest>) -> HRESULT {
        if input.is_none() {
            E_INVALIDARG
        } else {
            unsafe { self.interface(input, output) }
        }
    }

    unsafe fn optional_output(&self, input: Ref<ITest>, output: OutRef<ITest>) -> HRESULT {
        if output.is_null() {
            S_FALSE
        } else {
            unsafe { self.interface(input, output) }
        }
    }

    unsafe fn result_usize(&self, input: usize, output: OutRef<usize>) -> Result<()> {
        output.write(input)
    }
    unsafe fn result_hstring(&self, input: Ref<HSTRING>, output: OutRef<HSTRING>) -> Result<()> {
        output.write(input.clone())
    }
    unsafe fn result_interface(&self, input: Ref<ITest>, output: OutRef<ITest>) -> Result<()> {
        output.write(input.clone())
    }
    unsafe fn result_required_input(&self, input: Ref<ITest>, output: OutRef<ITest>) -> Result<()> {
        if input.is_none() {
            E_INVALIDARG.ok()
        } else {
            unsafe { self.result_interface(input, output) }
        }
    }
}

#[test]
fn test() {
    unsafe {
        let test: ITest = Test.into();

        assert_eq!(test.usize(0, None), E_POINTER);
        assert_eq!(test.hstring(h!("hello"), None), E_POINTER);
        assert_eq!(test.interface(None, None), E_POINTER);
        assert_eq!(test.required_input(None, None), E_INVALIDARG);

        let mut output = 0;
        assert_eq!(test.usize(123, &mut output), S_OK);
        assert_eq!(output, 123);

        let mut output = HSTRING::from("will be dropped");
        // `output` will be dropped before receiving value, avoiding a leak.
        assert_eq!(test.hstring(h!("hello"), &mut output), S_OK);
        assert_eq!(&output, h!("hello"));

        let mut output = None;
        assert_eq!(test.interface(&test, &mut output), S_OK);
        assert_eq!(output.as_ref(), Some(&test));

        // `output` will be dropped before receiving next value, avoiding a leak.
        assert_eq!(test.required_input(&test, &mut output), S_OK);
        assert_eq!(output.as_ref(), Some(&test));

        assert_eq!(test.optional_output(&test, None), S_FALSE);
        assert_eq!(test.optional_output(&test, &mut output), S_OK);
        assert_eq!(output, Some(test));
    }
}

#[test]
fn test_result() {
    unsafe {
        let test: ITest = Test.into();

        assert_eq!(test.result_usize(0, None), E_POINTER.ok());
        assert_eq!(test.result_hstring(h!("hello"), None), E_POINTER.ok());
        assert_eq!(test.result_interface(None, None), E_POINTER.ok());
        assert_eq!(test.result_required_input(None, None), E_INVALIDARG.ok());

        let mut output = 0;
        assert_eq!(test.result_usize(123, &mut output), Ok(()));
        assert_eq!(output, 123);

        let mut output = HSTRING::from("will be dropped");
        // `output` will be dropped before receiving value, avoiding a leak.
        assert_eq!(test.result_hstring(h!("hello"), &mut output), Ok(()));
        assert_eq!(&output, h!("hello"));

        let mut output = None;
        assert_eq!(test.result_interface(&test, &mut output), Ok(()));
        assert_eq!(output.as_ref(), Some(&test));

        // `output` will be dropped before receiving next value, avoiding a leak.
        assert_eq!(test.result_required_input(&test, &mut output), Ok(()));
        assert_eq!(output, Some(test));
    }
}
