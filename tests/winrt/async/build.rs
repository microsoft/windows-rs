fn main() {
    windows::build! {
        Component::Async::*,

        // TODO: generic dependencies be included automatically
        Windows::Foundation::*,
    };
}
