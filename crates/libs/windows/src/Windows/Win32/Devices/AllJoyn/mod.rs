#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllJoynAcceptBusConnection<P0, P1>(serverbushandle: P0, abortevent: P1) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn AllJoynAcceptBusConnection ( serverbushandle : super::super::Foundation:: HANDLE , abortevent : super::super::Foundation:: HANDLE ) -> u32 );
    AllJoynAcceptBusConnection(serverbushandle.into_param().abi(), abortevent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllJoynCloseBusHandle<P0>(bushandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn AllJoynCloseBusHandle ( bushandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    AllJoynCloseBusHandle(bushandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllJoynConnectToBus<P0>(connectionspec: P0) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn AllJoynConnectToBus ( connectionspec : ::windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = AllJoynConnectToBus(connectionspec.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn AllJoynCreateBus(outbuffersize: u32, inbuffersize: u32, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::HANDLE {
    ::windows_targets::link ! ( "msajapi.dll""system" fn AllJoynCreateBus ( outbuffersize : u32 , inbuffersize : u32 , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: HANDLE );
    AllJoynCreateBus(outbuffersize, inbuffersize, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllJoynEnumEvents<P0, P1>(connectedbushandle: P0, eventtoreset: P1, eventtypes: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn AllJoynEnumEvents ( connectedbushandle : super::super::Foundation:: HANDLE , eventtoreset : super::super::Foundation:: HANDLE , eventtypes : *mut u32 ) -> super::super::Foundation:: BOOL );
    AllJoynEnumEvents(connectedbushandle.into_param().abi(), eventtoreset.into_param().abi(), eventtypes)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllJoynEventSelect<P0, P1>(connectedbushandle: P0, eventhandle: P1, eventtypes: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn AllJoynEventSelect ( connectedbushandle : super::super::Foundation:: HANDLE , eventhandle : super::super::Foundation:: HANDLE , eventtypes : u32 ) -> super::super::Foundation:: BOOL );
    AllJoynEventSelect(connectedbushandle.into_param().abi(), eventhandle.into_param().abi(), eventtypes)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllJoynReceiveFromBus<P0>(connectedbushandle: P0, buffer: ::core::option::Option<*mut ::core::ffi::c_void>, bytestoread: u32, bytestransferred: ::core::option::Option<*mut u32>, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn AllJoynReceiveFromBus ( connectedbushandle : super::super::Foundation:: HANDLE , buffer : *mut ::core::ffi::c_void , bytestoread : u32 , bytestransferred : *mut u32 , reserved : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    AllJoynReceiveFromBus(connectedbushandle.into_param().abi(), ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), bytestoread, ::core::mem::transmute(bytestransferred.unwrap_or(::std::ptr::null_mut())), reserved)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllJoynSendToBus<P0>(connectedbushandle: P0, buffer: ::core::option::Option<*const ::core::ffi::c_void>, bytestowrite: u32, bytestransferred: ::core::option::Option<*mut u32>, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn AllJoynSendToBus ( connectedbushandle : super::super::Foundation:: HANDLE , buffer : *const ::core::ffi::c_void , bytestowrite : u32 , bytestransferred : *mut u32 , reserved : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    AllJoynSendToBus(connectedbushandle.into_param().abi(), ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null())), bytestowrite, ::core::mem::transmute(bytestransferred.unwrap_or(::std::ptr::null_mut())), reserved)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn QCC_StatusText(status: QStatus) -> ::windows::core::PCSTR {
    ::windows_targets::link ! ( "msajapi.dll""system" fn QCC_StatusText ( status : QStatus ) -> ::windows::core::PCSTR );
    QCC_StatusText(status)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_create<P0>(defaultlanguage: P0) -> alljoyn_aboutdata
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_create ( defaultlanguage : ::windows::core::PCSTR ) -> alljoyn_aboutdata );
    alljoyn_aboutdata_create(defaultlanguage.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_create_empty() -> alljoyn_aboutdata {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_create_empty ( ) -> alljoyn_aboutdata );
    alljoyn_aboutdata_create_empty()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_create_full<P0, P1>(arg: P0, language: P1) -> alljoyn_aboutdata
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_create_full ( arg : alljoyn_msgarg , language : ::windows::core::PCSTR ) -> alljoyn_aboutdata );
    alljoyn_aboutdata_create_full(arg.into_param().abi(), language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_createfrommsgarg<P0, P1, P2>(data: P0, arg: P1, language: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_createfrommsgarg ( data : alljoyn_aboutdata , arg : alljoyn_msgarg , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_createfrommsgarg(data.into_param().abi(), arg.into_param().abi(), language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_createfromxml<P0, P1>(data: P0, aboutdataxml: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_createfromxml ( data : alljoyn_aboutdata , aboutdataxml : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_createfromxml(data.into_param().abi(), aboutdataxml.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_destroy<P0>(data: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_destroy ( data : alljoyn_aboutdata ) -> ( ) );
    alljoyn_aboutdata_destroy(data.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getaboutdata<P0, P1, P2>(data: P0, msgarg: P1, language: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getaboutdata ( data : alljoyn_aboutdata , msgarg : alljoyn_msgarg , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_getaboutdata(data.into_param().abi(), msgarg.into_param().abi(), language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getajsoftwareversion<P0>(data: P0, ajsoftwareversion: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getajsoftwareversion ( data : alljoyn_aboutdata , ajsoftwareversion : *mut *mut i8 ) -> QStatus );
    alljoyn_aboutdata_getajsoftwareversion(data.into_param().abi(), ajsoftwareversion)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getannouncedaboutdata<P0, P1>(data: P0, msgarg: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getannouncedaboutdata ( data : alljoyn_aboutdata , msgarg : alljoyn_msgarg ) -> QStatus );
    alljoyn_aboutdata_getannouncedaboutdata(data.into_param().abi(), msgarg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getappid<P0>(data: P0, appid: *mut *mut u8, num: *mut usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getappid ( data : alljoyn_aboutdata , appid : *mut *mut u8 , num : *mut usize ) -> QStatus );
    alljoyn_aboutdata_getappid(data.into_param().abi(), appid, num)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getappname<P0, P1>(data: P0, appname: *mut *mut i8, language: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getappname ( data : alljoyn_aboutdata , appname : *mut *mut i8 , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_getappname(data.into_param().abi(), appname, language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getdateofmanufacture<P0>(data: P0, dateofmanufacture: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getdateofmanufacture ( data : alljoyn_aboutdata , dateofmanufacture : *mut *mut i8 ) -> QStatus );
    alljoyn_aboutdata_getdateofmanufacture(data.into_param().abi(), dateofmanufacture)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getdefaultlanguage<P0>(data: P0, defaultlanguage: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getdefaultlanguage ( data : alljoyn_aboutdata , defaultlanguage : *mut *mut i8 ) -> QStatus );
    alljoyn_aboutdata_getdefaultlanguage(data.into_param().abi(), defaultlanguage)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getdescription<P0, P1>(data: P0, description: *mut *mut i8, language: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getdescription ( data : alljoyn_aboutdata , description : *mut *mut i8 , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_getdescription(data.into_param().abi(), description, language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getdeviceid<P0>(data: P0, deviceid: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getdeviceid ( data : alljoyn_aboutdata , deviceid : *mut *mut i8 ) -> QStatus );
    alljoyn_aboutdata_getdeviceid(data.into_param().abi(), deviceid)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getdevicename<P0, P1>(data: P0, devicename: *mut *mut i8, language: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getdevicename ( data : alljoyn_aboutdata , devicename : *mut *mut i8 , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_getdevicename(data.into_param().abi(), devicename, language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getfield<P0, P1, P2>(data: P0, name: P1, value: *mut alljoyn_msgarg, language: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getfield ( data : alljoyn_aboutdata , name : ::windows::core::PCSTR , value : *mut alljoyn_msgarg , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_getfield(data.into_param().abi(), name.into_param().abi(), value, language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getfields<P0>(data: P0, fields: *const *const i8, num_fields: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getfields ( data : alljoyn_aboutdata , fields : *const *const i8 , num_fields : usize ) -> usize );
    alljoyn_aboutdata_getfields(data.into_param().abi(), fields, num_fields)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getfieldsignature<P0, P1>(data: P0, fieldname: P1) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getfieldsignature ( data : alljoyn_aboutdata , fieldname : ::windows::core::PCSTR ) -> ::windows::core::PCSTR );
    alljoyn_aboutdata_getfieldsignature(data.into_param().abi(), fieldname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_gethardwareversion<P0>(data: P0, hardwareversion: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_gethardwareversion ( data : alljoyn_aboutdata , hardwareversion : *mut *mut i8 ) -> QStatus );
    alljoyn_aboutdata_gethardwareversion(data.into_param().abi(), hardwareversion)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getmanufacturer<P0, P1>(data: P0, manufacturer: *mut *mut i8, language: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getmanufacturer ( data : alljoyn_aboutdata , manufacturer : *mut *mut i8 , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_getmanufacturer(data.into_param().abi(), manufacturer, language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getmodelnumber<P0>(data: P0, modelnumber: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getmodelnumber ( data : alljoyn_aboutdata , modelnumber : *mut *mut i8 ) -> QStatus );
    alljoyn_aboutdata_getmodelnumber(data.into_param().abi(), modelnumber)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getsoftwareversion<P0>(data: P0, softwareversion: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getsoftwareversion ( data : alljoyn_aboutdata , softwareversion : *mut *mut i8 ) -> QStatus );
    alljoyn_aboutdata_getsoftwareversion(data.into_param().abi(), softwareversion)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getsupportedlanguages<P0>(data: P0, languagetags: *const *const i8, num: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getsupportedlanguages ( data : alljoyn_aboutdata , languagetags : *const *const i8 , num : usize ) -> usize );
    alljoyn_aboutdata_getsupportedlanguages(data.into_param().abi(), languagetags, num)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_getsupporturl<P0>(data: P0, supporturl: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_getsupporturl ( data : alljoyn_aboutdata , supporturl : *mut *mut i8 ) -> QStatus );
    alljoyn_aboutdata_getsupporturl(data.into_param().abi(), supporturl)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_isfieldannounced<P0, P1>(data: P0, fieldname: P1) -> u8
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_isfieldannounced ( data : alljoyn_aboutdata , fieldname : ::windows::core::PCSTR ) -> u8 );
    alljoyn_aboutdata_isfieldannounced(data.into_param().abi(), fieldname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_isfieldlocalized<P0, P1>(data: P0, fieldname: P1) -> u8
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_isfieldlocalized ( data : alljoyn_aboutdata , fieldname : ::windows::core::PCSTR ) -> u8 );
    alljoyn_aboutdata_isfieldlocalized(data.into_param().abi(), fieldname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_isfieldrequired<P0, P1>(data: P0, fieldname: P1) -> u8
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_isfieldrequired ( data : alljoyn_aboutdata , fieldname : ::windows::core::PCSTR ) -> u8 );
    alljoyn_aboutdata_isfieldrequired(data.into_param().abi(), fieldname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_isvalid<P0, P1>(data: P0, language: P1) -> u8
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_isvalid ( data : alljoyn_aboutdata , language : ::windows::core::PCSTR ) -> u8 );
    alljoyn_aboutdata_isvalid(data.into_param().abi(), language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setappid<P0>(data: P0, appid: *const u8, num: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setappid ( data : alljoyn_aboutdata , appid : *const u8 , num : usize ) -> QStatus );
    alljoyn_aboutdata_setappid(data.into_param().abi(), appid, num)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setappid_fromstring<P0, P1>(data: P0, appid: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setappid_fromstring ( data : alljoyn_aboutdata , appid : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setappid_fromstring(data.into_param().abi(), appid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setappname<P0, P1, P2>(data: P0, appname: P1, language: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setappname ( data : alljoyn_aboutdata , appname : ::windows::core::PCSTR , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setappname(data.into_param().abi(), appname.into_param().abi(), language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setdateofmanufacture<P0, P1>(data: P0, dateofmanufacture: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setdateofmanufacture ( data : alljoyn_aboutdata , dateofmanufacture : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setdateofmanufacture(data.into_param().abi(), dateofmanufacture.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setdefaultlanguage<P0, P1>(data: P0, defaultlanguage: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setdefaultlanguage ( data : alljoyn_aboutdata , defaultlanguage : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setdefaultlanguage(data.into_param().abi(), defaultlanguage.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setdescription<P0, P1, P2>(data: P0, description: P1, language: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setdescription ( data : alljoyn_aboutdata , description : ::windows::core::PCSTR , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setdescription(data.into_param().abi(), description.into_param().abi(), language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setdeviceid<P0, P1>(data: P0, deviceid: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setdeviceid ( data : alljoyn_aboutdata , deviceid : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setdeviceid(data.into_param().abi(), deviceid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setdevicename<P0, P1, P2>(data: P0, devicename: P1, language: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setdevicename ( data : alljoyn_aboutdata , devicename : ::windows::core::PCSTR , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setdevicename(data.into_param().abi(), devicename.into_param().abi(), language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setfield<P0, P1, P2, P3>(data: P0, name: P1, value: P2, language: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<alljoyn_msgarg>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setfield ( data : alljoyn_aboutdata , name : ::windows::core::PCSTR , value : alljoyn_msgarg , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setfield(data.into_param().abi(), name.into_param().abi(), value.into_param().abi(), language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_sethardwareversion<P0, P1>(data: P0, hardwareversion: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_sethardwareversion ( data : alljoyn_aboutdata , hardwareversion : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_sethardwareversion(data.into_param().abi(), hardwareversion.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setmanufacturer<P0, P1, P2>(data: P0, manufacturer: P1, language: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setmanufacturer ( data : alljoyn_aboutdata , manufacturer : ::windows::core::PCSTR , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setmanufacturer(data.into_param().abi(), manufacturer.into_param().abi(), language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setmodelnumber<P0, P1>(data: P0, modelnumber: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setmodelnumber ( data : alljoyn_aboutdata , modelnumber : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setmodelnumber(data.into_param().abi(), modelnumber.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setsoftwareversion<P0, P1>(data: P0, softwareversion: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setsoftwareversion ( data : alljoyn_aboutdata , softwareversion : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setsoftwareversion(data.into_param().abi(), softwareversion.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setsupportedlanguage<P0, P1>(data: P0, language: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setsupportedlanguage ( data : alljoyn_aboutdata , language : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setsupportedlanguage(data.into_param().abi(), language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdata_setsupporturl<P0, P1>(data: P0, supporturl: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdata>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdata_setsupporturl ( data : alljoyn_aboutdata , supporturl : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_aboutdata_setsupporturl(data.into_param().abi(), supporturl.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdatalistener_create(callbacks: *const alljoyn_aboutdatalistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_aboutdatalistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdatalistener_create ( callbacks : *const alljoyn_aboutdatalistener_callbacks , context : *const ::core::ffi::c_void ) -> alljoyn_aboutdatalistener );
    alljoyn_aboutdatalistener_create(callbacks, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutdatalistener_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_aboutdatalistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutdatalistener_destroy ( listener : alljoyn_aboutdatalistener ) -> ( ) );
    alljoyn_aboutdatalistener_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticon_clear(icon: *mut _alljoyn_abouticon_handle) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticon_clear ( icon : *mut _alljoyn_abouticon_handle ) -> ( ) );
    alljoyn_abouticon_clear(icon)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticon_create() -> *mut _alljoyn_abouticon_handle {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticon_create ( ) -> *mut _alljoyn_abouticon_handle );
    alljoyn_abouticon_create()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticon_destroy(icon: *mut _alljoyn_abouticon_handle) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticon_destroy ( icon : *mut _alljoyn_abouticon_handle ) -> ( ) );
    alljoyn_abouticon_destroy(icon)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticon_getcontent(icon: *mut _alljoyn_abouticon_handle, data: *const *const u8, size: *mut usize) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticon_getcontent ( icon : *mut _alljoyn_abouticon_handle , data : *const *const u8 , size : *mut usize ) -> ( ) );
    alljoyn_abouticon_getcontent(icon, data, size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticon_geturl(icon: *mut _alljoyn_abouticon_handle, r#type: *const *const i8, url: *const *const i8) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticon_geturl ( icon : *mut _alljoyn_abouticon_handle , r#type : *const *const i8 , url : *const *const i8 ) -> ( ) );
    alljoyn_abouticon_geturl(icon, r#type, url)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticon_setcontent<P0>(icon: *mut _alljoyn_abouticon_handle, r#type: P0, data: *mut u8, csize: usize, ownsdata: u8) -> QStatus
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticon_setcontent ( icon : *mut _alljoyn_abouticon_handle , r#type : ::windows::core::PCSTR , data : *mut u8 , csize : usize , ownsdata : u8 ) -> QStatus );
    alljoyn_abouticon_setcontent(icon, r#type.into_param().abi(), data, csize, ownsdata)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticon_setcontent_frommsgarg<P0>(icon: *mut _alljoyn_abouticon_handle, arg: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticon_setcontent_frommsgarg ( icon : *mut _alljoyn_abouticon_handle , arg : alljoyn_msgarg ) -> QStatus );
    alljoyn_abouticon_setcontent_frommsgarg(icon, arg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticon_seturl<P0, P1>(icon: *mut _alljoyn_abouticon_handle, r#type: P0, url: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticon_seturl ( icon : *mut _alljoyn_abouticon_handle , r#type : ::windows::core::PCSTR , url : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_abouticon_seturl(icon, r#type.into_param().abi(), url.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticonobj_create<P0>(bus: P0, icon: *mut _alljoyn_abouticon_handle) -> *mut _alljoyn_abouticonobj_handle
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticonobj_create ( bus : alljoyn_busattachment , icon : *mut _alljoyn_abouticon_handle ) -> *mut _alljoyn_abouticonobj_handle );
    alljoyn_abouticonobj_create(bus.into_param().abi(), icon)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticonobj_destroy(icon: *mut _alljoyn_abouticonobj_handle) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticonobj_destroy ( icon : *mut _alljoyn_abouticonobj_handle ) -> ( ) );
    alljoyn_abouticonobj_destroy(icon)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticonproxy_create<P0, P1>(bus: P0, busname: P1, sessionid: u32) -> *mut _alljoyn_abouticonproxy_handle
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticonproxy_create ( bus : alljoyn_busattachment , busname : ::windows::core::PCSTR , sessionid : u32 ) -> *mut _alljoyn_abouticonproxy_handle );
    alljoyn_abouticonproxy_create(bus.into_param().abi(), busname.into_param().abi(), sessionid)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticonproxy_destroy(proxy: *mut _alljoyn_abouticonproxy_handle) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticonproxy_destroy ( proxy : *mut _alljoyn_abouticonproxy_handle ) -> ( ) );
    alljoyn_abouticonproxy_destroy(proxy)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticonproxy_geticon(proxy: *mut _alljoyn_abouticonproxy_handle, icon: *mut _alljoyn_abouticon_handle) -> QStatus {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticonproxy_geticon ( proxy : *mut _alljoyn_abouticonproxy_handle , icon : *mut _alljoyn_abouticon_handle ) -> QStatus );
    alljoyn_abouticonproxy_geticon(proxy, icon)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_abouticonproxy_getversion(proxy: *mut _alljoyn_abouticonproxy_handle, version: *mut u16) -> QStatus {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_abouticonproxy_getversion ( proxy : *mut _alljoyn_abouticonproxy_handle , version : *mut u16 ) -> QStatus );
    alljoyn_abouticonproxy_getversion(proxy, version)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutlistener_create(callback: *const alljoyn_aboutlistener_callback, context: *const ::core::ffi::c_void) -> alljoyn_aboutlistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutlistener_create ( callback : *const alljoyn_aboutlistener_callback , context : *const ::core::ffi::c_void ) -> alljoyn_aboutlistener );
    alljoyn_aboutlistener_create(callback, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutlistener_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_aboutlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutlistener_destroy ( listener : alljoyn_aboutlistener ) -> ( ) );
    alljoyn_aboutlistener_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobj_announce<P0, P1>(obj: P0, sessionport: u16, aboutdata: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobj>,
    P1: ::windows::core::IntoParam<alljoyn_aboutdata>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobj_announce ( obj : alljoyn_aboutobj , sessionport : u16 , aboutdata : alljoyn_aboutdata ) -> QStatus );
    alljoyn_aboutobj_announce(obj.into_param().abi(), sessionport, aboutdata.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobj_announce_using_datalistener<P0, P1>(obj: P0, sessionport: u16, aboutlistener: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobj>,
    P1: ::windows::core::IntoParam<alljoyn_aboutdatalistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobj_announce_using_datalistener ( obj : alljoyn_aboutobj , sessionport : u16 , aboutlistener : alljoyn_aboutdatalistener ) -> QStatus );
    alljoyn_aboutobj_announce_using_datalistener(obj.into_param().abi(), sessionport, aboutlistener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobj_create<P0>(bus: P0, isannounced: alljoyn_about_announceflag) -> alljoyn_aboutobj
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobj_create ( bus : alljoyn_busattachment , isannounced : alljoyn_about_announceflag ) -> alljoyn_aboutobj );
    alljoyn_aboutobj_create(bus.into_param().abi(), isannounced)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobj_destroy<P0>(obj: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobj>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobj_destroy ( obj : alljoyn_aboutobj ) -> ( ) );
    alljoyn_aboutobj_destroy(obj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobj_unannounce<P0>(obj: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobj>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobj_unannounce ( obj : alljoyn_aboutobj ) -> QStatus );
    alljoyn_aboutobj_unannounce(obj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_clear<P0>(description: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobjectdescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_clear ( description : alljoyn_aboutobjectdescription ) -> ( ) );
    alljoyn_aboutobjectdescription_clear(description.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_create() -> alljoyn_aboutobjectdescription {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_create ( ) -> alljoyn_aboutobjectdescription );
    alljoyn_aboutobjectdescription_create()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_create_full<P0>(arg: P0) -> alljoyn_aboutobjectdescription
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_create_full ( arg : alljoyn_msgarg ) -> alljoyn_aboutobjectdescription );
    alljoyn_aboutobjectdescription_create_full(arg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_createfrommsgarg<P0, P1>(description: P0, arg: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobjectdescription>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_createfrommsgarg ( description : alljoyn_aboutobjectdescription , arg : alljoyn_msgarg ) -> QStatus );
    alljoyn_aboutobjectdescription_createfrommsgarg(description.into_param().abi(), arg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_destroy<P0>(description: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobjectdescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_destroy ( description : alljoyn_aboutobjectdescription ) -> ( ) );
    alljoyn_aboutobjectdescription_destroy(description.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_getinterfacepaths<P0, P1>(description: P0, interfacename: P1, paths: *const *const i8, numpaths: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobjectdescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_getinterfacepaths ( description : alljoyn_aboutobjectdescription , interfacename : ::windows::core::PCSTR , paths : *const *const i8 , numpaths : usize ) -> usize );
    alljoyn_aboutobjectdescription_getinterfacepaths(description.into_param().abi(), interfacename.into_param().abi(), paths, numpaths)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_getinterfaces<P0, P1>(description: P0, path: P1, interfaces: *const *const i8, numinterfaces: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobjectdescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_getinterfaces ( description : alljoyn_aboutobjectdescription , path : ::windows::core::PCSTR , interfaces : *const *const i8 , numinterfaces : usize ) -> usize );
    alljoyn_aboutobjectdescription_getinterfaces(description.into_param().abi(), path.into_param().abi(), interfaces, numinterfaces)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_getmsgarg<P0, P1>(description: P0, msgarg: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobjectdescription>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_getmsgarg ( description : alljoyn_aboutobjectdescription , msgarg : alljoyn_msgarg ) -> QStatus );
    alljoyn_aboutobjectdescription_getmsgarg(description.into_param().abi(), msgarg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_getpaths<P0>(description: P0, paths: *const *const i8, numpaths: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobjectdescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_getpaths ( description : alljoyn_aboutobjectdescription , paths : *const *const i8 , numpaths : usize ) -> usize );
    alljoyn_aboutobjectdescription_getpaths(description.into_param().abi(), paths, numpaths)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_hasinterface<P0, P1>(description: P0, interfacename: P1) -> u8
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobjectdescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_hasinterface ( description : alljoyn_aboutobjectdescription , interfacename : ::windows::core::PCSTR ) -> u8 );
    alljoyn_aboutobjectdescription_hasinterface(description.into_param().abi(), interfacename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_hasinterfaceatpath<P0, P1, P2>(description: P0, path: P1, interfacename: P2) -> u8
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobjectdescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_hasinterfaceatpath ( description : alljoyn_aboutobjectdescription , path : ::windows::core::PCSTR , interfacename : ::windows::core::PCSTR ) -> u8 );
    alljoyn_aboutobjectdescription_hasinterfaceatpath(description.into_param().abi(), path.into_param().abi(), interfacename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_haspath<P0, P1>(description: P0, path: P1) -> u8
where
    P0: ::windows::core::IntoParam<alljoyn_aboutobjectdescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutobjectdescription_haspath ( description : alljoyn_aboutobjectdescription , path : ::windows::core::PCSTR ) -> u8 );
    alljoyn_aboutobjectdescription_haspath(description.into_param().abi(), path.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutproxy_create<P0, P1>(bus: P0, busname: P1, sessionid: u32) -> alljoyn_aboutproxy
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutproxy_create ( bus : alljoyn_busattachment , busname : ::windows::core::PCSTR , sessionid : u32 ) -> alljoyn_aboutproxy );
    alljoyn_aboutproxy_create(bus.into_param().abi(), busname.into_param().abi(), sessionid)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutproxy_destroy<P0>(proxy: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_aboutproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutproxy_destroy ( proxy : alljoyn_aboutproxy ) -> ( ) );
    alljoyn_aboutproxy_destroy(proxy.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutproxy_getaboutdata<P0, P1, P2>(proxy: P0, language: P1, data: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutproxy>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutproxy_getaboutdata ( proxy : alljoyn_aboutproxy , language : ::windows::core::PCSTR , data : alljoyn_msgarg ) -> QStatus );
    alljoyn_aboutproxy_getaboutdata(proxy.into_param().abi(), language.into_param().abi(), data.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutproxy_getobjectdescription<P0, P1>(proxy: P0, objectdesc: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutproxy>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutproxy_getobjectdescription ( proxy : alljoyn_aboutproxy , objectdesc : alljoyn_msgarg ) -> QStatus );
    alljoyn_aboutproxy_getobjectdescription(proxy.into_param().abi(), objectdesc.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_aboutproxy_getversion<P0>(proxy: P0, version: *mut u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_aboutproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_aboutproxy_getversion ( proxy : alljoyn_aboutproxy , version : *mut u16 ) -> QStatus );
    alljoyn_aboutproxy_getversion(proxy.into_param().abi(), version)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_applicationstatelistener_create(callbacks: *const alljoyn_applicationstatelistener_callbacks, context: *mut ::core::ffi::c_void) -> alljoyn_applicationstatelistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_applicationstatelistener_create ( callbacks : *const alljoyn_applicationstatelistener_callbacks , context : *mut ::core::ffi::c_void ) -> alljoyn_applicationstatelistener );
    alljoyn_applicationstatelistener_create(callbacks, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_applicationstatelistener_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_applicationstatelistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_applicationstatelistener_destroy ( listener : alljoyn_applicationstatelistener ) -> ( ) );
    alljoyn_applicationstatelistener_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_authlistener_create(callbacks: *const alljoyn_authlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_authlistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_authlistener_create ( callbacks : *const alljoyn_authlistener_callbacks , context : *const ::core::ffi::c_void ) -> alljoyn_authlistener );
    alljoyn_authlistener_create(callbacks, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_authlistener_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_authlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_authlistener_destroy ( listener : alljoyn_authlistener ) -> ( ) );
    alljoyn_authlistener_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_authlistener_requestcredentialsresponse<P0, P1>(listener: P0, authcontext: *mut ::core::ffi::c_void, accept: i32, credentials: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_authlistener>,
    P1: ::windows::core::IntoParam<alljoyn_credentials>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_authlistener_requestcredentialsresponse ( listener : alljoyn_authlistener , authcontext : *mut ::core::ffi::c_void , accept : i32 , credentials : alljoyn_credentials ) -> QStatus );
    alljoyn_authlistener_requestcredentialsresponse(listener.into_param().abi(), authcontext, accept, credentials.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_authlistener_setsharedsecret<P0>(listener: P0, sharedsecret: *const u8, sharedsecretsize: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_authlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_authlistener_setsharedsecret ( listener : alljoyn_authlistener , sharedsecret : *const u8 , sharedsecretsize : usize ) -> QStatus );
    alljoyn_authlistener_setsharedsecret(listener.into_param().abi(), sharedsecret, sharedsecretsize)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_authlistener_verifycredentialsresponse<P0>(listener: P0, authcontext: *mut ::core::ffi::c_void, accept: i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_authlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_authlistener_verifycredentialsresponse ( listener : alljoyn_authlistener , authcontext : *mut ::core::ffi::c_void , accept : i32 ) -> QStatus );
    alljoyn_authlistener_verifycredentialsresponse(listener.into_param().abi(), authcontext, accept)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_authlistenerasync_create(callbacks: *const alljoyn_authlistenerasync_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_authlistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_authlistenerasync_create ( callbacks : *const alljoyn_authlistenerasync_callbacks , context : *const ::core::ffi::c_void ) -> alljoyn_authlistener );
    alljoyn_authlistenerasync_create(callbacks, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_authlistenerasync_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_authlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_authlistenerasync_destroy ( listener : alljoyn_authlistener ) -> ( ) );
    alljoyn_authlistenerasync_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_autopinger_adddestination<P0, P1, P2>(autopinger: P0, group: P1, destination: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_autopinger>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_autopinger_adddestination ( autopinger : alljoyn_autopinger , group : ::windows::core::PCSTR , destination : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_autopinger_adddestination(autopinger.into_param().abi(), group.into_param().abi(), destination.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_autopinger_addpinggroup<P0, P1, P2>(autopinger: P0, group: P1, listener: P2, pinginterval: u32)
where
    P0: ::windows::core::IntoParam<alljoyn_autopinger>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<alljoyn_pinglistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_autopinger_addpinggroup ( autopinger : alljoyn_autopinger , group : ::windows::core::PCSTR , listener : alljoyn_pinglistener , pinginterval : u32 ) -> ( ) );
    alljoyn_autopinger_addpinggroup(autopinger.into_param().abi(), group.into_param().abi(), listener.into_param().abi(), pinginterval)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_autopinger_create<P0>(bus: P0) -> alljoyn_autopinger
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_autopinger_create ( bus : alljoyn_busattachment ) -> alljoyn_autopinger );
    alljoyn_autopinger_create(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_autopinger_destroy<P0>(autopinger: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_autopinger>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_autopinger_destroy ( autopinger : alljoyn_autopinger ) -> ( ) );
    alljoyn_autopinger_destroy(autopinger.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_autopinger_pause<P0>(autopinger: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_autopinger>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_autopinger_pause ( autopinger : alljoyn_autopinger ) -> ( ) );
    alljoyn_autopinger_pause(autopinger.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_autopinger_removedestination<P0, P1, P2>(autopinger: P0, group: P1, destination: P2, removeall: i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_autopinger>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_autopinger_removedestination ( autopinger : alljoyn_autopinger , group : ::windows::core::PCSTR , destination : ::windows::core::PCSTR , removeall : i32 ) -> QStatus );
    alljoyn_autopinger_removedestination(autopinger.into_param().abi(), group.into_param().abi(), destination.into_param().abi(), removeall)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_autopinger_removepinggroup<P0, P1>(autopinger: P0, group: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_autopinger>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_autopinger_removepinggroup ( autopinger : alljoyn_autopinger , group : ::windows::core::PCSTR ) -> ( ) );
    alljoyn_autopinger_removepinggroup(autopinger.into_param().abi(), group.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_autopinger_resume<P0>(autopinger: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_autopinger>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_autopinger_resume ( autopinger : alljoyn_autopinger ) -> ( ) );
    alljoyn_autopinger_resume(autopinger.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_autopinger_setpinginterval<P0, P1>(autopinger: P0, group: P1, pinginterval: u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_autopinger>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_autopinger_setpinginterval ( autopinger : alljoyn_autopinger , group : ::windows::core::PCSTR , pinginterval : u32 ) -> QStatus );
    alljoyn_autopinger_setpinginterval(autopinger.into_param().abi(), group.into_param().abi(), pinginterval)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_addlogonentry<P0, P1, P2, P3>(bus: P0, authmechanism: P1, username: P2, password: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_addlogonentry ( bus : alljoyn_busattachment , authmechanism : ::windows::core::PCSTR , username : ::windows::core::PCSTR , password : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_addlogonentry(bus.into_param().abi(), authmechanism.into_param().abi(), username.into_param().abi(), password.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_addmatch<P0, P1>(bus: P0, rule: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_addmatch ( bus : alljoyn_busattachment , rule : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_addmatch(bus.into_param().abi(), rule.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_advertisename<P0, P1>(bus: P0, name: P1, transports: u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_advertisename ( bus : alljoyn_busattachment , name : ::windows::core::PCSTR , transports : u16 ) -> QStatus );
    alljoyn_busattachment_advertisename(bus.into_param().abi(), name.into_param().abi(), transports)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_bindsessionport<P0, P1, P2>(bus: P0, sessionport: *mut u16, opts: P1, listener: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_sessionopts>,
    P2: ::windows::core::IntoParam<alljoyn_sessionportlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_bindsessionport ( bus : alljoyn_busattachment , sessionport : *mut u16 , opts : alljoyn_sessionopts , listener : alljoyn_sessionportlistener ) -> QStatus );
    alljoyn_busattachment_bindsessionport(bus.into_param().abi(), sessionport, opts.into_param().abi(), listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_canceladvertisename<P0, P1>(bus: P0, name: P1, transports: u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_canceladvertisename ( bus : alljoyn_busattachment , name : ::windows::core::PCSTR , transports : u16 ) -> QStatus );
    alljoyn_busattachment_canceladvertisename(bus.into_param().abi(), name.into_param().abi(), transports)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_cancelfindadvertisedname<P0, P1>(bus: P0, nameprefix: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_cancelfindadvertisedname ( bus : alljoyn_busattachment , nameprefix : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_cancelfindadvertisedname(bus.into_param().abi(), nameprefix.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_cancelfindadvertisednamebytransport<P0, P1>(bus: P0, nameprefix: P1, transports: u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_cancelfindadvertisednamebytransport ( bus : alljoyn_busattachment , nameprefix : ::windows::core::PCSTR , transports : u16 ) -> QStatus );
    alljoyn_busattachment_cancelfindadvertisednamebytransport(bus.into_param().abi(), nameprefix.into_param().abi(), transports)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_cancelwhoimplements_interface<P0, P1>(bus: P0, implementsinterface: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_cancelwhoimplements_interface ( bus : alljoyn_busattachment , implementsinterface : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_cancelwhoimplements_interface(bus.into_param().abi(), implementsinterface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_cancelwhoimplements_interfaces<P0>(bus: P0, implementsinterfaces: *const *const i8, numberinterfaces: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_cancelwhoimplements_interfaces ( bus : alljoyn_busattachment , implementsinterfaces : *const *const i8 , numberinterfaces : usize ) -> QStatus );
    alljoyn_busattachment_cancelwhoimplements_interfaces(bus.into_param().abi(), implementsinterfaces, numberinterfaces)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_clearkeys<P0, P1>(bus: P0, guid: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_clearkeys ( bus : alljoyn_busattachment , guid : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_clearkeys(bus.into_param().abi(), guid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_clearkeystore<P0>(bus: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_clearkeystore ( bus : alljoyn_busattachment ) -> ( ) );
    alljoyn_busattachment_clearkeystore(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_connect<P0, P1>(bus: P0, connectspec: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_connect ( bus : alljoyn_busattachment , connectspec : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_connect(bus.into_param().abi(), connectspec.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_create<P0>(applicationname: P0, allowremotemessages: i32) -> alljoyn_busattachment
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_create ( applicationname : ::windows::core::PCSTR , allowremotemessages : i32 ) -> alljoyn_busattachment );
    alljoyn_busattachment_create(applicationname.into_param().abi(), allowremotemessages)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_create_concurrency<P0>(applicationname: P0, allowremotemessages: i32, concurrency: u32) -> alljoyn_busattachment
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_create_concurrency ( applicationname : ::windows::core::PCSTR , allowremotemessages : i32 , concurrency : u32 ) -> alljoyn_busattachment );
    alljoyn_busattachment_create_concurrency(applicationname.into_param().abi(), allowremotemessages, concurrency)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_createinterface<P0, P1>(bus: P0, name: P1, iface: *mut alljoyn_interfacedescription) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_createinterface ( bus : alljoyn_busattachment , name : ::windows::core::PCSTR , iface : *mut alljoyn_interfacedescription ) -> QStatus );
    alljoyn_busattachment_createinterface(bus.into_param().abi(), name.into_param().abi(), iface)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_createinterface_secure<P0, P1>(bus: P0, name: P1, iface: *mut alljoyn_interfacedescription, secpolicy: alljoyn_interfacedescription_securitypolicy) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_createinterface_secure ( bus : alljoyn_busattachment , name : ::windows::core::PCSTR , iface : *mut alljoyn_interfacedescription , secpolicy : alljoyn_interfacedescription_securitypolicy ) -> QStatus );
    alljoyn_busattachment_createinterface_secure(bus.into_param().abi(), name.into_param().abi(), iface, secpolicy)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_createinterfacesfromxml<P0, P1>(bus: P0, xml: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_createinterfacesfromxml ( bus : alljoyn_busattachment , xml : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_createinterfacesfromxml(bus.into_param().abi(), xml.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_deletedefaultkeystore<P0>(applicationname: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_deletedefaultkeystore ( applicationname : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_deletedefaultkeystore(applicationname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_deleteinterface<P0, P1>(bus: P0, iface: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_deleteinterface ( bus : alljoyn_busattachment , iface : alljoyn_interfacedescription ) -> QStatus );
    alljoyn_busattachment_deleteinterface(bus.into_param().abi(), iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_destroy<P0>(bus: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_destroy ( bus : alljoyn_busattachment ) -> ( ) );
    alljoyn_busattachment_destroy(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_disconnect<P0, P1>(bus: P0, unused: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_disconnect ( bus : alljoyn_busattachment , unused : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_disconnect(bus.into_param().abi(), unused.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_enableconcurrentcallbacks<P0>(bus: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_enableconcurrentcallbacks ( bus : alljoyn_busattachment ) -> ( ) );
    alljoyn_busattachment_enableconcurrentcallbacks(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_enablepeersecurity<P0, P1, P2, P3>(bus: P0, authmechanisms: P1, listener: P2, keystorefilename: P3, isshared: i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<alljoyn_authlistener>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_enablepeersecurity ( bus : alljoyn_busattachment , authmechanisms : ::windows::core::PCSTR , listener : alljoyn_authlistener , keystorefilename : ::windows::core::PCSTR , isshared : i32 ) -> QStatus );
    alljoyn_busattachment_enablepeersecurity(bus.into_param().abi(), authmechanisms.into_param().abi(), listener.into_param().abi(), keystorefilename.into_param().abi(), isshared)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_enablepeersecuritywithpermissionconfigurationlistener<P0, P1, P2, P3, P4>(bus: P0, authmechanisms: P1, authlistener: P2, keystorefilename: P3, isshared: i32, permissionconfigurationlistener: P4) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<alljoyn_authlistener>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<alljoyn_permissionconfigurationlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_enablepeersecuritywithpermissionconfigurationlistener ( bus : alljoyn_busattachment , authmechanisms : ::windows::core::PCSTR , authlistener : alljoyn_authlistener , keystorefilename : ::windows::core::PCSTR , isshared : i32 , permissionconfigurationlistener : alljoyn_permissionconfigurationlistener ) -> QStatus );
    alljoyn_busattachment_enablepeersecuritywithpermissionconfigurationlistener(bus.into_param().abi(), authmechanisms.into_param().abi(), authlistener.into_param().abi(), keystorefilename.into_param().abi(), isshared, permissionconfigurationlistener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_findadvertisedname<P0, P1>(bus: P0, nameprefix: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_findadvertisedname ( bus : alljoyn_busattachment , nameprefix : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_findadvertisedname(bus.into_param().abi(), nameprefix.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_findadvertisednamebytransport<P0, P1>(bus: P0, nameprefix: P1, transports: u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_findadvertisednamebytransport ( bus : alljoyn_busattachment , nameprefix : ::windows::core::PCSTR , transports : u16 ) -> QStatus );
    alljoyn_busattachment_findadvertisednamebytransport(bus.into_param().abi(), nameprefix.into_param().abi(), transports)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getalljoyndebugobj<P0>(bus: P0) -> alljoyn_proxybusobject
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getalljoyndebugobj ( bus : alljoyn_busattachment ) -> alljoyn_proxybusobject );
    alljoyn_busattachment_getalljoyndebugobj(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getalljoynproxyobj<P0>(bus: P0) -> alljoyn_proxybusobject
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getalljoynproxyobj ( bus : alljoyn_busattachment ) -> alljoyn_proxybusobject );
    alljoyn_busattachment_getalljoynproxyobj(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getconcurrency<P0>(bus: P0) -> u32
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getconcurrency ( bus : alljoyn_busattachment ) -> u32 );
    alljoyn_busattachment_getconcurrency(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getconnectspec<P0>(bus: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getconnectspec ( bus : alljoyn_busattachment ) -> ::windows::core::PCSTR );
    alljoyn_busattachment_getconnectspec(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getdbusproxyobj<P0>(bus: P0) -> alljoyn_proxybusobject
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getdbusproxyobj ( bus : alljoyn_busattachment ) -> alljoyn_proxybusobject );
    alljoyn_busattachment_getdbusproxyobj(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getglobalguidstring<P0>(bus: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getglobalguidstring ( bus : alljoyn_busattachment ) -> ::windows::core::PCSTR );
    alljoyn_busattachment_getglobalguidstring(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getinterface<P0, P1>(bus: P0, name: P1) -> alljoyn_interfacedescription
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getinterface ( bus : alljoyn_busattachment , name : ::windows::core::PCSTR ) -> alljoyn_interfacedescription );
    alljoyn_busattachment_getinterface(bus.into_param().abi(), name.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getinterfaces<P0>(bus: P0, ifaces: *const alljoyn_interfacedescription, numifaces: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getinterfaces ( bus : alljoyn_busattachment , ifaces : *const alljoyn_interfacedescription , numifaces : usize ) -> usize );
    alljoyn_busattachment_getinterfaces(bus.into_param().abi(), ifaces, numifaces)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getkeyexpiration<P0, P1>(bus: P0, guid: P1, timeout: *mut u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getkeyexpiration ( bus : alljoyn_busattachment , guid : ::windows::core::PCSTR , timeout : *mut u32 ) -> QStatus );
    alljoyn_busattachment_getkeyexpiration(bus.into_param().abi(), guid.into_param().abi(), timeout)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getpeerguid<P0, P1, P2>(bus: P0, name: P1, guid: P2, guidsz: *mut usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getpeerguid ( bus : alljoyn_busattachment , name : ::windows::core::PCSTR , guid : ::windows::core::PCSTR , guidsz : *mut usize ) -> QStatus );
    alljoyn_busattachment_getpeerguid(bus.into_param().abi(), name.into_param().abi(), guid.into_param().abi(), guidsz)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getpermissionconfigurator<P0>(bus: P0) -> alljoyn_permissionconfigurator
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getpermissionconfigurator ( bus : alljoyn_busattachment ) -> alljoyn_permissionconfigurator );
    alljoyn_busattachment_getpermissionconfigurator(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_gettimestamp() -> u32 {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_gettimestamp ( ) -> u32 );
    alljoyn_busattachment_gettimestamp()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_getuniquename<P0>(bus: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_getuniquename ( bus : alljoyn_busattachment ) -> ::windows::core::PCSTR );
    alljoyn_busattachment_getuniquename(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_isconnected<P0>(bus: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_isconnected ( bus : alljoyn_busattachment ) -> i32 );
    alljoyn_busattachment_isconnected(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_ispeersecurityenabled<P0>(bus: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_ispeersecurityenabled ( bus : alljoyn_busattachment ) -> i32 );
    alljoyn_busattachment_ispeersecurityenabled(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_isstarted<P0>(bus: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_isstarted ( bus : alljoyn_busattachment ) -> i32 );
    alljoyn_busattachment_isstarted(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_isstopping<P0>(bus: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_isstopping ( bus : alljoyn_busattachment ) -> i32 );
    alljoyn_busattachment_isstopping(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_join<P0>(bus: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_join ( bus : alljoyn_busattachment ) -> QStatus );
    alljoyn_busattachment_join(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_joinsession<P0, P1, P2, P3>(bus: P0, sessionhost: P1, sessionport: u16, listener: P2, sessionid: *mut u32, opts: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<alljoyn_sessionlistener>,
    P3: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_joinsession ( bus : alljoyn_busattachment , sessionhost : ::windows::core::PCSTR , sessionport : u16 , listener : alljoyn_sessionlistener , sessionid : *mut u32 , opts : alljoyn_sessionopts ) -> QStatus );
    alljoyn_busattachment_joinsession(bus.into_param().abi(), sessionhost.into_param().abi(), sessionport, listener.into_param().abi(), sessionid, opts.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_joinsessionasync<P0, P1, P2, P3>(bus: P0, sessionhost: P1, sessionport: u16, listener: P2, opts: P3, callback: alljoyn_busattachment_joinsessioncb_ptr, context: *mut ::core::ffi::c_void) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<alljoyn_sessionlistener>,
    P3: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_joinsessionasync ( bus : alljoyn_busattachment , sessionhost : ::windows::core::PCSTR , sessionport : u16 , listener : alljoyn_sessionlistener , opts : alljoyn_sessionopts , callback : alljoyn_busattachment_joinsessioncb_ptr , context : *mut ::core::ffi::c_void ) -> QStatus );
    alljoyn_busattachment_joinsessionasync(bus.into_param().abi(), sessionhost.into_param().abi(), sessionport, listener.into_param().abi(), opts.into_param().abi(), callback, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_leavesession<P0>(bus: P0, sessionid: u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_leavesession ( bus : alljoyn_busattachment , sessionid : u32 ) -> QStatus );
    alljoyn_busattachment_leavesession(bus.into_param().abi(), sessionid)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_namehasowner<P0, P1>(bus: P0, name: P1, hasowner: *mut i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_namehasowner ( bus : alljoyn_busattachment , name : ::windows::core::PCSTR , hasowner : *mut i32 ) -> QStatus );
    alljoyn_busattachment_namehasowner(bus.into_param().abi(), name.into_param().abi(), hasowner)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_ping<P0, P1>(bus: P0, name: P1, timeout: u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_ping ( bus : alljoyn_busattachment , name : ::windows::core::PCSTR , timeout : u32 ) -> QStatus );
    alljoyn_busattachment_ping(bus.into_param().abi(), name.into_param().abi(), timeout)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_registeraboutlistener<P0, P1>(bus: P0, aboutlistener: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_aboutlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_registeraboutlistener ( bus : alljoyn_busattachment , aboutlistener : alljoyn_aboutlistener ) -> ( ) );
    alljoyn_busattachment_registeraboutlistener(bus.into_param().abi(), aboutlistener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_registerapplicationstatelistener<P0, P1>(bus: P0, listener: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_applicationstatelistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_registerapplicationstatelistener ( bus : alljoyn_busattachment , listener : alljoyn_applicationstatelistener ) -> QStatus );
    alljoyn_busattachment_registerapplicationstatelistener(bus.into_param().abi(), listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_registerbuslistener<P0, P1>(bus: P0, listener: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_buslistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_registerbuslistener ( bus : alljoyn_busattachment , listener : alljoyn_buslistener ) -> ( ) );
    alljoyn_busattachment_registerbuslistener(bus.into_param().abi(), listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_registerbusobject<P0, P1>(bus: P0, obj: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_busobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_registerbusobject ( bus : alljoyn_busattachment , obj : alljoyn_busobject ) -> QStatus );
    alljoyn_busattachment_registerbusobject(bus.into_param().abi(), obj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_registerbusobject_secure<P0, P1>(bus: P0, obj: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_busobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_registerbusobject_secure ( bus : alljoyn_busattachment , obj : alljoyn_busobject ) -> QStatus );
    alljoyn_busattachment_registerbusobject_secure(bus.into_param().abi(), obj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_registerkeystorelistener<P0, P1>(bus: P0, listener: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_keystorelistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_registerkeystorelistener ( bus : alljoyn_busattachment , listener : alljoyn_keystorelistener ) -> QStatus );
    alljoyn_busattachment_registerkeystorelistener(bus.into_param().abi(), listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_registersignalhandler<P0, P1>(bus: P0, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, srcpath: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_registersignalhandler ( bus : alljoyn_busattachment , signal_handler : alljoyn_messagereceiver_signalhandler_ptr , member : alljoyn_interfacedescription_member , srcpath : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_registersignalhandler(bus.into_param().abi(), signal_handler, ::core::mem::transmute(member), srcpath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_registersignalhandlerwithrule<P0, P1>(bus: P0, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, matchrule: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_registersignalhandlerwithrule ( bus : alljoyn_busattachment , signal_handler : alljoyn_messagereceiver_signalhandler_ptr , member : alljoyn_interfacedescription_member , matchrule : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_registersignalhandlerwithrule(bus.into_param().abi(), signal_handler, ::core::mem::transmute(member), matchrule.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_releasename<P0, P1>(bus: P0, name: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_releasename ( bus : alljoyn_busattachment , name : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_releasename(bus.into_param().abi(), name.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_reloadkeystore<P0>(bus: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_reloadkeystore ( bus : alljoyn_busattachment ) -> QStatus );
    alljoyn_busattachment_reloadkeystore(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_removematch<P0, P1>(bus: P0, rule: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_removematch ( bus : alljoyn_busattachment , rule : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_removematch(bus.into_param().abi(), rule.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_removesessionmember<P0, P1>(bus: P0, sessionid: u32, membername: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_removesessionmember ( bus : alljoyn_busattachment , sessionid : u32 , membername : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_removesessionmember(bus.into_param().abi(), sessionid, membername.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_requestname<P0, P1>(bus: P0, requestedname: P1, flags: u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_requestname ( bus : alljoyn_busattachment , requestedname : ::windows::core::PCSTR , flags : u32 ) -> QStatus );
    alljoyn_busattachment_requestname(bus.into_param().abi(), requestedname.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_secureconnection<P0, P1>(bus: P0, name: P1, forceauth: i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_secureconnection ( bus : alljoyn_busattachment , name : ::windows::core::PCSTR , forceauth : i32 ) -> QStatus );
    alljoyn_busattachment_secureconnection(bus.into_param().abi(), name.into_param().abi(), forceauth)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_secureconnectionasync<P0, P1>(bus: P0, name: P1, forceauth: i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_secureconnectionasync ( bus : alljoyn_busattachment , name : ::windows::core::PCSTR , forceauth : i32 ) -> QStatus );
    alljoyn_busattachment_secureconnectionasync(bus.into_param().abi(), name.into_param().abi(), forceauth)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_setdaemondebug<P0, P1>(bus: P0, module: P1, level: u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_setdaemondebug ( bus : alljoyn_busattachment , module : ::windows::core::PCSTR , level : u32 ) -> QStatus );
    alljoyn_busattachment_setdaemondebug(bus.into_param().abi(), module.into_param().abi(), level)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_setkeyexpiration<P0, P1>(bus: P0, guid: P1, timeout: u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_setkeyexpiration ( bus : alljoyn_busattachment , guid : ::windows::core::PCSTR , timeout : u32 ) -> QStatus );
    alljoyn_busattachment_setkeyexpiration(bus.into_param().abi(), guid.into_param().abi(), timeout)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_setlinktimeout<P0>(bus: P0, sessionid: u32, linktimeout: *mut u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_setlinktimeout ( bus : alljoyn_busattachment , sessionid : u32 , linktimeout : *mut u32 ) -> QStatus );
    alljoyn_busattachment_setlinktimeout(bus.into_param().abi(), sessionid, linktimeout)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_setlinktimeoutasync<P0>(bus: P0, sessionid: u32, linktimeout: u32, callback: alljoyn_busattachment_setlinktimeoutcb_ptr, context: *mut ::core::ffi::c_void) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_setlinktimeoutasync ( bus : alljoyn_busattachment , sessionid : u32 , linktimeout : u32 , callback : alljoyn_busattachment_setlinktimeoutcb_ptr , context : *mut ::core::ffi::c_void ) -> QStatus );
    alljoyn_busattachment_setlinktimeoutasync(bus.into_param().abi(), sessionid, linktimeout, callback, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_setsessionlistener<P0, P1>(bus: P0, sessionid: u32, listener: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_sessionlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_setsessionlistener ( bus : alljoyn_busattachment , sessionid : u32 , listener : alljoyn_sessionlistener ) -> QStatus );
    alljoyn_busattachment_setsessionlistener(bus.into_param().abi(), sessionid, listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_start<P0>(bus: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_start ( bus : alljoyn_busattachment ) -> QStatus );
    alljoyn_busattachment_start(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_stop<P0>(bus: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_stop ( bus : alljoyn_busattachment ) -> QStatus );
    alljoyn_busattachment_stop(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_unbindsessionport<P0>(bus: P0, sessionport: u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_unbindsessionport ( bus : alljoyn_busattachment , sessionport : u16 ) -> QStatus );
    alljoyn_busattachment_unbindsessionport(bus.into_param().abi(), sessionport)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_unregisteraboutlistener<P0, P1>(bus: P0, aboutlistener: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_aboutlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_unregisteraboutlistener ( bus : alljoyn_busattachment , aboutlistener : alljoyn_aboutlistener ) -> ( ) );
    alljoyn_busattachment_unregisteraboutlistener(bus.into_param().abi(), aboutlistener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_unregisterallaboutlisteners<P0>(bus: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_unregisterallaboutlisteners ( bus : alljoyn_busattachment ) -> ( ) );
    alljoyn_busattachment_unregisterallaboutlisteners(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_unregisterallhandlers<P0>(bus: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_unregisterallhandlers ( bus : alljoyn_busattachment ) -> QStatus );
    alljoyn_busattachment_unregisterallhandlers(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_unregisterapplicationstatelistener<P0, P1>(bus: P0, listener: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_applicationstatelistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_unregisterapplicationstatelistener ( bus : alljoyn_busattachment , listener : alljoyn_applicationstatelistener ) -> QStatus );
    alljoyn_busattachment_unregisterapplicationstatelistener(bus.into_param().abi(), listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_unregisterbuslistener<P0, P1>(bus: P0, listener: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_buslistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_unregisterbuslistener ( bus : alljoyn_busattachment , listener : alljoyn_buslistener ) -> ( ) );
    alljoyn_busattachment_unregisterbuslistener(bus.into_param().abi(), listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_unregisterbusobject<P0, P1>(bus: P0, object: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<alljoyn_busobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_unregisterbusobject ( bus : alljoyn_busattachment , object : alljoyn_busobject ) -> ( ) );
    alljoyn_busattachment_unregisterbusobject(bus.into_param().abi(), object.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_unregistersignalhandler<P0, P1>(bus: P0, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, srcpath: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_unregistersignalhandler ( bus : alljoyn_busattachment , signal_handler : alljoyn_messagereceiver_signalhandler_ptr , member : alljoyn_interfacedescription_member , srcpath : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_unregistersignalhandler(bus.into_param().abi(), signal_handler, ::core::mem::transmute(member), srcpath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_unregistersignalhandlerwithrule<P0, P1>(bus: P0, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, matchrule: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_unregistersignalhandlerwithrule ( bus : alljoyn_busattachment , signal_handler : alljoyn_messagereceiver_signalhandler_ptr , member : alljoyn_interfacedescription_member , matchrule : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_unregistersignalhandlerwithrule(bus.into_param().abi(), signal_handler, ::core::mem::transmute(member), matchrule.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_whoimplements_interface<P0, P1>(bus: P0, implementsinterface: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_whoimplements_interface ( bus : alljoyn_busattachment , implementsinterface : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busattachment_whoimplements_interface(bus.into_param().abi(), implementsinterface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busattachment_whoimplements_interfaces<P0>(bus: P0, implementsinterfaces: *const *const i8, numberinterfaces: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busattachment_whoimplements_interfaces ( bus : alljoyn_busattachment , implementsinterfaces : *const *const i8 , numberinterfaces : usize ) -> QStatus );
    alljoyn_busattachment_whoimplements_interfaces(bus.into_param().abi(), implementsinterfaces, numberinterfaces)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_buslistener_create(callbacks: *const alljoyn_buslistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_buslistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_buslistener_create ( callbacks : *const alljoyn_buslistener_callbacks , context : *const ::core::ffi::c_void ) -> alljoyn_buslistener );
    alljoyn_buslistener_create(callbacks, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_buslistener_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_buslistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_buslistener_destroy ( listener : alljoyn_buslistener ) -> ( ) );
    alljoyn_buslistener_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_addinterface<P0, P1>(bus: P0, iface: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
    P1: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_addinterface ( bus : alljoyn_busobject , iface : alljoyn_interfacedescription ) -> QStatus );
    alljoyn_busobject_addinterface(bus.into_param().abi(), iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_addinterface_announced<P0, P1>(bus: P0, iface: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
    P1: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_addinterface_announced ( bus : alljoyn_busobject , iface : alljoyn_interfacedescription ) -> QStatus );
    alljoyn_busobject_addinterface_announced(bus.into_param().abi(), iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_addmethodhandler<P0>(bus: P0, member: alljoyn_interfacedescription_member, handler: alljoyn_messagereceiver_methodhandler_ptr, context: *mut ::core::ffi::c_void) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_addmethodhandler ( bus : alljoyn_busobject , member : alljoyn_interfacedescription_member , handler : alljoyn_messagereceiver_methodhandler_ptr , context : *mut ::core::ffi::c_void ) -> QStatus );
    alljoyn_busobject_addmethodhandler(bus.into_param().abi(), ::core::mem::transmute(member), handler, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_addmethodhandlers<P0>(bus: P0, entries: *const alljoyn_busobject_methodentry, numentries: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_addmethodhandlers ( bus : alljoyn_busobject , entries : *const alljoyn_busobject_methodentry , numentries : usize ) -> QStatus );
    alljoyn_busobject_addmethodhandlers(bus.into_param().abi(), entries, numentries)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_cancelsessionlessmessage<P0, P1>(bus: P0, msg: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
    P1: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_cancelsessionlessmessage ( bus : alljoyn_busobject , msg : alljoyn_message ) -> QStatus );
    alljoyn_busobject_cancelsessionlessmessage(bus.into_param().abi(), msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_cancelsessionlessmessage_serial<P0>(bus: P0, serialnumber: u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_cancelsessionlessmessage_serial ( bus : alljoyn_busobject , serialnumber : u32 ) -> QStatus );
    alljoyn_busobject_cancelsessionlessmessage_serial(bus.into_param().abi(), serialnumber)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_create<P0>(path: P0, isplaceholder: i32, callbacks_in: *const alljoyn_busobject_callbacks, context_in: *const ::core::ffi::c_void) -> alljoyn_busobject
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_create ( path : ::windows::core::PCSTR , isplaceholder : i32 , callbacks_in : *const alljoyn_busobject_callbacks , context_in : *const ::core::ffi::c_void ) -> alljoyn_busobject );
    alljoyn_busobject_create(path.into_param().abi(), isplaceholder, callbacks_in, context_in)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_destroy<P0>(bus: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_destroy ( bus : alljoyn_busobject ) -> ( ) );
    alljoyn_busobject_destroy(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_emitpropertieschanged<P0, P1>(bus: P0, ifcname: P1, propnames: *const *const i8, numprops: usize, id: u32)
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_emitpropertieschanged ( bus : alljoyn_busobject , ifcname : ::windows::core::PCSTR , propnames : *const *const i8 , numprops : usize , id : u32 ) -> ( ) );
    alljoyn_busobject_emitpropertieschanged(bus.into_param().abi(), ifcname.into_param().abi(), propnames, numprops, id)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_emitpropertychanged<P0, P1, P2, P3>(bus: P0, ifcname: P1, propname: P2, val: P3, id: u32)
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_emitpropertychanged ( bus : alljoyn_busobject , ifcname : ::windows::core::PCSTR , propname : ::windows::core::PCSTR , val : alljoyn_msgarg , id : u32 ) -> ( ) );
    alljoyn_busobject_emitpropertychanged(bus.into_param().abi(), ifcname.into_param().abi(), propname.into_param().abi(), val.into_param().abi(), id)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_getannouncedinterfacenames<P0>(bus: P0, interfaces: *const *const i8, numinterfaces: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_getannouncedinterfacenames ( bus : alljoyn_busobject , interfaces : *const *const i8 , numinterfaces : usize ) -> usize );
    alljoyn_busobject_getannouncedinterfacenames(bus.into_param().abi(), interfaces, numinterfaces)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_getbusattachment<P0>(bus: P0) -> alljoyn_busattachment
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_getbusattachment ( bus : alljoyn_busobject ) -> alljoyn_busattachment );
    alljoyn_busobject_getbusattachment(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_getname<P0, P1>(bus: P0, buffer: P1, buffersz: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_getname ( bus : alljoyn_busobject , buffer : ::windows::core::PCSTR , buffersz : usize ) -> usize );
    alljoyn_busobject_getname(bus.into_param().abi(), buffer.into_param().abi(), buffersz)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_getpath<P0>(bus: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_getpath ( bus : alljoyn_busobject ) -> ::windows::core::PCSTR );
    alljoyn_busobject_getpath(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_issecure<P0>(bus: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_issecure ( bus : alljoyn_busobject ) -> i32 );
    alljoyn_busobject_issecure(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_methodreply_args<P0, P1, P2>(bus: P0, msg: P1, args: P2, numargs: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
    P1: ::windows::core::IntoParam<alljoyn_message>,
    P2: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_methodreply_args ( bus : alljoyn_busobject , msg : alljoyn_message , args : alljoyn_msgarg , numargs : usize ) -> QStatus );
    alljoyn_busobject_methodreply_args(bus.into_param().abi(), msg.into_param().abi(), args.into_param().abi(), numargs)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_methodreply_err<P0, P1, P2, P3>(bus: P0, msg: P1, error: P2, errormessage: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
    P1: ::windows::core::IntoParam<alljoyn_message>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_methodreply_err ( bus : alljoyn_busobject , msg : alljoyn_message , error : ::windows::core::PCSTR , errormessage : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_busobject_methodreply_err(bus.into_param().abi(), msg.into_param().abi(), error.into_param().abi(), errormessage.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_methodreply_status<P0, P1>(bus: P0, msg: P1, status: QStatus) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
    P1: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_methodreply_status ( bus : alljoyn_busobject , msg : alljoyn_message , status : QStatus ) -> QStatus );
    alljoyn_busobject_methodreply_status(bus.into_param().abi(), msg.into_param().abi(), status)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_setannounceflag<P0, P1>(bus: P0, iface: P1, isannounced: alljoyn_about_announceflag) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
    P1: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_setannounceflag ( bus : alljoyn_busobject , iface : alljoyn_interfacedescription , isannounced : alljoyn_about_announceflag ) -> QStatus );
    alljoyn_busobject_setannounceflag(bus.into_param().abi(), iface.into_param().abi(), isannounced)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_busobject_signal<P0, P1, P2, P3>(bus: P0, destination: P1, sessionid: u32, signal: alljoyn_interfacedescription_member, args: P2, numargs: usize, timetolive: u16, flags: u8, msg: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_busobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<alljoyn_msgarg>,
    P3: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_busobject_signal ( bus : alljoyn_busobject , destination : ::windows::core::PCSTR , sessionid : u32 , signal : alljoyn_interfacedescription_member , args : alljoyn_msgarg , numargs : usize , timetolive : u16 , flags : u8 , msg : alljoyn_message ) -> QStatus );
    alljoyn_busobject_signal(bus.into_param().abi(), destination.into_param().abi(), sessionid, ::core::mem::transmute(signal), args.into_param().abi(), numargs, timetolive, flags, msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_clear<P0>(cred: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_clear ( cred : alljoyn_credentials ) -> ( ) );
    alljoyn_credentials_clear(cred.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_create() -> alljoyn_credentials {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_create ( ) -> alljoyn_credentials );
    alljoyn_credentials_create()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_destroy<P0>(cred: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_destroy ( cred : alljoyn_credentials ) -> ( ) );
    alljoyn_credentials_destroy(cred.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_getcertchain<P0>(cred: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_getcertchain ( cred : alljoyn_credentials ) -> ::windows::core::PCSTR );
    alljoyn_credentials_getcertchain(cred.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_getexpiration<P0>(cred: P0) -> u32
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_getexpiration ( cred : alljoyn_credentials ) -> u32 );
    alljoyn_credentials_getexpiration(cred.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_getlogonentry<P0>(cred: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_getlogonentry ( cred : alljoyn_credentials ) -> ::windows::core::PCSTR );
    alljoyn_credentials_getlogonentry(cred.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_getpassword<P0>(cred: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_getpassword ( cred : alljoyn_credentials ) -> ::windows::core::PCSTR );
    alljoyn_credentials_getpassword(cred.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_getprivateKey<P0>(cred: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_getprivateKey ( cred : alljoyn_credentials ) -> ::windows::core::PCSTR );
    alljoyn_credentials_getprivateKey(cred.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_getusername<P0>(cred: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_getusername ( cred : alljoyn_credentials ) -> ::windows::core::PCSTR );
    alljoyn_credentials_getusername(cred.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_isset<P0>(cred: P0, creds: u16) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_isset ( cred : alljoyn_credentials , creds : u16 ) -> i32 );
    alljoyn_credentials_isset(cred.into_param().abi(), creds)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_setcertchain<P0, P1>(cred: P0, certchain: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_setcertchain ( cred : alljoyn_credentials , certchain : ::windows::core::PCSTR ) -> ( ) );
    alljoyn_credentials_setcertchain(cred.into_param().abi(), certchain.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_setexpiration<P0>(cred: P0, expiration: u32)
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_setexpiration ( cred : alljoyn_credentials , expiration : u32 ) -> ( ) );
    alljoyn_credentials_setexpiration(cred.into_param().abi(), expiration)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_setlogonentry<P0, P1>(cred: P0, logonentry: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_setlogonentry ( cred : alljoyn_credentials , logonentry : ::windows::core::PCSTR ) -> ( ) );
    alljoyn_credentials_setlogonentry(cred.into_param().abi(), logonentry.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_setpassword<P0, P1>(cred: P0, pwd: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_setpassword ( cred : alljoyn_credentials , pwd : ::windows::core::PCSTR ) -> ( ) );
    alljoyn_credentials_setpassword(cred.into_param().abi(), pwd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_setprivatekey<P0, P1>(cred: P0, pk: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_setprivatekey ( cred : alljoyn_credentials , pk : ::windows::core::PCSTR ) -> ( ) );
    alljoyn_credentials_setprivatekey(cred.into_param().abi(), pk.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_credentials_setusername<P0, P1>(cred: P0, username: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_credentials>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_credentials_setusername ( cred : alljoyn_credentials , username : ::windows::core::PCSTR ) -> ( ) );
    alljoyn_credentials_setusername(cred.into_param().abi(), username.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_getbuildinfo() -> ::windows::core::PCSTR {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_getbuildinfo ( ) -> ::windows::core::PCSTR );
    alljoyn_getbuildinfo()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_getnumericversion() -> u32 {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_getnumericversion ( ) -> u32 );
    alljoyn_getnumericversion()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_getversion() -> ::windows::core::PCSTR {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_getversion ( ) -> ::windows::core::PCSTR );
    alljoyn_getversion()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_init() -> QStatus {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_init ( ) -> QStatus );
    alljoyn_init()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_activate<P0>(iface: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_activate ( iface : alljoyn_interfacedescription ) -> ( ) );
    alljoyn_interfacedescription_activate(iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_addannotation<P0, P1, P2>(iface: P0, name: P1, value: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_addannotation ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , value : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_addannotation(iface.into_param().abi(), name.into_param().abi(), value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_addargannotation<P0, P1, P2, P3, P4>(iface: P0, member: P1, argname: P2, name: P3, value: P4) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_addargannotation ( iface : alljoyn_interfacedescription , member : ::windows::core::PCSTR , argname : ::windows::core::PCSTR , name : ::windows::core::PCSTR , value : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_addargannotation(iface.into_param().abi(), member.into_param().abi(), argname.into_param().abi(), name.into_param().abi(), value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_addmember<P0, P1, P2, P3, P4>(iface: P0, r#type: alljoyn_messagetype, name: P1, inputsig: P2, outsig: P3, argnames: P4, annotation: u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_addmember ( iface : alljoyn_interfacedescription , r#type : alljoyn_messagetype , name : ::windows::core::PCSTR , inputsig : ::windows::core::PCSTR , outsig : ::windows::core::PCSTR , argnames : ::windows::core::PCSTR , annotation : u8 ) -> QStatus );
    alljoyn_interfacedescription_addmember(iface.into_param().abi(), r#type, name.into_param().abi(), inputsig.into_param().abi(), outsig.into_param().abi(), argnames.into_param().abi(), annotation)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_addmemberannotation<P0, P1, P2, P3>(iface: P0, member: P1, name: P2, value: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_addmemberannotation ( iface : alljoyn_interfacedescription , member : ::windows::core::PCSTR , name : ::windows::core::PCSTR , value : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_addmemberannotation(iface.into_param().abi(), member.into_param().abi(), name.into_param().abi(), value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_addmethod<P0, P1, P2, P3, P4, P5>(iface: P0, name: P1, inputsig: P2, outsig: P3, argnames: P4, annotation: u8, accessperms: P5) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P5: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_addmethod ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , inputsig : ::windows::core::PCSTR , outsig : ::windows::core::PCSTR , argnames : ::windows::core::PCSTR , annotation : u8 , accessperms : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_addmethod(iface.into_param().abi(), name.into_param().abi(), inputsig.into_param().abi(), outsig.into_param().abi(), argnames.into_param().abi(), annotation, accessperms.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_addproperty<P0, P1, P2>(iface: P0, name: P1, signature: P2, access: u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_addproperty ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , signature : ::windows::core::PCSTR , access : u8 ) -> QStatus );
    alljoyn_interfacedescription_addproperty(iface.into_param().abi(), name.into_param().abi(), signature.into_param().abi(), access)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_addpropertyannotation<P0, P1, P2, P3>(iface: P0, property: P1, name: P2, value: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_addpropertyannotation ( iface : alljoyn_interfacedescription , property : ::windows::core::PCSTR , name : ::windows::core::PCSTR , value : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_addpropertyannotation(iface.into_param().abi(), property.into_param().abi(), name.into_param().abi(), value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_addsignal<P0, P1, P2, P3, P4>(iface: P0, name: P1, sig: P2, argnames: P3, annotation: u8, accessperms: P4) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_addsignal ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , sig : ::windows::core::PCSTR , argnames : ::windows::core::PCSTR , annotation : u8 , accessperms : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_addsignal(iface.into_param().abi(), name.into_param().abi(), sig.into_param().abi(), argnames.into_param().abi(), annotation, accessperms.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_eql<P0, P1>(one: P0, other: P1) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_eql ( one : alljoyn_interfacedescription , other : alljoyn_interfacedescription ) -> i32 );
    alljoyn_interfacedescription_eql(one.into_param().abi(), other.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getannotation<P0, P1, P2>(iface: P0, name: P1, value: P2, value_size: *mut usize) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getannotation ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , value : ::windows::core::PCSTR , value_size : *mut usize ) -> i32 );
    alljoyn_interfacedescription_getannotation(iface.into_param().abi(), name.into_param().abi(), value.into_param().abi(), value_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getannotationatindex<P0, P1, P2>(iface: P0, index: usize, name: P1, name_size: *mut usize, value: P2, value_size: *mut usize)
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getannotationatindex ( iface : alljoyn_interfacedescription , index : usize , name : ::windows::core::PCSTR , name_size : *mut usize , value : ::windows::core::PCSTR , value_size : *mut usize ) -> ( ) );
    alljoyn_interfacedescription_getannotationatindex(iface.into_param().abi(), index, name.into_param().abi(), name_size, value.into_param().abi(), value_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getannotationscount<P0>(iface: P0) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getannotationscount ( iface : alljoyn_interfacedescription ) -> usize );
    alljoyn_interfacedescription_getannotationscount(iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getargdescriptionforlanguage<P0, P1, P2, P3, P4>(iface: P0, member: P1, arg: P2, description: P3, maxlanguagelength: usize, languagetag: P4) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getargdescriptionforlanguage ( iface : alljoyn_interfacedescription , member : ::windows::core::PCSTR , arg : ::windows::core::PCSTR , description : ::windows::core::PCSTR , maxlanguagelength : usize , languagetag : ::windows::core::PCSTR ) -> usize );
    alljoyn_interfacedescription_getargdescriptionforlanguage(iface.into_param().abi(), member.into_param().abi(), arg.into_param().abi(), description.into_param().abi(), maxlanguagelength, languagetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getdescriptionforlanguage<P0, P1, P2>(iface: P0, description: P1, maxlanguagelength: usize, languagetag: P2) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getdescriptionforlanguage ( iface : alljoyn_interfacedescription , description : ::windows::core::PCSTR , maxlanguagelength : usize , languagetag : ::windows::core::PCSTR ) -> usize );
    alljoyn_interfacedescription_getdescriptionforlanguage(iface.into_param().abi(), description.into_param().abi(), maxlanguagelength, languagetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getdescriptionlanguages<P0>(iface: P0, languages: *const *const i8, size: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getdescriptionlanguages ( iface : alljoyn_interfacedescription , languages : *const *const i8 , size : usize ) -> usize );
    alljoyn_interfacedescription_getdescriptionlanguages(iface.into_param().abi(), languages, size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getdescriptionlanguages2<P0, P1>(iface: P0, languages: P1, languagessize: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getdescriptionlanguages2 ( iface : alljoyn_interfacedescription , languages : ::windows::core::PCSTR , languagessize : usize ) -> usize );
    alljoyn_interfacedescription_getdescriptionlanguages2(iface.into_param().abi(), languages.into_param().abi(), languagessize)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getdescriptiontranslationcallback<P0>(iface: P0) -> alljoyn_interfacedescription_translation_callback_ptr
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getdescriptiontranslationcallback ( iface : alljoyn_interfacedescription ) -> alljoyn_interfacedescription_translation_callback_ptr );
    alljoyn_interfacedescription_getdescriptiontranslationcallback(iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmember<P0, P1>(iface: P0, name: P1, member: *mut alljoyn_interfacedescription_member) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getmember ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , member : *mut alljoyn_interfacedescription_member ) -> i32 );
    alljoyn_interfacedescription_getmember(iface.into_param().abi(), name.into_param().abi(), member)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmemberannotation<P0, P1, P2, P3>(iface: P0, member: P1, name: P2, value: P3, value_size: *mut usize) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getmemberannotation ( iface : alljoyn_interfacedescription , member : ::windows::core::PCSTR , name : ::windows::core::PCSTR , value : ::windows::core::PCSTR , value_size : *mut usize ) -> i32 );
    alljoyn_interfacedescription_getmemberannotation(iface.into_param().abi(), member.into_param().abi(), name.into_param().abi(), value.into_param().abi(), value_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmemberargannotation<P0, P1, P2, P3, P4>(iface: P0, member: P1, argname: P2, name: P3, value: P4, value_size: *mut usize) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getmemberargannotation ( iface : alljoyn_interfacedescription , member : ::windows::core::PCSTR , argname : ::windows::core::PCSTR , name : ::windows::core::PCSTR , value : ::windows::core::PCSTR , value_size : *mut usize ) -> i32 );
    alljoyn_interfacedescription_getmemberargannotation(iface.into_param().abi(), member.into_param().abi(), argname.into_param().abi(), name.into_param().abi(), value.into_param().abi(), value_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmemberdescriptionforlanguage<P0, P1, P2, P3>(iface: P0, member: P1, description: P2, maxlanguagelength: usize, languagetag: P3) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getmemberdescriptionforlanguage ( iface : alljoyn_interfacedescription , member : ::windows::core::PCSTR , description : ::windows::core::PCSTR , maxlanguagelength : usize , languagetag : ::windows::core::PCSTR ) -> usize );
    alljoyn_interfacedescription_getmemberdescriptionforlanguage(iface.into_param().abi(), member.into_param().abi(), description.into_param().abi(), maxlanguagelength, languagetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmembers<P0>(iface: P0, members: *mut alljoyn_interfacedescription_member, nummembers: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getmembers ( iface : alljoyn_interfacedescription , members : *mut alljoyn_interfacedescription_member , nummembers : usize ) -> usize );
    alljoyn_interfacedescription_getmembers(iface.into_param().abi(), members, nummembers)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmethod<P0, P1>(iface: P0, name: P1, member: *mut alljoyn_interfacedescription_member) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getmethod ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , member : *mut alljoyn_interfacedescription_member ) -> i32 );
    alljoyn_interfacedescription_getmethod(iface.into_param().abi(), name.into_param().abi(), member)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getname<P0>(iface: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getname ( iface : alljoyn_interfacedescription ) -> ::windows::core::PCSTR );
    alljoyn_interfacedescription_getname(iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getproperties<P0>(iface: P0, props: *mut alljoyn_interfacedescription_property, numprops: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getproperties ( iface : alljoyn_interfacedescription , props : *mut alljoyn_interfacedescription_property , numprops : usize ) -> usize );
    alljoyn_interfacedescription_getproperties(iface.into_param().abi(), props, numprops)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getproperty<P0, P1>(iface: P0, name: P1, property: *mut alljoyn_interfacedescription_property) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getproperty ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , property : *mut alljoyn_interfacedescription_property ) -> i32 );
    alljoyn_interfacedescription_getproperty(iface.into_param().abi(), name.into_param().abi(), property)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getpropertyannotation<P0, P1, P2, P3>(iface: P0, property: P1, name: P2, value: P3, str_size: *mut usize) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getpropertyannotation ( iface : alljoyn_interfacedescription , property : ::windows::core::PCSTR , name : ::windows::core::PCSTR , value : ::windows::core::PCSTR , str_size : *mut usize ) -> i32 );
    alljoyn_interfacedescription_getpropertyannotation(iface.into_param().abi(), property.into_param().abi(), name.into_param().abi(), value.into_param().abi(), str_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getpropertydescriptionforlanguage<P0, P1, P2, P3>(iface: P0, property: P1, description: P2, maxlanguagelength: usize, languagetag: P3) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getpropertydescriptionforlanguage ( iface : alljoyn_interfacedescription , property : ::windows::core::PCSTR , description : ::windows::core::PCSTR , maxlanguagelength : usize , languagetag : ::windows::core::PCSTR ) -> usize );
    alljoyn_interfacedescription_getpropertydescriptionforlanguage(iface.into_param().abi(), property.into_param().abi(), description.into_param().abi(), maxlanguagelength, languagetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getsecuritypolicy<P0>(iface: P0) -> alljoyn_interfacedescription_securitypolicy
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getsecuritypolicy ( iface : alljoyn_interfacedescription ) -> alljoyn_interfacedescription_securitypolicy );
    alljoyn_interfacedescription_getsecuritypolicy(iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_getsignal<P0, P1>(iface: P0, name: P1, member: *mut alljoyn_interfacedescription_member) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_getsignal ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , member : *mut alljoyn_interfacedescription_member ) -> i32 );
    alljoyn_interfacedescription_getsignal(iface.into_param().abi(), name.into_param().abi(), member)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_hasdescription<P0>(iface: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_hasdescription ( iface : alljoyn_interfacedescription ) -> i32 );
    alljoyn_interfacedescription_hasdescription(iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_hasmember<P0, P1, P2, P3>(iface: P0, name: P1, insig: P2, outsig: P3) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_hasmember ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , insig : ::windows::core::PCSTR , outsig : ::windows::core::PCSTR ) -> i32 );
    alljoyn_interfacedescription_hasmember(iface.into_param().abi(), name.into_param().abi(), insig.into_param().abi(), outsig.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_hasproperties<P0>(iface: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_hasproperties ( iface : alljoyn_interfacedescription ) -> i32 );
    alljoyn_interfacedescription_hasproperties(iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_hasproperty<P0, P1>(iface: P0, name: P1) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_hasproperty ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR ) -> i32 );
    alljoyn_interfacedescription_hasproperty(iface.into_param().abi(), name.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_introspect<P0, P1>(iface: P0, str: P1, buf: usize, indent: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_introspect ( iface : alljoyn_interfacedescription , str : ::windows::core::PCSTR , buf : usize , indent : usize ) -> usize );
    alljoyn_interfacedescription_introspect(iface.into_param().abi(), str.into_param().abi(), buf, indent)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_issecure<P0>(iface: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_issecure ( iface : alljoyn_interfacedescription ) -> i32 );
    alljoyn_interfacedescription_issecure(iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_eql(one: alljoyn_interfacedescription_member, other: alljoyn_interfacedescription_member) -> i32 {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_member_eql ( one : alljoyn_interfacedescription_member , other : alljoyn_interfacedescription_member ) -> i32 );
    alljoyn_interfacedescription_member_eql(::core::mem::transmute(one), ::core::mem::transmute(other))
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getannotation<P0, P1>(member: alljoyn_interfacedescription_member, name: P0, value: P1, value_size: *mut usize) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_member_getannotation ( member : alljoyn_interfacedescription_member , name : ::windows::core::PCSTR , value : ::windows::core::PCSTR , value_size : *mut usize ) -> i32 );
    alljoyn_interfacedescription_member_getannotation(::core::mem::transmute(member), name.into_param().abi(), value.into_param().abi(), value_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getannotationatindex<P0, P1>(member: alljoyn_interfacedescription_member, index: usize, name: P0, name_size: *mut usize, value: P1, value_size: *mut usize)
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_member_getannotationatindex ( member : alljoyn_interfacedescription_member , index : usize , name : ::windows::core::PCSTR , name_size : *mut usize , value : ::windows::core::PCSTR , value_size : *mut usize ) -> ( ) );
    alljoyn_interfacedescription_member_getannotationatindex(::core::mem::transmute(member), index, name.into_param().abi(), name_size, value.into_param().abi(), value_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getannotationscount(member: alljoyn_interfacedescription_member) -> usize {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_member_getannotationscount ( member : alljoyn_interfacedescription_member ) -> usize );
    alljoyn_interfacedescription_member_getannotationscount(::core::mem::transmute(member))
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getargannotation<P0, P1, P2>(member: alljoyn_interfacedescription_member, argname: P0, name: P1, value: P2, value_size: *mut usize) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_member_getargannotation ( member : alljoyn_interfacedescription_member , argname : ::windows::core::PCSTR , name : ::windows::core::PCSTR , value : ::windows::core::PCSTR , value_size : *mut usize ) -> i32 );
    alljoyn_interfacedescription_member_getargannotation(::core::mem::transmute(member), argname.into_param().abi(), name.into_param().abi(), value.into_param().abi(), value_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getargannotationatindex<P0, P1, P2>(member: alljoyn_interfacedescription_member, argname: P0, index: usize, name: P1, name_size: *mut usize, value: P2, value_size: *mut usize)
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_member_getargannotationatindex ( member : alljoyn_interfacedescription_member , argname : ::windows::core::PCSTR , index : usize , name : ::windows::core::PCSTR , name_size : *mut usize , value : ::windows::core::PCSTR , value_size : *mut usize ) -> ( ) );
    alljoyn_interfacedescription_member_getargannotationatindex(::core::mem::transmute(member), argname.into_param().abi(), index, name.into_param().abi(), name_size, value.into_param().abi(), value_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getargannotationscount<P0>(member: alljoyn_interfacedescription_member, argname: P0) -> usize
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_member_getargannotationscount ( member : alljoyn_interfacedescription_member , argname : ::windows::core::PCSTR ) -> usize );
    alljoyn_interfacedescription_member_getargannotationscount(::core::mem::transmute(member), argname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_property_eql(one: alljoyn_interfacedescription_property, other: alljoyn_interfacedescription_property) -> i32 {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_property_eql ( one : alljoyn_interfacedescription_property , other : alljoyn_interfacedescription_property ) -> i32 );
    alljoyn_interfacedescription_property_eql(::core::mem::transmute(one), ::core::mem::transmute(other))
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_property_getannotation<P0, P1>(property: alljoyn_interfacedescription_property, name: P0, value: P1, value_size: *mut usize) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_property_getannotation ( property : alljoyn_interfacedescription_property , name : ::windows::core::PCSTR , value : ::windows::core::PCSTR , value_size : *mut usize ) -> i32 );
    alljoyn_interfacedescription_property_getannotation(::core::mem::transmute(property), name.into_param().abi(), value.into_param().abi(), value_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_property_getannotationatindex<P0, P1>(property: alljoyn_interfacedescription_property, index: usize, name: P0, name_size: *mut usize, value: P1, value_size: *mut usize)
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_property_getannotationatindex ( property : alljoyn_interfacedescription_property , index : usize , name : ::windows::core::PCSTR , name_size : *mut usize , value : ::windows::core::PCSTR , value_size : *mut usize ) -> ( ) );
    alljoyn_interfacedescription_property_getannotationatindex(::core::mem::transmute(property), index, name.into_param().abi(), name_size, value.into_param().abi(), value_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_property_getannotationscount(property: alljoyn_interfacedescription_property) -> usize {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_property_getannotationscount ( property : alljoyn_interfacedescription_property ) -> usize );
    alljoyn_interfacedescription_property_getannotationscount(::core::mem::transmute(property))
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_setargdescription<P0, P1, P2, P3>(iface: P0, member: P1, argname: P2, description: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_setargdescription ( iface : alljoyn_interfacedescription , member : ::windows::core::PCSTR , argname : ::windows::core::PCSTR , description : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_setargdescription(iface.into_param().abi(), member.into_param().abi(), argname.into_param().abi(), description.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_setargdescriptionforlanguage<P0, P1, P2, P3, P4>(iface: P0, member: P1, arg: P2, description: P3, languagetag: P4) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_setargdescriptionforlanguage ( iface : alljoyn_interfacedescription , member : ::windows::core::PCSTR , arg : ::windows::core::PCSTR , description : ::windows::core::PCSTR , languagetag : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_setargdescriptionforlanguage(iface.into_param().abi(), member.into_param().abi(), arg.into_param().abi(), description.into_param().abi(), languagetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_setdescription<P0, P1>(iface: P0, description: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_setdescription ( iface : alljoyn_interfacedescription , description : ::windows::core::PCSTR ) -> ( ) );
    alljoyn_interfacedescription_setdescription(iface.into_param().abi(), description.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_setdescriptionforlanguage<P0, P1, P2>(iface: P0, description: P1, languagetag: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_setdescriptionforlanguage ( iface : alljoyn_interfacedescription , description : ::windows::core::PCSTR , languagetag : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_setdescriptionforlanguage(iface.into_param().abi(), description.into_param().abi(), languagetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_setdescriptionlanguage<P0, P1>(iface: P0, language: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_setdescriptionlanguage ( iface : alljoyn_interfacedescription , language : ::windows::core::PCSTR ) -> ( ) );
    alljoyn_interfacedescription_setdescriptionlanguage(iface.into_param().abi(), language.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_setdescriptiontranslationcallback<P0>(iface: P0, translationcallback: alljoyn_interfacedescription_translation_callback_ptr)
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_setdescriptiontranslationcallback ( iface : alljoyn_interfacedescription , translationcallback : alljoyn_interfacedescription_translation_callback_ptr ) -> ( ) );
    alljoyn_interfacedescription_setdescriptiontranslationcallback(iface.into_param().abi(), translationcallback)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_setmemberdescription<P0, P1, P2>(iface: P0, member: P1, description: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_setmemberdescription ( iface : alljoyn_interfacedescription , member : ::windows::core::PCSTR , description : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_setmemberdescription(iface.into_param().abi(), member.into_param().abi(), description.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_setmemberdescriptionforlanguage<P0, P1, P2, P3>(iface: P0, member: P1, description: P2, languagetag: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_setmemberdescriptionforlanguage ( iface : alljoyn_interfacedescription , member : ::windows::core::PCSTR , description : ::windows::core::PCSTR , languagetag : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_setmemberdescriptionforlanguage(iface.into_param().abi(), member.into_param().abi(), description.into_param().abi(), languagetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_setpropertydescription<P0, P1, P2>(iface: P0, name: P1, description: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_setpropertydescription ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , description : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_setpropertydescription(iface.into_param().abi(), name.into_param().abi(), description.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_interfacedescription_setpropertydescriptionforlanguage<P0, P1, P2, P3>(iface: P0, name: P1, description: P2, languagetag: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_interfacedescription>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_interfacedescription_setpropertydescriptionforlanguage ( iface : alljoyn_interfacedescription , name : ::windows::core::PCSTR , description : ::windows::core::PCSTR , languagetag : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_interfacedescription_setpropertydescriptionforlanguage(iface.into_param().abi(), name.into_param().abi(), description.into_param().abi(), languagetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_keystorelistener_create(callbacks: *const alljoyn_keystorelistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_keystorelistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_keystorelistener_create ( callbacks : *const alljoyn_keystorelistener_callbacks , context : *const ::core::ffi::c_void ) -> alljoyn_keystorelistener );
    alljoyn_keystorelistener_create(callbacks, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_keystorelistener_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_keystorelistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_keystorelistener_destroy ( listener : alljoyn_keystorelistener ) -> ( ) );
    alljoyn_keystorelistener_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_keystorelistener_getkeys<P0, P1, P2>(listener: P0, keystore: P1, sink: P2, sink_sz: *mut usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_keystorelistener>,
    P1: ::windows::core::IntoParam<alljoyn_keystore>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_keystorelistener_getkeys ( listener : alljoyn_keystorelistener , keystore : alljoyn_keystore , sink : ::windows::core::PCSTR , sink_sz : *mut usize ) -> QStatus );
    alljoyn_keystorelistener_getkeys(listener.into_param().abi(), keystore.into_param().abi(), sink.into_param().abi(), sink_sz)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_keystorelistener_putkeys<P0, P1, P2, P3>(listener: P0, keystore: P1, source: P2, password: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_keystorelistener>,
    P1: ::windows::core::IntoParam<alljoyn_keystore>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_keystorelistener_putkeys ( listener : alljoyn_keystorelistener , keystore : alljoyn_keystore , source : ::windows::core::PCSTR , password : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_keystorelistener_putkeys(listener.into_param().abi(), keystore.into_param().abi(), source.into_param().abi(), password.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_keystorelistener_with_synchronization_create(callbacks: *const alljoyn_keystorelistener_with_synchronization_callbacks, context: *mut ::core::ffi::c_void) -> alljoyn_keystorelistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_keystorelistener_with_synchronization_create ( callbacks : *const alljoyn_keystorelistener_with_synchronization_callbacks , context : *mut ::core::ffi::c_void ) -> alljoyn_keystorelistener );
    alljoyn_keystorelistener_with_synchronization_create(callbacks, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_create<P0>(bus: P0) -> alljoyn_message
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_create ( bus : alljoyn_busattachment ) -> alljoyn_message );
    alljoyn_message_create(bus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_description<P0, P1>(msg: P0, str: P1, buf: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_description ( msg : alljoyn_message , str : ::windows::core::PCSTR , buf : usize ) -> usize );
    alljoyn_message_description(msg.into_param().abi(), str.into_param().abi(), buf)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_destroy<P0>(msg: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_destroy ( msg : alljoyn_message ) -> ( ) );
    alljoyn_message_destroy(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_eql<P0, P1>(one: P0, other: P1) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
    P1: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_eql ( one : alljoyn_message , other : alljoyn_message ) -> i32 );
    alljoyn_message_eql(one.into_param().abi(), other.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getarg<P0>(msg: P0, argn: usize) -> alljoyn_msgarg
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getarg ( msg : alljoyn_message , argn : usize ) -> alljoyn_msgarg );
    alljoyn_message_getarg(msg.into_param().abi(), argn)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getargs<P0>(msg: P0, numargs: *mut usize, args: *mut alljoyn_msgarg)
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getargs ( msg : alljoyn_message , numargs : *mut usize , args : *mut alljoyn_msgarg ) -> ( ) );
    alljoyn_message_getargs(msg.into_param().abi(), numargs, args)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getauthmechanism<P0>(msg: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getauthmechanism ( msg : alljoyn_message ) -> ::windows::core::PCSTR );
    alljoyn_message_getauthmechanism(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getcallserial<P0>(msg: P0) -> u32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getcallserial ( msg : alljoyn_message ) -> u32 );
    alljoyn_message_getcallserial(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getcompressiontoken<P0>(msg: P0) -> u32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getcompressiontoken ( msg : alljoyn_message ) -> u32 );
    alljoyn_message_getcompressiontoken(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getdestination<P0>(msg: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getdestination ( msg : alljoyn_message ) -> ::windows::core::PCSTR );
    alljoyn_message_getdestination(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_geterrorname<P0, P1>(msg: P0, errormessage: P1, errormessage_size: *mut usize) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_geterrorname ( msg : alljoyn_message , errormessage : ::windows::core::PCSTR , errormessage_size : *mut usize ) -> ::windows::core::PCSTR );
    alljoyn_message_geterrorname(msg.into_param().abi(), errormessage.into_param().abi(), errormessage_size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getflags<P0>(msg: P0) -> u8
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getflags ( msg : alljoyn_message ) -> u8 );
    alljoyn_message_getflags(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getinterface<P0>(msg: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getinterface ( msg : alljoyn_message ) -> ::windows::core::PCSTR );
    alljoyn_message_getinterface(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getmembername<P0>(msg: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getmembername ( msg : alljoyn_message ) -> ::windows::core::PCSTR );
    alljoyn_message_getmembername(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getobjectpath<P0>(msg: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getobjectpath ( msg : alljoyn_message ) -> ::windows::core::PCSTR );
    alljoyn_message_getobjectpath(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getreceiveendpointname<P0>(msg: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getreceiveendpointname ( msg : alljoyn_message ) -> ::windows::core::PCSTR );
    alljoyn_message_getreceiveendpointname(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getreplyserial<P0>(msg: P0) -> u32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getreplyserial ( msg : alljoyn_message ) -> u32 );
    alljoyn_message_getreplyserial(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getsender<P0>(msg: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getsender ( msg : alljoyn_message ) -> ::windows::core::PCSTR );
    alljoyn_message_getsender(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getsessionid<P0>(msg: P0) -> u32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getsessionid ( msg : alljoyn_message ) -> u32 );
    alljoyn_message_getsessionid(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_getsignature<P0>(msg: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_getsignature ( msg : alljoyn_message ) -> ::windows::core::PCSTR );
    alljoyn_message_getsignature(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_gettimestamp<P0>(msg: P0) -> u32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_gettimestamp ( msg : alljoyn_message ) -> u32 );
    alljoyn_message_gettimestamp(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_gettype<P0>(msg: P0) -> alljoyn_messagetype
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_gettype ( msg : alljoyn_message ) -> alljoyn_messagetype );
    alljoyn_message_gettype(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_isbroadcastsignal<P0>(msg: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_isbroadcastsignal ( msg : alljoyn_message ) -> i32 );
    alljoyn_message_isbroadcastsignal(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_isencrypted<P0>(msg: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_isencrypted ( msg : alljoyn_message ) -> i32 );
    alljoyn_message_isencrypted(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_isexpired<P0>(msg: P0, tillexpirems: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_isexpired ( msg : alljoyn_message , tillexpirems : *mut u32 ) -> i32 );
    alljoyn_message_isexpired(msg.into_param().abi(), tillexpirems)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_isglobalbroadcast<P0>(msg: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_isglobalbroadcast ( msg : alljoyn_message ) -> i32 );
    alljoyn_message_isglobalbroadcast(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_issessionless<P0>(msg: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_issessionless ( msg : alljoyn_message ) -> i32 );
    alljoyn_message_issessionless(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_isunreliable<P0>(msg: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_isunreliable ( msg : alljoyn_message ) -> i32 );
    alljoyn_message_isunreliable(msg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_parseargs<P0, P1>(msg: P0, signature: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""cdecl" fn alljoyn_message_parseargs ( msg : alljoyn_message , signature : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_message_parseargs(msg.into_param().abi(), signature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_setendianess(endian: i8) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_setendianess ( endian : i8 ) -> ( ) );
    alljoyn_message_setendianess(endian)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_message_tostring<P0, P1>(msg: P0, str: P1, buf: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_message>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_message_tostring ( msg : alljoyn_message , str : ::windows::core::PCSTR , buf : usize ) -> usize );
    alljoyn_message_tostring(msg.into_param().abi(), str.into_param().abi(), buf)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_array_create(size: usize) -> alljoyn_msgarg {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_array_create ( size : usize ) -> alljoyn_msgarg );
    alljoyn_msgarg_array_create(size)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_array_element<P0>(arg: P0, index: usize) -> alljoyn_msgarg
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_array_element ( arg : alljoyn_msgarg , index : usize ) -> alljoyn_msgarg );
    alljoyn_msgarg_array_element(arg.into_param().abi(), index)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_array_get<P0, P1>(args: P0, numargs: usize, signature: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""cdecl" fn alljoyn_msgarg_array_get ( args : alljoyn_msgarg , numargs : usize , signature : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_msgarg_array_get(args.into_param().abi(), numargs, signature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_array_set<P0, P1>(args: P0, numargs: *mut usize, signature: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""cdecl" fn alljoyn_msgarg_array_set ( args : alljoyn_msgarg , numargs : *mut usize , signature : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_msgarg_array_set(args.into_param().abi(), numargs, signature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_array_set_offset<P0, P1>(args: P0, argoffset: usize, numargs: *mut usize, signature: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""cdecl" fn alljoyn_msgarg_array_set_offset ( args : alljoyn_msgarg , argoffset : usize , numargs : *mut usize , signature : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_msgarg_array_set_offset(args.into_param().abi(), argoffset, numargs, signature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_array_signature<P0, P1>(values: P0, numvalues: usize, str: P1, buf: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_array_signature ( values : alljoyn_msgarg , numvalues : usize , str : ::windows::core::PCSTR , buf : usize ) -> usize );
    alljoyn_msgarg_array_signature(values.into_param().abi(), numvalues, str.into_param().abi(), buf)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_array_tostring<P0, P1>(args: P0, numargs: usize, str: P1, buf: usize, indent: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_array_tostring ( args : alljoyn_msgarg , numargs : usize , str : ::windows::core::PCSTR , buf : usize , indent : usize ) -> usize );
    alljoyn_msgarg_array_tostring(args.into_param().abi(), numargs, str.into_param().abi(), buf, indent)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_clear<P0>(arg: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_clear ( arg : alljoyn_msgarg ) -> ( ) );
    alljoyn_msgarg_clear(arg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_clone<P0, P1>(destination: P0, source: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_clone ( destination : alljoyn_msgarg , source : alljoyn_msgarg ) -> ( ) );
    alljoyn_msgarg_clone(destination.into_param().abi(), source.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_copy<P0>(source: P0) -> alljoyn_msgarg
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_copy ( source : alljoyn_msgarg ) -> alljoyn_msgarg );
    alljoyn_msgarg_copy(source.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_create() -> alljoyn_msgarg {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_create ( ) -> alljoyn_msgarg );
    alljoyn_msgarg_create()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_create_and_set<P0>(signature: P0) -> alljoyn_msgarg
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""cdecl" fn alljoyn_msgarg_create_and_set ( signature : ::windows::core::PCSTR ) -> alljoyn_msgarg );
    alljoyn_msgarg_create_and_set(signature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_destroy<P0>(arg: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_destroy ( arg : alljoyn_msgarg ) -> ( ) );
    alljoyn_msgarg_destroy(arg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_equal<P0, P1>(lhv: P0, rhv: P1) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_equal ( lhv : alljoyn_msgarg , rhv : alljoyn_msgarg ) -> i32 );
    alljoyn_msgarg_equal(lhv.into_param().abi(), rhv.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get<P0, P1>(arg: P0, signature: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""cdecl" fn alljoyn_msgarg_get ( arg : alljoyn_msgarg , signature : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_msgarg_get(arg.into_param().abi(), signature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_array_element<P0>(arg: P0, index: usize, element: *mut alljoyn_msgarg)
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_array_element ( arg : alljoyn_msgarg , index : usize , element : *mut alljoyn_msgarg ) -> ( ) );
    alljoyn_msgarg_get_array_element(arg.into_param().abi(), index, element)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_array_elementsignature<P0>(arg: P0, index: usize) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_array_elementsignature ( arg : alljoyn_msgarg , index : usize ) -> ::windows::core::PCSTR );
    alljoyn_msgarg_get_array_elementsignature(arg.into_param().abi(), index)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_array_numberofelements<P0>(arg: P0) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_array_numberofelements ( arg : alljoyn_msgarg ) -> usize );
    alljoyn_msgarg_get_array_numberofelements(arg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_bool<P0>(arg: P0, b: *mut i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_bool ( arg : alljoyn_msgarg , b : *mut i32 ) -> QStatus );
    alljoyn_msgarg_get_bool(arg.into_param().abi(), b)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_bool_array<P0>(arg: P0, length: *mut usize, ab: *mut i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_bool_array ( arg : alljoyn_msgarg , length : *mut usize , ab : *mut i32 ) -> QStatus );
    alljoyn_msgarg_get_bool_array(arg.into_param().abi(), length, ab)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_double<P0>(arg: P0, d: *mut f64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_double ( arg : alljoyn_msgarg , d : *mut f64 ) -> QStatus );
    alljoyn_msgarg_get_double(arg.into_param().abi(), d)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_double_array<P0>(arg: P0, length: *mut usize, ad: *mut f64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_double_array ( arg : alljoyn_msgarg , length : *mut usize , ad : *mut f64 ) -> QStatus );
    alljoyn_msgarg_get_double_array(arg.into_param().abi(), length, ad)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_int16<P0>(arg: P0, n: *mut i16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_int16 ( arg : alljoyn_msgarg , n : *mut i16 ) -> QStatus );
    alljoyn_msgarg_get_int16(arg.into_param().abi(), n)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_int16_array<P0>(arg: P0, length: *mut usize, an: *mut i16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_int16_array ( arg : alljoyn_msgarg , length : *mut usize , an : *mut i16 ) -> QStatus );
    alljoyn_msgarg_get_int16_array(arg.into_param().abi(), length, an)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_int32<P0>(arg: P0, i: *mut i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_int32 ( arg : alljoyn_msgarg , i : *mut i32 ) -> QStatus );
    alljoyn_msgarg_get_int32(arg.into_param().abi(), i)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_int32_array<P0>(arg: P0, length: *mut usize, ai: *mut i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_int32_array ( arg : alljoyn_msgarg , length : *mut usize , ai : *mut i32 ) -> QStatus );
    alljoyn_msgarg_get_int32_array(arg.into_param().abi(), length, ai)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_int64<P0>(arg: P0, x: *mut i64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_int64 ( arg : alljoyn_msgarg , x : *mut i64 ) -> QStatus );
    alljoyn_msgarg_get_int64(arg.into_param().abi(), x)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_int64_array<P0>(arg: P0, length: *mut usize, ax: *mut i64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_int64_array ( arg : alljoyn_msgarg , length : *mut usize , ax : *mut i64 ) -> QStatus );
    alljoyn_msgarg_get_int64_array(arg.into_param().abi(), length, ax)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_objectpath<P0>(arg: P0, o: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_objectpath ( arg : alljoyn_msgarg , o : *mut *mut i8 ) -> QStatus );
    alljoyn_msgarg_get_objectpath(arg.into_param().abi(), o)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_signature<P0>(arg: P0, g: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_signature ( arg : alljoyn_msgarg , g : *mut *mut i8 ) -> QStatus );
    alljoyn_msgarg_get_signature(arg.into_param().abi(), g)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_string<P0>(arg: P0, s: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_string ( arg : alljoyn_msgarg , s : *mut *mut i8 ) -> QStatus );
    alljoyn_msgarg_get_string(arg.into_param().abi(), s)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint16<P0>(arg: P0, q: *mut u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_uint16 ( arg : alljoyn_msgarg , q : *mut u16 ) -> QStatus );
    alljoyn_msgarg_get_uint16(arg.into_param().abi(), q)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint16_array<P0>(arg: P0, length: *mut usize, aq: *mut u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_uint16_array ( arg : alljoyn_msgarg , length : *mut usize , aq : *mut u16 ) -> QStatus );
    alljoyn_msgarg_get_uint16_array(arg.into_param().abi(), length, aq)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint32<P0>(arg: P0, u: *mut u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_uint32 ( arg : alljoyn_msgarg , u : *mut u32 ) -> QStatus );
    alljoyn_msgarg_get_uint32(arg.into_param().abi(), u)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint32_array<P0>(arg: P0, length: *mut usize, au: *mut u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_uint32_array ( arg : alljoyn_msgarg , length : *mut usize , au : *mut u32 ) -> QStatus );
    alljoyn_msgarg_get_uint32_array(arg.into_param().abi(), length, au)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint64<P0>(arg: P0, t: *mut u64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_uint64 ( arg : alljoyn_msgarg , t : *mut u64 ) -> QStatus );
    alljoyn_msgarg_get_uint64(arg.into_param().abi(), t)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint64_array<P0>(arg: P0, length: *mut usize, at: *mut u64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_uint64_array ( arg : alljoyn_msgarg , length : *mut usize , at : *mut u64 ) -> QStatus );
    alljoyn_msgarg_get_uint64_array(arg.into_param().abi(), length, at)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint8<P0>(arg: P0, y: *mut u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_uint8 ( arg : alljoyn_msgarg , y : *mut u8 ) -> QStatus );
    alljoyn_msgarg_get_uint8(arg.into_param().abi(), y)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint8_array<P0>(arg: P0, length: *mut usize, ay: *mut u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_uint8_array ( arg : alljoyn_msgarg , length : *mut usize , ay : *mut u8 ) -> QStatus );
    alljoyn_msgarg_get_uint8_array(arg.into_param().abi(), length, ay)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_variant<P0, P1>(arg: P0, v: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_variant ( arg : alljoyn_msgarg , v : alljoyn_msgarg ) -> QStatus );
    alljoyn_msgarg_get_variant(arg.into_param().abi(), v.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_get_variant_array<P0, P1>(arg: P0, signature: P1, length: *mut usize, av: *mut alljoyn_msgarg) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_get_variant_array ( arg : alljoyn_msgarg , signature : ::windows::core::PCSTR , length : *mut usize , av : *mut alljoyn_msgarg ) -> QStatus );
    alljoyn_msgarg_get_variant_array(arg.into_param().abi(), signature.into_param().abi(), length, av)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_getdictelement<P0, P1>(arg: P0, elemsig: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""cdecl" fn alljoyn_msgarg_getdictelement ( arg : alljoyn_msgarg , elemsig : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_msgarg_getdictelement(arg.into_param().abi(), elemsig.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_getkey<P0>(arg: P0) -> alljoyn_msgarg
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_getkey ( arg : alljoyn_msgarg ) -> alljoyn_msgarg );
    alljoyn_msgarg_getkey(arg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_getmember<P0>(arg: P0, index: usize) -> alljoyn_msgarg
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_getmember ( arg : alljoyn_msgarg , index : usize ) -> alljoyn_msgarg );
    alljoyn_msgarg_getmember(arg.into_param().abi(), index)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_getnummembers<P0>(arg: P0) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_getnummembers ( arg : alljoyn_msgarg ) -> usize );
    alljoyn_msgarg_getnummembers(arg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_gettype<P0>(arg: P0) -> alljoyn_typeid
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_gettype ( arg : alljoyn_msgarg ) -> alljoyn_typeid );
    alljoyn_msgarg_gettype(arg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_getvalue<P0>(arg: P0) -> alljoyn_msgarg
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_getvalue ( arg : alljoyn_msgarg ) -> alljoyn_msgarg );
    alljoyn_msgarg_getvalue(arg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_hassignature<P0, P1>(arg: P0, signature: P1) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_hassignature ( arg : alljoyn_msgarg , signature : ::windows::core::PCSTR ) -> i32 );
    alljoyn_msgarg_hassignature(arg.into_param().abi(), signature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set<P0, P1>(arg: P0, signature: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""cdecl" fn alljoyn_msgarg_set ( arg : alljoyn_msgarg , signature : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_msgarg_set(arg.into_param().abi(), signature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_and_stabilize<P0, P1>(arg: P0, signature: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""cdecl" fn alljoyn_msgarg_set_and_stabilize ( arg : alljoyn_msgarg , signature : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_msgarg_set_and_stabilize(arg.into_param().abi(), signature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_bool<P0>(arg: P0, b: i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_bool ( arg : alljoyn_msgarg , b : i32 ) -> QStatus );
    alljoyn_msgarg_set_bool(arg.into_param().abi(), b)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_bool_array<P0>(arg: P0, length: usize, ab: *mut i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_bool_array ( arg : alljoyn_msgarg , length : usize , ab : *mut i32 ) -> QStatus );
    alljoyn_msgarg_set_bool_array(arg.into_param().abi(), length, ab)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_double<P0>(arg: P0, d: f64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_double ( arg : alljoyn_msgarg , d : f64 ) -> QStatus );
    alljoyn_msgarg_set_double(arg.into_param().abi(), d)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_double_array<P0>(arg: P0, length: usize, ad: *mut f64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_double_array ( arg : alljoyn_msgarg , length : usize , ad : *mut f64 ) -> QStatus );
    alljoyn_msgarg_set_double_array(arg.into_param().abi(), length, ad)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_int16<P0>(arg: P0, n: i16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_int16 ( arg : alljoyn_msgarg , n : i16 ) -> QStatus );
    alljoyn_msgarg_set_int16(arg.into_param().abi(), n)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_int16_array<P0>(arg: P0, length: usize, an: *mut i16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_int16_array ( arg : alljoyn_msgarg , length : usize , an : *mut i16 ) -> QStatus );
    alljoyn_msgarg_set_int16_array(arg.into_param().abi(), length, an)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_int32<P0>(arg: P0, i: i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_int32 ( arg : alljoyn_msgarg , i : i32 ) -> QStatus );
    alljoyn_msgarg_set_int32(arg.into_param().abi(), i)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_int32_array<P0>(arg: P0, length: usize, ai: *mut i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_int32_array ( arg : alljoyn_msgarg , length : usize , ai : *mut i32 ) -> QStatus );
    alljoyn_msgarg_set_int32_array(arg.into_param().abi(), length, ai)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_int64<P0>(arg: P0, x: i64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_int64 ( arg : alljoyn_msgarg , x : i64 ) -> QStatus );
    alljoyn_msgarg_set_int64(arg.into_param().abi(), x)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_int64_array<P0>(arg: P0, length: usize, ax: *mut i64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_int64_array ( arg : alljoyn_msgarg , length : usize , ax : *mut i64 ) -> QStatus );
    alljoyn_msgarg_set_int64_array(arg.into_param().abi(), length, ax)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_objectpath<P0, P1>(arg: P0, o: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_objectpath ( arg : alljoyn_msgarg , o : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_msgarg_set_objectpath(arg.into_param().abi(), o.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_objectpath_array<P0>(arg: P0, length: usize, ao: *const *const i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_objectpath_array ( arg : alljoyn_msgarg , length : usize , ao : *const *const i8 ) -> QStatus );
    alljoyn_msgarg_set_objectpath_array(arg.into_param().abi(), length, ao)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_signature<P0, P1>(arg: P0, g: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_signature ( arg : alljoyn_msgarg , g : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_msgarg_set_signature(arg.into_param().abi(), g.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_signature_array<P0>(arg: P0, length: usize, ag: *const *const i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_signature_array ( arg : alljoyn_msgarg , length : usize , ag : *const *const i8 ) -> QStatus );
    alljoyn_msgarg_set_signature_array(arg.into_param().abi(), length, ag)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_string<P0, P1>(arg: P0, s: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_string ( arg : alljoyn_msgarg , s : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_msgarg_set_string(arg.into_param().abi(), s.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_string_array<P0>(arg: P0, length: usize, r#as: *const *const i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_string_array ( arg : alljoyn_msgarg , length : usize , r#as : *const *const i8 ) -> QStatus );
    alljoyn_msgarg_set_string_array(arg.into_param().abi(), length, r#as)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint16<P0>(arg: P0, q: u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_uint16 ( arg : alljoyn_msgarg , q : u16 ) -> QStatus );
    alljoyn_msgarg_set_uint16(arg.into_param().abi(), q)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint16_array<P0>(arg: P0, length: usize, aq: *mut u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_uint16_array ( arg : alljoyn_msgarg , length : usize , aq : *mut u16 ) -> QStatus );
    alljoyn_msgarg_set_uint16_array(arg.into_param().abi(), length, aq)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint32<P0>(arg: P0, u: u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_uint32 ( arg : alljoyn_msgarg , u : u32 ) -> QStatus );
    alljoyn_msgarg_set_uint32(arg.into_param().abi(), u)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint32_array<P0>(arg: P0, length: usize, au: *mut u32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_uint32_array ( arg : alljoyn_msgarg , length : usize , au : *mut u32 ) -> QStatus );
    alljoyn_msgarg_set_uint32_array(arg.into_param().abi(), length, au)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint64<P0>(arg: P0, t: u64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_uint64 ( arg : alljoyn_msgarg , t : u64 ) -> QStatus );
    alljoyn_msgarg_set_uint64(arg.into_param().abi(), t)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint64_array<P0>(arg: P0, length: usize, at: *mut u64) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_uint64_array ( arg : alljoyn_msgarg , length : usize , at : *mut u64 ) -> QStatus );
    alljoyn_msgarg_set_uint64_array(arg.into_param().abi(), length, at)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint8<P0>(arg: P0, y: u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_uint8 ( arg : alljoyn_msgarg , y : u8 ) -> QStatus );
    alljoyn_msgarg_set_uint8(arg.into_param().abi(), y)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint8_array<P0>(arg: P0, length: usize, ay: *mut u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_set_uint8_array ( arg : alljoyn_msgarg , length : usize , ay : *mut u8 ) -> QStatus );
    alljoyn_msgarg_set_uint8_array(arg.into_param().abi(), length, ay)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_setdictentry<P0, P1, P2>(arg: P0, key: P1, value: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
    P2: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_setdictentry ( arg : alljoyn_msgarg , key : alljoyn_msgarg , value : alljoyn_msgarg ) -> QStatus );
    alljoyn_msgarg_setdictentry(arg.into_param().abi(), key.into_param().abi(), value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_setstruct<P0, P1>(arg: P0, struct_members: P1, num_members: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_setstruct ( arg : alljoyn_msgarg , struct_members : alljoyn_msgarg , num_members : usize ) -> QStatus );
    alljoyn_msgarg_setstruct(arg.into_param().abi(), struct_members.into_param().abi(), num_members)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_signature<P0, P1>(arg: P0, str: P1, buf: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_signature ( arg : alljoyn_msgarg , str : ::windows::core::PCSTR , buf : usize ) -> usize );
    alljoyn_msgarg_signature(arg.into_param().abi(), str.into_param().abi(), buf)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_stabilize<P0>(arg: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_stabilize ( arg : alljoyn_msgarg ) -> ( ) );
    alljoyn_msgarg_stabilize(arg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_msgarg_tostring<P0, P1>(arg: P0, str: P1, buf: usize, indent: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_msgarg>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_msgarg_tostring ( arg : alljoyn_msgarg , str : ::windows::core::PCSTR , buf : usize , indent : usize ) -> usize );
    alljoyn_msgarg_tostring(arg.into_param().abi(), str.into_param().abi(), buf, indent)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_observer_create<P0>(bus: P0, mandatoryinterfaces: *const *const i8, nummandatoryinterfaces: usize) -> alljoyn_observer
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_observer_create ( bus : alljoyn_busattachment , mandatoryinterfaces : *const *const i8 , nummandatoryinterfaces : usize ) -> alljoyn_observer );
    alljoyn_observer_create(bus.into_param().abi(), mandatoryinterfaces, nummandatoryinterfaces)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_observer_destroy<P0>(observer: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_observer>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_observer_destroy ( observer : alljoyn_observer ) -> ( ) );
    alljoyn_observer_destroy(observer.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_observer_get<P0, P1, P2>(observer: P0, uniquebusname: P1, objectpath: P2) -> alljoyn_proxybusobject_ref
where
    P0: ::windows::core::IntoParam<alljoyn_observer>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_observer_get ( observer : alljoyn_observer , uniquebusname : ::windows::core::PCSTR , objectpath : ::windows::core::PCSTR ) -> alljoyn_proxybusobject_ref );
    alljoyn_observer_get(observer.into_param().abi(), uniquebusname.into_param().abi(), objectpath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_observer_getfirst<P0>(observer: P0) -> alljoyn_proxybusobject_ref
where
    P0: ::windows::core::IntoParam<alljoyn_observer>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_observer_getfirst ( observer : alljoyn_observer ) -> alljoyn_proxybusobject_ref );
    alljoyn_observer_getfirst(observer.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_observer_getnext<P0, P1>(observer: P0, proxyref: P1) -> alljoyn_proxybusobject_ref
where
    P0: ::windows::core::IntoParam<alljoyn_observer>,
    P1: ::windows::core::IntoParam<alljoyn_proxybusobject_ref>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_observer_getnext ( observer : alljoyn_observer , proxyref : alljoyn_proxybusobject_ref ) -> alljoyn_proxybusobject_ref );
    alljoyn_observer_getnext(observer.into_param().abi(), proxyref.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_observer_registerlistener<P0, P1>(observer: P0, listener: P1, triggeronexisting: i32)
where
    P0: ::windows::core::IntoParam<alljoyn_observer>,
    P1: ::windows::core::IntoParam<alljoyn_observerlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_observer_registerlistener ( observer : alljoyn_observer , listener : alljoyn_observerlistener , triggeronexisting : i32 ) -> ( ) );
    alljoyn_observer_registerlistener(observer.into_param().abi(), listener.into_param().abi(), triggeronexisting)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_observer_unregisteralllisteners<P0>(observer: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_observer>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_observer_unregisteralllisteners ( observer : alljoyn_observer ) -> ( ) );
    alljoyn_observer_unregisteralllisteners(observer.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_observer_unregisterlistener<P0, P1>(observer: P0, listener: P1)
where
    P0: ::windows::core::IntoParam<alljoyn_observer>,
    P1: ::windows::core::IntoParam<alljoyn_observerlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_observer_unregisterlistener ( observer : alljoyn_observer , listener : alljoyn_observerlistener ) -> ( ) );
    alljoyn_observer_unregisterlistener(observer.into_param().abi(), listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_observerlistener_create(callback: *const alljoyn_observerlistener_callback, context: *const ::core::ffi::c_void) -> alljoyn_observerlistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_observerlistener_create ( callback : *const alljoyn_observerlistener_callback , context : *const ::core::ffi::c_void ) -> alljoyn_observerlistener );
    alljoyn_observerlistener_create(callback, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_observerlistener_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_observerlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_observerlistener_destroy ( listener : alljoyn_observerlistener ) -> ( ) );
    alljoyn_observerlistener_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_passwordmanager_setcredentials<P0, P1>(authmechanism: P0, password: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_passwordmanager_setcredentials ( authmechanism : ::windows::core::PCSTR , password : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_passwordmanager_setcredentials(authmechanism.into_param().abi(), password.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurationlistener_create(callbacks: *const alljoyn_permissionconfigurationlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_permissionconfigurationlistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurationlistener_create ( callbacks : *const alljoyn_permissionconfigurationlistener_callbacks , context : *const ::core::ffi::c_void ) -> alljoyn_permissionconfigurationlistener );
    alljoyn_permissionconfigurationlistener_create(callbacks, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurationlistener_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurationlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurationlistener_destroy ( listener : alljoyn_permissionconfigurationlistener ) -> ( ) );
    alljoyn_permissionconfigurationlistener_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_certificatechain_destroy(certificatechain: *mut i8) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_certificatechain_destroy ( certificatechain : *mut i8 ) -> ( ) );
    alljoyn_permissionconfigurator_certificatechain_destroy(certificatechain)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_certificateid_cleanup(certificateid: *mut alljoyn_certificateid) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_certificateid_cleanup ( certificateid : *mut alljoyn_certificateid ) -> ( ) );
    alljoyn_permissionconfigurator_certificateid_cleanup(certificateid)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_certificateidarray_cleanup(certificateidarray: *mut alljoyn_certificateidarray) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_certificateidarray_cleanup ( certificateidarray : *mut alljoyn_certificateidarray ) -> ( ) );
    alljoyn_permissionconfigurator_certificateidarray_cleanup(certificateidarray)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_claim<P0>(configurator: P0, cakey: *mut i8, identitycertificatechain: *mut i8, groupid: *const u8, groupsize: usize, groupauthority: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_claim ( configurator : alljoyn_permissionconfigurator , cakey : *mut i8 , identitycertificatechain : *mut i8 , groupid : *const u8 , groupsize : usize , groupauthority : *mut i8 , manifestsxmls : *mut *mut i8 , manifestscount : usize ) -> QStatus );
    alljoyn_permissionconfigurator_claim(configurator.into_param().abi(), cakey, identitycertificatechain, groupid, groupsize, groupauthority, manifestsxmls, manifestscount)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_endmanagement<P0>(configurator: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_endmanagement ( configurator : alljoyn_permissionconfigurator ) -> QStatus );
    alljoyn_permissionconfigurator_endmanagement(configurator.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getapplicationstate<P0>(configurator: P0, state: *mut alljoyn_applicationstate) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getapplicationstate ( configurator : alljoyn_permissionconfigurator , state : *mut alljoyn_applicationstate ) -> QStatus );
    alljoyn_permissionconfigurator_getapplicationstate(configurator.into_param().abi(), state)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getclaimcapabilities<P0>(configurator: P0, claimcapabilities: *mut u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getclaimcapabilities ( configurator : alljoyn_permissionconfigurator , claimcapabilities : *mut u16 ) -> QStatus );
    alljoyn_permissionconfigurator_getclaimcapabilities(configurator.into_param().abi(), claimcapabilities)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getclaimcapabilitiesadditionalinfo<P0>(configurator: P0, additionalinfo: *mut u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getclaimcapabilitiesadditionalinfo ( configurator : alljoyn_permissionconfigurator , additionalinfo : *mut u16 ) -> QStatus );
    alljoyn_permissionconfigurator_getclaimcapabilitiesadditionalinfo(configurator.into_param().abi(), additionalinfo)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getdefaultclaimcapabilities() -> u16 {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getdefaultclaimcapabilities ( ) -> u16 );
    alljoyn_permissionconfigurator_getdefaultclaimcapabilities()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getdefaultpolicy<P0>(configurator: P0, policyxml: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getdefaultpolicy ( configurator : alljoyn_permissionconfigurator , policyxml : *mut *mut i8 ) -> QStatus );
    alljoyn_permissionconfigurator_getdefaultpolicy(configurator.into_param().abi(), policyxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getidentity<P0>(configurator: P0, identitycertificatechain: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getidentity ( configurator : alljoyn_permissionconfigurator , identitycertificatechain : *mut *mut i8 ) -> QStatus );
    alljoyn_permissionconfigurator_getidentity(configurator.into_param().abi(), identitycertificatechain)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getidentitycertificateid<P0>(configurator: P0, certificateid: *mut alljoyn_certificateid) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getidentitycertificateid ( configurator : alljoyn_permissionconfigurator , certificateid : *mut alljoyn_certificateid ) -> QStatus );
    alljoyn_permissionconfigurator_getidentitycertificateid(configurator.into_param().abi(), certificateid)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getmanifests<P0>(configurator: P0, manifestarray: *mut alljoyn_manifestarray) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getmanifests ( configurator : alljoyn_permissionconfigurator , manifestarray : *mut alljoyn_manifestarray ) -> QStatus );
    alljoyn_permissionconfigurator_getmanifests(configurator.into_param().abi(), manifestarray)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getmanifesttemplate<P0>(configurator: P0, manifesttemplatexml: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getmanifesttemplate ( configurator : alljoyn_permissionconfigurator , manifesttemplatexml : *mut *mut i8 ) -> QStatus );
    alljoyn_permissionconfigurator_getmanifesttemplate(configurator.into_param().abi(), manifesttemplatexml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getmembershipsummaries<P0>(configurator: P0, certificateids: *mut alljoyn_certificateidarray) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getmembershipsummaries ( configurator : alljoyn_permissionconfigurator , certificateids : *mut alljoyn_certificateidarray ) -> QStatus );
    alljoyn_permissionconfigurator_getmembershipsummaries(configurator.into_param().abi(), certificateids)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getpolicy<P0>(configurator: P0, policyxml: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getpolicy ( configurator : alljoyn_permissionconfigurator , policyxml : *mut *mut i8 ) -> QStatus );
    alljoyn_permissionconfigurator_getpolicy(configurator.into_param().abi(), policyxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getpublickey<P0>(configurator: P0, publickey: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_getpublickey ( configurator : alljoyn_permissionconfigurator , publickey : *mut *mut i8 ) -> QStatus );
    alljoyn_permissionconfigurator_getpublickey(configurator.into_param().abi(), publickey)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_installmanifests<P0>(configurator: P0, manifestsxmls: *mut *mut i8, manifestscount: usize, append: i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_installmanifests ( configurator : alljoyn_permissionconfigurator , manifestsxmls : *mut *mut i8 , manifestscount : usize , append : i32 ) -> QStatus );
    alljoyn_permissionconfigurator_installmanifests(configurator.into_param().abi(), manifestsxmls, manifestscount, append)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_installmembership<P0>(configurator: P0, membershipcertificatechain: *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_installmembership ( configurator : alljoyn_permissionconfigurator , membershipcertificatechain : *mut i8 ) -> QStatus );
    alljoyn_permissionconfigurator_installmembership(configurator.into_param().abi(), membershipcertificatechain)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_manifestarray_cleanup(manifestarray: *mut alljoyn_manifestarray) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_manifestarray_cleanup ( manifestarray : *mut alljoyn_manifestarray ) -> ( ) );
    alljoyn_permissionconfigurator_manifestarray_cleanup(manifestarray)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_manifesttemplate_destroy(manifesttemplatexml: *mut i8) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_manifesttemplate_destroy ( manifesttemplatexml : *mut i8 ) -> ( ) );
    alljoyn_permissionconfigurator_manifesttemplate_destroy(manifesttemplatexml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_policy_destroy(policyxml: *mut i8) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_policy_destroy ( policyxml : *mut i8 ) -> ( ) );
    alljoyn_permissionconfigurator_policy_destroy(policyxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_publickey_destroy(publickey: *mut i8) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_publickey_destroy ( publickey : *mut i8 ) -> ( ) );
    alljoyn_permissionconfigurator_publickey_destroy(publickey)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_removemembership<P0>(configurator: P0, serial: *const u8, seriallen: usize, issuerpublickey: *mut i8, issueraki: *const u8, issuerakilen: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_removemembership ( configurator : alljoyn_permissionconfigurator , serial : *const u8 , seriallen : usize , issuerpublickey : *mut i8 , issueraki : *const u8 , issuerakilen : usize ) -> QStatus );
    alljoyn_permissionconfigurator_removemembership(configurator.into_param().abi(), serial, seriallen, issuerpublickey, issueraki, issuerakilen)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_reset<P0>(configurator: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_reset ( configurator : alljoyn_permissionconfigurator ) -> QStatus );
    alljoyn_permissionconfigurator_reset(configurator.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_resetpolicy<P0>(configurator: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_resetpolicy ( configurator : alljoyn_permissionconfigurator ) -> QStatus );
    alljoyn_permissionconfigurator_resetpolicy(configurator.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_setapplicationstate<P0>(configurator: P0, state: alljoyn_applicationstate) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_setapplicationstate ( configurator : alljoyn_permissionconfigurator , state : alljoyn_applicationstate ) -> QStatus );
    alljoyn_permissionconfigurator_setapplicationstate(configurator.into_param().abi(), state)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_setclaimcapabilities<P0>(configurator: P0, claimcapabilities: u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_setclaimcapabilities ( configurator : alljoyn_permissionconfigurator , claimcapabilities : u16 ) -> QStatus );
    alljoyn_permissionconfigurator_setclaimcapabilities(configurator.into_param().abi(), claimcapabilities)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_setclaimcapabilitiesadditionalinfo<P0>(configurator: P0, additionalinfo: u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_setclaimcapabilitiesadditionalinfo ( configurator : alljoyn_permissionconfigurator , additionalinfo : u16 ) -> QStatus );
    alljoyn_permissionconfigurator_setclaimcapabilitiesadditionalinfo(configurator.into_param().abi(), additionalinfo)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_setmanifesttemplatefromxml<P0>(configurator: P0, manifesttemplatexml: *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_setmanifesttemplatefromxml ( configurator : alljoyn_permissionconfigurator , manifesttemplatexml : *mut i8 ) -> QStatus );
    alljoyn_permissionconfigurator_setmanifesttemplatefromxml(configurator.into_param().abi(), manifesttemplatexml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_startmanagement<P0>(configurator: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_startmanagement ( configurator : alljoyn_permissionconfigurator ) -> QStatus );
    alljoyn_permissionconfigurator_startmanagement(configurator.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_updateidentity<P0>(configurator: P0, identitycertificatechain: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_updateidentity ( configurator : alljoyn_permissionconfigurator , identitycertificatechain : *mut i8 , manifestsxmls : *mut *mut i8 , manifestscount : usize ) -> QStatus );
    alljoyn_permissionconfigurator_updateidentity(configurator.into_param().abi(), identitycertificatechain, manifestsxmls, manifestscount)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_updatepolicy<P0>(configurator: P0, policyxml: *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_permissionconfigurator>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_permissionconfigurator_updatepolicy ( configurator : alljoyn_permissionconfigurator , policyxml : *mut i8 ) -> QStatus );
    alljoyn_permissionconfigurator_updatepolicy(configurator.into_param().abi(), policyxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_pinglistener_create(callback: *const alljoyn_pinglistener_callback, context: *const ::core::ffi::c_void) -> alljoyn_pinglistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_pinglistener_create ( callback : *const alljoyn_pinglistener_callback , context : *const ::core::ffi::c_void ) -> alljoyn_pinglistener );
    alljoyn_pinglistener_create(callback, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_pinglistener_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_pinglistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_pinglistener_destroy ( listener : alljoyn_pinglistener ) -> ( ) );
    alljoyn_pinglistener_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_addchild<P0, P1>(proxyobj: P0, child: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_addchild ( proxyobj : alljoyn_proxybusobject , child : alljoyn_proxybusobject ) -> QStatus );
    alljoyn_proxybusobject_addchild(proxyobj.into_param().abi(), child.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_addinterface<P0, P1>(proxyobj: P0, iface: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<alljoyn_interfacedescription>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_addinterface ( proxyobj : alljoyn_proxybusobject , iface : alljoyn_interfacedescription ) -> QStatus );
    alljoyn_proxybusobject_addinterface(proxyobj.into_param().abi(), iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_addinterface_by_name<P0, P1>(proxyobj: P0, name: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_addinterface_by_name ( proxyobj : alljoyn_proxybusobject , name : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_proxybusobject_addinterface_by_name(proxyobj.into_param().abi(), name.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_copy<P0>(source: P0) -> alljoyn_proxybusobject
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_copy ( source : alljoyn_proxybusobject ) -> alljoyn_proxybusobject );
    alljoyn_proxybusobject_copy(source.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_create<P0, P1, P2>(bus: P0, service: P1, path: P2, sessionid: u32) -> alljoyn_proxybusobject
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_create ( bus : alljoyn_busattachment , service : ::windows::core::PCSTR , path : ::windows::core::PCSTR , sessionid : u32 ) -> alljoyn_proxybusobject );
    alljoyn_proxybusobject_create(bus.into_param().abi(), service.into_param().abi(), path.into_param().abi(), sessionid)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_create_secure<P0, P1, P2>(bus: P0, service: P1, path: P2, sessionid: u32) -> alljoyn_proxybusobject
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_create_secure ( bus : alljoyn_busattachment , service : ::windows::core::PCSTR , path : ::windows::core::PCSTR , sessionid : u32 ) -> alljoyn_proxybusobject );
    alljoyn_proxybusobject_create_secure(bus.into_param().abi(), service.into_param().abi(), path.into_param().abi(), sessionid)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_destroy<P0>(proxyobj: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_destroy ( proxyobj : alljoyn_proxybusobject ) -> ( ) );
    alljoyn_proxybusobject_destroy(proxyobj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_enablepropertycaching<P0>(proxyobj: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_enablepropertycaching ( proxyobj : alljoyn_proxybusobject ) -> ( ) );
    alljoyn_proxybusobject_enablepropertycaching(proxyobj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getallproperties<P0, P1, P2>(proxyobj: P0, iface: P1, values: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getallproperties ( proxyobj : alljoyn_proxybusobject , iface : ::windows::core::PCSTR , values : alljoyn_msgarg ) -> QStatus );
    alljoyn_proxybusobject_getallproperties(proxyobj.into_param().abi(), iface.into_param().abi(), values.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getallpropertiesasync<P0, P1>(proxyobj: P0, iface: P1, callback: alljoyn_proxybusobject_listener_getallpropertiescb_ptr, timeout: u32, context: *mut ::core::ffi::c_void) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getallpropertiesasync ( proxyobj : alljoyn_proxybusobject , iface : ::windows::core::PCSTR , callback : alljoyn_proxybusobject_listener_getallpropertiescb_ptr , timeout : u32 , context : *mut ::core::ffi::c_void ) -> QStatus );
    alljoyn_proxybusobject_getallpropertiesasync(proxyobj.into_param().abi(), iface.into_param().abi(), callback, timeout, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getchild<P0, P1>(proxyobj: P0, path: P1) -> alljoyn_proxybusobject
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getchild ( proxyobj : alljoyn_proxybusobject , path : ::windows::core::PCSTR ) -> alljoyn_proxybusobject );
    alljoyn_proxybusobject_getchild(proxyobj.into_param().abi(), path.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getchildren<P0>(proxyobj: P0, children: *mut alljoyn_proxybusobject, numchildren: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getchildren ( proxyobj : alljoyn_proxybusobject , children : *mut alljoyn_proxybusobject , numchildren : usize ) -> usize );
    alljoyn_proxybusobject_getchildren(proxyobj.into_param().abi(), children, numchildren)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getinterface<P0, P1>(proxyobj: P0, iface: P1) -> alljoyn_interfacedescription
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getinterface ( proxyobj : alljoyn_proxybusobject , iface : ::windows::core::PCSTR ) -> alljoyn_interfacedescription );
    alljoyn_proxybusobject_getinterface(proxyobj.into_param().abi(), iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getinterfaces<P0>(proxyobj: P0, ifaces: *const alljoyn_interfacedescription, numifaces: usize) -> usize
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getinterfaces ( proxyobj : alljoyn_proxybusobject , ifaces : *const alljoyn_interfacedescription , numifaces : usize ) -> usize );
    alljoyn_proxybusobject_getinterfaces(proxyobj.into_param().abi(), ifaces, numifaces)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getpath<P0>(proxyobj: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getpath ( proxyobj : alljoyn_proxybusobject ) -> ::windows::core::PCSTR );
    alljoyn_proxybusobject_getpath(proxyobj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getproperty<P0, P1, P2, P3>(proxyobj: P0, iface: P1, property: P2, value: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getproperty ( proxyobj : alljoyn_proxybusobject , iface : ::windows::core::PCSTR , property : ::windows::core::PCSTR , value : alljoyn_msgarg ) -> QStatus );
    alljoyn_proxybusobject_getproperty(proxyobj.into_param().abi(), iface.into_param().abi(), property.into_param().abi(), value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getpropertyasync<P0, P1, P2>(proxyobj: P0, iface: P1, property: P2, callback: alljoyn_proxybusobject_listener_getpropertycb_ptr, timeout: u32, context: *mut ::core::ffi::c_void) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getpropertyasync ( proxyobj : alljoyn_proxybusobject , iface : ::windows::core::PCSTR , property : ::windows::core::PCSTR , callback : alljoyn_proxybusobject_listener_getpropertycb_ptr , timeout : u32 , context : *mut ::core::ffi::c_void ) -> QStatus );
    alljoyn_proxybusobject_getpropertyasync(proxyobj.into_param().abi(), iface.into_param().abi(), property.into_param().abi(), callback, timeout, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getservicename<P0>(proxyobj: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getservicename ( proxyobj : alljoyn_proxybusobject ) -> ::windows::core::PCSTR );
    alljoyn_proxybusobject_getservicename(proxyobj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getsessionid<P0>(proxyobj: P0) -> u32
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getsessionid ( proxyobj : alljoyn_proxybusobject ) -> u32 );
    alljoyn_proxybusobject_getsessionid(proxyobj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_getuniquename<P0>(proxyobj: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_getuniquename ( proxyobj : alljoyn_proxybusobject ) -> ::windows::core::PCSTR );
    alljoyn_proxybusobject_getuniquename(proxyobj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_implementsinterface<P0, P1>(proxyobj: P0, iface: P1) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_implementsinterface ( proxyobj : alljoyn_proxybusobject , iface : ::windows::core::PCSTR ) -> i32 );
    alljoyn_proxybusobject_implementsinterface(proxyobj.into_param().abi(), iface.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_introspectremoteobject<P0>(proxyobj: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_introspectremoteobject ( proxyobj : alljoyn_proxybusobject ) -> QStatus );
    alljoyn_proxybusobject_introspectremoteobject(proxyobj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_introspectremoteobjectasync<P0>(proxyobj: P0, callback: alljoyn_proxybusobject_listener_introspectcb_ptr, context: *mut ::core::ffi::c_void) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_introspectremoteobjectasync ( proxyobj : alljoyn_proxybusobject , callback : alljoyn_proxybusobject_listener_introspectcb_ptr , context : *mut ::core::ffi::c_void ) -> QStatus );
    alljoyn_proxybusobject_introspectremoteobjectasync(proxyobj.into_param().abi(), callback, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_issecure<P0>(proxyobj: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_issecure ( proxyobj : alljoyn_proxybusobject ) -> i32 );
    alljoyn_proxybusobject_issecure(proxyobj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_isvalid<P0>(proxyobj: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_isvalid ( proxyobj : alljoyn_proxybusobject ) -> i32 );
    alljoyn_proxybusobject_isvalid(proxyobj.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcall<P0, P1, P2, P3, P4>(proxyobj: P0, ifacename: P1, methodname: P2, args: P3, numargs: usize, replymsg: P4, timeout: u32, flags: u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<alljoyn_msgarg>,
    P4: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_methodcall ( proxyobj : alljoyn_proxybusobject , ifacename : ::windows::core::PCSTR , methodname : ::windows::core::PCSTR , args : alljoyn_msgarg , numargs : usize , replymsg : alljoyn_message , timeout : u32 , flags : u8 ) -> QStatus );
    alljoyn_proxybusobject_methodcall(proxyobj.into_param().abi(), ifacename.into_param().abi(), methodname.into_param().abi(), args.into_param().abi(), numargs, replymsg.into_param().abi(), timeout, flags)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcall_member<P0, P1, P2>(proxyobj: P0, method: alljoyn_interfacedescription_member, args: P1, numargs: usize, replymsg: P2, timeout: u32, flags: u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
    P2: ::windows::core::IntoParam<alljoyn_message>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_methodcall_member ( proxyobj : alljoyn_proxybusobject , method : alljoyn_interfacedescription_member , args : alljoyn_msgarg , numargs : usize , replymsg : alljoyn_message , timeout : u32 , flags : u8 ) -> QStatus );
    alljoyn_proxybusobject_methodcall_member(proxyobj.into_param().abi(), ::core::mem::transmute(method), args.into_param().abi(), numargs, replymsg.into_param().abi(), timeout, flags)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcall_member_noreply<P0, P1>(proxyobj: P0, method: alljoyn_interfacedescription_member, args: P1, numargs: usize, flags: u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_methodcall_member_noreply ( proxyobj : alljoyn_proxybusobject , method : alljoyn_interfacedescription_member , args : alljoyn_msgarg , numargs : usize , flags : u8 ) -> QStatus );
    alljoyn_proxybusobject_methodcall_member_noreply(proxyobj.into_param().abi(), ::core::mem::transmute(method), args.into_param().abi(), numargs, flags)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcall_noreply<P0, P1, P2, P3>(proxyobj: P0, ifacename: P1, methodname: P2, args: P3, numargs: usize, flags: u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_methodcall_noreply ( proxyobj : alljoyn_proxybusobject , ifacename : ::windows::core::PCSTR , methodname : ::windows::core::PCSTR , args : alljoyn_msgarg , numargs : usize , flags : u8 ) -> QStatus );
    alljoyn_proxybusobject_methodcall_noreply(proxyobj.into_param().abi(), ifacename.into_param().abi(), methodname.into_param().abi(), args.into_param().abi(), numargs, flags)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcallasync<P0, P1, P2, P3>(proxyobj: P0, ifacename: P1, methodname: P2, replyfunc: alljoyn_messagereceiver_replyhandler_ptr, args: P3, numargs: usize, context: *mut ::core::ffi::c_void, timeout: u32, flags: u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_methodcallasync ( proxyobj : alljoyn_proxybusobject , ifacename : ::windows::core::PCSTR , methodname : ::windows::core::PCSTR , replyfunc : alljoyn_messagereceiver_replyhandler_ptr , args : alljoyn_msgarg , numargs : usize , context : *mut ::core::ffi::c_void , timeout : u32 , flags : u8 ) -> QStatus );
    alljoyn_proxybusobject_methodcallasync(proxyobj.into_param().abi(), ifacename.into_param().abi(), methodname.into_param().abi(), replyfunc, args.into_param().abi(), numargs, context, timeout, flags)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcallasync_member<P0, P1>(proxyobj: P0, method: alljoyn_interfacedescription_member, replyfunc: alljoyn_messagereceiver_replyhandler_ptr, args: P1, numargs: usize, context: *mut ::core::ffi::c_void, timeout: u32, flags: u8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_methodcallasync_member ( proxyobj : alljoyn_proxybusobject , method : alljoyn_interfacedescription_member , replyfunc : alljoyn_messagereceiver_replyhandler_ptr , args : alljoyn_msgarg , numargs : usize , context : *mut ::core::ffi::c_void , timeout : u32 , flags : u8 ) -> QStatus );
    alljoyn_proxybusobject_methodcallasync_member(proxyobj.into_param().abi(), ::core::mem::transmute(method), replyfunc, args.into_param().abi(), numargs, context, timeout, flags)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_parsexml<P0, P1, P2>(proxyobj: P0, xml: P1, identifier: P2) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_parsexml ( proxyobj : alljoyn_proxybusobject , xml : ::windows::core::PCSTR , identifier : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_proxybusobject_parsexml(proxyobj.into_param().abi(), xml.into_param().abi(), identifier.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_ref_create<P0>(proxy: P0) -> alljoyn_proxybusobject_ref
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_ref_create ( proxy : alljoyn_proxybusobject ) -> alljoyn_proxybusobject_ref );
    alljoyn_proxybusobject_ref_create(proxy.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_ref_decref<P0>(r#ref: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject_ref>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_ref_decref ( r#ref : alljoyn_proxybusobject_ref ) -> ( ) );
    alljoyn_proxybusobject_ref_decref(r#ref.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_ref_get<P0>(r#ref: P0) -> alljoyn_proxybusobject
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject_ref>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_ref_get ( r#ref : alljoyn_proxybusobject_ref ) -> alljoyn_proxybusobject );
    alljoyn_proxybusobject_ref_get(r#ref.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_ref_incref<P0>(r#ref: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject_ref>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_ref_incref ( r#ref : alljoyn_proxybusobject_ref ) -> ( ) );
    alljoyn_proxybusobject_ref_incref(r#ref.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_registerpropertieschangedlistener<P0, P1>(proxyobj: P0, iface: P1, properties: *const *const i8, numproperties: usize, callback: alljoyn_proxybusobject_listener_propertieschanged_ptr, context: *mut ::core::ffi::c_void) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_registerpropertieschangedlistener ( proxyobj : alljoyn_proxybusobject , iface : ::windows::core::PCSTR , properties : *const *const i8 , numproperties : usize , callback : alljoyn_proxybusobject_listener_propertieschanged_ptr , context : *mut ::core::ffi::c_void ) -> QStatus );
    alljoyn_proxybusobject_registerpropertieschangedlistener(proxyobj.into_param().abi(), iface.into_param().abi(), properties, numproperties, callback, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_removechild<P0, P1>(proxyobj: P0, path: P1) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_removechild ( proxyobj : alljoyn_proxybusobject , path : ::windows::core::PCSTR ) -> QStatus );
    alljoyn_proxybusobject_removechild(proxyobj.into_param().abi(), path.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_secureconnection<P0>(proxyobj: P0, forceauth: i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_secureconnection ( proxyobj : alljoyn_proxybusobject , forceauth : i32 ) -> QStatus );
    alljoyn_proxybusobject_secureconnection(proxyobj.into_param().abi(), forceauth)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_secureconnectionasync<P0>(proxyobj: P0, forceauth: i32) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_secureconnectionasync ( proxyobj : alljoyn_proxybusobject , forceauth : i32 ) -> QStatus );
    alljoyn_proxybusobject_secureconnectionasync(proxyobj.into_param().abi(), forceauth)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_setproperty<P0, P1, P2, P3>(proxyobj: P0, iface: P1, property: P2, value: P3) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_setproperty ( proxyobj : alljoyn_proxybusobject , iface : ::windows::core::PCSTR , property : ::windows::core::PCSTR , value : alljoyn_msgarg ) -> QStatus );
    alljoyn_proxybusobject_setproperty(proxyobj.into_param().abi(), iface.into_param().abi(), property.into_param().abi(), value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_setpropertyasync<P0, P1, P2, P3>(proxyobj: P0, iface: P1, property: P2, value: P3, callback: alljoyn_proxybusobject_listener_setpropertycb_ptr, timeout: u32, context: *mut ::core::ffi::c_void) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<alljoyn_msgarg>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_setpropertyasync ( proxyobj : alljoyn_proxybusobject , iface : ::windows::core::PCSTR , property : ::windows::core::PCSTR , value : alljoyn_msgarg , callback : alljoyn_proxybusobject_listener_setpropertycb_ptr , timeout : u32 , context : *mut ::core::ffi::c_void ) -> QStatus );
    alljoyn_proxybusobject_setpropertyasync(proxyobj.into_param().abi(), iface.into_param().abi(), property.into_param().abi(), value.into_param().abi(), callback, timeout, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_proxybusobject_unregisterpropertieschangedlistener<P0, P1>(proxyobj: P0, iface: P1, callback: alljoyn_proxybusobject_listener_propertieschanged_ptr) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_proxybusobject>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_proxybusobject_unregisterpropertieschangedlistener ( proxyobj : alljoyn_proxybusobject , iface : ::windows::core::PCSTR , callback : alljoyn_proxybusobject_listener_propertieschanged_ptr ) -> QStatus );
    alljoyn_proxybusobject_unregisterpropertieschangedlistener(proxyobj.into_param().abi(), iface.into_param().abi(), callback)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_routerinit() -> QStatus {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_routerinit ( ) -> QStatus );
    alljoyn_routerinit()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_routerinitwithconfig(configxml: *mut i8) -> QStatus {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_routerinitwithconfig ( configxml : *mut i8 ) -> QStatus );
    alljoyn_routerinitwithconfig(configxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_routershutdown() -> QStatus {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_routershutdown ( ) -> QStatus );
    alljoyn_routershutdown()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_claim<P0>(proxy: P0, cakey: *mut i8, identitycertificatechain: *mut i8, groupid: *const u8, groupsize: usize, groupauthority: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_claim ( proxy : alljoyn_securityapplicationproxy , cakey : *mut i8 , identitycertificatechain : *mut i8 , groupid : *const u8 , groupsize : usize , groupauthority : *mut i8 , manifestsxmls : *mut *mut i8 , manifestscount : usize ) -> QStatus );
    alljoyn_securityapplicationproxy_claim(proxy.into_param().abi(), cakey, identitycertificatechain, groupid, groupsize, groupauthority, manifestsxmls, manifestscount)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_computemanifestdigest(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, digest: *mut *mut u8, digestsize: *mut usize) -> QStatus {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_computemanifestdigest ( unsignedmanifestxml : *mut i8 , identitycertificatepem : *mut i8 , digest : *mut *mut u8 , digestsize : *mut usize ) -> QStatus );
    alljoyn_securityapplicationproxy_computemanifestdigest(unsignedmanifestxml, identitycertificatepem, digest, digestsize)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_create<P0>(bus: P0, appbusname: *mut i8, sessionid: u32) -> alljoyn_securityapplicationproxy
where
    P0: ::windows::core::IntoParam<alljoyn_busattachment>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_create ( bus : alljoyn_busattachment , appbusname : *mut i8 , sessionid : u32 ) -> alljoyn_securityapplicationproxy );
    alljoyn_securityapplicationproxy_create(bus.into_param().abi(), appbusname, sessionid)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_destroy<P0>(proxy: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_destroy ( proxy : alljoyn_securityapplicationproxy ) -> ( ) );
    alljoyn_securityapplicationproxy_destroy(proxy.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_digest_destroy(digest: *mut u8) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_digest_destroy ( digest : *mut u8 ) -> ( ) );
    alljoyn_securityapplicationproxy_digest_destroy(digest)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_eccpublickey_destroy(eccpublickey: *mut i8) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_eccpublickey_destroy ( eccpublickey : *mut i8 ) -> ( ) );
    alljoyn_securityapplicationproxy_eccpublickey_destroy(eccpublickey)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_endmanagement<P0>(proxy: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_endmanagement ( proxy : alljoyn_securityapplicationproxy ) -> QStatus );
    alljoyn_securityapplicationproxy_endmanagement(proxy.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getapplicationstate<P0>(proxy: P0, applicationstate: *mut alljoyn_applicationstate) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_getapplicationstate ( proxy : alljoyn_securityapplicationproxy , applicationstate : *mut alljoyn_applicationstate ) -> QStatus );
    alljoyn_securityapplicationproxy_getapplicationstate(proxy.into_param().abi(), applicationstate)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getclaimcapabilities<P0>(proxy: P0, capabilities: *mut u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_getclaimcapabilities ( proxy : alljoyn_securityapplicationproxy , capabilities : *mut u16 ) -> QStatus );
    alljoyn_securityapplicationproxy_getclaimcapabilities(proxy.into_param().abi(), capabilities)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getclaimcapabilitiesadditionalinfo<P0>(proxy: P0, additionalinfo: *mut u16) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_getclaimcapabilitiesadditionalinfo ( proxy : alljoyn_securityapplicationproxy , additionalinfo : *mut u16 ) -> QStatus );
    alljoyn_securityapplicationproxy_getclaimcapabilitiesadditionalinfo(proxy.into_param().abi(), additionalinfo)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getdefaultpolicy<P0>(proxy: P0, policyxml: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_getdefaultpolicy ( proxy : alljoyn_securityapplicationproxy , policyxml : *mut *mut i8 ) -> QStatus );
    alljoyn_securityapplicationproxy_getdefaultpolicy(proxy.into_param().abi(), policyxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_geteccpublickey<P0>(proxy: P0, eccpublickey: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_geteccpublickey ( proxy : alljoyn_securityapplicationproxy , eccpublickey : *mut *mut i8 ) -> QStatus );
    alljoyn_securityapplicationproxy_geteccpublickey(proxy.into_param().abi(), eccpublickey)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getmanifesttemplate<P0>(proxy: P0, manifesttemplatexml: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_getmanifesttemplate ( proxy : alljoyn_securityapplicationproxy , manifesttemplatexml : *mut *mut i8 ) -> QStatus );
    alljoyn_securityapplicationproxy_getmanifesttemplate(proxy.into_param().abi(), manifesttemplatexml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getpermissionmanagementsessionport() -> u16 {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_getpermissionmanagementsessionport ( ) -> u16 );
    alljoyn_securityapplicationproxy_getpermissionmanagementsessionport()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getpolicy<P0>(proxy: P0, policyxml: *mut *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_getpolicy ( proxy : alljoyn_securityapplicationproxy , policyxml : *mut *mut i8 ) -> QStatus );
    alljoyn_securityapplicationproxy_getpolicy(proxy.into_param().abi(), policyxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_installmembership<P0>(proxy: P0, membershipcertificatechain: *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_installmembership ( proxy : alljoyn_securityapplicationproxy , membershipcertificatechain : *mut i8 ) -> QStatus );
    alljoyn_securityapplicationproxy_installmembership(proxy.into_param().abi(), membershipcertificatechain)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_manifest_destroy(signedmanifestxml: *mut i8) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_manifest_destroy ( signedmanifestxml : *mut i8 ) -> ( ) );
    alljoyn_securityapplicationproxy_manifest_destroy(signedmanifestxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_manifesttemplate_destroy(manifesttemplatexml: *mut i8) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_manifesttemplate_destroy ( manifesttemplatexml : *mut i8 ) -> ( ) );
    alljoyn_securityapplicationproxy_manifesttemplate_destroy(manifesttemplatexml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_policy_destroy(policyxml: *mut i8) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_policy_destroy ( policyxml : *mut i8 ) -> ( ) );
    alljoyn_securityapplicationproxy_policy_destroy(policyxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_reset<P0>(proxy: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_reset ( proxy : alljoyn_securityapplicationproxy ) -> QStatus );
    alljoyn_securityapplicationproxy_reset(proxy.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_resetpolicy<P0>(proxy: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_resetpolicy ( proxy : alljoyn_securityapplicationproxy ) -> QStatus );
    alljoyn_securityapplicationproxy_resetpolicy(proxy.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_setmanifestsignature(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, signature: *const u8, signaturesize: usize, signedmanifestxml: *mut *mut i8) -> QStatus {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_setmanifestsignature ( unsignedmanifestxml : *mut i8 , identitycertificatepem : *mut i8 , signature : *const u8 , signaturesize : usize , signedmanifestxml : *mut *mut i8 ) -> QStatus );
    alljoyn_securityapplicationproxy_setmanifestsignature(unsignedmanifestxml, identitycertificatepem, signature, signaturesize, signedmanifestxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_signmanifest(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, signingprivatekeypem: *mut i8, signedmanifestxml: *mut *mut i8) -> QStatus {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_signmanifest ( unsignedmanifestxml : *mut i8 , identitycertificatepem : *mut i8 , signingprivatekeypem : *mut i8 , signedmanifestxml : *mut *mut i8 ) -> QStatus );
    alljoyn_securityapplicationproxy_signmanifest(unsignedmanifestxml, identitycertificatepem, signingprivatekeypem, signedmanifestxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_startmanagement<P0>(proxy: P0) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_startmanagement ( proxy : alljoyn_securityapplicationproxy ) -> QStatus );
    alljoyn_securityapplicationproxy_startmanagement(proxy.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_updateidentity<P0>(proxy: P0, identitycertificatechain: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_updateidentity ( proxy : alljoyn_securityapplicationproxy , identitycertificatechain : *mut i8 , manifestsxmls : *mut *mut i8 , manifestscount : usize ) -> QStatus );
    alljoyn_securityapplicationproxy_updateidentity(proxy.into_param().abi(), identitycertificatechain, manifestsxmls, manifestscount)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_updatepolicy<P0>(proxy: P0, policyxml: *mut i8) -> QStatus
where
    P0: ::windows::core::IntoParam<alljoyn_securityapplicationproxy>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_securityapplicationproxy_updatepolicy ( proxy : alljoyn_securityapplicationproxy , policyxml : *mut i8 ) -> QStatus );
    alljoyn_securityapplicationproxy_updatepolicy(proxy.into_param().abi(), policyxml)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionlistener_create(callbacks: *const alljoyn_sessionlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_sessionlistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionlistener_create ( callbacks : *const alljoyn_sessionlistener_callbacks , context : *const ::core::ffi::c_void ) -> alljoyn_sessionlistener );
    alljoyn_sessionlistener_create(callbacks, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionlistener_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_sessionlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionlistener_destroy ( listener : alljoyn_sessionlistener ) -> ( ) );
    alljoyn_sessionlistener_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_cmp<P0, P1>(one: P0, other: P1) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_sessionopts>,
    P1: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_cmp ( one : alljoyn_sessionopts , other : alljoyn_sessionopts ) -> i32 );
    alljoyn_sessionopts_cmp(one.into_param().abi(), other.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_create(traffic: u8, ismultipoint: i32, proximity: u8, transports: u16) -> alljoyn_sessionopts {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_create ( traffic : u8 , ismultipoint : i32 , proximity : u8 , transports : u16 ) -> alljoyn_sessionopts );
    alljoyn_sessionopts_create(traffic, ismultipoint, proximity, transports)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_destroy<P0>(opts: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_destroy ( opts : alljoyn_sessionopts ) -> ( ) );
    alljoyn_sessionopts_destroy(opts.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_get_multipoint<P0>(opts: P0) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_get_multipoint ( opts : alljoyn_sessionopts ) -> i32 );
    alljoyn_sessionopts_get_multipoint(opts.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_get_proximity<P0>(opts: P0) -> u8
where
    P0: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_get_proximity ( opts : alljoyn_sessionopts ) -> u8 );
    alljoyn_sessionopts_get_proximity(opts.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_get_traffic<P0>(opts: P0) -> u8
where
    P0: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_get_traffic ( opts : alljoyn_sessionopts ) -> u8 );
    alljoyn_sessionopts_get_traffic(opts.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_get_transports<P0>(opts: P0) -> u16
where
    P0: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_get_transports ( opts : alljoyn_sessionopts ) -> u16 );
    alljoyn_sessionopts_get_transports(opts.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_iscompatible<P0, P1>(one: P0, other: P1) -> i32
where
    P0: ::windows::core::IntoParam<alljoyn_sessionopts>,
    P1: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_iscompatible ( one : alljoyn_sessionopts , other : alljoyn_sessionopts ) -> i32 );
    alljoyn_sessionopts_iscompatible(one.into_param().abi(), other.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_set_multipoint<P0>(opts: P0, ismultipoint: i32)
where
    P0: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_set_multipoint ( opts : alljoyn_sessionopts , ismultipoint : i32 ) -> ( ) );
    alljoyn_sessionopts_set_multipoint(opts.into_param().abi(), ismultipoint)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_set_proximity<P0>(opts: P0, proximity: u8)
where
    P0: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_set_proximity ( opts : alljoyn_sessionopts , proximity : u8 ) -> ( ) );
    alljoyn_sessionopts_set_proximity(opts.into_param().abi(), proximity)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_set_traffic<P0>(opts: P0, traffic: u8)
where
    P0: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_set_traffic ( opts : alljoyn_sessionopts , traffic : u8 ) -> ( ) );
    alljoyn_sessionopts_set_traffic(opts.into_param().abi(), traffic)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionopts_set_transports<P0>(opts: P0, transports: u16)
where
    P0: ::windows::core::IntoParam<alljoyn_sessionopts>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionopts_set_transports ( opts : alljoyn_sessionopts , transports : u16 ) -> ( ) );
    alljoyn_sessionopts_set_transports(opts.into_param().abi(), transports)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionportlistener_create(callbacks: *const alljoyn_sessionportlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_sessionportlistener {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionportlistener_create ( callbacks : *const alljoyn_sessionportlistener_callbacks , context : *const ::core::ffi::c_void ) -> alljoyn_sessionportlistener );
    alljoyn_sessionportlistener_create(callbacks, context)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_sessionportlistener_destroy<P0>(listener: P0)
where
    P0: ::windows::core::IntoParam<alljoyn_sessionportlistener>,
{
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_sessionportlistener_destroy ( listener : alljoyn_sessionportlistener ) -> ( ) );
    alljoyn_sessionportlistener_destroy(listener.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_shutdown() -> QStatus {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_shutdown ( ) -> QStatus );
    alljoyn_shutdown()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_unity_deferred_callbacks_process() -> i32 {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_unity_deferred_callbacks_process ( ) -> i32 );
    alljoyn_unity_deferred_callbacks_process()
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[inline]
pub unsafe fn alljoyn_unity_set_deferred_callback_mainthread_only(mainthread_only: i32) {
    ::windows_targets::link ! ( "msajapi.dll""system" fn alljoyn_unity_set_deferred_callback_mainthread_only ( mainthread_only : i32 ) -> ( ) );
    alljoyn_unity_set_deferred_callback_mainthread_only(mainthread_only)
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_BIG_ENDIAN: u8 = 66u8;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_CRED_CERT_CHAIN: u16 = 4u16;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_CRED_EXPIRATION: u16 = 32u16;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_CRED_LOGON_ENTRY: u16 = 16u16;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_CRED_NEW_PASSWORD: u16 = 4097u16;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_CRED_ONE_TIME_PWD: u16 = 8193u16;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_CRED_PASSWORD: u16 = 1u16;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_CRED_PRIVATE_KEY: u16 = 8u16;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_CRED_USER_NAME: u16 = 2u16;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_DISCONNECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_LITTLE_ENDIAN: u8 = 108u8;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_DEPRECATED: u8 = 2u8;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_GLOBAL_BROADCAST: u8 = 32u8;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_NO_REPLY: u8 = 1u8;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONCAST: u8 = 4u8;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONLESS: u8 = 8u8;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_UNICAST: u8 = 16u8;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_DEFAULT_TIMEOUT: u32 = 25000u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_FLAG_ALLOW_REMOTE_MSG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_FLAG_AUTO_START: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_FLAG_ENCRYPTED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_FLAG_GLOBAL_BROADCAST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_FLAG_NO_REPLY_EXPECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_FLAG_SESSIONLESS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_NAMED_PIPE_CONNECT_SPEC: ::windows::core::PCWSTR = ::windows::core::w!("npipe:");
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_PROP_ACCESS_READ: u8 = 1u8;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_PROP_ACCESS_RW: u8 = 3u8;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_PROP_ACCESS_WRITE: u8 = 2u8;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_PROXIMITY_ANY: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_PROXIMITY_NETWORK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_PROXIMITY_PHYSICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_READ_READY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_TRAFFIC_TYPE_MESSAGES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_TRAFFIC_TYPE_RAW_RELIABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_TRAFFIC_TYPE_RAW_UNRELIABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_WRITE_READY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const QCC_FALSE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const QCC_TRUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QStatus(pub i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_OK: QStatus = QStatus(0i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_FAIL: QStatus = QStatus(1i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UTF_CONVERSION_FAILED: QStatus = QStatus(2i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUFFER_TOO_SMALL: QStatus = QStatus(3i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_OS_ERROR: QStatus = QStatus(4i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_OUT_OF_MEMORY: QStatus = QStatus(5i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SOCKET_BIND_ERROR: QStatus = QStatus(6i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INIT_FAILED: QStatus = QStatus(7i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_WOULDBLOCK: QStatus = QStatus(8i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_NOT_IMPLEMENTED: QStatus = QStatus(9i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_TIMEOUT: QStatus = QStatus(10i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SOCK_OTHER_END_CLOSED: QStatus = QStatus(11i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_ARG_1: QStatus = QStatus(12i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_ARG_2: QStatus = QStatus(13i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_ARG_3: QStatus = QStatus(14i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_ARG_4: QStatus = QStatus(15i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_ARG_5: QStatus = QStatus(16i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_ARG_6: QStatus = QStatus(17i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_ARG_7: QStatus = QStatus(18i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_ARG_8: QStatus = QStatus(19i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_ADDRESS: QStatus = QStatus(20i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_DATA: QStatus = QStatus(21i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_READ_ERROR: QStatus = QStatus(22i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_WRITE_ERROR: QStatus = QStatus(23i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_OPEN_FAILED: QStatus = QStatus(24i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_PARSE_ERROR: QStatus = QStatus(25i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_END_OF_DATA: QStatus = QStatus(26i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_CONN_REFUSED: QStatus = QStatus(27i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_ARG_COUNT: QStatus = QStatus(28i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_WARNING: QStatus = QStatus(29i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_EOF: QStatus = QStatus(30i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DEADLOCK: QStatus = QStatus(31i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_COMMON_ERRORS: QStatus = QStatus(4096i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_STOPPING_THREAD: QStatus = QStatus(4097i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALERTED_THREAD: QStatus = QStatus(4098i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_MALFORMED: QStatus = QStatus(4099i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_AUTH_FAIL: QStatus = QStatus(4100i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_AUTH_USER_REJECT: QStatus = QStatus(4101i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_NO_SUCH_ALARM: QStatus = QStatus(4102i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_TIMER_FALLBEHIND: QStatus = QStatus(4103i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SSL_ERRORS: QStatus = QStatus(4104i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SSL_INIT: QStatus = QStatus(4105i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SSL_CONNECT: QStatus = QStatus(4106i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SSL_VERIFY: QStatus = QStatus(4107i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_EXTERNAL_THREAD: QStatus = QStatus(4108i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_CRYPTO_ERROR: QStatus = QStatus(4109i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_CRYPTO_TRUNCATED: QStatus = QStatus(4110i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_CRYPTO_KEY_UNAVAILABLE: QStatus = QStatus(4111i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_HOSTNAME: QStatus = QStatus(4112i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_CRYPTO_KEY_UNUSABLE: QStatus = QStatus(4113i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_EMPTY_KEY_BLOB: QStatus = QStatus(4114i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_CORRUPT_KEYBLOB: QStatus = QStatus(4115i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_KEY_ENCODING: QStatus = QStatus(4116i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DEAD_THREAD: QStatus = QStatus(4117i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_THREAD_RUNNING: QStatus = QStatus(4118i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_THREAD_STOPPING: QStatus = QStatus(4119i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_STRING_ENCODING: QStatus = QStatus(4120i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_CRYPTO_INSUFFICIENT_SECURITY: QStatus = QStatus(4121i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_CRYPTO_ILLEGAL_PARAMETERS: QStatus = QStatus(4122i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_CRYPTO_HASH_UNINITIALIZED: QStatus = QStatus(4123i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_THREAD_NO_WAIT: QStatus = QStatus(4124i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_TIMER_EXITING: QStatus = QStatus(4125i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_GUID: QStatus = QStatus(4126i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_THREADPOOL_EXHAUSTED: QStatus = QStatus(4127i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_THREADPOOL_STOPPING: QStatus = QStatus(4128i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_STREAM: QStatus = QStatus(4129i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_TIMER_FULL: QStatus = QStatus(4130i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_IODISPATCH_STOPPING: QStatus = QStatus(4131i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SLAP_INVALID_PACKET_LEN: QStatus = QStatus(4132i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SLAP_HDR_CHECKSUM_ERROR: QStatus = QStatus(4133i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SLAP_INVALID_PACKET_TYPE: QStatus = QStatus(4134i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SLAP_LEN_MISMATCH: QStatus = QStatus(4135i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SLAP_PACKET_TYPE_MISMATCH: QStatus = QStatus(4136i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SLAP_CRC_ERROR: QStatus = QStatus(4137i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SLAP_ERROR: QStatus = QStatus(4138i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SLAP_OTHER_END_CLOSED: QStatus = QStatus(4139i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_TIMER_NOT_ALLOWED: QStatus = QStatus(4140i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_NOT_CONN: QStatus = QStatus(4141i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_CONVERTER_ERROR: QStatus = QStatus(8192i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_RULES_COUNT: QStatus = QStatus(8193i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INTERFACE_MEMBERS_MISSING: QStatus = QStatus(8194i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_MEMBER_TYPE: QStatus = QStatus(8195i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_MEMBER_ACTION: QStatus = QStatus(8196i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_MEMBER_DENY_ACTION_WITH_OTHER: QStatus = QStatus(8197i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_ANNOTATIONS_COUNT: QStatus = QStatus(8198i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_ELEMENT_NAME: QStatus = QStatus(8199i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_ATTRIBUTE_VALUE: QStatus = QStatus(8200i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_SECURITY_LEVEL_ANNOTATION_VALUE: QStatus = QStatus(8201i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_ELEMENT_CHILDREN_COUNT: QStatus = QStatus(8202i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_POLICY_VERSION: QStatus = QStatus(8203i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_POLICY_SERIAL_NUMBER: QStatus = QStatus(8204i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_ACL_PEER_TYPE: QStatus = QStatus(8205i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_ACL_PEER_CHILDREN_COUNT: QStatus = QStatus(8206i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_ACL_ALL_TYPE_PEER_WITH_OTHERS: QStatus = QStatus(8207i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_ACL_PEER_PUBLIC_KEY: QStatus = QStatus(8208i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_ACL_PEER_NOT_UNIQUE: QStatus = QStatus(8209i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_ACL_PEER_PUBLIC_KEY_SET: QStatus = QStatus(8210i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_ACLS_MISSING: QStatus = QStatus(8211i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_ACL_PEERS_MISSING: QStatus = QStatus(8212i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_OBJECT_PATH: QStatus = QStatus(8213i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_INTERFACE_NAME: QStatus = QStatus(8214i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_MEMBER_NAME: QStatus = QStatus(8215i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_MANIFEST_VERSION: QStatus = QStatus(8216i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_OID: QStatus = QStatus(8217i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INVALID_BASE64: QStatus = QStatus(8218i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_INTERFACE_NAME_NOT_UNIQUE: QStatus = QStatus(8219i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_MEMBER_NAME_NOT_UNIQUE: QStatus = QStatus(8220i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_OBJECT_PATH_NOT_UNIQUE: QStatus = QStatus(8221i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_XML_ANNOTATION_NOT_UNIQUE: QStatus = QStatus(8222i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_NONE: QStatus = QStatus(65535i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_ERRORS: QStatus = QStatus(36864i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_READ_ERROR: QStatus = QStatus(36865i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_WRITE_ERROR: QStatus = QStatus(36866i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_VALUE_TYPE: QStatus = QStatus(36867i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_HEADER_FIELD: QStatus = QStatus(36868i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_SIGNATURE: QStatus = QStatus(36869i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_OBJ_PATH: QStatus = QStatus(36870i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_MEMBER_NAME: QStatus = QStatus(36871i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_INTERFACE_NAME: QStatus = QStatus(36872i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_ERROR_NAME: QStatus = QStatus(36873i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_BUS_NAME: QStatus = QStatus(36874i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NAME_TOO_LONG: QStatus = QStatus(36875i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_LENGTH: QStatus = QStatus(36876i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_VALUE: QStatus = QStatus(36877i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_HDR_FLAGS: QStatus = QStatus(36878i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_BODY_LEN: QStatus = QStatus(36879i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_HEADER_LEN: QStatus = QStatus(36880i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_UNKNOWN_SERIAL: QStatus = QStatus(36881i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_UNKNOWN_PATH: QStatus = QStatus(36882i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_UNKNOWN_INTERFACE: QStatus = QStatus(36883i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_ESTABLISH_FAILED: QStatus = QStatus(36884i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_UNEXPECTED_SIGNATURE: QStatus = QStatus(36885i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_INTERFACE_MISSING: QStatus = QStatus(36886i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_PATH_MISSING: QStatus = QStatus(36887i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_MEMBER_MISSING: QStatus = QStatus(36888i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_REPLY_SERIAL_MISSING: QStatus = QStatus(36889i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_ERROR_NAME_MISSING: QStatus = QStatus(36890i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_INTERFACE_NO_SUCH_MEMBER: QStatus = QStatus(36891i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_SUCH_OBJECT: QStatus = QStatus(36892i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_OBJECT_NO_SUCH_MEMBER: QStatus = QStatus(36893i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_OBJECT_NO_SUCH_INTERFACE: QStatus = QStatus(36894i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_SUCH_INTERFACE: QStatus = QStatus(36895i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_MEMBER_NO_SUCH_SIGNATURE: QStatus = QStatus(36896i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NOT_NUL_TERMINATED: QStatus = QStatus(36897i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_SUCH_PROPERTY: QStatus = QStatus(36898i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_SET_WRONG_SIGNATURE: QStatus = QStatus(36899i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_PROPERTY_VALUE_NOT_SET: QStatus = QStatus(36900i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_PROPERTY_ACCESS_DENIED: QStatus = QStatus(36901i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_TRANSPORTS: QStatus = QStatus(36902i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_TRANSPORT_ARGS: QStatus = QStatus(36903i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_ROUTE: QStatus = QStatus(36904i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_ENDPOINT: QStatus = QStatus(36905i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_SEND_PARAMETER: QStatus = QStatus(36906i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_UNMATCHED_REPLY_SERIAL: QStatus = QStatus(36907i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_SENDER_ID: QStatus = QStatus(36908i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_TRANSPORT_NOT_STARTED: QStatus = QStatus(36909i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_EMPTY_MESSAGE: QStatus = QStatus(36910i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NOT_OWNER: QStatus = QStatus(36911i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_SET_PROPERTY_REJECTED: QStatus = QStatus(36912i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_CONNECT_FAILED: QStatus = QStatus(36913i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_REPLY_IS_ERROR_MESSAGE: QStatus = QStatus(36914i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NOT_AUTHENTICATING: QStatus = QStatus(36915i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_LISTENER: QStatus = QStatus(36916i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NOT_ALLOWED: QStatus = QStatus(36918i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_WRITE_QUEUE_FULL: QStatus = QStatus(36919i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_ENDPOINT_CLOSING: QStatus = QStatus(36920i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_INTERFACE_MISMATCH: QStatus = QStatus(36921i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_MEMBER_ALREADY_EXISTS: QStatus = QStatus(36922i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_PROPERTY_ALREADY_EXISTS: QStatus = QStatus(36923i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_IFACE_ALREADY_EXISTS: QStatus = QStatus(36924i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_ERROR_RESPONSE: QStatus = QStatus(36925i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_XML: QStatus = QStatus(36926i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_CHILD_PATH: QStatus = QStatus(36927i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_OBJ_ALREADY_EXISTS: QStatus = QStatus(36928i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_OBJ_NOT_FOUND: QStatus = QStatus(36929i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_CANNOT_EXPAND_MESSAGE: QStatus = QStatus(36930i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NOT_COMPRESSED: QStatus = QStatus(36931i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_ALREADY_CONNECTED: QStatus = QStatus(36932i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NOT_CONNECTED: QStatus = QStatus(36933i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_ALREADY_LISTENING: QStatus = QStatus(36934i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_KEY_UNAVAILABLE: QStatus = QStatus(36935i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_TRUNCATED: QStatus = QStatus(36936i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_KEY_STORE_NOT_LOADED: QStatus = QStatus(36937i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_AUTHENTICATION_MECHANISM: QStatus = QStatus(36938i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BUS_ALREADY_STARTED: QStatus = QStatus(36939i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BUS_NOT_STARTED: QStatus = QStatus(36940i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_KEYBLOB_OP_INVALID: QStatus = QStatus(36941i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_INVALID_HEADER_CHECKSUM: QStatus = QStatus(36942i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_MESSAGE_NOT_ENCRYPTED: QStatus = QStatus(36943i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_INVALID_HEADER_SERIAL: QStatus = QStatus(36944i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_TIME_TO_LIVE_EXPIRED: QStatus = QStatus(36945i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_HDR_EXPANSION_INVALID: QStatus = QStatus(36946i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_MISSING_COMPRESSION_TOKEN: QStatus = QStatus(36947i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_PEER_GUID: QStatus = QStatus(36948i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_MESSAGE_DECRYPTION_FAILED: QStatus = QStatus(36949i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_SECURITY_FATAL: QStatus = QStatus(36950i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_KEY_EXPIRED: QStatus = QStatus(36951i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_CORRUPT_KEYSTORE: QStatus = QStatus(36952i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_CALL_FOR_REPLY: QStatus = QStatus(36953i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NOT_A_COMPLETE_TYPE: QStatus = QStatus(36954i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_POLICY_VIOLATION: QStatus = QStatus(36955i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_SUCH_SERVICE: QStatus = QStatus(36956i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_TRANSPORT_NOT_AVAILABLE: QStatus = QStatus(36957i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_INVALID_AUTH_MECHANISM: QStatus = QStatus(36958i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_KEYSTORE_VERSION_MISMATCH: QStatus = QStatus(36959i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BLOCKING_CALL_NOT_ALLOWED: QStatus = QStatus(36960i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_SIGNATURE_MISMATCH: QStatus = QStatus(36961i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_STOPPING: QStatus = QStatus(36962i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_METHOD_CALL_ABORTED: QStatus = QStatus(36963i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_CANNOT_ADD_INTERFACE: QStatus = QStatus(36964i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_CANNOT_ADD_HANDLER: QStatus = QStatus(36965i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_KEYSTORE_NOT_LOADED: QStatus = QStatus(36966i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_SUCH_HANDLE: QStatus = QStatus(36971i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_HANDLES_NOT_ENABLED: QStatus = QStatus(36972i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_HANDLES_MISMATCH: QStatus = QStatus(36973i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_SESSION: QStatus = QStatus(36975i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_ELEMENT_NOT_FOUND: QStatus = QStatus(36976i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NOT_A_DICTIONARY: QStatus = QStatus(36977i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_WAIT_FAILED: QStatus = QStatus(36978i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_BAD_SESSION_OPTS: QStatus = QStatus(36980i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_CONNECTION_REJECTED: QStatus = QStatus(36981i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DBUS_REQUEST_NAME_REPLY_PRIMARY_OWNER: QStatus = QStatus(36982i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DBUS_REQUEST_NAME_REPLY_IN_QUEUE: QStatus = QStatus(36983i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DBUS_REQUEST_NAME_REPLY_EXISTS: QStatus = QStatus(36984i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DBUS_REQUEST_NAME_REPLY_ALREADY_OWNER: QStatus = QStatus(36985i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DBUS_RELEASE_NAME_REPLY_RELEASED: QStatus = QStatus(36986i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DBUS_RELEASE_NAME_REPLY_NON_EXISTENT: QStatus = QStatus(36987i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DBUS_RELEASE_NAME_REPLY_NOT_OWNER: QStatus = QStatus(36988i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DBUS_START_REPLY_ALREADY_RUNNING: QStatus = QStatus(36990i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_ALREADY_EXISTS: QStatus = QStatus(36992i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_FAILED: QStatus = QStatus(36993i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_JOINSESSION_REPLY_NO_SESSION: QStatus = QStatus(36995i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_JOINSESSION_REPLY_UNREACHABLE: QStatus = QStatus(36996i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_JOINSESSION_REPLY_CONNECT_FAILED: QStatus = QStatus(36997i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_JOINSESSION_REPLY_REJECTED: QStatus = QStatus(36998i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_JOINSESSION_REPLY_BAD_SESSION_OPTS: QStatus = QStatus(36999i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_JOINSESSION_REPLY_FAILED: QStatus = QStatus(37000i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_LEAVESESSION_REPLY_NO_SESSION: QStatus = QStatus(37002i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_LEAVESESSION_REPLY_FAILED: QStatus = QStatus(37003i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_TRANSPORT_NOT_AVAILABLE: QStatus = QStatus(37004i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_ALREADY_ADVERTISING: QStatus = QStatus(37005i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_FAILED: QStatus = QStatus(37006i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_CANCELADVERTISENAME_REPLY_FAILED: QStatus = QStatus(37008i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_TRANSPORT_NOT_AVAILABLE: QStatus = QStatus(37009i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_ALREADY_DISCOVERING: QStatus = QStatus(37010i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_FAILED: QStatus = QStatus(37011i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_CANCELFINDADVERTISEDNAME_REPLY_FAILED: QStatus = QStatus(37013i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_UNEXPECTED_DISPOSITION: QStatus = QStatus(37014i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_INTERFACE_ACTIVATED: QStatus = QStatus(37015i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_UNBINDSESSIONPORT_REPLY_BAD_PORT: QStatus = QStatus(37016i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_UNBINDSESSIONPORT_REPLY_FAILED: QStatus = QStatus(37017i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_INVALID_OPTS: QStatus = QStatus(37018i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_JOINSESSION_REPLY_ALREADY_JOINED: QStatus = QStatus(37019i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_SELF_CONNECT: QStatus = QStatus(37020i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_SECURITY_NOT_ENABLED: QStatus = QStatus(37021i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_LISTENER_ALREADY_SET: QStatus = QStatus(37022i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_PEER_AUTH_VERSION_MISMATCH: QStatus = QStatus(37023i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_NOT_SUPPORTED: QStatus = QStatus(37024i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_NO_DEST_SUPPORT: QStatus = QStatus(37025i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_FAILED: QStatus = QStatus(37026i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_ACCESS_PERMISSION_WARNING: QStatus = QStatus(37027i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_ACCESS_PERMISSION_ERROR: QStatus = QStatus(37028i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_DESTINATION_NOT_AUTHENTICATED: QStatus = QStatus(37029i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_ENDPOINT_REDIRECTED: QStatus = QStatus(37030i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_AUTHENTICATION_PENDING: QStatus = QStatus(37031i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NOT_AUTHORIZED: QStatus = QStatus(37032i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_PACKET_BUS_NO_SUCH_CHANNEL: QStatus = QStatus(37033i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_PACKET_BAD_FORMAT: QStatus = QStatus(37034i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_PACKET_CONNECT_TIMEOUT: QStatus = QStatus(37035i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_PACKET_CHANNEL_FAIL: QStatus = QStatus(37036i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_PACKET_TOO_LARGE: QStatus = QStatus(37037i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_PACKET_BAD_PARAMETER: QStatus = QStatus(37038i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_PACKET_BAD_CRC: QStatus = QStatus(37039i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_RENDEZVOUS_SERVER_DEACTIVATED_USER: QStatus = QStatus(37067i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_RENDEZVOUS_SERVER_UNKNOWN_USER: QStatus = QStatus(37068i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UNABLE_TO_CONNECT_TO_RENDEZVOUS_SERVER: QStatus = QStatus(37069i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_NOT_CONNECTED_TO_RENDEZVOUS_SERVER: QStatus = QStatus(37070i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UNABLE_TO_SEND_MESSAGE_TO_RENDEZVOUS_SERVER: QStatus = QStatus(37071i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_RENDEZVOUS_SERVER_INTERFACE_MESSAGE: QStatus = QStatus(37072i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_PERSISTENT_CONNECTION_MESSAGE_RESPONSE: QStatus = QStatus(37073i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_ON_DEMAND_CONNECTION_MESSAGE_RESPONSE: QStatus = QStatus(37074i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_HTTP_METHOD_USED_FOR_RENDEZVOUS_SERVER_INTERFACE_MESSAGE: QStatus = QStatus(37075i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_RENDEZVOUS_SERVER_ERR500_INTERNAL_ERROR: QStatus = QStatus(37076i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_RENDEZVOUS_SERVER_ERR503_STATUS_UNAVAILABLE: QStatus = QStatus(37077i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_RENDEZVOUS_SERVER_ERR401_UNAUTHORIZED_REQUEST: QStatus = QStatus(37078i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_RENDEZVOUS_SERVER_UNRECOVERABLE_ERROR: QStatus = QStatus(37079i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_RENDEZVOUS_SERVER_ROOT_CERTIFICATE_UNINITIALIZED: QStatus = QStatus(37080i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_SUCH_ANNOTATION: QStatus = QStatus(37081i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_ANNOTATION_ALREADY_EXISTS: QStatus = QStatus(37082i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_SOCK_CLOSING: QStatus = QStatus(37083i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_NO_SUCH_DEVICE: QStatus = QStatus(37084i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_P2P: QStatus = QStatus(37085i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_P2P_TIMEOUT: QStatus = QStatus(37086i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_P2P_NOT_CONNECTED: QStatus = QStatus(37087i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BAD_TRANSPORT_MASK: QStatus = QStatus(37088i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_PROXIMITY_CONNECTION_ESTABLISH_FAIL: QStatus = QStatus(37089i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_PROXIMITY_NO_PEERS_FOUND: QStatus = QStatus(37090i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_OBJECT_NOT_REGISTERED: QStatus = QStatus(37091i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_P2P_DISABLED: QStatus = QStatus(37092i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_P2P_BUSY: QStatus = QStatus(37093i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_INCOMPATIBLE_DAEMON: QStatus = QStatus(37094i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_P2P_NO_GO: QStatus = QStatus(37095i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_P2P_NO_STA: QStatus = QStatus(37096i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_P2P_FORBIDDEN: QStatus = QStatus(37097i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_ONAPPSUSPEND_REPLY_FAILED: QStatus = QStatus(37098i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_ONAPPSUSPEND_REPLY_UNSUPPORTED: QStatus = QStatus(37099i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_ONAPPRESUME_REPLY_FAILED: QStatus = QStatus(37100i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_ONAPPRESUME_REPLY_UNSUPPORTED: QStatus = QStatus(37101i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_NO_SUCH_MESSAGE: QStatus = QStatus(37102i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_REPLY_NO_SESSION: QStatus = QStatus(37103i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_BINDER: QStatus = QStatus(37104i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_MULTIPOINT: QStatus = QStatus(37105i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_FOUND: QStatus = QStatus(37106i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_INCOMPATIBLE_REMOTE_DAEMON: QStatus = QStatus(37107i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_REPLY_FAILED: QStatus = QStatus(37108i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_REMOVED_BY_BINDER: QStatus = QStatus(37109i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_MATCH_RULE_NOT_FOUND: QStatus = QStatus(37110i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_PING_FAILED: QStatus = QStatus(37111i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_PING_REPLY_UNREACHABLE: QStatus = QStatus(37112i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_MSG_TOO_LONG: QStatus = QStatus(37113i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_DEMUX_NO_ENDPOINT: QStatus = QStatus(37114i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_NO_NETWORK: QStatus = QStatus(37115i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_UNEXPECTED_LENGTH: QStatus = QStatus(37116i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_UNEXPECTED_FLOW: QStatus = QStatus(37117i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_DISCONNECT: QStatus = QStatus(37118i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_NOT_IMPLEMENTED: QStatus = QStatus(37119i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_NO_LISTENER: QStatus = QStatus(37120i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_STOPPING: QStatus = QStatus(37121i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ARDP_BACKPRESSURE: QStatus = QStatus(37122i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_BACKPRESSURE: QStatus = QStatus(37123i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ARDP_INVALID_STATE: QStatus = QStatus(37124i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ARDP_TTL_EXPIRED: QStatus = QStatus(37125i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ARDP_PERSIST_TIMEOUT: QStatus = QStatus(37126i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ARDP_PROBE_TIMEOUT: QStatus = QStatus(37127i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ARDP_REMOTE_CONNECTION_RESET: QStatus = QStatus(37128i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_BUSHELLO: QStatus = QStatus(37129i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_MESSAGE: QStatus = QStatus(37130i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_INVALID: QStatus = QStatus(37131i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_UNSUPPORTED: QStatus = QStatus(37132i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_ENDPOINT_STALLED: QStatus = QStatus(37133i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ARDP_INVALID_RESPONSE: QStatus = QStatus(37134i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ARDP_INVALID_CONNECTION: QStatus = QStatus(37135i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_LOCAL_DISCONNECT: QStatus = QStatus(37136i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_EARLY_EXIT: QStatus = QStatus(37137i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_LOCAL_DISCONNECT_FAIL: QStatus = QStatus(37138i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ARDP_DISCONNECTING: QStatus = QStatus(37139i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_PING_REPLY_INCOMPATIBLE_REMOTE_ROUTING_NODE: QStatus = QStatus(37140i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_PING_REPLY_TIMEOUT: QStatus = QStatus(37141i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_PING_REPLY_UNKNOWN_NAME: QStatus = QStatus(37142i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_PING_REPLY_FAILED: QStatus = QStatus(37143i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_TCP_MAX_UNTRUSTED: QStatus = QStatus(37144i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ALLJOYN_PING_REPLY_IN_PROGRESS: QStatus = QStatus(37145i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_LANGUAGE_NOT_SUPPORTED: QStatus = QStatus(37146i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ABOUT_FIELD_ALREADY_SPECIFIED: QStatus = QStatus(37147i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_NOT_DISCONNECTED: QStatus = QStatus(37148i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_ENDPOINT_NOT_STARTED: QStatus = QStatus(37149i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UDP_ENDPOINT_REMOVED: QStatus = QStatus(37150i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ARDP_VERSION_NOT_SUPPORTED: QStatus = QStatus(37151i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_CONNECTION_LIMIT_EXCEEDED: QStatus = QStatus(37152i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ARDP_WRITE_BLOCKED: QStatus = QStatus(37153i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_PERMISSION_DENIED: QStatus = QStatus(37154i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ABOUT_DEFAULT_LANGUAGE_NOT_SPECIFIED: QStatus = QStatus(37155i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ABOUT_SESSIONPORT_NOT_BOUND: QStatus = QStatus(37156i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ABOUT_ABOUTDATA_MISSING_REQUIRED_FIELD: QStatus = QStatus(37157i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ABOUT_INVALID_ABOUTDATA_LISTENER: QStatus = QStatus(37158i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_PING_GROUP_NOT_FOUND: QStatus = QStatus(37159i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_REMOVED_BY_BINDER_SELF: QStatus = QStatus(37160i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_CONFIG: QStatus = QStatus(37161i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ABOUT_INVALID_ABOUTDATA_FIELD_VALUE: QStatus = QStatus(37162i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_ABOUT_INVALID_ABOUTDATA_FIELD_APPID_SIZE: QStatus = QStatus(37163i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_TRANSPORT_ACCESS_DENIED: QStatus = QStatus(37164i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_CERTIFICATE: QStatus = QStatus(37165i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_CERTIFICATE_NOT_FOUND: QStatus = QStatus(37166i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DUPLICATE_CERTIFICATE: QStatus = QStatus(37167i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_UNKNOWN_CERTIFICATE: QStatus = QStatus(37168i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_MISSING_DIGEST_IN_CERTIFICATE: QStatus = QStatus(37169i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DIGEST_MISMATCH: QStatus = QStatus(37170i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_DUPLICATE_KEY: QStatus = QStatus(37171i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_NO_COMMON_TRUST: QStatus = QStatus(37172i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_MANIFEST_NOT_FOUND: QStatus = QStatus(37173i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_CERT_CHAIN: QStatus = QStatus(37174i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_NO_TRUST_ANCHOR: QStatus = QStatus(37175i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_APPLICATION_STATE: QStatus = QStatus(37176i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_FEATURE_NOT_AVAILABLE: QStatus = QStatus(37177i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_KEY_STORE_ALREADY_INITIALIZED: QStatus = QStatus(37178i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_KEY_STORE_ID_NOT_YET_SET: QStatus = QStatus(37179i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_POLICY_NOT_NEWER: QStatus = QStatus(37180i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_MANIFEST_REJECTED: QStatus = QStatus(37181i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_CERTIFICATE_USAGE: QStatus = QStatus(37182i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_INVALID_SIGNAL_EMISSION_TYPE: QStatus = QStatus(37183i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_APPLICATION_STATE_LISTENER_ALREADY_EXISTS: QStatus = QStatus(37184i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_APPLICATION_STATE_LISTENER_NO_SUCH_LISTENER: QStatus = QStatus(37185i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_MANAGEMENT_ALREADY_STARTED: QStatus = QStatus(37186i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_MANAGEMENT_NOT_STARTED: QStatus = QStatus(37187i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ER_BUS_DESCRIPTION_ALREADY_EXISTS: QStatus = QStatus(37188i32);
impl ::core::marker::Copy for QStatus {}
impl ::core::clone::Clone for QStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for QStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for QStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_about_announceflag(pub i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const UNANNOUNCED: alljoyn_about_announceflag = alljoyn_about_announceflag(0i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ANNOUNCED: alljoyn_about_announceflag = alljoyn_about_announceflag(1i32);
impl ::core::marker::Copy for alljoyn_about_announceflag {}
impl ::core::clone::Clone for alljoyn_about_announceflag {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for alljoyn_about_announceflag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for alljoyn_about_announceflag {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for alljoyn_about_announceflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_about_announceflag").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_applicationstate(pub i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const NOT_CLAIMABLE: alljoyn_applicationstate = alljoyn_applicationstate(0i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const CLAIMABLE: alljoyn_applicationstate = alljoyn_applicationstate(1i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const CLAIMED: alljoyn_applicationstate = alljoyn_applicationstate(2i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const NEED_UPDATE: alljoyn_applicationstate = alljoyn_applicationstate(3i32);
impl ::core::marker::Copy for alljoyn_applicationstate {}
impl ::core::clone::Clone for alljoyn_applicationstate {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for alljoyn_applicationstate {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for alljoyn_applicationstate {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for alljoyn_applicationstate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_applicationstate").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_claimcapability_masks(pub i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const CAPABLE_ECDHE_NULL: alljoyn_claimcapability_masks = alljoyn_claimcapability_masks(1i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const CAPABLE_ECDHE_ECDSA: alljoyn_claimcapability_masks = alljoyn_claimcapability_masks(4i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const CAPABLE_ECDHE_SPEKE: alljoyn_claimcapability_masks = alljoyn_claimcapability_masks(8i32);
impl ::core::marker::Copy for alljoyn_claimcapability_masks {}
impl ::core::clone::Clone for alljoyn_claimcapability_masks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for alljoyn_claimcapability_masks {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for alljoyn_claimcapability_masks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for alljoyn_claimcapability_masks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_claimcapability_masks").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_claimcapabilityadditionalinfo_masks(pub i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const PASSWORD_GENERATED_BY_SECURITY_MANAGER: alljoyn_claimcapabilityadditionalinfo_masks = alljoyn_claimcapabilityadditionalinfo_masks(1i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const PASSWORD_GENERATED_BY_APPLICATION: alljoyn_claimcapabilityadditionalinfo_masks = alljoyn_claimcapabilityadditionalinfo_masks(2i32);
impl ::core::marker::Copy for alljoyn_claimcapabilityadditionalinfo_masks {}
impl ::core::clone::Clone for alljoyn_claimcapabilityadditionalinfo_masks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for alljoyn_claimcapabilityadditionalinfo_masks {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for alljoyn_claimcapabilityadditionalinfo_masks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for alljoyn_claimcapabilityadditionalinfo_masks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_claimcapabilityadditionalinfo_masks").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_interfacedescription_securitypolicy(pub i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const AJ_IFC_SECURITY_INHERIT: alljoyn_interfacedescription_securitypolicy = alljoyn_interfacedescription_securitypolicy(0i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const AJ_IFC_SECURITY_REQUIRED: alljoyn_interfacedescription_securitypolicy = alljoyn_interfacedescription_securitypolicy(1i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const AJ_IFC_SECURITY_OFF: alljoyn_interfacedescription_securitypolicy = alljoyn_interfacedescription_securitypolicy(2i32);
impl ::core::marker::Copy for alljoyn_interfacedescription_securitypolicy {}
impl ::core::clone::Clone for alljoyn_interfacedescription_securitypolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for alljoyn_interfacedescription_securitypolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for alljoyn_interfacedescription_securitypolicy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for alljoyn_interfacedescription_securitypolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_interfacedescription_securitypolicy").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_messagetype(pub i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_INVALID: alljoyn_messagetype = alljoyn_messagetype(0i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_METHOD_CALL: alljoyn_messagetype = alljoyn_messagetype(1i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_METHOD_RET: alljoyn_messagetype = alljoyn_messagetype(2i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_ERROR: alljoyn_messagetype = alljoyn_messagetype(3i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_MESSAGE_SIGNAL: alljoyn_messagetype = alljoyn_messagetype(4i32);
impl ::core::marker::Copy for alljoyn_messagetype {}
impl ::core::clone::Clone for alljoyn_messagetype {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for alljoyn_messagetype {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for alljoyn_messagetype {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for alljoyn_messagetype {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_messagetype").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_sessionlostreason(pub i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_SESSIONLOST_INVALID: alljoyn_sessionlostreason = alljoyn_sessionlostreason(0i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_SESSIONLOST_REMOTE_END_LEFT_SESSION: alljoyn_sessionlostreason = alljoyn_sessionlostreason(1i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_SESSIONLOST_REMOTE_END_CLOSED_ABRUPTLY: alljoyn_sessionlostreason = alljoyn_sessionlostreason(2i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_SESSIONLOST_REMOVED_BY_BINDER: alljoyn_sessionlostreason = alljoyn_sessionlostreason(3i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_SESSIONLOST_LINK_TIMEOUT: alljoyn_sessionlostreason = alljoyn_sessionlostreason(4i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_SESSIONLOST_REASON_OTHER: alljoyn_sessionlostreason = alljoyn_sessionlostreason(5i32);
impl ::core::marker::Copy for alljoyn_sessionlostreason {}
impl ::core::clone::Clone for alljoyn_sessionlostreason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for alljoyn_sessionlostreason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for alljoyn_sessionlostreason {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for alljoyn_sessionlostreason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_sessionlostreason").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_typeid(pub i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_INVALID: alljoyn_typeid = alljoyn_typeid(0i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_ARRAY: alljoyn_typeid = alljoyn_typeid(97i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_BOOLEAN: alljoyn_typeid = alljoyn_typeid(98i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_DOUBLE: alljoyn_typeid = alljoyn_typeid(100i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_DICT_ENTRY: alljoyn_typeid = alljoyn_typeid(101i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_SIGNATURE: alljoyn_typeid = alljoyn_typeid(103i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_HANDLE: alljoyn_typeid = alljoyn_typeid(104i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_INT32: alljoyn_typeid = alljoyn_typeid(105i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_INT16: alljoyn_typeid = alljoyn_typeid(110i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_OBJECT_PATH: alljoyn_typeid = alljoyn_typeid(111i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_UINT16: alljoyn_typeid = alljoyn_typeid(113i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_STRUCT: alljoyn_typeid = alljoyn_typeid(114i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_STRING: alljoyn_typeid = alljoyn_typeid(115i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_UINT64: alljoyn_typeid = alljoyn_typeid(116i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_UINT32: alljoyn_typeid = alljoyn_typeid(117i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_VARIANT: alljoyn_typeid = alljoyn_typeid(118i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_INT64: alljoyn_typeid = alljoyn_typeid(120i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_BYTE: alljoyn_typeid = alljoyn_typeid(121i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_STRUCT_OPEN: alljoyn_typeid = alljoyn_typeid(40i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_STRUCT_CLOSE: alljoyn_typeid = alljoyn_typeid(41i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_DICT_ENTRY_OPEN: alljoyn_typeid = alljoyn_typeid(123i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_DICT_ENTRY_CLOSE: alljoyn_typeid = alljoyn_typeid(125i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_BOOLEAN_ARRAY: alljoyn_typeid = alljoyn_typeid(25185i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_DOUBLE_ARRAY: alljoyn_typeid = alljoyn_typeid(25697i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_INT32_ARRAY: alljoyn_typeid = alljoyn_typeid(26977i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_INT16_ARRAY: alljoyn_typeid = alljoyn_typeid(28257i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_UINT16_ARRAY: alljoyn_typeid = alljoyn_typeid(29025i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_UINT64_ARRAY: alljoyn_typeid = alljoyn_typeid(29793i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_UINT32_ARRAY: alljoyn_typeid = alljoyn_typeid(30049i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_INT64_ARRAY: alljoyn_typeid = alljoyn_typeid(30817i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_BYTE_ARRAY: alljoyn_typeid = alljoyn_typeid(31073i32);
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub const ALLJOYN_WILDCARD: alljoyn_typeid = alljoyn_typeid(42i32);
impl ::core::marker::Copy for alljoyn_typeid {}
impl ::core::clone::Clone for alljoyn_typeid {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for alljoyn_typeid {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for alljoyn_typeid {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for alljoyn_typeid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_typeid").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct _alljoyn_abouticon_handle(pub u8);
#[repr(C)]
pub struct _alljoyn_abouticonobj_handle(pub u8);
#[repr(C)]
pub struct _alljoyn_abouticonproxy_handle(pub u8);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_aboutdata(pub isize);
impl ::core::default::Default for alljoyn_aboutdata {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_aboutdata {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_aboutdata {}
impl ::core::fmt::Debug for alljoyn_aboutdata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_aboutdata").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_aboutdata {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_aboutdatalistener(pub isize);
impl ::core::default::Default for alljoyn_aboutdatalistener {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_aboutdatalistener {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_aboutdatalistener {}
impl ::core::fmt::Debug for alljoyn_aboutdatalistener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_aboutdatalistener").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_aboutdatalistener {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_aboutdatalistener_callbacks {
    pub about_datalistener_getaboutdata: alljoyn_aboutdatalistener_getaboutdata_ptr,
    pub about_datalistener_getannouncedaboutdata: alljoyn_aboutdatalistener_getannouncedaboutdata_ptr,
}
impl ::core::marker::Copy for alljoyn_aboutdatalistener_callbacks {}
impl ::core::clone::Clone for alljoyn_aboutdatalistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_aboutdatalistener_callbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_aboutdatalistener_callbacks").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_aboutdatalistener_callbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_aboutdatalistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_aboutlistener(pub isize);
impl ::core::default::Default for alljoyn_aboutlistener {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_aboutlistener {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_aboutlistener {}
impl ::core::fmt::Debug for alljoyn_aboutlistener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_aboutlistener").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_aboutlistener {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_aboutlistener_callback {
    pub about_listener_announced: alljoyn_about_announced_ptr,
}
impl ::core::marker::Copy for alljoyn_aboutlistener_callback {}
impl ::core::clone::Clone for alljoyn_aboutlistener_callback {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_aboutlistener_callback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_aboutlistener_callback").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_aboutlistener_callback {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_aboutlistener_callback {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_aboutobj(pub isize);
impl ::core::default::Default for alljoyn_aboutobj {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_aboutobj {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_aboutobj {}
impl ::core::fmt::Debug for alljoyn_aboutobj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_aboutobj").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_aboutobj {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_aboutobjectdescription(pub isize);
impl ::core::default::Default for alljoyn_aboutobjectdescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_aboutobjectdescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_aboutobjectdescription {}
impl ::core::fmt::Debug for alljoyn_aboutobjectdescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_aboutobjectdescription").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_aboutobjectdescription {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_aboutproxy(pub isize);
impl ::core::default::Default for alljoyn_aboutproxy {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_aboutproxy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_aboutproxy {}
impl ::core::fmt::Debug for alljoyn_aboutproxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_aboutproxy").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_aboutproxy {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_applicationstatelistener(pub isize);
impl ::core::default::Default for alljoyn_applicationstatelistener {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_applicationstatelistener {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_applicationstatelistener {}
impl ::core::fmt::Debug for alljoyn_applicationstatelistener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_applicationstatelistener").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_applicationstatelistener {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_applicationstatelistener_callbacks {
    pub state: alljoyn_applicationstatelistener_state_ptr,
}
impl ::core::marker::Copy for alljoyn_applicationstatelistener_callbacks {}
impl ::core::clone::Clone for alljoyn_applicationstatelistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_applicationstatelistener_callbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_applicationstatelistener_callbacks").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_applicationstatelistener_callbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_applicationstatelistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_authlistener(pub isize);
impl ::core::default::Default for alljoyn_authlistener {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_authlistener {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_authlistener {}
impl ::core::fmt::Debug for alljoyn_authlistener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_authlistener").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_authlistener {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_authlistener_callbacks {
    pub request_credentials: alljoyn_authlistener_requestcredentials_ptr,
    pub verify_credentials: alljoyn_authlistener_verifycredentials_ptr,
    pub security_violation: alljoyn_authlistener_securityviolation_ptr,
    pub authentication_complete: alljoyn_authlistener_authenticationcomplete_ptr,
}
impl ::core::marker::Copy for alljoyn_authlistener_callbacks {}
impl ::core::clone::Clone for alljoyn_authlistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_authlistener_callbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_authlistener_callbacks").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_authlistener_callbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_authlistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_authlistenerasync_callbacks {
    pub request_credentials: alljoyn_authlistener_requestcredentialsasync_ptr,
    pub verify_credentials: alljoyn_authlistener_verifycredentialsasync_ptr,
    pub security_violation: alljoyn_authlistener_securityviolation_ptr,
    pub authentication_complete: alljoyn_authlistener_authenticationcomplete_ptr,
}
impl ::core::marker::Copy for alljoyn_authlistenerasync_callbacks {}
impl ::core::clone::Clone for alljoyn_authlistenerasync_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_authlistenerasync_callbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_authlistenerasync_callbacks").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_authlistenerasync_callbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_authlistenerasync_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_autopinger(pub isize);
impl ::core::default::Default for alljoyn_autopinger {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_autopinger {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_autopinger {}
impl ::core::fmt::Debug for alljoyn_autopinger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_autopinger").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_autopinger {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_busattachment(pub isize);
impl ::core::default::Default for alljoyn_busattachment {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_busattachment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_busattachment {}
impl ::core::fmt::Debug for alljoyn_busattachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_busattachment").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_busattachment {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_buslistener(pub isize);
impl ::core::default::Default for alljoyn_buslistener {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_buslistener {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_buslistener {}
impl ::core::fmt::Debug for alljoyn_buslistener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_buslistener").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_buslistener {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_buslistener_callbacks {
    pub listener_registered: alljoyn_buslistener_listener_registered_ptr,
    pub listener_unregistered: alljoyn_buslistener_listener_unregistered_ptr,
    pub found_advertised_name: alljoyn_buslistener_found_advertised_name_ptr,
    pub lost_advertised_name: alljoyn_buslistener_lost_advertised_name_ptr,
    pub name_owner_changed: alljoyn_buslistener_name_owner_changed_ptr,
    pub bus_stopping: alljoyn_buslistener_bus_stopping_ptr,
    pub bus_disconnected: alljoyn_buslistener_bus_disconnected_ptr,
    pub property_changed: alljoyn_buslistener_bus_prop_changed_ptr,
}
impl ::core::marker::Copy for alljoyn_buslistener_callbacks {}
impl ::core::clone::Clone for alljoyn_buslistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_buslistener_callbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_buslistener_callbacks").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_buslistener_callbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_buslistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_busobject(pub isize);
impl ::core::default::Default for alljoyn_busobject {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_busobject {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_busobject {}
impl ::core::fmt::Debug for alljoyn_busobject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_busobject").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_busobject {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_busobject_callbacks {
    pub property_get: alljoyn_busobject_prop_get_ptr,
    pub property_set: alljoyn_busobject_prop_set_ptr,
    pub object_registered: alljoyn_busobject_object_registration_ptr,
    pub object_unregistered: alljoyn_busobject_object_registration_ptr,
}
impl ::core::marker::Copy for alljoyn_busobject_callbacks {}
impl ::core::clone::Clone for alljoyn_busobject_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_busobject_callbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_busobject_callbacks").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_busobject_callbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_busobject_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_busobject_methodentry {
    pub member: *const alljoyn_interfacedescription_member,
    pub method_handler: alljoyn_messagereceiver_methodhandler_ptr,
}
impl ::core::marker::Copy for alljoyn_busobject_methodentry {}
impl ::core::clone::Clone for alljoyn_busobject_methodentry {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_busobject_methodentry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_busobject_methodentry").field("member", &self.member).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_busobject_methodentry {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_busobject_methodentry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_certificateid {
    pub serial: *mut u8,
    pub serialLen: usize,
    pub issuerPublicKey: *mut i8,
    pub issuerAki: *mut u8,
    pub issuerAkiLen: usize,
}
impl ::core::marker::Copy for alljoyn_certificateid {}
impl ::core::clone::Clone for alljoyn_certificateid {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_certificateid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_certificateid").field("serial", &self.serial).field("serialLen", &self.serialLen).field("issuerPublicKey", &self.issuerPublicKey).field("issuerAki", &self.issuerAki).field("issuerAkiLen", &self.issuerAkiLen).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_certificateid {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for alljoyn_certificateid {
    fn eq(&self, other: &Self) -> bool {
        self.serial == other.serial && self.serialLen == other.serialLen && self.issuerPublicKey == other.issuerPublicKey && self.issuerAki == other.issuerAki && self.issuerAkiLen == other.issuerAkiLen
    }
}
impl ::core::cmp::Eq for alljoyn_certificateid {}
impl ::core::default::Default for alljoyn_certificateid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_certificateidarray {
    pub count: usize,
    pub ids: *mut alljoyn_certificateid,
}
impl ::core::marker::Copy for alljoyn_certificateidarray {}
impl ::core::clone::Clone for alljoyn_certificateidarray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_certificateidarray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_certificateidarray").field("count", &self.count).field("ids", &self.ids).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_certificateidarray {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for alljoyn_certificateidarray {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.ids == other.ids
    }
}
impl ::core::cmp::Eq for alljoyn_certificateidarray {}
impl ::core::default::Default for alljoyn_certificateidarray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_credentials(pub isize);
impl ::core::default::Default for alljoyn_credentials {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_credentials {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_credentials {}
impl ::core::fmt::Debug for alljoyn_credentials {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_credentials").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_credentials {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_interfacedescription(pub isize);
impl ::core::default::Default for alljoyn_interfacedescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_interfacedescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_interfacedescription {}
impl ::core::fmt::Debug for alljoyn_interfacedescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_interfacedescription").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_interfacedescription {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_interfacedescription_member {
    pub iface: alljoyn_interfacedescription,
    pub memberType: alljoyn_messagetype,
    pub name: ::windows::core::PCSTR,
    pub signature: ::windows::core::PCSTR,
    pub returnSignature: ::windows::core::PCSTR,
    pub argNames: ::windows::core::PCSTR,
    pub internal_member: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for alljoyn_interfacedescription_member {}
impl ::core::clone::Clone for alljoyn_interfacedescription_member {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_interfacedescription_member {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_interfacedescription_member").field("iface", &self.iface).field("memberType", &self.memberType).field("name", &self.name).field("signature", &self.signature).field("returnSignature", &self.returnSignature).field("argNames", &self.argNames).field("internal_member", &self.internal_member).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_interfacedescription_member {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for alljoyn_interfacedescription_member {
    fn eq(&self, other: &Self) -> bool {
        self.iface == other.iface && self.memberType == other.memberType && self.name == other.name && self.signature == other.signature && self.returnSignature == other.returnSignature && self.argNames == other.argNames && self.internal_member == other.internal_member
    }
}
impl ::core::cmp::Eq for alljoyn_interfacedescription_member {}
impl ::core::default::Default for alljoyn_interfacedescription_member {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_interfacedescription_property {
    pub name: ::windows::core::PCSTR,
    pub signature: ::windows::core::PCSTR,
    pub access: u8,
    pub internal_property: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for alljoyn_interfacedescription_property {}
impl ::core::clone::Clone for alljoyn_interfacedescription_property {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_interfacedescription_property {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_interfacedescription_property").field("name", &self.name).field("signature", &self.signature).field("access", &self.access).field("internal_property", &self.internal_property).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_interfacedescription_property {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for alljoyn_interfacedescription_property {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.signature == other.signature && self.access == other.access && self.internal_property == other.internal_property
    }
}
impl ::core::cmp::Eq for alljoyn_interfacedescription_property {}
impl ::core::default::Default for alljoyn_interfacedescription_property {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_keystore(pub isize);
impl ::core::default::Default for alljoyn_keystore {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_keystore {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_keystore {}
impl ::core::fmt::Debug for alljoyn_keystore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_keystore").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_keystore {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_keystorelistener(pub isize);
impl ::core::default::Default for alljoyn_keystorelistener {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_keystorelistener {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_keystorelistener {}
impl ::core::fmt::Debug for alljoyn_keystorelistener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_keystorelistener").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_keystorelistener {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_keystorelistener_callbacks {
    pub load_request: alljoyn_keystorelistener_loadrequest_ptr,
    pub store_request: alljoyn_keystorelistener_storerequest_ptr,
}
impl ::core::marker::Copy for alljoyn_keystorelistener_callbacks {}
impl ::core::clone::Clone for alljoyn_keystorelistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_keystorelistener_callbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_keystorelistener_callbacks").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_keystorelistener_callbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_keystorelistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_keystorelistener_with_synchronization_callbacks {
    pub load_request: alljoyn_keystorelistener_loadrequest_ptr,
    pub store_request: alljoyn_keystorelistener_storerequest_ptr,
    pub acquire_exclusive_lock: alljoyn_keystorelistener_acquireexclusivelock_ptr,
    pub release_exclusive_lock: alljoyn_keystorelistener_releaseexclusivelock_ptr,
}
impl ::core::marker::Copy for alljoyn_keystorelistener_with_synchronization_callbacks {}
impl ::core::clone::Clone for alljoyn_keystorelistener_with_synchronization_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_keystorelistener_with_synchronization_callbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_keystorelistener_with_synchronization_callbacks").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_keystorelistener_with_synchronization_callbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_keystorelistener_with_synchronization_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_manifestarray {
    pub count: usize,
    pub xmls: *mut *mut i8,
}
impl ::core::marker::Copy for alljoyn_manifestarray {}
impl ::core::clone::Clone for alljoyn_manifestarray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_manifestarray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_manifestarray").field("count", &self.count).field("xmls", &self.xmls).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_manifestarray {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for alljoyn_manifestarray {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.xmls == other.xmls
    }
}
impl ::core::cmp::Eq for alljoyn_manifestarray {}
impl ::core::default::Default for alljoyn_manifestarray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_message(pub isize);
impl ::core::default::Default for alljoyn_message {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_message {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_message {}
impl ::core::fmt::Debug for alljoyn_message {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_message").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_message {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_msgarg(pub isize);
impl ::core::default::Default for alljoyn_msgarg {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_msgarg {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_msgarg {}
impl ::core::fmt::Debug for alljoyn_msgarg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_msgarg").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_msgarg {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_observer(pub isize);
impl ::core::default::Default for alljoyn_observer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_observer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_observer {}
impl ::core::fmt::Debug for alljoyn_observer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_observer").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_observer {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_observerlistener(pub isize);
impl ::core::default::Default for alljoyn_observerlistener {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_observerlistener {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_observerlistener {}
impl ::core::fmt::Debug for alljoyn_observerlistener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_observerlistener").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_observerlistener {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_observerlistener_callback {
    pub object_discovered: alljoyn_observer_object_discovered_ptr,
    pub object_lost: alljoyn_observer_object_lost_ptr,
}
impl ::core::marker::Copy for alljoyn_observerlistener_callback {}
impl ::core::clone::Clone for alljoyn_observerlistener_callback {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_observerlistener_callback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_observerlistener_callback").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_observerlistener_callback {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_observerlistener_callback {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_permissionconfigurationlistener(pub isize);
impl ::core::default::Default for alljoyn_permissionconfigurationlistener {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_permissionconfigurationlistener {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_permissionconfigurationlistener {}
impl ::core::fmt::Debug for alljoyn_permissionconfigurationlistener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_permissionconfigurationlistener").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_permissionconfigurationlistener {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_permissionconfigurationlistener_callbacks {
    pub factory_reset: alljoyn_permissionconfigurationlistener_factoryreset_ptr,
    pub policy_changed: alljoyn_permissionconfigurationlistener_policychanged_ptr,
    pub start_management: alljoyn_permissionconfigurationlistener_startmanagement_ptr,
    pub end_management: alljoyn_permissionconfigurationlistener_endmanagement_ptr,
}
impl ::core::marker::Copy for alljoyn_permissionconfigurationlistener_callbacks {}
impl ::core::clone::Clone for alljoyn_permissionconfigurationlistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_permissionconfigurationlistener_callbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_permissionconfigurationlistener_callbacks").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_permissionconfigurationlistener_callbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_permissionconfigurationlistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_permissionconfigurator(pub isize);
impl ::core::default::Default for alljoyn_permissionconfigurator {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_permissionconfigurator {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_permissionconfigurator {}
impl ::core::fmt::Debug for alljoyn_permissionconfigurator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_permissionconfigurator").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_permissionconfigurator {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_pinglistener(pub isize);
impl ::core::default::Default for alljoyn_pinglistener {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_pinglistener {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_pinglistener {}
impl ::core::fmt::Debug for alljoyn_pinglistener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_pinglistener").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_pinglistener {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_pinglistener_callback {
    pub destination_found: alljoyn_autopinger_destination_found_ptr,
    pub destination_lost: alljoyn_autopinger_destination_lost_ptr,
}
impl ::core::marker::Copy for alljoyn_pinglistener_callback {}
impl ::core::clone::Clone for alljoyn_pinglistener_callback {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_pinglistener_callback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_pinglistener_callback").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_pinglistener_callback {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_pinglistener_callback {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_proxybusobject(pub isize);
impl ::core::default::Default for alljoyn_proxybusobject {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_proxybusobject {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_proxybusobject {}
impl ::core::fmt::Debug for alljoyn_proxybusobject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_proxybusobject").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_proxybusobject {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_proxybusobject_ref(pub isize);
impl ::core::default::Default for alljoyn_proxybusobject_ref {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_proxybusobject_ref {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_proxybusobject_ref {}
impl ::core::fmt::Debug for alljoyn_proxybusobject_ref {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_proxybusobject_ref").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_proxybusobject_ref {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_securityapplicationproxy(pub isize);
impl ::core::default::Default for alljoyn_securityapplicationproxy {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_securityapplicationproxy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_securityapplicationproxy {}
impl ::core::fmt::Debug for alljoyn_securityapplicationproxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_securityapplicationproxy").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_securityapplicationproxy {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_sessionlistener(pub isize);
impl ::core::default::Default for alljoyn_sessionlistener {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_sessionlistener {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_sessionlistener {}
impl ::core::fmt::Debug for alljoyn_sessionlistener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_sessionlistener").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_sessionlistener {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_sessionlistener_callbacks {
    pub session_lost: alljoyn_sessionlistener_sessionlost_ptr,
    pub session_member_added: alljoyn_sessionlistener_sessionmemberadded_ptr,
    pub session_member_removed: alljoyn_sessionlistener_sessionmemberremoved_ptr,
}
impl ::core::marker::Copy for alljoyn_sessionlistener_callbacks {}
impl ::core::clone::Clone for alljoyn_sessionlistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_sessionlistener_callbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_sessionlistener_callbacks").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_sessionlistener_callbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_sessionlistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_sessionopts(pub isize);
impl ::core::default::Default for alljoyn_sessionopts {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_sessionopts {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_sessionopts {}
impl ::core::fmt::Debug for alljoyn_sessionopts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_sessionopts").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_sessionopts {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct alljoyn_sessionportlistener(pub isize);
impl ::core::default::Default for alljoyn_sessionportlistener {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for alljoyn_sessionportlistener {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for alljoyn_sessionportlistener {}
impl ::core::fmt::Debug for alljoyn_sessionportlistener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_sessionportlistener").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_sessionportlistener {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub struct alljoyn_sessionportlistener_callbacks {
    pub accept_session_joiner: alljoyn_sessionportlistener_acceptsessionjoiner_ptr,
    pub session_joined: alljoyn_sessionportlistener_sessionjoined_ptr,
}
impl ::core::marker::Copy for alljoyn_sessionportlistener_callbacks {}
impl ::core::clone::Clone for alljoyn_sessionportlistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for alljoyn_sessionportlistener_callbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_sessionportlistener_callbacks").finish()
    }
}
impl ::windows::core::TypeKind for alljoyn_sessionportlistener_callbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for alljoyn_sessionportlistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_about_announced_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, busname: ::windows::core::PCSTR, version: u16, port: u16, objectdescriptionarg: alljoyn_msgarg, aboutdataarg: alljoyn_msgarg) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_aboutdatalistener_getaboutdata_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, msgarg: alljoyn_msgarg, language: ::windows::core::PCSTR) -> QStatus>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_aboutdatalistener_getannouncedaboutdata_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, msgarg: alljoyn_msgarg) -> QStatus>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_applicationstatelistener_state_ptr = ::core::option::Option<unsafe extern "system" fn(busname: *mut i8, publickey: *mut i8, applicationstate: alljoyn_applicationstate, context: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_authlistener_authenticationcomplete_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, authmechanism: ::windows::core::PCSTR, peername: ::windows::core::PCSTR, success: i32) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_authlistener_requestcredentials_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, authmechanism: ::windows::core::PCSTR, peername: ::windows::core::PCSTR, authcount: u16, username: ::windows::core::PCSTR, credmask: u16, credentials: alljoyn_credentials) -> i32>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_authlistener_requestcredentialsasync_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_authlistener, authmechanism: ::windows::core::PCSTR, peername: ::windows::core::PCSTR, authcount: u16, username: ::windows::core::PCSTR, credmask: u16, authcontext: *mut ::core::ffi::c_void) -> QStatus>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_authlistener_securityviolation_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, status: QStatus, msg: alljoyn_message) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_authlistener_verifycredentials_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, authmechanism: ::windows::core::PCSTR, peername: ::windows::core::PCSTR, credentials: alljoyn_credentials) -> i32>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_authlistener_verifycredentialsasync_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_authlistener, authmechanism: ::windows::core::PCSTR, peername: ::windows::core::PCSTR, credentials: alljoyn_credentials, authcontext: *mut ::core::ffi::c_void) -> QStatus>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_autopinger_destination_found_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, group: ::windows::core::PCSTR, destination: ::windows::core::PCSTR) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_autopinger_destination_lost_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, group: ::windows::core::PCSTR, destination: ::windows::core::PCSTR) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_busattachment_joinsessioncb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, sessionid: u32, opts: alljoyn_sessionopts, context: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_busattachment_setlinktimeoutcb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, timeout: u32, context: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_buslistener_bus_disconnected_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_buslistener_bus_prop_changed_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, prop_name: ::windows::core::PCSTR, prop_value: alljoyn_msgarg) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_buslistener_bus_stopping_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_buslistener_found_advertised_name_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, name: ::windows::core::PCSTR, transport: u16, nameprefix: ::windows::core::PCSTR) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_buslistener_listener_registered_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, bus: alljoyn_busattachment) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_buslistener_listener_unregistered_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_buslistener_lost_advertised_name_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, name: ::windows::core::PCSTR, transport: u16, nameprefix: ::windows::core::PCSTR) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_buslistener_name_owner_changed_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, busname: ::windows::core::PCSTR, previousowner: ::windows::core::PCSTR, newowner: ::windows::core::PCSTR) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_busobject_object_registration_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_busobject_prop_get_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, ifcname: ::windows::core::PCSTR, propname: ::windows::core::PCSTR, val: alljoyn_msgarg) -> QStatus>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_busobject_prop_set_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, ifcname: ::windows::core::PCSTR, propname: ::windows::core::PCSTR, val: alljoyn_msgarg) -> QStatus>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_interfacedescription_translation_callback_ptr = ::core::option::Option<unsafe extern "system" fn(sourcelanguage: ::windows::core::PCSTR, targetlanguage: ::windows::core::PCSTR, sourcetext: ::windows::core::PCSTR) -> ::windows::core::PCSTR>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_keystorelistener_acquireexclusivelock_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_keystorelistener) -> QStatus>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_keystorelistener_loadrequest_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_keystorelistener, keystore: alljoyn_keystore) -> QStatus>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_keystorelistener_releaseexclusivelock_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_keystorelistener) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_keystorelistener_storerequest_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_keystorelistener, keystore: alljoyn_keystore) -> QStatus>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_messagereceiver_methodhandler_ptr = ::core::option::Option<unsafe extern "system" fn(bus: alljoyn_busobject, member: *const alljoyn_interfacedescription_member, message: alljoyn_message) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_messagereceiver_replyhandler_ptr = ::core::option::Option<unsafe extern "system" fn(message: alljoyn_message, context: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_messagereceiver_signalhandler_ptr = ::core::option::Option<unsafe extern "system" fn(member: *const alljoyn_interfacedescription_member, srcpath: ::windows::core::PCSTR, message: alljoyn_message) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_observer_object_discovered_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, proxyref: alljoyn_proxybusobject_ref) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_observer_object_lost_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, proxyref: alljoyn_proxybusobject_ref) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_permissionconfigurationlistener_endmanagement_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_permissionconfigurationlistener_factoryreset_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> QStatus>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_permissionconfigurationlistener_policychanged_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_permissionconfigurationlistener_startmanagement_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_proxybusobject_listener_getallpropertiescb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, values: alljoyn_msgarg, context: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_proxybusobject_listener_getpropertycb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, value: alljoyn_msgarg, context: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_proxybusobject_listener_introspectcb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, context: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_proxybusobject_listener_propertieschanged_ptr = ::core::option::Option<unsafe extern "system" fn(obj: alljoyn_proxybusobject, ifacename: ::windows::core::PCSTR, changed: alljoyn_msgarg, invalidated: alljoyn_msgarg, context: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_proxybusobject_listener_setpropertycb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, context: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_sessionlistener_sessionlost_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionid: u32, reason: alljoyn_sessionlostreason) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_sessionlistener_sessionmemberadded_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionid: u32, uniquename: ::windows::core::PCSTR) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_sessionlistener_sessionmemberremoved_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionid: u32, uniquename: ::windows::core::PCSTR) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_sessionportlistener_acceptsessionjoiner_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionport: u16, joiner: ::windows::core::PCSTR, opts: alljoyn_sessionopts) -> i32>;
#[doc = "*Required features: `\"Win32_Devices_AllJoyn\"`*"]
pub type alljoyn_sessionportlistener_sessionjoined_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionport: u16, id: u32, joiner: ::windows::core::PCSTR) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
