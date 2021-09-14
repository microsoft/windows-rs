fn main() {
    windows::build! {
        Component::Collections::*,
        Windows::Foundation::IStringable,

        // TODO: should generic dependencies be included automatically?
        Windows::Foundation::Collections::IVector,
    };
}
