fn main() {
    windows::build! {
        // TODO: Dependency tracking is a little too conservative for the implement macro at the moment.
        Windows::ApplicationModel::Activation::*,
        Windows::Foundation::{IClosable, IStringable},
        Windows::UI::Xaml::*,
    };
}
