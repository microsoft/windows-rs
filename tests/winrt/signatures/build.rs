fn main() {
    windows::build! {
        Component::Signatures::*,
        Component::Simple::Class,
        Windows::Win32::Foundation::{E_NOINTERFACE, E_POINTER, S_FALSE, S_OK},
        Windows::Foundation::PropertyValue,
    };
}
