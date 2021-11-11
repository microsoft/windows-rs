#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CortanaActionableInsights();
    fn CortanaActionableInsightsOptions();
    fn CortanaPermission();
    fn CortanaPermissionsChangeResult();
    fn CortanaPermissionsManager();
    fn CortanaSettings();
    fn ICortanaActionableInsights();
    fn ICortanaActionableInsightsOptions();
    fn ICortanaActionableInsightsStatics();
    fn ICortanaPermissionsManager();
    fn ICortanaPermissionsManagerStatics();
    fn ICortanaSettings();
    fn ICortanaSettingsStatics();
}
