fn main() {
    // The Windows crate manually injects various functions needed to implement BSTR.
    // This test validates these are included.
    windows::core::build! {
        Windows::Win32::Foundation::BSTR,
    };
}
