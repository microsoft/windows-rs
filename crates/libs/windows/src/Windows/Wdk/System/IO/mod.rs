#[doc = "*Required features: `\"Wdk_System_IO\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn NtDeviceIoControlFile<P0, P1>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: ::core::option::Option<*const ::core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, iocontrolcode: u32, inputbuffer: ::core::option::Option<*const ::core::ffi::c_void>, inputbufferlength: u32, outputbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, outputbufferlength: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdll.dll" "system" fn NtDeviceIoControlFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, iocontrolcode : u32, inputbuffer : *const ::core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut ::core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtDeviceIoControlFile(filehandle.into_param().abi(), event.into_param().abi(), apcroutine, ::core::mem::transmute(apccontext.unwrap_or(::std::ptr::null())), iostatusblock, iocontrolcode, ::core::mem::transmute(inputbuffer.unwrap_or(::std::ptr::null())), inputbufferlength, ::core::mem::transmute(outputbuffer.unwrap_or(::std::ptr::null_mut())), outputbufferlength).ok()
}
