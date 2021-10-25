fn main() {
    // GetProcessHeap returns HeapHandle. So including GetProcessHeap
    // should also include HeapHandle.
    windows::runtime::build! {
        Windows::Win32::System::Memory::GetProcessHeap,
    };
    // Note: don't add anything else to this build macro!
}
