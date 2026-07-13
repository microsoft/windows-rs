windows_link::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn HasExpandedResources(hasexpandedresources : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn ReleaseExclusiveCpuSets() -> windows_sys::core::HRESULT);
