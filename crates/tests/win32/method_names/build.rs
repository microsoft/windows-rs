fn main() {
    windows::runtime::build! {
        Component::Win32::MethodNames::*,
    };
}
