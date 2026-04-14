/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    SensorsDef.h

Abstract:

    Sensors definitions header file

--*/

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef __midl
#include <stdio.h>
#include <stdlib.h>
#include <initguid.h>
#include <propidl.h>
#include <propkeydef.h>
#include <devpropdef.h>
#include <propvarutil.h>
#endif

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#ifndef __midl

/////////////////////////////////////////////////////////////////////
// Sensor Categories
//

// {C317C286-C468-4288-9975-D4C4587C442C}
DEFINE_GUID(GUID_SensorCategory_All, 0XC317C286, 0XC468, 0X4288, 0X99, 0X75, 0XD4, 0XC4, 0X58, 0X7C, 0X44, 0X2C);
// {CA19690F-A2C7-477D-A99E-99EC6E2B5648}
DEFINE_GUID(GUID_SensorCategory_Biometric, 0XCA19690F, 0XA2C7, 0X477D, 0XA9, 0X9E, 0X99, 0XEC, 0X6E, 0X2B, 0X56, 0X48);
// {FB73FCD8-FC4A-483C-AC58-27B691C6BEFF}
DEFINE_GUID(GUID_SensorCategory_Electrical, 0XFB73FCD8, 0XFC4A, 0X483C, 0XAC, 0X58, 0X27, 0XB6, 0X91, 0XC6, 0XBE, 0XFF);
// {323439AA-7F66-492B-BA0C-73E9AA0A65D5}
DEFINE_GUID(GUID_SensorCategory_Environmental, 0X323439AA, 0X7F66, 0X492B, 0XBA, 0X0C, 0X73, 0XE9, 0XAA, 0X0A, 0X65, 0XD5);
// {17A665C0-9063-4216-B202-5C7A255E18CE}
DEFINE_GUID(GUID_SensorCategory_Light, 0X17A665C0, 0X9063, 0X4216, 0XB2, 0X02, 0X5C, 0X7A, 0X25, 0X5E, 0X18, 0XCE);
// {BFA794E4-F964-4FDB-90F6-51056BFE4B44}
DEFINE_GUID(GUID_SensorCategory_Location, 0XBFA794E4, 0XF964, 0X4FDB, 0X90, 0XF6, 0X51, 0X05, 0X6B, 0XFE, 0X4B, 0X44);
// {8D131D68-8EF7-4656-80B5-CCCBD93791C5}
DEFINE_GUID(GUID_SensorCategory_Mechanical, 0X8D131D68, 0X8EF7, 0X4656, 0X80, 0XB5, 0XCC, 0XCB, 0XD9, 0X37, 0X91, 0XC5);
// {CD09DAF1-3B2E-4C3D-B598-B5E5FF93FD46}
DEFINE_GUID(GUID_SensorCategory_Motion, 0XCD09DAF1, 0X3B2E, 0X4C3D, 0XB5, 0X98, 0XB5, 0XE5, 0XFF, 0X93, 0XFD, 0X46);
// {9E6C04B6-96FE-4954-B726-68682A473F69}
DEFINE_GUID(GUID_SensorCategory_Orientation, 0X9E6C04B6, 0X96FE, 0X4954, 0XB7, 0X26, 0X68, 0X68, 0X2A, 0X47, 0X3F, 0X69);
// {2C90E7A9-F4C9-4FA2-AF37-56D471FE5A3D}
DEFINE_GUID(GUID_SensorCategory_Other, 0x2C90E7A9, 0xF4C9, 0x4FA2, 0xAF, 0x37, 0x56, 0xD4, 0x71, 0xFE, 0x5A, 0x3D);
// {F1609081-1E12-412B-A14D-CBB0E95BD2E5}
DEFINE_GUID(GUID_SensorCategory_PersonalActivity, 0XF1609081, 0X1E12, 0X412B, 0XA1, 0X4D, 0XCB, 0XB0, 0XE9, 0X5B, 0XD2, 0XE5);
// {B000E77E-F5B5-420F-815D-0270A726F270}
DEFINE_GUID(GUID_SensorCategory_Scanner, 0XB000E77E, 0XF5B5, 0X420F, 0X81, 0X5D, 0X02, 0X70, 0XA7, 0X26, 0XF2, 0X70);
// {2BEAE7FA-19B0-48C5-A1F6-B5480DC206B0}
DEFINE_GUID(GUID_SensorCategory_Unsupported, 0x2BEAE7fA, 0x19B0, 0x48C5, 0xA1, 0xF6, 0xB5, 0x48, 0x0D, 0xC2, 0x06, 0xB0);


/////////////////////////////////////////////////////////////////////
// Sensor Types
//

// {C2FB0F5F-E2D2-4C78-BCD0-352A9582819D}
DEFINE_GUID(GUID_SensorType_Accelerometer3D, 0XC2FB0F5F, 0XE2D2, 0X4C78, 0XBC, 0XD0, 0X35, 0X2A, 0X95, 0X82, 0X81, 0X9D);
// {9D9E0118-1807-4F2E-96E4-2CE57142E196}
DEFINE_GUID(GUID_SensorType_ActivityDetection, 0X9D9E0118, 0X1807, 0X4F2E, 0X96, 0XE4, 0X2C, 0XE5, 0X71, 0X42, 0XE1, 0X96);
// {97F115C8-599A-4153-8894-D2D12899918A}
DEFINE_GUID(GUID_SensorType_AmbientLight, 0X97F115C8, 0X599A, 0X4153, 0X88, 0X94, 0XD2, 0XD1, 0X28, 0X99, 0X91, 0X8A);
// {0E903829-FF8A-4A93-97DF-3DCBDE402288}
DEFINE_GUID(GUID_SensorType_Barometer, 0X0E903829, 0XFF8A, 0X4A93, 0X97, 0XDF, 0X3D, 0XCB, 0XDE, 0X40, 0X22, 0X88);
// {E83AF229-8640-4D18-A213-E22675EBB2C3}
DEFINE_GUID(GUID_SensorType_Custom, 0XE83AF229, 0X8640, 0X4D18, 0XA2, 0X13, 0XE2, 0X26, 0X75, 0XEB, 0XB2, 0XC3);
// {ADE4987F-7AC4-4DFA-9722-0A027181C747}
DEFINE_GUID(GUID_SensorType_FloorElevation, 0xADE4987F, 0x7AC4, 0x4DFA, 0x97, 0x22, 0x0A, 0x02, 0x71, 0x81, 0xC7, 0x47);
// {E77195F8-2D1F-4823-971B-1C4467556C9D}
DEFINE_GUID(GUID_SensorType_GeomagneticOrientation, 0XE77195F8, 0X2D1F, 0X4823, 0X97, 0X1B, 0X1C, 0X44, 0X67, 0X55, 0X6C, 0X9D);
// {03B52C73-BB76-463F-9524-38DE76EB700B}
DEFINE_GUID(GUID_SensorType_GravityVector, 0X03B52C73, 0XBB76, 0X463F, 0X95, 0X24, 0X38, 0XDE, 0X76, 0XEB, 0X70, 0X0B);
// {09485F5A-759E-42C2-BD4B-A349B75C8643}
DEFINE_GUID(GUID_SensorType_Gyrometer3D, 0X09485F5A, 0X759E, 0X42C2, 0XBD, 0X4B, 0XA3, 0X49, 0XB7, 0X5C, 0X86, 0X43);
// {5C72BF67-BD7E-4257-990B-98A3BA3B400A}
DEFINE_GUID(GUID_SensorType_Humidity, 0X5C72BF67, 0XBD7E, 0X4257, 0X99, 0X0B, 0X98, 0XA3, 0XBA, 0X3B, 0X40, 0X0A);
// {038B0283-97B4-41C8-BC24-5FF1AA48FEC7}
DEFINE_GUID(GUID_SensorType_LinearAccelerometer, 0X038B0283, 0X97B4, 0X41C8, 0XBC, 0X24, 0X5F, 0XF1, 0XAA, 0X48, 0XFE, 0XC7);
// {55E5EFFB-15C7-40df-8698-A84B7C863C53}
DEFINE_GUID(GUID_SensorType_Magnetometer3D, 0X55E5EFFB, 0X15C7, 0X40DF, 0X86, 0X98, 0XA8, 0X4B, 0X7C, 0X86, 0X3C, 0X53);
// {CDB5D8F7-3CFD-41C8-8542-CCE622CF5D6E}
DEFINE_GUID(GUID_SensorType_Orientation, 0XCDB5D8F7, 0X3CFD, 0X41C8, 0X85, 0X42, 0XCC, 0XE6, 0X22, 0XCF, 0X5D, 0X6E);
// {B19F89AF-E3EB-444B-8DEA-202575A71599}
DEFINE_GUID(GUID_SensorType_Pedometer, 0XB19F89AF, 0XE3EB, 0X444B, 0X8D, 0XEA, 0X20, 0X25, 0X75, 0XA7, 0X15, 0X99);
// {5220DAE9-3179-4430-9F90-06266D2A34DE}
DEFINE_GUID(GUID_SensorType_Proximity, 0X5220DAE9, 0X3179, 0X4430, 0X9F, 0X90, 0X06, 0X26, 0X6D, 0X2A, 0X34, 0XDE);
// {40993B51-4706-44DC-98D5-C920C037FFAB}
DEFINE_GUID(GUID_SensorType_RelativeOrientation, 0x40993b51, 0x4706, 0x44dc, 0x98, 0xd5, 0xc9, 0x20, 0xc0, 0x37, 0xff, 0xab);
// {86A19291-0482-402C-BF4C-ADDAC52B1C39}
DEFINE_GUID(GUID_SensorType_SimpleDeviceOrientation, 0X86A19291, 0X0482, 0X402C, 0XBF, 0X4C, 0XAD, 0XDA, 0XC5, 0X2B, 0X1C, 0X39);
// {04FD0EC4-D5DA-45FA-95A9-5DB38EE19306}
DEFINE_GUID(GUID_SensorType_Temperature, 0X04FD0EC4, 0XD5DA, 0X45FA, 0X95, 0XA9, 0X5D, 0XB3, 0X8E, 0XE1, 0X93, 0X06);
// {82358065-F4C4-4DA1-B272-13C23332A207}
DEFINE_GUID(GUID_SensorType_HingeAngle, 0x82358065, 0xf4c4, 0x4da1, 0xb2, 0x72, 0x13, 0xc2, 0x33, 0x32, 0xa2, 0x7);


////////////////////////////////////////////////////////////////////
// Properties
//

// {D4247382-969D-4F24-BB14-FB9671870BBF}

// Sensor Enumeration Properties (2-19)
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_Type,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 2);  //[VT_CLSID]
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_Category,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 3);  //[VT_CLSID]
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_ConnectionType, // Possible values for this key are defined in the SENSOR_CONNECTION_TYPES enum below
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 4);  //[VT_UI4]
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_IsPrimary,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 5);  //[VT_BOOL]
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_Name,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 6);  //[VT_LPWSTR]
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_Manufacturer,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 7);  //[VT_LPWSTR]
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_Model,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 8);  //[VT_LPWSTR]
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_PersistentUniqueId,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 9);  //[VT_CLSID]
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_VendorDefinedSubType,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 10);  //[VT_CLSID]

// Common Sensor Properties (2, 20-29)
DEFINE_PROPERTYKEY(PKEY_Sensor_Type,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 2);  //[VT_CLSID]
DEFINE_PROPERTYKEY(PKEY_Sensor_State,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 20); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_Sensor_MinimumDataInterval_Ms,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 21); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_Sensor_MaximumDataFieldSize_Bytes,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 22); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_Sensor_Power_Milliwatts,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 23); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorHistory_MaxSize_Bytes,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 24); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorHistory_Interval_Ms,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 25); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorHistory_MaximumRecordSize_Bytes,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 26); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_Sensor_FifoReservedSize_Samples,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 27); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_Sensor_FifoMaxSize_Samples,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 28); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_Sensor_WakeCapable,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 29); //[VT_BOOL]

// Light Sensor Property (30-39)
DEFINE_PROPERTYKEY(PKEY_LightSensor_ResponseCurve,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 30); //[VT_VECTOR | VT_UI4]
DEFINE_PROPERTYKEY(DEVPKEY_LightSensor_AutoBrightnessPreferred,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 31); //[VT_BOOL]
DEFINE_PROPERTYKEY(DEVPKEY_LightSensor_ColorCapable,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 32); //[VT_BOOL]
DEFINE_PROPERTYKEY(DEVPKEY_LightSensor_AdaptiveColorPreferred,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 33); //[VT_BOOL]

// Orientation Sensor Property (40-49)
DEFINE_PROPERTYKEY(PKEY_OrientationSensor_GyroscopeUsed,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 40); //[VT_BOOL]

// Data-Field Property (50-59)
DEFINE_PROPERTYKEY(PKEY_SensorDataField_Resolution,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 50);
DEFINE_PROPERTYKEY(PKEY_SensorDataField_RangeMinimum,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 51);
DEFINE_PROPERTYKEY(PKEY_SensorDataField_RangeMaximum,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 52);

// Activity Sensor Property (60-69)
DEFINE_PROPERTYKEY(PKEY_SensorData_SupportedActivityStates,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 60); //[VT_UI4]

DEFINE_PROPERTYKEY(PKEY_SensorData_MinimumDetectionIntervals_Ms,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 61); //[VT_VECTOR|VT_UI4]

// Pedometer Sensor Property (70-79)
DEFINE_PROPERTYKEY(PKEY_SensorData_SupportedStepTypes,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 70); //[VT_UI4]

// Proximity Sensor Property (80-89)
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_ProximityType,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 80); //[VT_UI4]
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_HumanPresenceDetectionType,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 81); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_Sensor_Proximity_SensorCapabilities,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 82); //[VT_UI4]
DEFINE_PROPERTYKEY(DEVPKEY_Sensor_HumanPresence_MaxDetectablePersonsCount,
    0xd4247382, 0x969d, 0x4f24, 0xbb, 0x14, 0xfb, 0x96, 0x71, 0x87, 0xb, 0xbf, 83); //[VT_UI4]

//////////////////////////////////////////////////////////////
// Data-Fields
//

// {C458F8A7-4AE8-4777-9607-2E9BDD65110A}

// Common Data-Fields (2-9)
DEFINE_PROPERTYKEY(PKEY_SensorData_Timestamp,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 2);  //[VT_FILETIME]
DEFINE_PROPERTYKEY(PKEY_SensorData_IsValid,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 3);  //[VT_BOOL]
DEFINE_PROPERTYKEY(PKEY_SensorData_TimestampQPC,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 4);  //[VT_UI8]


// Accelerometer Data-Fields (10-19)
DEFINE_PROPERTYKEY(PKEY_SensorData_AccelerationX_Gs,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 10); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_AccelerationY_Gs,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 11); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_AccelerationZ_Gs,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 12); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_Shake,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 33); //[VT_BOOL]

// Magnetometer Data-Fields (20-29)
DEFINE_PROPERTYKEY(PKEY_SensorData_MagneticFieldStrengthX_Microteslas,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 20); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_MagneticFieldStrengthY_Microteslas,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 21); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_MagneticFieldStrengthZ_Microteslas,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 22); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_MagnetometerAccuracy,        // Possible values for this key are defined in MAGNETOMETER_ACCURACY enum below
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 23); //[VT_UI4]

// Gyrometer Data-Fields (40-49)
DEFINE_PROPERTYKEY(PKEY_SensorData_AngularVelocityX_DegreesPerSecond,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 40); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_AngularVelocityY_DegreesPerSecond,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 41); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_AngularVelocityZ_DegreesPerSecond,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 42); //[VT_R4]

// Light Sensor Data-Fields (60-69)
DEFINE_PROPERTYKEY(PKEY_SensorData_LightLevel_Lux,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 60); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_LightTemperature_Kelvins,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 61); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_LightChromaticityX,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 62); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_LightChromaticityY,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 63); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_LightLevel_Lux_Threshold_AbsoluteDifference,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 64); //[VT_R4]

// Device Orientation Sensor Data-Fields (70-89)
DEFINE_PROPERTYKEY(PKEY_SensorData_QuaternionW,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 70); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_QuaternionX,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 71); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_QuaternionY,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 72); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_QuaternionZ,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 73); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_DeclinationAngle_Degrees,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 74); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_LinearAccelerationX_Gs,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 75); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_LinearAccelerationY_Gs,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 76); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_LinearAccelerationZ_Gs,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 77); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CorrectedAngularVelocityX_DegreesPerSecond,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 78); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CorrectedAngularVelocityY_DegreesPerSecond,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 79); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CorrectedAngularVelocityZ_DegreesPerSecond,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 80); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_RotationAngle_Degrees,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 81); //[VT_R4]

// Proximity Sensor Data-Fields (90-99)
DEFINE_PROPERTYKEY(PKEY_SensorData_ProximityDetection,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 90); //[VT_BOOL]
DEFINE_PROPERTYKEY(PKEY_SensorData_ProximityDistanceMillimeters,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 91); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_DetectionDistance_Threshold,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 92); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_AttentionDetection,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 93); //[VT_BOOL]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_HeadAzimuth,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 94); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_HeadAltitude,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 95); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_HeadRoll,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 96); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_HeadPitch,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 97); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_HeadYaw,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 98); //[VT_R4]

// Environmental Sensor Data-Fields (100-109)
DEFINE_PROPERTYKEY(PKEY_SensorData_AtmosphericPressure_Bars,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 100); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_Temperature_Celsius,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 101); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_RelativeHumidity_Percent,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 102); //[VT_R4]

// Simple Device Orientation Sensor Data-Fields (120-129)
DEFINE_PROPERTYKEY(PKEY_SensorData_SimpleDeviceOrientation,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 120); //[VT_UI4]

// Activity Detection Sensor Data-Fields (130-139)
DEFINE_PROPERTYKEY(PKEY_SensorData_CurrentActivityState, // Possible values for this key are defined in ACTIVITY_STATES enum below
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 130); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CurrentActivityStateConfidence_Percentage,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 131); //[VT_UI2]
DEFINE_PROPERTYKEY(PKEY_SensorData_SubscribedActivityStates, // Possible values for this key are defined in ACTIVITY_STATES enum below
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 132); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_ActivityStream,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 133); //[VT_BOOL]
DEFINE_PROPERTYKEY(PKEY_SensorData_ConfidenceThreshold_Percentage,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 134); //[VT_UI2]

// Pedometer Sensor Data-Fields (140-149)
DEFINE_PROPERTYKEY(PKEY_SensorData_PedometerStepType, // Possible values for this key are defined in PEDOMETER_STEP_TYPE enum below
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 140); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_PedometerStepCount,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 141); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_PedometerStepDuration_Ms,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 142); //[VT_I8]
DEFINE_PROPERTYKEY(PKEY_SensorData_PedometerReset,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 143); //[VT_BOOL]

// Custom Sensor Data-Fields (160-199)
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue0,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 160); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue1,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 161); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue2,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 162); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue3,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 163); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue4,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 164); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue5,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 165); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue6,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 166); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue7,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 167); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue8,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 168); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue9,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 169); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue10,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 170); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue11,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 171); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue12,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 172); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue13,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 173); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue14,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 174); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue15,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 175); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue16,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 176); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue17,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 177); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue18,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 178); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue19,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 179); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue20,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 180); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue21,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 181); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue22,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 182); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue23,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 183); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue24,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 184); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue25,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 185); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue26,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 186); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue27,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 187); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CustomValue28,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 188); //[VT_UI4]

// Floor Elevation Sensor Data-Fields (200-209)
DEFINE_PROPERTYKEY(PKEY_SensorData_ElevationChangeMode, // Possible values for this key are defined in ELEVATION_CHANGE_MODE enum below
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 200); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CumulativeElevation_Meters,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 201); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CumulativeElevationFloorCount,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 202); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_CumulativeElevationStepCount,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 203); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_ElevationChange_Meters,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 204); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_VerticalSpeed_MetersPerSecond,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 205); //[VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_ElevationChangeFloorCount,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 206); //[VT_UI4]

// Human Presence Sensor Multi-Person Detection Data-Fields (210-230)
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_DetectedPersonsCount,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 210); //[VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_DistanceMillimetersVector,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 211); //[VT_VECTOR|VT_UI4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_AttentionVector,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 212); //[VT_VECTOR|VT_BOOL]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_HeadAzimuthVector,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 213); //[VT_VECTOR|VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_HeadAltitudeVector,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 214); //[VT_VECTOR|VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_HeadRollVector,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 215); //[VT_VECTOR|VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_HeadPitchVector,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 216); //[VT_VECTOR|VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_HeadYawVector,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 217); //[VT_VECTOR|VT_R4]
DEFINE_PROPERTYKEY(PKEY_SensorData_HumanPresence_PersonIdVector,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 218); //[VT_VECTOR|VT_UI4]

// Hinge Angle Sensor Data Fields
DEFINE_PROPERTYKEY(PKEY_SensorData_HingeAngle,
    0xc458f8a7, 0x4ae8, 0x4777, 0x96, 0x7, 0x2e, 0x9b, 0xdd, 0x65, 0x11, 0xa, 300); //[VT_R4]

#endif //__midl

typedef enum ACTIVITY_STATE_COUNT
{
    ActivityStateCount = 8,
} ACTIVITY_STATE_COUNT;

typedef enum ACTIVITY_STATE
{
    ActivityState_Unknown = 0x00000001,
    ActivityState_Stationary = 0x00000002,
    ActivityState_Fidgeting = 0x00000004,
    ActivityState_Walking = 0x00000008,
    ActivityState_Running = 0x00000010,
    ActivityState_InVehicle = 0x00000020,
    ActivityState_Biking = 0x00000040,
    ActivityState_Idle = 0x00000080,
    ActivityState_Max = 1 << ActivityStateCount,

    ActivityState_Force_Dword = 0xFFFFFFFF // Make sure the enum is 32bit
} ACTIVITY_STATE;

typedef enum ELEVATION_CHANGE_MODE
{
    ElevationChangeMode_Unknown = 0,
    ElevationChangeMode_Elevator,
    ElevationChangeMode_Stepping,
    ElevationChangeMode_Max,

    ElevationChangeMode_Force_Dword = 0xFFFFFFFF // Make sure the enum is 32bit
} ELEVATION_CHANGE_MODE;

typedef enum MAGNETOMETER_ACCURACY
{
    MagnetometerAccuracy_Unknown = 0,
    MagnetometerAccuracy_Unreliable,
    MagnetometerAccuracy_Approximate,
    MagnetometerAccuracy_High
} MAGNETOMETER_ACCURACY;

// Represents the number of step types in the PEDOMETER_STEP_TYPE enum below
typedef enum PEDOMETER_STEP_TYPE_COUNT
{
    PedometerStepTypeCount = 3,
} PEDOMETER_STEP_TYPE_COUNT;

typedef enum PEDOMETER_STEP_TYPE
{
    PedometerStepType_Unknown = 0x00000001,
    PedometerStepType_Walking = 0x00000002,
    PedometerStepType_Running = 0x00000004,
    PedometerStepType_Max = 1 << PedometerStepTypeCount,

    PedometerStepType_Force_Dword = 0xFFFFFFFF // Make sure the enum is 32bit
} PEDOMETER_STEP_TYPE;

typedef enum PROXIMITY_TYPE
{
    ProximityType_ObjectProximity = 0,
    ProximityType_HumanProximity = 1,
    ProximityType_Force_Dword = 0xFFFFFFFF
} PROXIMITY_TYPE;

// Represents the number of human presence detection types in the HUMAN_PRESENCE_DETECTION_TYPE enum below
typedef enum HUMAN_PRESENCE_DETECTION_TYPE_COUNT
{
    HumanPresenceDetectionTypeCount = 4,
} HUMAN_PRESENCE_DETECTION_TYPE_COUNT;

typedef enum HUMAN_PRESENCE_DETECTION_TYPE
{
    HumanPresenceDetectionType_Undefined                    = 0x00000000,
    HumanPresenceDetectionType_VendorDefinedNonBiometric    = 0x00000001,
    HumanPresenceDetectionType_VendorDefinedBiometric       = 0x00000002,
    HumanPresenceDetectionType_FacialBiometric              = 0x00000004,
    HumanPresenceDetectionType_AudioBiometric               = 0x00000008,
    HumanPresenceDetectionType_Force_Dword                  = 0xFFFFFFFF // Make sure the enum is 32bit
} HUMAN_PRESENCE_DETECTION_TYPE;

// This enum needs to be updated as new Sensor Capabilities are added 
// and Proximity_Sensor_Supported_Capabilities must advertise this accordingly.
typedef enum PROXIMITY_SENSOR_CAPABILITIES
{
    Proximity_Sensor_Human_Presence_Capable         = 0x0001,
    Proximity_Sensor_Human_Engagement_Capable       = 0x0002,
    Proximity_Sensor_Human_Head_Azimuth_Capable     = 0x0004,
    Proximity_Sensor_Human_Head_Altitude_Capable    = 0x0008,
    Proximity_Sensor_Human_Head_Roll_Capable        = 0x0010,
    Proximity_Sensor_Human_Head_Pitch_Capable       = 0x0020,
    Proximity_Sensor_Human_Head_Yaw_Capable         = 0x0040,
    Proximity_Sensor_Human_Identification_Capable   = 0x0080,
    Proximity_Sensor_Multi_Person_Detection_Capable = 0x0100,
    Proximity_Sensor_Supported_Capabilities         = 0x01FF
} PROXIMITY_SENSOR_CAPABILITIES;

typedef enum SIMPLE_DEVICE_ORIENTATION
{
    SimpleDeviceOrientation_NotRotated = 0,
    SimpleDeviceOrientation_Rotated90DegreesCounterclockwise = 1,
    SimpleDeviceOrientation_Rotated180DegreesCounterclockwise = 2,
    SimpleDeviceOrientation_Rotated270DegreesCounterclockwise = 3,
    SimpleDeviceOrientation_Faceup = 4,
    SimpleDeviceOrientation_Facedown = 5,
} SIMPLE_DEVICE_ORIENTATION;

typedef enum SENSOR_STATE
{
    SensorState_Initializing = 0,
    SensorState_Idle,
    SensorState_Active,
    SensorState_Error,
} SENSOR_STATE;

typedef enum SENSOR_CONNECTION_TYPES
{
    SensorConnectionType_Integrated = 0,
    SensorConnectionType_Attached,
    SensorConnectionType_External
} SENSOR_CONNECTION_TYPES;

typedef struct SENSOR_VALUE_PAIR
{
    PROPERTYKEY Key;
    PROPVARIANT Value;
} SENSOR_VALUE_PAIR, *PSENSOR_VALUE_PAIR;

// This list contains the contains a sensors properties and their values
typedef struct SENSOR_COLLECTION_LIST
{
#ifndef __midl
    _Field_range_(>, SENSOR_COLLECTION_LIST_HEADER_SIZE)
#endif
    ULONG AllocatedSizeInBytes; // Size of structure
#ifndef __midl
    _Field_range_(<=, (AllocatedSizeInBytes - SENSOR_COLLECTION_LIST_HEADER_SIZE) / sizeof(SENSOR_VALUE_PAIR))
#endif
    ULONG Count;                // Number of total elements populated
#ifndef __midl
    _Field_size_bytes_(AllocatedSizeInBytes - SENSOR_COLLECTION_LIST_HEADER_SIZE)
#endif
    SENSOR_VALUE_PAIR List[1];  // Variable sized buffer, first element is a placeholder and may not reflect actual size
} SENSOR_COLLECTION_LIST, *PSENSOR_COLLECTION_LIST;

// This list contains ONLY the PROPERTYKEY's associated with a particular sensor,
// not the property values
typedef struct SENSOR_PROPERTY_LIST
{
#ifndef __midl
    _Field_range_(>, SENSOR_PROPERTY_LIST_HEADER_SIZE)
#endif
    ULONG AllocatedSizeInBytes; // Size of structure
#ifndef __midl
    _Field_range_(<=, (AllocatedSizeInBytes - SENSOR_PROPERTY_LIST_HEADER_SIZE) / sizeof(PROPERTYKEY))
#endif
    ULONG Count;                // Number of total elements populated
#ifndef __midl
    _Field_size_bytes_(AllocatedSizeInBytes - SENSOR_PROPERTY_LIST_HEADER_SIZE)
#endif
    PROPERTYKEY List[1];        // Variable sized buffer, first element is a placeholder and may not reflect actual size
} SENSOR_PROPERTY_LIST, *PSENSOR_PROPERTY_LIST;

#ifndef _CONTRACT_GEN // Exclude implementations from contract file generation.

static const ULONG SENSOR_COLLECTION_LIST_HEADER_SIZE =
    (sizeof(SENSOR_COLLECTION_LIST) - sizeof(SENSOR_VALUE_PAIR));

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef __midl
VOID
FORCEINLINE
SENSOR_COLLECTION_LIST_INIT(
    _Out_writes_bytes_(CollectionListSize)
    _Post_satisfies_(pCollectionList->AllocatedSizeInBytes == CollectionListSize) 
    PSENSOR_COLLECTION_LIST pCollectionList,
    _In_ _Pre_satisfies_(SENSOR_COLLECTION_LIST_HEADER_SIZE <= CollectionListSize) ULONG CollectionListSize
    )
{
    // Problematic if the pCollectionList buffer is too small
    if (SENSOR_COLLECTION_LIST_HEADER_SIZE > CollectionListSize)
    {
        DbgRaiseAssertionFailure();
    }

    RtlZeroMemory(pCollectionList, CollectionListSize);
    pCollectionList->AllocatedSizeInBytes = CollectionListSize;
    pCollectionList->Count = 0;
}

_Ret_range_(==, SENSOR_COLLECTION_LIST_HEADER_SIZE + sizeof(SENSOR_VALUE_PAIR) * Count)
ULONG
FORCEINLINE
SENSOR_COLLECTION_LIST_SIZE(
    _In_ ULONG Count
    )
{
    return SENSOR_COLLECTION_LIST_HEADER_SIZE + sizeof(SENSOR_VALUE_PAIR) * Count;
}

ULONG
FORCEINLINE
SENSOR_COLLECTION_LIST_CALCULATE_MAX_COUNT(
    _In_ PSENSOR_COLLECTION_LIST pCollectionList
    )
{
    if (NULL == pCollectionList ||
        pCollectionList->AllocatedSizeInBytes < SENSOR_COLLECTION_LIST_HEADER_SIZE)
    {
        return 0;
    }
    else
    {
        return ((pCollectionList->AllocatedSizeInBytes - SENSOR_COLLECTION_LIST_HEADER_SIZE) /
            sizeof(SENSOR_VALUE_PAIR));
    }
}
#endif // __midl
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#pragma deprecated(SENSOR_COLLECTION_LIST_CALCULATE_MAX_COUNT) // Use CollectionsListGetFillableCount instead.
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

static const ULONG SENSOR_PROPERTY_LIST_HEADER_SIZE =
    (sizeof(SENSOR_PROPERTY_LIST) - sizeof(PROPERTYKEY));

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef __midl
VOID
FORCEINLINE
SENSOR_PROPERTY_LIST_INIT(
    _Out_writes_bytes_(PropertyListSize) PSENSOR_PROPERTY_LIST pPropertyList,
    _In_ _Pre_satisfies_(SENSOR_PROPERTY_LIST_HEADER_SIZE <= PropertyListSize) ULONG PropertyListSize
    )
{
    // Problematic if the pPropertyList buffer is too small
    if (SENSOR_PROPERTY_LIST_HEADER_SIZE > PropertyListSize)
    {
        DbgRaiseAssertionFailure();
    }

    RtlZeroMemory(pPropertyList, PropertyListSize);
    pPropertyList->AllocatedSizeInBytes = PropertyListSize;
    pPropertyList->Count = 0;
}

_Ret_range_(== , SENSOR_PROPERTY_LIST_HEADER_SIZE + sizeof(PROPERTYKEY) * Count)
ULONG
FORCEINLINE
SENSOR_PROPERTY_LIST_SIZE(
    _In_ ULONG Count
    )
{
    return SENSOR_PROPERTY_LIST_HEADER_SIZE + sizeof(PROPERTYKEY) * Count;
}

ULONG
FORCEINLINE
SENSOR_PROPERTY_LIST_CALCULATE_MAX_COUNT(
    _In_ PSENSOR_PROPERTY_LIST pPropertyList
    )
{
    if (NULL == pPropertyList ||
        pPropertyList->AllocatedSizeInBytes < SENSOR_PROPERTY_LIST_HEADER_SIZE)
    {
        return 0;
    }
    else
    {
        return ((pPropertyList->AllocatedSizeInBytes - SENSOR_PROPERTY_LIST_HEADER_SIZE) /
            sizeof(PROPERTYKEY));
    }
}
#endif //__midl
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#endif // _CONTRACT_GEN

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#pragma deprecated(SENSOR_PROPERTY_LIST_CALCULATE_MAX_COUNT) // Use PropertiesListGetFillableCount instead.
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#ifdef __cplusplus
}
#endif

