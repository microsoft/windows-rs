fn main() {
    windows::build!(
        Windows::Foundation::{IStringable, IClosable},
        // TODO: Dependency tracking is a little too conservative for the implement macro at the moment.
        Windows::ApplicationModel::Activation::*,
        Windows::UI::Xaml::*,
    );
}
