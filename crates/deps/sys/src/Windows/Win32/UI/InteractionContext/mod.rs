#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AddPointerInteractionContext();
    fn BufferPointerPacketsInteractionContext();
    fn CreateInteractionContext();
    fn DestroyInteractionContext();
    fn GetCrossSlideParameterInteractionContext();
    fn GetHoldParameterInteractionContext();
    fn GetInertiaParameterInteractionContext();
    fn GetInteractionConfigurationInteractionContext();
    fn GetMouseWheelParameterInteractionContext();
    fn GetPropertyInteractionContext();
    fn GetStateInteractionContext();
    fn GetTapParameterInteractionContext();
    fn GetTranslationParameterInteractionContext();
    fn ProcessBufferedPacketsInteractionContext();
    fn ProcessInertiaInteractionContext();
    fn ProcessPointerFramesInteractionContext();
    fn RegisterOutputCallbackInteractionContext();
    fn RegisterOutputCallbackInteractionContext2();
    fn RemovePointerInteractionContext();
    fn ResetInteractionContext();
    fn SetCrossSlideParametersInteractionContext();
    fn SetHoldParameterInteractionContext();
    fn SetInertiaParameterInteractionContext();
    fn SetInteractionConfigurationInteractionContext();
    fn SetMouseWheelParameterInteractionContext();
    fn SetPivotInteractionContext();
    fn SetPropertyInteractionContext();
    fn SetTapParameterInteractionContext();
    fn SetTranslationParameterInteractionContext();
    fn StopInteractionContext();
}
