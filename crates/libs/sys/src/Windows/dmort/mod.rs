#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoCopyMediaType(pmtdest : *mut super::mediaobj::DMO_MEDIA_TYPE, pmtsrc : *const super::mediaobj::DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoCreateMediaType(ppmt : *mut *mut super::mediaobj::DMO_MEDIA_TYPE, cbformat : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoDeleteMediaType(pmt : *mut super::mediaobj::DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoDuplicateMediaType(ppmtdest : *mut *mut super::mediaobj::DMO_MEDIA_TYPE, pmtsrc : *const super::mediaobj::DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoFreeMediaType(pmt : *mut super::mediaobj::DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoInitMediaType(pmt : *mut super::mediaobj::DMO_MEDIA_TYPE, cbformat : u32) -> windows_sys::core::HRESULT);
