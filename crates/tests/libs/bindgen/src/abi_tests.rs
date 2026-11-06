use super::abi::*;
use core::{ffi::c_void, mem::MaybeUninit, ptr::null_mut};

#[test]
fn optional_output_buffer() {
    let mut values = [99; 4];
    let mut written = MaybeUninit::uninit();
    unsafe {
        assert_eq!(
            AbiFill(Some(values.as_mut_ptr()), 2, written.as_mut_ptr()),
            1
        );
        assert_eq!(written.assume_init(), 2);
    }
    assert_eq!(values, [10, 11, 99, 99]);

    for values in [None, Some(null_mut())] {
        for count in [0, 17] {
            let mut written = MaybeUninit::uninit();
            unsafe {
                assert_eq!(AbiFill(values, count, written.as_mut_ptr()), 0);
                assert_eq!(written.assume_init(), count);
            }
        }
    }

    let mut empty = [];
    let mut written = MaybeUninit::uninit();
    unsafe {
        assert_eq!(
            AbiFill(Some(empty.as_mut_ptr()), 0, written.as_mut_ptr()),
            1
        );
        assert_eq!(written.assume_init(), 0);
    }
}

#[test]
fn optional_input_buffer() {
    let values = [10, 20, 30];
    unsafe {
        assert_eq!(AbiSum(2, Some(values.as_ptr())), 30);
        assert_eq!(AbiSum(17, None), 0);
        assert_eq!(AbiSum(17, Some(core::ptr::null())), 0);
        assert_eq!(AbiSum(0, Some([].as_ptr())), 0);
    }
}

#[test]
fn returned_pointer() {
    let mut values = [10, 20, 30];
    let base = values.as_mut_ptr();
    let mut selected = MaybeUninit::uninit();
    unsafe {
        assert_eq!(AbiSelect(base, 2, selected.as_mut_ptr()), 1);
        let selected = selected.assume_init();
        assert_eq!(selected, base.add(1));
        selected.write(42);
    }
    assert_eq!(values, [10, 42, 30]);

    let mut selected = MaybeUninit::uninit();
    unsafe {
        assert_eq!(AbiSelect(null_mut(), 0, selected.as_mut_ptr()), 0);
        assert!(selected.assume_init().is_null());
    }
}

struct Context {
    values: *mut u32,
    calls: u32,
}

unsafe fn callback(context: *mut c_void, values: *mut u32, count: u32) -> u64 {
    unsafe {
        let context = &mut *context.cast::<Context>();
        assert_eq!(values, context.values);
        context.calls += 1;
        let values = core::slice::from_raw_parts_mut(values, count as usize);
        for value in values.iter_mut() {
            *value += 1;
        }
        (u64::from(context.calls) << 32) | u64::from(values.iter().sum::<u32>())
    }
}

unsafe extern "system" fn system_callback(
    context: *mut c_void,
    values: *mut u32,
    count: u32,
) -> u64 {
    unsafe { callback(context, values, count) }
}

unsafe extern "C" fn cdecl_callback(context: *mut c_void, values: *mut u32, count: u32) -> u64 {
    unsafe { callback(context, values, count) }
}

#[test]
fn callback_roundtrip() {
    let mut values = [10, 20, 30];
    let mut context = Context {
        values: values.as_mut_ptr(),
        calls: 0,
    };
    unsafe {
        assert_eq!(
            AbiInvoke(
                Some(system_callback),
                (&raw mut context).cast(),
                context.values,
                2
            ),
            (1 << 32) | 32
        );
        assert_eq!(
            AbiInvokeCdecl(
                Some(cdecl_callback),
                (&raw mut context).cast(),
                context.values,
                2
            ),
            (2 << 32) | 34
        );
    }
    assert_eq!(values, [12, 22, 30]);
    assert_eq!(context.calls, 2);

    let mut empty = [];
    context.values = empty.as_mut_ptr();
    unsafe {
        assert_eq!(
            AbiInvoke(
                Some(system_callback),
                (&raw mut context).cast(),
                context.values,
                0
            ),
            3 << 32
        );
        assert_eq!(AbiInvoke(None, null_mut(), null_mut(), 17), 0);
        assert_eq!(AbiInvokeCdecl(None, null_mut(), null_mut(), 17), 0);
    }
    assert_eq!(context.calls, 3);
}
