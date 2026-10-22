fn main() {
    let value = Address64::from_address(0xfedc_ba98_7654_3210);
    assert_eq!(value.address(), 0xfedc_ba98_7654_3210);
    assert_eq!(Address64::default().address(), 0);
    let _: __windows_bindgen_Pointer64<*mut Element> = value;
    let _: __windows_bindgen_Pointer64<*const Element> = ConstAddress64::default();
    let mut segment = Segment { alignment: 0 };
    segment.buffer = __windows_bindgen_Pointer64::from_address(0x1234_5678_9abc_def0);
    assert_eq!(unsafe { segment.alignment }, 0x1234_5678_9abc_def0);
    let value = Addresses::default();
    let _: *mut Address64 = value.storage;
    let _: *const Address64 = value.constant_storage;
    let _: *mut __windows_bindgen_Pointer64<*const i32> = value.mixed_outer;
    let _: __windows_bindgen_Pointer64<*mut *mut i32> = value.mixed_inner;
    let _: __windows_bindgen_Pointer64<*mut Opaque> = value.opaque;
    let _: [Address64; 2] = value.array;
    let copied = value.opaque;
    assert_eq!(copied, value.opaque);
    assert_eq!(copied.address(), 0);
    assert_eq!(core::mem::size_of::<Address64>(), 8);
    assert_eq!(core::mem::align_of::<Address64>(), 8);
}
