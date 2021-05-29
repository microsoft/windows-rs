fn main() {
    windows::build! {
        Windows::ApplicationModel::Activation::*, Windows::UI::Xaml::Controls::TextBox,
        Windows::UI::Xaml::*,
    };
}
