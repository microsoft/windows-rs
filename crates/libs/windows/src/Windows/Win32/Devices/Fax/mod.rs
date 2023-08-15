#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CanSendToFaxRecipient() -> super::super::Foundation::BOOL {
    ::windows_targets::link!("fxsutility.dll" "system" fn CanSendToFaxRecipient() -> super::super::Foundation:: BOOL);
    CanSendToFaxRecipient()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxAbort<P0>(faxhandle: P0, jobid: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxAbort(faxhandle : super::super::Foundation:: HANDLE, jobid : u32) -> super::super::Foundation:: BOOL);
    FaxAbort(faxhandle.into_param().abi(), jobid)
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxAccessCheck<P0>(faxhandle: P0, accessmask: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxAccessCheck(faxhandle : super::super::Foundation:: HANDLE, accessmask : u32) -> super::super::Foundation:: BOOL);
    FaxAccessCheck(faxhandle.into_param().abi(), accessmask)
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxClose<P0>(faxhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxClose(faxhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    FaxClose(faxhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxCompleteJobParamsA(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("winfax.dll" "system" fn FaxCompleteJobParamsA(jobparams : *mut *mut FAX_JOB_PARAMA, coverpageinfo : *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation:: BOOL);
    FaxCompleteJobParamsA(jobparams, coverpageinfo)
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxCompleteJobParamsW(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("winfax.dll" "system" fn FaxCompleteJobParamsW(jobparams : *mut *mut FAX_JOB_PARAMW, coverpageinfo : *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation:: BOOL);
    FaxCompleteJobParamsW(jobparams, coverpageinfo)
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxConnectFaxServerA<P0>(machinename: P0, faxhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxConnectFaxServerA(machinename : ::windows_core::PCSTR, faxhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    FaxConnectFaxServerA(machinename.into_param().abi(), faxhandle).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxConnectFaxServerW<P0>(machinename: P0, faxhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxConnectFaxServerW(machinename : ::windows_core::PCWSTR, faxhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    FaxConnectFaxServerW(machinename.into_param().abi(), faxhandle).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnableRoutingMethodA<P0, P1, P2>(faxporthandle: P0, routingguid: P1, enabled: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxEnableRoutingMethodA(faxporthandle : super::super::Foundation:: HANDLE, routingguid : ::windows_core::PCSTR, enabled : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    FaxEnableRoutingMethodA(faxporthandle.into_param().abi(), routingguid.into_param().abi(), enabled.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnableRoutingMethodW<P0, P1, P2>(faxporthandle: P0, routingguid: P1, enabled: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxEnableRoutingMethodW(faxporthandle : super::super::Foundation:: HANDLE, routingguid : ::windows_core::PCWSTR, enabled : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    FaxEnableRoutingMethodW(faxporthandle.into_param().abi(), routingguid.into_param().abi(), enabled.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumGlobalRoutingInfoA<P0>(faxhandle: P0, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxEnumGlobalRoutingInfoA(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned : *mut u32) -> super::super::Foundation:: BOOL);
    FaxEnumGlobalRoutingInfoA(faxhandle.into_param().abi(), routinginfo, methodsreturned).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumGlobalRoutingInfoW<P0>(faxhandle: P0, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxEnumGlobalRoutingInfoW(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned : *mut u32) -> super::super::Foundation:: BOOL);
    FaxEnumGlobalRoutingInfoW(faxhandle.into_param().abi(), routinginfo, methodsreturned).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumJobsA<P0>(faxhandle: P0, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxEnumJobsA(faxhandle : super::super::Foundation:: HANDLE, jobentry : *mut *mut FAX_JOB_ENTRYA, jobsreturned : *mut u32) -> super::super::Foundation:: BOOL);
    FaxEnumJobsA(faxhandle.into_param().abi(), jobentry, jobsreturned).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumJobsW<P0>(faxhandle: P0, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxEnumJobsW(faxhandle : super::super::Foundation:: HANDLE, jobentry : *mut *mut FAX_JOB_ENTRYW, jobsreturned : *mut u32) -> super::super::Foundation:: BOOL);
    FaxEnumJobsW(faxhandle.into_param().abi(), jobentry, jobsreturned).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumPortsA<P0>(faxhandle: P0, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxEnumPortsA(faxhandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOA, portsreturned : *mut u32) -> super::super::Foundation:: BOOL);
    FaxEnumPortsA(faxhandle.into_param().abi(), portinfo, portsreturned).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumPortsW<P0>(faxhandle: P0, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxEnumPortsW(faxhandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOW, portsreturned : *mut u32) -> super::super::Foundation:: BOOL);
    FaxEnumPortsW(faxhandle.into_param().abi(), portinfo, portsreturned).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumRoutingMethodsA<P0>(faxporthandle: P0, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxEnumRoutingMethodsA(faxporthandle : super::super::Foundation:: HANDLE, routingmethod : *mut *mut FAX_ROUTING_METHODA, methodsreturned : *mut u32) -> super::super::Foundation:: BOOL);
    FaxEnumRoutingMethodsA(faxporthandle.into_param().abi(), routingmethod, methodsreturned).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumRoutingMethodsW<P0>(faxporthandle: P0, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxEnumRoutingMethodsW(faxporthandle : super::super::Foundation:: HANDLE, routingmethod : *mut *mut FAX_ROUTING_METHODW, methodsreturned : *mut u32) -> super::super::Foundation:: BOOL);
    FaxEnumRoutingMethodsW(faxporthandle.into_param().abi(), routingmethod, methodsreturned).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[inline]
pub unsafe fn FaxFreeBuffer(buffer: *mut ::core::ffi::c_void) {
    ::windows_targets::link!("winfax.dll" "system" fn FaxFreeBuffer(buffer : *mut ::core::ffi::c_void) -> ());
    FaxFreeBuffer(buffer)
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetConfigurationA<P0>(faxhandle: P0, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetConfigurationA(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation:: BOOL);
    FaxGetConfigurationA(faxhandle.into_param().abi(), faxconfig).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetConfigurationW<P0>(faxhandle: P0, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetConfigurationW(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation:: BOOL);
    FaxGetConfigurationW(faxhandle.into_param().abi(), faxconfig).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetDeviceStatusA<P0>(faxporthandle: P0, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetDeviceStatusA(faxporthandle : super::super::Foundation:: HANDLE, devicestatus : *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation:: BOOL);
    FaxGetDeviceStatusA(faxporthandle.into_param().abi(), devicestatus).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetDeviceStatusW<P0>(faxporthandle: P0, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetDeviceStatusW(faxporthandle : super::super::Foundation:: HANDLE, devicestatus : *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation:: BOOL);
    FaxGetDeviceStatusW(faxporthandle.into_param().abi(), devicestatus).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetJobA<P0>(faxhandle: P0, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetJobA(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, jobentry : *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation:: BOOL);
    FaxGetJobA(faxhandle.into_param().abi(), jobid, jobentry).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetJobW<P0>(faxhandle: P0, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetJobW(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, jobentry : *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation:: BOOL);
    FaxGetJobW(faxhandle.into_param().abi(), jobid, jobentry).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetLoggingCategoriesA<P0>(faxhandle: P0, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetLoggingCategoriesA(faxhandle : super::super::Foundation:: HANDLE, categories : *mut *mut FAX_LOG_CATEGORYA, numbercategories : *mut u32) -> super::super::Foundation:: BOOL);
    FaxGetLoggingCategoriesA(faxhandle.into_param().abi(), categories, numbercategories).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetLoggingCategoriesW<P0>(faxhandle: P0, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetLoggingCategoriesW(faxhandle : super::super::Foundation:: HANDLE, categories : *mut *mut FAX_LOG_CATEGORYW, numbercategories : *mut u32) -> super::super::Foundation:: BOOL);
    FaxGetLoggingCategoriesW(faxhandle.into_param().abi(), categories, numbercategories).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetPageData<P0>(faxhandle: P0, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetPageData(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, buffer : *mut *mut u8, buffersize : *mut u32, imagewidth : *mut u32, imageheight : *mut u32) -> super::super::Foundation:: BOOL);
    FaxGetPageData(faxhandle.into_param().abi(), jobid, buffer, buffersize, imagewidth, imageheight)
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetPortA<P0>(faxporthandle: P0, portinfo: *mut *mut FAX_PORT_INFOA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetPortA(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOA) -> super::super::Foundation:: BOOL);
    FaxGetPortA(faxporthandle.into_param().abi(), portinfo).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetPortW<P0>(faxporthandle: P0, portinfo: *mut *mut FAX_PORT_INFOW) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetPortW(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOW) -> super::super::Foundation:: BOOL);
    FaxGetPortW(faxporthandle.into_param().abi(), portinfo).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetRoutingInfoA<P0, P1>(faxporthandle: P0, routingguid: P1, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetRoutingInfoA(faxporthandle : super::super::Foundation:: HANDLE, routingguid : ::windows_core::PCSTR, routinginfobuffer : *mut *mut u8, routinginfobuffersize : *mut u32) -> super::super::Foundation:: BOOL);
    FaxGetRoutingInfoA(faxporthandle.into_param().abi(), routingguid.into_param().abi(), routinginfobuffer, routinginfobuffersize).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetRoutingInfoW<P0, P1>(faxporthandle: P0, routingguid: P1, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxGetRoutingInfoW(faxporthandle : super::super::Foundation:: HANDLE, routingguid : ::windows_core::PCWSTR, routinginfobuffer : *mut *mut u8, routinginfobuffersize : *mut u32) -> super::super::Foundation:: BOOL);
    FaxGetRoutingInfoW(faxporthandle.into_param().abi(), routingguid.into_param().abi(), routinginfobuffer, routinginfobuffersize).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxInitializeEventQueue<P0, P1, P2>(faxhandle: P0, completionport: P1, completionkey: usize, hwnd: P2, messagestart: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxInitializeEventQueue(faxhandle : super::super::Foundation:: HANDLE, completionport : super::super::Foundation:: HANDLE, completionkey : usize, hwnd : super::super::Foundation:: HWND, messagestart : u32) -> super::super::Foundation:: BOOL);
    FaxInitializeEventQueue(faxhandle.into_param().abi(), completionport.into_param().abi(), completionkey, hwnd.into_param().abi(), messagestart)
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxOpenPort<P0>(faxhandle: P0, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxOpenPort(faxhandle : super::super::Foundation:: HANDLE, deviceid : u32, flags : u32, faxporthandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    FaxOpenPort(faxhandle.into_param().abi(), deviceid, flags, faxporthandle)
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxPrintCoverPageA(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winfax.dll" "system" fn FaxPrintCoverPageA(faxcontextinfo : *const FAX_CONTEXT_INFOA, coverpageinfo : *const FAX_COVERPAGE_INFOA) -> super::super::Foundation:: BOOL);
    FaxPrintCoverPageA(faxcontextinfo, coverpageinfo).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxPrintCoverPageW(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> ::windows_core::Result<()> {
    ::windows_targets::link!("winfax.dll" "system" fn FaxPrintCoverPageW(faxcontextinfo : *const FAX_CONTEXT_INFOW, coverpageinfo : *const FAX_COVERPAGE_INFOW) -> super::super::Foundation:: BOOL);
    FaxPrintCoverPageW(faxcontextinfo, coverpageinfo).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxRegisterRoutingExtensionW<P0, P1, P2, P3>(faxhandle: P0, extensionname: P1, friendlyname: P2, imagename: P3, callback: PFAX_ROUTING_INSTALLATION_CALLBACKW, context: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxRegisterRoutingExtensionW(faxhandle : super::super::Foundation:: HANDLE, extensionname : ::windows_core::PCWSTR, friendlyname : ::windows_core::PCWSTR, imagename : ::windows_core::PCWSTR, callback : PFAX_ROUTING_INSTALLATION_CALLBACKW, context : *mut ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    FaxRegisterRoutingExtensionW(faxhandle.into_param().abi(), extensionname.into_param().abi(), friendlyname.into_param().abi(), imagename.into_param().abi(), callback, context).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxRegisterServiceProviderW<P0, P1, P2, P3>(deviceprovider: P0, friendlyname: P1, imagename: P2, tspname: P3) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxRegisterServiceProviderW(deviceprovider : ::windows_core::PCWSTR, friendlyname : ::windows_core::PCWSTR, imagename : ::windows_core::PCWSTR, tspname : ::windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    FaxRegisterServiceProviderW(deviceprovider.into_param().abi(), friendlyname.into_param().abi(), imagename.into_param().abi(), tspname.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentA<P0, P1>(faxhandle: P0, filename: P1, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSendDocumentA(faxhandle : super::super::Foundation:: HANDLE, filename : ::windows_core::PCSTR, jobparams : *mut FAX_JOB_PARAMA, coverpageinfo : *const FAX_COVERPAGE_INFOA, faxjobid : *mut u32) -> super::super::Foundation:: BOOL);
    FaxSendDocumentA(faxhandle.into_param().abi(), filename.into_param().abi(), jobparams, coverpageinfo, faxjobid).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentForBroadcastA<P0, P1>(faxhandle: P0, filename: P1, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKA, context: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSendDocumentForBroadcastA(faxhandle : super::super::Foundation:: HANDLE, filename : ::windows_core::PCSTR, faxjobid : *mut u32, faxrecipientcallback : PFAX_RECIPIENT_CALLBACKA, context : *mut ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    FaxSendDocumentForBroadcastA(faxhandle.into_param().abi(), filename.into_param().abi(), faxjobid, faxrecipientcallback, context).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentForBroadcastW<P0, P1>(faxhandle: P0, filename: P1, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKW, context: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSendDocumentForBroadcastW(faxhandle : super::super::Foundation:: HANDLE, filename : ::windows_core::PCWSTR, faxjobid : *mut u32, faxrecipientcallback : PFAX_RECIPIENT_CALLBACKW, context : *mut ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    FaxSendDocumentForBroadcastW(faxhandle.into_param().abi(), filename.into_param().abi(), faxjobid, faxrecipientcallback, context).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentW<P0, P1>(faxhandle: P0, filename: P1, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSendDocumentW(faxhandle : super::super::Foundation:: HANDLE, filename : ::windows_core::PCWSTR, jobparams : *mut FAX_JOB_PARAMW, coverpageinfo : *const FAX_COVERPAGE_INFOW, faxjobid : *mut u32) -> super::super::Foundation:: BOOL);
    FaxSendDocumentW(faxhandle.into_param().abi(), filename.into_param().abi(), jobparams, coverpageinfo, faxjobid).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetConfigurationA<P0>(faxhandle: P0, faxconfig: *const FAX_CONFIGURATIONA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetConfigurationA(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *const FAX_CONFIGURATIONA) -> super::super::Foundation:: BOOL);
    FaxSetConfigurationA(faxhandle.into_param().abi(), faxconfig).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetConfigurationW<P0>(faxhandle: P0, faxconfig: *const FAX_CONFIGURATIONW) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetConfigurationW(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *const FAX_CONFIGURATIONW) -> super::super::Foundation:: BOOL);
    FaxSetConfigurationW(faxhandle.into_param().abi(), faxconfig).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetGlobalRoutingInfoA<P0>(faxhandle: P0, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetGlobalRoutingInfoA(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation:: BOOL);
    FaxSetGlobalRoutingInfoA(faxhandle.into_param().abi(), routinginfo).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetGlobalRoutingInfoW<P0>(faxhandle: P0, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetGlobalRoutingInfoW(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation:: BOOL);
    FaxSetGlobalRoutingInfoW(faxhandle.into_param().abi(), routinginfo).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetJobA<P0>(faxhandle: P0, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetJobA(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, command : u32, jobentry : *const FAX_JOB_ENTRYA) -> super::super::Foundation:: BOOL);
    FaxSetJobA(faxhandle.into_param().abi(), jobid, command, jobentry).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetJobW<P0>(faxhandle: P0, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetJobW(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, command : u32, jobentry : *const FAX_JOB_ENTRYW) -> super::super::Foundation:: BOOL);
    FaxSetJobW(faxhandle.into_param().abi(), jobid, command, jobentry).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetLoggingCategoriesA<P0>(faxhandle: P0, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetLoggingCategoriesA(faxhandle : super::super::Foundation:: HANDLE, categories : *const FAX_LOG_CATEGORYA, numbercategories : u32) -> super::super::Foundation:: BOOL);
    FaxSetLoggingCategoriesA(faxhandle.into_param().abi(), categories, numbercategories).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetLoggingCategoriesW<P0>(faxhandle: P0, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetLoggingCategoriesW(faxhandle : super::super::Foundation:: HANDLE, categories : *const FAX_LOG_CATEGORYW, numbercategories : u32) -> super::super::Foundation:: BOOL);
    FaxSetLoggingCategoriesW(faxhandle.into_param().abi(), categories, numbercategories).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetPortA<P0>(faxporthandle: P0, portinfo: *const FAX_PORT_INFOA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetPortA(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *const FAX_PORT_INFOA) -> super::super::Foundation:: BOOL);
    FaxSetPortA(faxporthandle.into_param().abi(), portinfo).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetPortW<P0>(faxporthandle: P0, portinfo: *const FAX_PORT_INFOW) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetPortW(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *const FAX_PORT_INFOW) -> super::super::Foundation:: BOOL);
    FaxSetPortW(faxporthandle.into_param().abi(), portinfo).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetRoutingInfoA<P0, P1>(faxporthandle: P0, routingguid: P1, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetRoutingInfoA(faxporthandle : super::super::Foundation:: HANDLE, routingguid : ::windows_core::PCSTR, routinginfobuffer : *const u8, routinginfobuffersize : u32) -> super::super::Foundation:: BOOL);
    FaxSetRoutingInfoA(faxporthandle.into_param().abi(), routingguid.into_param().abi(), routinginfobuffer, routinginfobuffersize).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetRoutingInfoW<P0, P1>(faxporthandle: P0, routingguid: P1, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxSetRoutingInfoW(faxporthandle : super::super::Foundation:: HANDLE, routingguid : ::windows_core::PCWSTR, routinginfobuffer : *const u8, routinginfobuffersize : u32) -> super::super::Foundation:: BOOL);
    FaxSetRoutingInfoW(faxporthandle.into_param().abi(), routingguid.into_param().abi(), routinginfobuffer, routinginfobuffersize).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxStartPrintJobA<P0>(printername: P0, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxStartPrintJobA(printername : ::windows_core::PCSTR, printinfo : *const FAX_PRINT_INFOA, faxjobid : *mut u32, faxcontextinfo : *mut FAX_CONTEXT_INFOA) -> super::super::Foundation:: BOOL);
    FaxStartPrintJobA(printername.into_param().abi(), printinfo, faxjobid, faxcontextinfo).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxStartPrintJobW<P0>(printername: P0, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxStartPrintJobW(printername : ::windows_core::PCWSTR, printinfo : *const FAX_PRINT_INFOW, faxjobid : *mut u32, faxcontextinfo : *mut FAX_CONTEXT_INFOW) -> super::super::Foundation:: BOOL);
    FaxStartPrintJobW(printername.into_param().abi(), printinfo, faxjobid, faxcontextinfo).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxUnregisterServiceProviderW<P0>(deviceprovider: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winfax.dll" "system" fn FaxUnregisterServiceProviderW(deviceprovider : ::windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    FaxUnregisterServiceProviderW(deviceprovider.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[inline]
pub unsafe fn SendToFaxRecipient<P0>(sndmode: SendToMode, lpfilename: P0) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("fxsutility.dll" "system" fn SendToFaxRecipient(sndmode : SendToMode, lpfilename : ::windows_core::PCWSTR) -> u32);
    SendToFaxRecipient(sndmode, lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StiCreateInstanceW<P0, P1>(hinst: P0, dwver: u32, ppsti: *mut ::core::option::Option<IStillImageW>, punkouter: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("sti.dll" "system" fn StiCreateInstanceW(hinst : super::super::Foundation:: HINSTANCE, dwver : u32, ppsti : *mut * mut::core::ffi::c_void, punkouter : * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    StiCreateInstanceW(hinst.into_param().abi(), dwver, ::core::mem::transmute(ppsti), punkouter.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccount(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccount {
    pub unsafe fn AccountName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AccountName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Folders(&self) -> ::windows_core::Result<IFaxAccountFolders> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Folders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ListenToAccountEvents(&self, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ListenToAccountEvents)(::windows_core::Interface::as_raw(self), eventtypes).ok()
    }
    pub unsafe fn RegisteredEvents(&self) -> ::windows_core::Result<FAX_ACCOUNT_EVENTS_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisteredEvents)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxAccount, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccount {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccount").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxAccount {
    type Vtable = IFaxAccount_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxAccount {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68535b33_5dc4_4086_be26_b76f9b711006);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccount_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstraccountname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Folders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfolders: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Folders: usize,
    pub ListenToAccountEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub RegisteredEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pregisteredevents: *mut FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountFolders(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountFolders {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutgoingQueue(&self) -> ::windows_core::Result<IFaxAccountOutgoingQueue> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OutgoingQueue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncomingQueue(&self) -> ::windows_core::Result<IFaxAccountIncomingQueue> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IncomingQueue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncomingArchive(&self) -> ::windows_core::Result<IFaxAccountIncomingArchive> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IncomingArchive)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutgoingArchive(&self) -> ::windows_core::Result<IFaxAccountOutgoingArchive> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OutgoingArchive)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxAccountFolders, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountFolders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountFolders {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountFolders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountFolders").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxAccountFolders {
    type Vtable = IFaxAccountFolders_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountFolders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxAccountFolders {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6463f89d_23d8_46a9_8f86_c47b77ca7926);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountFolders_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingArchive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingArchive: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountIncomingArchive(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountIncomingArchive {
    pub unsafe fn SizeLow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SizeLow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SizeHigh(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SizeHigh)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows_core::Result<IFaxIncomingMessageIterator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMessages)(::windows_core::Interface::as_raw(self), lprefetchsize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessage<P0>(&self, bstrmessageid: P0) -> ::windows_core::Result<IFaxIncomingMessage>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMessage)(::windows_core::Interface::as_raw(self), bstrmessageid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxAccountIncomingArchive, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountIncomingArchive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountIncomingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountIncomingArchive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountIncomingArchive").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxAccountIncomingArchive {
    type Vtable = IFaxAccountIncomingArchive_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountIncomingArchive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxAccountIncomingArchive {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8a5b6ef_e0d6_4aee_955c_91625bec9db4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountIncomingArchive_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SizeLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows_core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessage: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountIncomingQueue(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountIncomingQueue {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJobs(&self) -> ::windows_core::Result<IFaxIncomingJobs> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetJobs)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJob<P0>(&self, bstrjobid: P0) -> ::windows_core::Result<IFaxIncomingJob>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetJob)(::windows_core::Interface::as_raw(self), bstrjobid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxAccountIncomingQueue, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountIncomingQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountIncomingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountIncomingQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountIncomingQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxAccountIncomingQueue {
    type Vtable = IFaxAccountIncomingQueue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountIncomingQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxAccountIncomingQueue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd142d92_0186_4a95_a090_cbc3eadba6b4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountIncomingQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJob: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountNotify(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountNotify {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnIncomingJobAdded<P0, P1>(&self, pfaxaccount: P0, bstrjobid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxAccount>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnIncomingJobAdded)(::windows_core::Interface::as_raw(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnIncomingJobRemoved<P0, P1>(&self, pfaxaccount: P0, bstrjobid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxAccount>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnIncomingJobRemoved)(::windows_core::Interface::as_raw(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnIncomingJobChanged<P0, P1, P2>(&self, pfaxaccount: P0, bstrjobid: P1, pjobstatus: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxAccount>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<IFaxJobStatus>,
    {
        (::windows_core::Interface::vtable(self).OnIncomingJobChanged)(::windows_core::Interface::as_raw(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingJobAdded<P0, P1>(&self, pfaxaccount: P0, bstrjobid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxAccount>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingJobAdded)(::windows_core::Interface::as_raw(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingJobRemoved<P0, P1>(&self, pfaxaccount: P0, bstrjobid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxAccount>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingJobRemoved)(::windows_core::Interface::as_raw(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingJobChanged<P0, P1, P2>(&self, pfaxaccount: P0, bstrjobid: P1, pjobstatus: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxAccount>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<IFaxJobStatus>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingJobChanged)(::windows_core::Interface::as_raw(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingMessageAdded<P0, P1, P2>(&self, pfaxaccount: P0, bstrmessageid: P1, faddedtoreceivefolder: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxAccount>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).OnIncomingMessageAdded)(::windows_core::Interface::as_raw(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi(), faddedtoreceivefolder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingMessageRemoved<P0, P1, P2>(&self, pfaxaccount: P0, bstrmessageid: P1, fremovedfromreceivefolder: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxAccount>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).OnIncomingMessageRemoved)(::windows_core::Interface::as_raw(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi(), fremovedfromreceivefolder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingMessageAdded<P0, P1>(&self, pfaxaccount: P0, bstrmessageid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxAccount>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingMessageAdded)(::windows_core::Interface::as_raw(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingMessageRemoved<P0, P1>(&self, pfaxaccount: P0, bstrmessageid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxAccount>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingMessageRemoved)(::windows_core::Interface::as_raw(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnServerShutDown<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnServerShutDown)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxAccountNotify, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountNotify {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountNotify").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxAccountNotify {
    type Vtable = IFaxAccountNotify_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxAccountNotify {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9b3bc81_ac1b_46f3_b39d_0adc30e1b788);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountNotify_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnIncomingJobAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnIncomingJobAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnIncomingJobRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnIncomingJobRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnIncomingJobChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnIncomingJobChanged: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingJobAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingJobAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingJobRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingJobRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingJobChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingJobChanged: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingMessageAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, faddedtoreceivefolder: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingMessageAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingMessageRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, fremovedfromreceivefolder: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingMessageRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingMessageAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingMessageAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingMessageRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingMessageRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnServerShutDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnServerShutDown: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountOutgoingArchive(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountOutgoingArchive {
    pub unsafe fn SizeLow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SizeLow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SizeHigh(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SizeHigh)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows_core::Result<IFaxOutgoingMessageIterator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMessages)(::windows_core::Interface::as_raw(self), lprefetchsize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessage<P0>(&self, bstrmessageid: P0) -> ::windows_core::Result<IFaxOutgoingMessage>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMessage)(::windows_core::Interface::as_raw(self), bstrmessageid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxAccountOutgoingArchive, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountOutgoingArchive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountOutgoingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountOutgoingArchive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountOutgoingArchive").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxAccountOutgoingArchive {
    type Vtable = IFaxAccountOutgoingArchive_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountOutgoingArchive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxAccountOutgoingArchive {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5463076d_ec14_491f_926e_b3ceda5e5662);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountOutgoingArchive_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SizeLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows_core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessage: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountOutgoingQueue(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountOutgoingQueue {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJobs(&self) -> ::windows_core::Result<IFaxOutgoingJobs> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetJobs)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJob<P0>(&self, bstrjobid: P0) -> ::windows_core::Result<IFaxOutgoingJob>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetJob)(::windows_core::Interface::as_raw(self), bstrjobid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxAccountOutgoingQueue, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountOutgoingQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountOutgoingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountOutgoingQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountOutgoingQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxAccountOutgoingQueue {
    type Vtable = IFaxAccountOutgoingQueue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountOutgoingQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxAccountOutgoingQueue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f1424e9_f22d_4553_b7a5_0d24bd0d7e46);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountOutgoingQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJob: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountSet(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountSet {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAccounts(&self) -> ::windows_core::Result<IFaxAccounts> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAccounts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAccount<P0>(&self, bstraccountname: P0) -> ::windows_core::Result<IFaxAccount>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAccount)(::windows_core::Interface::as_raw(self), bstraccountname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddAccount<P0>(&self, bstraccountname: P0) -> ::windows_core::Result<IFaxAccount>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddAccount)(::windows_core::Interface::as_raw(self), bstraccountname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveAccount<P0>(&self, bstraccountname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveAccount)(::windows_core::Interface::as_raw(self), bstraccountname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxAccountSet, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxAccountSet {
    type Vtable = IFaxAccountSet_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxAccountSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7428fbae_841e_47b8_86f4_2288946dca1b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountSet_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxaccounts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAccounts: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraccountname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAccount: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraccountname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddAccount: usize,
    pub RemoveAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraccountname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccounts(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccounts {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxAccount> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vindex), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxAccounts, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccounts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccounts {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccounts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccounts").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxAccounts {
    type Vtable = IFaxAccounts_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccounts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxAccounts {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93ea8162_8be7_42d1_ae7b_ec74e2d989da);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccounts_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxActivity(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxActivity {
    pub unsafe fn IncomingMessages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IncomingMessages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RoutingMessages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RoutingMessages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OutgoingMessages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OutgoingMessages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueuedMessages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueuedMessages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxActivity, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxActivity {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxActivity").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxActivity {
    type Vtable = IFaxActivity_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxActivity {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b106f97_3df5_40f2_bc3c_44cb8115ebdf);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxActivity_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IncomingMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plincomingmessages: *mut i32) -> ::windows_core::HRESULT,
    pub RoutingMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plroutingmessages: *mut i32) -> ::windows_core::HRESULT,
    pub OutgoingMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploutgoingmessages: *mut i32) -> ::windows_core::HRESULT,
    pub QueuedMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plqueuedmessages: *mut i32) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxActivityLogging(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxActivityLogging {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogIncoming(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LogIncoming)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogIncoming<P0>(&self, blogincoming: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetLogIncoming)(::windows_core::Interface::as_raw(self), blogincoming.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogOutgoing(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LogOutgoing)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogOutgoing<P0>(&self, blogoutgoing: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetLogOutgoing)(::windows_core::Interface::as_raw(self), blogoutgoing.into_param().abi()).ok()
    }
    pub unsafe fn DatabasePath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DatabasePath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDatabasePath<P0>(&self, bstrdatabasepath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDatabasePath)(::windows_core::Interface::as_raw(self), bstrdatabasepath.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxActivityLogging, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxActivityLogging {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxActivityLogging {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxActivityLogging {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxActivityLogging").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxActivityLogging {
    type Vtable = IFaxActivityLogging_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxActivityLogging {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxActivityLogging {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e29078b_5a69_497b_9592_49b7e7faddb5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxActivityLogging_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub LogIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblogincoming: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogIncoming: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blogincoming: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogIncoming: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogOutgoing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblogoutgoing: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogOutgoing: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogOutgoing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blogoutgoing: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogOutgoing: usize,
    pub DatabasePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdatabasepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDatabasePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdatabasepath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxConfiguration(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxConfiguration {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseArchive(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UseArchive)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseArchive<P0>(&self, busearchive: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUseArchive)(::windows_core::Interface::as_raw(self), busearchive.into_param().abi()).ok()
    }
    pub unsafe fn ArchiveLocation(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ArchiveLocation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetArchiveLocation<P0>(&self, bstrarchivelocation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetArchiveLocation)(::windows_core::Interface::as_raw(self), bstrarchivelocation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SizeQuotaWarning(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SizeQuotaWarning)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSizeQuotaWarning<P0>(&self, bsizequotawarning: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetSizeQuotaWarning)(::windows_core::Interface::as_raw(self), bsizequotawarning.into_param().abi()).ok()
    }
    pub unsafe fn HighQuotaWaterMark(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HighQuotaWaterMark)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHighQuotaWaterMark)(::windows_core::Interface::as_raw(self), lhighquotawatermark).ok()
    }
    pub unsafe fn LowQuotaWaterMark(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LowQuotaWaterMark)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLowQuotaWaterMark)(::windows_core::Interface::as_raw(self), llowquotawatermark).ok()
    }
    pub unsafe fn ArchiveAgeLimit(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ArchiveAgeLimit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetArchiveAgeLimit(&self, larchiveagelimit: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetArchiveAgeLimit)(::windows_core::Interface::as_raw(self), larchiveagelimit).ok()
    }
    pub unsafe fn ArchiveSizeLow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ArchiveSizeLow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ArchiveSizeHigh(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ArchiveSizeHigh)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutgoingQueueBlocked(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OutgoingQueueBlocked)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutgoingQueueBlocked<P0>(&self, boutgoingblocked: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetOutgoingQueueBlocked)(::windows_core::Interface::as_raw(self), boutgoingblocked.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutgoingQueuePaused(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OutgoingQueuePaused)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutgoingQueuePaused<P0>(&self, boutgoingpaused: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetOutgoingQueuePaused)(::windows_core::Interface::as_raw(self), boutgoingpaused.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowPersonalCoverPages(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowPersonalCoverPages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowPersonalCoverPages<P0>(&self, ballowpersonalcoverpages: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowPersonalCoverPages)(::windows_core::Interface::as_raw(self), ballowpersonalcoverpages.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseDeviceTSID(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UseDeviceTSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseDeviceTSID<P0>(&self, busedevicetsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUseDeviceTSID)(::windows_core::Interface::as_raw(self), busedevicetsid.into_param().abi()).ok()
    }
    pub unsafe fn Retries(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Retries)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRetries(&self, lretries: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRetries)(::windows_core::Interface::as_raw(self), lretries).ok()
    }
    pub unsafe fn RetryDelay(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RetryDelay)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRetryDelay(&self, lretrydelay: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRetryDelay)(::windows_core::Interface::as_raw(self), lretrydelay).ok()
    }
    pub unsafe fn DiscountRateStart(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DiscountRateStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDiscountRateStart(&self, datediscountratestart: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDiscountRateStart)(::windows_core::Interface::as_raw(self), datediscountratestart).ok()
    }
    pub unsafe fn DiscountRateEnd(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DiscountRateEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDiscountRateEnd)(::windows_core::Interface::as_raw(self), datediscountrateend).ok()
    }
    pub unsafe fn OutgoingQueueAgeLimit(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OutgoingQueueAgeLimit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOutgoingQueueAgeLimit(&self, loutgoingqueueagelimit: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutgoingQueueAgeLimit)(::windows_core::Interface::as_raw(self), loutgoingqueueagelimit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Branding(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Branding)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBranding<P0>(&self, bbranding: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetBranding)(::windows_core::Interface::as_raw(self), bbranding.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IncomingQueueBlocked(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IncomingQueueBlocked)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIncomingQueueBlocked<P0>(&self, bincomingblocked: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIncomingQueueBlocked)(::windows_core::Interface::as_raw(self), bincomingblocked.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoCreateAccountOnConnect(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AutoCreateAccountOnConnect)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoCreateAccountOnConnect<P0>(&self, bautocreateaccountonconnect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAutoCreateAccountOnConnect)(::windows_core::Interface::as_raw(self), bautocreateaccountonconnect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IncomingFaxesArePublic(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IncomingFaxesArePublic)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIncomingFaxesArePublic<P0>(&self, bincomingfaxesarepublic: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIncomingFaxesArePublic)(::windows_core::Interface::as_raw(self), bincomingfaxesarepublic.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxConfiguration, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxConfiguration {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxConfiguration").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxConfiguration {
    type Vtable = IFaxConfiguration_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10f4d0f7_0994_4543_ab6e_506949128c40);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxConfiguration_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub UseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseArchive: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseArchive: usize,
    pub ArchiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrarchivelocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetArchiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrarchivelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SizeQuotaWarning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSizeQuotaWarning: usize,
    pub HighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows_core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows_core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows_core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows_core::HRESULT,
    pub ArchiveAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plarchiveagelimit: *mut i32) -> ::windows_core::HRESULT,
    pub SetArchiveAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, larchiveagelimit: i32) -> ::windows_core::HRESULT,
    pub ArchiveSizeLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows_core::HRESULT,
    pub ArchiveSizeHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OutgoingQueueBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboutgoingblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OutgoingQueueBlocked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutgoingQueueBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boutgoingblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutgoingQueueBlocked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OutgoingQueuePaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboutgoingpaused: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OutgoingQueuePaused: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutgoingQueuePaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boutgoingpaused: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutgoingQueuePaused: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowPersonalCoverPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowPersonalCoverPages: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowPersonalCoverPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowPersonalCoverPages: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UseDeviceTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseDeviceTSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseDeviceTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseDeviceTSID: usize,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT,
    pub SetRetries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows_core::HRESULT,
    pub RetryDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows_core::HRESULT,
    pub SetRetryDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows_core::HRESULT,
    pub DiscountRateStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows_core::HRESULT,
    pub SetDiscountRateStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows_core::HRESULT,
    pub DiscountRateEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows_core::HRESULT,
    pub SetDiscountRateEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows_core::HRESULT,
    pub OutgoingQueueAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploutgoingqueueagelimit: *mut i32) -> ::windows_core::HRESULT,
    pub SetOutgoingQueueAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loutgoingqueueagelimit: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Branding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbranding: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Branding: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBranding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bbranding: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBranding: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IncomingQueueBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbincomingblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IncomingQueueBlocked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIncomingQueueBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bincomingblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIncomingQueueBlocked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AutoCreateAccountOnConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbautocreateaccountonconnect: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AutoCreateAccountOnConnect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAutoCreateAccountOnConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bautocreateaccountonconnect: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAutoCreateAccountOnConnect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IncomingFaxesArePublic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbincomingfaxesarepublic: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IncomingFaxesArePublic: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIncomingFaxesArePublic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bincomingfaxesarepublic: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIncomingFaxesArePublic: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDevice(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDevice {
    pub unsafe fn Id(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProviderUniqueName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProviderUniqueName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PoweredOff(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PoweredOff)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReceivingNow(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReceivingNow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendingNow(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SendingNow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn UsedRoutingMethods(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UsedRoutingMethods)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SendEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSendEnabled<P0>(&self, bsendenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetSendEnabled)(::windows_core::Interface::as_raw(self), bsendenabled.into_param().abi()).ok()
    }
    pub unsafe fn ReceiveMode(&self) -> ::windows_core::Result<FAX_DEVICE_RECEIVE_MODE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReceiveMode(&self, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReceiveMode)(::windows_core::Interface::as_raw(self), receivemode).ok()
    }
    pub unsafe fn RingsBeforeAnswer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RingsBeforeAnswer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRingsBeforeAnswer(&self, lringsbeforeanswer: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRingsBeforeAnswer)(::windows_core::Interface::as_raw(self), lringsbeforeanswer).ok()
    }
    pub unsafe fn CSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCSID<P0>(&self, bstrcsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCSID)(::windows_core::Interface::as_raw(self), bstrcsid.into_param().abi()).ok()
    }
    pub unsafe fn TSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTSID<P0>(&self, bstrtsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTSID)(::windows_core::Interface::as_raw(self), bstrtsid.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetExtensionProperty<P0>(&self, bstrguid: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetExtensionProperty)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetExtensionProperty<P0>(&self, bstrguid: P0, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetExtensionProperty)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), ::core::mem::transmute(vproperty)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseRoutingMethod<P0, P1>(&self, bstrmethodguid: P0, buse: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).UseRoutingMethod)(::windows_core::Interface::as_raw(self), bstrmethodguid.into_param().abi(), buse.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RingingNow(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RingingNow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AnswerCall(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AnswerCall)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxDevice, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDevice {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDevice").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxDevice {
    type Vtable = IFaxDevice_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49306c59_b52e_4867_9df4_ca5841c956d0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDevice_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ProviderUniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprovideruniquename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PoweredOff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpoweredoff: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PoweredOff: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReceivingNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreceivingnow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReceivingNow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SendingNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsendingnow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendingNow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub UsedRoutingMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvusedroutingmethods: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    UsedRoutingMethods: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsendenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSendEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsendenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSendEnabled: usize,
    pub ReceiveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preceivemode: *mut FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows_core::HRESULT,
    pub SetReceiveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows_core::HRESULT,
    pub RingsBeforeAnswer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plringsbeforeanswer: *mut i32) -> ::windows_core::HRESULT,
    pub SetRingsBeforeAnswer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lringsbeforeanswer: i32) -> ::windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetExtensionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvproperty: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetExtensionProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetExtensionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetExtensionProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UseRoutingMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmethodguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, buse: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseRoutingMethod: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RingingNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbringingnow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RingingNow: usize,
    pub AnswerCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDeviceIds(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceIds {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Add(&self, ldeviceid: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), ldeviceid).ok()
    }
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), lindex).ok()
    }
    pub unsafe fn SetOrder(&self, ldeviceid: i32, lneworder: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOrder)(::windows_core::Interface::as_raw(self), ldeviceid, lneworder).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxDeviceIds, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDeviceIds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDeviceIds {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDeviceIds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDeviceIds").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxDeviceIds {
    type Vtable = IFaxDeviceIds_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDeviceIds {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxDeviceIds {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f0f813f_4ce9_443e_8ca1_738cfaeee149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceIds_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pldeviceid: *mut i32) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldeviceid: i32) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT,
    pub SetOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldeviceid: i32, lneworder: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDeviceProvider(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceProvider {
    pub unsafe fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FriendlyName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ImageName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ImageName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UniqueName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UniqueName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TapiProviderName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TapiProviderName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MajorVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MinorVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MajorBuild(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MajorBuild)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MinorBuild(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MinorBuild)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Debug(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Debug)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<FAX_PROVIDER_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitErrorCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InitErrorCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeviceIds(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceIds)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxDeviceProvider, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDeviceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDeviceProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDeviceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDeviceProvider").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxDeviceProvider {
    type Vtable = IFaxDeviceProvider_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDeviceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxDeviceProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x290eac63_83ec_449c_8417_f148df8c682a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceProvider_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ImageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrimagename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub UniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruniquename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TapiProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtapiprovidername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows_core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows_core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Debug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Debug: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows_core::HRESULT,
    pub InitErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeviceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdeviceids: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeviceIds: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDeviceProviders(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceProviders {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxDeviceProvider> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vindex), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxDeviceProviders, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDeviceProviders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDeviceProviders {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDeviceProviders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDeviceProviders").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxDeviceProviders {
    type Vtable = IFaxDeviceProviders_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDeviceProviders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxDeviceProviders {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fb76f62_4c7e_43a5_b6fd_502893f7e13e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceProviders_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxdeviceprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDevices(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDevices {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vindex), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemById(&self, lid: i32) -> ::windows_core::Result<IFaxDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_ItemById)(::windows_core::Interface::as_raw(self), lid, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxDevices, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDevices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDevices {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDevices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDevices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxDevices {
    type Vtable = IFaxDevices_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDevices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxDevices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e46783e_f34f_482e_a360_0416becbbd96);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDevices_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lid: i32, ppfaxdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemById: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDocument(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDocument {
    pub unsafe fn Body(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Body)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBody<P0>(&self, bstrbody: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetBody)(::windows_core::Interface::as_raw(self), bstrbody.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows_core::Result<IFaxSender> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Sender)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipients(&self) -> ::windows_core::Result<IFaxRecipients> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Recipients)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CoverPage(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CoverPage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCoverPage<P0>(&self, bstrcoverpage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCoverPage)(::windows_core::Interface::as_raw(self), bstrcoverpage.into_param().abi()).ok()
    }
    pub unsafe fn Subject(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Subject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubject<P0>(&self, bstrsubject: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSubject)(::windows_core::Interface::as_raw(self), bstrsubject.into_param().abi()).ok()
    }
    pub unsafe fn Note(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Note)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNote<P0>(&self, bstrnote: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetNote)(::windows_core::Interface::as_raw(self), bstrnote.into_param().abi()).ok()
    }
    pub unsafe fn ScheduleTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ScheduleTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScheduleTime(&self, datescheduletime: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScheduleTime)(::windows_core::Interface::as_raw(self), datescheduletime).ok()
    }
    pub unsafe fn ReceiptAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReceiptAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReceiptAddress<P0>(&self, bstrreceiptaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetReceiptAddress)(::windows_core::Interface::as_raw(self), bstrreceiptaddress.into_param().abi()).ok()
    }
    pub unsafe fn DocumentName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DocumentName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDocumentName<P0>(&self, bstrdocumentname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDocumentName)(::windows_core::Interface::as_raw(self), bstrdocumentname.into_param().abi()).ok()
    }
    pub unsafe fn CallHandle(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CallHandle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCallHandle(&self, lcallhandle: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCallHandle)(::windows_core::Interface::as_raw(self), lcallhandle).ok()
    }
    pub unsafe fn CoverPageType(&self) -> ::windows_core::Result<FAX_COVERPAGE_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CoverPageType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCoverPageType(&self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCoverPageType)(::windows_core::Interface::as_raw(self), coverpagetype).ok()
    }
    pub unsafe fn ScheduleType(&self) -> ::windows_core::Result<FAX_SCHEDULE_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ScheduleType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScheduleType(&self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScheduleType)(::windows_core::Interface::as_raw(self), scheduletype).ok()
    }
    pub unsafe fn ReceiptType(&self) -> ::windows_core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReceiptType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReceiptType(&self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReceiptType)(::windows_core::Interface::as_raw(self), receipttype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GroupBroadcastReceipts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGroupBroadcastReceipts<P0>(&self, busegrouping: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetGroupBroadcastReceipts)(::windows_core::Interface::as_raw(self), busegrouping.into_param().abi()).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPriority(&self, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), priority).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TapiConnection(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TapiConnection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_TapiConnection<P0>(&self, ptapiconnection: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).putref_TapiConnection)(::windows_core::Interface::as_raw(self), ptapiconnection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit<P0>(&self, bstrfaxservername: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Submit)(::windows_core::Interface::as_raw(self), bstrfaxservername.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ConnectedSubmit<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<IFaxServer>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConnectedSubmit)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AttachFaxToReceipt(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AttachFaxToReceipt)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttachFaxToReceipt<P0>(&self, battachfax: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAttachFaxToReceipt)(::windows_core::Interface::as_raw(self), battachfax.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxDocument, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDocument {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDocument").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxDocument {
    type Vtable = IFaxDocument_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxDocument {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb207a246_09e3_4a4e_a7dc_fea31d29458f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDocument_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbody: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbody: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sender: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxrecipients: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recipients: usize,
    pub CoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcoverpage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcoverpage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubject: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnote: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetNote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnote: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ScheduleTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatescheduletime: *mut f64) -> ::windows_core::HRESULT,
    pub SetScheduleTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datescheduletime: f64) -> ::windows_core::HRESULT,
    pub ReceiptAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetReceiptAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreceiptaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DocumentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDocumentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdocumentname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CallHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcallhandle: *mut i32) -> ::windows_core::HRESULT,
    pub SetCallHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcallhandle: i32) -> ::windows_core::HRESULT,
    pub CoverPageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoverpagetype: *mut FAX_COVERPAGE_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub SetCoverPageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub ScheduleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub SetScheduleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub ReceiptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub SetReceiptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GroupBroadcastReceipts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusegrouping: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GroupBroadcastReceipts: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGroupBroadcastReceipts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busegrouping: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGroupBroadcastReceipts: usize,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TapiConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptapiconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TapiConnection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_TapiConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptapiconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_TapiConnection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ConnectedSubmit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ConnectedSubmit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AttachFaxToReceipt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbattachfax: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AttachFaxToReceipt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAttachFaxToReceipt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, battachfax: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAttachFaxToReceipt: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDocument2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDocument2 {
    pub unsafe fn Body(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Body)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBody<P0>(&self, bstrbody: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetBody)(::windows_core::Interface::as_raw(self), bstrbody.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows_core::Result<IFaxSender> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Sender)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipients(&self) -> ::windows_core::Result<IFaxRecipients> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Recipients)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CoverPage(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CoverPage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCoverPage<P0>(&self, bstrcoverpage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetCoverPage)(::windows_core::Interface::as_raw(self), bstrcoverpage.into_param().abi()).ok()
    }
    pub unsafe fn Subject(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Subject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubject<P0>(&self, bstrsubject: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetSubject)(::windows_core::Interface::as_raw(self), bstrsubject.into_param().abi()).ok()
    }
    pub unsafe fn Note(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Note)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNote<P0>(&self, bstrnote: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetNote)(::windows_core::Interface::as_raw(self), bstrnote.into_param().abi()).ok()
    }
    pub unsafe fn ScheduleTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ScheduleTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScheduleTime(&self, datescheduletime: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetScheduleTime)(::windows_core::Interface::as_raw(self), datescheduletime).ok()
    }
    pub unsafe fn ReceiptAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReceiptAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReceiptAddress<P0>(&self, bstrreceiptaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetReceiptAddress)(::windows_core::Interface::as_raw(self), bstrreceiptaddress.into_param().abi()).ok()
    }
    pub unsafe fn DocumentName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DocumentName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDocumentName<P0>(&self, bstrdocumentname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDocumentName)(::windows_core::Interface::as_raw(self), bstrdocumentname.into_param().abi()).ok()
    }
    pub unsafe fn CallHandle(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CallHandle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCallHandle(&self, lcallhandle: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCallHandle)(::windows_core::Interface::as_raw(self), lcallhandle).ok()
    }
    pub unsafe fn CoverPageType(&self) -> ::windows_core::Result<FAX_COVERPAGE_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CoverPageType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCoverPageType(&self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCoverPageType)(::windows_core::Interface::as_raw(self), coverpagetype).ok()
    }
    pub unsafe fn ScheduleType(&self) -> ::windows_core::Result<FAX_SCHEDULE_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ScheduleType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScheduleType(&self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetScheduleType)(::windows_core::Interface::as_raw(self), scheduletype).ok()
    }
    pub unsafe fn ReceiptType(&self) -> ::windows_core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReceiptType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReceiptType(&self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetReceiptType)(::windows_core::Interface::as_raw(self), receipttype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GroupBroadcastReceipts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGroupBroadcastReceipts<P0>(&self, busegrouping: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetGroupBroadcastReceipts)(::windows_core::Interface::as_raw(self), busegrouping.into_param().abi()).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Priority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPriority(&self, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPriority)(::windows_core::Interface::as_raw(self), priority).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TapiConnection(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TapiConnection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_TapiConnection<P0>(&self, ptapiconnection: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.putref_TapiConnection)(::windows_core::Interface::as_raw(self), ptapiconnection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit<P0>(&self, bstrfaxservername: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Submit)(::windows_core::Interface::as_raw(self), bstrfaxservername.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ConnectedSubmit<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<IFaxServer>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ConnectedSubmit)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AttachFaxToReceipt(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.AttachFaxToReceipt)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttachFaxToReceipt<P0>(&self, battachfax: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetAttachFaxToReceipt)(::windows_core::Interface::as_raw(self), battachfax.into_param().abi()).ok()
    }
    pub unsafe fn SubmissionId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubmissionId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Bodies(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Bodies)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetBodies(&self, vbodies: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBodies)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbodies)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit2<P0>(&self, bstrfaxservername: P0, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Submit2)(::windows_core::Interface::as_raw(self), bstrfaxservername.into_param().abi(), pvfaxoutgoingjobids, plerrorbodyfile).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ConnectedSubmit2<P0>(&self, pfaxserver: P0, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer>,
    {
        (::windows_core::Interface::vtable(self).ConnectedSubmit2)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), pvfaxoutgoingjobids, plerrorbodyfile).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxDocument2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFaxDocument);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDocument2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDocument2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDocument2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDocument2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxDocument2 {
    type Vtable = IFaxDocument2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDocument2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxDocument2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1347661_f9ef_4d6d_b4a5_c0a068b65cff);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDocument2_Vtbl {
    pub base__: IFaxDocument_Vtbl,
    pub SubmissionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Bodies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbodies: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Bodies: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetBodies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbodies: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetBodies: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Submit2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Submit2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ConnectedSubmit2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ConnectedSubmit2: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxEventLogging(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxEventLogging {
    pub unsafe fn InitEventsLevel(&self) -> ::windows_core::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InitEventsLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInitEventsLevel(&self, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInitEventsLevel)(::windows_core::Interface::as_raw(self), initeventlevel).ok()
    }
    pub unsafe fn InboundEventsLevel(&self) -> ::windows_core::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InboundEventsLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInboundEventsLevel(&self, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInboundEventsLevel)(::windows_core::Interface::as_raw(self), inboundeventlevel).ok()
    }
    pub unsafe fn OutboundEventsLevel(&self) -> ::windows_core::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OutboundEventsLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOutboundEventsLevel(&self, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutboundEventsLevel)(::windows_core::Interface::as_raw(self), outboundeventlevel).ok()
    }
    pub unsafe fn GeneralEventsLevel(&self) -> ::windows_core::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GeneralEventsLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGeneralEventsLevel(&self, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGeneralEventsLevel)(::windows_core::Interface::as_raw(self), generaleventlevel).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxEventLogging, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxEventLogging {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxEventLogging {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxEventLogging {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxEventLogging").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxEventLogging {
    type Vtable = IFaxEventLogging_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxEventLogging {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxEventLogging {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0880d965_20e8_42e4_8e17_944f192caad4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxEventLogging_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub InitEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piniteventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT,
    pub SetInitEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT,
    pub InboundEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT,
    pub SetInboundEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT,
    pub OutboundEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT,
    pub SetOutboundEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT,
    pub GeneralEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgeneraleventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT,
    pub SetGeneralEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxFolders(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxFolders {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutgoingQueue(&self) -> ::windows_core::Result<IFaxOutgoingQueue> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OutgoingQueue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncomingQueue(&self) -> ::windows_core::Result<IFaxIncomingQueue> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IncomingQueue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncomingArchive(&self) -> ::windows_core::Result<IFaxIncomingArchive> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IncomingArchive)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutgoingArchive(&self) -> ::windows_core::Result<IFaxOutgoingArchive> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OutgoingArchive)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxFolders, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxFolders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxFolders {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxFolders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxFolders").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxFolders {
    type Vtable = IFaxFolders_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxFolders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxFolders {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdce3b2a8_a7ab_42bc_9d0a_3149457261a0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxFolders_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingArchive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingArchive: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxInboundRouting(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRouting {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExtensions(&self) -> ::windows_core::Result<IFaxInboundRoutingExtensions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetExtensions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMethods(&self) -> ::windows_core::Result<IFaxInboundRoutingMethods> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMethods)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxInboundRouting, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxInboundRouting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxInboundRouting {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxInboundRouting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxInboundRouting").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxInboundRouting {
    type Vtable = IFaxInboundRouting_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxInboundRouting {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxInboundRouting {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8148c20f_9d52_45b1_bf96_38fc12713527);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRouting_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxinboundroutingextensions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxinboundroutingmethods: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMethods: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxInboundRoutingExtension(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingExtension {
    pub unsafe fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FriendlyName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ImageName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ImageName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UniqueName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UniqueName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MajorVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MinorVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MajorBuild(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MajorBuild)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MinorBuild(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MinorBuild)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Debug(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Debug)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<FAX_PROVIDER_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitErrorCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InitErrorCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Methods(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Methods)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxInboundRoutingExtension, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxInboundRoutingExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxInboundRoutingExtension {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxInboundRoutingExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxInboundRoutingExtension").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxInboundRoutingExtension {
    type Vtable = IFaxInboundRoutingExtension_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxInboundRoutingExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxInboundRoutingExtension {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x885b5e08_c26c_4ef9_af83_51580a750be1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingExtension_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ImageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrimagename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub UniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruniquename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows_core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows_core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Debug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Debug: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows_core::HRESULT,
    pub InitErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Methods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvmethods: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Methods: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxInboundRoutingExtensions(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingExtensions {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxInboundRoutingExtension> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vindex), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxInboundRoutingExtensions, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxInboundRoutingExtensions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxInboundRoutingExtensions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxInboundRoutingExtensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxInboundRoutingExtensions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxInboundRoutingExtensions {
    type Vtable = IFaxInboundRoutingExtensions_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxInboundRoutingExtensions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxInboundRoutingExtensions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f6c9673_7b26_42de_8eb0_915dcd2a4f4c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingExtensions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxinboundroutingextension: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxInboundRoutingMethod(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingMethod {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FunctionName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FunctionName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExtensionFriendlyName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExtensionFriendlyName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExtensionImageName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExtensionImageName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), lpriority).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxInboundRoutingMethod, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxInboundRoutingMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxInboundRoutingMethod {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxInboundRoutingMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxInboundRoutingMethod").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxInboundRoutingMethod {
    type Vtable = IFaxInboundRoutingMethod_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxInboundRoutingMethod {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxInboundRoutingMethod {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45700061_ad9d_4776_a8c4_64065492cf4b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingMethod_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FunctionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfunctionname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ExtensionFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextensionfriendlyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ExtensionImageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextensionimagename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxInboundRoutingMethods(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingMethods {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxInboundRoutingMethod> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vindex), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxInboundRoutingMethods, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxInboundRoutingMethods {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxInboundRoutingMethods {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxInboundRoutingMethods {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxInboundRoutingMethods").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxInboundRoutingMethods {
    type Vtable = IFaxInboundRoutingMethods_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxInboundRoutingMethods {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxInboundRoutingMethods {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x783fca10_8908_4473_9d69_f67fbea0c6b9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingMethods_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxinboundroutingmethod: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingArchive(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingArchive {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseArchive(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UseArchive)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseArchive<P0>(&self, busearchive: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUseArchive)(::windows_core::Interface::as_raw(self), busearchive.into_param().abi()).ok()
    }
    pub unsafe fn ArchiveFolder(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ArchiveFolder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetArchiveFolder<P0>(&self, bstrarchivefolder: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetArchiveFolder)(::windows_core::Interface::as_raw(self), bstrarchivefolder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SizeQuotaWarning(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SizeQuotaWarning)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSizeQuotaWarning<P0>(&self, bsizequotawarning: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetSizeQuotaWarning)(::windows_core::Interface::as_raw(self), bsizequotawarning.into_param().abi()).ok()
    }
    pub unsafe fn HighQuotaWaterMark(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HighQuotaWaterMark)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHighQuotaWaterMark)(::windows_core::Interface::as_raw(self), lhighquotawatermark).ok()
    }
    pub unsafe fn LowQuotaWaterMark(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LowQuotaWaterMark)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLowQuotaWaterMark)(::windows_core::Interface::as_raw(self), llowquotawatermark).ok()
    }
    pub unsafe fn AgeLimit(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AgeLimit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAgeLimit)(::windows_core::Interface::as_raw(self), lagelimit).ok()
    }
    pub unsafe fn SizeLow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SizeLow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SizeHigh(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SizeHigh)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows_core::Result<IFaxIncomingMessageIterator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMessages)(::windows_core::Interface::as_raw(self), lprefetchsize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessage<P0>(&self, bstrmessageid: P0) -> ::windows_core::Result<IFaxIncomingMessage>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMessage)(::windows_core::Interface::as_raw(self), bstrmessageid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxIncomingArchive, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingArchive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingArchive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingArchive").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxIncomingArchive {
    type Vtable = IFaxIncomingArchive_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingArchive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxIncomingArchive {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76062cc7_f714_4fbd_aa06_ed6e4a4b70f3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingArchive_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub UseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseArchive: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseArchive: usize,
    pub ArchiveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetArchiveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SizeQuotaWarning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSizeQuotaWarning: usize,
    pub HighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows_core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows_core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows_core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows_core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows_core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows_core::HRESULT,
    pub SizeLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows_core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessage: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingJob(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingJob {
    pub unsafe fn Size(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Size)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentPage(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentPage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceId(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExtendedStatusCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExtendedStatus(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExtendedStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AvailableOperations(&self) -> ::windows_core::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AvailableOperations)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Retries)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransmissionStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransmissionEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CallerId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CallerId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RoutingInformation(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RoutingInformation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn JobType(&self) -> ::windows_core::Result<FAX_JOB_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).JobType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CopyTiff<P0>(&self, bstrtiffpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).CopyTiff)(::windows_core::Interface::as_raw(self), bstrtiffpath.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxIncomingJob, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxIncomingJob {
    type Vtable = IFaxIncomingJob_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxIncomingJob {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x207529e6_654a_4916_9f88_4d232ee8a107);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingJob_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CurrentPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows_core::HRESULT,
    pub ExtendedStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AvailableOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows_core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CallerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RoutingInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub JobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CopyTiff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingJobs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingJobs {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxIncomingJob> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vindex), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxIncomingJobs, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingJobs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingJobs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingJobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingJobs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxIncomingJobs {
    type Vtable = IFaxIncomingJobs_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingJobs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxIncomingJobs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x011f04e9_4fd6_4c23_9513_b6b66bb26be9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingJobs_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingMessage(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessage {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Pages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Size(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Size)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Retries)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransmissionStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransmissionEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CallerId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CallerId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RoutingInformation(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RoutingInformation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CopyTiff<P0>(&self, bstrtiffpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).CopyTiff)(::windows_core::Interface::as_raw(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxIncomingMessage, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingMessage {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxIncomingMessage {
    type Vtable = IFaxIncomingMessage_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxIncomingMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cab88fa_2ef9_4851_b2f3_1d148fed8447);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessage_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Pages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows_core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CallerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RoutingInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CopyTiff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingMessage2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessage2 {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Pages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Size(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Size)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DeviceName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Retries)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TransmissionStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TransmissionEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CallerId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CallerId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RoutingInformation(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RoutingInformation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CopyTiff<P0>(&self, bstrtiffpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.CopyTiff)(::windows_core::Interface::as_raw(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Subject(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Subject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubject<P0>(&self, bstrsubject: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSubject)(::windows_core::Interface::as_raw(self), bstrsubject.into_param().abi()).ok()
    }
    pub unsafe fn SenderName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SenderName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSenderName<P0>(&self, bstrsendername: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSenderName)(::windows_core::Interface::as_raw(self), bstrsendername.into_param().abi()).ok()
    }
    pub unsafe fn SenderFaxNumber(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SenderFaxNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSenderFaxNumber<P0>(&self, bstrsenderfaxnumber: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSenderFaxNumber)(::windows_core::Interface::as_raw(self), bstrsenderfaxnumber.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCoverPage(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HasCoverPage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHasCoverPage<P0>(&self, bhascoverpage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetHasCoverPage)(::windows_core::Interface::as_raw(self), bhascoverpage.into_param().abi()).ok()
    }
    pub unsafe fn Recipients(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Recipients)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRecipients<P0>(&self, bstrrecipients: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRecipients)(::windows_core::Interface::as_raw(self), bstrrecipients.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WasReAssigned(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).WasReAssigned)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Read(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Read)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRead<P0>(&self, bread: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetRead)(::windows_core::Interface::as_raw(self), bread.into_param().abi()).ok()
    }
    pub unsafe fn ReAssign(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReAssign)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxIncomingMessage2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFaxIncomingMessage);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingMessage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingMessage2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingMessage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingMessage2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxIncomingMessage2 {
    type Vtable = IFaxIncomingMessage2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxIncomingMessage2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9208503_e2bc_48f3_9ec0_e6236f9b509a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessage2_Vtbl {
    pub base__: IFaxIncomingMessage_Vtbl,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubject: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SenderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsendername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSenderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsendername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SenderFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsenderfaxnumber: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSenderFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsenderfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasCoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasCoverPage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHasCoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bhascoverpage: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHasCoverPage: usize,
    pub Recipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrecipients: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRecipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrecipients: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WasReAssigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbwasreassigned: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WasReAssigned: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbread: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Read: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bread: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRead: usize,
    pub ReAssign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingMessageIterator(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessageIterator {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows_core::Result<IFaxIncomingMessage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Message)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PrefetchSize(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PrefetchSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrefetchSize(&self, lprefetchsize: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrefetchSize)(::windows_core::Interface::as_raw(self), lprefetchsize).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AtEOF(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AtEOF)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveFirst(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveFirst)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxIncomingMessageIterator, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingMessageIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingMessageIterator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingMessageIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingMessageIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxIncomingMessageIterator {
    type Vtable = IFaxIncomingMessageIterator_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingMessageIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxIncomingMessageIterator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd73ecc4_6f06_4f52_82a8_f7ba06ae3108);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessageIterator_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
    pub PrefetchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows_core::HRESULT,
    pub SetPrefetchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AtEOF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbeof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AtEOF: usize,
    pub MoveFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingQueue(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingQueue {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Blocked(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Blocked)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBlocked<P0>(&self, bblocked: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetBlocked)(::windows_core::Interface::as_raw(self), bblocked.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJobs(&self) -> ::windows_core::Result<IFaxIncomingJobs> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetJobs)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJob<P0>(&self, bstrjobid: P0) -> ::windows_core::Result<IFaxIncomingJob>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetJob)(::windows_core::Interface::as_raw(self), bstrjobid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxIncomingQueue, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxIncomingQueue {
    type Vtable = IFaxIncomingQueue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxIncomingQueue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x902e64ef_8fd8_4b75_9725_6014df161545);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Blocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Blocked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBlocked: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJob: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxJobStatus(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxJobStatus {
    pub unsafe fn Status(&self) -> ::windows_core::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Pages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Size(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Size)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentPage(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentPage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceId(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExtendedStatusCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExtendedStatus(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExtendedStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AvailableOperations(&self) -> ::windows_core::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AvailableOperations)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Retries)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn JobType(&self) -> ::windows_core::Result<FAX_JOB_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).JobType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ScheduledTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ScheduledTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransmissionStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransmissionEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CallerId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CallerId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RoutingInformation(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RoutingInformation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxJobStatus, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxJobStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxJobStatus {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxJobStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxJobStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxJobStatus {
    type Vtable = IFaxJobStatus_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxJobStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxJobStatus {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b86f485_fd7f_4824_886b_40c5caa617cc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxJobStatus_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows_core::HRESULT,
    pub Pages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT,
    pub CurrentPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ExtendedStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AvailableOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT,
    pub JobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub ScheduledTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows_core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows_core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows_core::HRESULT,
    pub CallerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RoutingInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxLoggingOptions(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxLoggingOptions {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EventLogging(&self) -> ::windows_core::Result<IFaxEventLogging> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EventLogging)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActivityLogging(&self) -> ::windows_core::Result<IFaxActivityLogging> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ActivityLogging)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxLoggingOptions, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxLoggingOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxLoggingOptions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxLoggingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxLoggingOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxLoggingOptions {
    type Vtable = IFaxLoggingOptions_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxLoggingOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxLoggingOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34e64fb9_6b31_4d32_8b27_d286c0c33606);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxLoggingOptions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EventLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxeventlogging: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EventLogging: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActivityLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxactivitylogging: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActivityLogging: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutboundRouting(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRouting {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGroups(&self) -> ::windows_core::Result<IFaxOutboundRoutingGroups> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGroups)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRules(&self) -> ::windows_core::Result<IFaxOutboundRoutingRules> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRules)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutboundRouting, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutboundRouting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutboundRouting {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutboundRouting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutboundRouting").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutboundRouting {
    type Vtable = IFaxOutboundRouting_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutboundRouting {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutboundRouting {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25dc05a4_9909_41bd_a95b_7e5d1dec1d43);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRouting_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutboundroutinggroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutboundroutingrules: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRules: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutboundRoutingGroup(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingGroup {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<FAX_GROUP_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceIds(&self) -> ::windows_core::Result<IFaxDeviceIds> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceIds)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutboundRoutingGroup, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutboundRoutingGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutboundRoutingGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutboundRoutingGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutboundRoutingGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutboundRoutingGroup {
    type Vtable = IFaxOutboundRoutingGroup_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutboundRoutingGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutboundRoutingGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca6289a1_7e25_4f87_9a0b_93365734962c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingGroup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_GROUP_STATUS_ENUM) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxdeviceids: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceIds: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutboundRoutingGroups(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingGroups {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxOutboundRoutingGroup> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vindex), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, bstrname: P0) -> ::windows_core::Result<IFaxOutboundRoutingGroup>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, vindex: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutboundRoutingGroups, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutboundRoutingGroups {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutboundRoutingGroups {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutboundRoutingGroups {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutboundRoutingGroups").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutboundRoutingGroups {
    type Vtable = IFaxOutboundRoutingGroups_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutboundRoutingGroups {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutboundRoutingGroups {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x235cbef7_c2de_4bfd_b8da_75097c82c87f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingGroups_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxoutboundroutinggroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxoutboundroutinggroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutboundRoutingRule(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingRule {
    pub unsafe fn CountryCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CountryCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AreaCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AreaCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<FAX_RULE_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseDevice(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UseDevice)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseDevice<P0>(&self, busedevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUseDevice)(::windows_core::Interface::as_raw(self), busedevice.into_param().abi()).ok()
    }
    pub unsafe fn DeviceId(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDeviceId(&self, deviceid: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDeviceId)(::windows_core::Interface::as_raw(self), deviceid).ok()
    }
    pub unsafe fn GroupName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GroupName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGroupName<P0>(&self, bstrgroupname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetGroupName)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutboundRoutingRule, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutboundRoutingRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutboundRoutingRule {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutboundRoutingRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutboundRoutingRule").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutboundRoutingRule {
    type Vtable = IFaxOutboundRoutingRule_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutboundRoutingRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutboundRoutingRule {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1f795d5_07c2_469f_b027_acacc23219da);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingRule_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CountryCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows_core::HRESULT,
    pub AreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plareacode: *mut i32) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_RULE_STATUS_ENUM) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UseDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusedevice: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseDevice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busedevice: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseDevice: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows_core::HRESULT,
    pub SetDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows_core::HRESULT,
    pub GroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutboundRoutingRules(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingRules {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<IFaxOutboundRoutingRule> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ItemByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> ::windows_core::Result<IFaxOutboundRoutingRule> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ItemByCountryAndArea)(::windows_core::Interface::as_raw(self), lcountrycode, lareacode, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveByCountryAndArea)(::windows_core::Interface::as_raw(self), lcountrycode, lareacode).ok()
    }
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), lindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<P0, P1>(&self, lcountrycode: i32, lareacode: i32, busedevice: P0, bstrgroupname: P1, ldeviceid: i32) -> ::windows_core::Result<IFaxOutboundRoutingRule>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), lcountrycode, lareacode, busedevice.into_param().abi(), bstrgroupname.into_param().abi(), ldeviceid, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutboundRoutingRules, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutboundRoutingRules {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutboundRoutingRules {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutboundRoutingRules {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutboundRoutingRules").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutboundRoutingRules {
    type Vtable = IFaxOutboundRoutingRules_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutboundRoutingRules {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutboundRoutingRules {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdcefa1e7_ae7d_4ed6_8521_369edcca5120);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingRules_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ItemByCountryAndArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ItemByCountryAndArea: usize,
    pub RemoveByCountryAndArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, busedevice: super::super::Foundation::VARIANT_BOOL, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ldeviceid: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingArchive(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingArchive {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseArchive(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UseArchive)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseArchive<P0>(&self, busearchive: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUseArchive)(::windows_core::Interface::as_raw(self), busearchive.into_param().abi()).ok()
    }
    pub unsafe fn ArchiveFolder(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ArchiveFolder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetArchiveFolder<P0>(&self, bstrarchivefolder: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetArchiveFolder)(::windows_core::Interface::as_raw(self), bstrarchivefolder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SizeQuotaWarning(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SizeQuotaWarning)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSizeQuotaWarning<P0>(&self, bsizequotawarning: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetSizeQuotaWarning)(::windows_core::Interface::as_raw(self), bsizequotawarning.into_param().abi()).ok()
    }
    pub unsafe fn HighQuotaWaterMark(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HighQuotaWaterMark)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHighQuotaWaterMark)(::windows_core::Interface::as_raw(self), lhighquotawatermark).ok()
    }
    pub unsafe fn LowQuotaWaterMark(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LowQuotaWaterMark)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLowQuotaWaterMark)(::windows_core::Interface::as_raw(self), llowquotawatermark).ok()
    }
    pub unsafe fn AgeLimit(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AgeLimit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAgeLimit)(::windows_core::Interface::as_raw(self), lagelimit).ok()
    }
    pub unsafe fn SizeLow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SizeLow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SizeHigh(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SizeHigh)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows_core::Result<IFaxOutgoingMessageIterator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMessages)(::windows_core::Interface::as_raw(self), lprefetchsize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessage<P0>(&self, bstrmessageid: P0) -> ::windows_core::Result<IFaxOutgoingMessage>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMessage)(::windows_core::Interface::as_raw(self), bstrmessageid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutgoingArchive, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingArchive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingArchive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingArchive").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutgoingArchive {
    type Vtable = IFaxOutgoingArchive_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingArchive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutgoingArchive {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9c28f40_8d80_4e53_810f_9a79919b49fd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingArchive_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub UseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseArchive: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseArchive: usize,
    pub ArchiveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetArchiveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SizeQuotaWarning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSizeQuotaWarning: usize,
    pub HighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows_core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows_core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows_core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows_core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows_core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows_core::HRESULT,
    pub SizeLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows_core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessage: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingJob(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJob {
    pub unsafe fn Subject(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Subject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DocumentName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DocumentName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Pages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Size(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Size)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SubmissionId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubmissionId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OriginalScheduledTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SubmissionTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubmissionTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReceiptType(&self) -> ::windows_core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReceiptType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows_core::Result<IFaxSender> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Sender)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipient(&self) -> ::windows_core::Result<IFaxRecipient> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Recipient)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentPage(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentPage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceId(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExtendedStatusCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExtendedStatus(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExtendedStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AvailableOperations(&self) -> ::windows_core::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AvailableOperations)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Retries)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ScheduledTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ScheduledTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransmissionStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransmissionEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GroupBroadcastReceipts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Restart(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Restart)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CopyTiff<P0>(&self, bstrtiffpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).CopyTiff)(::windows_core::Interface::as_raw(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutgoingJob, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutgoingJob {
    type Vtable = IFaxOutgoingJob_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutgoingJob {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6356daad_6614_4583_bf7a_3ad67bbfc71c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJob_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DocumentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Pages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT,
    pub SubmissionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OriginalScheduledTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows_core::HRESULT,
    pub SubmissionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows_core::HRESULT,
    pub ReceiptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sender: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recipient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recipient: usize,
    pub CurrentPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows_core::HRESULT,
    pub ExtendedStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AvailableOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT,
    pub ScheduledTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows_core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows_core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GroupBroadcastReceipts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgroupbroadcastreceipts: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GroupBroadcastReceipts: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CopyTiff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingJob2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJob2 {
    pub unsafe fn Subject(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Subject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DocumentName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DocumentName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Pages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Size(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Size)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SubmissionId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SubmissionId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.OriginalScheduledTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SubmissionTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SubmissionTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReceiptType(&self) -> ::windows_core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReceiptType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Priority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows_core::Result<IFaxSender> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Sender)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipient(&self) -> ::windows_core::Result<IFaxRecipient> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Recipient)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentPage(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CurrentPage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceId(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DeviceId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ExtendedStatusCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExtendedStatus(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ExtendedStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AvailableOperations(&self) -> ::windows_core::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.AvailableOperations)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Retries)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ScheduledTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ScheduledTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TransmissionStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TransmissionEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GroupBroadcastReceipts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Restart(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Restart)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CopyTiff<P0>(&self, bstrtiffpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.CopyTiff)(::windows_core::Interface::as_raw(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCoverPage(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HasCoverPage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReceiptAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReceiptAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ScheduleType(&self) -> ::windows_core::Result<FAX_SCHEDULE_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ScheduleType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutgoingJob2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFaxOutgoingJob);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingJob2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingJob2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingJob2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingJob2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutgoingJob2 {
    type Vtable = IFaxOutgoingJob2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingJob2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutgoingJob2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x418a8d96_59a0_4789_b176_edf3dc8fa8f7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJob2_Vtbl {
    pub base__: IFaxOutgoingJob_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub HasCoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasCoverPage: usize,
    pub ReceiptAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ScheduleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingJobs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJobs {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxOutgoingJob> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vindex), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutgoingJobs, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingJobs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingJobs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingJobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingJobs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutgoingJobs {
    type Vtable = IFaxOutgoingJobs_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingJobs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutgoingJobs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c56d8e6_8c2f_4573_944c_e505f8f5aeed);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJobs_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingMessage(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessage {
    pub unsafe fn SubmissionId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubmissionId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Subject(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Subject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DocumentName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DocumentName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Retries)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Pages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Size(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Size)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OriginalScheduledTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SubmissionTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubmissionTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows_core::Result<IFaxSender> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Sender)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipient(&self) -> ::windows_core::Result<IFaxRecipient> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Recipient)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransmissionStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransmissionEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CopyTiff<P0>(&self, bstrtiffpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).CopyTiff)(::windows_core::Interface::as_raw(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutgoingMessage, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingMessage {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutgoingMessage {
    type Vtable = IFaxOutgoingMessage_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutgoingMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0ea35de_caa5_4a7c_82c7_2b60ba5f2be2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessage_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SubmissionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DocumentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT,
    pub Pages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT,
    pub OriginalScheduledTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows_core::HRESULT,
    pub SubmissionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sender: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recipient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recipient: usize,
    pub DeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows_core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CopyTiff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingMessage2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessage2 {
    pub unsafe fn SubmissionId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SubmissionId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Subject(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Subject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DocumentName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DocumentName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Retries)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Pages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Size(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Size)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.OriginalScheduledTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SubmissionTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SubmissionTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Priority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows_core::Result<IFaxSender> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Sender)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipient(&self) -> ::windows_core::Result<IFaxRecipient> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Recipient)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DeviceName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TransmissionStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TransmissionEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CopyTiff<P0>(&self, bstrtiffpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.CopyTiff)(::windows_core::Interface::as_raw(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCoverPage(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HasCoverPage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReceiptType(&self) -> ::windows_core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReceiptType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReceiptAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReceiptAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Read(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Read)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRead<P0>(&self, bread: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetRead)(::windows_core::Interface::as_raw(self), bread.into_param().abi()).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutgoingMessage2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFaxOutgoingMessage);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingMessage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingMessage2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingMessage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingMessage2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutgoingMessage2 {
    type Vtable = IFaxOutgoingMessage2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutgoingMessage2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb37df687_bc88_4b46_b3be_b458b3ea9e7f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessage2_Vtbl {
    pub base__: IFaxOutgoingMessage_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub HasCoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasCoverPage: usize,
    pub ReceiptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub ReceiptAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbread: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Read: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bread: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRead: usize,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingMessageIterator(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessageIterator {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows_core::Result<IFaxOutgoingMessage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Message)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AtEOF(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AtEOF)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PrefetchSize(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PrefetchSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrefetchSize(&self, lprefetchsize: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrefetchSize)(::windows_core::Interface::as_raw(self), lprefetchsize).ok()
    }
    pub unsafe fn MoveFirst(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveFirst)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutgoingMessageIterator, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingMessageIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingMessageIterator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingMessageIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingMessageIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutgoingMessageIterator {
    type Vtable = IFaxOutgoingMessageIterator_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingMessageIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutgoingMessageIterator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5ec5d4f_b840_432f_9980_112fe42a9b7a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessageIterator_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AtEOF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbeof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AtEOF: usize,
    pub PrefetchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows_core::HRESULT,
    pub SetPrefetchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows_core::HRESULT,
    pub MoveFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingQueue(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingQueue {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Blocked(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Blocked)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBlocked<P0>(&self, bblocked: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetBlocked)(::windows_core::Interface::as_raw(self), bblocked.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Paused(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Paused)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPaused<P0>(&self, bpaused: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetPaused)(::windows_core::Interface::as_raw(self), bpaused.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowPersonalCoverPages(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowPersonalCoverPages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowPersonalCoverPages<P0>(&self, ballowpersonalcoverpages: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowPersonalCoverPages)(::windows_core::Interface::as_raw(self), ballowpersonalcoverpages.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseDeviceTSID(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UseDeviceTSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseDeviceTSID<P0>(&self, busedevicetsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUseDeviceTSID)(::windows_core::Interface::as_raw(self), busedevicetsid.into_param().abi()).ok()
    }
    pub unsafe fn Retries(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Retries)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRetries(&self, lretries: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRetries)(::windows_core::Interface::as_raw(self), lretries).ok()
    }
    pub unsafe fn RetryDelay(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RetryDelay)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRetryDelay(&self, lretrydelay: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRetryDelay)(::windows_core::Interface::as_raw(self), lretrydelay).ok()
    }
    pub unsafe fn DiscountRateStart(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DiscountRateStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDiscountRateStart(&self, datediscountratestart: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDiscountRateStart)(::windows_core::Interface::as_raw(self), datediscountratestart).ok()
    }
    pub unsafe fn DiscountRateEnd(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DiscountRateEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDiscountRateEnd)(::windows_core::Interface::as_raw(self), datediscountrateend).ok()
    }
    pub unsafe fn AgeLimit(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AgeLimit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAgeLimit)(::windows_core::Interface::as_raw(self), lagelimit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Branding(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Branding)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBranding<P0>(&self, bbranding: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetBranding)(::windows_core::Interface::as_raw(self), bbranding.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJobs(&self) -> ::windows_core::Result<IFaxOutgoingJobs> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetJobs)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJob<P0>(&self, bstrjobid: P0) -> ::windows_core::Result<IFaxOutgoingJob>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetJob)(::windows_core::Interface::as_raw(self), bstrjobid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxOutgoingQueue, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxOutgoingQueue {
    type Vtable = IFaxOutgoingQueue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxOutgoingQueue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80b1df24_d9ac_4333_b373_487cedc80ce5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Blocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Blocked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBlocked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Paused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpaused: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Paused: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpaused: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPaused: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowPersonalCoverPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowPersonalCoverPages: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowPersonalCoverPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowPersonalCoverPages: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UseDeviceTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseDeviceTSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseDeviceTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseDeviceTSID: usize,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT,
    pub SetRetries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows_core::HRESULT,
    pub RetryDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows_core::HRESULT,
    pub SetRetryDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows_core::HRESULT,
    pub DiscountRateStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows_core::HRESULT,
    pub SetDiscountRateStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows_core::HRESULT,
    pub DiscountRateEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows_core::HRESULT,
    pub SetDiscountRateEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows_core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows_core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Branding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbranding: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Branding: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBranding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bbranding: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBranding: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJob: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxReceiptOptions(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxReceiptOptions {
    pub unsafe fn AuthenticationType(&self) -> ::windows_core::Result<FAX_SMTP_AUTHENTICATION_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AuthenticationType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAuthenticationType(&self, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticationType)(::windows_core::Interface::as_raw(self), r#type).ok()
    }
    pub unsafe fn SMTPServer(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SMTPServer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSMTPServer<P0>(&self, bstrsmtpserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSMTPServer)(::windows_core::Interface::as_raw(self), bstrsmtpserver.into_param().abi()).ok()
    }
    pub unsafe fn SMTPPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SMTPPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSMTPPort(&self, lsmtpport: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSMTPPort)(::windows_core::Interface::as_raw(self), lsmtpport).ok()
    }
    pub unsafe fn SMTPSender(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SMTPSender)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSMTPSender<P0>(&self, bstrsmtpsender: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSMTPSender)(::windows_core::Interface::as_raw(self), bstrsmtpsender.into_param().abi()).ok()
    }
    pub unsafe fn SMTPUser(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SMTPUser)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSMTPUser<P0>(&self, bstrsmtpuser: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSMTPUser)(::windows_core::Interface::as_raw(self), bstrsmtpuser.into_param().abi()).ok()
    }
    pub unsafe fn AllowedReceipts(&self) -> ::windows_core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowedReceipts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAllowedReceipts(&self, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowedReceipts)(::windows_core::Interface::as_raw(self), allowedreceipts).ok()
    }
    pub unsafe fn SMTPPassword(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SMTPPassword)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSMTPPassword<P0>(&self, bstrsmtppassword: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSMTPPassword)(::windows_core::Interface::as_raw(self), bstrsmtppassword.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseForInboundRouting(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UseForInboundRouting)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseForInboundRouting<P0>(&self, buseforinboundrouting: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUseForInboundRouting)(::windows_core::Interface::as_raw(self), buseforinboundrouting.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxReceiptOptions, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxReceiptOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxReceiptOptions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxReceiptOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxReceiptOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxReceiptOptions {
    type Vtable = IFaxReceiptOptions_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxReceiptOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxReceiptOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x378efaeb_5fcb_4afb_b2ee_e16e80614487);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxReceiptOptions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub SMTPServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsmtpserver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSMTPServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsmtpserver: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SMTPPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsmtpport: *mut i32) -> ::windows_core::HRESULT,
    pub SetSMTPPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsmtpport: i32) -> ::windows_core::HRESULT,
    pub SMTPSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsmtpsender: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSMTPSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsmtpsender: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SMTPUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsmtpuser: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSMTPUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsmtpuser: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AllowedReceipts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pallowedreceipts: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub SetAllowedReceipts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub SMTPPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsmtppassword: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSMTPPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsmtppassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UseForInboundRouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuseforinboundrouting: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseForInboundRouting: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseForInboundRouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buseforinboundrouting: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseForInboundRouting: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxRecipient(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxRecipient {
    pub unsafe fn FaxNumber(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FaxNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFaxNumber<P0>(&self, bstrfaxnumber: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFaxNumber)(::windows_core::Interface::as_raw(self), bstrfaxnumber.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxRecipient, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxRecipient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxRecipient {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxRecipient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxRecipient").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxRecipient {
    type Vtable = IFaxRecipient_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxRecipient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxRecipient {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a3da3a0_538d_42b6_9444_aaa57d0ce2bc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxRecipient_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub FaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxRecipients(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxRecipients {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<IFaxRecipient> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0, P1>(&self, bstrfaxnumber: P0, bstrrecipientname: P1) -> ::windows_core::Result<IFaxRecipient>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), bstrfaxnumber.into_param().abi(), bstrrecipientname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), lindex).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxRecipients, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxRecipients {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxRecipients {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxRecipients {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxRecipients").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxRecipients {
    type Vtable = IFaxRecipients_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxRecipients {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxRecipients {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9c9de5a_894e_4492_9fa3_08c627c11d5d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxRecipients_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrecipientname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxSecurity(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxSecurity {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Descriptor(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Descriptor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDescriptor(&self, vdescriptor: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vdescriptor)).ok()
    }
    pub unsafe fn GrantedRights(&self) -> ::windows_core::Result<FAX_ACCESS_RIGHTS_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GrantedRights)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InformationType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InformationType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInformationType(&self, linformationtype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInformationType)(::windows_core::Interface::as_raw(self), linformationtype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxSecurity, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxSecurity {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxSecurity").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxSecurity {
    type Vtable = IFaxSecurity_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxSecurity {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77b508c1_09c0_47a2_91eb_fce7fdf2690e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSecurity_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Descriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Descriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vdescriptor: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDescriptor: usize,
    pub GrantedRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InformationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows_core::HRESULT,
    pub SetInformationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxSecurity2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxSecurity2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Descriptor(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Descriptor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDescriptor(&self, vdescriptor: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vdescriptor)).ok()
    }
    pub unsafe fn GrantedRights(&self) -> ::windows_core::Result<FAX_ACCESS_RIGHTS_ENUM_2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GrantedRights)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InformationType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InformationType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInformationType(&self, linformationtype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInformationType)(::windows_core::Interface::as_raw(self), linformationtype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxSecurity2, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxSecurity2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxSecurity2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxSecurity2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxSecurity2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxSecurity2 {
    type Vtable = IFaxSecurity2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxSecurity2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxSecurity2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17d851f4_d09b_48fc_99c9_8f24c4db9ab1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSecurity2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Descriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Descriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vdescriptor: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDescriptor: usize,
    pub GrantedRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM_2) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InformationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows_core::HRESULT,
    pub SetInformationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxSender(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxSender {
    pub unsafe fn BillingCode(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BillingCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBillingCode<P0>(&self, bstrbillingcode: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetBillingCode)(::windows_core::Interface::as_raw(self), bstrbillingcode.into_param().abi()).ok()
    }
    pub unsafe fn City(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).City)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCity<P0>(&self, bstrcity: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCity)(::windows_core::Interface::as_raw(self), bstrcity.into_param().abi()).ok()
    }
    pub unsafe fn Company(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Company)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCompany<P0>(&self, bstrcompany: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCompany)(::windows_core::Interface::as_raw(self), bstrcompany.into_param().abi()).ok()
    }
    pub unsafe fn Country(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Country)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCountry<P0>(&self, bstrcountry: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCountry)(::windows_core::Interface::as_raw(self), bstrcountry.into_param().abi()).ok()
    }
    pub unsafe fn Department(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Department)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDepartment<P0>(&self, bstrdepartment: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDepartment)(::windows_core::Interface::as_raw(self), bstrdepartment.into_param().abi()).ok()
    }
    pub unsafe fn Email(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Email)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEmail<P0>(&self, bstremail: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEmail)(::windows_core::Interface::as_raw(self), bstremail.into_param().abi()).ok()
    }
    pub unsafe fn FaxNumber(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FaxNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFaxNumber<P0>(&self, bstrfaxnumber: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFaxNumber)(::windows_core::Interface::as_raw(self), bstrfaxnumber.into_param().abi()).ok()
    }
    pub unsafe fn HomePhone(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HomePhone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHomePhone<P0>(&self, bstrhomephone: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetHomePhone)(::windows_core::Interface::as_raw(self), bstrhomephone.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn TSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTSID<P0>(&self, bstrtsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTSID)(::windows_core::Interface::as_raw(self), bstrtsid.into_param().abi()).ok()
    }
    pub unsafe fn OfficePhone(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OfficePhone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOfficePhone<P0>(&self, bstrofficephone: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOfficePhone)(::windows_core::Interface::as_raw(self), bstrofficephone.into_param().abi()).ok()
    }
    pub unsafe fn OfficeLocation(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OfficeLocation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOfficeLocation<P0>(&self, bstrofficelocation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOfficeLocation)(::windows_core::Interface::as_raw(self), bstrofficelocation.into_param().abi()).ok()
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetState<P0>(&self, bstrstate: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetState)(::windows_core::Interface::as_raw(self), bstrstate.into_param().abi()).ok()
    }
    pub unsafe fn StreetAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StreetAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreetAddress<P0>(&self, bstrstreetaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetStreetAddress)(::windows_core::Interface::as_raw(self), bstrstreetaddress.into_param().abi()).ok()
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTitle<P0>(&self, bstrtitle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTitle)(::windows_core::Interface::as_raw(self), bstrtitle.into_param().abi()).ok()
    }
    pub unsafe fn ZipCode(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ZipCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetZipCode<P0>(&self, bstrzipcode: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetZipCode)(::windows_core::Interface::as_raw(self), bstrzipcode.into_param().abi()).ok()
    }
    pub unsafe fn LoadDefaultSender(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadDefaultSender)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SaveDefaultSender(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SaveDefaultSender)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxSender, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxSender {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxSender {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxSender {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxSender").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxSender {
    type Vtable = IFaxSender_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxSender {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxSender {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d879d7d_f57a_4cc6_a6f9_3ee5d527b46a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSender_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub BillingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbillingcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetBillingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbillingcode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcity: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcity: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Company: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcompany: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCompany: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcompany: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcountry: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcountry: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Department: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdepartment: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdepartment: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Email: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstremail: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstremail: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub HomePhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhomephone: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetHomePhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhomephone: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OfficePhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrofficephone: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOfficePhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrofficephone: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OfficeLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrofficelocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOfficeLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrofficelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstate: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstate: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub StreetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstreetaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetStreetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstreetaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ZipCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrzipcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetZipCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrzipcode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LoadDefaultSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SaveDefaultSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxServer(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxServer {
    pub unsafe fn Connect<P0>(&self, bstrservername: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self), bstrservername.into_param().abi()).ok()
    }
    pub unsafe fn ServerName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServerName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDeviceProviders(&self) -> ::windows_core::Result<IFaxDeviceProviders> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceProviders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDevices(&self) -> ::windows_core::Result<IFaxDevices> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDevices)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InboundRouting(&self) -> ::windows_core::Result<IFaxInboundRouting> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InboundRouting)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Folders(&self) -> ::windows_core::Result<IFaxFolders> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Folders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoggingOptions(&self) -> ::windows_core::Result<IFaxLoggingOptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoggingOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MajorVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MinorVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MajorBuild(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MajorBuild)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MinorBuild(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MinorBuild)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Debug(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Debug)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Activity(&self) -> ::windows_core::Result<IFaxActivity> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Activity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutboundRouting(&self) -> ::windows_core::Result<IFaxOutboundRouting> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OutboundRouting)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiptOptions(&self) -> ::windows_core::Result<IFaxReceiptOptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReceiptOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security(&self) -> ::windows_core::Result<IFaxSecurity> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Security)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Disconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetExtensionProperty<P0>(&self, bstrguid: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetExtensionProperty)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetExtensionProperty<P0>(&self, bstrguid: P0, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetExtensionProperty)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), ::core::mem::transmute(vproperty)).ok()
    }
    pub unsafe fn ListenToServerEvents(&self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ListenToServerEvents)(::windows_core::Interface::as_raw(self), eventtypes).ok()
    }
    pub unsafe fn RegisterDeviceProvider<P0, P1, P2, P3>(&self, bstrguid: P0, bstrfriendlyname: P1, bstrimagename: P2, tspname: P3, lfspiversion: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterDeviceProvider)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), tspname.into_param().abi(), lfspiversion).ok()
    }
    pub unsafe fn UnregisterDeviceProvider<P0>(&self, bstruniquename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).UnregisterDeviceProvider)(::windows_core::Interface::as_raw(self), bstruniquename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RegisterInboundRoutingExtension<P0, P1, P2>(&self, bstrextensionname: P0, bstrfriendlyname: P1, bstrimagename: P2, vmethods: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterInboundRoutingExtension)(::windows_core::Interface::as_raw(self), bstrextensionname.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), ::core::mem::transmute(vmethods)).ok()
    }
    pub unsafe fn UnregisterInboundRoutingExtension<P0>(&self, bstrextensionuniquename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).UnregisterInboundRoutingExtension)(::windows_core::Interface::as_raw(self), bstrextensionuniquename.into_param().abi()).ok()
    }
    pub unsafe fn RegisteredEvents(&self) -> ::windows_core::Result<FAX_SERVER_EVENTS_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisteredEvents)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn APIVersion(&self) -> ::windows_core::Result<FAX_SERVER_APIVERSION_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).APIVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxServer, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxServer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxServer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxServer {
    type Vtable = IFaxServer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxServer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxServer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x475b6469_90a5_4878_a577_17a86e8e3462);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServer_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrservername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ServerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrservername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDeviceProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxdeviceproviders: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDeviceProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxdevices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDevices: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InboundRouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxinboundrouting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InboundRouting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Folders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxfolders: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Folders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LoggingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxloggingoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoggingOptions: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows_core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows_core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Debug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Debug: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Activity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxactivity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Activity: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutboundRouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxoutboundrouting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutboundRouting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiptOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxreceiptoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiptOptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetExtensionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvproperty: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetExtensionProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetExtensionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetExtensionProperty: usize,
    pub ListenToServerEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub RegisterDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrimagename: ::std::mem::MaybeUninit<::windows_core::BSTR>, tspname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lfspiversion: i32) -> ::windows_core::HRESULT,
    pub UnregisterDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstruniquename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RegisterInboundRoutingExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrextensionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrimagename: ::std::mem::MaybeUninit<::windows_core::BSTR>, vmethods: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RegisterInboundRoutingExtension: usize,
    pub UnregisterInboundRoutingExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrextensionuniquename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RegisteredEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtypes: *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows_core::HRESULT,
    pub APIVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papiversion: *mut FAX_SERVER_APIVERSION_ENUM) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxServer2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxServer2 {
    pub unsafe fn Connect<P0>(&self, bstrservername: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Connect)(::windows_core::Interface::as_raw(self), bstrservername.into_param().abi()).ok()
    }
    pub unsafe fn ServerName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServerName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDeviceProviders(&self) -> ::windows_core::Result<IFaxDeviceProviders> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDeviceProviders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDevices(&self) -> ::windows_core::Result<IFaxDevices> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDevices)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InboundRouting(&self) -> ::windows_core::Result<IFaxInboundRouting> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.InboundRouting)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Folders(&self) -> ::windows_core::Result<IFaxFolders> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Folders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoggingOptions(&self) -> ::windows_core::Result<IFaxLoggingOptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LoggingOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MajorVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MinorVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MajorBuild(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MajorBuild)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MinorBuild(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MinorBuild)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Debug(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Debug)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Activity(&self) -> ::windows_core::Result<IFaxActivity> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Activity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutboundRouting(&self) -> ::windows_core::Result<IFaxOutboundRouting> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.OutboundRouting)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiptOptions(&self) -> ::windows_core::Result<IFaxReceiptOptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReceiptOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security(&self) -> ::windows_core::Result<IFaxSecurity> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Security)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Disconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Disconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetExtensionProperty<P0>(&self, bstrguid: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetExtensionProperty)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetExtensionProperty<P0>(&self, bstrguid: P0, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetExtensionProperty)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), ::core::mem::transmute(vproperty)).ok()
    }
    pub unsafe fn ListenToServerEvents(&self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ListenToServerEvents)(::windows_core::Interface::as_raw(self), eventtypes).ok()
    }
    pub unsafe fn RegisterDeviceProvider<P0, P1, P2, P3>(&self, bstrguid: P0, bstrfriendlyname: P1, bstrimagename: P2, tspname: P3, lfspiversion: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.RegisterDeviceProvider)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), tspname.into_param().abi(), lfspiversion).ok()
    }
    pub unsafe fn UnregisterDeviceProvider<P0>(&self, bstruniquename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.UnregisterDeviceProvider)(::windows_core::Interface::as_raw(self), bstruniquename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RegisterInboundRoutingExtension<P0, P1, P2>(&self, bstrextensionname: P0, bstrfriendlyname: P1, bstrimagename: P2, vmethods: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.RegisterInboundRoutingExtension)(::windows_core::Interface::as_raw(self), bstrextensionname.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), ::core::mem::transmute(vmethods)).ok()
    }
    pub unsafe fn UnregisterInboundRoutingExtension<P0>(&self, bstrextensionuniquename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.UnregisterInboundRoutingExtension)(::windows_core::Interface::as_raw(self), bstrextensionuniquename.into_param().abi()).ok()
    }
    pub unsafe fn RegisteredEvents(&self) -> ::windows_core::Result<FAX_SERVER_EVENTS_TYPE_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RegisteredEvents)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn APIVersion(&self) -> ::windows_core::Result<FAX_SERVER_APIVERSION_ENUM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.APIVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Configuration(&self) -> ::windows_core::Result<IFaxConfiguration> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Configuration)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentAccount(&self) -> ::windows_core::Result<IFaxAccount> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentAccount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FaxAccountSet(&self) -> ::windows_core::Result<IFaxAccountSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FaxAccountSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security2(&self) -> ::windows_core::Result<IFaxSecurity2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Security2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxServer2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFaxServer);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxServer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxServer2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxServer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxServer2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxServer2 {
    type Vtable = IFaxServer2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxServer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxServer2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x571ced0f_5609_4f40_9176_547e3a72ca7c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServer2_Vtbl {
    pub base__: IFaxServer_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Configuration: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcurrentaccount: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentAccount: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FaxAccountSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxaccountset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FaxAccountSet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxsecurity2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security2: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxServerNotify(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxServerNotify {}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxServerNotify, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxServerNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxServerNotify {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxServerNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxServerNotify").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxServerNotify {
    type Vtable = IFaxServerNotify_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxServerNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxServerNotify {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e037b27_cf8a_4abd_b1e0_5704943bea6f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServerNotify_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxServerNotify2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxServerNotify2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnIncomingJobAdded<P0, P1>(&self, pfaxserver: P0, bstrjobid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnIncomingJobAdded)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnIncomingJobRemoved<P0, P1>(&self, pfaxserver: P0, bstrjobid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnIncomingJobRemoved)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnIncomingJobChanged<P0, P1, P2>(&self, pfaxserver: P0, bstrjobid: P1, pjobstatus: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<IFaxJobStatus>,
    {
        (::windows_core::Interface::vtable(self).OnIncomingJobChanged)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingJobAdded<P0, P1>(&self, pfaxserver: P0, bstrjobid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingJobAdded)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingJobRemoved<P0, P1>(&self, pfaxserver: P0, bstrjobid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingJobRemoved)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingJobChanged<P0, P1, P2>(&self, pfaxserver: P0, bstrjobid: P1, pjobstatus: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<IFaxJobStatus>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingJobChanged)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnIncomingMessageAdded<P0, P1>(&self, pfaxserver: P0, bstrmessageid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnIncomingMessageAdded)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnIncomingMessageRemoved<P0, P1>(&self, pfaxserver: P0, bstrmessageid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnIncomingMessageRemoved)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingMessageAdded<P0, P1>(&self, pfaxserver: P0, bstrmessageid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingMessageAdded)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingMessageRemoved<P0, P1>(&self, pfaxserver: P0, bstrmessageid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingMessageRemoved)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnReceiptOptionsChange<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnReceiptOptionsChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnActivityLoggingConfigChange<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnActivityLoggingConfigChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSecurityConfigChange<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnSecurityConfigChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventLoggingConfigChange<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnEventLoggingConfigChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingQueueConfigChange<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingQueueConfigChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingArchiveConfigChange<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnOutgoingArchiveConfigChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnIncomingArchiveConfigChange<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnIncomingArchiveConfigChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnDevicesConfigChange<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnDevicesConfigChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutboundRoutingGroupsConfigChange<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnOutboundRoutingGroupsConfigChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutboundRoutingRulesConfigChange<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnOutboundRoutingRulesConfigChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnServerActivityChange<P0>(&self, pfaxserver: P0, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnServerActivityChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), lincomingmessages, lroutingmessages, loutgoingmessages, lqueuedmessages).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnQueuesStatusChange<P0, P1, P2, P3>(&self, pfaxserver: P0, boutgoingqueueblocked: P1, boutgoingqueuepaused: P2, bincomingqueueblocked: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).OnQueuesStatusChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), boutgoingqueueblocked.into_param().abi(), boutgoingqueuepaused.into_param().abi(), bincomingqueueblocked.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnNewCall<P0, P1>(&self, pfaxserver: P0, lcallid: i32, ldeviceid: i32, bstrcallerid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnNewCall)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), lcallid, ldeviceid, bstrcallerid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnServerShutDown<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnServerShutDown)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnDeviceStatusChange<P0, P1, P2, P3, P4>(&self, pfaxserver: P0, ldeviceid: i32, bpoweredoff: P1, bsending: P2, breceiving: P3, bringing: P4) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).OnDeviceStatusChange)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi(), ldeviceid, bpoweredoff.into_param().abi(), bsending.into_param().abi(), breceiving.into_param().abi(), bringing.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnGeneralServerConfigChanged<P0>(&self, pfaxserver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFaxServer2>,
    {
        (::windows_core::Interface::vtable(self).OnGeneralServerConfigChanged)(::windows_core::Interface::as_raw(self), pfaxserver.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFaxServerNotify2, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxServerNotify2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxServerNotify2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxServerNotify2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxServerNotify2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFaxServerNotify2 {
    type Vtable = IFaxServerNotify2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxServerNotify2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFaxServerNotify2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec9c69b9_5fe7_4805_9467_82fcd96af903);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServerNotify2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnIncomingJobAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnIncomingJobAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnIncomingJobRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnIncomingJobRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnIncomingJobChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnIncomingJobChanged: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingJobAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingJobAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingJobRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingJobRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingJobChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingJobChanged: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnIncomingMessageAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnIncomingMessageAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnIncomingMessageRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnIncomingMessageRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingMessageAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingMessageAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingMessageRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingMessageRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnReceiptOptionsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnReceiptOptionsChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnActivityLoggingConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnActivityLoggingConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSecurityConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSecurityConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnEventLoggingConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnEventLoggingConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingQueueConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingQueueConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingArchiveConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingArchiveConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnIncomingArchiveConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnIncomingArchiveConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnDevicesConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnDevicesConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutboundRoutingGroupsConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutboundRoutingGroupsConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutboundRoutingRulesConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutboundRoutingRulesConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnServerActivityChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnServerActivityChange: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnQueuesStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, boutgoingqueueblocked: super::super::Foundation::VARIANT_BOOL, boutgoingqueuepaused: super::super::Foundation::VARIANT_BOOL, bincomingqueueblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnQueuesStatusChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnNewCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, lcallid: i32, ldeviceid: i32, bstrcallerid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnNewCall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnServerShutDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnServerShutDown: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnDeviceStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, ldeviceid: i32, bpoweredoff: super::super::Foundation::VARIANT_BOOL, bsending: super::super::Foundation::VARIANT_BOOL, breceiving: super::super::Foundation::VARIANT_BOOL, bringing: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnDeviceStatusChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnGeneralServerConfigChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnGeneralServerConfigChanged: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
pub struct IStiDevice(::windows_core::IUnknown);
impl IStiDevice {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0, P1>(&self, hinst: P0, pwszdevicename: P1, dwversion: u32, dwmode: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), hinst.into_param().abi(), pwszdevicename.into_param().abi(), dwversion, dwmode).ok()
    }
    pub unsafe fn GetCapabilities(&self, pdevcaps: *mut STI_DEV_CAPS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), pdevcaps).ok()
    }
    pub unsafe fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), pdevstatus).ok()
    }
    pub unsafe fn DeviceReset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceReset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Diagnostic)(::windows_core::Interface::as_raw(self), pbuffer).ok()
    }
    pub unsafe fn Escape(&self, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Escape)(::windows_core::Interface::as_raw(self), escapefunction, lpindata, cbindatasize, poutdata, dwoutdatasize, pdwactualdata).ok()
    }
    pub unsafe fn GetLastError(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLastError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LockDevice(&self, dwtimeout: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockDevice)(::windows_core::Interface::as_raw(self), dwtimeout).ok()
    }
    pub unsafe fn UnLockDevice(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnLockDevice)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: ::core::option::Option<*const super::super::System::IO::OVERLAPPED>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawReadData)(::windows_core::Interface::as_raw(self), lpbuffer, lpdwnumberofbytes, ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteData(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: ::core::option::Option<*const super::super::System::IO::OVERLAPPED>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawWriteData)(::windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: ::core::option::Option<*const super::super::System::IO::OVERLAPPED>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawReadCommand)(::windows_core::Interface::as_raw(self), lpbuffer, lpdwnumberofbytes, ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: ::core::option::Option<*const super::super::System::IO::OVERLAPPED>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawWriteCommand)(::windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subscribe(&self, lpsubsribe: *mut STISUBSCRIBE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Subscribe)(::windows_core::Interface::as_raw(self), lpsubsribe).ok()
    }
    pub unsafe fn GetLastNotificationData(&self, lpnotify: *mut STINOTIFY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLastNotificationData)(::windows_core::Interface::as_raw(self), lpnotify).ok()
    }
    pub unsafe fn UnSubscribe(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnSubscribe)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetLastErrorInfo(&self, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLastErrorInfo)(::windows_core::Interface::as_raw(self), plasterrorinfo).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IStiDevice, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IStiDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStiDevice {}
impl ::core::fmt::Debug for IStiDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStiDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IStiDevice {
    type Vtable = IStiDevice_Vtbl;
}
impl ::core::clone::Clone for IStiDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStiDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cfa5a80_2dc8_11d0_90ea_00aa0060f86c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiDevice_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: ::windows_core::PCWSTR, dwversion: u32, dwmode: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_DEV_CAPS) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows_core::HRESULT,
    pub DeviceReset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows_core::HRESULT,
    pub LockDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows_core::HRESULT,
    pub UnLockDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadCommand: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteCommand: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Subscribe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpsubsribe: *mut STISUBSCRIBE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subscribe: usize,
    pub GetLastNotificationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows_core::HRESULT,
    pub UnSubscribe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLastErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
pub struct IStiDeviceControl(::windows_core::IUnknown);
impl IStiDeviceControl {
    pub unsafe fn Initialize<P0>(&self, dwdevicetype: u32, dwmode: u32, pwszportname: P0, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), dwdevicetype, dwmode, pwszportname.into_param().abi(), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawReadData)(::windows_core::Interface::as_raw(self), lpbuffer, lpdwnumberofbytes, lpoverlapped).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteData(&self, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawWriteData)(::windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, lpoverlapped).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawReadCommand)(::windows_core::Interface::as_raw(self), lpbuffer, lpdwnumberofbytes, lpoverlapped).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawWriteCommand)(::windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, lpoverlapped).ok()
    }
    pub unsafe fn RawDeviceControl(&self, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawDeviceControl)(::windows_core::Interface::as_raw(self), escapefunction, lpindata, cbindatasize, poutdata, dwoutdatasize, pdwactualdata).ok()
    }
    pub unsafe fn GetLastError(&self, lpdwlasterror: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLastError)(::windows_core::Interface::as_raw(self), lpdwlasterror).ok()
    }
    pub unsafe fn GetMyDevicePortName(&self, lpszdevicepath: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMyDevicePortName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpszdevicepath.as_ptr()), lpszdevicepath.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMyDeviceHandle(&self, lph: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMyDeviceHandle)(::windows_core::Interface::as_raw(self), lph).ok()
    }
    pub unsafe fn GetMyDeviceOpenMode(&self, pdwopenmode: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMyDeviceOpenMode)(::windows_core::Interface::as_raw(self), pdwopenmode).ok()
    }
    pub unsafe fn WriteToErrorLog<P0>(&self, dwmessagetype: u32, pszmessage: P0, dwerrorcode: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WriteToErrorLog)(::windows_core::Interface::as_raw(self), dwmessagetype, pszmessage.into_param().abi(), dwerrorcode).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IStiDeviceControl, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IStiDeviceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStiDeviceControl {}
impl ::core::fmt::Debug for IStiDeviceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStiDeviceControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IStiDeviceControl {
    type Vtable = IStiDeviceControl_Vtbl;
}
impl ::core::clone::Clone for IStiDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStiDeviceControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x128a9860_52dc_11d0_9edf_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiDeviceControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdevicetype: u32, dwmode: u32, pwszportname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadCommand: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteCommand: usize,
    pub RawDeviceControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdwlasterror: *mut u32) -> ::windows_core::HRESULT,
    pub GetMyDevicePortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszdevicepath: ::windows_core::PWSTR, cwdevicepathsize: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMyDeviceHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lph: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMyDeviceHandle: usize,
    pub GetMyDeviceOpenMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwopenmode: *mut u32) -> ::windows_core::HRESULT,
    pub WriteToErrorLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: ::windows_core::PCWSTR, dwerrorcode: u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
pub struct IStiUSD(::windows_core::IUnknown);
impl IStiUSD {
    #[doc = "*Required features: `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<P0, P1>(&self, pheldcb: P0, dwstiversion: u32, hparameterskey: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IStiDeviceControl>,
        P1: ::windows_core::IntoParam<super::super::System::Registry::HKEY>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pheldcb.into_param().abi(), dwstiversion, hparameterskey.into_param().abi()).ok()
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows_core::Result<STI_USD_CAPS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), pdevstatus).ok()
    }
    pub unsafe fn DeviceReset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceReset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Diagnostic)(::windows_core::Interface::as_raw(self), pbuffer).ok()
    }
    pub unsafe fn Escape(&self, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Escape)(::windows_core::Interface::as_raw(self), escapefunction, lpindata, cbindatasize, poutdata, cboutdatasize, pdwactualdata).ok()
    }
    pub unsafe fn GetLastError(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLastError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LockDevice(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockDevice)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UnLockDevice(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnLockDevice)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: ::core::option::Option<*const super::super::System::IO::OVERLAPPED>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawReadData)(::windows_core::Interface::as_raw(self), lpbuffer, lpdwnumberofbytes, ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteData(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: ::core::option::Option<*const super::super::System::IO::OVERLAPPED>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawWriteData)(::windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: ::core::option::Option<*const super::super::System::IO::OVERLAPPED>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawReadCommand)(::windows_core::Interface::as_raw(self), lpbuffer, lpdwnumberofbytes, ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: ::core::option::Option<*const super::super::System::IO::OVERLAPPED>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RawWriteCommand)(::windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotificationHandle<P0>(&self, hevent: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).SetNotificationHandle)(::windows_core::Interface::as_raw(self), hevent.into_param().abi()).ok()
    }
    pub unsafe fn GetNotificationData(&self, lpnotify: *mut STINOTIFY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNotificationData)(::windows_core::Interface::as_raw(self), lpnotify).ok()
    }
    pub unsafe fn GetLastErrorInfo(&self, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLastErrorInfo)(::windows_core::Interface::as_raw(self), plasterrorinfo).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IStiUSD, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IStiUSD {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStiUSD {}
impl ::core::fmt::Debug for IStiUSD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStiUSD").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IStiUSD {
    type Vtable = IStiUSD_Vtbl;
}
impl ::core::clone::Clone for IStiUSD {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStiUSD {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c9bb460_51ac_11d0_90ea_00aa0060f86c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiUSD_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheldcb: *mut ::core::ffi::c_void, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_USD_CAPS) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows_core::HRESULT,
    pub DeviceReset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows_core::HRESULT,
    pub LockDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnLockDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadCommand: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteCommand: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotificationHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotificationHandle: usize,
    pub GetNotificationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows_core::HRESULT,
    pub GetLastErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
pub struct IStillImageW(::windows_core::IUnknown);
impl IStillImageW {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, hinst: P0, dwversion: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), hinst.into_param().abi(), dwversion).ok()
    }
    pub unsafe fn GetDeviceList(&self, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceList)(::windows_core::Interface::as_raw(self), dwtype, dwflags, pdwitemsreturned, ppbuffer).ok()
    }
    pub unsafe fn GetDeviceInfo<P0>(&self, pwszdevicename: P0, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetDeviceInfo)(::windows_core::Interface::as_raw(self), pwszdevicename.into_param().abi(), ppbuffer).ok()
    }
    pub unsafe fn CreateDevice<P0, P1>(&self, pwszdevicename: P0, dwmode: u32, pdevice: *mut ::core::option::Option<IStiDevice>, punkouter: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateDevice)(::windows_core::Interface::as_raw(self), pwszdevicename.into_param().abi(), dwmode, ::core::mem::transmute(pdevice), punkouter.into_param().abi()).ok()
    }
    pub unsafe fn GetDeviceValue<P0, P1>(&self, pwszdevicename: P0, pvaluename: P1, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetDeviceValue)(::windows_core::Interface::as_raw(self), pwszdevicename.into_param().abi(), pvaluename.into_param().abi(), ptype, pdata, cbdata).ok()
    }
    pub unsafe fn SetDeviceValue<P0, P1>(&self, pwszdevicename: P0, pvaluename: P1, r#type: u32, pdata: &[u8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDeviceValue)(::windows_core::Interface::as_raw(self), pwszdevicename.into_param().abi(), pvaluename.into_param().abi(), r#type, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _).ok()
    }
    pub unsafe fn GetSTILaunchInformation(&self, pwszdevicename: &mut [u16; 128], pdweventcode: ::core::option::Option<*mut u32>, pwszeventname: &mut [u16; 128]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSTILaunchInformation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszdevicename.as_ptr()), ::core::mem::transmute(pdweventcode.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pwszeventname.as_ptr())).ok()
    }
    pub unsafe fn RegisterLaunchApplication<P0, P1>(&self, pwszappname: P0, pwszcommandline: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterLaunchApplication)(::windows_core::Interface::as_raw(self), pwszappname.into_param().abi(), pwszcommandline.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterLaunchApplication<P0>(&self, pwszappname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).UnregisterLaunchApplication)(::windows_core::Interface::as_raw(self), pwszappname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableHwNotifications<P0, P1>(&self, pwszdevicename: P0, bnewstate: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).EnableHwNotifications)(::windows_core::Interface::as_raw(self), pwszdevicename.into_param().abi(), bnewstate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwNotificationState<P0>(&self, pwszdevicename: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetHwNotificationState)(::windows_core::Interface::as_raw(self), pwszdevicename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RefreshDeviceBus<P0>(&self, pwszdevicename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RefreshDeviceBus)(::windows_core::Interface::as_raw(self), pwszdevicename.into_param().abi()).ok()
    }
    pub unsafe fn LaunchApplicationForDevice<P0, P1>(&self, pwszdevicename: P0, pwszappname: P1, pstinotify: *const STINOTIFY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).LaunchApplicationForDevice)(::windows_core::Interface::as_raw(self), pwszdevicename.into_param().abi(), pwszappname.into_param().abi(), pstinotify).ok()
    }
    pub unsafe fn SetupDeviceParameters(&self, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetupDeviceParameters)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn WriteToErrorLog<P0>(&self, dwmessagetype: u32, pszmessage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WriteToErrorLog)(::windows_core::Interface::as_raw(self), dwmessagetype, pszmessage.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IStillImageW, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IStillImageW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStillImageW {}
impl ::core::fmt::Debug for IStillImageW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStillImageW").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IStillImageW {
    type Vtable = IStillImageW_Vtbl;
}
impl ::core::clone::Clone for IStillImageW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStillImageW {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x641bd880_2dc8_11d0_90ea_00aa0060f86c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStillImageW_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetDeviceList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, dwmode: u32, pdevice: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows_core::HRESULT,
    pub SetDeviceValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows_core::HRESULT,
    pub GetSTILaunchInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PWSTR, pdweventcode: *mut u32, pwszeventname: ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub RegisterLaunchApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszappname: ::windows_core::PCWSTR, pwszcommandline: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub UnregisterLaunchApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszappname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableHwNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, bnewstate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableHwNotifications: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHwNotificationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, pbcurrentstate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHwNotificationState: usize,
    pub RefreshDeviceBus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub LaunchApplicationForDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, pwszappname: ::windows_core::PCWSTR, pstinotify: *const STINOTIFY) -> ::windows_core::HRESULT,
    pub SetupDeviceParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows_core::HRESULT,
    pub WriteToErrorLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_DEVICE_ID: ::windows_core::PCWSTR = ::windows_core::w!("FAXSRV_DeviceID");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_FSP_GUID: ::windows_core::PCWSTR = ::windows_core::w!("FAXSRV_FSPGuid");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_ROUTEEXT_NAME: ::windows_core::PCWSTR = ::windows_core::w!("FAXSRV_RoutingExtName");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_ROUTING_METHOD_GUID: ::windows_core::PCWSTR = ::windows_core::w!("FAXSRV_RoutingMethodGuid");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_SERVER_NAME: ::windows_core::PCWSTR = ::windows_core::w!("FAXSRV_ServerName");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CLSID_Sti: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb323f8e0_2e68_11d0_90ea_00aa0060f86c);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WIA_DeviceType: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WIA_USDClassId: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DEV_ID_SRC_FAX: FAX_ENUM_DEVICE_ID_SOURCE = FAX_ENUM_DEVICE_ID_SOURCE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DEV_ID_SRC_TAPI: FAX_ENUM_DEVICE_ID_SOURCE = FAX_ENUM_DEVICE_ID_SOURCE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DRT_EMAIL: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DRT_INBOX: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DRT_NONE: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXDEVRECEIVE_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXDEVREPORTSTATUS_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_INBOUND: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_INIT: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_OUTBOUND: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_UNKNOWN: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_MAX: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_MED: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_MIN: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_NONE: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXSRV_DEVICE_NODETYPE_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3115a19a_6251_46ac_9425_14782858b8c9);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXSRV_DEVICE_PROVIDER_NODETYPE_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd38e2ac_b926_4161_8640_0f6956ee2ba3);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXSRV_ROUTING_METHOD_NODETYPE_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x220d2cb0_85a9_4a43_b6e8_9d66b44f1af5);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_CONFIG_QUERY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_CONFIG_SET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_BAD_GROUP_CONFIGURATION: i32 = 7003i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_DEVICE_NUM_LIMIT_EXCEEDED: i32 = 7010i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_DIRECTORY_IN_USE: i32 = 7007i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_END: i32 = 7013i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_FILE_ACCESS_DENIED: i32 = 7008i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_GROUP_IN_USE: i32 = 7004i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_GROUP_NOT_FOUND: i32 = 7002i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_MESSAGE_NOT_FOUND: i32 = 7009i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_NOT_NTFS: i32 = 7006i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_NOT_SUPPORTED_ON_THIS_SKU: i32 = 7011i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_RECIPIENTS_LIMIT: i32 = 7013i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_RULE_NOT_FOUND: i32 = 7005i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_SRV_OUTOFMEMORY: i32 = 7001i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_START: i32 = 7001i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_VERSION_MISMATCH: i32 = 7012i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_BAD_GROUP_CONFIGURATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214501i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_DEVICE_NUM_LIMIT_EXCEEDED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214494i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_DIRECTORY_IN_USE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214497i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_FILE_ACCESS_DENIED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214496i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_GROUP_IN_USE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214500i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_GROUP_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214502i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_MESSAGE_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214495i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_NOT_NTFS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214498i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_NOT_SUPPORTED_ON_THIS_SKU: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214493i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_RECIPIENTS_LIMIT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214491i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_RULE_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214499i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_SRV_OUTOFMEMORY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214503i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_VERSION_MISMATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147214492i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_JOB_MANAGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_JOB_QUERY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_JOB_SUBMIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_PORT_QUERY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_PORT_SET: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_ABORTING: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_ANSWERED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_BAD_ADDRESS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_BUSY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_CALL_BLACKLISTED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_CALL_DELAYED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_COMPLETED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_DELETED: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_DIALING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_DISCONNECTED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_FATAL_ERROR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_FAXSVC_ENDED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_FAXSVC_STARTED: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_HANDLED: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_IDLE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_INITIALIZING: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_JOB_QUEUED: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_LINE_UNAVAILABLE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_MODEM_POWERED_OFF: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_MODEM_POWERED_ON: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NEVENTS: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NOT_FAX_CALL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NO_ANSWER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NO_DIAL_TONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_RECEIVING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_RINGING: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_ROUTING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_SENDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPF_RECEIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPF_SEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPF_VIRTUAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_ABORTING: u32 = 538968064u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_ANSWERED: u32 = 545259520u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_AVAILABLE: u32 = 537919488u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_BAD_ADDRESS: u32 = 536871168u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_BUSY: u32 = 536870976u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_CALL_BLACKLISTED: u32 = 536887296u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_CALL_DELAYED: u32 = 536879104u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_COMPLETED: u32 = 536870920u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_DIALING: u32 = 536870913u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_DISCONNECTED: u32 = 536871936u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_FATAL_ERROR: u32 = 536872960u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_HANDLED: u32 = 536870928u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_INITIALIZING: u32 = 536903680u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_NOT_FAX_CALL: u32 = 536875008u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_NO_ANSWER: u32 = 536871040u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_NO_DIAL_TONE: u32 = 536871424u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_OFFLINE: u32 = 536936448u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_RECEIVING: u32 = 536870916u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_RINGING: u32 = 537001984u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_ROUTING: u32 = 541065216u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_SENDING: u32 = 536870914u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_UNAVAILABLE: u32 = 536870944u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_ANSWERED: u32 = 545259520u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_BAD_ADDRESS: u32 = 536871168u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_BUSY: u32 = 536870976u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_CALL_BLACKLISTED: u32 = 536887296u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_CALL_DELAYED: u32 = 536879104u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_COMPLETED: u32 = 536870920u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_DIALING: u32 = 536870913u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_DISCONNECTED: u32 = 536871936u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_FATAL_ERROR: u32 = 536872960u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_HANDLED: u32 = 536870928u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_INITIALIZING: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_LINE_UNAVAILABLE: u32 = 536870944u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_NOT_FAX_CALL: u32 = 536875008u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_NO_ANSWER: u32 = 536871040u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_NO_DIAL_TONE: u32 = 536871424u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_RECEIVING: u32 = 536870916u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_TRANSMITTING: u32 = 536870914u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_USER_ABORT: u32 = 538968064u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxAccount: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7e0647f_4524_4464_a56d_b9fe666f715e);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxAccountFolders: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85398f49_c034_4a3f_821c_db7d685e8129);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxAccountIncomingArchive: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14b33db5_4c40_4ecf_9ef8_a360cbe809ed);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxAccountIncomingQueue: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9bcf6094_b4da_45f4_b8d6_ddeb2186652c);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxAccountOutgoingArchive: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x851e7af5_433a_4739_a2df_ad245c2cb98e);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxAccountOutgoingQueue: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfeeceefb_c149_48ba_bab8_b791e101f62f);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxAccountSet: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbc23c4b_79e0_4291_bc56_c12e253bbf3a);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxAccounts: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda1f94aa_ee2c_47c0_8f4f_2a217075b76e);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxActivity: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfef5d0e_e84d_462e_aabb_87d31eb04fef);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxActivityLogging: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0a0294e_3bbd_48b8_8f13_8c591a55bdbc);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxConfiguration: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5857326f_e7b3_41a7_9c19_a91b463e2d56);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxDevice: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59e3a5b2_d676_484b_a6de_720bfa89b5af);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxDeviceIds: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcdc539ea_7277_460e_8de0_48a0a5760d1f);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxDeviceProvider: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17cf1aa3_f5eb_484a_9c9a_4440a5baabfc);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxDeviceProviders: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb8fe768_875a_4f5f_82c5_03f23aac1bd7);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxDevices: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5589e28e_23cb_4919_8808_e6101846e80d);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxDocument: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f3f9f91_c838_415e_a4f3_3e828ca445e0);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxEventLogging: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6850930_a0f6_4a6f_95b7_db2ebf3d02e3);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxFolders: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc35211d7_5776_48cb_af44_c31be3b2cfe5);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxInboundRouting: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe80248ed_ad65_4218_8108_991924d4e7ed);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxInboundRoutingExtension: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d7dfb51_7207_4436_a0d9_24e32ee56988);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxInboundRoutingExtensions: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x189a48ed_623c_4c0d_80f2_d66c7b9efec2);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxInboundRoutingMethod: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b9fd75c_0194_4b72_9ce5_02a8205ac7d4);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxInboundRoutingMethods: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25fcb76a_b750_4b82_9266_fbbbae8922ba);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxIncomingArchive: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8426c56a_35a1_4c6f_af93_fc952422e2c2);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxIncomingJob: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc47311ec_ae32_41b8_ae4b_3eae0629d0c9);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxIncomingJobs: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1bb8a43_8866_4fb7_a15d_6266c875a5cc);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxIncomingMessage: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1932fcf7_9d43_4d5a_89ff_03861b321736);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxIncomingMessageIterator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6088e1d8_3fc8_45c2_87b1_909a29607ea9);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxIncomingQueue: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69131717_f3f1_40e3_809d_a6cbf7bd85e5);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxJobStatus: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bf222f4_be8d_442f_841d_6132742423bb);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxLoggingOptions: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bf9eea6_ece0_4785_a18b_de56e9eef96a);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxOutboundRouting: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc81b385e_b869_4afd_86c0_616498ed9be2);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxOutboundRoutingGroup: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0213f3e0_6791_4d77_a271_04d2357c50d6);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxOutboundRoutingGroups: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccbea1a5_e2b4_4b57_9421_b04b6289464b);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxOutboundRoutingRule: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6549eebf_08d1_475a_828b_3bf105952fa0);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxOutboundRoutingRules: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd385beca_e624_4473_bfaa_9f4000831f54);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxOutgoingArchive: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43c28403_e04f_474d_990c_b94669148f59);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxOutgoingJob: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71bb429c_0ef9_4915_bec5_a5d897a3e924);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxOutgoingJobs: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92bf2a6c_37be_43fa_a37d_cb0e5f753b35);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxOutgoingMessage: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91b4a378_4ad8_4aef_a4dc_97d96e939a3a);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxOutgoingMessageIterator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a3224d0_d30b_49de_9813_cb385790fbbb);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxOutgoingQueue: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7421169e_8c43_4b0d_bb16_645c8fa40357);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxReceiptOptions: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6982487b_227b_4c96_a61c_248348b05ab6);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxRecipient: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60bf3301_7df8_4bd8_9148_7b5801f9efdf);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxRecipients: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea9bdf53_10a9_4d4f_a067_63c8f84f01b0);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxSecurity: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10c4ddde_abf0_43df_964f_7f3ac21a4c7b);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxSecurity2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x735c1248_ec89_4c30_a127_656e92e3c4ea);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxSender: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x265d84d0_1850_4360_b7c8_758bbb5f0b96);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FaxServer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcda8acb0_8cf5_4f6c_9ba2_5931d40c8cae);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const GUID_DeviceArrivedLaunch: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x740d9ee6_70f1_11d1_ad10_00a02438ad48);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const GUID_STIUserDefined1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc00eb795_8c6e_11d2_977a_0000f87a926f);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const GUID_STIUserDefined2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc77ae9c5_8c6e_11d2_977a_0000f87a926f);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const GUID_STIUserDefined3: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc77ae9c6_8c6e_11d2_977a_0000f87a926f);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const GUID_ScanFaxImage: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc00eb793_8c6e_11d2_977a_0000f87a926f);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const GUID_ScanImage: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6c5a715_8c6e_11d2_977a_0000f87a926f);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const GUID_ScanPrintImage: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb441f425_8c6e_11d2_977a_0000f87a926f);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const IS_DIGITAL_CAMERA_STR: ::windows_core::PCWSTR = ::windows_core::w!("IsDigitalCamera");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const IS_DIGITAL_CAMERA_VAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_DELETE: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_PAUSE: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_RESUME: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_UNKNOWN: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JSA_DISCOUNT_PERIOD: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JSA_NOW: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JSA_SPECIFIC_TIME: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_DELETING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_FAILED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_INPROGRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_NOLINE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_PAUSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_PENDING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_RETRIES_EXCEEDED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_RETRYING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_FAIL_RECEIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_RECEIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_ROUTING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_SEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MAX_NOTIFICATION_DATA: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MS_FAXROUTE_EMAIL_GUID: ::windows_core::PCWSTR = ::windows_core::w!("{6bbf7bfe-9af2-11d0-abf7-00c04fd91a4e}");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MS_FAXROUTE_FOLDER_GUID: ::windows_core::PCWSTR = ::windows_core::w!("{92041a90-9af2-11d0-abf7-00c04fd91a4e}");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MS_FAXROUTE_PRINTING_GUID: ::windows_core::PCWSTR = ::windows_core::w!("{aec1b37c-9af2-11d0-abf7-00c04fd91a4e}");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const PORT_OPEN_MODIFY: FAX_ENUM_PORT_OPEN_TYPE = FAX_ENUM_PORT_OPEN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const PORT_OPEN_QUERY: FAX_ENUM_PORT_OPEN_TYPE = FAX_ENUM_PORT_OPEN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const QUERY_STATUS: FAXROUTE_ENABLE = FAXROUTE_ENABLE(-1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_BAUDRATE: ::windows_core::PCWSTR = ::windows_core::w!("BaudRate");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_BAUDRATE_A: ::windows_core::PCSTR = ::windows_core::s!("BaudRate");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DATA_W: ::windows_core::PCWSTR = ::windows_core::w!("DeviceData");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEVICESUBTYPE_W: ::windows_core::PCWSTR = ::windows_core::w!("DeviceSubType");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEVICETYPE_W: ::windows_core::PCWSTR = ::windows_core::w!("DeviceType");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEVICE_NAME_W: ::windows_core::PCWSTR = ::windows_core::w!("DriverDesc");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEV_NAME_W: ::windows_core::PCWSTR = ::windows_core::w!("DeviceName");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DRIVER_DESC_W: ::windows_core::PCWSTR = ::windows_core::w!("DriverDesc");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_FRIENDLY_NAME_W: ::windows_core::PCWSTR = ::windows_core::w!("FriendlyName");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_GENERIC_CAPS_W: ::windows_core::PCWSTR = ::windows_core::w!("Capabilities");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_GUID: ::windows_core::PCWSTR = ::windows_core::w!("GUID");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_GUID_W: ::windows_core::PCWSTR = ::windows_core::w!("GUID");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_HARDWARE: ::windows_core::PCWSTR = ::windows_core::w!("HardwareConfig");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_HARDWARE_W: ::windows_core::PCWSTR = ::windows_core::w!("HardwareConfig");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCHABLE: ::windows_core::PCWSTR = ::windows_core::w!("Launchable");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCHABLE_W: ::windows_core::PCWSTR = ::windows_core::w!("Launchable");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCH_APPS: ::windows_core::PCWSTR = ::windows_core::w!("LaunchApplications");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCH_APPS_W: ::windows_core::PCWSTR = ::windows_core::w!("LaunchApplications");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_SHUTDOWNDELAY: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownIfUnusedDelay");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_SHUTDOWNDELAY_W: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownIfUnusedDelay");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_TYPE_W: ::windows_core::PCWSTR = ::windows_core::w!("Type");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_VENDOR_NAME_W: ::windows_core::PCWSTR = ::windows_core::w!("Vendor");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const SEND_TO_FAX_RECIPIENT_ATTACHMENT: SendToMode = SendToMode(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STATUS_DISABLE: FAXROUTE_ENABLE = FAXROUTE_ENABLE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STATUS_ENABLE: FAXROUTE_ENABLE = FAXROUTE_ENABLE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIEDFL_ALLDEVICES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIEDFL_ATTACHEDONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_ALREADY_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147023649i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_BADDRIVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024777i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_BETA_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147023743i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_DEVICENOTREG: i32 = -2147221164i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_DEVICE_LOCKED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024863i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_DEVICE_NOTREADY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024875i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_GENERIC: i32 = -2147467259i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_HANDLEEXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024713i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_INVALID_DEVICE_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024773i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_INVALID_HW_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024883i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_INVALID_PARAM: i32 = -2147024809i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NEEDS_LOCK: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024738i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOEVENTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024637i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOINTERFACE: i32 = -2147467262i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOTINITIALIZED: i32 = -2147024891i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOT_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024875i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_OBJECTNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024894i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_OLD_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147023746i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_OUTOFMEMORY: i32 = -2147024882i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_READONLY: i32 = -2147024891i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_SHARING_VIOLATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024864i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_UNSUPPORTED: i32 = -2147467263i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ADD_DEVICE_BROADCAST_ACTION: ::windows_core::PCSTR = ::windows_core::s!("Arrival");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ADD_DEVICE_BROADCAST_STRING: ::windows_core::PCSTR = ::windows_core::s!("STI\\");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_CHANGENOEFFECT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_FOR_MONITOR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP: ::windows_core::PCWSTR = ::windows_core::w!("DefaultLaunchApp");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP_A: ::windows_core::PCSTR = ::windows_core::s!("DefaultLaunchApp");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS: ::windows_core::PCWSTR = ::windows_core::w!("DisableNotifications");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS_A: ::windows_core::PCSTR = ::windows_core::s!("DisableNotifications");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ICM_PROFILE: ::windows_core::PCWSTR = ::windows_core::w!("ICMProfile");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ICM_PROFILE_A: ::windows_core::PCSTR = ::windows_core::s!("ICMProfile");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ISIS_NAME: ::windows_core::PCWSTR = ::windows_core::w!("ISISDriverName");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ISIS_NAME_A: ::windows_core::PCSTR = ::windows_core::s!("ISISDriverName");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TIMEOUT: ::windows_core::PCWSTR = ::windows_core::w!("PollTimeout");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TIMEOUT_A: ::windows_core::PCSTR = ::windows_core::s!("PollTimeout");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TWAIN_NAME: ::windows_core::PCWSTR = ::windows_core::w!("TwainDS");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TWAIN_NAME_A: ::windows_core::PCSTR = ::windows_core::s!("TwainDS");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVSTATUS_EVENTS_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVSTATUS_ONLINE_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DIAGCODE_HWPRESENCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ERROR_NO_ERROR: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_EVENTHANDLING_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_EVENTHANDLING_PENDING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_EVENTHANDLING_POLLING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_AUTO_PORTSELECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_COMMON_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_GENERATE_ARRIVALEVENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_NOTIFICATIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_POLLING_NEEDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_SUBSET: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_WIA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_PARALLEL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_SCSI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_SERIAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_USB: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_MAX_INTERNAL_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_NOTCONNECTED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_OK: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_BUSY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_INITIALIZING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_IO_ACTIVE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_OFFLINE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_OPERATIONAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PAPER_JAM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PAPER_PROBLEM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PAUSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PENDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_POWER_SAVE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_TRANSFERRING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_USER_INTERVENTION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_WARMING_UP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_RAW_RESERVED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_REMOVE_DEVICE_BROADCAST_ACTION: ::windows_core::PCSTR = ::windows_core::s!("Removal");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_REMOVE_DEVICE_BROADCAST_STRING: ::windows_core::PCSTR = ::windows_core::s!("STI\\");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_SUBSCRIBE_FLAG_EVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_SUBSCRIBE_FLAG_WINDOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_TRACE_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_TRACE_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_TRACE_WARNING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_UNICODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_USD_GENCAP_NATIVE_PUSHSUPPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_FLAG_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_FLAG_UNICODE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_MIN_ALLOWED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_REAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const SUPPORTS_MSCPLUS_STR: ::windows_core::PCWSTR = ::windows_core::w!("SupportsMSCPlus");
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const SUPPORTS_MSCPLUS_VAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeDefault: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeDigitalCamera: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeScanner: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeStreamingVideo: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const WIA_INCOMPAT_XP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetFXSSVC_ENDED: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetIN_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetIN_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetNONE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetOUT_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetOUT_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(256i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(64i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_RECEIVE_FOLDER: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(512i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2QUERY_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(128i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2QUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(32i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2QUERY_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2SUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2SUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2SUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(64i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(256i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_JOBS: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(1024i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(128i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_JOBS: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(512i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farSUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farSUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farSUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fcptLOCAL: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fcptNONE: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fcptSERVER: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fdrmAUTO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fdrmMANUAL_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fdrmNO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsALL_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsALL_DEV_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsEMPTY: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsSOME_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesANSWERED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesBAD_ADDRESS: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(10i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesBUSY: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_ABORTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(19i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_BLACKLISTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(14i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_COMPLETED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(18i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_DELAYED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(13i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesDIALING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesDISCONNECTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesFATAL_ERROR: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(12i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesHANDLED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(17i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesINITIALIZING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesLINE_UNAVAILABLE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(7i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNONE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNOT_FAX_CALL: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(15i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNO_ANSWER: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(9i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNO_DIAL_TONE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(11i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesPARTIALLY_RECEIVED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesPROPRIETARY: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(16777216i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesRECEIVING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesTRANSMITTING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoDELETE: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoPAUSE: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoRECIPIENT_INFO: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoRESTART: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoRESUME: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoSENDER_INFO: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(64i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoVIEW: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsCANCELED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(512i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsCANCELING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(1024i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsCOMPLETED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(256i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsFAILED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsINPROGRESS: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsNOLINE: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsPAUSED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsPENDING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsRETRIES_EXCEEDED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(128i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsRETRYING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(64i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsROUTING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(2048i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjtRECEIVE: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjtROUTING: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjtSEND: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllMAX: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllMED: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllMIN: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllNONE: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsBAD_GUID: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsBAD_VERSION: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsCANT_INIT: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsCANT_LINK: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsCANT_LOAD: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsSERVER_ERROR: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsSUCCESS: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fptHIGH: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fptLOW: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fptNORMAL: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frrcANY_CODE: FAX_ROUTING_RULE_CODE_ENUM = FAX_ROUTING_RULE_CODE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsALL_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsBAD_DEVICE: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsEMPTY_GROUP: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsSOME_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsVALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frtMAIL: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frtMSGBOX: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frtNONE: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_0: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_1: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(65536i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_2: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(131072i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_3: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(196608i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsatANONYMOUS: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsatBASIC: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsatNTLM: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetACTIVITY: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetCONFIG: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetDEVICE_STATUS: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(256i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetFXSSVC_ENDED: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(128i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetINCOMING_CALL: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(512i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetIN_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetIN_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetNONE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetOUT_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(64i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetOUT_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetQUEUE_STATE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fstDISCOUNT_PERIOD: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fstNOW: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fstSPECIFIC_TIME: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const lDEFAULT_PREFETCH_SIZE: i32 = 100i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const prv_DEFAULT_PREFETCH_SIZE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const wcharREASSIGN_RECIPIENTS_DELIMITER: u16 = 59u16;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAXROUTE_ENABLE(pub i32);
impl ::core::marker::Copy for FAXROUTE_ENABLE {}
impl ::core::clone::Clone for FAXROUTE_ENABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAXROUTE_ENABLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAXROUTE_ENABLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAXROUTE_ENABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAXROUTE_ENABLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_ACCESS_RIGHTS_ENUM(pub i32);
impl ::core::marker::Copy for FAX_ACCESS_RIGHTS_ENUM {}
impl ::core::clone::Clone for FAX_ACCESS_RIGHTS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ACCESS_RIGHTS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_ACCESS_RIGHTS_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_ACCESS_RIGHTS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ACCESS_RIGHTS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_ACCESS_RIGHTS_ENUM_2(pub i32);
impl ::core::marker::Copy for FAX_ACCESS_RIGHTS_ENUM_2 {}
impl ::core::clone::Clone for FAX_ACCESS_RIGHTS_ENUM_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ACCESS_RIGHTS_ENUM_2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_ACCESS_RIGHTS_ENUM_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_ACCESS_RIGHTS_ENUM_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ACCESS_RIGHTS_ENUM_2").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_ACCOUNT_EVENTS_TYPE_ENUM(pub i32);
impl ::core::marker::Copy for FAX_ACCOUNT_EVENTS_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_ACCOUNT_EVENTS_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ACCOUNT_EVENTS_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_ACCOUNT_EVENTS_TYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_ACCOUNT_EVENTS_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ACCOUNT_EVENTS_TYPE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_COVERPAGE_TYPE_ENUM(pub i32);
impl ::core::marker::Copy for FAX_COVERPAGE_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_COVERPAGE_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_COVERPAGE_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_COVERPAGE_TYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_COVERPAGE_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_COVERPAGE_TYPE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_DEVICE_RECEIVE_MODE_ENUM(pub i32);
impl ::core::marker::Copy for FAX_DEVICE_RECEIVE_MODE_ENUM {}
impl ::core::clone::Clone for FAX_DEVICE_RECEIVE_MODE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_DEVICE_RECEIVE_MODE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_DEVICE_RECEIVE_MODE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_DEVICE_RECEIVE_MODE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_DEVICE_RECEIVE_MODE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_ENUM_DELIVERY_REPORT_TYPES(pub i32);
impl ::core::marker::Copy for FAX_ENUM_DELIVERY_REPORT_TYPES {}
impl ::core::clone::Clone for FAX_ENUM_DELIVERY_REPORT_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_DELIVERY_REPORT_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_ENUM_DELIVERY_REPORT_TYPES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_ENUM_DELIVERY_REPORT_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_DELIVERY_REPORT_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_ENUM_DEVICE_ID_SOURCE(pub i32);
impl ::core::marker::Copy for FAX_ENUM_DEVICE_ID_SOURCE {}
impl ::core::clone::Clone for FAX_ENUM_DEVICE_ID_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_DEVICE_ID_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_ENUM_DEVICE_ID_SOURCE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_ENUM_DEVICE_ID_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_DEVICE_ID_SOURCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_ENUM_JOB_COMMANDS(pub i32);
impl ::core::marker::Copy for FAX_ENUM_JOB_COMMANDS {}
impl ::core::clone::Clone for FAX_ENUM_JOB_COMMANDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_JOB_COMMANDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_ENUM_JOB_COMMANDS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_ENUM_JOB_COMMANDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_JOB_COMMANDS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_ENUM_JOB_SEND_ATTRIBUTES(pub i32);
impl ::core::marker::Copy for FAX_ENUM_JOB_SEND_ATTRIBUTES {}
impl ::core::clone::Clone for FAX_ENUM_JOB_SEND_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_JOB_SEND_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_ENUM_JOB_SEND_ATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_ENUM_JOB_SEND_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_JOB_SEND_ATTRIBUTES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_ENUM_LOG_CATEGORIES(pub i32);
impl ::core::marker::Copy for FAX_ENUM_LOG_CATEGORIES {}
impl ::core::clone::Clone for FAX_ENUM_LOG_CATEGORIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_LOG_CATEGORIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_ENUM_LOG_CATEGORIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_ENUM_LOG_CATEGORIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_LOG_CATEGORIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_ENUM_LOG_LEVELS(pub i32);
impl ::core::marker::Copy for FAX_ENUM_LOG_LEVELS {}
impl ::core::clone::Clone for FAX_ENUM_LOG_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_LOG_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_ENUM_LOG_LEVELS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_ENUM_LOG_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_LOG_LEVELS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_ENUM_PORT_OPEN_TYPE(pub i32);
impl ::core::marker::Copy for FAX_ENUM_PORT_OPEN_TYPE {}
impl ::core::clone::Clone for FAX_ENUM_PORT_OPEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_PORT_OPEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_ENUM_PORT_OPEN_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_ENUM_PORT_OPEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_PORT_OPEN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_GROUP_STATUS_ENUM(pub i32);
impl ::core::marker::Copy for FAX_GROUP_STATUS_ENUM {}
impl ::core::clone::Clone for FAX_GROUP_STATUS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_GROUP_STATUS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_GROUP_STATUS_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_GROUP_STATUS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_GROUP_STATUS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_JOB_EXTENDED_STATUS_ENUM(pub i32);
impl ::core::marker::Copy for FAX_JOB_EXTENDED_STATUS_ENUM {}
impl ::core::clone::Clone for FAX_JOB_EXTENDED_STATUS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_JOB_EXTENDED_STATUS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_JOB_EXTENDED_STATUS_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_JOB_EXTENDED_STATUS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_JOB_EXTENDED_STATUS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_JOB_OPERATIONS_ENUM(pub i32);
impl ::core::marker::Copy for FAX_JOB_OPERATIONS_ENUM {}
impl ::core::clone::Clone for FAX_JOB_OPERATIONS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_JOB_OPERATIONS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_JOB_OPERATIONS_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_JOB_OPERATIONS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_JOB_OPERATIONS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_JOB_STATUS_ENUM(pub i32);
impl ::core::marker::Copy for FAX_JOB_STATUS_ENUM {}
impl ::core::clone::Clone for FAX_JOB_STATUS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_JOB_STATUS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_JOB_STATUS_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_JOB_STATUS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_JOB_STATUS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_JOB_TYPE_ENUM(pub i32);
impl ::core::marker::Copy for FAX_JOB_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_JOB_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_JOB_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_JOB_TYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_JOB_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_JOB_TYPE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_LOG_LEVEL_ENUM(pub i32);
impl ::core::marker::Copy for FAX_LOG_LEVEL_ENUM {}
impl ::core::clone::Clone for FAX_LOG_LEVEL_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_LOG_LEVEL_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_LOG_LEVEL_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_LOG_LEVEL_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_LOG_LEVEL_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_PRIORITY_TYPE_ENUM(pub i32);
impl ::core::marker::Copy for FAX_PRIORITY_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_PRIORITY_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_PRIORITY_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_PRIORITY_TYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_PRIORITY_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_PRIORITY_TYPE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_PROVIDER_STATUS_ENUM(pub i32);
impl ::core::marker::Copy for FAX_PROVIDER_STATUS_ENUM {}
impl ::core::clone::Clone for FAX_PROVIDER_STATUS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_PROVIDER_STATUS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_PROVIDER_STATUS_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_PROVIDER_STATUS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_PROVIDER_STATUS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_RECEIPT_TYPE_ENUM(pub i32);
impl ::core::marker::Copy for FAX_RECEIPT_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_RECEIPT_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_RECEIPT_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_RECEIPT_TYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_RECEIPT_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_RECEIPT_TYPE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_ROUTING_RULE_CODE_ENUM(pub i32);
impl ::core::marker::Copy for FAX_ROUTING_RULE_CODE_ENUM {}
impl ::core::clone::Clone for FAX_ROUTING_RULE_CODE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ROUTING_RULE_CODE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_ROUTING_RULE_CODE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_ROUTING_RULE_CODE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ROUTING_RULE_CODE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_RULE_STATUS_ENUM(pub i32);
impl ::core::marker::Copy for FAX_RULE_STATUS_ENUM {}
impl ::core::clone::Clone for FAX_RULE_STATUS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_RULE_STATUS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_RULE_STATUS_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_RULE_STATUS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_RULE_STATUS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_SCHEDULE_TYPE_ENUM(pub i32);
impl ::core::marker::Copy for FAX_SCHEDULE_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_SCHEDULE_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_SCHEDULE_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_SCHEDULE_TYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_SCHEDULE_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_SCHEDULE_TYPE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_SERVER_APIVERSION_ENUM(pub i32);
impl ::core::marker::Copy for FAX_SERVER_APIVERSION_ENUM {}
impl ::core::clone::Clone for FAX_SERVER_APIVERSION_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_SERVER_APIVERSION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_SERVER_APIVERSION_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_SERVER_APIVERSION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_SERVER_APIVERSION_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_SERVER_EVENTS_TYPE_ENUM(pub i32);
impl ::core::marker::Copy for FAX_SERVER_EVENTS_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_SERVER_EVENTS_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_SERVER_EVENTS_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_SERVER_EVENTS_TYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_SERVER_EVENTS_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_SERVER_EVENTS_TYPE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAX_SMTP_AUTHENTICATION_TYPE_ENUM(pub i32);
impl ::core::marker::Copy for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_SMTP_AUTHENTICATION_TYPE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STI_DEVICE_MJ_TYPE(pub i32);
impl ::core::marker::Copy for STI_DEVICE_MJ_TYPE {}
impl ::core::clone::Clone for STI_DEVICE_MJ_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STI_DEVICE_MJ_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for STI_DEVICE_MJ_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for STI_DEVICE_MJ_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STI_DEVICE_MJ_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SendToMode(pub i32);
impl ::core::marker::Copy for SendToMode {}
impl ::core::clone::Clone for SendToMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SendToMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SendToMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SendToMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SendToMode").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_CONFIGURATIONA {
    pub SizeOfStruct: u32,
    pub Retries: u32,
    pub RetryDelay: u32,
    pub DirtyDays: u32,
    pub Branding: super::super::Foundation::BOOL,
    pub UseDeviceTsid: super::super::Foundation::BOOL,
    pub ServerCp: super::super::Foundation::BOOL,
    pub PauseServerQueue: super::super::Foundation::BOOL,
    pub StartCheapTime: FAX_TIME,
    pub StopCheapTime: FAX_TIME,
    pub ArchiveOutgoingFaxes: super::super::Foundation::BOOL,
    pub ArchiveDirectory: ::windows_core::PCSTR,
    pub Reserved: ::windows_core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_CONFIGURATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_CONFIGURATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_CONFIGURATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_CONFIGURATIONA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("Retries", &self.Retries)
            .field("RetryDelay", &self.RetryDelay)
            .field("DirtyDays", &self.DirtyDays)
            .field("Branding", &self.Branding)
            .field("UseDeviceTsid", &self.UseDeviceTsid)
            .field("ServerCp", &self.ServerCp)
            .field("PauseServerQueue", &self.PauseServerQueue)
            .field("StartCheapTime", &self.StartCheapTime)
            .field("StopCheapTime", &self.StopCheapTime)
            .field("ArchiveOutgoingFaxes", &self.ArchiveOutgoingFaxes)
            .field("ArchiveDirectory", &self.ArchiveDirectory)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_CONFIGURATIONA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_CONFIGURATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Retries == other.Retries && self.RetryDelay == other.RetryDelay && self.DirtyDays == other.DirtyDays && self.Branding == other.Branding && self.UseDeviceTsid == other.UseDeviceTsid && self.ServerCp == other.ServerCp && self.PauseServerQueue == other.PauseServerQueue && self.StartCheapTime == other.StartCheapTime && self.StopCheapTime == other.StopCheapTime && self.ArchiveOutgoingFaxes == other.ArchiveOutgoingFaxes && self.ArchiveDirectory == other.ArchiveDirectory && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_CONFIGURATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_CONFIGURATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_CONFIGURATIONW {
    pub SizeOfStruct: u32,
    pub Retries: u32,
    pub RetryDelay: u32,
    pub DirtyDays: u32,
    pub Branding: super::super::Foundation::BOOL,
    pub UseDeviceTsid: super::super::Foundation::BOOL,
    pub ServerCp: super::super::Foundation::BOOL,
    pub PauseServerQueue: super::super::Foundation::BOOL,
    pub StartCheapTime: FAX_TIME,
    pub StopCheapTime: FAX_TIME,
    pub ArchiveOutgoingFaxes: super::super::Foundation::BOOL,
    pub ArchiveDirectory: ::windows_core::PCWSTR,
    pub Reserved: ::windows_core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_CONFIGURATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_CONFIGURATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_CONFIGURATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_CONFIGURATIONW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("Retries", &self.Retries)
            .field("RetryDelay", &self.RetryDelay)
            .field("DirtyDays", &self.DirtyDays)
            .field("Branding", &self.Branding)
            .field("UseDeviceTsid", &self.UseDeviceTsid)
            .field("ServerCp", &self.ServerCp)
            .field("PauseServerQueue", &self.PauseServerQueue)
            .field("StartCheapTime", &self.StartCheapTime)
            .field("StopCheapTime", &self.StopCheapTime)
            .field("ArchiveOutgoingFaxes", &self.ArchiveOutgoingFaxes)
            .field("ArchiveDirectory", &self.ArchiveDirectory)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_CONFIGURATIONW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_CONFIGURATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Retries == other.Retries && self.RetryDelay == other.RetryDelay && self.DirtyDays == other.DirtyDays && self.Branding == other.Branding && self.UseDeviceTsid == other.UseDeviceTsid && self.ServerCp == other.ServerCp && self.PauseServerQueue == other.PauseServerQueue && self.StartCheapTime == other.StartCheapTime && self.StopCheapTime == other.StopCheapTime && self.ArchiveOutgoingFaxes == other.ArchiveOutgoingFaxes && self.ArchiveDirectory == other.ArchiveDirectory && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_CONFIGURATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_CONFIGURATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct FAX_CONTEXT_INFOA {
    pub SizeOfStruct: u32,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub ServerName: [u8; 16],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for FAX_CONTEXT_INFOA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for FAX_CONTEXT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for FAX_CONTEXT_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_CONTEXT_INFOA").field("SizeOfStruct", &self.SizeOfStruct).field("hDC", &self.hDC).field("ServerName", &self.ServerName).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::TypeKind for FAX_CONTEXT_INFOA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for FAX_CONTEXT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.hDC == other.hDC && self.ServerName == other.ServerName
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for FAX_CONTEXT_INFOA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for FAX_CONTEXT_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct FAX_CONTEXT_INFOW {
    pub SizeOfStruct: u32,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub ServerName: [u16; 16],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for FAX_CONTEXT_INFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for FAX_CONTEXT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for FAX_CONTEXT_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_CONTEXT_INFOW").field("SizeOfStruct", &self.SizeOfStruct).field("hDC", &self.hDC).field("ServerName", &self.ServerName).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::TypeKind for FAX_CONTEXT_INFOW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for FAX_CONTEXT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.hDC == other.hDC && self.ServerName == other.ServerName
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for FAX_CONTEXT_INFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for FAX_CONTEXT_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_COVERPAGE_INFOA {
    pub SizeOfStruct: u32,
    pub CoverPageName: ::windows_core::PCSTR,
    pub UseServerCoverPage: super::super::Foundation::BOOL,
    pub RecName: ::windows_core::PCSTR,
    pub RecFaxNumber: ::windows_core::PCSTR,
    pub RecCompany: ::windows_core::PCSTR,
    pub RecStreetAddress: ::windows_core::PCSTR,
    pub RecCity: ::windows_core::PCSTR,
    pub RecState: ::windows_core::PCSTR,
    pub RecZip: ::windows_core::PCSTR,
    pub RecCountry: ::windows_core::PCSTR,
    pub RecTitle: ::windows_core::PCSTR,
    pub RecDepartment: ::windows_core::PCSTR,
    pub RecOfficeLocation: ::windows_core::PCSTR,
    pub RecHomePhone: ::windows_core::PCSTR,
    pub RecOfficePhone: ::windows_core::PCSTR,
    pub SdrName: ::windows_core::PCSTR,
    pub SdrFaxNumber: ::windows_core::PCSTR,
    pub SdrCompany: ::windows_core::PCSTR,
    pub SdrAddress: ::windows_core::PCSTR,
    pub SdrTitle: ::windows_core::PCSTR,
    pub SdrDepartment: ::windows_core::PCSTR,
    pub SdrOfficeLocation: ::windows_core::PCSTR,
    pub SdrHomePhone: ::windows_core::PCSTR,
    pub SdrOfficePhone: ::windows_core::PCSTR,
    pub Note: ::windows_core::PCSTR,
    pub Subject: ::windows_core::PCSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_COVERPAGE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_COVERPAGE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_COVERPAGE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_COVERPAGE_INFOA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("CoverPageName", &self.CoverPageName)
            .field("UseServerCoverPage", &self.UseServerCoverPage)
            .field("RecName", &self.RecName)
            .field("RecFaxNumber", &self.RecFaxNumber)
            .field("RecCompany", &self.RecCompany)
            .field("RecStreetAddress", &self.RecStreetAddress)
            .field("RecCity", &self.RecCity)
            .field("RecState", &self.RecState)
            .field("RecZip", &self.RecZip)
            .field("RecCountry", &self.RecCountry)
            .field("RecTitle", &self.RecTitle)
            .field("RecDepartment", &self.RecDepartment)
            .field("RecOfficeLocation", &self.RecOfficeLocation)
            .field("RecHomePhone", &self.RecHomePhone)
            .field("RecOfficePhone", &self.RecOfficePhone)
            .field("SdrName", &self.SdrName)
            .field("SdrFaxNumber", &self.SdrFaxNumber)
            .field("SdrCompany", &self.SdrCompany)
            .field("SdrAddress", &self.SdrAddress)
            .field("SdrTitle", &self.SdrTitle)
            .field("SdrDepartment", &self.SdrDepartment)
            .field("SdrOfficeLocation", &self.SdrOfficeLocation)
            .field("SdrHomePhone", &self.SdrHomePhone)
            .field("SdrOfficePhone", &self.SdrOfficePhone)
            .field("Note", &self.Note)
            .field("Subject", &self.Subject)
            .field("TimeSent", &self.TimeSent)
            .field("PageCount", &self.PageCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_COVERPAGE_INFOA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_COVERPAGE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.CoverPageName == other.CoverPageName
            && self.UseServerCoverPage == other.UseServerCoverPage
            && self.RecName == other.RecName
            && self.RecFaxNumber == other.RecFaxNumber
            && self.RecCompany == other.RecCompany
            && self.RecStreetAddress == other.RecStreetAddress
            && self.RecCity == other.RecCity
            && self.RecState == other.RecState
            && self.RecZip == other.RecZip
            && self.RecCountry == other.RecCountry
            && self.RecTitle == other.RecTitle
            && self.RecDepartment == other.RecDepartment
            && self.RecOfficeLocation == other.RecOfficeLocation
            && self.RecHomePhone == other.RecHomePhone
            && self.RecOfficePhone == other.RecOfficePhone
            && self.SdrName == other.SdrName
            && self.SdrFaxNumber == other.SdrFaxNumber
            && self.SdrCompany == other.SdrCompany
            && self.SdrAddress == other.SdrAddress
            && self.SdrTitle == other.SdrTitle
            && self.SdrDepartment == other.SdrDepartment
            && self.SdrOfficeLocation == other.SdrOfficeLocation
            && self.SdrHomePhone == other.SdrHomePhone
            && self.SdrOfficePhone == other.SdrOfficePhone
            && self.Note == other.Note
            && self.Subject == other.Subject
            && self.TimeSent == other.TimeSent
            && self.PageCount == other.PageCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_COVERPAGE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_COVERPAGE_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_COVERPAGE_INFOW {
    pub SizeOfStruct: u32,
    pub CoverPageName: ::windows_core::PCWSTR,
    pub UseServerCoverPage: super::super::Foundation::BOOL,
    pub RecName: ::windows_core::PCWSTR,
    pub RecFaxNumber: ::windows_core::PCWSTR,
    pub RecCompany: ::windows_core::PCWSTR,
    pub RecStreetAddress: ::windows_core::PCWSTR,
    pub RecCity: ::windows_core::PCWSTR,
    pub RecState: ::windows_core::PCWSTR,
    pub RecZip: ::windows_core::PCWSTR,
    pub RecCountry: ::windows_core::PCWSTR,
    pub RecTitle: ::windows_core::PCWSTR,
    pub RecDepartment: ::windows_core::PCWSTR,
    pub RecOfficeLocation: ::windows_core::PCWSTR,
    pub RecHomePhone: ::windows_core::PCWSTR,
    pub RecOfficePhone: ::windows_core::PCWSTR,
    pub SdrName: ::windows_core::PCWSTR,
    pub SdrFaxNumber: ::windows_core::PCWSTR,
    pub SdrCompany: ::windows_core::PCWSTR,
    pub SdrAddress: ::windows_core::PCWSTR,
    pub SdrTitle: ::windows_core::PCWSTR,
    pub SdrDepartment: ::windows_core::PCWSTR,
    pub SdrOfficeLocation: ::windows_core::PCWSTR,
    pub SdrHomePhone: ::windows_core::PCWSTR,
    pub SdrOfficePhone: ::windows_core::PCWSTR,
    pub Note: ::windows_core::PCWSTR,
    pub Subject: ::windows_core::PCWSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_COVERPAGE_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_COVERPAGE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_COVERPAGE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_COVERPAGE_INFOW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("CoverPageName", &self.CoverPageName)
            .field("UseServerCoverPage", &self.UseServerCoverPage)
            .field("RecName", &self.RecName)
            .field("RecFaxNumber", &self.RecFaxNumber)
            .field("RecCompany", &self.RecCompany)
            .field("RecStreetAddress", &self.RecStreetAddress)
            .field("RecCity", &self.RecCity)
            .field("RecState", &self.RecState)
            .field("RecZip", &self.RecZip)
            .field("RecCountry", &self.RecCountry)
            .field("RecTitle", &self.RecTitle)
            .field("RecDepartment", &self.RecDepartment)
            .field("RecOfficeLocation", &self.RecOfficeLocation)
            .field("RecHomePhone", &self.RecHomePhone)
            .field("RecOfficePhone", &self.RecOfficePhone)
            .field("SdrName", &self.SdrName)
            .field("SdrFaxNumber", &self.SdrFaxNumber)
            .field("SdrCompany", &self.SdrCompany)
            .field("SdrAddress", &self.SdrAddress)
            .field("SdrTitle", &self.SdrTitle)
            .field("SdrDepartment", &self.SdrDepartment)
            .field("SdrOfficeLocation", &self.SdrOfficeLocation)
            .field("SdrHomePhone", &self.SdrHomePhone)
            .field("SdrOfficePhone", &self.SdrOfficePhone)
            .field("Note", &self.Note)
            .field("Subject", &self.Subject)
            .field("TimeSent", &self.TimeSent)
            .field("PageCount", &self.PageCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_COVERPAGE_INFOW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_COVERPAGE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.CoverPageName == other.CoverPageName
            && self.UseServerCoverPage == other.UseServerCoverPage
            && self.RecName == other.RecName
            && self.RecFaxNumber == other.RecFaxNumber
            && self.RecCompany == other.RecCompany
            && self.RecStreetAddress == other.RecStreetAddress
            && self.RecCity == other.RecCity
            && self.RecState == other.RecState
            && self.RecZip == other.RecZip
            && self.RecCountry == other.RecCountry
            && self.RecTitle == other.RecTitle
            && self.RecDepartment == other.RecDepartment
            && self.RecOfficeLocation == other.RecOfficeLocation
            && self.RecHomePhone == other.RecHomePhone
            && self.RecOfficePhone == other.RecOfficePhone
            && self.SdrName == other.SdrName
            && self.SdrFaxNumber == other.SdrFaxNumber
            && self.SdrCompany == other.SdrCompany
            && self.SdrAddress == other.SdrAddress
            && self.SdrTitle == other.SdrTitle
            && self.SdrDepartment == other.SdrDepartment
            && self.SdrOfficeLocation == other.SdrOfficeLocation
            && self.SdrHomePhone == other.SdrHomePhone
            && self.SdrOfficePhone == other.SdrOfficePhone
            && self.Note == other.Note
            && self.Subject == other.Subject
            && self.TimeSent == other.TimeSent
            && self.PageCount == other.PageCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_COVERPAGE_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_COVERPAGE_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_DEVICE_STATUSA {
    pub SizeOfStruct: u32,
    pub CallerId: ::windows_core::PCSTR,
    pub Csid: ::windows_core::PCSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: ::windows_core::PCSTR,
    pub DocumentName: ::windows_core::PCSTR,
    pub JobType: u32,
    pub PhoneNumber: ::windows_core::PCSTR,
    pub RoutingString: ::windows_core::PCSTR,
    pub SenderName: ::windows_core::PCSTR,
    pub RecipientName: ::windows_core::PCSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: ::windows_core::PCSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: ::windows_core::PCSTR,
    pub UserName: ::windows_core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_DEVICE_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_DEVICE_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_DEVICE_STATUSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_DEVICE_STATUSA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("CallerId", &self.CallerId)
            .field("Csid", &self.Csid)
            .field("CurrentPage", &self.CurrentPage)
            .field("DeviceId", &self.DeviceId)
            .field("DeviceName", &self.DeviceName)
            .field("DocumentName", &self.DocumentName)
            .field("JobType", &self.JobType)
            .field("PhoneNumber", &self.PhoneNumber)
            .field("RoutingString", &self.RoutingString)
            .field("SenderName", &self.SenderName)
            .field("RecipientName", &self.RecipientName)
            .field("Size", &self.Size)
            .field("StartTime", &self.StartTime)
            .field("Status", &self.Status)
            .field("StatusString", &self.StatusString)
            .field("SubmittedTime", &self.SubmittedTime)
            .field("TotalPages", &self.TotalPages)
            .field("Tsid", &self.Tsid)
            .field("UserName", &self.UserName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_DEVICE_STATUSA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_DEVICE_STATUSA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.CallerId == other.CallerId && self.Csid == other.Csid && self.CurrentPage == other.CurrentPage && self.DeviceId == other.DeviceId && self.DeviceName == other.DeviceName && self.DocumentName == other.DocumentName && self.JobType == other.JobType && self.PhoneNumber == other.PhoneNumber && self.RoutingString == other.RoutingString && self.SenderName == other.SenderName && self.RecipientName == other.RecipientName && self.Size == other.Size && self.StartTime == other.StartTime && self.Status == other.Status && self.StatusString == other.StatusString && self.SubmittedTime == other.SubmittedTime && self.TotalPages == other.TotalPages && self.Tsid == other.Tsid && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_DEVICE_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_DEVICE_STATUSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_DEVICE_STATUSW {
    pub SizeOfStruct: u32,
    pub CallerId: ::windows_core::PCWSTR,
    pub Csid: ::windows_core::PCWSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: ::windows_core::PCWSTR,
    pub DocumentName: ::windows_core::PCWSTR,
    pub JobType: u32,
    pub PhoneNumber: ::windows_core::PCWSTR,
    pub RoutingString: ::windows_core::PCWSTR,
    pub SenderName: ::windows_core::PCWSTR,
    pub RecipientName: ::windows_core::PCWSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: ::windows_core::PCWSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: ::windows_core::PCWSTR,
    pub UserName: ::windows_core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_DEVICE_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_DEVICE_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_DEVICE_STATUSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_DEVICE_STATUSW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("CallerId", &self.CallerId)
            .field("Csid", &self.Csid)
            .field("CurrentPage", &self.CurrentPage)
            .field("DeviceId", &self.DeviceId)
            .field("DeviceName", &self.DeviceName)
            .field("DocumentName", &self.DocumentName)
            .field("JobType", &self.JobType)
            .field("PhoneNumber", &self.PhoneNumber)
            .field("RoutingString", &self.RoutingString)
            .field("SenderName", &self.SenderName)
            .field("RecipientName", &self.RecipientName)
            .field("Size", &self.Size)
            .field("StartTime", &self.StartTime)
            .field("Status", &self.Status)
            .field("StatusString", &self.StatusString)
            .field("SubmittedTime", &self.SubmittedTime)
            .field("TotalPages", &self.TotalPages)
            .field("Tsid", &self.Tsid)
            .field("UserName", &self.UserName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_DEVICE_STATUSW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_DEVICE_STATUSW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.CallerId == other.CallerId && self.Csid == other.Csid && self.CurrentPage == other.CurrentPage && self.DeviceId == other.DeviceId && self.DeviceName == other.DeviceName && self.DocumentName == other.DocumentName && self.JobType == other.JobType && self.PhoneNumber == other.PhoneNumber && self.RoutingString == other.RoutingString && self.SenderName == other.SenderName && self.RecipientName == other.RecipientName && self.Size == other.Size && self.StartTime == other.StartTime && self.Status == other.Status && self.StatusString == other.StatusString && self.SubmittedTime == other.SubmittedTime && self.TotalPages == other.TotalPages && self.Tsid == other.Tsid && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_DEVICE_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_DEVICE_STATUSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_DEV_STATUS {
    pub SizeOfStruct: u32,
    pub StatusId: u32,
    pub StringId: u32,
    pub PageCount: u32,
    pub CSI: ::windows_core::PWSTR,
    pub CallerId: ::windows_core::PWSTR,
    pub RoutingInfo: ::windows_core::PWSTR,
    pub ErrorCode: u32,
    pub Reserved: [u32; 3],
}
impl ::core::marker::Copy for FAX_DEV_STATUS {}
impl ::core::clone::Clone for FAX_DEV_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_DEV_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_DEV_STATUS").field("SizeOfStruct", &self.SizeOfStruct).field("StatusId", &self.StatusId).field("StringId", &self.StringId).field("PageCount", &self.PageCount).field("CSI", &self.CSI).field("CallerId", &self.CallerId).field("RoutingInfo", &self.RoutingInfo).field("ErrorCode", &self.ErrorCode).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows_core::TypeKind for FAX_DEV_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_DEV_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.StatusId == other.StatusId && self.StringId == other.StringId && self.PageCount == other.PageCount && self.CSI == other.CSI && self.CallerId == other.CallerId && self.RoutingInfo == other.RoutingInfo && self.ErrorCode == other.ErrorCode && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for FAX_DEV_STATUS {}
impl ::core::default::Default for FAX_DEV_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_EVENTA {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_EVENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_EVENTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_EVENTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_EVENTA").field("SizeOfStruct", &self.SizeOfStruct).field("TimeStamp", &self.TimeStamp).field("DeviceId", &self.DeviceId).field("EventId", &self.EventId).field("JobId", &self.JobId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_EVENTA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_EVENTA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.TimeStamp == other.TimeStamp && self.DeviceId == other.DeviceId && self.EventId == other.EventId && self.JobId == other.JobId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_EVENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_EVENTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_EVENTW {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_EVENTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_EVENTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_EVENTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_EVENTW").field("SizeOfStruct", &self.SizeOfStruct).field("TimeStamp", &self.TimeStamp).field("DeviceId", &self.DeviceId).field("EventId", &self.EventId).field("JobId", &self.JobId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_EVENTW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_EVENTW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.TimeStamp == other.TimeStamp && self.DeviceId == other.DeviceId && self.EventId == other.EventId && self.JobId == other.JobId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_EVENTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_EVENTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_GLOBAL_ROUTING_INFOA {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: ::windows_core::PCSTR,
    pub FriendlyName: ::windows_core::PCSTR,
    pub FunctionName: ::windows_core::PCSTR,
    pub ExtensionImageName: ::windows_core::PCSTR,
    pub ExtensionFriendlyName: ::windows_core::PCSTR,
}
impl ::core::marker::Copy for FAX_GLOBAL_ROUTING_INFOA {}
impl ::core::clone::Clone for FAX_GLOBAL_ROUTING_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_GLOBAL_ROUTING_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_GLOBAL_ROUTING_INFOA").field("SizeOfStruct", &self.SizeOfStruct).field("Priority", &self.Priority).field("Guid", &self.Guid).field("FriendlyName", &self.FriendlyName).field("FunctionName", &self.FunctionName).field("ExtensionImageName", &self.ExtensionImageName).field("ExtensionFriendlyName", &self.ExtensionFriendlyName).finish()
    }
}
impl ::windows_core::TypeKind for FAX_GLOBAL_ROUTING_INFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_GLOBAL_ROUTING_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Priority == other.Priority && self.Guid == other.Guid && self.FriendlyName == other.FriendlyName && self.FunctionName == other.FunctionName && self.ExtensionImageName == other.ExtensionImageName && self.ExtensionFriendlyName == other.ExtensionFriendlyName
    }
}
impl ::core::cmp::Eq for FAX_GLOBAL_ROUTING_INFOA {}
impl ::core::default::Default for FAX_GLOBAL_ROUTING_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_GLOBAL_ROUTING_INFOW {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: ::windows_core::PCWSTR,
    pub FriendlyName: ::windows_core::PCWSTR,
    pub FunctionName: ::windows_core::PCWSTR,
    pub ExtensionImageName: ::windows_core::PCWSTR,
    pub ExtensionFriendlyName: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for FAX_GLOBAL_ROUTING_INFOW {}
impl ::core::clone::Clone for FAX_GLOBAL_ROUTING_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_GLOBAL_ROUTING_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_GLOBAL_ROUTING_INFOW").field("SizeOfStruct", &self.SizeOfStruct).field("Priority", &self.Priority).field("Guid", &self.Guid).field("FriendlyName", &self.FriendlyName).field("FunctionName", &self.FunctionName).field("ExtensionImageName", &self.ExtensionImageName).field("ExtensionFriendlyName", &self.ExtensionFriendlyName).finish()
    }
}
impl ::windows_core::TypeKind for FAX_GLOBAL_ROUTING_INFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_GLOBAL_ROUTING_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Priority == other.Priority && self.Guid == other.Guid && self.FriendlyName == other.FriendlyName && self.FunctionName == other.FunctionName && self.ExtensionImageName == other.ExtensionImageName && self.ExtensionFriendlyName == other.ExtensionFriendlyName
    }
}
impl ::core::cmp::Eq for FAX_GLOBAL_ROUTING_INFOW {}
impl ::core::default::Default for FAX_GLOBAL_ROUTING_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_ENTRYA {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: ::windows_core::PCSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: ::windows_core::PCSTR,
    pub RecipientName: ::windows_core::PCSTR,
    pub Tsid: ::windows_core::PCSTR,
    pub SenderName: ::windows_core::PCSTR,
    pub SenderCompany: ::windows_core::PCSTR,
    pub SenderDept: ::windows_core::PCSTR,
    pub BillingCode: ::windows_core::PCSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows_core::PCSTR,
    pub DocumentName: ::windows_core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_JOB_ENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_JOB_ENTRYA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("JobId", &self.JobId)
            .field("UserName", &self.UserName)
            .field("JobType", &self.JobType)
            .field("QueueStatus", &self.QueueStatus)
            .field("Status", &self.Status)
            .field("Size", &self.Size)
            .field("PageCount", &self.PageCount)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("RecipientName", &self.RecipientName)
            .field("Tsid", &self.Tsid)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("BillingCode", &self.BillingCode)
            .field("ScheduleAction", &self.ScheduleAction)
            .field("ScheduleTime", &self.ScheduleTime)
            .field("DeliveryReportType", &self.DeliveryReportType)
            .field("DeliveryReportAddress", &self.DeliveryReportAddress)
            .field("DocumentName", &self.DocumentName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_JOB_ENTRYA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_JOB_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.JobId == other.JobId
            && self.UserName == other.UserName
            && self.JobType == other.JobType
            && self.QueueStatus == other.QueueStatus
            && self.Status == other.Status
            && self.Size == other.Size
            && self.PageCount == other.PageCount
            && self.RecipientNumber == other.RecipientNumber
            && self.RecipientName == other.RecipientName
            && self.Tsid == other.Tsid
            && self.SenderName == other.SenderName
            && self.SenderCompany == other.SenderCompany
            && self.SenderDept == other.SenderDept
            && self.BillingCode == other.BillingCode
            && self.ScheduleAction == other.ScheduleAction
            && self.ScheduleTime == other.ScheduleTime
            && self.DeliveryReportType == other.DeliveryReportType
            && self.DeliveryReportAddress == other.DeliveryReportAddress
            && self.DocumentName == other.DocumentName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_JOB_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_JOB_ENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_ENTRYW {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: ::windows_core::PCWSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: ::windows_core::PCWSTR,
    pub RecipientName: ::windows_core::PCWSTR,
    pub Tsid: ::windows_core::PCWSTR,
    pub SenderName: ::windows_core::PCWSTR,
    pub SenderCompany: ::windows_core::PCWSTR,
    pub SenderDept: ::windows_core::PCWSTR,
    pub BillingCode: ::windows_core::PCWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows_core::PCWSTR,
    pub DocumentName: ::windows_core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_ENTRYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_JOB_ENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_JOB_ENTRYW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("JobId", &self.JobId)
            .field("UserName", &self.UserName)
            .field("JobType", &self.JobType)
            .field("QueueStatus", &self.QueueStatus)
            .field("Status", &self.Status)
            .field("Size", &self.Size)
            .field("PageCount", &self.PageCount)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("RecipientName", &self.RecipientName)
            .field("Tsid", &self.Tsid)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("BillingCode", &self.BillingCode)
            .field("ScheduleAction", &self.ScheduleAction)
            .field("ScheduleTime", &self.ScheduleTime)
            .field("DeliveryReportType", &self.DeliveryReportType)
            .field("DeliveryReportAddress", &self.DeliveryReportAddress)
            .field("DocumentName", &self.DocumentName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_JOB_ENTRYW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_JOB_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.JobId == other.JobId
            && self.UserName == other.UserName
            && self.JobType == other.JobType
            && self.QueueStatus == other.QueueStatus
            && self.Status == other.Status
            && self.Size == other.Size
            && self.PageCount == other.PageCount
            && self.RecipientNumber == other.RecipientNumber
            && self.RecipientName == other.RecipientName
            && self.Tsid == other.Tsid
            && self.SenderName == other.SenderName
            && self.SenderCompany == other.SenderCompany
            && self.SenderDept == other.SenderDept
            && self.BillingCode == other.BillingCode
            && self.ScheduleAction == other.ScheduleAction
            && self.ScheduleTime == other.ScheduleTime
            && self.DeliveryReportType == other.DeliveryReportType
            && self.DeliveryReportAddress == other.DeliveryReportAddress
            && self.DocumentName == other.DocumentName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_JOB_ENTRYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_JOB_ENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_PARAMA {
    pub SizeOfStruct: u32,
    pub RecipientNumber: ::windows_core::PCSTR,
    pub RecipientName: ::windows_core::PCSTR,
    pub Tsid: ::windows_core::PCSTR,
    pub SenderName: ::windows_core::PCSTR,
    pub SenderCompany: ::windows_core::PCSTR,
    pub SenderDept: ::windows_core::PCSTR,
    pub BillingCode: ::windows_core::PCSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows_core::PCSTR,
    pub DocumentName: ::windows_core::PCSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_PARAMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_PARAMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_JOB_PARAMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_JOB_PARAMA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("RecipientName", &self.RecipientName)
            .field("Tsid", &self.Tsid)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("BillingCode", &self.BillingCode)
            .field("ScheduleAction", &self.ScheduleAction)
            .field("ScheduleTime", &self.ScheduleTime)
            .field("DeliveryReportType", &self.DeliveryReportType)
            .field("DeliveryReportAddress", &self.DeliveryReportAddress)
            .field("DocumentName", &self.DocumentName)
            .field("CallHandle", &self.CallHandle)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_JOB_PARAMA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_JOB_PARAMA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.RecipientNumber == other.RecipientNumber && self.RecipientName == other.RecipientName && self.Tsid == other.Tsid && self.SenderName == other.SenderName && self.SenderCompany == other.SenderCompany && self.SenderDept == other.SenderDept && self.BillingCode == other.BillingCode && self.ScheduleAction == other.ScheduleAction && self.ScheduleTime == other.ScheduleTime && self.DeliveryReportType == other.DeliveryReportType && self.DeliveryReportAddress == other.DeliveryReportAddress && self.DocumentName == other.DocumentName && self.CallHandle == other.CallHandle && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_JOB_PARAMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_JOB_PARAMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_PARAMW {
    pub SizeOfStruct: u32,
    pub RecipientNumber: ::windows_core::PCWSTR,
    pub RecipientName: ::windows_core::PCWSTR,
    pub Tsid: ::windows_core::PCWSTR,
    pub SenderName: ::windows_core::PCWSTR,
    pub SenderCompany: ::windows_core::PCWSTR,
    pub SenderDept: ::windows_core::PCWSTR,
    pub BillingCode: ::windows_core::PCWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows_core::PCWSTR,
    pub DocumentName: ::windows_core::PCWSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_PARAMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_PARAMW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_JOB_PARAMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_JOB_PARAMW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("RecipientName", &self.RecipientName)
            .field("Tsid", &self.Tsid)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("BillingCode", &self.BillingCode)
            .field("ScheduleAction", &self.ScheduleAction)
            .field("ScheduleTime", &self.ScheduleTime)
            .field("DeliveryReportType", &self.DeliveryReportType)
            .field("DeliveryReportAddress", &self.DeliveryReportAddress)
            .field("DocumentName", &self.DocumentName)
            .field("CallHandle", &self.CallHandle)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_JOB_PARAMW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_JOB_PARAMW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.RecipientNumber == other.RecipientNumber && self.RecipientName == other.RecipientName && self.Tsid == other.Tsid && self.SenderName == other.SenderName && self.SenderCompany == other.SenderCompany && self.SenderDept == other.SenderDept && self.BillingCode == other.BillingCode && self.ScheduleAction == other.ScheduleAction && self.ScheduleTime == other.ScheduleTime && self.DeliveryReportType == other.DeliveryReportType && self.DeliveryReportAddress == other.DeliveryReportAddress && self.DocumentName == other.DocumentName && self.CallHandle == other.CallHandle && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_JOB_PARAMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_JOB_PARAMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_LOG_CATEGORYA {
    pub Name: ::windows_core::PCSTR,
    pub Category: u32,
    pub Level: u32,
}
impl ::core::marker::Copy for FAX_LOG_CATEGORYA {}
impl ::core::clone::Clone for FAX_LOG_CATEGORYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_LOG_CATEGORYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_LOG_CATEGORYA").field("Name", &self.Name).field("Category", &self.Category).field("Level", &self.Level).finish()
    }
}
impl ::windows_core::TypeKind for FAX_LOG_CATEGORYA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_LOG_CATEGORYA {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Category == other.Category && self.Level == other.Level
    }
}
impl ::core::cmp::Eq for FAX_LOG_CATEGORYA {}
impl ::core::default::Default for FAX_LOG_CATEGORYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_LOG_CATEGORYW {
    pub Name: ::windows_core::PCWSTR,
    pub Category: u32,
    pub Level: u32,
}
impl ::core::marker::Copy for FAX_LOG_CATEGORYW {}
impl ::core::clone::Clone for FAX_LOG_CATEGORYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_LOG_CATEGORYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_LOG_CATEGORYW").field("Name", &self.Name).field("Category", &self.Category).field("Level", &self.Level).finish()
    }
}
impl ::windows_core::TypeKind for FAX_LOG_CATEGORYW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_LOG_CATEGORYW {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Category == other.Category && self.Level == other.Level
    }
}
impl ::core::cmp::Eq for FAX_LOG_CATEGORYW {}
impl ::core::default::Default for FAX_LOG_CATEGORYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PORT_INFOA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: ::windows_core::PCSTR,
    pub Tsid: ::windows_core::PCSTR,
    pub Csid: ::windows_core::PCSTR,
}
impl ::core::marker::Copy for FAX_PORT_INFOA {}
impl ::core::clone::Clone for FAX_PORT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_PORT_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_PORT_INFOA").field("SizeOfStruct", &self.SizeOfStruct).field("DeviceId", &self.DeviceId).field("State", &self.State).field("Flags", &self.Flags).field("Rings", &self.Rings).field("Priority", &self.Priority).field("DeviceName", &self.DeviceName).field("Tsid", &self.Tsid).field("Csid", &self.Csid).finish()
    }
}
impl ::windows_core::TypeKind for FAX_PORT_INFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_PORT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DeviceId == other.DeviceId && self.State == other.State && self.Flags == other.Flags && self.Rings == other.Rings && self.Priority == other.Priority && self.DeviceName == other.DeviceName && self.Tsid == other.Tsid && self.Csid == other.Csid
    }
}
impl ::core::cmp::Eq for FAX_PORT_INFOA {}
impl ::core::default::Default for FAX_PORT_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PORT_INFOW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: ::windows_core::PCWSTR,
    pub Tsid: ::windows_core::PCWSTR,
    pub Csid: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for FAX_PORT_INFOW {}
impl ::core::clone::Clone for FAX_PORT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_PORT_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_PORT_INFOW").field("SizeOfStruct", &self.SizeOfStruct).field("DeviceId", &self.DeviceId).field("State", &self.State).field("Flags", &self.Flags).field("Rings", &self.Rings).field("Priority", &self.Priority).field("DeviceName", &self.DeviceName).field("Tsid", &self.Tsid).field("Csid", &self.Csid).finish()
    }
}
impl ::windows_core::TypeKind for FAX_PORT_INFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_PORT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DeviceId == other.DeviceId && self.State == other.State && self.Flags == other.Flags && self.Rings == other.Rings && self.Priority == other.Priority && self.DeviceName == other.DeviceName && self.Tsid == other.Tsid && self.Csid == other.Csid
    }
}
impl ::core::cmp::Eq for FAX_PORT_INFOW {}
impl ::core::default::Default for FAX_PORT_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PRINT_INFOA {
    pub SizeOfStruct: u32,
    pub DocName: ::windows_core::PCSTR,
    pub RecipientName: ::windows_core::PCSTR,
    pub RecipientNumber: ::windows_core::PCSTR,
    pub SenderName: ::windows_core::PCSTR,
    pub SenderCompany: ::windows_core::PCSTR,
    pub SenderDept: ::windows_core::PCSTR,
    pub SenderBillingCode: ::windows_core::PCSTR,
    pub Reserved: ::windows_core::PCSTR,
    pub DrEmailAddress: ::windows_core::PCSTR,
    pub OutputFileName: ::windows_core::PCSTR,
}
impl ::core::marker::Copy for FAX_PRINT_INFOA {}
impl ::core::clone::Clone for FAX_PRINT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_PRINT_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_PRINT_INFOA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("DocName", &self.DocName)
            .field("RecipientName", &self.RecipientName)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("SenderBillingCode", &self.SenderBillingCode)
            .field("Reserved", &self.Reserved)
            .field("DrEmailAddress", &self.DrEmailAddress)
            .field("OutputFileName", &self.OutputFileName)
            .finish()
    }
}
impl ::windows_core::TypeKind for FAX_PRINT_INFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_PRINT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DocName == other.DocName && self.RecipientName == other.RecipientName && self.RecipientNumber == other.RecipientNumber && self.SenderName == other.SenderName && self.SenderCompany == other.SenderCompany && self.SenderDept == other.SenderDept && self.SenderBillingCode == other.SenderBillingCode && self.Reserved == other.Reserved && self.DrEmailAddress == other.DrEmailAddress && self.OutputFileName == other.OutputFileName
    }
}
impl ::core::cmp::Eq for FAX_PRINT_INFOA {}
impl ::core::default::Default for FAX_PRINT_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PRINT_INFOW {
    pub SizeOfStruct: u32,
    pub DocName: ::windows_core::PCWSTR,
    pub RecipientName: ::windows_core::PCWSTR,
    pub RecipientNumber: ::windows_core::PCWSTR,
    pub SenderName: ::windows_core::PCWSTR,
    pub SenderCompany: ::windows_core::PCWSTR,
    pub SenderDept: ::windows_core::PCWSTR,
    pub SenderBillingCode: ::windows_core::PCWSTR,
    pub Reserved: ::windows_core::PCWSTR,
    pub DrEmailAddress: ::windows_core::PCWSTR,
    pub OutputFileName: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for FAX_PRINT_INFOW {}
impl ::core::clone::Clone for FAX_PRINT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_PRINT_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_PRINT_INFOW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("DocName", &self.DocName)
            .field("RecipientName", &self.RecipientName)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("SenderBillingCode", &self.SenderBillingCode)
            .field("Reserved", &self.Reserved)
            .field("DrEmailAddress", &self.DrEmailAddress)
            .field("OutputFileName", &self.OutputFileName)
            .finish()
    }
}
impl ::windows_core::TypeKind for FAX_PRINT_INFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_PRINT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DocName == other.DocName && self.RecipientName == other.RecipientName && self.RecipientNumber == other.RecipientNumber && self.SenderName == other.SenderName && self.SenderCompany == other.SenderCompany && self.SenderDept == other.SenderDept && self.SenderBillingCode == other.SenderBillingCode && self.Reserved == other.Reserved && self.DrEmailAddress == other.DrEmailAddress && self.OutputFileName == other.OutputFileName
    }
}
impl ::core::cmp::Eq for FAX_PRINT_INFOW {}
impl ::core::default::Default for FAX_PRINT_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_RECEIVE {
    pub SizeOfStruct: u32,
    pub FileName: ::windows_core::PWSTR,
    pub ReceiverName: ::windows_core::PWSTR,
    pub ReceiverNumber: ::windows_core::PWSTR,
    pub Reserved: [u32; 4],
}
impl ::core::marker::Copy for FAX_RECEIVE {}
impl ::core::clone::Clone for FAX_RECEIVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_RECEIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_RECEIVE").field("SizeOfStruct", &self.SizeOfStruct).field("FileName", &self.FileName).field("ReceiverName", &self.ReceiverName).field("ReceiverNumber", &self.ReceiverNumber).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows_core::TypeKind for FAX_RECEIVE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_RECEIVE {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.FileName == other.FileName && self.ReceiverName == other.ReceiverName && self.ReceiverNumber == other.ReceiverNumber && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for FAX_RECEIVE {}
impl ::core::default::Default for FAX_RECEIVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_ROUTE {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub ElapsedTime: u64,
    pub ReceiveTime: u64,
    pub PageCount: u32,
    pub Csid: ::windows_core::PCWSTR,
    pub Tsid: ::windows_core::PCWSTR,
    pub CallerId: ::windows_core::PCWSTR,
    pub RoutingInfo: ::windows_core::PCWSTR,
    pub ReceiverName: ::windows_core::PCWSTR,
    pub ReceiverNumber: ::windows_core::PCWSTR,
    pub DeviceName: ::windows_core::PCWSTR,
    pub DeviceId: u32,
    pub RoutingInfoData: *mut u8,
    pub RoutingInfoDataSize: u32,
}
impl ::core::marker::Copy for FAX_ROUTE {}
impl ::core::clone::Clone for FAX_ROUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_ROUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_ROUTE")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("JobId", &self.JobId)
            .field("ElapsedTime", &self.ElapsedTime)
            .field("ReceiveTime", &self.ReceiveTime)
            .field("PageCount", &self.PageCount)
            .field("Csid", &self.Csid)
            .field("Tsid", &self.Tsid)
            .field("CallerId", &self.CallerId)
            .field("RoutingInfo", &self.RoutingInfo)
            .field("ReceiverName", &self.ReceiverName)
            .field("ReceiverNumber", &self.ReceiverNumber)
            .field("DeviceName", &self.DeviceName)
            .field("DeviceId", &self.DeviceId)
            .field("RoutingInfoData", &self.RoutingInfoData)
            .field("RoutingInfoDataSize", &self.RoutingInfoDataSize)
            .finish()
    }
}
impl ::windows_core::TypeKind for FAX_ROUTE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_ROUTE {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.JobId == other.JobId && self.ElapsedTime == other.ElapsedTime && self.ReceiveTime == other.ReceiveTime && self.PageCount == other.PageCount && self.Csid == other.Csid && self.Tsid == other.Tsid && self.CallerId == other.CallerId && self.RoutingInfo == other.RoutingInfo && self.ReceiverName == other.ReceiverName && self.ReceiverNumber == other.ReceiverNumber && self.DeviceName == other.DeviceName && self.DeviceId == other.DeviceId && self.RoutingInfoData == other.RoutingInfoData && self.RoutingInfoDataSize == other.RoutingInfoDataSize
    }
}
impl ::core::cmp::Eq for FAX_ROUTE {}
impl ::core::default::Default for FAX_ROUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTE_CALLBACKROUTINES {
    pub SizeOfStruct: u32,
    pub FaxRouteAddFile: PFAXROUTEADDFILE,
    pub FaxRouteDeleteFile: PFAXROUTEDELETEFILE,
    pub FaxRouteGetFile: PFAXROUTEGETFILE,
    pub FaxRouteEnumFiles: PFAXROUTEENUMFILES,
    pub FaxRouteModifyRoutingData: PFAXROUTEMODIFYROUTINGDATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_ROUTE_CALLBACKROUTINES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_ROUTE_CALLBACKROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_ROUTE_CALLBACKROUTINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_ROUTE_CALLBACKROUTINES").field("SizeOfStruct", &self.SizeOfStruct).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_ROUTE_CALLBACKROUTINES {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_ROUTE_CALLBACKROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTING_METHODA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: super::super::Foundation::BOOL,
    pub DeviceName: ::windows_core::PCSTR,
    pub Guid: ::windows_core::PCSTR,
    pub FriendlyName: ::windows_core::PCSTR,
    pub FunctionName: ::windows_core::PCSTR,
    pub ExtensionImageName: ::windows_core::PCSTR,
    pub ExtensionFriendlyName: ::windows_core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_ROUTING_METHODA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_ROUTING_METHODA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_ROUTING_METHODA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_ROUTING_METHODA").field("SizeOfStruct", &self.SizeOfStruct).field("DeviceId", &self.DeviceId).field("Enabled", &self.Enabled).field("DeviceName", &self.DeviceName).field("Guid", &self.Guid).field("FriendlyName", &self.FriendlyName).field("FunctionName", &self.FunctionName).field("ExtensionImageName", &self.ExtensionImageName).field("ExtensionFriendlyName", &self.ExtensionFriendlyName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_ROUTING_METHODA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_ROUTING_METHODA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DeviceId == other.DeviceId && self.Enabled == other.Enabled && self.DeviceName == other.DeviceName && self.Guid == other.Guid && self.FriendlyName == other.FriendlyName && self.FunctionName == other.FunctionName && self.ExtensionImageName == other.ExtensionImageName && self.ExtensionFriendlyName == other.ExtensionFriendlyName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_ROUTING_METHODA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_ROUTING_METHODA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTING_METHODW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: super::super::Foundation::BOOL,
    pub DeviceName: ::windows_core::PCWSTR,
    pub Guid: ::windows_core::PCWSTR,
    pub FriendlyName: ::windows_core::PCWSTR,
    pub FunctionName: ::windows_core::PCWSTR,
    pub ExtensionImageName: ::windows_core::PCWSTR,
    pub ExtensionFriendlyName: ::windows_core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_ROUTING_METHODW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_ROUTING_METHODW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_ROUTING_METHODW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_ROUTING_METHODW").field("SizeOfStruct", &self.SizeOfStruct).field("DeviceId", &self.DeviceId).field("Enabled", &self.Enabled).field("DeviceName", &self.DeviceName).field("Guid", &self.Guid).field("FriendlyName", &self.FriendlyName).field("FunctionName", &self.FunctionName).field("ExtensionImageName", &self.ExtensionImageName).field("ExtensionFriendlyName", &self.ExtensionFriendlyName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_ROUTING_METHODW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_ROUTING_METHODW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DeviceId == other.DeviceId && self.Enabled == other.Enabled && self.DeviceName == other.DeviceName && self.Guid == other.Guid && self.FriendlyName == other.FriendlyName && self.FunctionName == other.FunctionName && self.ExtensionImageName == other.ExtensionImageName && self.ExtensionFriendlyName == other.ExtensionFriendlyName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_ROUTING_METHODW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_ROUTING_METHODW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_SEND {
    pub SizeOfStruct: u32,
    pub FileName: ::windows_core::PWSTR,
    pub CallerName: ::windows_core::PWSTR,
    pub CallerNumber: ::windows_core::PWSTR,
    pub ReceiverName: ::windows_core::PWSTR,
    pub ReceiverNumber: ::windows_core::PWSTR,
    pub Branding: super::super::Foundation::BOOL,
    pub CallHandle: u32,
    pub Reserved: [u32; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_SEND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_SEND {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_SEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_SEND").field("SizeOfStruct", &self.SizeOfStruct).field("FileName", &self.FileName).field("CallerName", &self.CallerName).field("CallerNumber", &self.CallerNumber).field("ReceiverName", &self.ReceiverName).field("ReceiverNumber", &self.ReceiverNumber).field("Branding", &self.Branding).field("CallHandle", &self.CallHandle).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for FAX_SEND {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_SEND {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.FileName == other.FileName && self.CallerName == other.CallerName && self.CallerNumber == other.CallerNumber && self.ReceiverName == other.ReceiverName && self.ReceiverNumber == other.ReceiverNumber && self.Branding == other.Branding && self.CallHandle == other.CallHandle && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_SEND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_SEND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_TIME {
    pub Hour: u16,
    pub Minute: u16,
}
impl ::core::marker::Copy for FAX_TIME {}
impl ::core::clone::Clone for FAX_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_TIME").field("Hour", &self.Hour).field("Minute", &self.Minute).finish()
    }
}
impl ::windows_core::TypeKind for FAX_TIME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FAX_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.Hour == other.Hour && self.Minute == other.Minute
    }
}
impl ::core::cmp::Eq for FAX_TIME {}
impl ::core::default::Default for FAX_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STINOTIFY {
    pub dwSize: u32,
    pub guidNotificationCode: ::windows_core::GUID,
    pub abNotificationData: [u8; 64],
}
impl ::core::marker::Copy for STINOTIFY {}
impl ::core::clone::Clone for STINOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STINOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STINOTIFY").field("dwSize", &self.dwSize).field("guidNotificationCode", &self.guidNotificationCode).field("abNotificationData", &self.abNotificationData).finish()
    }
}
impl ::windows_core::TypeKind for STINOTIFY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for STINOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidNotificationCode == other.guidNotificationCode && self.abNotificationData == other.abNotificationData
    }
}
impl ::core::cmp::Eq for STINOTIFY {}
impl ::core::default::Default for STINOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STISUBSCRIBE {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFilter: u32,
    pub hWndNotify: super::super::Foundation::HWND,
    pub hEvent: super::super::Foundation::HANDLE,
    pub uiNotificationMessage: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STISUBSCRIBE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STISUBSCRIBE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STISUBSCRIBE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STISUBSCRIBE").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwFilter", &self.dwFilter).field("hWndNotify", &self.hWndNotify).field("hEvent", &self.hEvent).field("uiNotificationMessage", &self.uiNotificationMessage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for STISUBSCRIBE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STISUBSCRIBE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwFilter == other.dwFilter && self.hWndNotify == other.hWndNotify && self.hEvent == other.hEvent && self.uiNotificationMessage == other.uiNotificationMessage
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STISUBSCRIBE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STISUBSCRIBE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: ::windows_core::PWSTR,
    pub pszDeviceDescription: ::windows_core::PWSTR,
    pub pszPortName: ::windows_core::PWSTR,
    pub pszPropProvider: ::windows_core::PWSTR,
    pub pszLocalName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for STI_DEVICE_INFORMATIONW {}
impl ::core::clone::Clone for STI_DEVICE_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_DEVICE_INFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_DEVICE_INFORMATIONW")
            .field("dwSize", &self.dwSize)
            .field("DeviceType", &self.DeviceType)
            .field("szDeviceInternalName", &self.szDeviceInternalName)
            .field("DeviceCapabilitiesA", &self.DeviceCapabilitiesA)
            .field("dwHardwareConfiguration", &self.dwHardwareConfiguration)
            .field("pszVendorDescription", &self.pszVendorDescription)
            .field("pszDeviceDescription", &self.pszDeviceDescription)
            .field("pszPortName", &self.pszPortName)
            .field("pszPropProvider", &self.pszPropProvider)
            .field("pszLocalName", &self.pszLocalName)
            .finish()
    }
}
impl ::windows_core::TypeKind for STI_DEVICE_INFORMATIONW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for STI_DEVICE_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.DeviceType == other.DeviceType && self.szDeviceInternalName == other.szDeviceInternalName && self.DeviceCapabilitiesA == other.DeviceCapabilitiesA && self.dwHardwareConfiguration == other.dwHardwareConfiguration && self.pszVendorDescription == other.pszVendorDescription && self.pszDeviceDescription == other.pszDeviceDescription && self.pszPortName == other.pszPortName && self.pszPropProvider == other.pszPropProvider && self.pszLocalName == other.pszLocalName
    }
}
impl ::core::cmp::Eq for STI_DEVICE_INFORMATIONW {}
impl ::core::default::Default for STI_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DEVICE_STATUS {
    pub dwSize: u32,
    pub StatusMask: u32,
    pub dwOnlineState: u32,
    pub dwHardwareStatusCode: u32,
    pub dwEventHandlingState: u32,
    pub dwPollingInterval: u32,
}
impl ::core::marker::Copy for STI_DEVICE_STATUS {}
impl ::core::clone::Clone for STI_DEVICE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_DEVICE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_DEVICE_STATUS").field("dwSize", &self.dwSize).field("StatusMask", &self.StatusMask).field("dwOnlineState", &self.dwOnlineState).field("dwHardwareStatusCode", &self.dwHardwareStatusCode).field("dwEventHandlingState", &self.dwEventHandlingState).field("dwPollingInterval", &self.dwPollingInterval).finish()
    }
}
impl ::windows_core::TypeKind for STI_DEVICE_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for STI_DEVICE_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.StatusMask == other.StatusMask && self.dwOnlineState == other.dwOnlineState && self.dwHardwareStatusCode == other.dwHardwareStatusCode && self.dwEventHandlingState == other.dwEventHandlingState && self.dwPollingInterval == other.dwPollingInterval
    }
}
impl ::core::cmp::Eq for STI_DEVICE_STATUS {}
impl ::core::default::Default for STI_DEVICE_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DEV_CAPS {
    pub dwGeneric: u32,
}
impl ::core::marker::Copy for STI_DEV_CAPS {}
impl ::core::clone::Clone for STI_DEV_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_DEV_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_DEV_CAPS").field("dwGeneric", &self.dwGeneric).finish()
    }
}
impl ::windows_core::TypeKind for STI_DEV_CAPS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for STI_DEV_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwGeneric == other.dwGeneric
    }
}
impl ::core::cmp::Eq for STI_DEV_CAPS {}
impl ::core::default::Default for STI_DEV_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DIAG {
    pub dwSize: u32,
    pub dwBasicDiagCode: u32,
    pub dwVendorDiagCode: u32,
    pub dwStatusMask: u32,
    pub sErrorInfo: _ERROR_INFOW,
}
impl ::core::marker::Copy for STI_DIAG {}
impl ::core::clone::Clone for STI_DIAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_DIAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_DIAG").field("dwSize", &self.dwSize).field("dwBasicDiagCode", &self.dwBasicDiagCode).field("dwVendorDiagCode", &self.dwVendorDiagCode).field("dwStatusMask", &self.dwStatusMask).field("sErrorInfo", &self.sErrorInfo).finish()
    }
}
impl ::windows_core::TypeKind for STI_DIAG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for STI_DIAG {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwBasicDiagCode == other.dwBasicDiagCode && self.dwVendorDiagCode == other.dwVendorDiagCode && self.dwStatusMask == other.dwStatusMask && self.sErrorInfo == other.sErrorInfo
    }
}
impl ::core::cmp::Eq for STI_DIAG {}
impl ::core::default::Default for STI_DIAG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_USD_CAPS {
    pub dwVersion: u32,
    pub dwGenericCaps: u32,
}
impl ::core::marker::Copy for STI_USD_CAPS {}
impl ::core::clone::Clone for STI_USD_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_USD_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_USD_CAPS").field("dwVersion", &self.dwVersion).field("dwGenericCaps", &self.dwGenericCaps).finish()
    }
}
impl ::windows_core::TypeKind for STI_USD_CAPS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for STI_USD_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwGenericCaps == other.dwGenericCaps
    }
}
impl ::core::cmp::Eq for STI_USD_CAPS {}
impl ::core::default::Default for STI_USD_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_WIA_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: ::windows_core::PWSTR,
    pub pszDeviceDescription: ::windows_core::PWSTR,
    pub pszPortName: ::windows_core::PWSTR,
    pub pszPropProvider: ::windows_core::PWSTR,
    pub pszLocalName: ::windows_core::PWSTR,
    pub pszUiDll: ::windows_core::PWSTR,
    pub pszServer: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for STI_WIA_DEVICE_INFORMATIONW {}
impl ::core::clone::Clone for STI_WIA_DEVICE_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_WIA_DEVICE_INFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_WIA_DEVICE_INFORMATIONW")
            .field("dwSize", &self.dwSize)
            .field("DeviceType", &self.DeviceType)
            .field("szDeviceInternalName", &self.szDeviceInternalName)
            .field("DeviceCapabilitiesA", &self.DeviceCapabilitiesA)
            .field("dwHardwareConfiguration", &self.dwHardwareConfiguration)
            .field("pszVendorDescription", &self.pszVendorDescription)
            .field("pszDeviceDescription", &self.pszDeviceDescription)
            .field("pszPortName", &self.pszPortName)
            .field("pszPropProvider", &self.pszPropProvider)
            .field("pszLocalName", &self.pszLocalName)
            .field("pszUiDll", &self.pszUiDll)
            .field("pszServer", &self.pszServer)
            .finish()
    }
}
impl ::windows_core::TypeKind for STI_WIA_DEVICE_INFORMATIONW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for STI_WIA_DEVICE_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.DeviceType == other.DeviceType && self.szDeviceInternalName == other.szDeviceInternalName && self.DeviceCapabilitiesA == other.DeviceCapabilitiesA && self.dwHardwareConfiguration == other.dwHardwareConfiguration && self.pszVendorDescription == other.pszVendorDescription && self.pszDeviceDescription == other.pszDeviceDescription && self.pszPortName == other.pszPortName && self.pszPropProvider == other.pszPropProvider && self.pszLocalName == other.pszLocalName && self.pszUiDll == other.pszUiDll && self.pszServer == other.pszServer
    }
}
impl ::core::cmp::Eq for STI_WIA_DEVICE_INFORMATIONW {}
impl ::core::default::Default for STI_WIA_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct _ERROR_INFOW {
    pub dwSize: u32,
    pub dwGenericError: u32,
    pub dwVendorError: u32,
    pub szExtendedErrorText: [u16; 255],
}
impl ::core::marker::Copy for _ERROR_INFOW {}
impl ::core::clone::Clone for _ERROR_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _ERROR_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_ERROR_INFOW").field("dwSize", &self.dwSize).field("dwGenericError", &self.dwGenericError).field("dwVendorError", &self.dwVendorError).field("szExtendedErrorText", &self.szExtendedErrorText).finish()
    }
}
impl ::windows_core::TypeKind for _ERROR_INFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for _ERROR_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwGenericError == other.dwGenericError && self.dwVendorError == other.dwVendorError && self.szExtendedErrorText == other.szExtendedErrorText
    }
}
impl ::core::cmp::Eq for _ERROR_INFOW {}
impl ::core::default::Default for _ERROR_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXABORT = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXACCESSCHECK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCLOSE = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCOMPLETEJOBPARAMSA = ::core::option::Option<unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCOMPLETEJOBPARAMSW = ::core::option::Option<unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCONNECTFAXSERVERA = ::core::option::Option<unsafe extern "system" fn(machinename: ::windows_core::PCSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCONNECTFAXSERVERW = ::core::option::Option<unsafe extern "system" fn(machinename: ::windows_core::PCWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVABORTOPERATION = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type PFAXDEVCONFIGURE = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVENDJOB = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVINITIALIZE = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::HANDLE, param2: *mut PFAX_LINECALLBACK, param3: PFAX_SERVICE_CALLBACK) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVRECEIVE = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: *mut FAX_RECEIVE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVREPORTSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_DEV_STATUS, param2: u32, param3: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVSEND = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_SEND, param2: PFAX_SEND_CALLBACK) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXDEVSHUTDOWN = ::core::option::Option<unsafe extern "system" fn() -> ::windows_core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVSTARTJOB = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: *mut super::super::Foundation::HANDLE, param3: super::super::Foundation::HANDLE, param4: usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVVIRTUALDEVICECREATION = ::core::option::Option<unsafe extern "system" fn(devicecount: *mut u32, devicenameprefix: ::windows_core::PWSTR, deviceidprefix: *mut u32, completionport: super::super::Foundation::HANDLE, completionkey: usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENABLEROUTINGMETHODA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_core::PCSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENABLEROUTINGMETHODW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_core::PCWSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMGLOBALROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMGLOBALROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMJOBSA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMJOBSW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMPORTSA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMPORTSW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMROUTINGMETHODSA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMROUTINGMETHODSW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXFREEBUFFER = ::core::option::Option<unsafe extern "system" fn(buffer: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETCONFIGURATIONA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETCONFIGURATIONW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETDEVICESTATUSA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETDEVICESTATUSW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETJOBA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETJOBW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETLOGGINGCATEGORIESA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETLOGGINGCATEGORIESW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPAGEDATA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPORTA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPORTW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_core::PCSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_core::PCWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXINITIALIZEEVENTQUEUE = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXOPENPORT = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXPRINTCOVERPAGEA = ::core::option::Option<unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXPRINTCOVERPAGEW = ::core::option::Option<unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXREGISTERROUTINGEXTENSIONW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, extensionname: ::windows_core::PCWSTR, friendlyname: ::windows_core::PCWSTR, imagename: ::windows_core::PCWSTR, callback: PFAX_ROUTING_INSTALLATION_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXREGISTERSERVICEPROVIDERW = ::core::option::Option<unsafe extern "system" fn(deviceprovider: ::windows_core::PCWSTR, friendlyname: ::windows_core::PCWSTR, imagename: ::windows_core::PCWSTR, tspname: ::windows_core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXROUTEADDFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, filename: ::windows_core::PCWSTR, guid: *mut ::windows_core::GUID) -> i32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXROUTEDELETEFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, filename: ::windows_core::PCWSTR) -> i32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEDEVICECHANGENOTIFICATION = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEDEVICEENABLE = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCWSTR, param1: u32, param2: i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEENUMFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, guidowner: *mut ::windows_core::GUID, guidcaller: *mut ::windows_core::GUID, filename: ::windows_core::PCWSTR, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEENUMFILES = ::core::option::Option<unsafe extern "system" fn(jobid: u32, guid: *mut ::windows_core::GUID, fileenumerator: PFAXROUTEENUMFILE, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEGETFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, index: u32, filenamebuffer: ::windows_core::PWSTR, requiredsize: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEGETROUTINGINFO = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCWSTR, param1: u32, param2: *mut u8, param3: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEINITIALIZE = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_ROUTE_CALLBACKROUTINES) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEMETHOD = ::core::option::Option<unsafe extern "system" fn(param0: *const FAX_ROUTE, param1: *mut *mut ::core::ffi::c_void, param2: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEMODIFYROUTINGDATA = ::core::option::Option<unsafe extern "system" fn(jobid: u32, routingguid: ::windows_core::PCWSTR, routingdata: *mut u8, routingdatasize: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTESETROUTINGINFO = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCWSTR, param1: u32, param2: *const u8, param3: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_core::PCSTR, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTFORBROADCASTA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_core::PCSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKA, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTFORBROADCASTW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_core::PCWSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_core::PCWSTR, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETCONFIGURATIONA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETCONFIGURATIONW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETGLOBALROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETGLOBALROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETJOBA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETJOBW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETLOGGINGCATEGORIESA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETLOGGINGCATEGORIESW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETPORTA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETPORTW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_core::PCSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_core::PCWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXSTARTPRINTJOBA = ::core::option::Option<unsafe extern "system" fn(printername: ::windows_core::PCSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXSTARTPRINTJOBW = ::core::option::Option<unsafe extern "system" fn(printername: ::windows_core::PCWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXUNREGISTERSERVICEPROVIDERW = ::core::option::Option<unsafe extern "system" fn(deviceprovider: ::windows_core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAX_EXT_CONFIG_CHANGE = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: ::windows_core::PCWSTR, param2: *mut u8, param3: u32) -> ::windows_core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAX_EXT_FREE_BUFFER = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAX_EXT_GET_DATA = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: FAX_ENUM_DEVICE_ID_SOURCE, param2: ::windows_core::PCWSTR, param3: *mut *mut u8, param4: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_INITIALIZE_CONFIG = ::core::option::Option<unsafe extern "system" fn(param0: PFAX_EXT_GET_DATA, param1: PFAX_EXT_SET_DATA, param2: PFAX_EXT_REGISTER_FOR_EVENTS, param3: PFAX_EXT_UNREGISTER_FOR_EVENTS, param4: PFAX_EXT_FREE_BUFFER) -> ::windows_core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_REGISTER_FOR_EVENTS = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: ::windows_core::PCWSTR, param4: PFAX_EXT_CONFIG_CHANGE) -> super::super::Foundation::HANDLE>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_SET_DATA = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: ::windows_core::PCWSTR, param4: *mut u8, param5: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_UNREGISTER_FOR_EVENTS = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_LINECALLBACK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_RECIPIENT_CALLBACKA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *mut ::core::ffi::c_void, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_RECIPIENT_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *mut ::core::ffi::c_void, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_ROUTING_INSTALLATION_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, context: *mut ::core::ffi::c_void, methodname: ::windows_core::PCWSTR, friendlyname: ::windows_core::PCWSTR, functionname: ::windows_core::PCWSTR, guid: ::windows_core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_SEND_CALLBACK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, callhandle: u32, reserved1: u32, reserved2: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_SERVICE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, param1: usize, param2: usize, param3: usize) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
