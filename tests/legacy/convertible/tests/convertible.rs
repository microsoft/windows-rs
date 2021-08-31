use test_convertible::Windows::Win32::System::Memory::{GetProcessHeap, HeapHandle};

#[test]
fn test() {
    unsafe {
        let _: HeapHandle = GetProcessHeap();
    }
}
