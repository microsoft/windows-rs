fn main() {
    windows::build!(
        Windows::ApplicationModel::Core::*,
        Windows::UI::*,
        Windows::UI::Core::*,
        Windows::ApplicationModel::Activation::*,
        Windows::UI::Xaml::Controls::TextBox,
        Windows::UI::Xaml::*,
    );
}
