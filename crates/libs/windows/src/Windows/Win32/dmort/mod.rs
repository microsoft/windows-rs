#[cfg(all(feature = "mediaobj", feature = "shared"))]
#[inline]
pub unsafe fn MoCopyMediaType(pmtdest: *mut super::DMO_MEDIA_TYPE, pmtsrc: *const super::DMO_MEDIA_TYPE) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoCopyMediaType(pmtdest : *mut super::DMO_MEDIA_TYPE, pmtsrc : *const super::DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoCopyMediaType(pmtdest as _, pmtsrc as _) }
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
#[inline]
pub unsafe fn MoCreateMediaType(ppmt: *mut *mut super::DMO_MEDIA_TYPE, cbformat: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoCreateMediaType(ppmt : *mut *mut super::DMO_MEDIA_TYPE, cbformat : super::DWORD) -> windows_core::HRESULT);
    unsafe { MoCreateMediaType(ppmt as _, cbformat) }
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
#[inline]
pub unsafe fn MoDeleteMediaType(pmt: *mut super::DMO_MEDIA_TYPE) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoDeleteMediaType(pmt : *mut super::DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoDeleteMediaType(pmt as _) }
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
#[inline]
pub unsafe fn MoDuplicateMediaType(ppmtdest: *mut *mut super::DMO_MEDIA_TYPE, pmtsrc: *const super::DMO_MEDIA_TYPE) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoDuplicateMediaType(ppmtdest : *mut *mut super::DMO_MEDIA_TYPE, pmtsrc : *const super::DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoDuplicateMediaType(ppmtdest as _, pmtsrc as _) }
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
#[inline]
pub unsafe fn MoFreeMediaType(pmt: *mut super::DMO_MEDIA_TYPE) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoFreeMediaType(pmt : *mut super::DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoFreeMediaType(pmt as _) }
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
#[inline]
pub unsafe fn MoInitMediaType(pmt: *mut super::DMO_MEDIA_TYPE, cbformat: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoInitMediaType(pmt : *mut super::DMO_MEDIA_TYPE, cbformat : super::DWORD) -> windows_core::HRESULT);
    unsafe { MoInitMediaType(pmt as _, cbformat) }
}
