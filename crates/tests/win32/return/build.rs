fn main() {
    windows::core::build! {
        Component::Win32::Return::*,
        Windows::Win32::Foundation::{E_APPLICATION_EXITING, STATUS_NOT_FOUND, STATUS_SUCCESS, S_OK},
    };
}
