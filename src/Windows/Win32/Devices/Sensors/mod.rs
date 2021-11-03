#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ACTIVITY_STATE(pub i32);
pub const ActivityState_Unknown: ACTIVITY_STATE = ACTIVITY_STATE(1i32);
pub const ActivityState_Stationary: ACTIVITY_STATE = ACTIVITY_STATE(2i32);
pub const ActivityState_Fidgeting: ACTIVITY_STATE = ACTIVITY_STATE(4i32);
pub const ActivityState_Walking: ACTIVITY_STATE = ACTIVITY_STATE(8i32);
pub const ActivityState_Running: ACTIVITY_STATE = ACTIVITY_STATE(16i32);
pub const ActivityState_InVehicle: ACTIVITY_STATE = ACTIVITY_STATE(32i32);
pub const ActivityState_Biking: ACTIVITY_STATE = ACTIVITY_STATE(64i32);
pub const ActivityState_Idle: ACTIVITY_STATE = ACTIVITY_STATE(128i32);
pub const ActivityState_Max: ACTIVITY_STATE = ACTIVITY_STATE(256i32);
pub const ActivityState_Force_Dword: ACTIVITY_STATE = ACTIVITY_STATE(-1i32);
impl ::std::convert::From<i32> for ACTIVITY_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACTIVITY_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ACTIVITY_STATE_COUNT(pub i32);
pub const ActivityStateCount: ACTIVITY_STATE_COUNT = ACTIVITY_STATE_COUNT(8i32);
impl ::std::convert::From<i32> for ACTIVITY_STATE_COUNT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACTIVITY_STATE_COUNT {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AXIS(pub i32);
pub const AXIS_X: AXIS = AXIS(0i32);
pub const AXIS_Y: AXIS = AXIS(1i32);
pub const AXIS_Z: AXIS = AXIS(2i32);
pub const AXIS_MAX: AXIS = AXIS(3i32);
impl ::std::convert::From<i32> for AXIS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AXIS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn CollectionsListAllocateBufferAndSerialize(sourcecollection: *const SENSOR_COLLECTION_LIST, ptargetbuffersizeinbytes: *mut u32, ptargetbuffer: *mut *mut u8) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListAllocateBufferAndSerialize(sourcecollection: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, ptargetbuffersizeinbytes: *mut u32, ptargetbuffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListAllocateBufferAndSerialize(::std::mem::transmute(sourcecollection), ::std::mem::transmute(ptargetbuffersizeinbytes), ::std::mem::transmute(ptargetbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn CollectionsListCopyAndMarshall(target: *mut SENSOR_COLLECTION_LIST, source: *const SENSOR_COLLECTION_LIST) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListCopyAndMarshall(target: *mut ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, source: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListCopyAndMarshall(::std::mem::transmute(target), ::std::mem::transmute(source)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn CollectionsListDeserializeFromBuffer(sourcebuffersizeinbytes: u32, sourcebuffer: *const u8, targetcollection: *mut SENSOR_COLLECTION_LIST) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListDeserializeFromBuffer(sourcebuffersizeinbytes: u32, sourcebuffer: *const u8, targetcollection: *mut ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListDeserializeFromBuffer(::std::mem::transmute(sourcebuffersizeinbytes), ::std::mem::transmute(sourcebuffer), ::std::mem::transmute(targetcollection)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[inline]
pub unsafe fn CollectionsListGetFillableCount(buffersizebytes: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListGetFillableCount(buffersizebytes: u32) -> u32;
        }
        ::std::mem::transmute(CollectionsListGetFillableCount(::std::mem::transmute(buffersizebytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn CollectionsListGetMarshalledSize(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListGetMarshalledSize(collection: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> u32;
        }
        ::std::mem::transmute(CollectionsListGetMarshalledSize(::std::mem::transmute(collection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn CollectionsListGetMarshalledSizeWithoutSerialization(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListGetMarshalledSizeWithoutSerialization(collection: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> u32;
        }
        ::std::mem::transmute(CollectionsListGetMarshalledSizeWithoutSerialization(::std::mem::transmute(collection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn CollectionsListGetSerializedSize(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListGetSerializedSize(collection: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> u32;
        }
        ::std::mem::transmute(CollectionsListGetSerializedSize(::std::mem::transmute(collection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn CollectionsListMarshall(target: *mut SENSOR_COLLECTION_LIST) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListMarshall(target: *mut ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListMarshall(::std::mem::transmute(target)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn CollectionsListSerializeToBuffer(sourcecollection: *const SENSOR_COLLECTION_LIST, targetbuffersizeinbytes: u32, targetbuffer: *mut u8) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListSerializeToBuffer(sourcecollection: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, targetbuffersizeinbytes: u32, targetbuffer: *mut u8) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListSerializeToBuffer(::std::mem::transmute(sourcecollection), ::std::mem::transmute(targetbuffersizeinbytes), ::std::mem::transmute(targetbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn CollectionsListSortSubscribedActivitiesByConfidence(thresholds: *const SENSOR_COLLECTION_LIST, pcollection: *mut SENSOR_COLLECTION_LIST) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListSortSubscribedActivitiesByConfidence(thresholds: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pcollection: *mut ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListSortSubscribedActivitiesByConfidence(::std::mem::transmute(thresholds), ::std::mem::transmute(pcollection)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn CollectionsListUpdateMarshalledPointer(collection: *mut SENSOR_COLLECTION_LIST) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CollectionsListUpdateMarshalledPointer(collection: *mut ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::NTSTATUS;
        }
        CollectionsListUpdateMarshalledPointer(::std::mem::transmute(collection)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ELEVATION_CHANGE_MODE(pub i32);
pub const ElevationChangeMode_Unknown: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(0i32);
pub const ElevationChangeMode_Elevator: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(1i32);
pub const ElevationChangeMode_Stepping: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(2i32);
pub const ElevationChangeMode_Max: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(3i32);
pub const ElevationChangeMode_Force_Dword: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(-1i32);
impl ::std::convert::From<i32> for ELEVATION_CHANGE_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ELEVATION_CHANGE_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn EvaluateActivityThresholds(newsample: *const SENSOR_COLLECTION_LIST, oldsample: *const SENSOR_COLLECTION_LIST, thresholds: *const SENSOR_COLLECTION_LIST) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvaluateActivityThresholds(newsample: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, oldsample: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, thresholds: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(EvaluateActivityThresholds(::std::mem::transmute(newsample), ::std::mem::transmute(oldsample), ::std::mem::transmute(thresholds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
pub const GNSS_CLEAR_ALL_ASSISTANCE_DATA: u32 = 1u32;
pub const GUID_DEVINTERFACE_SENSOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3122378386, 39802, 18483, [154, 30, 82, 94, 209, 52, 231, 226]);
pub const GUID_SensorCategory_All: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3273114246, 50280, 17032, [153, 117, 212, 196, 88, 124, 68, 44]);
pub const GUID_SensorCategory_Biometric: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3390662927, 41671, 18301, [169, 158, 153, 236, 110, 43, 86, 72]);
pub const GUID_SensorCategory_Electrical: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218682584, 64586, 18492, [172, 88, 39, 182, 145, 198, 190, 255]);
pub const GUID_SensorCategory_Environmental: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(842283434, 32614, 18731, [186, 12, 115, 233, 170, 10, 101, 213]);
pub const GUID_SensorCategory_Light: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(396780992, 36963, 16918, [178, 2, 92, 122, 37, 94, 24, 206]);
pub const GUID_SensorCategory_Location: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3215430884, 63844, 20443, [144, 246, 81, 5, 107, 254, 75, 68]);
pub const GUID_SensorCategory_Mechanical: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2366840168, 36599, 18006, [128, 181, 204, 203, 217, 55, 145, 197]);
pub const GUID_SensorCategory_Motion: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3439975153, 15150, 19517, [181, 152, 181, 229, 255, 147, 253, 70]);
pub const GUID_SensorCategory_Orientation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2657879222, 38654, 18772, [183, 38, 104, 104, 42, 71, 63, 105]);
pub const GUID_SensorCategory_Other: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747693993, 62665, 20386, [175, 55, 86, 212, 113, 254, 90, 61]);
pub const GUID_SensorCategory_PersonalActivity: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4049637505, 7698, 16683, [161, 77, 203, 176, 233, 91, 210, 229]);
pub const GUID_SensorCategory_Scanner: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2952849278, 62901, 16911, [129, 93, 2, 112, 167, 38, 242, 112]);
pub const GUID_SensorCategory_Unsupported: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(736815098, 6576, 18629, [161, 246, 181, 72, 13, 194, 6, 176]);
pub const GUID_SensorType_Accelerometer3D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3271233375, 58066, 19576, [188, 208, 53, 42, 149, 130, 129, 157]);
pub const GUID_SensorType_ActivityDetection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2644377880, 6151, 20270, [150, 228, 44, 229, 113, 66, 225, 150]);
pub const GUID_SensorType_AmbientLight: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2549159368, 22938, 16723, [136, 148, 210, 209, 40, 153, 145, 138]);
pub const GUID_SensorType_Barometer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(244332585, 65418, 19091, [151, 223, 61, 203, 222, 64, 34, 136]);
pub const GUID_SensorType_Custom: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3896177193, 34368, 19736, [162, 19, 226, 38, 117, 235, 178, 195]);
pub const GUID_SensorType_FloorElevation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2917439615, 31428, 19962, [151, 34, 10, 2, 113, 129, 199, 71]);
pub const GUID_SensorType_GeomagneticOrientation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3882980856, 11551, 18467, [151, 27, 28, 68, 103, 85, 108, 157]);
pub const GUID_SensorType_GravityVector: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(62205043, 47990, 17983, [149, 36, 56, 222, 118, 235, 112, 11]);
pub const GUID_SensorType_Gyrometer3D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(155737946, 30110, 17090, [189, 75, 163, 73, 183, 92, 134, 67]);
pub const GUID_SensorType_HingeAngle: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2184544357, 62660, 19873, [178, 114, 19, 194, 51, 50, 162, 7]);
pub const GUID_SensorType_Humidity: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1551023975, 48510, 16983, [153, 11, 152, 163, 186, 59, 64, 10]);
pub const GUID_SensorType_LinearAccelerometer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(59441795, 38836, 16840, [188, 36, 95, 241, 170, 72, 254, 199]);
pub const GUID_SensorType_Magnetometer3D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1441132539, 5575, 16607, [134, 152, 168, 75, 124, 134, 60, 83]);
pub const GUID_SensorType_Orientation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3451246839, 15613, 16840, [133, 66, 204, 230, 34, 207, 93, 110]);
pub const GUID_SensorType_Pedometer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2980022703, 58347, 17483, [141, 234, 32, 37, 117, 167, 21, 153]);
pub const GUID_SensorType_Proximity: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1377884905, 12665, 17456, [159, 144, 6, 38, 109, 42, 52, 222]);
pub const GUID_SensorType_RelativeOrientation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1083784017, 18182, 17628, [152, 213, 201, 32, 192, 55, 255, 171]);
pub const GUID_SensorType_SimpleDeviceOrientation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2258735761, 1154, 16428, [191, 76, 173, 218, 197, 43, 28, 57]);
pub const GUID_SensorType_Temperature: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(83693252, 54746, 17914, [149, 169, 93, 179, 142, 225, 147, 6]);
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetPerformanceTime(timems: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPerformanceTime(timems: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        GetPerformanceTime(::std::mem::transmute(timems)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HUMAN_PRESENCE_DETECTION_TYPE(pub i32);
pub const HumanPresenceDetectionType_VendorDefinedNonBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(1i32);
pub const HumanPresenceDetectionType_VendorDefinedBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(2i32);
pub const HumanPresenceDetectionType_FacialBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(4i32);
pub const HumanPresenceDetectionType_AudioBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(8i32);
pub const HumanPresenceDetectionType_Force_Dword: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(-1i32);
impl ::std::convert::From<i32> for HUMAN_PRESENCE_DETECTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HUMAN_PRESENCE_DETECTION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HUMAN_PRESENCE_DETECTION_TYPE_COUNT(pub i32);
pub const HumanPresenceDetectionTypeCount: HUMAN_PRESENCE_DETECTION_TYPE_COUNT = HUMAN_PRESENCE_DETECTION_TYPE_COUNT(4i32);
impl ::std::convert::From<i32> for HUMAN_PRESENCE_DETECTION_TYPE_COUNT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HUMAN_PRESENCE_DETECTION_TYPE_COUNT {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ILocationPermissions(::windows::runtime::IUnknown);
impl ILocationPermissions {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`*"]
    pub unsafe fn GetGlobalLocationPermission(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn CheckLocationCapability(&self, dwclientthreadid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwclientthreadid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILocationPermissions {
    type Vtable = ILocationPermissions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3589999231, 59214, 17653, [142, 2, 72, 6, 134, 58, 39, 79]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationPermissions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwclientthreadid: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISensor(::windows::runtime::IUnknown);
impl ISensor {
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn GetID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn GetCategory(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`*"]
    pub unsafe fn GetFriendlyName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetProperty(&self, key: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::PortableDevices::IPortableDeviceKeyCollection>>(&self, pkeys: Param0) -> ::windows::runtime::Result<super::PortableDevices::IPortableDeviceValues> {
        let mut result__: <super::PortableDevices::IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pkeys.into_param().abi(), &mut result__).from_abi::<super::PortableDevices::IPortableDeviceValues>(result__)
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedDataFields(&self) -> ::windows::runtime::Result<super::PortableDevices::IPortableDeviceKeyCollection> {
        let mut result__: <super::PortableDevices::IPortableDeviceKeyCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::PortableDevices::IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Devices_PortableDevices`*"]
    pub unsafe fn SetProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::PortableDevices::IPortableDeviceValues>>(&self, pproperties: Param0) -> ::windows::runtime::Result<super::PortableDevices::IPortableDeviceValues> {
        let mut result__: <super::PortableDevices::IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pproperties.into_param().abi(), &mut result__).from_abi::<super::PortableDevices::IPortableDeviceValues>(result__)
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SupportsDataField(&self, key: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn GetState(&self) -> ::windows::runtime::Result<SensorState> {
        let mut result__: <SensorState as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<SensorState>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn GetData(&self) -> ::windows::runtime::Result<ISensorDataReport> {
        let mut result__: <ISensorDataReport as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISensorDataReport>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn SupportsEvent(&self, eventguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventguid), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn GetEventInterest(&self, ppvalues: *mut *mut ::windows::runtime::GUID, pcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppvalues), ::std::mem::transmute(pcount)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn SetEventInterest(&self, pvalues: *const ::windows::runtime::GUID, count: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvalues), ::std::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn SetEventSink<'a, Param0: ::windows::runtime::IntoParam<'a, ISensorEvents>>(&self, pevents: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pevents.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISensor {
    type Vtable = ISensor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1604358016, 9815, 17806, [175, 117, 70, 247, 63, 166, 172, 92]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psensorcategory: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psensortype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfriendlyname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, pproperty: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkeys: ::windows::runtime::RawPtr, ppproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))] usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdatafields: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))] usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproperties: ::windows::runtime::RawPtr, ppresults: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, pissupported: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstate: *mut SensorState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdatareport: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventguid: *const ::windows::runtime::GUID, pissupported: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppvalues: *mut *mut ::windows::runtime::GUID, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalues: *const ::windows::runtime::GUID, count: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pevents: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISensorCollection(::windows::runtime::IUnknown);
impl ISensorCollection {
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn GetAt(&self, ulindex: u32) -> ::windows::runtime::Result<ISensor> {
        let mut result__: <ISensor as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulindex), &mut result__).from_abi::<ISensor>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, ISensor>>(&self, psensor: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), psensor.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ISensor>>(&self, psensor: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), psensor.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn RemoveByID(&self, sensorid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(sensorid)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISensorCollection {
    type Vtable = ISensorCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(592911889, 58693, 19928, [163, 55, 184, 155, 244, 75, 16, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulindex: u32, ppsensor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psensor: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psensor: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sensorid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISensorDataReport(::windows::runtime::IUnknown);
impl ISensorDataReport {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`*"]
    pub unsafe fn GetTimestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: <super::super::Foundation::SYSTEMTIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetSensorValue(&self, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pkey), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSensorValues<'a, Param0: ::windows::runtime::IntoParam<'a, super::PortableDevices::IPortableDeviceKeyCollection>>(&self, pkeys: Param0) -> ::windows::runtime::Result<super::PortableDevices::IPortableDeviceValues> {
        let mut result__: <super::PortableDevices::IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pkeys.into_param().abi(), &mut result__).from_abi::<super::PortableDevices::IPortableDeviceValues>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISensorDataReport {
    type Vtable = ISensorDataReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(179953563, 50357, 18326, [136, 152, 4, 112, 112, 106, 46, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptimestamp: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkeys: ::windows::runtime::RawPtr, ppvalues: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISensorEvents(::windows::runtime::IUnknown);
impl ISensorEvents {
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn OnStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, ISensor>>(&self, psensor: Param0, state: SensorState) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), psensor.into_param().abi(), ::std::mem::transmute(state)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn OnDataUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, ISensor>, Param1: ::windows::runtime::IntoParam<'a, ISensorDataReport>>(&self, psensor: Param0, pnewdata: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), psensor.into_param().abi(), pnewdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Devices_PortableDevices`*"]
    pub unsafe fn OnEvent<'a, Param0: ::windows::runtime::IntoParam<'a, ISensor>, Param2: ::windows::runtime::IntoParam<'a, super::PortableDevices::IPortableDeviceValues>>(&self, psensor: Param0, eventid: *const ::windows::runtime::GUID, peventdata: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), psensor.into_param().abi(), ::std::mem::transmute(eventid), peventdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn OnLeave(&self, id: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(id)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISensorEvents {
    type Vtable = ISensorEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1569574033, 17985, 18407, [183, 195, 183, 79, 72, 166, 195, 145]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psensor: ::windows::runtime::RawPtr, state: SensorState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psensor: ::windows::runtime::RawPtr, pnewdata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Devices_PortableDevices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psensor: ::windows::runtime::RawPtr, eventid: *const ::windows::runtime::GUID, peventdata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISensorManager(::windows::runtime::IUnknown);
impl ISensorManager {
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn GetSensorsByCategory(&self, sensorcategory: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ISensorCollection> {
        let mut result__: <ISensorCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(sensorcategory), &mut result__).from_abi::<ISensorCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn GetSensorsByType(&self, sensortype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ISensorCollection> {
        let mut result__: <ISensorCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(sensortype), &mut result__).from_abi::<ISensorCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn GetSensorByID(&self, sensorid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ISensor> {
        let mut result__: <ISensor as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(sensorid), &mut result__).from_abi::<ISensor>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn SetEventSink<'a, Param0: ::windows::runtime::IntoParam<'a, ISensorManagerEvents>>(&self, pevents: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pevents.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`*"]
    pub unsafe fn RequestPermissions<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ISensorCollection>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hparent: Param0, psensors: Param1, fmodal: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), hparent.into_param().abi(), psensors.into_param().abi(), fmodal.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISensorManager {
    type Vtable = ISensorManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3178748775, 17832, 17116, [141, 0, 109, 207, 21, 248, 55, 122]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sensorcategory: *const ::windows::runtime::GUID, ppsensorsfound: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sensortype: *const ::windows::runtime::GUID, ppsensorsfound: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sensorid: *const ::windows::runtime::GUID, ppsensor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pevents: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hparent: super::super::Foundation::HWND, psensors: ::windows::runtime::RawPtr, fmodal: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISensorManagerEvents(::windows::runtime::IUnknown);
impl ISensorManagerEvents {
    #[doc = "*Required features: `Win32_Devices_Sensors`*"]
    pub unsafe fn OnSensorEnter<'a, Param0: ::windows::runtime::IntoParam<'a, ISensor>>(&self, psensor: Param0, state: SensorState) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), psensor.into_param().abi(), ::std::mem::transmute(state)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISensorManagerEvents {
    type Vtable = ISensorManagerEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2604338054, 9834, 19117, [178, 31, 253, 229, 80, 16, 1, 183]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorManagerEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psensor: ::windows::runtime::RawPtr, state: SensorState) -> ::windows::runtime::HRESULT,
);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromCLSIDArray(members: *const ::windows::runtime::GUID, size: u32) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromCLSIDArray(members: *const ::windows::runtime::GUID, size: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromCLSIDArray(::std::mem::transmute(members), ::std::mem::transmute(size), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromFloat(fltval: f32) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromFloat(fltval: f32, ppropvar: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromFloat(::std::mem::transmute(fltval), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn IsCollectionListSame(lista: *const SENSOR_COLLECTION_LIST, listb: *const SENSOR_COLLECTION_LIST) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsCollectionListSame(lista: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, listb: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(IsCollectionListSame(::std::mem::transmute(lista), ::std::mem::transmute(listb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn IsGUIDPresentInList(guidarray: *const ::windows::runtime::GUID, arraylength: u32, guidelem: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsGUIDPresentInList(guidarray: *const ::windows::runtime::GUID, arraylength: u32, guidelem: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(IsGUIDPresentInList(::std::mem::transmute(guidarray), ::std::mem::transmute(arraylength), ::std::mem::transmute(guidelem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn IsKeyPresentInCollectionList(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsKeyPresentInCollectionList(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(IsKeyPresentInCollectionList(::std::mem::transmute(plist), ::std::mem::transmute(pkey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn IsKeyPresentInPropertyList(plist: *const SENSOR_PROPERTY_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsKeyPresentInPropertyList(plist: *const SENSOR_PROPERTY_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(IsKeyPresentInPropertyList(::std::mem::transmute(plist), ::std::mem::transmute(pkey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn IsSensorSubscribed<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(subscriptionlist: *const SENSOR_COLLECTION_LIST, currenttype: Param1) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsSensorSubscribed(subscriptionlist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, currenttype: ::windows::runtime::GUID) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(IsSensorSubscribed(::std::mem::transmute(subscriptionlist), currenttype.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LOCATION_DESIRED_ACCURACY(pub i32);
pub const LOCATION_DESIRED_ACCURACY_DEFAULT: LOCATION_DESIRED_ACCURACY = LOCATION_DESIRED_ACCURACY(0i32);
pub const LOCATION_DESIRED_ACCURACY_HIGH: LOCATION_DESIRED_ACCURACY = LOCATION_DESIRED_ACCURACY(1i32);
impl ::std::convert::From<i32> for LOCATION_DESIRED_ACCURACY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LOCATION_DESIRED_ACCURACY {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LOCATION_POSITION_SOURCE(pub i32);
pub const LOCATION_POSITION_SOURCE_CELLULAR: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(0i32);
pub const LOCATION_POSITION_SOURCE_SATELLITE: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(1i32);
pub const LOCATION_POSITION_SOURCE_WIFI: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(2i32);
pub const LOCATION_POSITION_SOURCE_IPADDRESS: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(3i32);
pub const LOCATION_POSITION_SOURCE_UNKNOWN: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(4i32);
impl ::std::convert::From<i32> for LOCATION_POSITION_SOURCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LOCATION_POSITION_SOURCE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MAGNETOMETER_ACCURACY(pub i32);
pub const MagnetometerAccuracy_Unknown: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(0i32);
pub const MagnetometerAccuracy_Unreliable: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(1i32);
pub const MagnetometerAccuracy_Approximate: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(2i32);
pub const MagnetometerAccuracy_High: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(3i32);
impl ::std::convert::From<i32> for MAGNETOMETER_ACCURACY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MAGNETOMETER_ACCURACY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
pub struct MATRIX3X3 {
    pub Anonymous: MATRIX3X3_0,
}
impl MATRIX3X3 {}
impl ::std::default::Default for MATRIX3X3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MATRIX3X3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MATRIX3X3 {}
unsafe impl ::windows::runtime::Abi for MATRIX3X3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
pub union MATRIX3X3_0 {
    pub Anonymous1: MATRIX3X3_0_0,
    pub Anonymous2: MATRIX3X3_0_1,
    pub M: [f32; 9],
}
impl MATRIX3X3_0 {}
impl ::std::default::Default for MATRIX3X3_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MATRIX3X3_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MATRIX3X3_0 {}
unsafe impl ::windows::runtime::Abi for MATRIX3X3_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
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
impl MATRIX3X3_0_0 {}
impl ::std::default::Default for MATRIX3X3_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MATRIX3X3_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct").field("A11", &self.A11).field("A12", &self.A12).field("A13", &self.A13).field("A21", &self.A21).field("A22", &self.A22).field("A23", &self.A23).field("A31", &self.A31).field("A32", &self.A32).field("A33", &self.A33).finish()
    }
}
impl ::std::cmp::PartialEq for MATRIX3X3_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.A11 == other.A11 && self.A12 == other.A12 && self.A13 == other.A13 && self.A21 == other.A21 && self.A22 == other.A22 && self.A23 == other.A23 && self.A31 == other.A31 && self.A32 == other.A32 && self.A33 == other.A33
    }
}
impl ::std::cmp::Eq for MATRIX3X3_0_0 {}
unsafe impl ::windows::runtime::Abi for MATRIX3X3_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
pub struct MATRIX3X3_0_1 {
    pub V1: VEC3D,
    pub V2: VEC3D,
    pub V3: VEC3D,
}
impl MATRIX3X3_0_1 {}
impl ::std::default::Default for MATRIX3X3_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MATRIX3X3_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct").field("V1", &self.V1).field("V2", &self.V2).field("V3", &self.V3).finish()
    }
}
impl ::std::cmp::PartialEq for MATRIX3X3_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.V1 == other.V1 && self.V2 == other.V2 && self.V3 == other.V3
    }
}
impl ::std::cmp::Eq for MATRIX3X3_0_1 {}
unsafe impl ::windows::runtime::Abi for MATRIX3X3_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagnetometerAccuracy(pub i32);
pub const MAGNETOMETER_ACCURACY_UNKNOWN: MagnetometerAccuracy = MagnetometerAccuracy(0i32);
pub const MAGNETOMETER_ACCURACY_UNRELIABLE: MagnetometerAccuracy = MagnetometerAccuracy(1i32);
pub const MAGNETOMETER_ACCURACY_APPROXIMATE: MagnetometerAccuracy = MagnetometerAccuracy(2i32);
pub const MAGNETOMETER_ACCURACY_HIGH: MagnetometerAccuracy = MagnetometerAccuracy(3i32);
impl ::std::convert::From<i32> for MagnetometerAccuracy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MagnetometerAccuracy {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PEDOMETER_STEP_TYPE(pub i32);
pub const PedometerStepType_Unknown: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(1i32);
pub const PedometerStepType_Walking: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(2i32);
pub const PedometerStepType_Running: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(4i32);
pub const PedometerStepType_Max: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(8i32);
pub const PedometerStepType_Force_Dword: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(-1i32);
impl ::std::convert::From<i32> for PEDOMETER_STEP_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PEDOMETER_STEP_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PEDOMETER_STEP_TYPE_COUNT(pub i32);
pub const PedometerStepTypeCount: PEDOMETER_STEP_TYPE_COUNT = PEDOMETER_STEP_TYPE_COUNT(3i32);
impl ::std::convert::From<i32> for PEDOMETER_STEP_TYPE_COUNT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PEDOMETER_STEP_TYPE_COUNT {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROXIMITY_TYPE(pub i32);
pub const ProximityType_ObjectProximity: PROXIMITY_TYPE = PROXIMITY_TYPE(0i32);
pub const ProximityType_HumanProximity: PROXIMITY_TYPE = PROXIMITY_TYPE(1i32);
pub const ProximityType_Force_Dword: PROXIMITY_TYPE = PROXIMITY_TYPE(-1i32);
impl ::std::convert::From<i32> for PROXIMITY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROXIMITY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetBool(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetBool(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::BOOL) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetBool(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetDouble(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetDouble(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f64) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetDouble(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetFileTime(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetFileTime(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetFileTime(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetFloat(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetFloat(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f32) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetFloat(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetGuid(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetGuid(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut ::windows::runtime::GUID) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetGuid(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetInt32(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetInt32(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i32) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetInt32(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetInt64(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetInt64(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i64) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetInt64(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthInt64(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut i64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetNthInt64(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut i64) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetNthInt64(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(occurrence), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthUlong(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetNthUlong(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetNthUlong(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(occurrence), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthUshort(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u16) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetNthUshort(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u16) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetNthUshort(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(occurrence), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetPropVariant<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, typecheck: Param2, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetPropVariant(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, typecheck: super::super::Foundation::BOOLEAN, pvalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetPropVariant(::std::mem::transmute(plist), ::std::mem::transmute(pkey), typecheck.into_param().abi(), ::std::mem::transmute(pvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetUlong(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetUlong(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetUlong(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeyGetUshort(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u16) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeyGetUshort(plist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u16) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeyGetUshort(::std::mem::transmute(plist), ::std::mem::transmute(pkey), ::std::mem::transmute(pretvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropKeyFindKeySetPropVariant<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(plist: *mut SENSOR_COLLECTION_LIST, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, typecheck: Param2, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropKeyFindKeySetPropVariant(plist: *mut ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY, typecheck: super::super::Foundation::BOOLEAN, pvalue: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> super::super::Foundation::NTSTATUS;
        }
        PropKeyFindKeySetPropVariant(::std::mem::transmute(plist), ::std::mem::transmute(pkey), typecheck.into_param().abi(), ::std::mem::transmute(pvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetInformation(propvariantvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propvariantoffset: *mut u32, propvariantsize: *mut u32, propvariantpointer: *mut *mut ::std::ffi::c_void, remappedtype: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetInformation(propvariantvalue: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, propvariantoffset: *mut u32, propvariantsize: *mut u32, propvariantpointer: *mut *mut ::std::ffi::c_void, remappedtype: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        PropVariantGetInformation(::std::mem::transmute(propvariantvalue), ::std::mem::transmute(propvariantoffset), ::std::mem::transmute(propvariantsize), ::std::mem::transmute(propvariantpointer), ::std::mem::transmute(remappedtype)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PropertiesListCopy(target: *mut SENSOR_PROPERTY_LIST, source: *const SENSOR_PROPERTY_LIST) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropertiesListCopy(target: *mut SENSOR_PROPERTY_LIST, source: *const SENSOR_PROPERTY_LIST) -> super::super::Foundation::NTSTATUS;
        }
        PropertiesListCopy(::std::mem::transmute(target), ::std::mem::transmute(source)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[inline]
pub unsafe fn PropertiesListGetFillableCount(buffersizebytes: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropertiesListGetFillableCount(buffersizebytes: u32) -> u32;
        }
        ::std::mem::transmute(PropertiesListGetFillableCount(::std::mem::transmute(buffersizebytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
pub struct QUATERNION {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}
impl QUATERNION {}
impl ::std::default::Default for QUATERNION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for QUATERNION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("QUATERNION").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).field("W", &self.W).finish()
    }
}
impl ::std::cmp::PartialEq for QUATERNION {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z && self.W == other.W
    }
}
impl ::std::cmp::Eq for QUATERNION {}
unsafe impl ::windows::runtime::Abi for QUATERNION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SENSOR_CATEGORY_ALL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3273114246, 50280, 17032, [153, 117, 212, 196, 88, 124, 68, 44]);
pub const SENSOR_CATEGORY_BIOMETRIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3390662927, 41671, 18301, [169, 158, 153, 236, 110, 43, 86, 72]);
pub const SENSOR_CATEGORY_ELECTRICAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218682584, 64586, 18492, [172, 88, 39, 182, 145, 198, 190, 255]);
pub const SENSOR_CATEGORY_ENVIRONMENTAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(842283434, 32614, 18731, [186, 12, 115, 233, 170, 10, 101, 213]);
pub const SENSOR_CATEGORY_LIGHT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(396780992, 36963, 16918, [178, 2, 92, 122, 37, 94, 24, 206]);
pub const SENSOR_CATEGORY_LOCATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3215430884, 63844, 20443, [144, 246, 81, 5, 107, 254, 75, 68]);
pub const SENSOR_CATEGORY_MECHANICAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2366840168, 36599, 18006, [128, 181, 204, 203, 217, 55, 145, 197]);
pub const SENSOR_CATEGORY_MOTION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3439975153, 15150, 19517, [181, 152, 181, 229, 255, 147, 253, 70]);
pub const SENSOR_CATEGORY_ORIENTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2657879222, 38654, 18772, [183, 38, 104, 104, 42, 71, 63, 105]);
pub const SENSOR_CATEGORY_OTHER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747693993, 62665, 20386, [175, 55, 86, 212, 113, 254, 90, 61]);
pub const SENSOR_CATEGORY_SCANNER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2952849278, 62901, 16911, [129, 93, 2, 112, 167, 38, 242, 112]);
pub const SENSOR_CATEGORY_UNSUPPORTED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(736815098, 6576, 18629, [161, 246, 181, 72, 13, 194, 6, 176]);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::clone::Clone for SENSOR_COLLECTION_LIST {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
pub struct SENSOR_COLLECTION_LIST {
    pub AllocatedSizeInBytes: u32,
    pub Count: u32,
    pub List: [SENSOR_VALUE_PAIR; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
impl SENSOR_COLLECTION_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::default::Default for SENSOR_COLLECTION_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::cmp::PartialEq for SENSOR_COLLECTION_LIST {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::cmp::Eq for SENSOR_COLLECTION_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
unsafe impl ::windows::runtime::Abi for SENSOR_COLLECTION_LIST {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SENSOR_CONNECTION_TYPES(pub i32);
pub const SensorConnectionType_Integrated: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(0i32);
pub const SensorConnectionType_Attached: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(1i32);
pub const SensorConnectionType_External: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(2i32);
impl ::std::convert::From<i32> for SENSOR_CONNECTION_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SENSOR_CONNECTION_TYPES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SENSOR_DATA_TYPE_BIOMETRIC_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(580462730, 28062, 19211, [183, 236, 53, 40, 248, 158, 64, 175]);
pub const SENSOR_DATA_TYPE_COMMON_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3680374002, 53023, 19480, [180, 108, 216, 96, 17, 214, 33, 80]);
pub const SENSOR_DATA_TYPE_CUSTOM_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2974578255, 1999, 16872, [157, 130, 235, 227, 208, 119, 106, 111]);
pub const SENSOR_DATA_TYPE_ELECTRICAL_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3149022929, 57922, 18304, [162, 211, 205, 237, 132, 243, 88, 66]);
pub const SENSOR_DATA_TYPE_ENVIRONMENTAL_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2332730097, 11607, 17134, [140, 192, 77, 39, 98, 43, 70, 196]);
pub const SENSOR_DATA_TYPE_GUID_MECHANICAL_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(945179260, 62194, 18875, [155, 43, 186, 96, 246, 106, 88, 223]);
pub const SENSOR_DATA_TYPE_LIGHT_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3838278882, 56503, 18153, [132, 57, 79, 236, 84, 136, 51, 166]);
pub const SENSOR_DATA_TYPE_LOCATION_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(89945304, 51823, 18390, [149, 198, 30, 211, 99, 122, 15, 244]);
pub const SENSOR_DATA_TYPE_MOTION_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1066035618, 1989, 20040, [169, 101, 205, 121, 122, 171, 86, 213]);
pub const SENSOR_DATA_TYPE_ORIENTATION_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(372758690, 16968, 17013, [134, 93, 85, 141, 232, 74, 237, 253]);
pub const SENSOR_DATA_TYPE_SCANNER_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3617954364, 13345, 17579, [141, 58, 157, 232, 171, 108, 76, 174]);
pub const SENSOR_ERROR_PARAMETER_COMMON_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1997614029, 64737, 20291, [184, 184, 168, 130, 86, 173, 180, 179]);
pub const SENSOR_EVENT_ACCELEROMETER_SHAKE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2187287188, 3912, 17302, [156, 160, 110, 203, 92, 153, 217, 21]);
pub const SENSOR_EVENT_DATA_UPDATED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(785445540, 135, 16851, [135, 219, 103, 115, 55, 11, 60, 136]);
pub const SENSOR_EVENT_PARAMETER_COMMON_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1681157680, 34600, 19252, [189, 246, 79, 82, 68, 44, 92, 40]);
pub const SENSOR_EVENT_PROPERTY_CHANGED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(593031321, 33993, 19773, [144, 223, 194, 66, 30, 43, 32, 69]);
pub const SENSOR_EVENT_STATE_CHANGED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3218694166, 27607, 17760, [173, 52, 242, 246, 96, 126, 143, 129]);
pub const SENSOR_PROPERTY_COMMON_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2139325420, 54252, 18780, [168, 207, 184, 187, 232, 92, 41, 32]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_System_PropertiesSystem`*"]
pub struct SENSOR_PROPERTY_LIST {
    pub AllocatedSizeInBytes: u32,
    pub Count: u32,
    pub List: [super::super::System::PropertiesSystem::PROPERTYKEY; 1],
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl SENSOR_PROPERTY_LIST {}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl ::std::default::Default for SENSOR_PROPERTY_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl ::std::fmt::Debug for SENSOR_PROPERTY_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SENSOR_PROPERTY_LIST").field("AllocatedSizeInBytes", &self.AllocatedSizeInBytes).field("Count", &self.Count).field("List", &self.List).finish()
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl ::std::cmp::PartialEq for SENSOR_PROPERTY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.AllocatedSizeInBytes == other.AllocatedSizeInBytes && self.Count == other.Count && self.List == other.List
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl ::std::cmp::Eq for SENSOR_PROPERTY_LIST {}
#[cfg(feature = "Win32_System_PropertiesSystem")]
unsafe impl ::windows::runtime::Abi for SENSOR_PROPERTY_LIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
pub const SENSOR_PROPERTY_LIST_HEADER_SIZE: u32 = 8u32;
pub const SENSOR_PROPERTY_TEST_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3790168820, 28261, 17911, [156, 54, 212, 135, 183, 177, 189, 52]);
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SENSOR_STATE(pub i32);
pub const SensorState_Initializing: SENSOR_STATE = SENSOR_STATE(0i32);
pub const SensorState_Idle: SENSOR_STATE = SENSOR_STATE(1i32);
pub const SensorState_Active: SENSOR_STATE = SENSOR_STATE(2i32);
pub const SensorState_Error: SENSOR_STATE = SENSOR_STATE(3i32);
impl ::std::convert::From<i32> for SENSOR_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SENSOR_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SENSOR_TYPE_ACCELEROMETER_1D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3226280839, 29504, 19650, [153, 30, 59, 24, 203, 142, 242, 244]);
pub const SENSOR_TYPE_ACCELEROMETER_2D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2999261096, 63157, 19366, [164, 35, 93, 245, 96, 180, 204, 7]);
pub const SENSOR_TYPE_ACCELEROMETER_3D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3271233375, 58066, 19576, [188, 208, 53, 42, 149, 130, 129, 157]);
pub const SENSOR_TYPE_AGGREGATED_DEVICE_ORIENTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3451246839, 15613, 16840, [133, 66, 204, 230, 34, 207, 93, 110]);
pub const SENSOR_TYPE_AGGREGATED_QUADRANT_ORIENTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2676093359, 50347, 17159, [153, 4, 200, 40, 191, 185, 8, 41]);
pub const SENSOR_TYPE_AGGREGATED_SIMPLE_DEVICE_ORIENTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2258735761, 1154, 16428, [191, 76, 173, 218, 197, 43, 28, 57]);
pub const SENSOR_TYPE_AMBIENT_LIGHT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2549159368, 22938, 16723, [136, 148, 210, 209, 40, 153, 145, 138]);
pub const SENSOR_TYPE_BARCODE_SCANNER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2567650703, 34235, 17919, [145, 77, 153, 140, 4, 243, 114, 223]);
pub const SENSOR_TYPE_BOOLEAN_SWITCH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2625517343, 4161, 17931, [141, 92, 113, 228, 117, 46, 53, 12]);
pub const SENSOR_TYPE_BOOLEAN_SWITCH_ARRAY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1415351205, 45379, 17733, [134, 143, 202, 127, 217, 134, 180, 246]);
pub const SENSOR_TYPE_CAPACITANCE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3392142108, 8983, 18880, [160, 180, 182, 60, 230, 52, 97, 160]);
pub const SENSOR_TYPE_COMPASS_1D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2752902853, 52048, 18896, [142, 98, 168, 39, 11, 215, 162, 108]);
pub const SENSOR_TYPE_COMPASS_2D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(358964416, 39290, 19760, [132, 219, 87, 202, 186, 54, 72, 187]);
pub const SENSOR_TYPE_COMPASS_3D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1991626253, 6109, 16717, [147, 161, 225, 39, 244, 11, 223, 110]);
pub const SENSOR_TYPE_CURRENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1524408270, 5536, 19390, [161, 173, 45, 56, 169, 174, 131, 28]);
pub const SENSOR_TYPE_CUSTOM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3896177193, 34368, 19736, [162, 19, 226, 38, 117, 235, 178, 195]);
pub const SENSOR_TYPE_DISTANCE_1D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1595190063, 5127, 17158, [169, 63, 177, 219, 171, 228, 249, 192]);
pub const SENSOR_TYPE_DISTANCE_2D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1559864428, 43426, 20053, [182, 161, 160, 74, 175, 169, 90, 146]);
pub const SENSOR_TYPE_DISTANCE_3D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2718740017, 3621, 18290, [159, 229, 150, 96, 138, 19, 84, 178]);
pub const SENSOR_TYPE_ELECTRICAL_POWER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(556732661, 5291, 17270, [154, 67, 167, 121, 64, 152, 194, 254]);
pub const SENSOR_TYPE_ENVIRONMENTAL_ATMOSPHERIC_PRESSURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(244332585, 65418, 19091, [151, 223, 61, 203, 222, 64, 34, 136]);
pub const SENSOR_TYPE_ENVIRONMENTAL_HUMIDITY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1551023975, 48510, 16983, [153, 11, 152, 163, 186, 59, 64, 10]);
pub const SENSOR_TYPE_ENVIRONMENTAL_TEMPERATURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(83693252, 54746, 17914, [149, 169, 93, 179, 142, 225, 147, 6]);
pub const SENSOR_TYPE_ENVIRONMENTAL_WIND_DIRECTION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2666887733, 37638, 17229, [175, 9, 55, 250, 90, 156, 0, 189]);
pub const SENSOR_TYPE_ENVIRONMENTAL_WIND_SPEED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3713032315, 42079, 17101, [142, 253, 236, 97, 118, 28, 66, 38]);
pub const SENSOR_TYPE_FORCE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3265997570, 6684, 18296, [168, 27, 149, 74, 23, 136, 204, 117]);
pub const SENSOR_TYPE_FREQUENCY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2362624950, 29670, 17984, [167, 9, 114, 174, 143, 182, 13, 127]);
pub const SENSOR_TYPE_GYROMETER_1D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4194862900, 62802, 17796, [131, 36, 237, 250, 246, 73, 101, 44]);
pub const SENSOR_TYPE_GYROMETER_2D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(837767043, 37275, 18623, [141, 224, 93, 122, 157, 36, 5, 86]);
pub const SENSOR_TYPE_GYROMETER_3D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(155737946, 30110, 17090, [189, 75, 163, 73, 183, 92, 134, 67]);
pub const SENSOR_TYPE_HUMAN_PRESENCE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3241722155, 44370, 17692, [147, 117, 135, 245, 24, 255, 16, 198]);
pub const SENSOR_TYPE_HUMAN_PROXIMITY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1377884905, 12665, 17456, [159, 144, 6, 38, 109, 42, 52, 222]);
pub const SENSOR_TYPE_INCLINOMETER_1D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3111098565, 31349, 19367, [148, 233, 172, 134, 140, 150, 109, 216]);
pub const SENSOR_TYPE_INCLINOMETER_2D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2870218605, 33771, 16996, [183, 11, 177, 106, 91, 37, 106, 1]);
pub const SENSOR_TYPE_INCLINOMETER_3D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3091798523, 60037, 18806, [132, 68, 111, 111, 92, 109, 49, 219]);
pub const SENSOR_TYPE_INDUCTANCE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3692925759, 50229, 19581, [162, 254, 96, 113, 146, 165, 36, 211]);
pub const SENSOR_TYPE_LOCATION_BROADCAST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3530131663, 20834, 16441, [187, 23, 76, 88, 182, 152, 228, 74]);
pub const SENSOR_TYPE_LOCATION_DEAD_RECKONING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(439866680, 62091, 17114, [159, 206, 169, 208, 162, 166, 216, 41]);
pub const SENSOR_TYPE_LOCATION_GPS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3981223305, 12922, 20473, [165, 96, 145, 218, 75, 72, 39, 94]);
pub const SENSOR_TYPE_LOCATION_LOOKUP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(992915018, 29390, 17261, [150, 210, 60, 91, 133, 112, 233, 135]);
pub const SENSOR_TYPE_LOCATION_OTHER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2603418982, 872, 20337, [184, 141, 83, 63, 19, 32, 49, 222]);
pub const SENSOR_TYPE_LOCATION_STATIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(157254020, 4009, 17477, [142, 110, 183, 15, 50, 11, 107, 76]);
pub const SENSOR_TYPE_LOCATION_TRIANGULATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1763456026, 21510, 20449, [148, 47, 34, 70, 203, 235, 57, 224]);
pub const SENSOR_TYPE_MOTION_DETECTOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1551637010, 12453, 17337, [164, 178, 207, 9, 236, 91, 123, 232]);
pub const SENSOR_TYPE_MULTIVALUE_SWITCH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3018739062, 14244, 17410, [178, 94, 153, 198, 10, 119, 95, 161]);
pub const SENSOR_TYPE_POTENTIOMETER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(724992425, 51932, 17834, [166, 255, 84, 149, 124, 139, 180, 64]);
pub const SENSOR_TYPE_PRESSURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(651370292, 25426, 16847, [183, 147, 234, 7, 19, 213, 61, 119]);
pub const SENSOR_TYPE_RESISTANCE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2576601800, 49495, 19026, [167, 181, 25, 92, 118, 3, 114, 49]);
pub const SENSOR_TYPE_RFID_SCANNER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1144164085, 733, 20109, [173, 93, 146, 73, 131, 43, 46, 202]);
pub const SENSOR_TYPE_SCALE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3228424492, 32747, 17294, [155, 246, 130, 32, 127, 255, 91, 184]);
pub const SENSOR_TYPE_SPEEDOMETER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1809267743, 2996, 17168, [129, 178, 223, 193, 138, 82, 191, 148]);
pub const SENSOR_TYPE_STRAIN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3335646222, 26627, 17249, [173, 61, 133, 188, 197, 140, 109, 41]);
pub const SENSOR_TYPE_TOUCH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(400240664, 1732, 20349, [129, 175, 146, 116, 183, 89, 156, 39]);
pub const SENSOR_TYPE_UNKNOWN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(280658915, 61263, 16877, [152, 133, 168, 125, 100, 53, 168, 225]);
pub const SENSOR_TYPE_VOLTAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3309848119, 20407, 18771, [152, 184, 165, 109, 138, 161, 251, 30]);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::clone::Clone for SENSOR_VALUE_PAIR {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
pub struct SENSOR_VALUE_PAIR {
    pub Key: super::super::System::PropertiesSystem::PROPERTYKEY,
    pub Value: super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
impl SENSOR_VALUE_PAIR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::default::Default for SENSOR_VALUE_PAIR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::cmp::PartialEq for SENSOR_VALUE_PAIR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::cmp::Eq for SENSOR_VALUE_PAIR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
unsafe impl ::windows::runtime::Abi for SENSOR_VALUE_PAIR {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SIMPLE_DEVICE_ORIENTATION(pub i32);
pub const SimpleDeviceOrientation_NotRotated: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(0i32);
pub const SimpleDeviceOrientation_Rotated90DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(1i32);
pub const SimpleDeviceOrientation_Rotated180DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(2i32);
pub const SimpleDeviceOrientation_Rotated270DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(3i32);
pub const SimpleDeviceOrientation_Faceup: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(4i32);
pub const SimpleDeviceOrientation_Facedown: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(5i32);
impl ::std::convert::From<i32> for SIMPLE_DEVICE_ORIENTATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SIMPLE_DEVICE_ORIENTATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const Sensor: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3917278464, 21050, 16691, [191, 111, 211, 162, 218, 231, 246, 186]);
pub const SensorCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2042903259, 42025, 18079, [170, 57, 47, 43, 116, 183, 89, 55]);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn SensorCollectionGetAt(index: u32, psensorslist: *const SENSOR_COLLECTION_LIST, pkey: *mut super::super::System::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SensorCollectionGetAt(index: u32, psensorslist: *const ::std::mem::ManuallyDrop<SENSOR_COLLECTION_LIST>, pkey: *mut super::super::System::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> super::super::Foundation::NTSTATUS;
        }
        SensorCollectionGetAt(::std::mem::transmute(index), ::std::mem::transmute(psensorslist), ::std::mem::transmute(pkey), ::std::mem::transmute(pvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SensorConnectionType(pub i32);
pub const SENSOR_CONNECTION_TYPE_PC_INTEGRATED: SensorConnectionType = SensorConnectionType(0i32);
pub const SENSOR_CONNECTION_TYPE_PC_ATTACHED: SensorConnectionType = SensorConnectionType(1i32);
pub const SENSOR_CONNECTION_TYPE_PC_EXTERNAL: SensorConnectionType = SensorConnectionType(2i32);
impl ::std::convert::From<i32> for SensorConnectionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SensorConnectionType {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SensorDataReport: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1319753455, 26955, 16920, [136, 22, 204, 218, 141, 167, 75, 186]);
pub const SensorManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2007091239, 64722, 18057, [137, 21, 157, 97, 60, 197, 250, 62]);
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SensorState(pub i32);
pub const SENSOR_STATE_MIN: SensorState = SensorState(0i32);
pub const SENSOR_STATE_READY: SensorState = SensorState(0i32);
pub const SENSOR_STATE_NOT_AVAILABLE: SensorState = SensorState(1i32);
pub const SENSOR_STATE_NO_DATA: SensorState = SensorState(2i32);
pub const SENSOR_STATE_INITIALIZING: SensorState = SensorState(3i32);
pub const SENSOR_STATE_ACCESS_DENIED: SensorState = SensorState(4i32);
pub const SENSOR_STATE_ERROR: SensorState = SensorState(5i32);
pub const SENSOR_STATE_MAX: SensorState = SensorState(5i32);
impl ::std::convert::From<i32> for SensorState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SensorState {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Sensors`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn SerializationBufferAllocate(sizeinbytes: u32, pbuffer: *mut *mut u8) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SerializationBufferAllocate(sizeinbytes: u32, pbuffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
        }
        SerializationBufferAllocate(::std::mem::transmute(sizeinbytes), ::std::mem::transmute(pbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[inline]
pub unsafe fn SerializationBufferFree(buffer: *const u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SerializationBufferFree(buffer: *const u8);
        }
        ::std::mem::transmute(SerializationBufferFree(::std::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SimpleDeviceOrientation(pub i32);
pub const SIMPLE_DEVICE_ORIENTATION_NOT_ROTATED: SimpleDeviceOrientation = SimpleDeviceOrientation(0i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_90: SimpleDeviceOrientation = SimpleDeviceOrientation(1i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_180: SimpleDeviceOrientation = SimpleDeviceOrientation(2i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_270: SimpleDeviceOrientation = SimpleDeviceOrientation(3i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_UP: SimpleDeviceOrientation = SimpleDeviceOrientation(4i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_DOWN: SimpleDeviceOrientation = SimpleDeviceOrientation(5i32);
impl ::std::convert::From<i32> for SimpleDeviceOrientation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SimpleDeviceOrientation {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Sensors`*"]
pub struct VEC3D {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
impl VEC3D {}
impl ::std::default::Default for VEC3D {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VEC3D {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VEC3D").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
impl ::std::cmp::PartialEq for VEC3D {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
impl ::std::cmp::Eq for VEC3D {}
unsafe impl ::windows::runtime::Abi for VEC3D {
    type Abi = Self;
    type DefaultType = Self;
}
