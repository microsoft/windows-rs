#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "wsclient.dll" ,"system" fn AcquireDeveloperLicense ( hwndparent : super::super::Foundation:: HWND , pexpiration : *mut super::super::Foundation:: FILETIME ) -> :: windows_sys::core::HRESULT );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "wsclient.dll" ,"system" fn CheckDeveloperLicense ( pexpiration : *mut super::super::Foundation:: FILETIME ) -> :: windows_sys::core::HRESULT );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "wsclient.dll" ,"system" fn RemoveDeveloperLicense ( hwndparent : super::super::Foundation:: HWND ) -> :: windows_sys::core::HRESULT );
