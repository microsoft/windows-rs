#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoCopyMediaType(pmtdest : *mut super::DMO_MEDIA_TYPE, pmtsrc : *const super::DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoCreateMediaType(ppmt : *mut *mut super::DMO_MEDIA_TYPE, cbformat : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoDeleteMediaType(pmt : *mut super::DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoDuplicateMediaType(ppmtdest : *mut *mut super::DMO_MEDIA_TYPE, pmtsrc : *const super::DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoFreeMediaType(pmt : *mut super::DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "mediaobj")]
windows_link::link!("msdmo.dll" "system" fn MoInitMediaType(pmt : *mut super::DMO_MEDIA_TYPE, cbformat : u32) -> windows_sys::core::HRESULT);
