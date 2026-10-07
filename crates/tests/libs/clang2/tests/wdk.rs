#![cfg(target_env = "msvc")]

#[allow(non_snake_case, non_camel_case_types, dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/wdk.rs"));
}
use bindings::*;

#[test]
fn real_wdk_offsets_match_native_compiler() {
    for (index, value) in [
        size_of::<DeviceIoControl>(),
        align_of::<DeviceIoControl>(),
        std::mem::offset_of!(DeviceIoControl, OutputBufferLength),
        std::mem::offset_of!(DeviceIoControl, InputBufferLength),
        std::mem::offset_of!(DeviceIoControl, IoControlCode),
        std::mem::offset_of!(DeviceIoControl, Type3InputBuffer),
        size_of::<QuerySecurity>(),
        align_of::<QuerySecurity>(),
        std::mem::offset_of!(QuerySecurity, SecurityInformation),
        std::mem::offset_of!(QuerySecurity, Length),
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            unsafe { WdkLayout(index.try_into().unwrap()) },
            u32::try_from(value).unwrap()
        );
    }
}

#[test]
fn real_wdk_fields_cross_native_pointer_calls() {
    let mut device = DeviceIoControl::default();
    let mut security = QuerySecurity::default();
    device.OutputBufferLength = 17;
    device.InputBufferLength = 23;
    device.IoControlCode = 0x1234;
    security.SecurityInformation = 0x42;
    security.Length = 31;
    unsafe { WdkMutate(&raw mut device, &raw mut security) };
    assert_eq!(device.OutputBufferLength, 40);
    assert_eq!(device.InputBufferLength, 23 ^ 0x1234);
    assert_eq!(device.IoControlCode, 0x1234 + 0x42);
    assert_eq!(security.SecurityInformation, 0x42 ^ 0x13579bdf);
    assert_eq!(security.Length, 71);
    assert_eq!(device.Type3InputBuffer, (&raw mut security).cast());
}
