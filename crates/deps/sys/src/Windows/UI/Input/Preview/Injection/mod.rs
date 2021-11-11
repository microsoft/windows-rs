#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IInjectedInputGamepadInfo();
    fn IInjectedInputGamepadInfoFactory();
    fn IInjectedInputKeyboardInfo();
    fn IInjectedInputMouseInfo();
    fn IInjectedInputPenInfo();
    fn IInjectedInputTouchInfo();
    fn IInputInjector();
    fn IInputInjector2();
    fn IInputInjectorStatics();
    fn IInputInjectorStatics2();
    fn InjectedInputButtonChangeKind();
    fn InjectedInputGamepadInfo();
    fn InjectedInputKeyOptions();
    fn InjectedInputKeyboardInfo();
    fn InjectedInputMouseInfo();
    fn InjectedInputMouseOptions();
    fn InjectedInputPenButtons();
    fn InjectedInputPenInfo();
    fn InjectedInputPenParameters();
    fn InjectedInputPoint();
    fn InjectedInputPointerInfo();
    fn InjectedInputPointerOptions();
    fn InjectedInputRectangle();
    fn InjectedInputShortcut();
    fn InjectedInputTouchInfo();
    fn InjectedInputTouchParameters();
    fn InjectedInputVisualizationMode();
    fn InputInjector();
}
