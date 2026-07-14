#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtArchiveExportedLog<P1>(session: Option<EVT_HANDLE>, logfilepath: P1, locale: super::winnt::LCID, flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wevtapi.dll" "system" fn EvtArchiveExportedLog(session : EVT_HANDLE, logfilepath : windows_core::PCWSTR, locale : super::winnt::LCID, flags : u32) -> windows_core::BOOL);
    unsafe { EvtArchiveExportedLog(session.unwrap_or(core::mem::zeroed()) as _, logfilepath.param().abi(), locale, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtCancel(object: Option<EVT_HANDLE>) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtCancel(object : EVT_HANDLE) -> windows_core::BOOL);
    unsafe { EvtCancel(object.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtClearLog<P1, P2>(session: Option<EVT_HANDLE>, channelpath: P1, targetfilepath: P2, flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wevtapi.dll" "system" fn EvtClearLog(session : EVT_HANDLE, channelpath : windows_core::PCWSTR, targetfilepath : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { EvtClearLog(session.unwrap_or(core::mem::zeroed()) as _, channelpath.param().abi(), targetfilepath.param().abi(), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtClose(object: EVT_HANDLE) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtClose(object : EVT_HANDLE) -> windows_core::BOOL);
    unsafe { EvtClose(object) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtCreateBookmark<P0>(bookmarkxml: P0) -> EVT_HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wevtapi.dll" "system" fn EvtCreateBookmark(bookmarkxml : windows_core::PCWSTR) -> EVT_HANDLE);
    unsafe { EvtCreateBookmark(bookmarkxml.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtCreateRenderContext(valuepaths: Option<&[windows_core::PCWSTR]>, flags: u32) -> EVT_HANDLE {
    windows_core::link!("wevtapi.dll" "system" fn EvtCreateRenderContext(valuepathscount : u32, valuepaths : *const windows_core::PCWSTR, flags : u32) -> EVT_HANDLE);
    unsafe { EvtCreateRenderContext(valuepaths.map_or(0, |slice| slice.len().try_into().unwrap()), valuepaths.map_or(core::ptr::null(), |slice| slice.as_ptr()), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtExportLog<P1, P2, P3>(session: Option<EVT_HANDLE>, path: P1, query: P2, targetfilepath: P3, flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wevtapi.dll" "system" fn EvtExportLog(session : EVT_HANDLE, path : windows_core::PCWSTR, query : windows_core::PCWSTR, targetfilepath : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { EvtExportLog(session.unwrap_or(core::mem::zeroed()) as _, path.param().abi(), query.param().abi(), targetfilepath.param().abi(), flags) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EvtFormatMessage(publishermetadata: Option<EVT_HANDLE>, event: Option<EVT_HANDLE>, messageid: u32, values: Option<&[EVT_VARIANT]>, flags: u32, buffer: Option<&mut [u16]>, bufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtFormatMessage(publishermetadata : EVT_HANDLE, event : EVT_HANDLE, messageid : u32, valuecount : u32, values : *const EVT_VARIANT, flags : u32, buffersize : u32, buffer : windows_core::PWSTR, bufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtFormatMessage(publishermetadata.unwrap_or(core::mem::zeroed()) as _, event.unwrap_or(core::mem::zeroed()) as _, messageid, values.map_or(0, |slice| slice.len().try_into().unwrap()), values.map_or(core::ptr::null(), |slice| slice.as_ptr()), flags, buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(buffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), bufferused as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EvtGetChannelConfigProperty(channelconfig: EVT_HANDLE, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtGetChannelConfigProperty(channelconfig : EVT_HANDLE, propertyid : EVT_CHANNEL_CONFIG_PROPERTY_ID, flags : u32, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EVT_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetChannelConfigProperty(channelconfig, propertyid, flags, propertyvaluebuffersize, propertyvaluebuffer.unwrap_or(core::mem::zeroed()) as _, propertyvaluebufferused as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EvtGetEventInfo(event: EVT_HANDLE, propertyid: EVT_EVENT_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtGetEventInfo(event : EVT_HANDLE, propertyid : EVT_EVENT_PROPERTY_ID, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EVT_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetEventInfo(event, propertyid, propertyvaluebuffersize, propertyvaluebuffer.unwrap_or(core::mem::zeroed()) as _, propertyvaluebufferused as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EvtGetEventMetadataProperty(eventmetadata: EVT_HANDLE, propertyid: EVT_EVENT_METADATA_PROPERTY_ID, flags: u32, eventmetadatapropertybuffersize: u32, eventmetadatapropertybuffer: Option<*mut EVT_VARIANT>, eventmetadatapropertybufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtGetEventMetadataProperty(eventmetadata : EVT_HANDLE, propertyid : EVT_EVENT_METADATA_PROPERTY_ID, flags : u32, eventmetadatapropertybuffersize : u32, eventmetadatapropertybuffer : *mut EVT_VARIANT, eventmetadatapropertybufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetEventMetadataProperty(eventmetadata, propertyid, flags, eventmetadatapropertybuffersize, eventmetadatapropertybuffer.unwrap_or(core::mem::zeroed()) as _, eventmetadatapropertybufferused as _) }
}
#[inline]
pub unsafe fn EvtGetExtendedStatus(buffer: Option<&mut [u16]>, bufferused: *mut u32) -> u32 {
    windows_core::link!("wevtapi.dll" "system" fn EvtGetExtendedStatus(buffersize : u32, buffer : windows_core::PWSTR, bufferused : *mut u32) -> u32);
    unsafe { EvtGetExtendedStatus(buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(buffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), bufferused as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EvtGetLogInfo(log: EVT_HANDLE, propertyid: EVT_LOG_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtGetLogInfo(log : EVT_HANDLE, propertyid : EVT_LOG_PROPERTY_ID, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EVT_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetLogInfo(log, propertyid, propertyvaluebuffersize, propertyvaluebuffer.unwrap_or(core::mem::zeroed()) as _, propertyvaluebufferused as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EvtGetObjectArrayProperty(objectarray: EVT_OBJECT_ARRAY_PROPERTY_HANDLE, propertyid: u32, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtGetObjectArrayProperty(objectarray : EVT_OBJECT_ARRAY_PROPERTY_HANDLE, propertyid : u32, arrayindex : u32, flags : u32, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EVT_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetObjectArrayProperty(objectarray, propertyid, arrayindex, flags, propertyvaluebuffersize, propertyvaluebuffer.unwrap_or(core::mem::zeroed()) as _, propertyvaluebufferused as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtGetObjectArraySize(objectarray: EVT_OBJECT_ARRAY_PROPERTY_HANDLE, objectarraysize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtGetObjectArraySize(objectarray : EVT_OBJECT_ARRAY_PROPERTY_HANDLE, objectarraysize : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetObjectArraySize(objectarray, objectarraysize as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EvtGetPublisherMetadataProperty(publishermetadata: EVT_HANDLE, propertyid: EVT_PUBLISHER_METADATA_PROPERTY_ID, flags: u32, publishermetadatapropertybuffersize: u32, publishermetadatapropertybuffer: Option<*mut EVT_VARIANT>, publishermetadatapropertybufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtGetPublisherMetadataProperty(publishermetadata : EVT_HANDLE, propertyid : EVT_PUBLISHER_METADATA_PROPERTY_ID, flags : u32, publishermetadatapropertybuffersize : u32, publishermetadatapropertybuffer : *mut EVT_VARIANT, publishermetadatapropertybufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetPublisherMetadataProperty(publishermetadata, propertyid, flags, publishermetadatapropertybuffersize, publishermetadatapropertybuffer.unwrap_or(core::mem::zeroed()) as _, publishermetadatapropertybufferused as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EvtGetQueryInfo(queryorsubscription: EVT_HANDLE, propertyid: EVT_QUERY_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtGetQueryInfo(queryorsubscription : EVT_HANDLE, propertyid : EVT_QUERY_PROPERTY_ID, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EVT_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetQueryInfo(queryorsubscription, propertyid, propertyvaluebuffersize, propertyvaluebuffer.unwrap_or(core::mem::zeroed()) as _, propertyvaluebufferused as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtNext(resultset: EVT_HANDLE, events: &mut [super::winnt::HANDLE], timeout: u32, flags: u32, returned: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtNext(resultset : EVT_HANDLE, eventssize : u32, events : *mut super::winnt::HANDLE, timeout : u32, flags : u32, returned : *mut u32) -> windows_core::BOOL);
    unsafe { EvtNext(resultset, events.len().try_into().unwrap(), events.as_mut_ptr(), timeout, flags, returned as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtNextChannelPath(channelenum: EVT_HANDLE, channelpathbuffer: Option<&mut [u16]>, channelpathbufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtNextChannelPath(channelenum : EVT_HANDLE, channelpathbuffersize : u32, channelpathbuffer : windows_core::PWSTR, channelpathbufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtNextChannelPath(channelenum, channelpathbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(channelpathbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), channelpathbufferused as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtNextEventMetadata(eventmetadataenum: EVT_HANDLE, flags: u32) -> EVT_HANDLE {
    windows_core::link!("wevtapi.dll" "system" fn EvtNextEventMetadata(eventmetadataenum : EVT_HANDLE, flags : u32) -> EVT_HANDLE);
    unsafe { EvtNextEventMetadata(eventmetadataenum, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtNextPublisherId(publisherenum: EVT_HANDLE, publisheridbuffer: Option<&mut [u16]>, publisheridbufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtNextPublisherId(publisherenum : EVT_HANDLE, publisheridbuffersize : u32, publisheridbuffer : windows_core::PWSTR, publisheridbufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtNextPublisherId(publisherenum, publisheridbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(publisheridbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), publisheridbufferused as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtOpenChannelConfig<P1>(session: Option<EVT_HANDLE>, channelpath: P1, flags: u32) -> EVT_HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wevtapi.dll" "system" fn EvtOpenChannelConfig(session : EVT_HANDLE, channelpath : windows_core::PCWSTR, flags : u32) -> EVT_HANDLE);
    unsafe { EvtOpenChannelConfig(session.unwrap_or(core::mem::zeroed()) as _, channelpath.param().abi(), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtOpenChannelEnum(session: Option<EVT_HANDLE>, flags: u32) -> EVT_HANDLE {
    windows_core::link!("wevtapi.dll" "system" fn EvtOpenChannelEnum(session : EVT_HANDLE, flags : u32) -> EVT_HANDLE);
    unsafe { EvtOpenChannelEnum(session.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtOpenEventMetadataEnum(publishermetadata: EVT_HANDLE, flags: u32) -> EVT_HANDLE {
    windows_core::link!("wevtapi.dll" "system" fn EvtOpenEventMetadataEnum(publishermetadata : EVT_HANDLE, flags : u32) -> EVT_HANDLE);
    unsafe { EvtOpenEventMetadataEnum(publishermetadata, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtOpenLog<P1>(session: Option<EVT_HANDLE>, path: P1, flags: u32) -> EVT_HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wevtapi.dll" "system" fn EvtOpenLog(session : EVT_HANDLE, path : windows_core::PCWSTR, flags : u32) -> EVT_HANDLE);
    unsafe { EvtOpenLog(session.unwrap_or(core::mem::zeroed()) as _, path.param().abi(), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtOpenPublisherEnum(session: Option<EVT_HANDLE>, flags: u32) -> EVT_HANDLE {
    windows_core::link!("wevtapi.dll" "system" fn EvtOpenPublisherEnum(session : EVT_HANDLE, flags : u32) -> EVT_HANDLE);
    unsafe { EvtOpenPublisherEnum(session.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtOpenPublisherMetadata<P1, P2>(session: Option<EVT_HANDLE>, publisherid: P1, logfilepath: P2, locale: super::winnt::LCID, flags: u32) -> EVT_HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wevtapi.dll" "system" fn EvtOpenPublisherMetadata(session : EVT_HANDLE, publisherid : windows_core::PCWSTR, logfilepath : windows_core::PCWSTR, locale : super::winnt::LCID, flags : u32) -> EVT_HANDLE);
    unsafe { EvtOpenPublisherMetadata(session.unwrap_or(core::mem::zeroed()) as _, publisherid.param().abi(), logfilepath.param().abi(), locale, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtOpenSession(loginclass: EVT_LOGIN_CLASS, login: *mut core::ffi::c_void, timeout: Option<u32>, flags: Option<u32>) -> EVT_HANDLE {
    windows_core::link!("wevtapi.dll" "system" fn EvtOpenSession(loginclass : EVT_LOGIN_CLASS, login : *mut core::ffi::c_void, timeout : u32, flags : u32) -> EVT_HANDLE);
    unsafe { EvtOpenSession(loginclass, login as _, timeout.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtQuery<P1, P2>(session: Option<EVT_HANDLE>, path: P1, query: P2, flags: u32) -> EVT_HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wevtapi.dll" "system" fn EvtQuery(session : EVT_HANDLE, path : windows_core::PCWSTR, query : windows_core::PCWSTR, flags : u32) -> EVT_HANDLE);
    unsafe { EvtQuery(session.unwrap_or(core::mem::zeroed()) as _, path.param().abi(), query.param().abi(), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtRender(context: Option<EVT_HANDLE>, fragment: EVT_HANDLE, flags: u32, buffersize: u32, buffer: Option<*mut core::ffi::c_void>, bufferused: *mut u32, propertycount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtRender(context : EVT_HANDLE, fragment : EVT_HANDLE, flags : u32, buffersize : u32, buffer : *mut core::ffi::c_void, bufferused : *mut u32, propertycount : *mut u32) -> windows_core::BOOL);
    unsafe { EvtRender(context.unwrap_or(core::mem::zeroed()) as _, fragment, flags, buffersize, buffer.unwrap_or(core::mem::zeroed()) as _, bufferused as _, propertycount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtSaveChannelConfig(channelconfig: EVT_HANDLE, flags: u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtSaveChannelConfig(channelconfig : EVT_HANDLE, flags : u32) -> windows_core::BOOL);
    unsafe { EvtSaveChannelConfig(channelconfig, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtSeek(resultset: EVT_HANDLE, position: i64, bookmark: Option<EVT_HANDLE>, timeout: Option<u32>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtSeek(resultset : EVT_HANDLE, position : i64, bookmark : EVT_HANDLE, timeout : u32, flags : u32) -> windows_core::BOOL);
    unsafe { EvtSeek(resultset, position, bookmark.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EvtSetChannelConfigProperty(channelconfig: EVT_HANDLE, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvalue: *const EVT_VARIANT) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtSetChannelConfigProperty(channelconfig : EVT_HANDLE, propertyid : EVT_CHANNEL_CONFIG_PROPERTY_ID, flags : u32, propertyvalue : *const EVT_VARIANT) -> windows_core::BOOL);
    unsafe { EvtSetChannelConfigProperty(channelconfig, propertyid, flags, propertyvalue) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtSubscribe<P2, P3>(session: Option<EVT_HANDLE>, signalevent: Option<super::winnt::HANDLE>, channelpath: P2, query: P3, bookmark: Option<EVT_HANDLE>, context: *mut core::ffi::c_void, callback: EVT_SUBSCRIBE_CALLBACK, flags: u32) -> EVT_HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wevtapi.dll" "system" fn EvtSubscribe(session : EVT_HANDLE, signalevent : super::winnt::HANDLE, channelpath : windows_core::PCWSTR, query : windows_core::PCWSTR, bookmark : EVT_HANDLE, context : *mut core::ffi::c_void, callback : EVT_SUBSCRIBE_CALLBACK, flags : u32) -> EVT_HANDLE);
    unsafe { EvtSubscribe(session.unwrap_or(core::mem::zeroed()) as _, signalevent.unwrap_or(core::mem::zeroed()) as _, channelpath.param().abi(), query.param().abi(), bookmark.unwrap_or(core::mem::zeroed()) as _, context as _, callback, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EvtUpdateBookmark(bookmark: EVT_HANDLE, event: EVT_HANDLE) -> windows_core::BOOL {
    windows_core::link!("wevtapi.dll" "system" fn EvtUpdateBookmark(bookmark : EVT_HANDLE, event : EVT_HANDLE) -> windows_core::BOOL);
    unsafe { EvtUpdateBookmark(bookmark, event) }
}
pub const EVT_ALL_ACCESS: u32 = 7;
pub type EVT_CHANNEL_CLOCK_TYPE = i32;
pub type EVT_CHANNEL_CONFIG_PROPERTY_ID = i32;
pub type EVT_CHANNEL_ISOLATION_TYPE = i32;
pub type EVT_CHANNEL_REFERENCE_FLAGS = i32;
pub type EVT_CHANNEL_SID_TYPE = i32;
pub type EVT_CHANNEL_TYPE = i32;
pub const EVT_CLEAR_ACCESS: u32 = 4;
pub type EVT_EVENT_METADATA_PROPERTY_ID = i32;
pub type EVT_EVENT_PROPERTY_ID = i32;
pub type EVT_EXPORTLOG_FLAGS = i32;
pub type EVT_FORMAT_MESSAGE_FLAGS = i32;
#[cfg(feature = "winnt")]
pub type EVT_HANDLE = super::winnt::HANDLE;
pub type EVT_LOGIN_CLASS = i32;
pub type EVT_LOG_PROPERTY_ID = i32;
#[cfg(feature = "winnt")]
pub type EVT_OBJECT_ARRAY_PROPERTY_HANDLE = super::winnt::HANDLE;
pub type EVT_OPEN_LOG_FLAGS = i32;
pub type EVT_PUBLISHER_METADATA_PROPERTY_ID = i32;
pub type EVT_QUERY_FLAGS = i32;
pub type EVT_QUERY_PROPERTY_ID = i32;
pub const EVT_READ_ACCESS: u32 = 1;
pub type EVT_RENDER_CONTEXT_FLAGS = i32;
pub type EVT_RENDER_FLAGS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVT_RPC_LOGIN {
    pub Server: windows_core::PWSTR,
    pub User: windows_core::PWSTR,
    pub Domain: windows_core::PWSTR,
    pub Password: windows_core::PWSTR,
    pub Flags: u32,
}
pub type EVT_RPC_LOGIN_FLAGS = i32;
pub type EVT_SEEK_FLAGS = i32;
#[cfg(feature = "winnt")]
pub type EVT_SUBSCRIBE_CALLBACK = Option<unsafe extern "system" fn(action: EVT_SUBSCRIBE_NOTIFY_ACTION, usercontext: *mut core::ffi::c_void, event: EVT_HANDLE) -> u32>;
pub type EVT_SUBSCRIBE_FLAGS = i32;
pub type EVT_SUBSCRIBE_NOTIFY_ACTION = i32;
pub type EVT_SYSTEM_PROPERTY_ID = i32;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct EVT_VARIANT {
    pub Anonymous: EVT_VARIANT_0,
    pub Count: u32,
    pub Type: u32,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
impl Default for EVT_VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union EVT_VARIANT_0 {
    pub BooleanVal: windows_core::BOOL,
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
    pub SysTimeVal: *mut super::minwinbase::SYSTEMTIME,
    pub GuidVal: *mut windows_core::GUID,
    pub StringVal: windows_core::PCWSTR,
    pub AnsiStringVal: windows_core::PCSTR,
    pub BinaryVal: super::minwindef::PBYTE,
    pub SidVal: super::winnt::PSID,
    pub SizeTVal: usize,
    pub BooleanArr: *mut windows_core::BOOL,
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
    pub FileTimeArr: *mut super::minwindef::FILETIME,
    pub SysTimeArr: *mut super::minwinbase::SYSTEMTIME,
    pub GuidArr: *mut windows_core::GUID,
    pub StringArr: *mut windows_core::PWSTR,
    pub AnsiStringArr: *mut windows_core::PSTR,
    pub SidArr: *mut super::winnt::PSID,
    pub SizeTArr: *mut usize,
    pub EvtHandleVal: EVT_HANDLE,
    pub XmlVal: windows_core::PCWSTR,
    pub XmlValArr: *mut windows_core::PCWSTR,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
impl Default for EVT_VARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EVT_VARIANT_TYPE = i32;
pub const EVT_VARIANT_TYPE_ARRAY: u32 = 128;
pub const EVT_VARIANT_TYPE_MASK: u32 = 127;
pub const EVT_WRITE_ACCESS: u32 = 2;
pub const EventMetadataEventChannel: EVT_EVENT_METADATA_PROPERTY_ID = 2;
pub const EventMetadataEventID: EVT_EVENT_METADATA_PROPERTY_ID = 0;
pub const EventMetadataEventKeyword: EVT_EVENT_METADATA_PROPERTY_ID = 6;
pub const EventMetadataEventLevel: EVT_EVENT_METADATA_PROPERTY_ID = 3;
pub const EventMetadataEventMessageID: EVT_EVENT_METADATA_PROPERTY_ID = 7;
pub const EventMetadataEventOpcode: EVT_EVENT_METADATA_PROPERTY_ID = 4;
pub const EventMetadataEventTask: EVT_EVENT_METADATA_PROPERTY_ID = 5;
pub const EventMetadataEventTemplate: EVT_EVENT_METADATA_PROPERTY_ID = 8;
pub const EventMetadataEventVersion: EVT_EVENT_METADATA_PROPERTY_ID = 1;
pub const EvtChannelClockTypeQPC: EVT_CHANNEL_CLOCK_TYPE = 1;
pub const EvtChannelClockTypeSystemTime: EVT_CHANNEL_CLOCK_TYPE = 0;
pub const EvtChannelConfigAccess: EVT_CHANNEL_CONFIG_PROPERTY_ID = 5;
pub const EvtChannelConfigClassicEventlog: EVT_CHANNEL_CONFIG_PROPERTY_ID = 4;
pub const EvtChannelConfigEnabled: EVT_CHANNEL_CONFIG_PROPERTY_ID = 0;
pub const EvtChannelConfigIsolation: EVT_CHANNEL_CONFIG_PROPERTY_ID = 1;
pub const EvtChannelConfigOwningPublisher: EVT_CHANNEL_CONFIG_PROPERTY_ID = 3;
pub const EvtChannelConfigPropertyIdEND: EVT_CHANNEL_CONFIG_PROPERTY_ID = 21;
pub const EvtChannelConfigType: EVT_CHANNEL_CONFIG_PROPERTY_ID = 2;
pub const EvtChannelIsolationTypeApplication: EVT_CHANNEL_ISOLATION_TYPE = 0;
pub const EvtChannelIsolationTypeCustom: EVT_CHANNEL_ISOLATION_TYPE = 2;
pub const EvtChannelIsolationTypeSystem: EVT_CHANNEL_ISOLATION_TYPE = 1;
pub const EvtChannelLoggingConfigAutoBackup: EVT_CHANNEL_CONFIG_PROPERTY_ID = 7;
pub const EvtChannelLoggingConfigLogFilePath: EVT_CHANNEL_CONFIG_PROPERTY_ID = 9;
pub const EvtChannelLoggingConfigMaxSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = 8;
pub const EvtChannelLoggingConfigRetention: EVT_CHANNEL_CONFIG_PROPERTY_ID = 6;
pub const EvtChannelPublisherList: EVT_CHANNEL_CONFIG_PROPERTY_ID = 19;
pub const EvtChannelPublishingConfigBufferSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = 13;
pub const EvtChannelPublishingConfigClockType: EVT_CHANNEL_CONFIG_PROPERTY_ID = 17;
pub const EvtChannelPublishingConfigControlGuid: EVT_CHANNEL_CONFIG_PROPERTY_ID = 12;
pub const EvtChannelPublishingConfigFileMax: EVT_CHANNEL_CONFIG_PROPERTY_ID = 20;
pub const EvtChannelPublishingConfigKeywords: EVT_CHANNEL_CONFIG_PROPERTY_ID = 11;
pub const EvtChannelPublishingConfigLatency: EVT_CHANNEL_CONFIG_PROPERTY_ID = 16;
pub const EvtChannelPublishingConfigLevel: EVT_CHANNEL_CONFIG_PROPERTY_ID = 10;
pub const EvtChannelPublishingConfigMaxBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = 15;
pub const EvtChannelPublishingConfigMinBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = 14;
pub const EvtChannelPublishingConfigSidType: EVT_CHANNEL_CONFIG_PROPERTY_ID = 18;
pub const EvtChannelReferenceImported: EVT_CHANNEL_REFERENCE_FLAGS = 1;
pub const EvtChannelSidTypeNone: EVT_CHANNEL_SID_TYPE = 0;
pub const EvtChannelSidTypePublishing: EVT_CHANNEL_SID_TYPE = 1;
pub const EvtChannelTypeAdmin: EVT_CHANNEL_TYPE = 0;
pub const EvtChannelTypeAnalytic: EVT_CHANNEL_TYPE = 2;
pub const EvtChannelTypeDebug: EVT_CHANNEL_TYPE = 3;
pub const EvtChannelTypeOperational: EVT_CHANNEL_TYPE = 1;
pub const EvtEventMetadataPropertyIdEND: EVT_EVENT_METADATA_PROPERTY_ID = 9;
pub const EvtEventPath: EVT_EVENT_PROPERTY_ID = 1;
pub const EvtEventPropertyIdEND: EVT_EVENT_PROPERTY_ID = 2;
pub const EvtEventQueryIDs: EVT_EVENT_PROPERTY_ID = 0;
pub const EvtExportLogChannelPath: EVT_EXPORTLOG_FLAGS = 1;
pub const EvtExportLogFilePath: EVT_EXPORTLOG_FLAGS = 2;
pub const EvtExportLogOverwrite: EVT_EXPORTLOG_FLAGS = 8192;
pub const EvtExportLogTolerateQueryErrors: EVT_EXPORTLOG_FLAGS = 4096;
pub const EvtFormatMessageChannel: EVT_FORMAT_MESSAGE_FLAGS = 6;
pub const EvtFormatMessageEvent: EVT_FORMAT_MESSAGE_FLAGS = 1;
pub const EvtFormatMessageId: EVT_FORMAT_MESSAGE_FLAGS = 8;
pub const EvtFormatMessageKeyword: EVT_FORMAT_MESSAGE_FLAGS = 5;
pub const EvtFormatMessageLevel: EVT_FORMAT_MESSAGE_FLAGS = 2;
pub const EvtFormatMessageOpcode: EVT_FORMAT_MESSAGE_FLAGS = 4;
pub const EvtFormatMessageProvider: EVT_FORMAT_MESSAGE_FLAGS = 7;
pub const EvtFormatMessageTask: EVT_FORMAT_MESSAGE_FLAGS = 3;
pub const EvtFormatMessageXml: EVT_FORMAT_MESSAGE_FLAGS = 9;
pub const EvtLogAttributes: EVT_LOG_PROPERTY_ID = 4;
pub const EvtLogCreationTime: EVT_LOG_PROPERTY_ID = 0;
pub const EvtLogFileSize: EVT_LOG_PROPERTY_ID = 3;
pub const EvtLogFull: EVT_LOG_PROPERTY_ID = 7;
pub const EvtLogLastAccessTime: EVT_LOG_PROPERTY_ID = 1;
pub const EvtLogLastWriteTime: EVT_LOG_PROPERTY_ID = 2;
pub const EvtLogNumberOfLogRecords: EVT_LOG_PROPERTY_ID = 5;
pub const EvtLogOldestRecordNumber: EVT_LOG_PROPERTY_ID = 6;
pub const EvtOpenChannelPath: EVT_OPEN_LOG_FLAGS = 1;
pub const EvtOpenFilePath: EVT_OPEN_LOG_FLAGS = 2;
pub const EvtPublisherMetadataChannelReferenceFlags: EVT_PUBLISHER_METADATA_PROPERTY_ID = 10;
pub const EvtPublisherMetadataChannelReferenceID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 9;
pub const EvtPublisherMetadataChannelReferenceIndex: EVT_PUBLISHER_METADATA_PROPERTY_ID = 8;
pub const EvtPublisherMetadataChannelReferenceMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 11;
pub const EvtPublisherMetadataChannelReferencePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 7;
pub const EvtPublisherMetadataChannelReferences: EVT_PUBLISHER_METADATA_PROPERTY_ID = 6;
pub const EvtPublisherMetadataHelpLink: EVT_PUBLISHER_METADATA_PROPERTY_ID = 4;
pub const EvtPublisherMetadataKeywordMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 28;
pub const EvtPublisherMetadataKeywordName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 26;
pub const EvtPublisherMetadataKeywordValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 27;
pub const EvtPublisherMetadataKeywords: EVT_PUBLISHER_METADATA_PROPERTY_ID = 25;
pub const EvtPublisherMetadataLevelMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 15;
pub const EvtPublisherMetadataLevelName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 13;
pub const EvtPublisherMetadataLevelValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 14;
pub const EvtPublisherMetadataLevels: EVT_PUBLISHER_METADATA_PROPERTY_ID = 12;
pub const EvtPublisherMetadataMessageFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 3;
pub const EvtPublisherMetadataOpcodeMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 24;
pub const EvtPublisherMetadataOpcodeName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 22;
pub const EvtPublisherMetadataOpcodeValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 23;
pub const EvtPublisherMetadataOpcodes: EVT_PUBLISHER_METADATA_PROPERTY_ID = 21;
pub const EvtPublisherMetadataParameterFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 2;
pub const EvtPublisherMetadataPropertyIdEND: EVT_PUBLISHER_METADATA_PROPERTY_ID = 29;
pub const EvtPublisherMetadataPublisherGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = 0;
pub const EvtPublisherMetadataPublisherMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 5;
pub const EvtPublisherMetadataResourceFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 1;
pub const EvtPublisherMetadataTaskEventGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = 18;
pub const EvtPublisherMetadataTaskMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 20;
pub const EvtPublisherMetadataTaskName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 17;
pub const EvtPublisherMetadataTaskValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 19;
pub const EvtPublisherMetadataTasks: EVT_PUBLISHER_METADATA_PROPERTY_ID = 16;
pub const EvtQueryChannelPath: EVT_QUERY_FLAGS = 1;
pub const EvtQueryFilePath: EVT_QUERY_FLAGS = 2;
pub const EvtQueryForwardDirection: EVT_QUERY_FLAGS = 256;
pub const EvtQueryNames: EVT_QUERY_PROPERTY_ID = 0;
pub const EvtQueryPropertyIdEND: EVT_QUERY_PROPERTY_ID = 2;
pub const EvtQueryReverseDirection: EVT_QUERY_FLAGS = 512;
pub const EvtQueryStatuses: EVT_QUERY_PROPERTY_ID = 1;
pub const EvtQueryTolerateQueryErrors: EVT_QUERY_FLAGS = 4096;
pub const EvtRenderBookmark: EVT_RENDER_FLAGS = 2;
pub const EvtRenderContextSystem: EVT_RENDER_CONTEXT_FLAGS = 1;
pub const EvtRenderContextUser: EVT_RENDER_CONTEXT_FLAGS = 2;
pub const EvtRenderContextValues: EVT_RENDER_CONTEXT_FLAGS = 0;
pub const EvtRenderEventValues: EVT_RENDER_FLAGS = 0;
pub const EvtRenderEventXml: EVT_RENDER_FLAGS = 1;
pub const EvtRpcLogin: EVT_LOGIN_CLASS = 1;
pub const EvtRpcLoginAuthDefault: EVT_RPC_LOGIN_FLAGS = 0;
pub const EvtRpcLoginAuthKerberos: EVT_RPC_LOGIN_FLAGS = 2;
pub const EvtRpcLoginAuthNTLM: EVT_RPC_LOGIN_FLAGS = 3;
pub const EvtRpcLoginAuthNegotiate: EVT_RPC_LOGIN_FLAGS = 1;
pub const EvtSeekOriginMask: EVT_SEEK_FLAGS = 7;
pub const EvtSeekRelativeToBookmark: EVT_SEEK_FLAGS = 4;
pub const EvtSeekRelativeToCurrent: EVT_SEEK_FLAGS = 3;
pub const EvtSeekRelativeToFirst: EVT_SEEK_FLAGS = 1;
pub const EvtSeekRelativeToLast: EVT_SEEK_FLAGS = 2;
pub const EvtSeekStrict: EVT_SEEK_FLAGS = 65536;
pub const EvtSubscribeActionDeliver: EVT_SUBSCRIBE_NOTIFY_ACTION = 1;
pub const EvtSubscribeActionError: EVT_SUBSCRIBE_NOTIFY_ACTION = 0;
pub const EvtSubscribeOriginMask: EVT_SUBSCRIBE_FLAGS = 3;
pub const EvtSubscribeStartAfterBookmark: EVT_SUBSCRIBE_FLAGS = 3;
pub const EvtSubscribeStartAtOldestRecord: EVT_SUBSCRIBE_FLAGS = 2;
pub const EvtSubscribeStrict: EVT_SUBSCRIBE_FLAGS = 65536;
pub const EvtSubscribeToFutureEvents: EVT_SUBSCRIBE_FLAGS = 1;
pub const EvtSubscribeTolerateQueryErrors: EVT_SUBSCRIBE_FLAGS = 4096;
pub const EvtSystemActivityID: EVT_SYSTEM_PROPERTY_ID = 10;
pub const EvtSystemChannel: EVT_SYSTEM_PROPERTY_ID = 14;
pub const EvtSystemComputer: EVT_SYSTEM_PROPERTY_ID = 15;
pub const EvtSystemEventID: EVT_SYSTEM_PROPERTY_ID = 2;
pub const EvtSystemEventRecordId: EVT_SYSTEM_PROPERTY_ID = 9;
pub const EvtSystemKeywords: EVT_SYSTEM_PROPERTY_ID = 7;
pub const EvtSystemLevel: EVT_SYSTEM_PROPERTY_ID = 4;
pub const EvtSystemOpcode: EVT_SYSTEM_PROPERTY_ID = 6;
pub const EvtSystemProcessID: EVT_SYSTEM_PROPERTY_ID = 12;
pub const EvtSystemPropertyIdEND: EVT_SYSTEM_PROPERTY_ID = 18;
pub const EvtSystemProviderGuid: EVT_SYSTEM_PROPERTY_ID = 1;
pub const EvtSystemProviderName: EVT_SYSTEM_PROPERTY_ID = 0;
pub const EvtSystemQualifiers: EVT_SYSTEM_PROPERTY_ID = 3;
pub const EvtSystemRelatedActivityID: EVT_SYSTEM_PROPERTY_ID = 11;
pub const EvtSystemTask: EVT_SYSTEM_PROPERTY_ID = 5;
pub const EvtSystemThreadID: EVT_SYSTEM_PROPERTY_ID = 13;
pub const EvtSystemTimeCreated: EVT_SYSTEM_PROPERTY_ID = 8;
pub const EvtSystemUserID: EVT_SYSTEM_PROPERTY_ID = 16;
pub const EvtSystemVersion: EVT_SYSTEM_PROPERTY_ID = 17;
pub const EvtVarTypeAnsiString: EVT_VARIANT_TYPE = 2;
pub const EvtVarTypeBinary: EVT_VARIANT_TYPE = 14;
pub const EvtVarTypeBoolean: EVT_VARIANT_TYPE = 13;
pub const EvtVarTypeByte: EVT_VARIANT_TYPE = 4;
pub const EvtVarTypeDouble: EVT_VARIANT_TYPE = 12;
pub const EvtVarTypeEvtHandle: EVT_VARIANT_TYPE = 32;
pub const EvtVarTypeEvtXml: EVT_VARIANT_TYPE = 35;
pub const EvtVarTypeFileTime: EVT_VARIANT_TYPE = 17;
pub const EvtVarTypeGuid: EVT_VARIANT_TYPE = 15;
pub const EvtVarTypeHexInt32: EVT_VARIANT_TYPE = 20;
pub const EvtVarTypeHexInt64: EVT_VARIANT_TYPE = 21;
pub const EvtVarTypeInt16: EVT_VARIANT_TYPE = 5;
pub const EvtVarTypeInt32: EVT_VARIANT_TYPE = 7;
pub const EvtVarTypeInt64: EVT_VARIANT_TYPE = 9;
pub const EvtVarTypeNull: EVT_VARIANT_TYPE = 0;
pub const EvtVarTypeSByte: EVT_VARIANT_TYPE = 3;
pub const EvtVarTypeSid: EVT_VARIANT_TYPE = 19;
pub const EvtVarTypeSingle: EVT_VARIANT_TYPE = 11;
pub const EvtVarTypeSizeT: EVT_VARIANT_TYPE = 16;
pub const EvtVarTypeString: EVT_VARIANT_TYPE = 1;
pub const EvtVarTypeSysTime: EVT_VARIANT_TYPE = 18;
pub const EvtVarTypeUInt16: EVT_VARIANT_TYPE = 6;
pub const EvtVarTypeUInt32: EVT_VARIANT_TYPE = 8;
pub const EvtVarTypeUInt64: EVT_VARIANT_TYPE = 10;
#[cfg(feature = "winnt")]
pub type PEVT_HANDLE = *mut super::winnt::HANDLE;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
pub type PEVT_VARIANT = *mut EVT_VARIANT;
