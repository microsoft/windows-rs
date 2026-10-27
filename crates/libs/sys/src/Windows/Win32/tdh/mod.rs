#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt"))]
windows_link::link!("tdh.dll" "system" fn TdhAggregatePayloadFilters(payloadfiltercount : super::ULONG, payloadfilterptrs : *const super::PVOID, eventmatchallflags : super::PBOOLEAN, eventfilterdescriptor : super::PEVENT_FILTER_DESCRIPTOR) -> TDHSTATUS);
#[cfg(all(feature = "evntprov", feature = "shared"))]
windows_link::link!("tdh.dll" "system" fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor : super::PEVENT_FILTER_DESCRIPTOR) -> TDHSTATUS);
windows_link::link!("tdh.dll" "system" fn TdhCloseDecodingHandle(handle : TDH_HANDLE) -> TDHSTATUS);
#[cfg(all(feature = "evntprov", feature = "guiddef", feature = "shared", feature = "winnt"))]
windows_link::link!("tdh.dll" "system" fn TdhCreatePayloadFilter(providerguid : super::LPCGUID, eventdescriptor : super::PCEVENT_DESCRIPTOR, eventmatchany : super::BOOLEAN, payloadpredicatecount : super::ULONG, payloadpredicates : PPAYLOAD_FILTER_PREDICATE, payloadfilter : *mut super::PVOID) -> TDHSTATUS);
#[cfg(feature = "winnt")]
windows_link::link!("tdh.dll" "system" fn TdhDeletePayloadFilter(payloadfilter : *mut super::PVOID) -> TDHSTATUS);
#[cfg(all(feature = "evntprov", feature = "guiddef", feature = "shared"))]
windows_link::link!("tdh.dll" "system" fn TdhEnumerateManifestProviderEvents(providerguid : super::LPGUID, buffer : PPROVIDER_EVENT_INFO, buffersize : *mut super::ULONG) -> TDHSTATUS);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("tdh.dll" "system" fn TdhEnumerateProviderFieldInformation(pguid : super::LPGUID, eventfieldtype : EVENT_FIELD_TYPE, pbuffer : PPROVIDER_FIELD_INFOARRAY, pbuffersize : *mut super::ULONG) -> TDHSTATUS);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("tdh.dll" "system" fn TdhEnumerateProviderFilters(guid : super::LPGUID, tdhcontextcount : super::ULONG, tdhcontext : PTDH_CONTEXT, filtercount : *mut super::ULONG, buffer : *mut PPROVIDER_FILTER_INFO, buffersize : *mut super::ULONG) -> TDHSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("tdh.dll" "system" fn TdhEnumerateProviders(pbuffer : PPROVIDER_ENUMERATION_INFO, pbuffersize : *mut super::ULONG) -> TDHSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("tdh.dll" "system" fn TdhEnumerateProvidersForDecodingSource(filter : DECODING_SOURCE, buffer : *mut PROVIDER_ENUMERATION_INFO, buffersize : super::ULONG, bufferrequired : *mut super::ULONG) -> TDHSTATUS);
#[cfg(all(feature = "evntprov", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("tdh.dll" "system" fn TdhFormatProperty(eventinfo : PTRACE_EVENT_INFO, mapinfo : PEVENT_MAP_INFO, pointersize : super::ULONG, propertyintype : super::USHORT, propertyouttype : super::USHORT, propertylength : super::USHORT, userdatalength : super::USHORT, userdata : super::PBYTE, buffersize : super::PULONG, buffer : super::PWCHAR, userdataconsumed : super::PUSHORT) -> TDHSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("tdh.dll" "system" fn TdhGetDecodingParameter(handle : TDH_HANDLE, tdhcontext : PTDH_CONTEXT) -> TDHSTATUS);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("tdh.dll" "system" fn TdhGetEventInformation(event : super::PEVENT_RECORD, tdhcontextcount : super::ULONG, tdhcontext : PTDH_CONTEXT, buffer : PTRACE_EVENT_INFO, buffersize : super::PULONG) -> TDHSTATUS);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace", feature = "shared", feature = "winnt"))]
windows_link::link!("tdh.dll" "system" fn TdhGetEventMapInformation(pevent : super::PEVENT_RECORD, pmapname : windows_sys::core::PCWSTR, pbuffer : PEVENT_MAP_INFO, pbuffersize : *mut super::ULONG) -> TDHSTATUS);
#[cfg(all(feature = "evntprov", feature = "guiddef", feature = "shared"))]
windows_link::link!("tdh.dll" "system" fn TdhGetManifestEventInformation(providerguid : super::LPGUID, eventdescriptor : super::PEVENT_DESCRIPTOR, buffer : PTRACE_EVENT_INFO, buffersize : *mut super::ULONG) -> TDHSTATUS);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace", feature = "shared", feature = "winnt"))]
windows_link::link!("tdh.dll" "system" fn TdhGetProperty(pevent : super::PEVENT_RECORD, tdhcontextcount : super::ULONG, ptdhcontext : PTDH_CONTEXT, propertydatacount : super::ULONG, ppropertydata : PPROPERTY_DATA_DESCRIPTOR, buffersize : super::ULONG, pbuffer : super::PBYTE) -> TDHSTATUS);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace", feature = "shared", feature = "winnt"))]
windows_link::link!("tdh.dll" "system" fn TdhGetPropertySize(pevent : super::PEVENT_RECORD, tdhcontextcount : super::ULONG, ptdhcontext : PTDH_CONTEXT, propertydatacount : super::ULONG, ppropertydata : PPROPERTY_DATA_DESCRIPTOR, ppropertysize : *mut super::ULONG) -> TDHSTATUS);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("tdh.dll" "system" fn TdhGetWppMessage(handle : TDH_HANDLE, eventrecord : super::PEVENT_RECORD, buffersize : super::PULONG, buffer : super::PBYTE) -> TDHSTATUS);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "evntrace", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("tdh.dll" "system" fn TdhGetWppProperty(handle : TDH_HANDLE, eventrecord : super::PEVENT_RECORD, propertyname : windows_sys::core::PCWSTR, buffersize : super::PULONG, buffer : super::PBYTE) -> TDHSTATUS);
windows_link::link!("tdh.dll" "system" fn TdhLoadManifest(manifest : windows_sys::core::PCWSTR) -> TDHSTATUS);
windows_link::link!("tdh.dll" "system" fn TdhLoadManifestFromBinary(binarypath : windows_sys::core::PCWSTR) -> TDHSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("tdh.dll" "system" fn TdhLoadManifestFromMemory(pdata : super::LPCVOID, cbdata : super::ULONG) -> TDHSTATUS);
windows_link::link!("tdh.dll" "system" fn TdhOpenDecodingHandle(handle : PTDH_HANDLE) -> TDHSTATUS);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("tdh.dll" "system" fn TdhQueryProviderFieldInformation(pguid : super::LPGUID, eventfieldvalue : super::ULONGLONG, eventfieldtype : EVENT_FIELD_TYPE, pbuffer : PPROVIDER_FIELD_INFOARRAY, pbuffersize : *mut super::ULONG) -> TDHSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("tdh.dll" "system" fn TdhSetDecodingParameter(handle : TDH_HANDLE, tdhcontext : PTDH_CONTEXT) -> TDHSTATUS);
windows_link::link!("tdh.dll" "system" fn TdhUnloadManifest(manifest : windows_sys::core::PCWSTR) -> TDHSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("tdh.dll" "system" fn TdhUnloadManifestFromMemory(pdata : super::LPCVOID, cbdata : super::ULONG) -> TDHSTATUS);
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EVENT_MAP_ENTRY {
    pub OutputOffset: super::ULONG,
    pub Anonymous: EVENT_MAP_ENTRY_0,
}
#[cfg(feature = "shared")]
impl Default for EVENT_MAP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union EVENT_MAP_ENTRY_0 {
    pub Value: super::ULONG,
    pub InputOffset: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for EVENT_MAP_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EVENT_MAP_INFO {
    pub NameOffset: super::ULONG,
    pub Flag: MAP_FLAGS,
    pub EntryCount: super::ULONG,
    pub Anonymous: EVENT_MAP_INFO_0,
    pub MapEntryArray: [EVENT_MAP_ENTRY; 1],
}
#[cfg(feature = "shared")]
impl Default for EVENT_MAP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union EVENT_MAP_INFO_0 {
    pub MapEntryValueType: MAP_VALUETYPE,
    pub FormatStringOffset: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for EVENT_MAP_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EVENT_PROPERTY_INFO {
    pub Flags: PROPERTY_FLAGS,
    pub NameOffset: super::ULONG,
    pub Anonymous: EVENT_PROPERTY_INFO_0,
    pub Anonymous2: EVENT_PROPERTY_INFO_1,
    pub Anonymous3: EVENT_PROPERTY_INFO_2,
    pub Anonymous4: EVENT_PROPERTY_INFO_3,
}
#[cfg(feature = "shared")]
impl Default for EVENT_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_0 {
    pub nonStructType: EVENT_PROPERTY_INFO_0__nonStructType,
    pub structType: EVENT_PROPERTY_INFO_0__structType,
    pub customSchemaType: EVENT_PROPERTY_INFO_0__customSchemaType,
}
#[cfg(feature = "shared")]
impl Default for EVENT_PROPERTY_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EVENT_PROPERTY_INFO_0__customSchemaType {
    pub InType: super::USHORT,
    pub OutType: super::USHORT,
    pub CustomSchemaOffset: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EVENT_PROPERTY_INFO_0__nonStructType {
    pub InType: super::USHORT,
    pub OutType: super::USHORT,
    pub MapNameOffset: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EVENT_PROPERTY_INFO_0__structType {
    pub StructStartIndex: super::USHORT,
    pub NumOfStructMembers: super::USHORT,
    pub padding: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_1 {
    pub count: super::USHORT,
    pub countPropertyIndex: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for EVENT_PROPERTY_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_2 {
    pub length: super::USHORT,
    pub lengthPropertyIndex: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for EVENT_PROPERTY_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_3 {
    pub Reserved: super::ULONG,
    pub Anonymous: EVENT_PROPERTY_INFO_3_0,
}
#[cfg(feature = "shared")]
impl Default for EVENT_PROPERTY_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EVENT_PROPERTY_INFO_3_0 {
    pub _bitfield: u32,
}
pub const EventChannelInformation: EVENT_FIELD_TYPE = 2;
pub const EventInformationMax: EVENT_FIELD_TYPE = 5;
pub const EventKeywordInformation: EVENT_FIELD_TYPE = 0;
pub const EventLevelInformation: EVENT_FIELD_TYPE = 1;
pub const EventOpcodeInformation: EVENT_FIELD_TYPE = 4;
pub const EventTaskInformation: EVENT_FIELD_TYPE = 3;
pub type MAP_FLAGS = i32;
pub type MAP_VALUETYPE = i32;
pub const MAX_PAYLOAD_PREDICATES: i32 = 8;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PAYLOAD_FILTER_PREDICATE {
    pub FieldName: windows_sys::core::PWSTR,
    pub CompareOp: super::USHORT,
    pub Value: windows_sys::core::PWSTR,
}
pub type PAYLOAD_OPERATOR = i32;
#[cfg(feature = "shared")]
pub type PEVENT_MAP_ENTRY = *mut EVENT_MAP_ENTRY;
#[cfg(feature = "shared")]
pub type PEVENT_MAP_INFO = *mut EVENT_MAP_INFO;
#[cfg(feature = "shared")]
pub type PEVENT_PROPERTY_INFO = *mut EVENT_PROPERTY_INFO;
#[cfg(feature = "shared")]
pub type PPAYLOAD_FILTER_PREDICATE = *mut PAYLOAD_FILTER_PREDICATE;
#[cfg(feature = "shared")]
pub type PPROPERTY_DATA_DESCRIPTOR = *mut PROPERTY_DATA_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PPROVIDER_ENUMERATION_INFO = *mut PROVIDER_ENUMERATION_INFO;
#[cfg(all(feature = "evntprov", feature = "shared"))]
pub type PPROVIDER_EVENT_INFO = *mut PROVIDER_EVENT_INFO;
#[cfg(feature = "shared")]
pub type PPROVIDER_FIELD_INFO = *mut PROVIDER_FIELD_INFO;
#[cfg(feature = "shared")]
pub type PPROVIDER_FIELD_INFOARRAY = *mut PROVIDER_FIELD_INFOARRAY;
#[cfg(feature = "shared")]
pub type PPROVIDER_FILTER_INFO = *mut PROVIDER_FILTER_INFO;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROPERTY_DATA_DESCRIPTOR {
    pub PropertyName: super::ULONGLONG,
    pub ArrayIndex: super::ULONG,
    pub Reserved: super::ULONG,
}
pub type PROPERTY_FLAGS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PROVIDER_ENUMERATION_INFO {
    pub NumberOfProviders: super::ULONG,
    pub Reserved: super::ULONG,
    pub TraceProviderInfoArray: [TRACE_PROVIDER_INFO; 1],
}
#[cfg(feature = "shared")]
impl Default for PROVIDER_ENUMERATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct PROVIDER_EVENT_INFO {
    pub NumberOfEvents: super::ULONG,
    pub Reserved: super::ULONG,
    pub EventDescriptorsArray: [super::EVENT_DESCRIPTOR; 1],
}
#[cfg(all(feature = "evntprov", feature = "shared"))]
impl Default for PROVIDER_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROVIDER_FIELD_INFO {
    pub NameOffset: super::ULONG,
    pub DescriptionOffset: super::ULONG,
    pub Value: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PROVIDER_FIELD_INFOARRAY {
    pub NumberOfElements: super::ULONG,
    pub FieldType: EVENT_FIELD_TYPE,
    pub FieldInfoArray: [PROVIDER_FIELD_INFO; 1],
}
#[cfg(feature = "shared")]
impl Default for PROVIDER_FIELD_INFOARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PROVIDER_FILTER_INFO {
    pub Id: super::UCHAR,
    pub Version: super::UCHAR,
    pub MessageOffset: super::ULONG,
    pub Reserved: super::ULONG,
    pub PropertyCount: super::ULONG,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
#[cfg(feature = "shared")]
impl Default for PROVIDER_FILTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type PTDH_CONTEXT = *mut TDH_CONTEXT;
pub type PTDH_HANDLE = *mut *mut core::ffi::c_void;
#[cfg(all(feature = "evntprov", feature = "shared"))]
pub type PTRACE_EVENT_INFO = *mut TRACE_EVENT_INFO;
#[cfg(feature = "shared")]
pub type PTRACE_PROVIDER_INFO = *mut TRACE_PROVIDER_INFO;
pub const PropertyHasCustomSchema: PROPERTY_FLAGS = 128;
pub const PropertyHasTags: PROPERTY_FLAGS = 64;
pub const PropertyParamCount: PROPERTY_FLAGS = 4;
pub const PropertyParamFixedCount: PROPERTY_FLAGS = 32;
pub const PropertyParamFixedLength: PROPERTY_FLAGS = 16;
pub const PropertyParamLength: PROPERTY_FLAGS = 2;
pub const PropertyStruct: PROPERTY_FLAGS = 1;
pub const PropertyWBEMXmlFragment: PROPERTY_FLAGS = 8;
pub type TDHSTATUS = u32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TDH_CONTEXT {
    pub ParameterValue: super::ULONGLONG,
    pub ParameterType: TDH_CONTEXT_TYPE,
    pub ParameterSize: super::ULONG,
}
pub const TDH_CONTEXT_MAXIMUM: TDH_CONTEXT_TYPE = 5;
pub const TDH_CONTEXT_PDB_PATH: TDH_CONTEXT_TYPE = 4;
pub const TDH_CONTEXT_POINTERSIZE: TDH_CONTEXT_TYPE = 3;
pub type TDH_CONTEXT_TYPE = i32;
pub const TDH_CONTEXT_WPP_GMT: TDH_CONTEXT_TYPE = 2;
pub const TDH_CONTEXT_WPP_TMFFILE: TDH_CONTEXT_TYPE = 0;
pub const TDH_CONTEXT_WPP_TMFSEARCHPATH: TDH_CONTEXT_TYPE = 1;
pub type TDH_HANDLE = *mut core::ffi::c_void;
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
pub const TDH_OUTYTPE_ERRORCODE: _TDH_OUT_TYPE = 29;
pub const TEMPLATE_CONTROL_GUID: TEMPLATE_FLAGS = 4;
pub const TEMPLATE_EVENT_DATA: TEMPLATE_FLAGS = 1;
pub type TEMPLATE_FLAGS = i32;
pub const TEMPLATE_USER_DATA: TEMPLATE_FLAGS = 2;
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct TRACE_EVENT_INFO {
    pub ProviderGuid: windows_sys::core::GUID,
    pub EventGuid: windows_sys::core::GUID,
    pub EventDescriptor: super::EVENT_DESCRIPTOR,
    pub DecodingSource: DECODING_SOURCE,
    pub ProviderNameOffset: super::ULONG,
    pub LevelNameOffset: super::ULONG,
    pub ChannelNameOffset: super::ULONG,
    pub KeywordsNameOffset: super::ULONG,
    pub TaskNameOffset: super::ULONG,
    pub OpcodeNameOffset: super::ULONG,
    pub EventMessageOffset: super::ULONG,
    pub ProviderMessageOffset: super::ULONG,
    pub BinaryXMLOffset: super::ULONG,
    pub BinaryXMLSize: super::ULONG,
    pub Anonymous: TRACE_EVENT_INFO_0,
    pub Anonymous2: TRACE_EVENT_INFO_1,
    pub PropertyCount: super::ULONG,
    pub TopLevelPropertyCount: super::ULONG,
    pub Anonymous3: TRACE_EVENT_INFO_2,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
#[cfg(all(feature = "evntprov", feature = "shared"))]
impl Default for TRACE_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared"))]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_0 {
    pub EventNameOffset: super::ULONG,
    pub ActivityIDNameOffset: super::ULONG,
}
#[cfg(all(feature = "evntprov", feature = "shared"))]
impl Default for TRACE_EVENT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared"))]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_1 {
    pub EventAttributesOffset: super::ULONG,
    pub RelatedActivityIDNameOffset: super::ULONG,
}
#[cfg(all(feature = "evntprov", feature = "shared"))]
impl Default for TRACE_EVENT_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared"))]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_2 {
    pub Flags: TEMPLATE_FLAGS,
    pub Anonymous: TRACE_EVENT_INFO_2_0,
}
#[cfg(all(feature = "evntprov", feature = "shared"))]
impl Default for TRACE_EVENT_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct TRACE_EVENT_INFO_2_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TRACE_PROVIDER_INFO {
    pub ProviderGuid: windows_sys::core::GUID,
    pub SchemaSource: super::ULONG,
    pub ProviderNameOffset: super::ULONG,
}
pub type _TDH_IN_TYPE = i32;
pub type _TDH_OUT_TYPE = i32;
