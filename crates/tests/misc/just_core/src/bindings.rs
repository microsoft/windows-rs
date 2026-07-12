#[inline]
pub unsafe fn CoGetCallerTID() -> windows_core::Result<u32> {
    windows_core::link!("ole32.dll" "system" fn CoGetCallerTID(lpdwtid : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetCallerTID(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetLastError() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetLastError() -> u32);
    unsafe { GetLastError() }
}
#[inline]
pub unsafe fn HidD_GetHidGuid() -> windows_core::GUID {
    windows_core::link!("hid.dll" "system" fn HidD_GetHidGuid(hidguid : *mut windows_core::GUID));
    unsafe {
        let mut result__ = core::mem::zeroed();
        HidD_GetHidGuid(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn IsCharLowerA(ch: i8) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsCharLowerA(ch : i8) -> windows_core::BOOL);
    unsafe { IsCharLowerA(ch) }
}
#[inline]
pub unsafe fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS);
    unsafe { RpcMgmtEnableIdleCleanup() }
}
#[inline]
pub unsafe fn SysFreeString(bstrstring: &windows_core::BSTR) {
    windows_core::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : *mut core::ffi::c_void));
    unsafe { SysFreeString(core::mem::transmute_copy(bstrstring)) }
}
windows_core::imp::define_interface!(
    IStringable,
    IStringable_Vtbl,
    0x96369f54_8eb6_48f0_abce_c1b211e627c3
);
impl windows_core::RuntimeType for IStringable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IStringable");
}
windows_core::imp::interface_hierarchy!(
    IStringable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for IStringable {
    const NAME: &'static str = "Windows.Foundation.IStringable";
}
#[repr(C)]
pub struct IStringable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RPC_STATUS(pub i32);
