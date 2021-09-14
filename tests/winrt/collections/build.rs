fn main() {
    windows::build! {
        Component::Collections::*,
        // TODO: generic dependencies be included automatically
        Windows::Foundation::Collections::IVector,
        Windows::Foundation::IStringable,
    };
}
