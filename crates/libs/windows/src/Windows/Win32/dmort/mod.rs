#[cfg(feature = "mediaobj")]
#[inline]
pub unsafe fn MoCopyMediaType(pmtdest: *mut super::DMO_MEDIA_TYPE, pmtsrc: *const super::DMO_MEDIA_TYPE) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoCopyMediaType(pmtdest : *mut super::DMO_MEDIA_TYPE, pmtsrc : *const super::DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoCopyMediaType(pmtdest, pmtsrc) }
}
#[cfg(feature = "mediaobj")]
#[inline]
pub unsafe fn MoCreateMediaType(ppmt: *mut *mut super::DMO_MEDIA_TYPE, cbformat: u32) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoCreateMediaType(ppmt : *mut *mut super::DMO_MEDIA_TYPE, cbformat : u32) -> windows_core::HRESULT);
    unsafe { MoCreateMediaType(ppmt as _, cbformat) }
}
#[cfg(feature = "mediaobj")]
#[inline]
pub unsafe fn MoDeleteMediaType(pmt: *mut super::DMO_MEDIA_TYPE) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoDeleteMediaType(pmt : *mut super::DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoDeleteMediaType(pmt) }
}
#[cfg(feature = "mediaobj")]
#[inline]
pub unsafe fn MoDuplicateMediaType(ppmtdest: *mut *mut super::DMO_MEDIA_TYPE, pmtsrc: *const super::DMO_MEDIA_TYPE) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoDuplicateMediaType(ppmtdest : *mut *mut super::DMO_MEDIA_TYPE, pmtsrc : *const super::DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoDuplicateMediaType(ppmtdest as _, pmtsrc) }
}
#[cfg(feature = "mediaobj")]
#[inline]
pub unsafe fn MoFreeMediaType(pmt: *mut super::DMO_MEDIA_TYPE) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoFreeMediaType(pmt : *mut super::DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoFreeMediaType(pmt) }
}
#[cfg(feature = "mediaobj")]
#[inline]
pub unsafe fn MoInitMediaType(pmt: *mut super::DMO_MEDIA_TYPE, cbformat: u32) -> windows_core::HRESULT {
    windows_core::link!("msdmo.dll" "system" fn MoInitMediaType(pmt : *mut super::DMO_MEDIA_TYPE, cbformat : u32) -> windows_core::HRESULT);
    unsafe { MoInitMediaType(pmt, cbformat) }
}
