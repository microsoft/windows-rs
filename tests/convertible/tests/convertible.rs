use test_convertible::Windows::Win32::SystemServices::{
    GetProcessHeap, HeapHandle, ProcessHeapHandle,
};

#[test]
fn test() {
    unsafe {
        let _: ProcessHeapHandle = GetProcessHeap();
        let _: HeapHandle = HeapHandle::default();
    }
}
