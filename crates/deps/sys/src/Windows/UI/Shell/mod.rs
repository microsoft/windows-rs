#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AdaptiveCardBuilder();
    fn IAdaptiveCard();
    fn IAdaptiveCardBuilderStatics();
    fn ISecurityAppManager();
    fn IShareWindowCommandEventArgs();
    fn IShareWindowCommandSource();
    fn IShareWindowCommandSourceStatics();
    fn ITaskbarManager();
    fn ITaskbarManager2();
    fn ITaskbarManagerStatics();
    fn SecurityAppKind();
    fn SecurityAppManager();
    fn SecurityAppManagerContract();
    fn SecurityAppState();
    fn SecurityAppSubstatus();
    fn ShareWindowCommand();
    fn ShareWindowCommandEventArgs();
    fn ShareWindowCommandSource();
    fn TaskbarManager();
}
