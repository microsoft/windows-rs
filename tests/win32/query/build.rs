fn main() {
    windows::build! {
        Component::Win32::Query::*,
        Windows::Win32::Foundation::E_NOINTERFACE,
    };
}
