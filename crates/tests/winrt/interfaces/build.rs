fn main() {
    windows::runtime::build! {Component::Interfaces::*, Windows::Win32::Foundation::E_NOINTERFACE};
}
