#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn CollectionsListAllocateBufferAndSerialize(sourcecollection: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, ptargetbuffersizeinbytes: *mut u32, ptargetbuffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn CollectionsListCopyAndMarshall(target: *mut ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, source: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn CollectionsListDeserializeFromBuffer(sourcebuffersizeinbytes: u32, sourcebuffer: *const u8, targetcollection: *mut ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub fn CollectionsListGetFillableCount(buffersizebytes: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn CollectionsListGetMarshalledSize(collection: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> u32;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn CollectionsListGetMarshalledSizeWithoutSerialization(collection: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> u32;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn CollectionsListGetSerializedSize(collection: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> u32;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn CollectionsListMarshall(target: *mut ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn CollectionsListSerializeToBuffer(sourcecollection: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, targetbuffersizeinbytes: u32, targetbuffer: *mut u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn CollectionsListSortSubscribedActivitiesByConfidence(thresholds: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pcollection: *mut ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn CollectionsListUpdateMarshalledPointer(collection: *mut ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn EvaluateActivityThresholds(newsample: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, oldsample: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, thresholds: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPerformanceTime(timems: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromCLSIDArray(members: *const ::windows::runtime::GUID, size: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromFloat(fltval: f32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn IsCollectionListSame(lista: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, listb: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsGUIDPresentInList(guidarray: *const ::windows::runtime::GUID, arraylength: u32, guidelem: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn IsKeyPresentInCollectionList(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn IsKeyPresentInPropertyList(plist: *const SENSOR_PROPERTY_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn IsSensorSubscribed(subscriptionlist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, currenttype: ::windows::runtime::GUID) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetBool(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::BOOL) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetDouble(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f64) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetFileTime(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetFloat(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetGuid(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut ::windows::runtime::GUID) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetInt32(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetInt64(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i64) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetNthInt64(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut i64) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetNthUlong(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetNthUshort(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u16) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetPropVariant(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, typecheck: super::super::Foundation::BOOLEAN, pvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetUlong(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeyGetUshort(plist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u16) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropKeyFindKeySetPropVariant(plist: *mut ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, typecheck: super::super::Foundation::BOOLEAN, pvalue: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetInformation(propvariantvalue: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, propvariantoffset: *mut u32, propvariantsize: *mut u32, propvariantpointer: *mut *mut ::core::ffi::c_void, remappedtype: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn PropertiesListCopy(target: *mut SENSOR_PROPERTY_LIST, source: *const SENSOR_PROPERTY_LIST) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub fn PropertiesListGetFillableCount(buffersizebytes: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn SensorCollectionGetAt(index: u32, psensorslist: *const ::core::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SerializationBufferAllocate(sizeinbytes: u32, pbuffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub fn SerializationBufferFree(buffer: *const u8);
}
