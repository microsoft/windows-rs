use super::ownership::*;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use windows_core::{Array, InRef, Interface, OutParam, OutRef, Param, ParamValue, RuntimeType};

#[windows_core::implement(OwnedResult, GenericResult<OwnedValue>, NativeResult)]
struct ResultProducer(OwnedValue);

fn owned_result() -> OwnedValue {
    OwnedValue {
        enabled: true,
        number: 42.25,
        ..Default::default()
    }
}

impl OwnedResult_Impl for ResultProducer_Impl {
    fn Get(&self) -> windows_core::Result<OwnedValue> {
        Ok(self.0.clone())
    }

    fn Infallible(&self) -> OwnedValue {
        self.0.clone()
    }

    fn GetMany(&self) -> windows_core::Result<Array<OwnedValue>> {
        Ok(self.InfallibleMany())
    }

    fn InfallibleMany(&self) -> Array<OwnedValue> {
        Array::from_slice(&[self.0.clone(), self.0.clone()])
    }
}

impl GenericResult_Impl<OwnedValue> for ResultProducer_Impl {
    fn Get(&self) -> windows_core::Result<OwnedValue> {
        Ok(self.0.clone())
    }

    fn Infallible(&self) -> OwnedValue {
        self.0.clone()
    }
}

impl NativeResult_Impl for ResultProducer_Impl {
    fn Get(&self) -> windows_core::Result<OwnedValue> {
        Ok(self.0.clone())
    }

    fn GetArray(&self) -> windows_core::Result<[OwnedValue; 2]> {
        Ok(core::array::from_fn(|_| self.0.clone()))
    }

    fn GetNestedArray(&self) -> windows_core::Result<[[OwnedValue; 2]; 2]> {
        Ok(core::array::from_fn(|_| {
            core::array::from_fn(|_| self.0.clone())
        }))
    }
}

#[test]
fn owning_return_abi() {
    let drops = Arc::new(AtomicUsize::new(0));
    let value = OwnedValue {
        object: Some(tracked(&drops)),
        ..owned_result()
    };
    let concrete: OwnedResult = ResultProducer(value.clone()).into();
    let generic: GenericResult<OwnedValue> = ResultProducer(value.clone()).into();
    let native: NativeResult = ResultProducer(value).into();
    let mut raw = core::mem::MaybeUninit::<OwnedValue>::uninit();
    unsafe {
        (concrete.vtable().Get)(concrete.as_raw(), raw.as_mut_ptr())
            .ok()
            .unwrap();
    }
    let results = [
        concrete.Get().unwrap(),
        concrete.Infallible(),
        generic.Get().unwrap(),
        generic.Infallible(),
        unsafe { native.Get().unwrap() },
        unsafe { raw.assume_init() },
    ];
    drop(concrete);
    drop(generic);
    drop(native);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    for value in results {
        assert!(value.text.is_empty());
        assert!(value.enabled);
        assert_eq!(value.number, 42.25);
        value.object.as_ref().unwrap().Invoke().unwrap();
    }
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

#[test]
fn owning_array_return_lifetimes() {
    let drops = Arc::new(AtomicUsize::new(0));
    let value = OwnedValue {
        object: Some(tracked(&drops)),
        ..owned_result()
    };
    let concrete: OwnedResult = ResultProducer(value.clone()).into();
    let native: NativeResult = ResultProducer(value).into();
    let array = unsafe { native.GetArray().unwrap() };
    let nested = unsafe { native.GetNestedArray().unwrap() };
    let mut raw_array = core::mem::MaybeUninit::<[OwnedValue; 2]>::uninit();
    let mut raw_nested = core::mem::MaybeUninit::<[[OwnedValue; 2]; 2]>::uninit();
    unsafe {
        (native.vtable().GetArray)(native.as_raw(), raw_array.as_mut_ptr())
            .ok()
            .unwrap();
        (native.vtable().GetNestedArray)(native.as_raw(), raw_nested.as_mut_ptr())
            .ok()
            .unwrap();
    }
    let raw_array = unsafe { raw_array.assume_init() };
    let raw_nested = unsafe { raw_nested.assume_init() };
    let many = concrete.GetMany().unwrap();
    let infallible_many = concrete.InfallibleMany();
    let mut raw_many = core::ptr::null_mut::<OwnedValue>();
    let mut len = 0;
    unsafe {
        (concrete.vtable().GetMany)(concrete.as_raw(), &mut len, &mut raw_many)
            .ok()
            .unwrap();
    }
    let raw_many = unsafe { Array::<OwnedValue>::from_raw_parts(raw_many, len) };
    assert_eq!(many.len(), 2);
    assert_eq!(infallible_many.len(), 2);
    assert_eq!(raw_many.len(), 2);
    drop(concrete);
    drop(native);
    for value in array
        .iter()
        .chain(nested.iter().flatten())
        .chain(raw_array.iter())
        .chain(raw_nested.iter().flatten())
        .chain(many.iter())
        .chain(infallible_many.iter())
        .chain(raw_many.iter())
    {
        assert!(value.enabled);
        assert_eq!(value.number, 42.25);
        value.object.as_ref().unwrap().Invoke().unwrap();
    }
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(array);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(nested);
    drop(raw_array);
    drop(raw_nested);
    drop(many);
    drop(infallible_many);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(raw_many);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

#[test]
fn owning_free_function_results() {
    for status in [windows_core::HRESULT(0), windows_core::HRESULT(1)] {
        let value = unsafe { GetOwned(status).unwrap() };
        assert_eq!(unsafe { BorrowOwned(&value) }, &raw const value);
        let array = unsafe { GetOwnedArray(status).unwrap() };
        let nested = unsafe { GetOwnedNestedArray(status).unwrap() };
        for value in core::iter::once(&value)
            .chain(array.iter())
            .chain(nested.iter().flatten())
        {
            assert!(value.text.is_empty());
            assert!(value.enabled);
            assert_eq!(value.number, 42.25);
            assert!(value.object.is_none());
        }
    }
}

#[cfg(any(not(miri), windows_slim_errors))]
#[test]
fn failed_owning_return_is_not_read() {
    let status = windows_core::HRESULT(0x80004005_u32 as i32);
    unsafe {
        assert_eq!(GetOwned(status).unwrap_err().code(), status);
        assert_eq!(GetOwnedArray(status).unwrap_err().code(), status);
        assert_eq!(GetOwnedNestedArray(status).unwrap_err().code(), status);
    }

    unsafe extern "system" fn fail<T>(
        _: *mut core::ffi::c_void,
        result: *mut T,
    ) -> windows_core::HRESULT {
        unsafe {
            // Even malformed failure bytes must remain unread storage.
            result.cast::<u8>().write_bytes(0xff, size_of::<T>());
        }
        windows_core::HRESULT(0x80004005_u32 as i32)
    }

    unsafe extern "system" fn uninitialized<T>(
        _: *mut core::ffi::c_void,
        result: *mut T,
    ) -> windows_core::HRESULT {
        let value = core::mem::MaybeUninit::uninit();
        unsafe {
            core::ptr::copy_nonoverlapping(value.as_ptr(), result, 1);
        }
        windows_core::HRESULT(0x80004005_u32 as i32)
    }

    for callback in [fail, uninitialized] {
        let mut concrete = OwnedResult_Vtbl::new::<ResultProducer_Impl, 0>();
        concrete.Get = callback;
        let vtable = &concrete;
        let raw = &vtable as *const _ as *mut core::ffi::c_void;
        unsafe {
            let borrowed = OwnedResult::from_raw_borrowed(&raw).unwrap();
            assert!(borrowed.Get().is_err());
        }

        let mut native = NativeResult_Vtbl::new::<ResultProducer_Impl, 0>();
        native.Get = callback;
        let vtable = &native;
        let raw = &vtable as *const _ as *mut core::ffi::c_void;
        unsafe {
            let borrowed = NativeResult::from_raw_borrowed(&raw).unwrap();
            assert!(borrowed.Get().is_err());
        }
    }

    for callback in [fail, uninitialized] {
        let mut generic = GenericResult_Vtbl::<OwnedValue>::new::<ResultProducer_Impl, 0>();
        generic.Get = callback;
        let vtable = &generic;
        let raw = &vtable as *const _ as *mut core::ffi::c_void;
        unsafe {
            let borrowed = GenericResult::<OwnedValue>::from_raw_borrowed(&raw).unwrap();
            assert!(borrowed.Get().is_err());
        }
    }

    for callback in [fail, uninitialized] {
        let mut native = NativeResult_Vtbl::new::<ResultProducer_Impl, 0>();
        native.GetArray = callback;
        let vtable = &native;
        let raw = &vtable as *const _ as *mut core::ffi::c_void;
        unsafe {
            let borrowed = NativeResult::from_raw_borrowed(&raw).unwrap();
            assert!(borrowed.GetArray().is_err());
        }
    }

    for callback in [fail, uninitialized] {
        let mut native = NativeResult_Vtbl::new::<ResultProducer_Impl, 0>();
        native.GetNestedArray = callback;
        let vtable = &native;
        let raw = &vtable as *const _ as *mut core::ffi::c_void;
        unsafe {
            let borrowed = NativeResult::from_raw_borrowed(&raw).unwrap();
            assert!(borrowed.GetNestedArray().is_err());
        }
    }
}

struct Lifetime(Arc<AtomicUsize>);

impl Drop for Lifetime {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

fn tracked(drops: &Arc<AtomicUsize>) -> Tracked {
    let lifetime = Lifetime(drops.clone());
    Tracked::new(move || {
        let _lifetime = &lifetime;
        Ok(())
    })
}

#[test]
fn array_roundtrip() {
    let input_drops = Arc::new(AtomicUsize::new(0));
    let fill_drops = Arc::new(AtomicUsize::new(0));
    let fill_capture = fill_drops.clone();
    let producer = ArrayRoundtrip::new(move |input, fill, receive| {
        assert_eq!(input.len(), 2);
        assert!(input[1].is_none());
        input[0].as_ref().unwrap().Invoke().unwrap();
        assert_eq!(fill.len(), 3);
        fill[0] = input[0].clone();
        fill[1] = None;
        fill[2] = Some(tracked(&fill_capture));
        *receive = Array::from_slice(input);
        Ok(Array::from_slice(input))
    });
    let input = [Some(tracked(&input_drops)), None];
    let identity = input[0].as_ref().unwrap().as_raw();
    let mut fill = [None, None, None];
    let mut receive = Array::new();
    let returned = producer.Invoke(&input, &mut fill, &mut receive).unwrap();
    for array in [&receive, &returned] {
        assert_eq!(array.len(), 2);
        assert_eq!(array[0].as_ref().unwrap().as_raw(), identity);
        assert!(array[1].is_none());
    }
    assert_eq!(fill[0].as_ref().unwrap().as_raw(), identity);
    assert!(fill[1].is_none());
    drop(producer);
    drop(input);
    drop(returned);
    drop(fill);
    assert_eq!(input_drops.load(Ordering::Relaxed), 0);
    assert_eq!(fill_drops.load(Ordering::Relaxed), 1);
    receive[0].as_ref().unwrap().Invoke().unwrap();
    receive.clear();
    assert_eq!(input_drops.load(Ordering::Relaxed), 1);
    receive.clear();
    drop(receive);
    assert_eq!(input_drops.load(Ordering::Relaxed), 1);
}

#[test]
fn interface_outputs() {
    let output_drops = Arc::new(AtomicUsize::new(0));
    let result_drops = Arc::new(AtomicUsize::new(0));
    let output_capture = output_drops.clone();
    let result_capture = result_drops.clone();
    let producer = InterfaceOutputs::new(move |output| {
        output.write(Some(tracked(&output_capture)))?;
        Ok(tracked(&result_capture))
    });
    let mut output = None;
    let result = producer.Invoke(&mut output).unwrap();
    drop(producer);
    assert_eq!(output_drops.load(Ordering::Relaxed), 0);
    assert_eq!(result_drops.load(Ordering::Relaxed), 0);
    result.Invoke().unwrap();
    drop(result);
    assert_eq!(result_drops.load(Ordering::Relaxed), 1);
    output.as_ref().unwrap().Invoke().unwrap();
    drop(output);
    assert_eq!(output_drops.load(Ordering::Relaxed), 1);
}

#[test]
fn direct_out_ref_replacement() {
    let old_drops = Arc::new(AtomicUsize::new(0));
    let new_drops = Arc::new(AtomicUsize::new(0));
    let mut output = Some(tracked(&old_drops));
    OutRef::<Tracked>::from(&mut output)
        .write(Some(tracked(&new_drops)))
        .unwrap();
    assert_eq!(old_drops.load(Ordering::Relaxed), 1);
    assert_eq!(new_drops.load(Ordering::Relaxed), 0);
    output.as_ref().unwrap().Invoke().unwrap();
    OutRef::<Tracked>::from(&mut output).write(None).unwrap();
    assert!(output.is_none());
    assert_eq!(new_drops.load(Ordering::Relaxed), 1);
}

#[test]
fn out_ref_writes_uninitialized_storage() {
    let drops = Arc::new(AtomicUsize::new(0));
    let mut storage = core::mem::MaybeUninit::<OwnedValue>::uninit();
    let output: OutRef<'_, OwnedValue> = unsafe { core::mem::transmute(storage.as_mut_ptr()) };
    output
        .write(OwnedValue {
            object: Some(tracked(&drops)),
            ..owned_result()
        })
        .unwrap();
    let value = unsafe { storage.assume_init() };
    assert!(value.enabled);
    assert_eq!(value.number, 42.25);
    value.object.as_ref().unwrap().Invoke().unwrap();
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(value);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert!(
        OutRef::<OwnedValue>::default()
            .write(OwnedValue {
                object: Some(tracked(&drops)),
                ..owned_result()
            })
            .is_err()
    );
    assert_eq!(drops.load(Ordering::Relaxed), 2);
}

#[windows_core::implement(GenericResult<T>)]
struct GenericProducer<T: RuntimeType + 'static>(T);

impl<T: RuntimeType + 'static> GenericResult_Impl<T> for GenericProducer_Impl<T> {
    fn Get(&self) -> windows_core::Result<T> {
        Ok(self.0.clone())
    }

    fn Infallible(&self) -> T {
        self.0.clone()
    }
}

#[test]
fn generic_return_representations() {
    let number: GenericResult<u32> = GenericProducer(42).into();
    assert_eq!(number.Get().unwrap(), 42);
    assert_eq!(number.Infallible(), 42);
    let text: GenericResult<windows_core::HSTRING> =
        GenericProducer(windows_core::HSTRING::new()).into();
    assert!(text.Get().unwrap().is_empty());
    assert!(text.Infallible().is_empty());

    let drops = Arc::new(AtomicUsize::new(0));
    let producer: GenericResult<Tracked> = GenericProducer(tracked(&drops)).into();
    let result = producer.Get().unwrap();
    let infallible = producer.Infallible();
    drop(producer);
    result.Invoke().unwrap();
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(result);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    infallible.Invoke().unwrap();
    drop(infallible);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

#[cfg(any(not(miri), windows_slim_errors))]
#[test]
fn generic_return_rejects_null_interface() {
    unsafe extern "system" fn null(
        _: *mut core::ffi::c_void,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe { result.write(core::ptr::null_mut()) };
        windows_core::HRESULT(0)
    }
    let mut vtable = GenericResult_Vtbl::<Tracked>::new::<GenericProducer_Impl<Tracked>, 0>();
    vtable.Get = null;
    vtable.Infallible = null;
    let vtable = &vtable;
    let raw = &vtable as *const _ as *mut core::ffi::c_void;
    let borrowed = unsafe { GenericResult::<Tracked>::from_raw_borrowed(&raw).unwrap() };
    assert!(borrowed.Get().is_err());
    // Miri cannot format Windows errors while constructing the panic payload.
    #[cfg(not(miri))]
    assert!(std::panic::catch_unwind(|| borrowed.Infallible()).is_err());
}

#[test]
fn direct_out_param_replacement() {
    let old_drops = Arc::new(AtomicUsize::new(0));
    let new_drops = Arc::new(AtomicUsize::new(0));
    let mut output = Some(tracked(&old_drops));
    unsafe {
        OutParam::<Tracked>::borrow_mut(&&mut output)
            .write(Some(tracked(&new_drops)))
            .unwrap();
    }
    assert_eq!(old_drops.load(Ordering::Relaxed), 1);
    assert_eq!(new_drops.load(Ordering::Relaxed), 0);
    output.as_ref().unwrap().Invoke().unwrap();
    drop(output);
    assert_eq!(new_drops.load(Ordering::Relaxed), 1);
}

#[derive(Clone, Default)]
struct TrackedValue(Option<Tracked>);

impl windows_core::imp::TypeKind for TrackedValue {
    type TypeKind = windows_core::imp::CloneType;
}

#[test]
fn optional_out_param_replacement() {
    let old_drops = Arc::new(AtomicUsize::new(0));
    let new_drops = Arc::new(AtomicUsize::new(0));
    let mut output = TrackedValue(Some(tracked(&old_drops)));
    unsafe {
        OutParam::<TrackedValue>::borrow_mut(&Some(&mut output))
            .write(TrackedValue(Some(tracked(&new_drops))))
            .unwrap();
    }
    assert_eq!(old_drops.load(Ordering::Relaxed), 1);
    assert_eq!(new_drops.load(Ordering::Relaxed), 0);
    output.0.as_ref().unwrap().Invoke().unwrap();
    {
        let _output = OutRef::<TrackedValue>::from(&mut output);
    }
    assert!(output.0.is_none());
    drop(output);
    assert_eq!(new_drops.load(Ordering::Relaxed), 1);
}

#[test]
fn optional_interface_output() {
    let drops = Arc::new(AtomicUsize::new(0));
    let mut output = Some(tracked(&drops));
    unsafe {
        OutParam::<Tracked>::borrow_mut(&Some(&mut output))
            .write(None)
            .unwrap();
        assert!(OutParam::<Tracked>::borrow_mut(&None::<&mut Option<Tracked>>).is_null());
    }
    assert!(output.is_none());
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

#[test]
fn copy_output_storage() {
    let mut output = 7u32;
    {
        let _output = OutRef::<u32>::from(&mut output);
    }
    assert_eq!(output, 7);
    unsafe {
        let _output = OutParam::<u32>::borrow_mut(&Some(&mut output));
    }
    assert_eq!(output, 7);
    OutRef::<u32>::from(&mut output).write(11).unwrap();
    assert_eq!(output, 11);
}

#[test]
fn borrowed_input_lifetimes() {
    let drops = Arc::new(AtomicUsize::new(0));
    let owner = Some(tracked(&drops));
    let cloned = {
        let borrowed = InRef::<Tracked>::from(&owner);
        assert_eq!(borrowed.unwrap().as_raw(), owner.as_ref().unwrap().as_raw());
        let from_ref = InRef::from(owner.as_ref().unwrap());
        from_ref.unwrap().Invoke().unwrap();
        let optional = InRef::<Tracked>::from(owner.as_ref());
        optional.unwrap().Invoke().unwrap();
        borrowed.cloned().unwrap()
    };
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(owner);
    cloned.Invoke().unwrap();
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(cloned);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert!(InRef::<Tracked>::from(&None).is_null());
    assert!(InRef::<Tracked>::from(None).is_null());
}

#[test]
fn parameter_value_lifetimes() {
    let drops = Arc::new(AtomicUsize::new(0));
    let value = tracked(&drops);
    let retained = {
        let borrowed: ParamValue<Tracked> = unsafe { (&value).param() };
        assert!(matches!(borrowed, ParamValue::Borrowed(_)));
        assert_eq!(borrowed.abi(), value.as_raw());
        borrowed.borrow().unwrap().Invoke().unwrap();
        let optional: ParamValue<Tracked> = unsafe { Some(&value).param() };
        optional.borrow().unwrap().Invoke().unwrap();
        borrowed.borrow().cloned().unwrap()
    };
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(value);
    let retained = {
        let owned = ParamValue::Owned(retained);
        owned.borrow().unwrap().Invoke().unwrap();
        owned.borrow().cloned().unwrap()
    };
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    retained.Invoke().unwrap();
    drop(retained);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    let absent: ParamValue<Tracked> = unsafe { None::<&Tracked>.param() };
    assert!(absent.borrow().is_null());
}

#[cfg(not(miri))]
#[test]
fn owning_record_abi_lifetimes() {
    let status = windows_core::HRESULT(0x80004005u32 as i32);
    for generic in [false, true] {
        for failure in [None, Some(false), Some(true)] {
            let input_drops = Arc::new(AtomicUsize::new(0));
            let old_drops = Arc::new(AtomicUsize::new(0));
            let input = OwnedValue {
                text: "borrowed\0text".into(),
                enabled: true,
                number: 1.25,
                object: Some(tracked(&input_drops)),
            };
            let mut output = OwnedValue {
                object: Some(tracked(&old_drops)),
                ..Default::default()
            };
            let invoke = move |input: &OwnedValue, output: &mut OwnedValue| {
                assert_eq!(input.text, "borrowed\0text");
                assert!(input.enabled);
                assert_eq!(input.number, 1.25);
                input.object.as_ref().unwrap().Invoke().unwrap();
                assert!(output.object.is_none());
                if failure != Some(false) {
                    *output = input.clone();
                }
                if failure.is_some() {
                    Err(windows_core::Error::from_hresult(status))
                } else {
                    Ok(input.clone())
                }
            };
            let result = if generic {
                GenericRoundtrip::<OwnedValue>::new(move |input, output| {
                    invoke(input.unwrap(), output)
                })
                .Invoke(&input, &mut output)
            } else {
                OwnedRoundtrip::new(invoke).Invoke(&input, &mut output)
            };
            assert_eq!(old_drops.load(Ordering::Relaxed), 1);
            assert_eq!(input_drops.load(Ordering::Relaxed), 0);
            assert_eq!(input.text, "borrowed\0text");
            if failure.is_some() {
                assert_eq!(result.as_ref().unwrap_err().code(), status);
            } else {
                assert_eq!(result.as_ref().unwrap(), &input);
            }
            drop(input);
            if failure == Some(false) {
                assert!(output.object.is_none());
                assert_eq!(input_drops.load(Ordering::Relaxed), 1);
            } else {
                output.object.as_ref().unwrap().Invoke().unwrap();
                assert_eq!(input_drops.load(Ordering::Relaxed), 0);
            }
            drop(output);
            assert_eq!(
                input_drops.load(Ordering::Relaxed),
                usize::from(failure.is_some())
            );
            drop(result);
            assert_eq!(input_drops.load(Ordering::Relaxed), 1);
        }
    }
}

#[test]
fn replace_array_output() {
    let old_drops = Arc::new(AtomicUsize::new(0));
    let new_drops = Arc::new(AtomicUsize::new(0));
    let capture = new_drops.clone();
    let producer = ArrayRoundtrip::new(move |_, _, receive| {
        *receive = Array::from_slice(&[Some(tracked(&capture))]);
        Ok(Array::new())
    });
    let mut output = Array::from_slice(&[Some(tracked(&old_drops))]);
    for generation in 0..2 {
        drop(producer.Invoke(&[], &mut [], &mut output).unwrap());
        assert_eq!(old_drops.load(Ordering::Relaxed), 1);
        assert_eq!(new_drops.load(Ordering::Relaxed), generation);
        output[0].as_ref().unwrap().Invoke().unwrap();
    }
    drop(output);
    assert_eq!(new_drops.load(Ordering::Relaxed), 2);

    let mut empty = Array::with_len(0);
    drop(producer.Invoke(&[], &mut [], &mut empty).unwrap());
    drop(empty);
    assert_eq!(new_drops.load(Ordering::Relaxed), 3);
}

#[test]
fn replace_fill_output() {
    let old_drops = Arc::new(AtomicUsize::new(0));
    let new_drops = Arc::new(AtomicUsize::new(0));
    let capture = new_drops.clone();
    let producer = ArrayRoundtrip::new(move |_, fill, _| {
        // Native output writers overwrite slots without releasing their previous contents.
        unsafe {
            fill.as_mut_ptr().write(Some(tracked(&capture)));
        }
        Ok(Array::new())
    });
    let mut fill = [Some(tracked(&old_drops)), Some(tracked(&old_drops)), None];
    let mut receive = Array::new();
    for generation in 0..2 {
        drop(producer.Invoke(&[], &mut fill, &mut receive).unwrap());
        assert_eq!(old_drops.load(Ordering::Relaxed), 2);
        assert_eq!(new_drops.load(Ordering::Relaxed), generation);
        fill[0].as_ref().unwrap().Invoke().unwrap();
        assert!(fill[1..].iter().all(Option::is_none));
    }
    drop(fill);
    assert_eq!(new_drops.load(Ordering::Relaxed), 2);
}

#[test]
fn replace_interface_output() {
    let old_drops = Arc::new(AtomicUsize::new(0));
    let output_drops = Arc::new(AtomicUsize::new(0));
    let result_drops = Arc::new(AtomicUsize::new(0));
    let output_capture = output_drops.clone();
    let result_capture = result_drops.clone();
    let producer = InterfaceOutputs::new(move |output| {
        output.write(Some(tracked(&output_capture)))?;
        Ok(tracked(&result_capture))
    });
    let mut output = Some(tracked(&old_drops));
    for generation in 0..2 {
        let result = producer.Invoke(&mut output).unwrap();
        assert_eq!(old_drops.load(Ordering::Relaxed), 1);
        assert_eq!(output_drops.load(Ordering::Relaxed), generation);
        assert_eq!(result_drops.load(Ordering::Relaxed), generation);
        result.Invoke().unwrap();
        drop(result);
    }
    output.as_ref().unwrap().Invoke().unwrap();
    drop(output);
    assert_eq!(output_drops.load(Ordering::Relaxed), 2);
    assert_eq!(result_drops.load(Ordering::Relaxed), 2);
}

#[cfg(not(miri))]
#[test]
fn generic_string_outputs() {
    let producer = GenericOutputs::<windows_core::HSTRING>::new(|fill, value| {
        unsafe {
            fill.as_mut_ptr().write("new fill".into());
            core::ptr::write(value, "new value".into());
        }
        Ok(())
    });
    let old: windows_core::HSTRING = "old".into();
    let mut fill = [old.clone(), old.clone()];
    let mut value = old.clone();
    producer.Invoke(&mut fill, &mut value).unwrap();
    assert_eq!(fill[0], "new fill");
    assert!(fill[1].is_empty());
    assert_eq!(value, "new value");
    assert_eq!(old, "old");
}

#[test]
fn replace_generic_outputs() {
    let old_drops = Arc::new(AtomicUsize::new(0));
    let new_drops = Arc::new(AtomicUsize::new(0));
    let capture = new_drops.clone();
    let producer = GenericOutputs::<Tracked>::new(move |fill, value| {
        unsafe {
            fill.as_mut_ptr().write(Some(tracked(&capture)));
            core::ptr::write(value, Some(tracked(&capture)));
        }
        Ok(())
    });
    let mut fill = [Some(tracked(&old_drops))];
    let mut value = Some(tracked(&old_drops));
    producer.Invoke(&mut fill, &mut value).unwrap();
    assert_eq!(old_drops.load(Ordering::Relaxed), 2);
    assert_eq!(new_drops.load(Ordering::Relaxed), 0);
    fill[0].as_ref().unwrap().Invoke().unwrap();
    value.as_ref().unwrap().Invoke().unwrap();
    drop(fill);
    drop(value);
    assert_eq!(new_drops.load(Ordering::Relaxed), 2);

    let producer = GenericOutputs::<u32>::new(|fill, value| {
        fill[0] = 10;
        *value = 20;
        Ok(())
    });
    let mut fill = [1, 2];
    let mut value = 3;
    producer.Invoke(&mut fill, &mut value).unwrap();
    assert_eq!(fill, [10, 2]);
    assert_eq!(value, 20);
}

#[cfg(not(miri))]
#[test]
fn replace_outputs_on_failure() {
    let status = windows_core::HRESULT(0x80004005u32 as i32);
    for write_output in [false, true] {
        let old_drops = Arc::new(AtomicUsize::new(0));
        let new_drops = Arc::new(AtomicUsize::new(0));
        let capture = new_drops.clone();
        let producer = InterfaceOutputs::new(move |output| {
            if write_output {
                output.write(Some(tracked(&capture)))?;
            }
            Err(windows_core::Error::from_hresult(status))
        });
        let mut output = Some(tracked(&old_drops));
        assert_eq!(producer.Invoke(&mut output).unwrap_err().code(), status);
        assert_eq!(old_drops.load(Ordering::Relaxed), 1);
        assert_eq!(output.is_some(), write_output);
        assert_eq!(new_drops.load(Ordering::Relaxed), 0);
        drop(output);
        assert_eq!(new_drops.load(Ordering::Relaxed), usize::from(write_output));

        let capture = new_drops.clone();
        let producer = ArrayRoundtrip::new(move |_, fill, receive| {
            if write_output {
                unsafe {
                    fill.as_mut_ptr().write(Some(tracked(&capture)));
                }
                *receive = Array::from_slice(&[Some(tracked(&capture))]);
            }
            Err(windows_core::Error::from_hresult(status))
        });
        let mut fill = [Some(tracked(&old_drops))];
        let mut receive = Array::from_slice(&[Some(tracked(&old_drops))]);
        assert_eq!(
            producer
                .Invoke(&[], &mut fill, &mut receive)
                .unwrap_err()
                .code(),
            status
        );
        assert_eq!(old_drops.load(Ordering::Relaxed), 3);
        assert_eq!(fill[0].is_some(), write_output);
        assert_eq!(!receive.is_empty(), write_output);
        assert_eq!(new_drops.load(Ordering::Relaxed), usize::from(write_output));
        drop(fill);
        drop(receive);
        assert_eq!(
            new_drops.load(Ordering::Relaxed),
            3 * usize::from(write_output)
        );
    }
}

#[test]
fn empty_array_clear() {
    let allocated = unsafe {
        let data = windows_core::imp::CoTaskMemAlloc(4).cast::<u32>();
        assert!(!data.is_null());
        Array::from_raw_parts(data, 0)
    };
    for mut array in [
        Array::<u32>::new(),
        Array::with_len(0),
        Array::from_slice(&[]),
        allocated,
    ] {
        array.clear();
        array.clear();
        let (data, len) = array.into_abi();
        assert_eq!(len, 0);
        assert!(data.is_null());
    }
}

#[test]
fn empty_array_return() {
    let producer = EmptyArray::new(|| Ok(Array::with_len(0)));
    let result = producer.Invoke().unwrap();
    assert!(result.is_empty());
    drop(result);
}

#[test]
fn empty_array_parameters() {
    let producer = ArrayRoundtrip::new(|input, fill, receive| {
        assert!(input.is_empty());
        assert!(fill.is_empty());
        *receive = Array::new();
        Ok(Array::new())
    });
    let mut receive = Array::new();
    let result = producer.Invoke(&[], &mut [], &mut receive).unwrap();
    assert!(receive.is_empty());
    assert!(result.is_empty());
}

#[cfg(not(miri))]
#[test]
fn array_return_failure() {
    let status = windows_core::HRESULT(0x80004005u32 as i32);
    let producer = EmptyArray::new(move || Err(windows_core::Error::from_hresult(status)));
    assert_eq!(producer.Invoke().unwrap_err().code(), status);
}

// Native runs use the real COM allocator. Miri uses matching allocation/free exports.
#[cfg(miri)]
mod allocator {
    use std::alloc::{Layout, alloc, dealloc};
    use std::ffi::c_void;

    const ALIGN: usize = 2 * size_of::<usize>();

    #[unsafe(export_name = "CoTaskMemAlloc")]
    unsafe extern "system" fn allocate(size: usize) -> *mut c_void {
        let layout = Layout::from_size_align(size.checked_add(ALIGN).unwrap(), ALIGN).unwrap();
        unsafe {
            let allocation = alloc(layout);
            if allocation.is_null() {
                return core::ptr::null_mut();
            }
            allocation.cast::<usize>().write(layout.size());
            allocation.add(ALIGN).cast()
        }
    }

    #[unsafe(export_name = "CoTaskMemFree")]
    unsafe extern "system" fn free(value: *mut c_void) {
        if !value.is_null() {
            unsafe {
                let allocation = value.cast::<u8>().sub(ALIGN);
                let size = allocation.cast::<usize>().read();
                dealloc(allocation, Layout::from_size_align(size, ALIGN).unwrap());
            }
        }
    }
}
