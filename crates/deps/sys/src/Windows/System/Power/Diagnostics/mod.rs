#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct BackgroundEnergyDiagnostics(i32);
pub struct ForegroundEnergyDiagnostics(i32);
pub struct IBackgroundEnergyDiagnosticsStatics(i32);
pub struct IForegroundEnergyDiagnosticsStatics(i32);
