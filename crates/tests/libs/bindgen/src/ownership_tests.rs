use super::ownership::*;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use windows_core::{Array, Interface};

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
