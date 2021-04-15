fn main() {
    windows::build!(
        // The Windows crate manually injects various functions needed to implement BSTR.
        // This test validates these are included.
        Windows::Win32::Automation::BSTR
    );
}
