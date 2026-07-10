#[inline]
pub unsafe fn GetExpandedResourceExclusiveCpuCount() -> windows_core::Result<u32> {
    windows_core::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetExpandedResourceExclusiveCpuCount(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn HasExpandedResources() -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn HasExpandedResources(hasexpandedresources : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HasExpandedResources(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn ReleaseExclusiveCpuSets() -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn ReleaseExclusiveCpuSets() -> windows_core::HRESULT);
    unsafe { ReleaseExclusiveCpuSets() }
}
