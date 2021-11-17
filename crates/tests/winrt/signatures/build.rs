fn main() {
    windows::core::build_legacy! {
        Component::Signatures::*,
        Component::Simple::Class,
        Windows::Foundation::PropertyValue,
        Windows::Win32::Foundation::{E_NOINTERFACE, E_POINTER, S_FALSE, S_OK},
    };
}
