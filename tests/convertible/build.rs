fn main() {
    windows::build!(
        // GetProcessHeap returns ProcessHeapHandle that is convertible to HeapHandle. So including ProcessHeapHandle
        // (by virtue of GetProcessHeap) should also include HeapHandle.
        Windows::Win32::SystemServices::GetProcessHeap,
    );
}
