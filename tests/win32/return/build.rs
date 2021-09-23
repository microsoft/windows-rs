fn main() {
    windows::build! {
        Component::Win32::Return::*,
        Windows::Win32::Foundation::{S_OK, E_APPLICATION_EXITING, STATUS_NOT_FOUND, STATUS_SUCCESS},
    };
}
