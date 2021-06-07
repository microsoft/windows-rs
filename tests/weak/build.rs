fn main() {
    windows::build! {
        Windows::Foundation::Uri,
        Windows::Win32::System::WinRT::{IWeakReference, IWeakReferenceSource},
    };
}
