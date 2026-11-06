use super::struct_packed::*;
use core::mem::MaybeUninit;

#[test]
fn union_bitfields() {
    let mut value = super::struct_bitfield::Partial { bits: 0x1234_5678 };
    unsafe {
        assert_eq!(value.Low(), 8);
        value.set_Low(3);
        assert_eq!(value.Low(), 3);
        assert_eq!(value.bits, 0x1234_5673);
    }
}

#[test]
fn packed_fields() {
    #[repr(C, align(8))]
    struct Storage(Packed);

    let mut values = [10, 20];
    let mut storage = Storage(Packed {
        tag: 42,
        bits: 0x4000,
        pointer: values.as_mut_ptr(),
    });
    let record = &mut storage.0;
    assert!(!(&raw const record.bits).is_aligned());
    assert!(!(&raw const record.pointer).is_aligned());
    record.set_Delta(-3);
    record.set_Flag(true);
    let copy = *record;
    assert_eq!((copy.Delta(), copy.Flag()), (-3, true));
    assert_eq!({ copy.bits }, 0x403d);
    assert_eq!(copy.tag, 42);
    unsafe {
        copy.pointer.add(1).write(30);
    }
    assert_eq!(values, [10, 30]);
}

#[test]
fn mixed_fields() {
    let mut values = [10, 20];
    let mut record = Mixed {
        tag: 42,
        bits: __windows_bindgen_Unaligned::new(0x4000),
        pointer: __windows_bindgen_Unaligned::new(values.as_mut_ptr()),
        nested: __windows_bindgen_Unaligned::new(Padded { tag: 7, value: 11 }),
    };
    assert!(!(&raw const record.bits).cast::<i32>().is_aligned());
    assert!(!(&raw const record.pointer).cast::<*mut u32>().is_aligned());
    assert!(!(&raw const record.nested).cast::<Padded>().is_aligned());
    unsafe {
        record.set_Delta(-3);
        record.set_Flag(true);
    }
    #[expect(clippy::clone_on_copy)]
    let copy = record.clone();
    unsafe {
        assert_eq!((copy.Delta(), copy.Flag()), (-3, true));
        assert_eq!(copy.bits.read(), 0x403d);
        let pointer = copy.pointer.read();
        assert_eq!(pointer, record.pointer.read());
        pointer.add(1).write(30);
        assert_eq!(copy.nested.read(), Padded { tag: 7, value: 11 });
        record.nested.write(Padded { tag: 8, value: 12 });
        assert_eq!(record.nested.read(), Padded { tag: 8, value: 12 });
        assert_eq!(copy.nested.read(), Padded { tag: 7, value: 11 });
    }
    assert_eq!(copy.tag, 42);
    assert_eq!(values, [10, 30]);
}

#[test]
fn mixed_union() {
    #[repr(C, align(8))]
    struct Storage {
        prefix: u16,
        value: MixedUnion,
    }

    let mut values = [10, 20];
    let mut storage = Storage {
        prefix: 0,
        value: MixedUnion {
            pointer: __windows_bindgen_Unaligned::new(values.as_mut_ptr()),
        },
    };
    let record = &mut storage.value;
    assert!(!(&raw const record.bits).cast::<i32>().is_aligned());
    assert!(!(&raw const record.pointer).cast::<*mut u32>().is_aligned());
    let copy = *record;
    unsafe {
        copy.pointer.read().add(1).write(30);
        record.bits = __windows_bindgen_Unaligned::new(0x4000);
        record.set_Delta(-3);
        assert_eq!(record.Delta(), -3);
        assert_eq!(record.bits.read(), 0x401d);
    }
    assert_eq!(values, [10, 30]);
}

#[test]
fn nested_uninitialized_padding() {
    use super::struct_nested_member_alignment::*;

    let mut child = MaybeUninit::<Named_Child>::uninit();
    let child = unsafe {
        (&raw mut (*child.as_mut_ptr()).first).write(7);
        (&raw mut (*child.as_mut_ptr()).second).write(11);
        child.assume_init()
    };
    let matrix = Matrix {
        values: [[child; 2]; 2],
    };
    #[expect(clippy::clone_on_copy)]
    let copy = matrix.clone();
    for row in copy.values {
        for child in row {
            assert_eq!((child.first, child.second), (7, 11));
        }
    }
}

#[test]
fn union_copies() {
    let mut values = [10u8, 20];
    let value = super::union::Value {
        p: values.as_mut_ptr(),
    };
    let copy = value;
    unsafe {
        copy.p.add(1).write(30);
    }
    assert_eq!(values, [10, 30]);

    let value = super::union_field::Outer {
        kind: 1,
        value: super::union_field::Inner { offset: 42 },
    };
    let copy = value.clone();
    assert_eq!(copy.kind, value.kind);
    assert_eq!(unsafe { copy.value.offset }, 42);
}
