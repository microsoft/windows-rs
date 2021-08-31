fn main() {
    windows::build! {
        // Tests for SignatureKind::ReturnStruct

        // This free function returns a struct
        Windows::Win32::Graphics::Direct2D::D2D1ConvertColorSpace,

        // This interface has a `GetBindingProperties` method that returns a struct
        Windows::Win32::AI::MachineLearning::DirectML::IDMLDispatchable,
    };
}
