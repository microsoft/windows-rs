fn main() {
    windows::build! {
        Component::Signatures::*,

        // Only used Windows metadata for constants.
        Windows::Win32::Foundation::{E_NOINTERFACE, S_OK, S_FALSE, E_POINTER},
    };
}
