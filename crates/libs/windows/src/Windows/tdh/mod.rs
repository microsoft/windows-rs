#[cfg(feature = "evntprov")]
#[inline]
pub unsafe fn TdhAggregatePayloadFilters(payloadfiltercount: u32, payloadfilterptrs: *const *const core::ffi::c_void, eventmatchallflags: Option<*const bool>, eventfilterdescriptor: *mut super::evntprov::EVENT_FILTER_DESCRIPTOR) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhAggregatePayloadFilters(payloadfiltercount : u32, payloadfilterptrs : *const *const core::ffi::c_void, eventmatchallflags : *const bool, eventfilterdescriptor : *mut super::evntprov::EVENT_FILTER_DESCRIPTOR) -> TDHSTATUS);
    unsafe { TdhAggregatePayloadFilters(payloadfiltercount, payloadfilterptrs, eventmatchallflags.unwrap_or(core::mem::zeroed()) as _, eventfilterdescriptor as _) }
}
#[cfg(feature = "evntprov")]
#[inline]
pub unsafe fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor: *mut super::evntprov::EVENT_FILTER_DESCRIPTOR) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor : *mut super::evntprov::EVENT_FILTER_DESCRIPTOR) -> TDHSTATUS);
    unsafe { TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TdhCloseDecodingHandle(handle: TDH_HANDLE) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhCloseDecodingHandle(handle : TDH_HANDLE) -> TDHSTATUS);
    unsafe { TdhCloseDecodingHandle(handle) }
}
#[cfg(feature = "evntprov")]
#[inline]
pub unsafe fn TdhCreatePayloadFilter(providerguid: *const windows_core::GUID, eventdescriptor: *const super::evntprov::EVENT_DESCRIPTOR, eventmatchany: bool, payloadpredicates: &[PAYLOAD_FILTER_PREDICATE], payloadfilter: *mut *mut core::ffi::c_void) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhCreatePayloadFilter(providerguid : *const windows_core::GUID, eventdescriptor : *const super::evntprov::EVENT_DESCRIPTOR, eventmatchany : bool, payloadpredicatecount : u32, payloadpredicates : *const PAYLOAD_FILTER_PREDICATE, payloadfilter : *mut *mut core::ffi::c_void) -> TDHSTATUS);
    unsafe { TdhCreatePayloadFilter(providerguid, eventdescriptor, eventmatchany, payloadpredicates.len().try_into().unwrap(), payloadpredicates.as_ptr(), payloadfilter as _) }
}
#[inline]
pub unsafe fn TdhDeletePayloadFilter(payloadfilter: *mut *mut core::ffi::c_void) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhDeletePayloadFilter(payloadfilter : *mut *mut core::ffi::c_void) -> TDHSTATUS);
    unsafe { TdhDeletePayloadFilter(payloadfilter as _) }
}
#[cfg(feature = "evntprov")]
#[inline]
pub unsafe fn TdhEnumerateManifestProviderEvents(providerguid: *const windows_core::GUID, buffer: Option<*mut PROVIDER_EVENT_INFO>, buffersize: *mut u32) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhEnumerateManifestProviderEvents(providerguid : *const windows_core::GUID, buffer : *mut PROVIDER_EVENT_INFO, buffersize : *mut u32) -> TDHSTATUS);
    unsafe { TdhEnumerateManifestProviderEvents(providerguid, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn TdhEnumerateProviderFieldInformation(pguid: *const windows_core::GUID, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: Option<*mut PROVIDER_FIELD_INFOARRAY>, pbuffersize: *mut u32) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhEnumerateProviderFieldInformation(pguid : *const windows_core::GUID, eventfieldtype : EVENT_FIELD_TYPE, pbuffer : *mut PROVIDER_FIELD_INFOARRAY, pbuffersize : *mut u32) -> TDHSTATUS);
    unsafe { TdhEnumerateProviderFieldInformation(pguid, eventfieldtype, pbuffer.unwrap_or(core::mem::zeroed()) as _, pbuffersize as _) }
}
#[inline]
pub unsafe fn TdhEnumerateProviderFilters(guid: *const windows_core::GUID, tdhcontext: Option<&[TDH_CONTEXT]>, filtercount: *mut u32, buffer: Option<*mut PPROVIDER_FILTER_INFO>, buffersize: *mut u32) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhEnumerateProviderFilters(guid : *const windows_core::GUID, tdhcontextcount : u32, tdhcontext : *const TDH_CONTEXT, filtercount : *mut u32, buffer : *mut PPROVIDER_FILTER_INFO, buffersize : *mut u32) -> TDHSTATUS);
    unsafe { TdhEnumerateProviderFilters(guid, tdhcontext.map_or(0, |slice| slice.len().try_into().unwrap()), tdhcontext.map_or(core::ptr::null(), |slice| slice.as_ptr()), filtercount as _, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn TdhEnumerateProviders(pbuffer: Option<*mut PROVIDER_ENUMERATION_INFO>, pbuffersize: *mut u32) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhEnumerateProviders(pbuffer : *mut PROVIDER_ENUMERATION_INFO, pbuffersize : *mut u32) -> TDHSTATUS);
    unsafe { TdhEnumerateProviders(pbuffer.unwrap_or(core::mem::zeroed()) as _, pbuffersize as _) }
}
#[inline]
pub unsafe fn TdhEnumerateProvidersForDecodingSource(filter: DECODING_SOURCE, buffer: Option<*mut PROVIDER_ENUMERATION_INFO>, buffersize: u32, bufferrequired: *mut u32) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhEnumerateProvidersForDecodingSource(filter : DECODING_SOURCE, buffer : *mut PROVIDER_ENUMERATION_INFO, buffersize : u32, bufferrequired : *mut u32) -> TDHSTATUS);
    unsafe { TdhEnumerateProvidersForDecodingSource(filter, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize, bufferrequired as _) }
}
#[cfg(feature = "evntprov")]
#[inline]
pub unsafe fn TdhFormatProperty(eventinfo: *const TRACE_EVENT_INFO, mapinfo: Option<*const EVENT_MAP_INFO>, pointersize: u32, propertyintype: u16, propertyouttype: u16, propertylength: u16, userdata: &[u8], buffersize: *mut u32, buffer: Option<*mut u16>, userdataconsumed: *mut u16) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhFormatProperty(eventinfo : *const TRACE_EVENT_INFO, mapinfo : *const EVENT_MAP_INFO, pointersize : u32, propertyintype : u16, propertyouttype : u16, propertylength : u16, userdatalength : u16, userdata : *const u8, buffersize : *mut u32, buffer : *mut u16, userdataconsumed : *mut u16) -> TDHSTATUS);
    unsafe { TdhFormatProperty(eventinfo, mapinfo.unwrap_or(core::mem::zeroed()) as _, pointersize, propertyintype, propertyouttype, propertylength, userdata.len().try_into().unwrap(), userdata.as_ptr(), buffersize as _, buffer.unwrap_or(core::mem::zeroed()) as _, userdataconsumed as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TdhGetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *mut TDH_CONTEXT) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhGetDecodingParameter(handle : TDH_HANDLE, tdhcontext : *mut TDH_CONTEXT) -> TDHSTATUS);
    unsafe { TdhGetDecodingParameter(handle, tdhcontext as _) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace"))]
#[inline]
pub unsafe fn TdhGetEventInformation(event: *const super::evntcons::EVENT_RECORD, tdhcontext: Option<&[TDH_CONTEXT]>, buffer: Option<*mut TRACE_EVENT_INFO>, buffersize: *mut u32) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhGetEventInformation(event : *const super::evntcons::EVENT_RECORD, tdhcontextcount : u32, tdhcontext : *const TDH_CONTEXT, buffer : *mut TRACE_EVENT_INFO, buffersize : *mut u32) -> TDHSTATUS);
    unsafe { TdhGetEventInformation(event, tdhcontext.map_or(0, |slice| slice.len().try_into().unwrap()), tdhcontext.map_or(core::ptr::null(), |slice| slice.as_ptr()), buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace"))]
#[inline]
pub unsafe fn TdhGetEventMapInformation<P1>(pevent: *const super::evntcons::EVENT_RECORD, pmapname: P1, pbuffer: Option<*mut EVENT_MAP_INFO>, pbuffersize: *mut u32) -> TDHSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("tdh.dll" "system" fn TdhGetEventMapInformation(pevent : *const super::evntcons::EVENT_RECORD, pmapname : windows_core::PCWSTR, pbuffer : *mut EVENT_MAP_INFO, pbuffersize : *mut u32) -> TDHSTATUS);
    unsafe { TdhGetEventMapInformation(pevent, pmapname.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, pbuffersize as _) }
}
#[cfg(feature = "evntprov")]
#[inline]
pub unsafe fn TdhGetManifestEventInformation(providerguid: *const windows_core::GUID, eventdescriptor: *const super::evntprov::EVENT_DESCRIPTOR, buffer: Option<*mut TRACE_EVENT_INFO>, buffersize: *mut u32) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhGetManifestEventInformation(providerguid : *const windows_core::GUID, eventdescriptor : *const super::evntprov::EVENT_DESCRIPTOR, buffer : *mut TRACE_EVENT_INFO, buffersize : *mut u32) -> TDHSTATUS);
    unsafe { TdhGetManifestEventInformation(providerguid, eventdescriptor, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace"))]
#[inline]
pub unsafe fn TdhGetProperty(pevent: *const super::evntcons::EVENT_RECORD, ptdhcontext: Option<&[TDH_CONTEXT]>, ppropertydata: &[PROPERTY_DATA_DESCRIPTOR], pbuffer: &mut [u8]) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhGetProperty(pevent : *const super::evntcons::EVENT_RECORD, tdhcontextcount : u32, ptdhcontext : *const TDH_CONTEXT, propertydatacount : u32, ppropertydata : *const PROPERTY_DATA_DESCRIPTOR, buffersize : u32, pbuffer : *mut u8) -> TDHSTATUS);
    unsafe { TdhGetProperty(pevent, ptdhcontext.map_or(0, |slice| slice.len().try_into().unwrap()), ptdhcontext.map_or(core::ptr::null(), |slice| slice.as_ptr()), ppropertydata.len().try_into().unwrap(), ppropertydata.as_ptr(), pbuffer.len().try_into().unwrap(), pbuffer.as_mut_ptr()) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace"))]
#[inline]
pub unsafe fn TdhGetPropertySize(pevent: *const super::evntcons::EVENT_RECORD, ptdhcontext: Option<&[TDH_CONTEXT]>, ppropertydata: &[PROPERTY_DATA_DESCRIPTOR], ppropertysize: *mut u32) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhGetPropertySize(pevent : *const super::evntcons::EVENT_RECORD, tdhcontextcount : u32, ptdhcontext : *const TDH_CONTEXT, propertydatacount : u32, ppropertydata : *const PROPERTY_DATA_DESCRIPTOR, ppropertysize : *mut u32) -> TDHSTATUS);
    unsafe { TdhGetPropertySize(pevent, ptdhcontext.map_or(0, |slice| slice.len().try_into().unwrap()), ptdhcontext.map_or(core::ptr::null(), |slice| slice.as_ptr()), ppropertydata.len().try_into().unwrap(), ppropertydata.as_ptr(), ppropertysize as _) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace", feature = "winnt"))]
#[inline]
pub unsafe fn TdhGetWppMessage(handle: TDH_HANDLE, eventrecord: *const super::evntcons::EVENT_RECORD, buffersize: *mut u32, buffer: *mut u8) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhGetWppMessage(handle : TDH_HANDLE, eventrecord : *const super::evntcons::EVENT_RECORD, buffersize : *mut u32, buffer : *mut u8) -> TDHSTATUS);
    unsafe { TdhGetWppMessage(handle, eventrecord, buffersize as _, buffer as _) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace", feature = "winnt"))]
#[inline]
pub unsafe fn TdhGetWppProperty<P2>(handle: TDH_HANDLE, eventrecord: *const super::evntcons::EVENT_RECORD, propertyname: P2, buffersize: *mut u32, buffer: *mut u8) -> TDHSTATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("tdh.dll" "system" fn TdhGetWppProperty(handle : TDH_HANDLE, eventrecord : *const super::evntcons::EVENT_RECORD, propertyname : windows_core::PCWSTR, buffersize : *mut u32, buffer : *mut u8) -> TDHSTATUS);
    unsafe { TdhGetWppProperty(handle, eventrecord, propertyname.param().abi(), buffersize as _, buffer as _) }
}
#[inline]
pub unsafe fn TdhLoadManifest<P0>(manifest: P0) -> TDHSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("tdh.dll" "system" fn TdhLoadManifest(manifest : windows_core::PCWSTR) -> TDHSTATUS);
    unsafe { TdhLoadManifest(manifest.param().abi()) }
}
#[inline]
pub unsafe fn TdhLoadManifestFromBinary<P0>(binarypath: P0) -> TDHSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("tdh.dll" "system" fn TdhLoadManifestFromBinary(binarypath : windows_core::PCWSTR) -> TDHSTATUS);
    unsafe { TdhLoadManifestFromBinary(binarypath.param().abi()) }
}
#[inline]
pub unsafe fn TdhLoadManifestFromMemory(pdata: *const core::ffi::c_void, cbdata: u32) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhLoadManifestFromMemory(pdata : *const core::ffi::c_void, cbdata : u32) -> TDHSTATUS);
    unsafe { TdhLoadManifestFromMemory(pdata, cbdata) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TdhOpenDecodingHandle(handle: *mut super::winnt::HANDLE) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhOpenDecodingHandle(handle : *mut super::winnt::HANDLE) -> TDHSTATUS);
    unsafe { TdhOpenDecodingHandle(handle as _) }
}
#[inline]
pub unsafe fn TdhQueryProviderFieldInformation(pguid: *const windows_core::GUID, eventfieldvalue: u64, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: Option<*mut PROVIDER_FIELD_INFOARRAY>, pbuffersize: *mut u32) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhQueryProviderFieldInformation(pguid : *const windows_core::GUID, eventfieldvalue : u64, eventfieldtype : EVENT_FIELD_TYPE, pbuffer : *mut PROVIDER_FIELD_INFOARRAY, pbuffersize : *mut u32) -> TDHSTATUS);
    unsafe { TdhQueryProviderFieldInformation(pguid, eventfieldvalue, eventfieldtype, pbuffer.unwrap_or(core::mem::zeroed()) as _, pbuffersize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TdhSetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *const TDH_CONTEXT) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhSetDecodingParameter(handle : TDH_HANDLE, tdhcontext : *const TDH_CONTEXT) -> TDHSTATUS);
    unsafe { TdhSetDecodingParameter(handle, tdhcontext) }
}
#[inline]
pub unsafe fn TdhUnloadManifest<P0>(manifest: P0) -> TDHSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("tdh.dll" "system" fn TdhUnloadManifest(manifest : windows_core::PCWSTR) -> TDHSTATUS);
    unsafe { TdhUnloadManifest(manifest.param().abi()) }
}
#[inline]
pub unsafe fn TdhUnloadManifestFromMemory(pdata: *const core::ffi::c_void, cbdata: u32) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhUnloadManifestFromMemory(pdata : *const core::ffi::c_void, cbdata : u32) -> TDHSTATUS);
    unsafe { TdhUnloadManifestFromMemory(pdata, cbdata) }
}
pub type DECODING_SOURCE = i32;
pub const DecodingSourceMax: DECODING_SOURCE = 4;
pub const DecodingSourceTlg: DECODING_SOURCE = 3;
pub const DecodingSourceWPP: DECODING_SOURCE = 2;
pub const DecodingSourceWbem: DECODING_SOURCE = 1;
pub const DecodingSourceXMLFile: DECODING_SOURCE = 0;
pub const EVENTMAP_ENTRY_VALUETYPE_STRING: MAP_VALUETYPE = 1;
pub const EVENTMAP_ENTRY_VALUETYPE_ULONG: MAP_VALUETYPE = 0;
pub const EVENTMAP_INFO_FLAG_MANIFEST_BITMAP: MAP_FLAGS = 2;
pub const EVENTMAP_INFO_FLAG_MANIFEST_PATTERNMAP: MAP_FLAGS = 4;
pub const EVENTMAP_INFO_FLAG_MANIFEST_VALUEMAP: MAP_FLAGS = 1;
pub const EVENTMAP_INFO_FLAG_WBEM_BITMAP: MAP_FLAGS = 16;
pub const EVENTMAP_INFO_FLAG_WBEM_FLAG: MAP_FLAGS = 32;
pub const EVENTMAP_INFO_FLAG_WBEM_NO_MAP: MAP_FLAGS = 64;
pub const EVENTMAP_INFO_FLAG_WBEM_VALUEMAP: MAP_FLAGS = 8;
pub type EVENT_FIELD_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_MAP_ENTRY {
    pub OutputOffset: u32,
    pub Anonymous: EVENT_MAP_ENTRY_0,
}
impl Default for EVENT_MAP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_MAP_ENTRY_0 {
    pub Value: u32,
    pub InputOffset: u32,
}
impl Default for EVENT_MAP_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_MAP_INFO {
    pub NameOffset: u32,
    pub Flag: MAP_FLAGS,
    pub EntryCount: u32,
    pub Anonymous: EVENT_MAP_INFO_0,
    pub MapEntryArray: [EVENT_MAP_ENTRY; 1],
}
impl Default for EVENT_MAP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_MAP_INFO_0 {
    pub MapEntryValueType: MAP_VALUETYPE,
    pub FormatStringOffset: u32,
}
impl Default for EVENT_MAP_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_PROPERTY_INFO {
    pub Flags: PROPERTY_FLAGS,
    pub NameOffset: u32,
    pub Anonymous: EVENT_PROPERTY_INFO_0,
    pub Anonymous2: EVENT_PROPERTY_INFO_1,
    pub Anonymous3: EVENT_PROPERTY_INFO_2,
    pub Anonymous4: EVENT_PROPERTY_INFO_3,
}
impl Default for EVENT_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_0 {
    pub nonStructType: EVENT_PROPERTY_INFO_0_0,
    pub structType: EVENT_PROPERTY_INFO_0_1,
    pub customSchemaType: EVENT_PROPERTY_INFO_0_2,
}
impl Default for EVENT_PROPERTY_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_1 {
    pub count: u16,
    pub countPropertyIndex: u16,
}
impl Default for EVENT_PROPERTY_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_2 {
    pub length: u16,
    pub lengthPropertyIndex: u16,
}
impl Default for EVENT_PROPERTY_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_3 {
    pub Reserved: u32,
    pub Anonymous: EVENT_PROPERTY_INFO_3_0,
}
impl Default for EVENT_PROPERTY_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_PROPERTY_INFO_3_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_PROPERTY_INFO_0_0 {
    pub InType: u16,
    pub OutType: u16,
    pub MapNameOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_PROPERTY_INFO_0_1 {
    pub StructStartIndex: u16,
    pub NumOfStructMembers: u16,
    pub padding: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_PROPERTY_INFO_0_2 {
    pub InType: u16,
    pub OutType: u16,
    pub CustomSchemaOffset: u32,
}
pub const EventChannelInformation: EVENT_FIELD_TYPE = 2;
pub const EventInformationMax: EVENT_FIELD_TYPE = 5;
pub const EventKeywordInformation: EVENT_FIELD_TYPE = 0;
pub const EventLevelInformation: EVENT_FIELD_TYPE = 1;
pub const EventOpcodeInformation: EVENT_FIELD_TYPE = 4;
pub const EventTaskInformation: EVENT_FIELD_TYPE = 3;
pub type MAP_FLAGS = i32;
pub type MAP_VALUETYPE = i32;
pub const MAX_PAYLOAD_PREDICATES: u32 = 8;
pub const PAYLOADFIELD_BETWEEN: PAYLOAD_OPERATOR = 6;
pub const PAYLOADFIELD_CONTAINS: PAYLOAD_OPERATOR = 20;
pub const PAYLOADFIELD_DOESNTCONTAIN: PAYLOAD_OPERATOR = 21;
pub const PAYLOADFIELD_EQ: PAYLOAD_OPERATOR = 0;
pub const PAYLOADFIELD_GE: PAYLOAD_OPERATOR = 5;
pub const PAYLOADFIELD_GT: PAYLOAD_OPERATOR = 3;
pub const PAYLOADFIELD_INVALID: PAYLOAD_OPERATOR = 32;
pub const PAYLOADFIELD_IS: PAYLOAD_OPERATOR = 30;
pub const PAYLOADFIELD_ISNOT: PAYLOAD_OPERATOR = 31;
pub const PAYLOADFIELD_LE: PAYLOAD_OPERATOR = 2;
pub const PAYLOADFIELD_LT: PAYLOAD_OPERATOR = 4;
pub const PAYLOADFIELD_MODULO: PAYLOAD_OPERATOR = 8;
pub const PAYLOADFIELD_NE: PAYLOAD_OPERATOR = 1;
pub const PAYLOADFIELD_NOTBETWEEN: PAYLOAD_OPERATOR = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAYLOAD_FILTER_PREDICATE {
    pub FieldName: windows_core::PWSTR,
    pub CompareOp: u16,
    pub Value: windows_core::PWSTR,
}
pub type PAYLOAD_OPERATOR = i32;
pub type PEVENT_MAP_ENTRY = *mut EVENT_MAP_ENTRY;
pub type PEVENT_MAP_INFO = *mut EVENT_MAP_INFO;
pub type PEVENT_PROPERTY_INFO = *mut EVENT_PROPERTY_INFO;
pub type PPAYLOAD_FILTER_PREDICATE = *mut PAYLOAD_FILTER_PREDICATE;
pub type PPROPERTY_DATA_DESCRIPTOR = *mut PROPERTY_DATA_DESCRIPTOR;
pub type PPROVIDER_ENUMERATION_INFO = *mut PROVIDER_ENUMERATION_INFO;
#[cfg(feature = "evntprov")]
pub type PPROVIDER_EVENT_INFO = *mut PROVIDER_EVENT_INFO;
pub type PPROVIDER_FIELD_INFO = *mut PROVIDER_FIELD_INFO;
pub type PPROVIDER_FIELD_INFOARRAY = *mut PROVIDER_FIELD_INFOARRAY;
pub type PPROVIDER_FILTER_INFO = *mut PROVIDER_FILTER_INFO;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPERTY_DATA_DESCRIPTOR {
    pub PropertyName: u64,
    pub ArrayIndex: u32,
    pub Reserved: u32,
}
pub type PROPERTY_FLAGS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROVIDER_ENUMERATION_INFO {
    pub NumberOfProviders: u32,
    pub Reserved: u32,
    pub TraceProviderInfoArray: [TRACE_PROVIDER_INFO; 1],
}
impl Default for PROVIDER_ENUMERATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "evntprov")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROVIDER_EVENT_INFO {
    pub NumberOfEvents: u32,
    pub Reserved: u32,
    pub EventDescriptorsArray: [super::evntprov::EVENT_DESCRIPTOR; 1],
}
#[cfg(feature = "evntprov")]
impl Default for PROVIDER_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROVIDER_FIELD_INFO {
    pub NameOffset: u32,
    pub DescriptionOffset: u32,
    pub Value: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROVIDER_FIELD_INFOARRAY {
    pub NumberOfElements: u32,
    pub FieldType: EVENT_FIELD_TYPE,
    pub FieldInfoArray: [PROVIDER_FIELD_INFO; 1],
}
impl Default for PROVIDER_FIELD_INFOARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROVIDER_FILTER_INFO {
    pub Id: u8,
    pub Version: u8,
    pub MessageOffset: u32,
    pub Reserved: u32,
    pub PropertyCount: u32,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
impl Default for PROVIDER_FILTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PTDH_CONTEXT = *mut TDH_CONTEXT;
#[cfg(feature = "winnt")]
pub type PTDH_HANDLE = *mut super::winnt::HANDLE;
#[cfg(feature = "evntprov")]
pub type PTRACE_EVENT_INFO = *mut TRACE_EVENT_INFO;
pub type PTRACE_PROVIDER_INFO = *mut TRACE_PROVIDER_INFO;
pub const PropertyHasCustomSchema: PROPERTY_FLAGS = 128;
pub const PropertyHasTags: PROPERTY_FLAGS = 64;
pub const PropertyParamCount: PROPERTY_FLAGS = 4;
pub const PropertyParamFixedCount: PROPERTY_FLAGS = 32;
pub const PropertyParamFixedLength: PROPERTY_FLAGS = 16;
pub const PropertyParamLength: PROPERTY_FLAGS = 2;
pub const PropertyStruct: PROPERTY_FLAGS = 1;
pub const PropertyWBEMXmlFragment: PROPERTY_FLAGS = 8;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TDHSTATUS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TDH_CONTEXT {
    pub ParameterValue: u64,
    pub ParameterType: TDH_CONTEXT_TYPE,
    pub ParameterSize: u32,
}
pub const TDH_CONTEXT_MAXIMUM: TDH_CONTEXT_TYPE = 5;
pub const TDH_CONTEXT_PDB_PATH: TDH_CONTEXT_TYPE = 4;
pub const TDH_CONTEXT_POINTERSIZE: TDH_CONTEXT_TYPE = 3;
pub type TDH_CONTEXT_TYPE = i32;
pub const TDH_CONTEXT_WPP_GMT: TDH_CONTEXT_TYPE = 2;
pub const TDH_CONTEXT_WPP_TMFFILE: TDH_CONTEXT_TYPE = 0;
pub const TDH_CONTEXT_WPP_TMFSEARCHPATH: TDH_CONTEXT_TYPE = 1;
#[cfg(feature = "winnt")]
pub type TDH_HANDLE = super::winnt::HANDLE;
pub const TDH_INTYPE_ANSICHAR: _TDH_IN_TYPE = 307;
pub const TDH_INTYPE_ANSISTRING: _TDH_IN_TYPE = 2;
pub const TDH_INTYPE_BINARY: _TDH_IN_TYPE = 14;
pub const TDH_INTYPE_BOOLEAN: _TDH_IN_TYPE = 13;
pub const TDH_INTYPE_COUNTEDANSISTRING: _TDH_IN_TYPE = 301;
pub const TDH_INTYPE_COUNTEDSTRING: _TDH_IN_TYPE = 300;
pub const TDH_INTYPE_DOUBLE: _TDH_IN_TYPE = 12;
pub const TDH_INTYPE_FILETIME: _TDH_IN_TYPE = 17;
pub const TDH_INTYPE_FLOAT: _TDH_IN_TYPE = 11;
pub const TDH_INTYPE_GUID: _TDH_IN_TYPE = 15;
pub const TDH_INTYPE_HEXDUMP: _TDH_IN_TYPE = 309;
pub const TDH_INTYPE_HEXINT32: _TDH_IN_TYPE = 20;
pub const TDH_INTYPE_HEXINT64: _TDH_IN_TYPE = 21;
pub const TDH_INTYPE_INT16: _TDH_IN_TYPE = 5;
pub const TDH_INTYPE_INT32: _TDH_IN_TYPE = 7;
pub const TDH_INTYPE_INT64: _TDH_IN_TYPE = 9;
pub const TDH_INTYPE_INT8: _TDH_IN_TYPE = 3;
pub const TDH_INTYPE_MANIFEST_COUNTEDANSISTRING: _TDH_IN_TYPE = 23;
pub const TDH_INTYPE_MANIFEST_COUNTEDBINARY: _TDH_IN_TYPE = 25;
pub const TDH_INTYPE_MANIFEST_COUNTEDSTRING: _TDH_IN_TYPE = 22;
pub const TDH_INTYPE_NONNULLTERMINATEDANSISTRING: _TDH_IN_TYPE = 305;
pub const TDH_INTYPE_NONNULLTERMINATEDSTRING: _TDH_IN_TYPE = 304;
pub const TDH_INTYPE_NULL: _TDH_IN_TYPE = 0;
pub const TDH_INTYPE_POINTER: _TDH_IN_TYPE = 16;
pub const TDH_INTYPE_RESERVED24: _TDH_IN_TYPE = 24;
pub const TDH_INTYPE_REVERSEDCOUNTEDANSISTRING: _TDH_IN_TYPE = 303;
pub const TDH_INTYPE_REVERSEDCOUNTEDSTRING: _TDH_IN_TYPE = 302;
pub const TDH_INTYPE_SID: _TDH_IN_TYPE = 19;
pub const TDH_INTYPE_SIZET: _TDH_IN_TYPE = 308;
pub const TDH_INTYPE_SYSTEMTIME: _TDH_IN_TYPE = 18;
pub const TDH_INTYPE_UINT16: _TDH_IN_TYPE = 6;
pub const TDH_INTYPE_UINT32: _TDH_IN_TYPE = 8;
pub const TDH_INTYPE_UINT64: _TDH_IN_TYPE = 10;
pub const TDH_INTYPE_UINT8: _TDH_IN_TYPE = 4;
pub const TDH_INTYPE_UNICODECHAR: _TDH_IN_TYPE = 306;
pub const TDH_INTYPE_UNICODESTRING: _TDH_IN_TYPE = 1;
pub const TDH_INTYPE_WBEMSID: _TDH_IN_TYPE = 310;
pub const TDH_OUTTYPE_BOOLEAN: _TDH_OUT_TYPE = 13;
pub const TDH_OUTTYPE_BYTE: _TDH_OUT_TYPE = 3;
pub const TDH_OUTTYPE_CIMDATETIME: _TDH_OUT_TYPE = 26;
pub const TDH_OUTTYPE_CODE_POINTER: _TDH_OUT_TYPE = 37;
pub const TDH_OUTTYPE_CULTURE_INSENSITIVE_DATETIME: _TDH_OUT_TYPE = 33;
pub const TDH_OUTTYPE_DATETIME: _TDH_OUT_TYPE = 2;
pub const TDH_OUTTYPE_DATETIME_UTC: _TDH_OUT_TYPE = 38;
pub const TDH_OUTTYPE_DOUBLE: _TDH_OUT_TYPE = 12;
pub const TDH_OUTTYPE_ERRORCODE: _TDH_OUT_TYPE = 29;
pub const TDH_OUTTYPE_ETWTIME: _TDH_OUT_TYPE = 27;
pub const TDH_OUTTYPE_FLOAT: _TDH_OUT_TYPE = 11;
pub const TDH_OUTTYPE_GUID: _TDH_OUT_TYPE = 14;
pub const TDH_OUTTYPE_HEXBINARY: _TDH_OUT_TYPE = 15;
pub const TDH_OUTTYPE_HEXINT16: _TDH_OUT_TYPE = 17;
pub const TDH_OUTTYPE_HEXINT32: _TDH_OUT_TYPE = 18;
pub const TDH_OUTTYPE_HEXINT64: _TDH_OUT_TYPE = 19;
pub const TDH_OUTTYPE_HEXINT8: _TDH_OUT_TYPE = 16;
pub const TDH_OUTTYPE_HRESULT: _TDH_OUT_TYPE = 32;
pub const TDH_OUTTYPE_INT: _TDH_OUT_TYPE = 7;
pub const TDH_OUTTYPE_IPV4: _TDH_OUT_TYPE = 23;
pub const TDH_OUTTYPE_IPV6: _TDH_OUT_TYPE = 24;
pub const TDH_OUTTYPE_JSON: _TDH_OUT_TYPE = 34;
pub const TDH_OUTTYPE_LONG: _TDH_OUT_TYPE = 9;
pub const TDH_OUTTYPE_NOPRINT: _TDH_OUT_TYPE = 301;
pub const TDH_OUTTYPE_NTSTATUS: _TDH_OUT_TYPE = 31;
pub const TDH_OUTTYPE_NULL: _TDH_OUT_TYPE = 0;
pub const TDH_OUTTYPE_PID: _TDH_OUT_TYPE = 20;
pub const TDH_OUTTYPE_PKCS7_WITH_TYPE_INFO: _TDH_OUT_TYPE = 36;
pub const TDH_OUTTYPE_PORT: _TDH_OUT_TYPE = 22;
pub const TDH_OUTTYPE_REDUCEDSTRING: _TDH_OUT_TYPE = 300;
pub const TDH_OUTTYPE_SHORT: _TDH_OUT_TYPE = 5;
pub const TDH_OUTTYPE_SOCKETADDRESS: _TDH_OUT_TYPE = 25;
pub const TDH_OUTTYPE_STRING: _TDH_OUT_TYPE = 1;
pub const TDH_OUTTYPE_TID: _TDH_OUT_TYPE = 21;
pub const TDH_OUTTYPE_UNSIGNEDBYTE: _TDH_OUT_TYPE = 4;
pub const TDH_OUTTYPE_UNSIGNEDINT: _TDH_OUT_TYPE = 8;
pub const TDH_OUTTYPE_UNSIGNEDLONG: _TDH_OUT_TYPE = 10;
pub const TDH_OUTTYPE_UNSIGNEDSHORT: _TDH_OUT_TYPE = 6;
pub const TDH_OUTTYPE_UTF8: _TDH_OUT_TYPE = 35;
pub const TDH_OUTTYPE_WIN32ERROR: _TDH_OUT_TYPE = 30;
pub const TDH_OUTTYPE_XML: _TDH_OUT_TYPE = 28;
pub const TDH_OUTYTPE_ERRORCODE: u32 = 29;
pub const TEMPLATE_CONTROL_GUID: TEMPLATE_FLAGS = 4;
pub const TEMPLATE_EVENT_DATA: TEMPLATE_FLAGS = 1;
pub type TEMPLATE_FLAGS = i32;
pub const TEMPLATE_USER_DATA: TEMPLATE_FLAGS = 2;
#[repr(C)]
#[cfg(feature = "evntprov")]
#[derive(Clone, Copy)]
pub struct TRACE_EVENT_INFO {
    pub ProviderGuid: windows_core::GUID,
    pub EventGuid: windows_core::GUID,
    pub EventDescriptor: super::evntprov::EVENT_DESCRIPTOR,
    pub DecodingSource: DECODING_SOURCE,
    pub ProviderNameOffset: u32,
    pub LevelNameOffset: u32,
    pub ChannelNameOffset: u32,
    pub KeywordsNameOffset: u32,
    pub TaskNameOffset: u32,
    pub OpcodeNameOffset: u32,
    pub EventMessageOffset: u32,
    pub ProviderMessageOffset: u32,
    pub BinaryXMLOffset: u32,
    pub BinaryXMLSize: u32,
    pub Anonymous: TRACE_EVENT_INFO_0,
    pub Anonymous2: TRACE_EVENT_INFO_1,
    pub PropertyCount: u32,
    pub TopLevelPropertyCount: u32,
    pub Anonymous3: TRACE_EVENT_INFO_2,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
#[cfg(feature = "evntprov")]
impl Default for TRACE_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "evntprov")]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_0 {
    pub EventNameOffset: u32,
    pub ActivityIDNameOffset: u32,
}
#[cfg(feature = "evntprov")]
impl Default for TRACE_EVENT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "evntprov")]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_1 {
    pub EventAttributesOffset: u32,
    pub RelatedActivityIDNameOffset: u32,
}
#[cfg(feature = "evntprov")]
impl Default for TRACE_EVENT_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "evntprov")]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_2 {
    pub Flags: TEMPLATE_FLAGS,
    pub Anonymous: TRACE_EVENT_INFO_2_0,
}
#[cfg(feature = "evntprov")]
impl Default for TRACE_EVENT_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "evntprov")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_EVENT_INFO_2_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_PROVIDER_INFO {
    pub ProviderGuid: windows_core::GUID,
    pub SchemaSource: u32,
    pub ProviderNameOffset: u32,
}
pub type _TDH_IN_TYPE = i32;
pub type _TDH_OUT_TYPE = i32;
