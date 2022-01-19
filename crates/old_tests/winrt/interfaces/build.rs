fn main() {
    windows::core::build_legacy! {Component::Interfaces::*, Windows::Win32::Foundation::E_NOINTERFACE};
}
