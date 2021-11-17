fn main() {
    windows::core::build_legacy! {
        Component::Classes::{Activatable, NoWeakRef},
        Windows::Foundation::IStringable,
        Windows::Win32::Foundation::E_NOINTERFACE,
        Windows::Win32::System::WinRT::{IWeakReference, IWeakReferenceSource},
    };
}
