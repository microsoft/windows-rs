fn main() {
    windows::build! {
        // GetProcessHeap returns HeapHandle. So including GetProcessHeap
        // should also include HeapHandle.
        Windows::Win32::System::Memory::GetProcessHeap,
        // Note: don't add anything else to this build macro!
    };
}
