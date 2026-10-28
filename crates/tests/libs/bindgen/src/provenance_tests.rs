use super::optional_buffers::*;
use core::ffi::c_void;
use std::cell::Cell;
use windows_core::Interface;

unsafe fn write_values<T: Copy>(values: *mut T, count: u32, value: T) -> u32 {
    if values.is_null() {
        assert_eq!(count, 0);
        0
    } else {
        for index in 0..count as usize {
            unsafe { values.add(index).write(value) };
        }
        1
    }
}

unsafe extern "system" fn elements(_: *mut c_void, values: *mut u32, count: u32) -> u32 {
    unsafe { write_values(values, count, 42) }
}

unsafe extern "system" fn count_first(_: *mut c_void, count: u32, values: *mut u32) -> u32 {
    unsafe { write_values(values, count, 42) }
}

unsafe extern "system" fn bytes(_: *mut c_void, values: *mut i8, count: u32) -> u32 {
    unsafe { write_values(values, count, 42) }
}

unsafe extern "system" fn fixed(_: *mut c_void, values: *mut u32) -> u32 {
    unsafe { write_values(values, if values.is_null() { 0 } else { 4 }, 42) }
}

unsafe extern "system" fn input(_: *mut c_void, values: *const u32, count: u32) -> u32 {
    if values.is_null() {
        assert_eq!(count, 0);
        0
    } else {
        for index in 0..count as usize {
            assert_eq!(unsafe { *values.add(index) }, 42);
        }
        1
    }
}

fn buffers() -> Buffers {
    // Keep buffer regressions independent of scoped-interface construction.
    static VTABLE: Buffers_Vtbl = Buffers_Vtbl {
        ElementsAfter: elements,
        ElementsBefore: count_first,
        Bytes: bytes,
        Fixed: fixed,
        Modern: elements,
        Input: input,
        Output: elements,
        Required: elements,
    };
    static OBJECT: &Buffers_Vtbl = &VTABLE;
    unsafe { Buffers::from_raw((&raw const OBJECT).cast_mut().cast()) }
}

#[test]
fn optional_elements() {
    let buffers = buffers();
    let methods: [unsafe fn(&Buffers, Option<&mut [u32]>) -> u32; 3] = [
        Buffers::ElementsAfter,
        Buffers::ElementsBefore,
        Buffers::Modern,
    ];
    for method in methods {
        let mut values = [0; 4];
        unsafe {
            assert_eq!(method(&buffers, Some(&mut values)), 1);
            assert_eq!(method(&buffers, None), 0);
            assert_eq!(method(&buffers, Some(&mut [])), 1);
        }
        assert_eq!(values, [42; 4]);
    }
}

#[test]
fn optional_bytes() {
    let buffers = buffers();
    let mut values = [0; 4];
    unsafe {
        assert_eq!(buffers.Bytes(Some(&mut values)), 1);
        assert_eq!(buffers.Bytes(None), 0);
        assert_eq!(buffers.Bytes(Some(&mut [])), 1);
    }
    assert_eq!(values, [42; 4]);
}

#[test]
fn optional_fixed() {
    let buffers = buffers();
    let mut values = [0; 4];
    unsafe {
        assert_eq!(buffers.Fixed(Some(&mut values)), 1);
        assert_eq!(buffers.Fixed(None), 0);
    }
    assert_eq!(values, [42; 4]);
}

#[test]
fn buffer_controls() {
    let buffers = buffers();
    let mut values = [0; 4];
    unsafe {
        assert_eq!(buffers.Required(&mut values), 1);
        assert_eq!(values, [42; 4]);
        assert_eq!(buffers.Input(Some(&values)), 1);
        assert_eq!(buffers.Input(None), 0);
        assert_eq!(buffers.Input(Some(&[])), 1);
        values.fill(0);
        assert_eq!(buffers.Output(Some(values.as_mut_ptr()), 4), 1);
        assert_eq!(buffers.Output(None, 0), 0);
    }
    assert_eq!(values, [42; 4]);
}

#[test]
fn scoped_interface() {
    struct Object<'a> {
        calls: Cell<u32>,
        dropped: &'a Cell<bool>,
    }

    impl Scoped_Impl for Object<'_> {
        fn Value(&self) -> u32 {
            self.calls.set(self.calls.get() + 1);
            self.calls.get()
        }
    }

    impl Drop for Object<'_> {
        fn drop(&mut self) {
            self.dropped.set(true);
        }
    }

    let dropped = Cell::new(false);
    let object = Object {
        calls: Cell::new(0),
        dropped: &dropped,
    };
    {
        let interface = Scoped::new(&object);
        assert_eq!(unsafe { interface.Value() }, 1);
        assert_eq!(unsafe { interface.Value() }, 2);
    }
    assert_eq!(object.calls.get(), 2);
    assert!(!dropped.get());
    drop(object);
    assert!(dropped.get());
}
