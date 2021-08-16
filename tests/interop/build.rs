fn main() {
    windows::build! {
        Windows::Foundation::Collections::StringMap, Windows::Win32::System::Com::CoInitializeEx,
        Windows::Win32::System::WinRT::RoActivateInstance,
        Windows::Win32::UI::RadialInput::IRadialControllerInterop,
        Windows::Win32::AI::MachineLearning::WinML::IWinMLEvaluationContext,
    };
}
