fn main() {
    windows::runtime::build! {
        StaticComponent::Win32::Simple::*,
    };
}
