#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub struct GuidHelper;
impl GuidHelper {}
impl windows_core::RuntimeName for GuidHelper {
    const NAME: &'static str = "Windows.Foundation.GuidHelper";
}
