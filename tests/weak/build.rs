fn main() {
    windows::build!(
        Windows::Foundation::Uri,
        Windows::Win32::WinRT::{IWeakReference, IWeakReferenceSource},
    );
}
