#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ACTIVITY_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ActivityState_Unknown: ACTIVITY_STATE = ACTIVITY_STATE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ActivityState_Stationary: ACTIVITY_STATE = ACTIVITY_STATE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ActivityState_Fidgeting: ACTIVITY_STATE = ACTIVITY_STATE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ActivityState_Walking: ACTIVITY_STATE = ACTIVITY_STATE(8i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ActivityState_Running: ACTIVITY_STATE = ACTIVITY_STATE(16i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ActivityState_InVehicle: ACTIVITY_STATE = ACTIVITY_STATE(32i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ActivityState_Biking: ACTIVITY_STATE = ACTIVITY_STATE(64i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ActivityState_Idle: ACTIVITY_STATE = ACTIVITY_STATE(128i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ActivityState_Max: ACTIVITY_STATE = ACTIVITY_STATE(256i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ActivityState_Force_Dword: ACTIVITY_STATE = ACTIVITY_STATE(-1i32);
impl ::core::marker::Copy for ACTIVITY_STATE {}
impl ::core::clone::Clone for ACTIVITY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTIVITY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ACTIVITY_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACTIVITY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVITY_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ACTIVITY_STATE_COUNT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ActivityStateCount: ACTIVITY_STATE_COUNT = ACTIVITY_STATE_COUNT(8i32);
impl ::core::marker::Copy for ACTIVITY_STATE_COUNT {}
impl ::core::clone::Clone for ACTIVITY_STATE_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTIVITY_STATE_COUNT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ACTIVITY_STATE_COUNT {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACTIVITY_STATE_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVITY_STATE_COUNT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AXIS(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const AXIS_X: AXIS = AXIS(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const AXIS_Y: AXIS = AXIS(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const AXIS_Z: AXIS = AXIS(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const AXIS_MAX: AXIS = AXIS(3i32);
impl ::core::marker::Copy for AXIS {}
impl ::core::clone::Clone for AXIS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AXIS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AXIS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AXIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AXIS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn CollectionsListAllocateBufferAndSerialize(sourcecollection: *const SENSOR_COLLECTION_LIST, ptargetbuffersizeinbytes: *mut u32, ptargetbuffer: *mut *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListAllocateBufferAndSerialize(sourcecollection: *const SENSOR_COLLECTION_LIST, ptargetbuffersizeinbytes: *mut u32, ptargetbuffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListAllocateBufferAndSerialize(::core::mem::transmute(sourcecollection), ::core::mem::transmute(ptargetbuffersizeinbytes), ::core::mem::transmute(ptargetbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn CollectionsListCopyAndMarshall(target: *mut SENSOR_COLLECTION_LIST, source: *const SENSOR_COLLECTION_LIST) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListCopyAndMarshall(target: *mut SENSOR_COLLECTION_LIST, source: *const SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListCopyAndMarshall(::core::mem::transmute(target), ::core::mem::transmute(source)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn CollectionsListDeserializeFromBuffer(sourcebuffersizeinbytes: u32, sourcebuffer: *const u8, targetcollection: *mut SENSOR_COLLECTION_LIST) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListDeserializeFromBuffer(sourcebuffersizeinbytes: u32, sourcebuffer: *const u8, targetcollection: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListDeserializeFromBuffer(::core::mem::transmute(sourcebuffersizeinbytes), ::core::mem::transmute(sourcebuffer), ::core::mem::transmute(targetcollection)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[inline]
pub unsafe fn CollectionsListGetFillableCount(buffersizebytes: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListGetFillableCount(buffersizebytes: u32) -> u32;
        }
        ::core::mem::transmute(CollectionsListGetFillableCount(::core::mem::transmute(buffersizebytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn CollectionsListGetMarshalledSize(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListGetMarshalledSize(collection: *const SENSOR_COLLECTION_LIST) -> u32;
        }
        ::core::mem::transmute(CollectionsListGetMarshalledSize(::core::mem::transmute(collection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn CollectionsListGetMarshalledSizeWithoutSerialization(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListGetMarshalledSizeWithoutSerialization(collection: *const SENSOR_COLLECTION_LIST) -> u32;
        }
        ::core::mem::transmute(CollectionsListGetMarshalledSizeWithoutSerialization(::core::mem::transmute(collection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn CollectionsListGetSerializedSize(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListGetSerializedSize(collection: *const SENSOR_COLLECTION_LIST) -> u32;
        }
        ::core::mem::transmute(CollectionsListGetSerializedSize(::core::mem::transmute(collection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn CollectionsListMarshall(target: *mut SENSOR_COLLECTION_LIST) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListMarshall(target: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListMarshall(::core::mem::transmute(target)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn CollectionsListSerializeToBuffer(sourcecollection: *const SENSOR_COLLECTION_LIST, targetbuffersizeinbytes: u32, targetbuffer: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListSerializeToBuffer(sourcecollection: *const SENSOR_COLLECTION_LIST, targetbuffersizeinbytes: u32, targetbuffer: *mut u8) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListSerializeToBuffer(::core::mem::transmute(sourcecollection), ::core::mem::transmute(targetbuffersizeinbytes), ::core::mem::transmute(targetbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn CollectionsListSortSubscribedActivitiesByConfidence(thresholds: *const SENSOR_COLLECTION_LIST, pcollection: *mut SENSOR_COLLECTION_LIST) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListSortSubscribedActivitiesByConfidence(thresholds: *const SENSOR_COLLECTION_LIST, pcollection: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListSortSubscribedActivitiesByConfidence(::core::mem::transmute(thresholds), ::core::mem::transmute(pcollection)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn CollectionsListUpdateMarshalledPointer(collection: *mut SENSOR_COLLECTION_LIST) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListUpdateMarshalledPointer(collection: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListUpdateMarshalledPointer(::core::mem::transmute(collection)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ELEVATION_CHANGE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ElevationChangeMode_Unknown: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ElevationChangeMode_Elevator: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ElevationChangeMode_Stepping: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ElevationChangeMode_Max: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ElevationChangeMode_Force_Dword: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(-1i32);
impl ::core::marker::Copy for ELEVATION_CHANGE_MODE {}
impl ::core::clone::Clone for ELEVATION_CHANGE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ELEVATION_CHANGE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ELEVATION_CHANGE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ELEVATION_CHANGE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ELEVATION_CHANGE_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn EvaluateActivityThresholds(newsample: *const SENSOR_COLLECTION_LIST, oldsample: *const SENSOR_COLLECTION_LIST, thresholds: *const SENSOR_COLLECTION_LIST) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvaluateActivityThresholds(newsample: *const SENSOR_COLLECTION_LIST, oldsample: *const SENSOR_COLLECTION_LIST, thresholds: *const SENSOR_COLLECTION_LIST) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(EvaluateActivityThresholds(::core::mem::transmute(newsample), ::core::mem::transmute(oldsample), ::core::mem::transmute(thresholds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const GNSS_CLEAR_ALL_ASSISTANCE_DATA: u32 = 1u32;
pub const GUID_DEVINTERFACE_SENSOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba1bb692_9b7a_4833_9a1e_525ed134e7e2);
pub const GUID_SensorCategory_All: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc317c286_c468_4288_9975_d4c4587c442c);
pub const GUID_SensorCategory_Biometric: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca19690f_a2c7_477d_a99e_99ec6e2b5648);
pub const GUID_SensorCategory_Electrical: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb73fcd8_fc4a_483c_ac58_27b691c6beff);
pub const GUID_SensorCategory_Environmental: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x323439aa_7f66_492b_ba0c_73e9aa0a65d5);
pub const GUID_SensorCategory_Light: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17a665c0_9063_4216_b202_5c7a255e18ce);
pub const GUID_SensorCategory_Location: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfa794e4_f964_4fdb_90f6_51056bfe4b44);
pub const GUID_SensorCategory_Mechanical: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d131d68_8ef7_4656_80b5_cccbd93791c5);
pub const GUID_SensorCategory_Motion: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd09daf1_3b2e_4c3d_b598_b5e5ff93fd46);
pub const GUID_SensorCategory_Orientation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e6c04b6_96fe_4954_b726_68682a473f69);
pub const GUID_SensorCategory_Other: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c90e7a9_f4c9_4fa2_af37_56d471fe5a3d);
pub const GUID_SensorCategory_PersonalActivity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1609081_1e12_412b_a14d_cbb0e95bd2e5);
pub const GUID_SensorCategory_Scanner: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb000e77e_f5b5_420f_815d_0270a726f270);
pub const GUID_SensorCategory_Unsupported: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2beae7fa_19b0_48c5_a1f6_b5480dc206b0);
pub const GUID_SensorType_Accelerometer3D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2fb0f5f_e2d2_4c78_bcd0_352a9582819d);
pub const GUID_SensorType_ActivityDetection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d9e0118_1807_4f2e_96e4_2ce57142e196);
pub const GUID_SensorType_AmbientLight: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97f115c8_599a_4153_8894_d2d12899918a);
pub const GUID_SensorType_Barometer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e903829_ff8a_4a93_97df_3dcbde402288);
pub const GUID_SensorType_Custom: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe83af229_8640_4d18_a213_e22675ebb2c3);
pub const GUID_SensorType_FloorElevation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xade4987f_7ac4_4dfa_9722_0a027181c747);
pub const GUID_SensorType_GeomagneticOrientation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe77195f8_2d1f_4823_971b_1c4467556c9d);
pub const GUID_SensorType_GravityVector: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03b52c73_bb76_463f_9524_38de76eb700b);
pub const GUID_SensorType_Gyrometer3D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09485f5a_759e_42c2_bd4b_a349b75c8643);
pub const GUID_SensorType_HingeAngle: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82358065_f4c4_4da1_b272_13c23332a207);
pub const GUID_SensorType_Humidity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c72bf67_bd7e_4257_990b_98a3ba3b400a);
pub const GUID_SensorType_LinearAccelerometer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x038b0283_97b4_41c8_bc24_5ff1aa48fec7);
pub const GUID_SensorType_Magnetometer3D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55e5effb_15c7_40df_8698_a84b7c863c53);
pub const GUID_SensorType_Orientation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdb5d8f7_3cfd_41c8_8542_cce622cf5d6e);
pub const GUID_SensorType_Pedometer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb19f89af_e3eb_444b_8dea_202575a71599);
pub const GUID_SensorType_Proximity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5220dae9_3179_4430_9f90_06266d2a34de);
pub const GUID_SensorType_RelativeOrientation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40993b51_4706_44dc_98d5_c920c037ffab);
pub const GUID_SensorType_SimpleDeviceOrientation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86a19291_0482_402c_bf4c_addac52b1c39);
pub const GUID_SensorType_Temperature: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04fd0ec4_d5da_45fa_95a9_5db38ee19306);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPerformanceTime(timems: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPerformanceTime(timems: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        GetPerformanceTime(::core::mem::transmute(timems)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HUMAN_PRESENCE_DETECTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const HumanPresenceDetectionType_VendorDefinedNonBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const HumanPresenceDetectionType_VendorDefinedBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const HumanPresenceDetectionType_FacialBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const HumanPresenceDetectionType_AudioBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const HumanPresenceDetectionType_Force_Dword: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(-1i32);
impl ::core::marker::Copy for HUMAN_PRESENCE_DETECTION_TYPE {}
impl ::core::clone::Clone for HUMAN_PRESENCE_DETECTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HUMAN_PRESENCE_DETECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HUMAN_PRESENCE_DETECTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HUMAN_PRESENCE_DETECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HUMAN_PRESENCE_DETECTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HUMAN_PRESENCE_DETECTION_TYPE_COUNT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const HumanPresenceDetectionTypeCount: HUMAN_PRESENCE_DETECTION_TYPE_COUNT = HUMAN_PRESENCE_DETECTION_TYPE_COUNT(4i32);
impl ::core::marker::Copy for HUMAN_PRESENCE_DETECTION_TYPE_COUNT {}
impl ::core::clone::Clone for HUMAN_PRESENCE_DETECTION_TYPE_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HUMAN_PRESENCE_DETECTION_TYPE_COUNT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HUMAN_PRESENCE_DETECTION_TYPE_COUNT {
    type Abi = Self;
}
impl ::core::fmt::Debug for HUMAN_PRESENCE_DETECTION_TYPE_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HUMAN_PRESENCE_DETECTION_TYPE_COUNT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ILocationPermissions(::windows::core::IUnknown);
impl ILocationPermissions {
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlobalLocationPermission(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGlobalLocationPermission)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn CheckLocationCapability(&self, dwclientthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckLocationCapability)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwclientthreadid)).ok()
    }
}
impl ::core::convert::From<ILocationPermissions> for ::windows::core::IUnknown {
    fn from(value: ILocationPermissions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILocationPermissions> for ::windows::core::IUnknown {
    fn from(value: &ILocationPermissions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILocationPermissions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILocationPermissions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILocationPermissions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILocationPermissions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILocationPermissions {}
impl ::core::fmt::Debug for ILocationPermissions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILocationPermissions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILocationPermissions {
    type Vtable = ILocationPermissions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5fb0a7f_e74e_44f5_8e02_4806863a274f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationPermissions_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGlobalLocationPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGlobalLocationPermission: usize,
    pub CheckLocationCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclientthreadid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ISensor(::windows::core::IUnknown);
impl ISensor {
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn GetID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn GetCategory(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCategory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFriendlyName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Devices_PortableDevices\"`*"]
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn GetProperties<'a, Param0: ::windows::core::IntoParam<'a, super::PortableDevices::IPortableDeviceKeyCollection>>(&self, pkeys: Param0) -> ::windows::core::Result<super::PortableDevices::IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProperties)(::core::mem::transmute_copy(self), pkeys.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::PortableDevices::IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Devices_PortableDevices\"`*"]
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn GetSupportedDataFields(&self) -> ::windows::core::Result<super::PortableDevices::IPortableDeviceKeyCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSupportedDataFields)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::PortableDevices::IPortableDeviceKeyCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Devices_PortableDevices\"`*"]
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn SetProperties<'a, Param0: ::windows::core::IntoParam<'a, super::PortableDevices::IPortableDeviceValues>>(&self, pproperties: Param0) -> ::windows::core::Result<super::PortableDevices::IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetProperties)(::core::mem::transmute_copy(self), pproperties.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::PortableDevices::IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SupportsDataField(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SupportsDataField)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn GetState(&self) -> ::windows::core::Result<SensorState> {
        let mut result__: SensorState = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<SensorState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn GetData(&self) -> ::windows::core::Result<ISensorDataReport> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISensorDataReport>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn SupportsEvent(&self, eventguid: *const ::windows::core::GUID) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SupportsEvent)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventguid), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn GetEventInterest(&self, ppvalues: *mut *mut ::windows::core::GUID, pcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEventInterest)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppvalues), ::core::mem::transmute(pcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn SetEventInterest(&self, pvalues: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventInterest)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalues)), pvalues.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn SetEventSink<'a, Param0: ::windows::core::IntoParam<'a, ISensorEvents>>(&self, pevents: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventSink)(::core::mem::transmute_copy(self), pevents.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISensor> for ::windows::core::IUnknown {
    fn from(value: ISensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISensor> for ::windows::core::IUnknown {
    fn from(value: &ISensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensor {}
impl ::core::fmt::Debug for ISensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISensor {
    type Vtable = ISensor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fa08f80_2657_458e_af75_46f73fa6ac5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensor_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensorcategory: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensortype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFriendlyName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pproperty: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkeys: ::windows::core::RawPtr, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    GetProperties: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub GetSupportedDataFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdatafields: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    GetSupportedDataFields: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub SetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproperties: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    SetProperties: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SupportsDataField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pissupported: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SupportsDataField: usize,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut SensorState) -> ::windows::core::HRESULT,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdatareport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SupportsEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventguid: *const ::windows::core::GUID, pissupported: *mut i16) -> ::windows::core::HRESULT,
    pub GetEventInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvalues: *mut *mut ::windows::core::GUID, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub SetEventInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: *const ::windows::core::GUID, count: u32) -> ::windows::core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ISensorCollection(::windows::core::IUnknown);
impl ISensorCollection {
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn GetAt(&self, ulindex: u32) -> ::windows::core::Result<ISensor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(&mut result__)).from_abi::<ISensor>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, ISensor>>(&self, psensor: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), psensor.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ISensor>>(&self, psensor: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), psensor.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn RemoveByID(&self, sensorid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveByID)(::core::mem::transmute_copy(self), ::core::mem::transmute(sensorid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISensorCollection> for ::windows::core::IUnknown {
    fn from(value: ISensorCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISensorCollection> for ::windows::core::IUnknown {
    fn from(value: &ISensorCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISensorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISensorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISensorCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISensorCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorCollection {}
impl ::core::fmt::Debug for ISensorCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISensorCollection {
    type Vtable = ISensorCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23571e11_e545_4dd8_a337_b89bf44b10df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ppsensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ISensorDataReport(::windows::core::IUnknown);
impl ISensorDataReport {
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTimestamp)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetSensorValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSensorValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(pkey), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Devices_PortableDevices\"`*"]
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn GetSensorValues<'a, Param0: ::windows::core::IntoParam<'a, super::PortableDevices::IPortableDeviceKeyCollection>>(&self, pkeys: Param0) -> ::windows::core::Result<super::PortableDevices::IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSensorValues)(::core::mem::transmute_copy(self), pkeys.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::PortableDevices::IPortableDeviceValues>(result__)
    }
}
impl ::core::convert::From<ISensorDataReport> for ::windows::core::IUnknown {
    fn from(value: ISensorDataReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISensorDataReport> for ::windows::core::IUnknown {
    fn from(value: &ISensorDataReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISensorDataReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISensorDataReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISensorDataReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISensorDataReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorDataReport {}
impl ::core::fmt::Debug for ISensorDataReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorDataReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISensorDataReport {
    type Vtable = ISensorDataReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ab9df9b_c4b5_4796_8898_0470706a2e1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataReport_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptimestamp: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTimestamp: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetSensorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetSensorValue: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub GetSensorValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkeys: ::windows::core::RawPtr, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    GetSensorValues: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ISensorEvents(::windows::core::IUnknown);
impl ISensorEvents {
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn OnStateChanged<'a, Param0: ::windows::core::IntoParam<'a, ISensor>>(&self, psensor: Param0, state: SensorState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStateChanged)(::core::mem::transmute_copy(self), psensor.into_param().abi(), ::core::mem::transmute(state)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn OnDataUpdated<'a, Param0: ::windows::core::IntoParam<'a, ISensor>, Param1: ::windows::core::IntoParam<'a, ISensorDataReport>>(&self, psensor: Param0, pnewdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDataUpdated)(::core::mem::transmute_copy(self), psensor.into_param().abi(), pnewdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Devices_PortableDevices\"`*"]
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn OnEvent<'a, Param0: ::windows::core::IntoParam<'a, ISensor>, Param2: ::windows::core::IntoParam<'a, super::PortableDevices::IPortableDeviceValues>>(&self, psensor: Param0, eventid: *const ::windows::core::GUID, peventdata: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEvent)(::core::mem::transmute_copy(self), psensor.into_param().abi(), ::core::mem::transmute(eventid), peventdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn OnLeave(&self, id: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLeave)(::core::mem::transmute_copy(self), ::core::mem::transmute(id)).ok()
    }
}
impl ::core::convert::From<ISensorEvents> for ::windows::core::IUnknown {
    fn from(value: ISensorEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISensorEvents> for ::windows::core::IUnknown {
    fn from(value: &ISensorEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISensorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISensorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISensorEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISensorEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorEvents {}
impl ::core::fmt::Debug for ISensorEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISensorEvents {
    type Vtable = ISensorEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d8dcc91_4641_47e7_b7c3_b74f48a6c391);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, state: SensorState) -> ::windows::core::HRESULT,
    pub OnDataUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, pnewdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub OnEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, eventid: *const ::windows::core::GUID, peventdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    OnEvent: usize,
    pub OnLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ISensorManager(::windows::core::IUnknown);
impl ISensorManager {
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn GetSensorsByCategory(&self, sensorcategory: *const ::windows::core::GUID) -> ::windows::core::Result<ISensorCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSensorsByCategory)(::core::mem::transmute_copy(self), ::core::mem::transmute(sensorcategory), ::core::mem::transmute(&mut result__)).from_abi::<ISensorCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn GetSensorsByType(&self, sensortype: *const ::windows::core::GUID) -> ::windows::core::Result<ISensorCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSensorsByType)(::core::mem::transmute_copy(self), ::core::mem::transmute(sensortype), ::core::mem::transmute(&mut result__)).from_abi::<ISensorCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn GetSensorByID(&self, sensorid: *const ::windows::core::GUID) -> ::windows::core::Result<ISensor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSensorByID)(::core::mem::transmute_copy(self), ::core::mem::transmute(sensorid), ::core::mem::transmute(&mut result__)).from_abi::<ISensor>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn SetEventSink<'a, Param0: ::windows::core::IntoParam<'a, ISensorManagerEvents>>(&self, pevents: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventSink)(::core::mem::transmute_copy(self), pevents.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestPermissions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ISensorCollection>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hparent: Param0, psensors: Param1, fmodal: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestPermissions)(::core::mem::transmute_copy(self), hparent.into_param().abi(), psensors.into_param().abi(), fmodal.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISensorManager> for ::windows::core::IUnknown {
    fn from(value: ISensorManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISensorManager> for ::windows::core::IUnknown {
    fn from(value: &ISensorManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISensorManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISensorManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISensorManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISensorManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorManager {}
impl ::core::fmt::Debug for ISensorManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISensorManager {
    type Vtable = ISensorManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd77db67_45a8_42dc_8d00_6dcf15f8377a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetSensorsByCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorcategory: *const ::windows::core::GUID, ppsensorsfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSensorsByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensortype: *const ::windows::core::GUID, ppsensorsfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSensorByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorid: *const ::windows::core::GUID, ppsensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestPermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, psensors: ::windows::core::RawPtr, fmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestPermissions: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ISensorManagerEvents(::windows::core::IUnknown);
impl ISensorManagerEvents {
    #[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
    pub unsafe fn OnSensorEnter<'a, Param0: ::windows::core::IntoParam<'a, ISensor>>(&self, psensor: Param0, state: SensorState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSensorEnter)(::core::mem::transmute_copy(self), psensor.into_param().abi(), ::core::mem::transmute(state)).ok()
    }
}
impl ::core::convert::From<ISensorManagerEvents> for ::windows::core::IUnknown {
    fn from(value: ISensorManagerEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISensorManagerEvents> for ::windows::core::IUnknown {
    fn from(value: &ISensorManagerEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISensorManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISensorManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISensorManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISensorManagerEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorManagerEvents {}
impl ::core::fmt::Debug for ISensorManagerEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorManagerEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISensorManagerEvents {
    type Vtable = ISensorManagerEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b3b0b86_266a_4aad_b21f_fde5501001b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorManagerEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnSensorEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, state: SensorState) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromCLSIDArray(members: &[::windows::core::GUID]) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromCLSIDArray(members: *const ::windows::core::GUID, size: u32, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT;
        }
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        InitPropVariantFromCLSIDArray(::core::mem::transmute(::windows::core::as_ptr_or_null(members)), members.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromFloat(fltval: f32) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromFloat(fltval: f32, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT;
        }
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        InitPropVariantFromFloat(::core::mem::transmute(fltval), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn IsCollectionListSame(lista: *const SENSOR_COLLECTION_LIST, listb: *const SENSOR_COLLECTION_LIST) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsCollectionListSame(lista: *const SENSOR_COLLECTION_LIST, listb: *const SENSOR_COLLECTION_LIST) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(IsCollectionListSame(::core::mem::transmute(lista), ::core::mem::transmute(listb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsGUIDPresentInList(guidarray: &[::windows::core::GUID], guidelem: *const ::windows::core::GUID) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsGUIDPresentInList(guidarray: *const ::windows::core::GUID, arraylength: u32, guidelem: *const ::windows::core::GUID) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(IsGUIDPresentInList(::core::mem::transmute(::windows::core::as_ptr_or_null(guidarray)), guidarray.len() as _, ::core::mem::transmute(guidelem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn IsKeyPresentInCollectionList(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsKeyPresentInCollectionList(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(IsKeyPresentInCollectionList(::core::mem::transmute(plist), ::core::mem::transmute(pkey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn IsKeyPresentInPropertyList(plist: *const SENSOR_PROPERTY_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsKeyPresentInPropertyList(plist: *const SENSOR_PROPERTY_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(IsKeyPresentInPropertyList(::core::mem::transmute(plist), ::core::mem::transmute(pkey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn IsSensorSubscribed<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(subscriptionlist: *const SENSOR_COLLECTION_LIST, currenttype: Param1) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsSensorSubscribed(subscriptionlist: *const SENSOR_COLLECTION_LIST, currenttype: ::windows::core::GUID) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(IsSensorSubscribed(::core::mem::transmute(subscriptionlist), currenttype.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LOCATION_DESIRED_ACCURACY(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const LOCATION_DESIRED_ACCURACY_DEFAULT: LOCATION_DESIRED_ACCURACY = LOCATION_DESIRED_ACCURACY(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const LOCATION_DESIRED_ACCURACY_HIGH: LOCATION_DESIRED_ACCURACY = LOCATION_DESIRED_ACCURACY(1i32);
impl ::core::marker::Copy for LOCATION_DESIRED_ACCURACY {}
impl ::core::clone::Clone for LOCATION_DESIRED_ACCURACY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOCATION_DESIRED_ACCURACY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LOCATION_DESIRED_ACCURACY {
    type Abi = Self;
}
impl ::core::fmt::Debug for LOCATION_DESIRED_ACCURACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCATION_DESIRED_ACCURACY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LOCATION_POSITION_SOURCE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const LOCATION_POSITION_SOURCE_CELLULAR: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const LOCATION_POSITION_SOURCE_SATELLITE: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const LOCATION_POSITION_SOURCE_WIFI: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const LOCATION_POSITION_SOURCE_IPADDRESS: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const LOCATION_POSITION_SOURCE_UNKNOWN: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(4i32);
impl ::core::marker::Copy for LOCATION_POSITION_SOURCE {}
impl ::core::clone::Clone for LOCATION_POSITION_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOCATION_POSITION_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LOCATION_POSITION_SOURCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for LOCATION_POSITION_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCATION_POSITION_SOURCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MAGNETOMETER_ACCURACY(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const MagnetometerAccuracy_Unknown: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const MagnetometerAccuracy_Unreliable: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const MagnetometerAccuracy_Approximate: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const MagnetometerAccuracy_High: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(3i32);
impl ::core::marker::Copy for MAGNETOMETER_ACCURACY {}
impl ::core::clone::Clone for MAGNETOMETER_ACCURACY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MAGNETOMETER_ACCURACY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MAGNETOMETER_ACCURACY {
    type Abi = Self;
}
impl ::core::fmt::Debug for MAGNETOMETER_ACCURACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MAGNETOMETER_ACCURACY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub struct MATRIX3X3 {
    pub Anonymous: MATRIX3X3_0,
}
impl ::core::marker::Copy for MATRIX3X3 {}
impl ::core::clone::Clone for MATRIX3X3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MATRIX3X3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MATRIX3X3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MATRIX3X3>()) == 0 }
    }
}
impl ::core::cmp::Eq for MATRIX3X3 {}
impl ::core::default::Default for MATRIX3X3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub union MATRIX3X3_0 {
    pub Anonymous1: MATRIX3X3_0_0,
    pub Anonymous2: MATRIX3X3_0_1,
    pub M: [f32; 9],
}
impl ::core::marker::Copy for MATRIX3X3_0 {}
impl ::core::clone::Clone for MATRIX3X3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MATRIX3X3_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MATRIX3X3_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MATRIX3X3_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MATRIX3X3_0 {}
impl ::core::default::Default for MATRIX3X3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub struct MATRIX3X3_0_0 {
    pub A11: f32,
    pub A12: f32,
    pub A13: f32,
    pub A21: f32,
    pub A22: f32,
    pub A23: f32,
    pub A31: f32,
    pub A32: f32,
    pub A33: f32,
}
impl ::core::marker::Copy for MATRIX3X3_0_0 {}
impl ::core::clone::Clone for MATRIX3X3_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MATRIX3X3_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MATRIX3X3_0_0").field("A11", &self.A11).field("A12", &self.A12).field("A13", &self.A13).field("A21", &self.A21).field("A22", &self.A22).field("A23", &self.A23).field("A31", &self.A31).field("A32", &self.A32).field("A33", &self.A33).finish()
    }
}
unsafe impl ::windows::core::Abi for MATRIX3X3_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MATRIX3X3_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MATRIX3X3_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MATRIX3X3_0_0 {}
impl ::core::default::Default for MATRIX3X3_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub struct MATRIX3X3_0_1 {
    pub V1: VEC3D,
    pub V2: VEC3D,
    pub V3: VEC3D,
}
impl ::core::marker::Copy for MATRIX3X3_0_1 {}
impl ::core::clone::Clone for MATRIX3X3_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MATRIX3X3_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MATRIX3X3_0_1").field("V1", &self.V1).field("V2", &self.V2).field("V3", &self.V3).finish()
    }
}
unsafe impl ::windows::core::Abi for MATRIX3X3_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MATRIX3X3_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MATRIX3X3_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MATRIX3X3_0_1 {}
impl ::core::default::Default for MATRIX3X3_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagnetometerAccuracy(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const MAGNETOMETER_ACCURACY_UNKNOWN: MagnetometerAccuracy = MagnetometerAccuracy(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const MAGNETOMETER_ACCURACY_UNRELIABLE: MagnetometerAccuracy = MagnetometerAccuracy(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const MAGNETOMETER_ACCURACY_APPROXIMATE: MagnetometerAccuracy = MagnetometerAccuracy(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const MAGNETOMETER_ACCURACY_HIGH: MagnetometerAccuracy = MagnetometerAccuracy(3i32);
impl ::core::marker::Copy for MagnetometerAccuracy {}
impl ::core::clone::Clone for MagnetometerAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagnetometerAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagnetometerAccuracy {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagnetometerAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagnetometerAccuracy").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PEDOMETER_STEP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const PedometerStepType_Unknown: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const PedometerStepType_Walking: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const PedometerStepType_Running: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const PedometerStepType_Max: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const PedometerStepType_Force_Dword: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(-1i32);
impl ::core::marker::Copy for PEDOMETER_STEP_TYPE {}
impl ::core::clone::Clone for PEDOMETER_STEP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEDOMETER_STEP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PEDOMETER_STEP_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PEDOMETER_STEP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEDOMETER_STEP_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PEDOMETER_STEP_TYPE_COUNT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const PedometerStepTypeCount: PEDOMETER_STEP_TYPE_COUNT = PEDOMETER_STEP_TYPE_COUNT(3i32);
impl ::core::marker::Copy for PEDOMETER_STEP_TYPE_COUNT {}
impl ::core::clone::Clone for PEDOMETER_STEP_TYPE_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEDOMETER_STEP_TYPE_COUNT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PEDOMETER_STEP_TYPE_COUNT {
    type Abi = Self;
}
impl ::core::fmt::Debug for PEDOMETER_STEP_TYPE_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEDOMETER_STEP_TYPE_COUNT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROXIMITY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ProximityType_ObjectProximity: PROXIMITY_TYPE = PROXIMITY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ProximityType_HumanProximity: PROXIMITY_TYPE = PROXIMITY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const ProximityType_Force_Dword: PROXIMITY_TYPE = PROXIMITY_TYPE(-1i32);
impl ::core::marker::Copy for PROXIMITY_TYPE {}
impl ::core::clone::Clone for PROXIMITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROXIMITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROXIMITY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROXIMITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROXIMITY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetBool(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetBool(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::BOOL) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetBool(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetDouble(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetDouble(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f64) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetDouble(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetFileTime(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetFileTime(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetFileTime(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetFloat(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetFloat(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f32) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetFloat(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetGuid(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetGuid(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut ::windows::core::GUID) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetGuid(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetInt32(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetInt32(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i32) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetInt32(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetInt64(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetInt64(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i64) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetInt64(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthInt64(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut i64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetNthInt64(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut i64) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetNthInt64(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(occurrence), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthUlong(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetNthUlong(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetNthUlong(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(occurrence), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthUshort(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetNthUshort(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u16) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetNthUshort(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(occurrence), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetPropVariant<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, typecheck: Param2, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetPropVariant(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, typecheck: super::super::Foundation::BOOLEAN, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetPropVariant(::core::mem::transmute(plist), ::core::mem::transmute(pkey), typecheck.into_param().abi(), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetUlong(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetUlong(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetUlong(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetUshort(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetUshort(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u16) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetUshort(::core::mem::transmute(plist), ::core::mem::transmute(pkey), ::core::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropKeyFindKeySetPropVariant<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(plist: *mut SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, typecheck: Param2, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeySetPropVariant(plist: *mut SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, typecheck: super::super::Foundation::BOOLEAN, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeySetPropVariant(::core::mem::transmute(plist), ::core::mem::transmute(pkey), typecheck.into_param().abi(), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetInformation(propvariantvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propvariantoffset: *mut u32, propvariantsize: *mut u32, propvariantpointer: *mut *mut ::core::ffi::c_void, remappedtype: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetInformation(propvariantvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propvariantoffset: *mut u32, propvariantsize: *mut u32, propvariantpointer: *mut *mut ::core::ffi::c_void, remappedtype: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        PropVariantGetInformation(::core::mem::transmute(propvariantvalue), ::core::mem::transmute(propvariantoffset), ::core::mem::transmute(propvariantsize), ::core::mem::transmute(propvariantpointer), ::core::mem::transmute(remappedtype)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn PropertiesListCopy(target: *mut SENSOR_PROPERTY_LIST, source: *const SENSOR_PROPERTY_LIST) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropertiesListCopy(target: *mut SENSOR_PROPERTY_LIST, source: *const SENSOR_PROPERTY_LIST) -> super::super::Foundation::NTSTATUS;
        }
        PropertiesListCopy(::core::mem::transmute(target), ::core::mem::transmute(source)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[inline]
pub unsafe fn PropertiesListGetFillableCount(buffersizebytes: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropertiesListGetFillableCount(buffersizebytes: u32) -> u32;
        }
        ::core::mem::transmute(PropertiesListGetFillableCount(::core::mem::transmute(buffersizebytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub struct QUATERNION {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}
impl ::core::marker::Copy for QUATERNION {}
impl ::core::clone::Clone for QUATERNION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUATERNION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUATERNION").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).field("W", &self.W).finish()
    }
}
unsafe impl ::windows::core::Abi for QUATERNION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUATERNION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUATERNION>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUATERNION {}
impl ::core::default::Default for QUATERNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SENSOR_CATEGORY_ALL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc317c286_c468_4288_9975_d4c4587c442c);
pub const SENSOR_CATEGORY_BIOMETRIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca19690f_a2c7_477d_a99e_99ec6e2b5648);
pub const SENSOR_CATEGORY_ELECTRICAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb73fcd8_fc4a_483c_ac58_27b691c6beff);
pub const SENSOR_CATEGORY_ENVIRONMENTAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x323439aa_7f66_492b_ba0c_73e9aa0a65d5);
pub const SENSOR_CATEGORY_LIGHT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17a665c0_9063_4216_b202_5c7a255e18ce);
pub const SENSOR_CATEGORY_LOCATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfa794e4_f964_4fdb_90f6_51056bfe4b44);
pub const SENSOR_CATEGORY_MECHANICAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d131d68_8ef7_4656_80b5_cccbd93791c5);
pub const SENSOR_CATEGORY_MOTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd09daf1_3b2e_4c3d_b598_b5e5ff93fd46);
pub const SENSOR_CATEGORY_ORIENTATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e6c04b6_96fe_4954_b726_68682a473f69);
pub const SENSOR_CATEGORY_OTHER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c90e7a9_f4c9_4fa2_af37_56d471fe5a3d);
pub const SENSOR_CATEGORY_SCANNER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb000e77e_f5b5_420f_815d_0270a726f270);
pub const SENSOR_CATEGORY_UNSUPPORTED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2beae7fa_19b0_48c5_a1f6_b5480dc206b0);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct SENSOR_COLLECTION_LIST {
    pub AllocatedSizeInBytes: u32,
    pub Count: u32,
    pub List: [SENSOR_VALUE_PAIR; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for SENSOR_COLLECTION_LIST {
    fn clone(&self) -> Self {
        Self { AllocatedSizeInBytes: self.AllocatedSizeInBytes, Count: self.Count, List: self.List.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
unsafe impl ::windows::core::Abi for SENSOR_COLLECTION_LIST {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for SENSOR_COLLECTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.AllocatedSizeInBytes == other.AllocatedSizeInBytes && self.Count == other.Count && self.List == other.List
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for SENSOR_COLLECTION_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for SENSOR_COLLECTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SENSOR_CONNECTION_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SensorConnectionType_Integrated: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SensorConnectionType_Attached: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SensorConnectionType_External: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(2i32);
impl ::core::marker::Copy for SENSOR_CONNECTION_TYPES {}
impl ::core::clone::Clone for SENSOR_CONNECTION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SENSOR_CONNECTION_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SENSOR_CONNECTION_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for SENSOR_CONNECTION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SENSOR_CONNECTION_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ABSOLUTE_PRESSURE_PASCAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ACCELERATION_X_G: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ACCELERATION_Y_G: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ACCELERATION_Z_G: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ADDRESS1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 23u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ADDRESS2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 24u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ALTITUDE_ANTENNA_SEALEVEL_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 36u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ALTITUDE_ELLIPSOID_ERROR_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 29u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ALTITUDE_ELLIPSOID_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ALTITUDE_SEALEVEL_ERROR_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 30u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ALTITUDE_SEALEVEL_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_ACCELERATION_X_DEGREES_PER_SECOND_SQUARED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_ACCELERATION_Y_DEGREES_PER_SECOND_SQUARED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_ACCELERATION_Z_DEGREES_PER_SECOND_SQUARED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_VELOCITY_X_DEGREES_PER_SECOND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_VELOCITY_Y_DEGREES_PER_SECOND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_VELOCITY_Z_DEGREES_PER_SECOND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ATMOSPHERIC_PRESSURE_BAR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 4u32 };
pub const SENSOR_DATA_TYPE_BIOMETRIC_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_BOOLEAN_SWITCH_ARRAY_STATES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_BOOLEAN_SWITCH_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CAPACITANCE_FARAD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 25u32 };
pub const SENSOR_DATA_TYPE_COMMON_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb5e0cf2_cf1f_4c18_b46c_d86011d62150);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_COUNTRY_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 28u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CURRENT_AMPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_BOOLEAN_ARRAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 6u32 };
pub const SENSOR_DATA_TYPE_CUSTOM_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_USAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE10: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE11: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 17u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE12: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 18u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE13: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 19u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE14: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 20u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE15: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 21u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE16: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 22u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE17: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 23u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE18: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 24u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE19: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 25u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE20: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 26u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE21: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 27u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE22: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 28u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE23: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 29u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE24: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 30u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE25: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 31u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE26: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 32u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE27: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 33u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE28: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 34u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE3: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE4: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE5: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE6: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE7: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE8: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE9: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_DGPS_DATA_AGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 35u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_DIFFERENTIAL_REFERENCE_STATION_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 37u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_DISTANCE_X_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_DISTANCE_Y_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_DISTANCE_Z_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ELECTRICAL_FREQUENCY_HERTZ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 9u32 };
pub const SENSOR_DATA_TYPE_ELECTRICAL_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ELECTRICAL_PERCENT_OF_RANGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ELECTRICAL_POWER_WATTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 7u32 };
pub const SENSOR_DATA_TYPE_ENVIRONMENTAL_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ERROR_RADIUS_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 22u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_FIX_QUALITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_FIX_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_FORCE_NEWTONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_GAUGE_PRESSURE_PASCAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_GEOIDAL_SEPARATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 34u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_GPS_OPERATION_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 32u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_GPS_SELECTION_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 31u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_GPS_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 33u32 };
pub const SENSOR_DATA_TYPE_GUID_MECHANICAL_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_HORIZONAL_DILUTION_OF_PRECISION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_HUMAN_PRESENCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_HUMAN_PROXIMITY_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_INDUCTANCE_HENRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LATITUDE_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LIGHT_CHROMACITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6), pid: 4u32 };
pub const SENSOR_DATA_TYPE_LIGHT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LIGHT_LEVEL_LUX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LIGHT_TEMPERATURE_KELVIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6), pid: 3u32 };
pub const SENSOR_DATA_TYPE_LOCATION_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LOCATION_SOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 40u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LONGITUDE_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_FIELD_STRENGTH_X_MILLIGAUSS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 19u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_FIELD_STRENGTH_Y_MILLIGAUSS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 20u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_FIELD_STRENGTH_Z_MILLIGAUSS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 21u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_COMPENSATED_MAGNETIC_NORTH_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_COMPENSATED_TRUE_NORTH_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_MAGNETIC_NORTH_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_TRUE_NORTH_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_X_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_Y_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_Z_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_VARIATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETOMETER_ACCURACY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 22u32 };
pub const SENSOR_DATA_TYPE_MOTION_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MOTION_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MULTIVALUE_SWITCH_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_NMEA_SENTENCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 38u32 };
pub const SENSOR_DATA_TYPE_ORIENTATION_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_POSITION_DILUTION_OF_PRECISION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_POSTALCODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 27u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_QUADRANT_ANGLE_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_QUATERNION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 17u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_RELATIVE_HUMIDITY_PERCENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_RESISTANCE_OHMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_RFID_TAG_40_BIT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd7a59a3c_3421_44ab_8d3a_9de8ab6c4cae), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ROTATION_MATRIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 17u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_AZIMUTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 20u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_ELEVATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 19u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 39u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_PRNS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 18u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_STN_RATIO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 21u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_USED_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_USED_PRNS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_USED_PRNS_AND_CONSTELLATIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 41u32 };
pub const SENSOR_DATA_TYPE_SCANNER_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7a59a3c_3421_44ab_8d3a_9de8ab6c4cae);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SIMPLE_DEVICE_ORIENTATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 18u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SPEED_KNOTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SPEED_METERS_PER_SECOND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_STATE_PROVINCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 26u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_STRAIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TEMPERATURE_CELSIUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TILT_X_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TILT_Y_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TILT_Z_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TIMESTAMP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xdb5e0cf2_cf1f_4c18_b46c_d86011d62150), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TOUCH_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TRUE_HEADING_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_VERTICAL_DILUTION_OF_PRECISION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_VOLTAGE_VOLTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_WEIGHT_KILOGRAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_WIND_DIRECTION_DEGREES_ANTICLOCKWISE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_WIND_SPEED_METERS_PER_SECOND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 6u32 };
pub const SENSOR_ERROR_PARAMETER_COMMON_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77112bcd_fce1_4f43_b8b8_a88256adb4b3);
pub const SENSOR_EVENT_ACCELEROMETER_SHAKE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x825f5a94_0f48_4396_9ca0_6ecb5c99d915);
pub const SENSOR_EVENT_DATA_UPDATED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ed0f2a4_0087_41d3_87db_6773370b3c88);
pub const SENSOR_EVENT_PARAMETER_COMMON_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64346e30_8728_4b34_bdf6_4f52442c5c28);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_EVENT_PARAMETER_EVENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x64346e30_8728_4b34_bdf6_4f52442c5c28), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_EVENT_PARAMETER_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x64346e30_8728_4b34_bdf6_4f52442c5c28), pid: 3u32 };
pub const SENSOR_EVENT_PROPERTY_CHANGED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2358f099_84c9_4d3d_90df_c2421e2b2045);
pub const SENSOR_EVENT_STATE_CHANGED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfd96016_6bd7_4560_ad34_f2f6607e8f81);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_ACCURACY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 17u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_CHANGE_SENSITIVITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_CLEAR_ASSISTANCE_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe1e962f4_6e65_45f7_9c36_d487b7b1bd34), pid: 2u32 };
pub const SENSOR_PROPERTY_COMMON_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_CONNECTION_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_CURRENT_REPORT_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_DEVICE_PATH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_FRIENDLY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_HID_USAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 22u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_LIGHT_RESPONSE_CURVE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 16u32 };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct SENSOR_PROPERTY_LIST {
    pub AllocatedSizeInBytes: u32,
    pub Count: u32,
    pub List: [super::super::UI::Shell::PropertiesSystem::PROPERTYKEY; 1],
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for SENSOR_PROPERTY_LIST {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for SENSOR_PROPERTY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for SENSOR_PROPERTY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SENSOR_PROPERTY_LIST").field("AllocatedSizeInBytes", &self.AllocatedSizeInBytes).field("Count", &self.Count).field("List", &self.List).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows::core::Abi for SENSOR_PROPERTY_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for SENSOR_PROPERTY_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SENSOR_PROPERTY_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for SENSOR_PROPERTY_LIST {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for SENSOR_PROPERTY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_PROPERTY_LIST_HEADER_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_LOCATION_DESIRED_ACCURACY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 19u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_MIN_REPORT_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_RADIO_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 23u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_RADIO_STATE_PREVIOUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 24u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_RANGE_MAXIMUM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 21u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_RANGE_MINIMUM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 20u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 18u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 3u32 };
pub const SENSOR_PROPERTY_TEST_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1e962f4_6e65_45f7_9c36_d487b7b1bd34);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_TURN_ON_OFF_NMEA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe1e962f4_6e65_45f7_9c36_d487b7b1bd34), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SENSOR_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SensorState_Initializing: SENSOR_STATE = SENSOR_STATE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SensorState_Idle: SENSOR_STATE = SENSOR_STATE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SensorState_Active: SENSOR_STATE = SENSOR_STATE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SensorState_Error: SENSOR_STATE = SENSOR_STATE(3i32);
impl ::core::marker::Copy for SENSOR_STATE {}
impl ::core::clone::Clone for SENSOR_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SENSOR_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SENSOR_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SENSOR_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SENSOR_STATE").field(&self.0).finish()
    }
}
pub const SENSOR_TYPE_ACCELEROMETER_1D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc04d2387_7340_4cc2_991e_3b18cb8ef2f4);
pub const SENSOR_TYPE_ACCELEROMETER_2D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2c517a8_f6b5_4ba6_a423_5df560b4cc07);
pub const SENSOR_TYPE_ACCELEROMETER_3D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2fb0f5f_e2d2_4c78_bcd0_352a9582819d);
pub const SENSOR_TYPE_AGGREGATED_DEVICE_ORIENTATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdb5d8f7_3cfd_41c8_8542_cce622cf5d6e);
pub const SENSOR_TYPE_AGGREGATED_QUADRANT_ORIENTATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f81f1af_c4ab_4307_9904_c828bfb90829);
pub const SENSOR_TYPE_AGGREGATED_SIMPLE_DEVICE_ORIENTATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86a19291_0482_402c_bf4c_addac52b1c39);
pub const SENSOR_TYPE_AMBIENT_LIGHT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97f115c8_599a_4153_8894_d2d12899918a);
pub const SENSOR_TYPE_BARCODE_SCANNER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x990b3d8f_85bb_45ff_914d_998c04f372df);
pub const SENSOR_TYPE_BOOLEAN_SWITCH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c7e371f_1041_460b_8d5c_71e4752e350c);
pub const SENSOR_TYPE_BOOLEAN_SWITCH_ARRAY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x545c8ba5_b143_4545_868f_ca7fd986b4f6);
pub const SENSOR_TYPE_CAPACITANCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca2ffb1c_2317_49c0_a0b4_b63ce63461a0);
pub const SENSOR_TYPE_COMPASS_1D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa415f6c5_cb50_49d0_8e62_a8270bd7a26c);
pub const SENSOR_TYPE_COMPASS_2D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15655cc0_997a_4d30_84db_57caba3648bb);
pub const SENSOR_TYPE_COMPASS_3D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76b5ce0d_17dd_414d_93a1_e127f40bdf6e);
pub const SENSOR_TYPE_CURRENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5adc9fce_15a0_4bbe_a1ad_2d38a9ae831c);
pub const SENSOR_TYPE_CUSTOM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe83af229_8640_4d18_a213_e22675ebb2c3);
pub const SENSOR_TYPE_DISTANCE_1D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f14ab2f_1407_4306_a93f_b1dbabe4f9c0);
pub const SENSOR_TYPE_DISTANCE_2D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cf9a46c_a9a2_4e55_b6a1_a04aafa95a92);
pub const SENSOR_TYPE_DISTANCE_3D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa20cae31_0e25_4772_9fe5_96608a1354b2);
pub const SENSOR_TYPE_ELECTRICAL_POWER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x212f10f5_14ab_4376_9a43_a7794098c2fe);
pub const SENSOR_TYPE_ENVIRONMENTAL_ATMOSPHERIC_PRESSURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e903829_ff8a_4a93_97df_3dcbde402288);
pub const SENSOR_TYPE_ENVIRONMENTAL_HUMIDITY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c72bf67_bd7e_4257_990b_98a3ba3b400a);
pub const SENSOR_TYPE_ENVIRONMENTAL_TEMPERATURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04fd0ec4_d5da_45fa_95a9_5db38ee19306);
pub const SENSOR_TYPE_ENVIRONMENTAL_WIND_DIRECTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ef57a35_9306_434d_af09_37fa5a9c00bd);
pub const SENSOR_TYPE_ENVIRONMENTAL_WIND_SPEED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd50607b_a45f_42cd_8efd_ec61761c4226);
pub const SENSOR_TYPE_FORCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2ab2b02_1a1c_4778_a81b_954a1788cc75);
pub const SENSOR_TYPE_FREQUENCY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cd2cbb6_73e6_4640_a709_72ae8fb60d7f);
pub const SENSOR_TYPE_GYROMETER_1D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa088734_f552_4584_8324_edfaf649652c);
pub const SENSOR_TYPE_GYROMETER_2D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31ef4f83_919b_48bf_8de0_5d7a9d240556);
pub const SENSOR_TYPE_GYROMETER_3D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09485f5a_759e_42c2_bd4b_a349b75c8643);
pub const SENSOR_TYPE_HUMAN_PRESENCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc138c12b_ad52_451c_9375_87f518ff10c6);
pub const SENSOR_TYPE_HUMAN_PROXIMITY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5220dae9_3179_4430_9f90_06266d2a34de);
pub const SENSOR_TYPE_INCLINOMETER_1D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96f98c5_7a75_4ba7_94e9_ac868c966dd8);
pub const SENSOR_TYPE_INCLINOMETER_2D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab140f6d_83eb_4264_b70b_b16a5b256a01);
pub const SENSOR_TYPE_INCLINOMETER_3D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb84919fb_ea85_4976_8444_6f6f5c6d31db);
pub const SENSOR_TYPE_INDUCTANCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc1d933f_c435_4c7d_a2fe_607192a524d3);
pub const SENSOR_TYPE_LOCATION_BROADCAST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd26988cf_5162_4039_bb17_4c58b698e44a);
pub const SENSOR_TYPE_LOCATION_DEAD_RECKONING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a37d538_f28b_42da_9fce_a9d0a2a6d829);
pub const SENSOR_TYPE_LOCATION_GPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed4ca589_327a_4ff9_a560_91da4b48275e);
pub const SENSOR_TYPE_LOCATION_LOOKUP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b2eae4a_72ce_436d_96d2_3c5b8570e987);
pub const SENSOR_TYPE_LOCATION_OTHER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b2d0566_0368_4f71_b88d_533f132031de);
pub const SENSOR_TYPE_LOCATION_STATIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x095f8184_0fa9_4445_8e6e_b70f320b6b4c);
pub const SENSOR_TYPE_LOCATION_TRIANGULATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x691c341a_5406_4fe1_942f_2246cbeb39e0);
pub const SENSOR_TYPE_MOTION_DETECTOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c7c1a12_30a5_43b9_a4b2_cf09ec5b7be8);
pub const SENSOR_TYPE_MULTIVALUE_SWITCH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3ee4d76_37a4_4402_b25e_99c60a775fa1);
pub const SENSOR_TYPE_POTENTIOMETER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b3681a9_cadc_45aa_a6ff_54957c8bb440);
pub const SENSOR_TYPE_PRESSURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26d31f34_6352_41cf_b793_ea0713d53d77);
pub const SENSOR_TYPE_RESISTANCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9993d2c8_c157_4a52_a7b5_195c76037231);
pub const SENSOR_TYPE_RFID_SCANNER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44328ef5_02dd_4e8d_ad5d_9249832b2eca);
pub const SENSOR_TYPE_SCALE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc06dd92c_7feb_438e_9bf6_82207fff5bb8);
pub const SENSOR_TYPE_SPEEDOMETER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bd73c1f_0bb4_4310_81b2_dfc18a52bf94);
pub const SENSOR_TYPE_STRAIN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6d1ec0e_6803_4361_ad3d_85bcc58c6d29);
pub const SENSOR_TYPE_TOUCH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17db3018_06c4_4f7d_81af_9274b7599c27);
pub const SENSOR_TYPE_UNKNOWN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10ba83e3_ef4f_41ed_9885_a87d6435a8e1);
pub const SENSOR_TYPE_VOLTAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5484637_4fb7_4953_98b8_a56d8aa1fb1e);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct SENSOR_VALUE_PAIR {
    pub Key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
    pub Value: super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for SENSOR_VALUE_PAIR {
    fn clone(&self) -> Self {
        Self { Key: self.Key, Value: self.Value.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
unsafe impl ::windows::core::Abi for SENSOR_VALUE_PAIR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for SENSOR_VALUE_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Value == other.Value
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for SENSOR_VALUE_PAIR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for SENSOR_VALUE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SIMPLE_DEVICE_ORIENTATION(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SimpleDeviceOrientation_NotRotated: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SimpleDeviceOrientation_Rotated90DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SimpleDeviceOrientation_Rotated180DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SimpleDeviceOrientation_Rotated270DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(3i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SimpleDeviceOrientation_Faceup: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(4i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SimpleDeviceOrientation_Facedown: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(5i32);
impl ::core::marker::Copy for SIMPLE_DEVICE_ORIENTATION {}
impl ::core::clone::Clone for SIMPLE_DEVICE_ORIENTATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SIMPLE_DEVICE_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SIMPLE_DEVICE_ORIENTATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SIMPLE_DEVICE_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIMPLE_DEVICE_ORIENTATION").field(&self.0).finish()
    }
}
pub const Sensor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe97ced00_523a_4133_bf6f_d3a2dae7f6ba);
pub const SensorCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79c43adb_a429_469f_aa39_2f2b74b75937);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[inline]
pub unsafe fn SensorCollectionGetAt(index: u32, psensorslist: *const SENSOR_COLLECTION_LIST, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SensorCollectionGetAt(index: u32, psensorslist: *const SENSOR_COLLECTION_LIST, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> super::super::Foundation::NTSTATUS;
        }
        SensorCollectionGetAt(::core::mem::transmute(index), ::core::mem::transmute(psensorslist), ::core::mem::transmute(pkey), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SensorConnectionType(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_CONNECTION_TYPE_PC_INTEGRATED: SensorConnectionType = SensorConnectionType(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_CONNECTION_TYPE_PC_ATTACHED: SensorConnectionType = SensorConnectionType(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_CONNECTION_TYPE_PC_EXTERNAL: SensorConnectionType = SensorConnectionType(2i32);
impl ::core::marker::Copy for SensorConnectionType {}
impl ::core::clone::Clone for SensorConnectionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SensorConnectionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SensorConnectionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SensorConnectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorConnectionType").field(&self.0).finish()
    }
}
pub const SensorDataReport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ea9d6ef_694b_4218_8816_ccda8da74bba);
pub const SensorManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77a1c827_fcd2_4689_8915_9d613cc5fa3e);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SensorState(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_STATE_MIN: SensorState = SensorState(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_STATE_READY: SensorState = SensorState(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_STATE_NOT_AVAILABLE: SensorState = SensorState(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_STATE_NO_DATA: SensorState = SensorState(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_STATE_INITIALIZING: SensorState = SensorState(3i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_STATE_ACCESS_DENIED: SensorState = SensorState(4i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_STATE_ERROR: SensorState = SensorState(5i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SENSOR_STATE_MAX: SensorState = SensorState(5i32);
impl ::core::marker::Copy for SensorState {}
impl ::core::clone::Clone for SensorState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SensorState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SensorState {
    type Abi = Self;
}
impl ::core::fmt::Debug for SensorState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SerializationBufferAllocate(sizeinbytes: u32, pbuffer: *mut *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SerializationBufferAllocate(sizeinbytes: u32, pbuffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
        }
        SerializationBufferAllocate(::core::mem::transmute(sizeinbytes), ::core::mem::transmute(pbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[inline]
pub unsafe fn SerializationBufferFree(buffer: *const u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SerializationBufferFree(buffer: *const u8);
        }
        SerializationBufferFree(::core::mem::transmute(buffer))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SimpleDeviceOrientation(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SIMPLE_DEVICE_ORIENTATION_NOT_ROTATED: SimpleDeviceOrientation = SimpleDeviceOrientation(0i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_90: SimpleDeviceOrientation = SimpleDeviceOrientation(1i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_180: SimpleDeviceOrientation = SimpleDeviceOrientation(2i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_270: SimpleDeviceOrientation = SimpleDeviceOrientation(3i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_UP: SimpleDeviceOrientation = SimpleDeviceOrientation(4i32);
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_DOWN: SimpleDeviceOrientation = SimpleDeviceOrientation(5i32);
impl ::core::marker::Copy for SimpleDeviceOrientation {}
impl ::core::clone::Clone for SimpleDeviceOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SimpleDeviceOrientation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SimpleDeviceOrientation {
    type Abi = Self;
}
impl ::core::fmt::Debug for SimpleDeviceOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleDeviceOrientation").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`*"]
pub struct VEC3D {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
impl ::core::marker::Copy for VEC3D {}
impl ::core::clone::Clone for VEC3D {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VEC3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VEC3D").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
unsafe impl ::windows::core::Abi for VEC3D {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VEC3D {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VEC3D>()) == 0 }
    }
}
impl ::core::cmp::Eq for VEC3D {}
impl ::core::default::Default for VEC3D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
