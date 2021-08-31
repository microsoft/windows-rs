fn main() {
    windows::build! {
        Windows::Foundation::Uri,
        Windows::Win32::{
            Foundation::E_NOINTERFACE,
            System::WinRT::{IWeakReference, IWeakReferenceSource},
        },
    };
}
