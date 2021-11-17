fn main() {
    windows::core::build_legacy! {
        Component::Win32::MethodNames::*,
    };
}
