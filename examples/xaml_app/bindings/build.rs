fn main() {
    windows::build! {
        Windows::ApplicationModel::Activation::*,
        Windows::UI::Xaml::*,
        Windows::UI::Xaml::Controls::TextBox,
    };
}
