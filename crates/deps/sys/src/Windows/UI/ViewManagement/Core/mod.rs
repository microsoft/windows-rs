#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CoreFrameworkInputView();
    fn CoreFrameworkInputViewAnimationStartingEventArgs();
    fn CoreFrameworkInputViewOcclusionsChangedEventArgs();
    fn CoreInputView();
    fn CoreInputViewAnimationStartingEventArgs();
    fn CoreInputViewHidingEventArgs();
    fn CoreInputViewKind();
    fn CoreInputViewOcclusion();
    fn CoreInputViewOcclusionKind();
    fn CoreInputViewOcclusionsChangedEventArgs();
    fn CoreInputViewShowingEventArgs();
    fn CoreInputViewTransferringXYFocusEventArgs();
    fn CoreInputViewXYFocusTransferDirection();
    fn ICoreFrameworkInputView();
    fn ICoreFrameworkInputViewAnimationStartingEventArgs();
    fn ICoreFrameworkInputViewOcclusionsChangedEventArgs();
    fn ICoreFrameworkInputViewStatics();
    fn ICoreInputView();
    fn ICoreInputView2();
    fn ICoreInputView3();
    fn ICoreInputView4();
    fn ICoreInputView5();
    fn ICoreInputViewAnimationStartingEventArgs();
    fn ICoreInputViewHidingEventArgs();
    fn ICoreInputViewOcclusion();
    fn ICoreInputViewOcclusionsChangedEventArgs();
    fn ICoreInputViewShowingEventArgs();
    fn ICoreInputViewStatics();
    fn ICoreInputViewStatics2();
    fn ICoreInputViewTransferringXYFocusEventArgs();
    fn IUISettingsController();
    fn IUISettingsControllerStatics();
    fn UISettingsController();
}
