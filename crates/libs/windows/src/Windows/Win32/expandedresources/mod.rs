#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { GetExpandedResourceExclusiveCpuCount(exclusivecpucount as _) }
}
#[inline]
pub unsafe fn HasExpandedResources(hasexpandedresources: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn HasExpandedResources(hasexpandedresources : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { HasExpandedResources(hasexpandedresources as _) }
}
#[inline]
pub unsafe fn ReleaseExclusiveCpuSets() -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn ReleaseExclusiveCpuSets() -> windows_core::HRESULT);
    unsafe { ReleaseExclusiveCpuSets() }
}
