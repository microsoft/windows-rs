#![expect(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]

include!(concat!(env!("OUT_DIR"), "/compile_fixtures.rs"));

#[cfg(windows)]
#[test]
fn perflib_query() {
    use perflib::*;

    let mut query = HANDLE::default();
    let status = unsafe { PerfOpenQueryHandle(windows_core::PCWSTR::null(), &mut query) };
    assert_eq!(status, 0);

    let mut bytes = u32::MAX;
    let status = unsafe { PerfQueryCounterInfo(query, core::ptr::null_mut(), 0, &mut bytes) };
    let close_status = unsafe { PerfCloseQueryHandle(query) };
    assert_eq!(close_status, 0);
    assert_eq!(status, 0);
    assert_eq!(bytes, 0);
}
