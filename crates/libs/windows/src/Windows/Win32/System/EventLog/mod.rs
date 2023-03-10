#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupEventLogA<P0, P1>(heventlog: P0, lpbackupfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventLogHandle>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BackupEventLogA ( heventlog : EventLogHandle , lpbackupfilename : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    BackupEventLogA(heventlog.into_param().abi(), lpbackupfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupEventLogW<P0, P1>(heventlog: P0, lpbackupfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventLogHandle>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BackupEventLogW ( heventlog : EventLogHandle , lpbackupfilename : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    BackupEventLogW(heventlog.into_param().abi(), lpbackupfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClearEventLogA<P0, P1>(heventlog: P0, lpbackupfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventLogHandle>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ClearEventLogA ( heventlog : EventLogHandle , lpbackupfilename : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    ClearEventLogA(heventlog.into_param().abi(), lpbackupfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClearEventLogW<P0, P1>(heventlog: P0, lpbackupfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventLogHandle>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ClearEventLogW ( heventlog : EventLogHandle , lpbackupfilename : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    ClearEventLogW(heventlog.into_param().abi(), lpbackupfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseEventLog<P0>(heventlog: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventLogHandle>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CloseEventLog ( heventlog : EventLogHandle ) -> super::super::Foundation:: BOOL );
    CloseEventLog(heventlog.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeregisterEventSource<P0>(heventlog: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventSourceHandle>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn DeregisterEventSource ( heventlog : EventSourceHandle ) -> super::super::Foundation:: BOOL );
    DeregisterEventSource(heventlog.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtArchiveExportedLog<P0, P1>(session: P0, logfilepath: P1, locale: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtArchiveExportedLog ( session : EVT_HANDLE , logfilepath : :: windows::core::PCWSTR , locale : u32 , flags : u32 ) -> super::super::Foundation:: BOOL );
    EvtArchiveExportedLog(session.into_param().abi(), logfilepath.into_param().abi(), locale, flags)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtCancel<P0>(object: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtCancel ( object : EVT_HANDLE ) -> super::super::Foundation:: BOOL );
    EvtCancel(object.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtClearLog<P0, P1, P2>(session: P0, channelpath: P1, targetfilepath: P2, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtClearLog ( session : EVT_HANDLE , channelpath : :: windows::core::PCWSTR , targetfilepath : :: windows::core::PCWSTR , flags : u32 ) -> super::super::Foundation:: BOOL );
    EvtClearLog(session.into_param().abi(), channelpath.into_param().abi(), targetfilepath.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtClose<P0>(object: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtClose ( object : EVT_HANDLE ) -> super::super::Foundation:: BOOL );
    EvtClose(object.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtCreateBookmark<P0>(bookmarkxml: P0) -> ::windows::core::Result<EVT_HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtCreateBookmark ( bookmarkxml : :: windows::core::PCWSTR ) -> EVT_HANDLE );
    let result__ = EvtCreateBookmark(bookmarkxml.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtCreateRenderContext(valuepaths: ::core::option::Option<&[::windows::core::PCWSTR]>, flags: u32) -> ::windows::core::Result<EVT_HANDLE> {
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtCreateRenderContext ( valuepathscount : u32 , valuepaths : *const :: windows::core::PCWSTR , flags : u32 ) -> EVT_HANDLE );
    let result__ = EvtCreateRenderContext(valuepaths.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(valuepaths.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtExportLog<P0, P1, P2, P3>(session: P0, path: P1, query: P2, targetfilepath: P3, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtExportLog ( session : EVT_HANDLE , path : :: windows::core::PCWSTR , query : :: windows::core::PCWSTR , targetfilepath : :: windows::core::PCWSTR , flags : u32 ) -> super::super::Foundation:: BOOL );
    EvtExportLog(session.into_param().abi(), path.into_param().abi(), query.into_param().abi(), targetfilepath.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtFormatMessage<P0, P1>(publishermetadata: P0, event: P1, messageid: u32, values: ::core::option::Option<&[EVT_VARIANT]>, flags: u32, buffer: ::core::option::Option<&mut [u16]>, bufferused: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtFormatMessage ( publishermetadata : EVT_HANDLE , event : EVT_HANDLE , messageid : u32 , valuecount : u32 , values : *const EVT_VARIANT , flags : u32 , buffersize : u32 , buffer : :: windows::core::PWSTR , bufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtFormatMessage(publishermetadata.into_param().abi(), event.into_param().abi(), messageid, values.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(values.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), flags, buffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(buffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bufferused)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetChannelConfigProperty<P0>(channelconfig: P0, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: ::core::option::Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtGetChannelConfigProperty ( channelconfig : EVT_HANDLE , propertyid : EVT_CHANNEL_CONFIG_PROPERTY_ID , flags : u32 , propertyvaluebuffersize : u32 , propertyvaluebuffer : *mut EVT_VARIANT , propertyvaluebufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtGetChannelConfigProperty(channelconfig.into_param().abi(), propertyid, flags, propertyvaluebuffersize, ::core::mem::transmute(propertyvaluebuffer.unwrap_or(::std::ptr::null_mut())), propertyvaluebufferused)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetEventInfo<P0>(event: P0, propertyid: EVT_EVENT_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: ::core::option::Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtGetEventInfo ( event : EVT_HANDLE , propertyid : EVT_EVENT_PROPERTY_ID , propertyvaluebuffersize : u32 , propertyvaluebuffer : *mut EVT_VARIANT , propertyvaluebufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtGetEventInfo(event.into_param().abi(), propertyid, propertyvaluebuffersize, ::core::mem::transmute(propertyvaluebuffer.unwrap_or(::std::ptr::null_mut())), propertyvaluebufferused)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetEventMetadataProperty<P0>(eventmetadata: P0, propertyid: EVT_EVENT_METADATA_PROPERTY_ID, flags: u32, eventmetadatapropertybuffersize: u32, eventmetadatapropertybuffer: ::core::option::Option<*mut EVT_VARIANT>, eventmetadatapropertybufferused: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtGetEventMetadataProperty ( eventmetadata : EVT_HANDLE , propertyid : EVT_EVENT_METADATA_PROPERTY_ID , flags : u32 , eventmetadatapropertybuffersize : u32 , eventmetadatapropertybuffer : *mut EVT_VARIANT , eventmetadatapropertybufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtGetEventMetadataProperty(eventmetadata.into_param().abi(), propertyid, flags, eventmetadatapropertybuffersize, ::core::mem::transmute(eventmetadatapropertybuffer.unwrap_or(::std::ptr::null_mut())), eventmetadatapropertybufferused)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtGetExtendedStatus(buffer: ::core::option::Option<&mut [u16]>, bufferused: *mut u32) -> u32 {
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtGetExtendedStatus ( buffersize : u32 , buffer : :: windows::core::PWSTR , bufferused : *mut u32 ) -> u32 );
    EvtGetExtendedStatus(buffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(buffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bufferused)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetLogInfo<P0>(log: P0, propertyid: EVT_LOG_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: ::core::option::Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtGetLogInfo ( log : EVT_HANDLE , propertyid : EVT_LOG_PROPERTY_ID , propertyvaluebuffersize : u32 , propertyvaluebuffer : *mut EVT_VARIANT , propertyvaluebufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtGetLogInfo(log.into_param().abi(), propertyid, propertyvaluebuffersize, ::core::mem::transmute(propertyvaluebuffer.unwrap_or(::std::ptr::null_mut())), propertyvaluebufferused)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetObjectArrayProperty(objectarray: isize, propertyid: u32, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: ::core::option::Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtGetObjectArrayProperty ( objectarray : isize , propertyid : u32 , arrayindex : u32 , flags : u32 , propertyvaluebuffersize : u32 , propertyvaluebuffer : *mut EVT_VARIANT , propertyvaluebufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtGetObjectArrayProperty(objectarray, propertyid, arrayindex, flags, propertyvaluebuffersize, ::core::mem::transmute(propertyvaluebuffer.unwrap_or(::std::ptr::null_mut())), propertyvaluebufferused)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetObjectArraySize(objectarray: isize, objectarraysize: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtGetObjectArraySize ( objectarray : isize , objectarraysize : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtGetObjectArraySize(objectarray, objectarraysize)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetPublisherMetadataProperty<P0>(publishermetadata: P0, propertyid: EVT_PUBLISHER_METADATA_PROPERTY_ID, flags: u32, publishermetadatapropertybuffersize: u32, publishermetadatapropertybuffer: ::core::option::Option<*mut EVT_VARIANT>, publishermetadatapropertybufferused: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtGetPublisherMetadataProperty ( publishermetadata : EVT_HANDLE , propertyid : EVT_PUBLISHER_METADATA_PROPERTY_ID , flags : u32 , publishermetadatapropertybuffersize : u32 , publishermetadatapropertybuffer : *mut EVT_VARIANT , publishermetadatapropertybufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtGetPublisherMetadataProperty(publishermetadata.into_param().abi(), propertyid, flags, publishermetadatapropertybuffersize, ::core::mem::transmute(publishermetadatapropertybuffer.unwrap_or(::std::ptr::null_mut())), publishermetadatapropertybufferused)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetQueryInfo<P0>(queryorsubscription: P0, propertyid: EVT_QUERY_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: ::core::option::Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtGetQueryInfo ( queryorsubscription : EVT_HANDLE , propertyid : EVT_QUERY_PROPERTY_ID , propertyvaluebuffersize : u32 , propertyvaluebuffer : *mut EVT_VARIANT , propertyvaluebufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtGetQueryInfo(queryorsubscription.into_param().abi(), propertyid, propertyvaluebuffersize, ::core::mem::transmute(propertyvaluebuffer.unwrap_or(::std::ptr::null_mut())), propertyvaluebufferused)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtNext<P0>(resultset: P0, events: &mut [isize], timeout: u32, flags: u32, returned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtNext ( resultset : EVT_HANDLE , eventssize : u32 , events : *mut isize , timeout : u32 , flags : u32 , returned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtNext(resultset.into_param().abi(), events.len() as _, ::core::mem::transmute(events.as_ptr()), timeout, flags, returned)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtNextChannelPath<P0>(channelenum: P0, channelpathbuffer: ::core::option::Option<&mut [u16]>, channelpathbufferused: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtNextChannelPath ( channelenum : EVT_HANDLE , channelpathbuffersize : u32 , channelpathbuffer : :: windows::core::PWSTR , channelpathbufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtNextChannelPath(channelenum.into_param().abi(), channelpathbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(channelpathbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), channelpathbufferused)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtNextEventMetadata<P0>(eventmetadataenum: P0, flags: u32) -> ::windows::core::Result<EVT_HANDLE>
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtNextEventMetadata ( eventmetadataenum : EVT_HANDLE , flags : u32 ) -> EVT_HANDLE );
    let result__ = EvtNextEventMetadata(eventmetadataenum.into_param().abi(), flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtNextPublisherId<P0>(publisherenum: P0, publisheridbuffer: ::core::option::Option<&mut [u16]>, publisheridbufferused: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtNextPublisherId ( publisherenum : EVT_HANDLE , publisheridbuffersize : u32 , publisheridbuffer : :: windows::core::PWSTR , publisheridbufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtNextPublisherId(publisherenum.into_param().abi(), publisheridbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(publisheridbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), publisheridbufferused)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtOpenChannelConfig<P0, P1>(session: P0, channelpath: P1, flags: u32) -> ::windows::core::Result<EVT_HANDLE>
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtOpenChannelConfig ( session : EVT_HANDLE , channelpath : :: windows::core::PCWSTR , flags : u32 ) -> EVT_HANDLE );
    let result__ = EvtOpenChannelConfig(session.into_param().abi(), channelpath.into_param().abi(), flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtOpenChannelEnum<P0>(session: P0, flags: u32) -> ::windows::core::Result<EVT_HANDLE>
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtOpenChannelEnum ( session : EVT_HANDLE , flags : u32 ) -> EVT_HANDLE );
    let result__ = EvtOpenChannelEnum(session.into_param().abi(), flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtOpenEventMetadataEnum<P0>(publishermetadata: P0, flags: u32) -> ::windows::core::Result<EVT_HANDLE>
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtOpenEventMetadataEnum ( publishermetadata : EVT_HANDLE , flags : u32 ) -> EVT_HANDLE );
    let result__ = EvtOpenEventMetadataEnum(publishermetadata.into_param().abi(), flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtOpenLog<P0, P1>(session: P0, path: P1, flags: u32) -> ::windows::core::Result<EVT_HANDLE>
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtOpenLog ( session : EVT_HANDLE , path : :: windows::core::PCWSTR , flags : u32 ) -> EVT_HANDLE );
    let result__ = EvtOpenLog(session.into_param().abi(), path.into_param().abi(), flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtOpenPublisherEnum<P0>(session: P0, flags: u32) -> ::windows::core::Result<EVT_HANDLE>
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtOpenPublisherEnum ( session : EVT_HANDLE , flags : u32 ) -> EVT_HANDLE );
    let result__ = EvtOpenPublisherEnum(session.into_param().abi(), flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtOpenPublisherMetadata<P0, P1, P2>(session: P0, publisherid: P1, logfilepath: P2, locale: u32, flags: u32) -> ::windows::core::Result<EVT_HANDLE>
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtOpenPublisherMetadata ( session : EVT_HANDLE , publisherid : :: windows::core::PCWSTR , logfilepath : :: windows::core::PCWSTR , locale : u32 , flags : u32 ) -> EVT_HANDLE );
    let result__ = EvtOpenPublisherMetadata(session.into_param().abi(), publisherid.into_param().abi(), logfilepath.into_param().abi(), locale, flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtOpenSession(loginclass: EVT_LOGIN_CLASS, login: *const ::core::ffi::c_void, timeout: u32, flags: u32) -> ::windows::core::Result<EVT_HANDLE> {
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtOpenSession ( loginclass : EVT_LOGIN_CLASS , login : *const ::core::ffi::c_void , timeout : u32 , flags : u32 ) -> EVT_HANDLE );
    let result__ = EvtOpenSession(loginclass, login, timeout, flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtQuery<P0, P1, P2>(session: P0, path: P1, query: P2, flags: u32) -> ::windows::core::Result<EVT_HANDLE>
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtQuery ( session : EVT_HANDLE , path : :: windows::core::PCWSTR , query : :: windows::core::PCWSTR , flags : u32 ) -> EVT_HANDLE );
    let result__ = EvtQuery(session.into_param().abi(), path.into_param().abi(), query.into_param().abi(), flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtRender<P0, P1>(context: P0, fragment: P1, flags: u32, buffersize: u32, buffer: ::core::option::Option<*mut ::core::ffi::c_void>, bufferused: *mut u32, propertycount: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtRender ( context : EVT_HANDLE , fragment : EVT_HANDLE , flags : u32 , buffersize : u32 , buffer : *mut ::core::ffi::c_void , bufferused : *mut u32 , propertycount : *mut u32 ) -> super::super::Foundation:: BOOL );
    EvtRender(context.into_param().abi(), fragment.into_param().abi(), flags, buffersize, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), bufferused, propertycount)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtSaveChannelConfig<P0>(channelconfig: P0, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtSaveChannelConfig ( channelconfig : EVT_HANDLE , flags : u32 ) -> super::super::Foundation:: BOOL );
    EvtSaveChannelConfig(channelconfig.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtSeek<P0, P1>(resultset: P0, position: i64, bookmark: P1, timeout: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtSeek ( resultset : EVT_HANDLE , position : i64 , bookmark : EVT_HANDLE , timeout : u32 , flags : u32 ) -> super::super::Foundation:: BOOL );
    EvtSeek(resultset.into_param().abi(), position, bookmark.into_param().abi(), timeout, flags)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtSetChannelConfigProperty<P0>(channelconfig: P0, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvalue: *const EVT_VARIANT) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtSetChannelConfigProperty ( channelconfig : EVT_HANDLE , propertyid : EVT_CHANNEL_CONFIG_PROPERTY_ID , flags : u32 , propertyvalue : *const EVT_VARIANT ) -> super::super::Foundation:: BOOL );
    EvtSetChannelConfigProperty(channelconfig.into_param().abi(), propertyid, flags, propertyvalue)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtSubscribe<P0, P1, P2, P3, P4>(session: P0, signalevent: P1, channelpath: P2, query: P3, bookmark: P4, context: ::core::option::Option<*const ::core::ffi::c_void>, callback: EVT_SUBSCRIBE_CALLBACK, flags: u32) -> ::windows::core::Result<EVT_HANDLE>
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtSubscribe ( session : EVT_HANDLE , signalevent : super::super::Foundation:: HANDLE , channelpath : :: windows::core::PCWSTR , query : :: windows::core::PCWSTR , bookmark : EVT_HANDLE , context : *const ::core::ffi::c_void , callback : EVT_SUBSCRIBE_CALLBACK , flags : u32 ) -> EVT_HANDLE );
    let result__ = EvtSubscribe(session.into_param().abi(), signalevent.into_param().abi(), channelpath.into_param().abi(), query.into_param().abi(), bookmark.into_param().abi(), ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), callback, flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtUpdateBookmark<P0, P1>(bookmark: P0, event: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EVT_HANDLE>,
    P1: ::windows::core::IntoParam<EVT_HANDLE>,
{
    ::windows::imp::link ! ( "wevtapi.dll""system" fn EvtUpdateBookmark ( bookmark : EVT_HANDLE , event : EVT_HANDLE ) -> super::super::Foundation:: BOOL );
    EvtUpdateBookmark(bookmark.into_param().abi(), event.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEventLogInformation<P0>(heventlog: P0, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventLogHandle>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetEventLogInformation ( heventlog : EventLogHandle , dwinfolevel : u32 , lpbuffer : *mut ::core::ffi::c_void , cbbufsize : u32 , pcbbytesneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetEventLogInformation(heventlog.into_param().abi(), dwinfolevel, lpbuffer, cbbufsize, pcbbytesneeded)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumberOfEventLogRecords<P0>(heventlog: P0, numberofrecords: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventLogHandle>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetNumberOfEventLogRecords ( heventlog : EventLogHandle , numberofrecords : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetNumberOfEventLogRecords(heventlog.into_param().abi(), numberofrecords)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOldestEventLogRecord<P0>(heventlog: P0, oldestrecord: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventLogHandle>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetOldestEventLogRecord ( heventlog : EventLogHandle , oldestrecord : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetOldestEventLogRecord(heventlog.into_param().abi(), oldestrecord)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NotifyChangeEventLog<P0, P1>(heventlog: P0, hevent: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventLogHandle>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn NotifyChangeEventLog ( heventlog : EventLogHandle , hevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    NotifyChangeEventLog(heventlog.into_param().abi(), hevent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn OpenBackupEventLogA<P0, P1>(lpuncservername: P0, lpfilename: P1) -> ::windows::core::Result<EventLogHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn OpenBackupEventLogA ( lpuncservername : :: windows::core::PCSTR , lpfilename : :: windows::core::PCSTR ) -> EventLogHandle );
    let result__ = OpenBackupEventLogA(lpuncservername.into_param().abi(), lpfilename.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn OpenBackupEventLogW<P0, P1>(lpuncservername: P0, lpfilename: P1) -> ::windows::core::Result<EventLogHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn OpenBackupEventLogW ( lpuncservername : :: windows::core::PCWSTR , lpfilename : :: windows::core::PCWSTR ) -> EventLogHandle );
    let result__ = OpenBackupEventLogW(lpuncservername.into_param().abi(), lpfilename.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn OpenEventLogA<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> ::windows::core::Result<EventLogHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn OpenEventLogA ( lpuncservername : :: windows::core::PCSTR , lpsourcename : :: windows::core::PCSTR ) -> EventLogHandle );
    let result__ = OpenEventLogA(lpuncservername.into_param().abi(), lpsourcename.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn OpenEventLogW<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> ::windows::core::Result<EventLogHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn OpenEventLogW ( lpuncservername : :: windows::core::PCWSTR , lpsourcename : :: windows::core::PCWSTR ) -> EventLogHandle );
    let result__ = OpenEventLogW(lpuncservername.into_param().abi(), lpsourcename.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadEventLogA<P0>(heventlog: P0, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventLogHandle>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ReadEventLogA ( heventlog : EventLogHandle , dwreadflags : READ_EVENT_LOG_READ_FLAGS , dwrecordoffset : u32 , lpbuffer : *mut ::core::ffi::c_void , nnumberofbytestoread : u32 , pnbytesread : *mut u32 , pnminnumberofbytesneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    ReadEventLogA(heventlog.into_param().abi(), dwreadflags, dwrecordoffset, lpbuffer, nnumberofbytestoread, pnbytesread, pnminnumberofbytesneeded)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadEventLogW<P0>(heventlog: P0, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventLogHandle>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ReadEventLogW ( heventlog : EventLogHandle , dwreadflags : READ_EVENT_LOG_READ_FLAGS , dwrecordoffset : u32 , lpbuffer : *mut ::core::ffi::c_void , nnumberofbytestoread : u32 , pnbytesread : *mut u32 , pnminnumberofbytesneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    ReadEventLogW(heventlog.into_param().abi(), dwreadflags, dwrecordoffset, lpbuffer, nnumberofbytestoread, pnbytesread, pnminnumberofbytesneeded)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn RegisterEventSourceA<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> ::windows::core::Result<EventSourceHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegisterEventSourceA ( lpuncservername : :: windows::core::PCSTR , lpsourcename : :: windows::core::PCSTR ) -> EventSourceHandle );
    let result__ = RegisterEventSourceA(lpuncservername.into_param().abi(), lpsourcename.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn RegisterEventSourceW<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> ::windows::core::Result<EventSourceHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegisterEventSourceW ( lpuncservername : :: windows::core::PCWSTR , lpsourcename : :: windows::core::PCWSTR ) -> EventSourceHandle );
    let result__ = RegisterEventSourceW(lpuncservername.into_param().abi(), lpsourcename.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportEventA<P0, P1>(heventlog: P0, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: P1, dwdatasize: u32, lpstrings: ::core::option::Option<&[::windows::core::PCSTR]>, lprawdata: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventSourceHandle>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ReportEventA ( heventlog : EventSourceHandle , wtype : REPORT_EVENT_TYPE , wcategory : u16 , dweventid : u32 , lpusersid : super::super::Foundation:: PSID , wnumstrings : u16 , dwdatasize : u32 , lpstrings : *const :: windows::core::PCSTR , lprawdata : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    ReportEventA(heventlog.into_param().abi(), wtype, wcategory, dweventid, lpusersid.into_param().abi(), lpstrings.as_deref().map_or(0, |slice| slice.len() as _), dwdatasize, ::core::mem::transmute(lpstrings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(lprawdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportEventW<P0, P1>(heventlog: P0, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: P1, dwdatasize: u32, lpstrings: ::core::option::Option<&[::windows::core::PCWSTR]>, lprawdata: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<EventSourceHandle>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ReportEventW ( heventlog : EventSourceHandle , wtype : REPORT_EVENT_TYPE , wcategory : u16 , dweventid : u32 , lpusersid : super::super::Foundation:: PSID , wnumstrings : u16 , dwdatasize : u32 , lpstrings : *const :: windows::core::PCWSTR , lprawdata : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    ReportEventW(heventlog.into_param().abi(), wtype, wcategory, dweventid, lpusersid.into_param().abi(), lpstrings.as_deref().map_or(0, |slice| slice.len() as _), dwdatasize, ::core::mem::transmute(lpstrings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(lprawdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_ALL_ACCESS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_CLEAR_ACCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_READ_ACCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_VARIANT_TYPE_ARRAY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_VARIANT_TYPE_MASK: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_WRITE_ACCESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_CHANNEL_CLOCK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelClockTypeSystemTime: EVT_CHANNEL_CLOCK_TYPE = EVT_CHANNEL_CLOCK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelClockTypeQPC: EVT_CHANNEL_CLOCK_TYPE = EVT_CHANNEL_CLOCK_TYPE(1i32);
impl ::core::marker::Copy for EVT_CHANNEL_CLOCK_TYPE {}
impl ::core::clone::Clone for EVT_CHANNEL_CLOCK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_CLOCK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_CHANNEL_CLOCK_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_CHANNEL_CLOCK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_CLOCK_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_CHANNEL_CONFIG_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigEnabled: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigIsolation: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigOwningPublisher: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigClassicEventlog: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigAccess: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelLoggingConfigRetention: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelLoggingConfigAutoBackup: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelLoggingConfigMaxSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelLoggingConfigLogFilePath: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigLevel: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigKeywords: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigControlGuid: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigBufferSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigMinBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigMaxBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigLatency: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigClockType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigSidType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(18i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublisherList: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(19i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigFileMax: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(20i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigPropertyIdEND: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(21i32);
impl ::core::marker::Copy for EVT_CHANNEL_CONFIG_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_CONFIG_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_CHANNEL_ISOLATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelIsolationTypeApplication: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelIsolationTypeSystem: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelIsolationTypeCustom: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(2i32);
impl ::core::marker::Copy for EVT_CHANNEL_ISOLATION_TYPE {}
impl ::core::clone::Clone for EVT_CHANNEL_ISOLATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_ISOLATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_CHANNEL_ISOLATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_CHANNEL_ISOLATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_ISOLATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_CHANNEL_REFERENCE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelReferenceImported: EVT_CHANNEL_REFERENCE_FLAGS = EVT_CHANNEL_REFERENCE_FLAGS(1u32);
impl ::core::marker::Copy for EVT_CHANNEL_REFERENCE_FLAGS {}
impl ::core::clone::Clone for EVT_CHANNEL_REFERENCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_REFERENCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_CHANNEL_REFERENCE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_CHANNEL_REFERENCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_REFERENCE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_CHANNEL_SID_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelSidTypeNone: EVT_CHANNEL_SID_TYPE = EVT_CHANNEL_SID_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelSidTypePublishing: EVT_CHANNEL_SID_TYPE = EVT_CHANNEL_SID_TYPE(1i32);
impl ::core::marker::Copy for EVT_CHANNEL_SID_TYPE {}
impl ::core::clone::Clone for EVT_CHANNEL_SID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_SID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_CHANNEL_SID_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_CHANNEL_SID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_SID_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_CHANNEL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelTypeAdmin: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelTypeOperational: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelTypeAnalytic: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelTypeDebug: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(3i32);
impl ::core::marker::Copy for EVT_CHANNEL_TYPE {}
impl ::core::clone::Clone for EVT_CHANNEL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_CHANNEL_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_CHANNEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_EVENT_METADATA_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventID: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventVersion: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventChannel: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventLevel: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventOpcode: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventTask: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventKeyword: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventMessageID: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventTemplate: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtEventMetadataPropertyIdEND: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(9i32);
impl ::core::marker::Copy for EVT_EVENT_METADATA_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_EVENT_METADATA_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_EVENT_METADATA_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_EVENT_METADATA_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_EVENT_METADATA_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_EVENT_METADATA_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_EVENT_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtEventQueryIDs: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtEventPath: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtEventPropertyIdEND: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(2i32);
impl ::core::marker::Copy for EVT_EVENT_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_EVENT_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_EVENT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_EVENT_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_EVENT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_EVENT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_EXPORTLOG_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtExportLogChannelPath: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtExportLogFilePath: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtExportLogTolerateQueryErrors: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtExportLogOverwrite: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(8192u32);
impl ::core::marker::Copy for EVT_EXPORTLOG_FLAGS {}
impl ::core::clone::Clone for EVT_EXPORTLOG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_EXPORTLOG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_EXPORTLOG_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_EXPORTLOG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_EXPORTLOG_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_FORMAT_MESSAGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageEvent: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageLevel: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageTask: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageOpcode: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageKeyword: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(5u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageChannel: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(6u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageProvider: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(7u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageId: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageXml: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(9u32);
impl ::core::marker::Copy for EVT_FORMAT_MESSAGE_FLAGS {}
impl ::core::clone::Clone for EVT_FORMAT_MESSAGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_FORMAT_MESSAGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_FORMAT_MESSAGE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_FORMAT_MESSAGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_FORMAT_MESSAGE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_LOGIN_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRpcLogin: EVT_LOGIN_CLASS = EVT_LOGIN_CLASS(1i32);
impl ::core::marker::Copy for EVT_LOGIN_CLASS {}
impl ::core::clone::Clone for EVT_LOGIN_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_LOGIN_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_LOGIN_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_LOGIN_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_LOGIN_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_LOG_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogCreationTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogLastAccessTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogLastWriteTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogFileSize: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogAttributes: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogNumberOfLogRecords: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogOldestRecordNumber: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogFull: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(7i32);
impl ::core::marker::Copy for EVT_LOG_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_LOG_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_LOG_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_LOG_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_LOG_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_LOG_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_OPEN_LOG_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtOpenChannelPath: EVT_OPEN_LOG_FLAGS = EVT_OPEN_LOG_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtOpenFilePath: EVT_OPEN_LOG_FLAGS = EVT_OPEN_LOG_FLAGS(2u32);
impl ::core::marker::Copy for EVT_OPEN_LOG_FLAGS {}
impl ::core::clone::Clone for EVT_OPEN_LOG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_OPEN_LOG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_OPEN_LOG_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_OPEN_LOG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_OPEN_LOG_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_PUBLISHER_METADATA_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataPublisherGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataResourceFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataParameterFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataMessageFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataHelpLink: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataPublisherMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferences: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferencePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferenceIndex: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferenceID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferenceFlags: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferenceMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataLevels: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataLevelName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataLevelValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataLevelMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataTasks: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataTaskName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataTaskEventGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(18i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataTaskValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(19i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataTaskMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(20i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataOpcodes: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(21i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataOpcodeName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(22i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataOpcodeValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(23i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataOpcodeMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(24i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataKeywords: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(25i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataKeywordName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(26i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataKeywordValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(27i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataKeywordMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(28i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataPropertyIdEND: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(29i32);
impl ::core::marker::Copy for EVT_PUBLISHER_METADATA_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_PUBLISHER_METADATA_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_QUERY_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryChannelPath: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryFilePath: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryForwardDirection: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryReverseDirection: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryTolerateQueryErrors: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(4096u32);
impl ::core::marker::Copy for EVT_QUERY_FLAGS {}
impl ::core::clone::Clone for EVT_QUERY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_QUERY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_QUERY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_QUERY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_QUERY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_QUERY_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryNames: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryStatuses: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryPropertyIdEND: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(2i32);
impl ::core::marker::Copy for EVT_QUERY_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_QUERY_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_QUERY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_QUERY_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_QUERY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_QUERY_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_RENDER_CONTEXT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderContextValues: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderContextSystem: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderContextUser: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(2u32);
impl ::core::marker::Copy for EVT_RENDER_CONTEXT_FLAGS {}
impl ::core::clone::Clone for EVT_RENDER_CONTEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_RENDER_CONTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_RENDER_CONTEXT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_RENDER_CONTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_RENDER_CONTEXT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_RENDER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderEventValues: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderEventXml: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderBookmark: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(2u32);
impl ::core::marker::Copy for EVT_RENDER_FLAGS {}
impl ::core::clone::Clone for EVT_RENDER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_RENDER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_RENDER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_RENDER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_RENDER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_RPC_LOGIN_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRpcLoginAuthDefault: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRpcLoginAuthNegotiate: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRpcLoginAuthKerberos: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRpcLoginAuthNTLM: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(3u32);
impl ::core::marker::Copy for EVT_RPC_LOGIN_FLAGS {}
impl ::core::clone::Clone for EVT_RPC_LOGIN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_RPC_LOGIN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_RPC_LOGIN_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_RPC_LOGIN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_RPC_LOGIN_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_SEEK_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekRelativeToFirst: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekRelativeToLast: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekRelativeToCurrent: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekRelativeToBookmark: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekOriginMask: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(7u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekStrict: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(65536u32);
impl ::core::marker::Copy for EVT_SEEK_FLAGS {}
impl ::core::clone::Clone for EVT_SEEK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_SEEK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_SEEK_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_SEEK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SEEK_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_SUBSCRIBE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeToFutureEvents: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeStartAtOldestRecord: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeStartAfterBookmark: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeOriginMask: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeTolerateQueryErrors: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeStrict: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(65536u32);
impl ::core::marker::Copy for EVT_SUBSCRIBE_FLAGS {}
impl ::core::clone::Clone for EVT_SUBSCRIBE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_SUBSCRIBE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_SUBSCRIBE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_SUBSCRIBE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SUBSCRIBE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_SUBSCRIBE_NOTIFY_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeActionError: EVT_SUBSCRIBE_NOTIFY_ACTION = EVT_SUBSCRIBE_NOTIFY_ACTION(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeActionDeliver: EVT_SUBSCRIBE_NOTIFY_ACTION = EVT_SUBSCRIBE_NOTIFY_ACTION(1i32);
impl ::core::marker::Copy for EVT_SUBSCRIBE_NOTIFY_ACTION {}
impl ::core::clone::Clone for EVT_SUBSCRIBE_NOTIFY_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_SUBSCRIBE_NOTIFY_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_SUBSCRIBE_NOTIFY_ACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_SUBSCRIBE_NOTIFY_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SUBSCRIBE_NOTIFY_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_SYSTEM_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemProviderName: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemProviderGuid: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemEventID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemQualifiers: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemLevel: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemTask: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemOpcode: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemKeywords: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemTimeCreated: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemEventRecordId: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemActivityID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemRelatedActivityID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemProcessID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemThreadID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemChannel: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemComputer: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemUserID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemVersion: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemPropertyIdEND: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(18i32);
impl ::core::marker::Copy for EVT_SYSTEM_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_SYSTEM_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_SYSTEM_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_SYSTEM_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_SYSTEM_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SYSTEM_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_VARIANT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeNull: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeString: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeAnsiString: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeSByte: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeByte: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeInt16: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeUInt16: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeUInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeUInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeSingle: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeDouble: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeBoolean: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeBinary: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeGuid: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeSizeT: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeFileTime: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeSysTime: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeSid: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(19i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeHexInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(20i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeHexInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(21i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeEvtHandle: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeEvtXml: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(35i32);
impl ::core::marker::Copy for EVT_VARIANT_TYPE {}
impl ::core::clone::Clone for EVT_VARIANT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_VARIANT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVT_VARIANT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVT_VARIANT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_VARIANT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct READ_EVENT_LOG_READ_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_SEEK_READ: READ_EVENT_LOG_READ_FLAGS = READ_EVENT_LOG_READ_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_SEQUENTIAL_READ: READ_EVENT_LOG_READ_FLAGS = READ_EVENT_LOG_READ_FLAGS(1u32);
impl ::core::marker::Copy for READ_EVENT_LOG_READ_FLAGS {}
impl ::core::clone::Clone for READ_EVENT_LOG_READ_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for READ_EVENT_LOG_READ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for READ_EVENT_LOG_READ_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for READ_EVENT_LOG_READ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("READ_EVENT_LOG_READ_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REPORT_EVENT_TYPE(pub u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_SUCCESS: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(0u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_AUDIT_FAILURE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(16u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_AUDIT_SUCCESS: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(8u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_ERROR_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(1u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_INFORMATION_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(4u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_WARNING_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(2u16);
impl ::core::marker::Copy for REPORT_EVENT_TYPE {}
impl ::core::clone::Clone for REPORT_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPORT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REPORT_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REPORT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPORT_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub struct EVENTLOGRECORD {
    pub Length: u32,
    pub Reserved: u32,
    pub RecordNumber: u32,
    pub TimeGenerated: u32,
    pub TimeWritten: u32,
    pub EventID: u32,
    pub EventType: REPORT_EVENT_TYPE,
    pub NumStrings: u16,
    pub EventCategory: u16,
    pub ReservedFlags: u16,
    pub ClosingRecordNumber: u32,
    pub StringOffset: u32,
    pub UserSidLength: u32,
    pub UserSidOffset: u32,
    pub DataLength: u32,
    pub DataOffset: u32,
}
impl ::core::marker::Copy for EVENTLOGRECORD {}
impl ::core::clone::Clone for EVENTLOGRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENTLOGRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENTLOGRECORD")
            .field("Length", &self.Length)
            .field("Reserved", &self.Reserved)
            .field("RecordNumber", &self.RecordNumber)
            .field("TimeGenerated", &self.TimeGenerated)
            .field("TimeWritten", &self.TimeWritten)
            .field("EventID", &self.EventID)
            .field("EventType", &self.EventType)
            .field("NumStrings", &self.NumStrings)
            .field("EventCategory", &self.EventCategory)
            .field("ReservedFlags", &self.ReservedFlags)
            .field("ClosingRecordNumber", &self.ClosingRecordNumber)
            .field("StringOffset", &self.StringOffset)
            .field("UserSidLength", &self.UserSidLength)
            .field("UserSidOffset", &self.UserSidOffset)
            .field("DataLength", &self.DataLength)
            .field("DataOffset", &self.DataOffset)
            .finish()
    }
}
impl ::windows::core::TypeKind for EVENTLOGRECORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EVENTLOGRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Reserved == other.Reserved && self.RecordNumber == other.RecordNumber && self.TimeGenerated == other.TimeGenerated && self.TimeWritten == other.TimeWritten && self.EventID == other.EventID && self.EventType == other.EventType && self.NumStrings == other.NumStrings && self.EventCategory == other.EventCategory && self.ReservedFlags == other.ReservedFlags && self.ClosingRecordNumber == other.ClosingRecordNumber && self.StringOffset == other.StringOffset && self.UserSidLength == other.UserSidLength && self.UserSidOffset == other.UserSidOffset && self.DataLength == other.DataLength && self.DataOffset == other.DataOffset
    }
}
impl ::core::cmp::Eq for EVENTLOGRECORD {}
impl ::core::default::Default for EVENTLOGRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub struct EVENTLOG_FULL_INFORMATION {
    pub dwFull: u32,
}
impl ::core::marker::Copy for EVENTLOG_FULL_INFORMATION {}
impl ::core::clone::Clone for EVENTLOG_FULL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENTLOG_FULL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENTLOG_FULL_INFORMATION").field("dwFull", &self.dwFull).finish()
    }
}
impl ::windows::core::TypeKind for EVENTLOG_FULL_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EVENTLOG_FULL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwFull == other.dwFull
    }
}
impl ::core::cmp::Eq for EVENTLOG_FULL_INFORMATION {}
impl ::core::default::Default for EVENTLOG_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub struct EVENTSFORLOGFILE {
    pub ulSize: u32,
    pub szLogicalLogFile: [u16; 256],
    pub ulNumRecords: u32,
    pub pEventLogRecords: [EVENTLOGRECORD; 1],
}
impl ::core::marker::Copy for EVENTSFORLOGFILE {}
impl ::core::clone::Clone for EVENTSFORLOGFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENTSFORLOGFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENTSFORLOGFILE").field("ulSize", &self.ulSize).field("szLogicalLogFile", &self.szLogicalLogFile).field("ulNumRecords", &self.ulNumRecords).field("pEventLogRecords", &self.pEventLogRecords).finish()
    }
}
impl ::windows::core::TypeKind for EVENTSFORLOGFILE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EVENTSFORLOGFILE {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize && self.szLogicalLogFile == other.szLogicalLogFile && self.ulNumRecords == other.ulNumRecords && self.pEventLogRecords == other.pEventLogRecords
    }
}
impl ::core::cmp::Eq for EVENTSFORLOGFILE {}
impl ::core::default::Default for EVENTSFORLOGFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVT_HANDLE(pub isize);
impl EVT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for EVT_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for EVT_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for EVT_HANDLE {}
impl ::core::fmt::Debug for EVT_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for EVT_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub struct EVT_RPC_LOGIN {
    pub Server: ::windows::core::PWSTR,
    pub User: ::windows::core::PWSTR,
    pub Domain: ::windows::core::PWSTR,
    pub Password: ::windows::core::PWSTR,
    pub Flags: u32,
}
impl ::core::marker::Copy for EVT_RPC_LOGIN {}
impl ::core::clone::Clone for EVT_RPC_LOGIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVT_RPC_LOGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVT_RPC_LOGIN").field("Server", &self.Server).field("User", &self.User).field("Domain", &self.Domain).field("Password", &self.Password).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for EVT_RPC_LOGIN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EVT_RPC_LOGIN {
    fn eq(&self, other: &Self) -> bool {
        self.Server == other.Server && self.User == other.User && self.Domain == other.Domain && self.Password == other.Password && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for EVT_RPC_LOGIN {}
impl ::core::default::Default for EVT_RPC_LOGIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVT_VARIANT {
    pub Anonymous: EVT_VARIANT_0,
    pub Count: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVT_VARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVT_VARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for EVT_VARIANT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVT_VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union EVT_VARIANT_0 {
    pub BooleanVal: super::super::Foundation::BOOL,
    pub SByteVal: i8,
    pub Int16Val: i16,
    pub Int32Val: i32,
    pub Int64Val: i64,
    pub ByteVal: u8,
    pub UInt16Val: u16,
    pub UInt32Val: u32,
    pub UInt64Val: u64,
    pub SingleVal: f32,
    pub DoubleVal: f64,
    pub FileTimeVal: u64,
    pub SysTimeVal: *mut super::super::Foundation::SYSTEMTIME,
    pub GuidVal: *mut ::windows::core::GUID,
    pub StringVal: ::windows::core::PCWSTR,
    pub AnsiStringVal: ::windows::core::PCSTR,
    pub BinaryVal: *mut u8,
    pub SidVal: super::super::Foundation::PSID,
    pub SizeTVal: usize,
    pub BooleanArr: *mut super::super::Foundation::BOOL,
    pub SByteArr: *mut i8,
    pub Int16Arr: *mut i16,
    pub Int32Arr: *mut i32,
    pub Int64Arr: *mut i64,
    pub ByteArr: *mut u8,
    pub UInt16Arr: *mut u16,
    pub UInt32Arr: *mut u32,
    pub UInt64Arr: *mut u64,
    pub SingleArr: *mut f32,
    pub DoubleArr: *mut f64,
    pub FileTimeArr: *mut super::super::Foundation::FILETIME,
    pub SysTimeArr: *mut super::super::Foundation::SYSTEMTIME,
    pub GuidArr: *mut ::windows::core::GUID,
    pub StringArr: *mut ::windows::core::PWSTR,
    pub AnsiStringArr: *mut ::windows::core::PSTR,
    pub SidArr: *mut super::super::Foundation::PSID,
    pub SizeTArr: *mut usize,
    pub EvtHandleVal: EVT_HANDLE,
    pub XmlVal: ::windows::core::PCWSTR,
    pub XmlValArr: *const ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVT_VARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVT_VARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for EVT_VARIANT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVT_VARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EventLogHandle(pub isize);
impl EventLogHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for EventLogHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for EventLogHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for EventLogHandle {}
impl ::core::fmt::Debug for EventLogHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EventLogHandle").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for EventLogHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EventSourceHandle(pub isize);
impl EventSourceHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for EventSourceHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for EventSourceHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for EventSourceHandle {}
impl ::core::fmt::Debug for EventSourceHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EventSourceHandle").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for EventSourceHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub type EVT_SUBSCRIBE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(action: EVT_SUBSCRIBE_NOTIFY_ACTION, usercontext: *const ::core::ffi::c_void, event: EVT_HANDLE) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
