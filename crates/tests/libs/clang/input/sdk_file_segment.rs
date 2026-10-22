const _: () = {
    assert!(core::mem::size_of::<FILE_SEGMENT_ELEMENT>() == 8);
    assert!(core::mem::align_of::<FILE_SEGMENT_ELEMENT>() == 8);
    assert!(core::mem::offset_of!(FILE_SEGMENT_ELEMENT, Buffer) == 0);
    assert!(core::mem::size_of::<PVOID64>() == 8);
    assert!(core::mem::align_of::<PVOID64>() == 8);
};

fn main() {
    let callback: SegmentConsumer = None;
    let _: Option<unsafe extern "C" fn(*mut FILE_SEGMENT_ELEMENT)> = callback;
    for address in [0, 0x1234_5678_9abc_def0, u64::MAX] {
        let value = FILE_SEGMENT_ELEMENT { Buffer: PVOID64::from_address(address) };
        assert_eq!(unsafe { value.Alignment }, address);
        assert_eq!(unsafe { value.Buffer }.address(), address);
    }
    let _: __windows_bindgen_Pointer64<*mut core::ffi::c_void> = PVOID64::default();
}
