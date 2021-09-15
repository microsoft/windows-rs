fn main() {
    windows::build! {
        Component::Collections::*,
        // TODO: generic dependencies must be included automatically! Rust compiler errors are horrific if we don't.
        Windows::Foundation::Collections::*,
        Windows::Foundation::IStringable,
    };
}
