#![cfg(windows)]

use std::cell::Cell;
use std::ffi::c_void;
use windows::core::*;

#[interface("ce1a8805-5f4f-41db-8257-a9420586a5c2")]
unsafe trait IPacked: IUnknown {
    unsafe fn read(&self) -> u32;
}

#[repr(C, packed)]
struct Packed {
    vtable: *const IPacked_Vtbl,
    refs: *const imp::WeakRefCount,
    remaining: *const Cell<u32>,
}

#[repr(C, align(16))]
struct Storage {
    prefix: u8,
    object: Packed,
}

unsafe extern "system" fn query(
    this: *mut c_void,
    iid: *const GUID,
    result: *mut *mut c_void,
) -> HRESULT {
    unsafe {
        if *iid == IUnknown::IID || *iid == IPacked::IID {
            *result = this;
            add_ref(this);
            HRESULT(0)
        } else {
            *result = (*(*this.cast::<Packed>()).refs).query(&*iid, this);
            if (*result).is_null() {
                HRESULT(0x80004002_u32 as i32)
            } else {
                HRESULT(0)
            }
        }
    }
}

unsafe extern "system" fn add_ref(this: *mut c_void) -> u32 {
    let refs = unsafe { &*(*this.cast::<Packed>()).refs };
    let count = refs.add_ref();
    unsafe { (*(*this.cast::<Packed>()).remaining).set(count) };
    count
}

unsafe extern "system" fn release(this: *mut c_void) -> u32 {
    let refs = unsafe { &*(*this.cast::<Packed>()).refs };
    let count = refs.release();
    unsafe { (*(*this.cast::<Packed>()).remaining).set(count) };
    count
}

unsafe extern "system" fn read(this: *mut c_void) -> u32 {
    unsafe { (*(*this.cast::<Packed>()).remaining).get() }
}

static VTABLE: IPacked_Vtbl = IPacked_Vtbl {
    base__: IUnknown_Vtbl {
        QueryInterface: query,
        AddRef: add_ref,
        Release: release,
    },
    read,
};

#[test]
fn packed_com_object() {
    let refs = imp::WeakRefCount::new();
    let remaining = Cell::new(1);
    let mut storage = Storage {
        prefix: 0,
        object: Packed {
            vtable: &VTABLE,
            refs: &refs,
            remaining: &remaining,
        },
    };
    let raw = (&raw mut storage.object).cast::<c_void>();
    assert!(
        !raw.addr()
            .is_multiple_of(align_of::<*const IUnknown_Vtbl>())
    );
    let object = unsafe { IPacked::from_raw(raw) };
    assert!(std::ptr::eq(object.vtable(), &VTABLE));
    assert_eq!(unsafe { object.read() }, 1);
    assert!(std::ptr::eq(
        unsafe { object.assume_vtable::<IUnknown>() },
        &VTABLE.base__
    ));
    let clone = object.clone();
    let queried = clone.cast::<IUnknown>().unwrap();
    assert_eq!(remaining.get(), 3);
    drop(queried);
    drop(clone);
    let source = object.cast::<imp::IWeakReferenceSource>().unwrap();
    drop(source.cast::<IUnknown>().unwrap());
    drop(source);
    let weak = object.downgrade().unwrap();
    let upgraded = weak.upgrade().unwrap();
    assert_eq!(upgraded.as_raw(), raw);
    drop(upgraded);
    assert_eq!(remaining.get(), 1);
    drop(object);
    assert_eq!(remaining.get(), 0);
    assert!(weak.upgrade().is_none());
}
