/*++

Copyright (C) Microsoft Corporation. All rights reserved.

    THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY
    KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
    IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR
    PURPOSE.

Module Name:

    GnssDriver.h

Abstract:

    Defines the interfaces, IOCTL codes and data structures for the GNSS device driver

Environment:

    User mode or Kernel mode

--*/
#pragma once

#if (NTDDI_VERSION >= NTDDI_WINBLUE) /* ABRACADABRA_THRESHOLD */

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#pragma warning(push)
#pragma warning(disable : 4201) // Disable compiler warning C4201: nameless struct/union

//
// Current DDK Version (Major):     Version 6
//
// Changes in Version 6 (requires GNSS Driver to compile with this version of the DDK header):
//                      - Support additional accuracy and higher precision data
//
// Changes in Version 5 (requires GNSS Driver to compile with this version of the DDK header):
//                      - Added service indicator in GNSS_SUPL_VERSION interface
//
// Changes in Version 4 (requires GNSS Driver to compile with this version of the DDK header):
//                      - Added breadcrumbing interface
//
// Changes in Version 3 (requires GNSS Driver to compile with this version of the DDK header):
//                      - Removed unused fields
//                      - Support for Galileo
//                      - Support for driver to get out-of-band data
//
// Changes in Version 2 (requires GNSS Driver to compile with this version of the DDK header):
//                      - Geofence Support
//                      - Support for Beidou
//                      - SUPL support enhancements (Multiple Root Certificates, Emergency Call Location Support)
//
//
// Previous DDK Versions :          Version 1
//                                  Version 2
//                                  Version 3
//                                  Version 4
//                                  Version 5
//

//
// Define Driver Versions
//

#define GNSS_DRIVER_VERSION_1 1
#define GNSS_DRIVER_VERSION_2 2
#define GNSS_DRIVER_VERSION_3 3
#define GNSS_DRIVER_VERSION_4 4
#define GNSS_DRIVER_VERSION_5 5
#define GNSS_DRIVER_VERSION_6 6

//
// Define GNSS IOCTL Codes
//

#define IOCTL_GNSS_SEND_PLATFORM_CAPABILITY CTL_CODE(FILE_DEVICE_UNKNOWN, 0x001, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_GET_DEVICE_CAPABILITY CTL_CODE(FILE_DEVICE_UNKNOWN, 0x002, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_SEND_DRIVERCOMMAND CTL_CODE(FILE_DEVICE_UNKNOWN, 0x003, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_START_FIXSESSION CTL_CODE(FILE_DEVICE_UNKNOWN, 0x010, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_MODIFY_FIXSESSION CTL_CODE(FILE_DEVICE_UNKNOWN, 0x011, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_STOP_FIXSESSION CTL_CODE(FILE_DEVICE_UNKNOWN, 0x012, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_GET_FIXDATA CTL_CODE(FILE_DEVICE_UNKNOWN, 0x013, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_INJECT_AGNSS CTL_CODE(FILE_DEVICE_UNKNOWN, 0x020, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_LISTEN_AGNSS CTL_CODE(FILE_DEVICE_UNKNOWN, 0x030, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_LISTEN_ERROR CTL_CODE(FILE_DEVICE_UNKNOWN, 0x031, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_LISTEN_NI CTL_CODE(FILE_DEVICE_UNKNOWN, 0x040, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_SET_SUPL_HSLP CTL_CODE(FILE_DEVICE_UNKNOWN, 0x041, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_CONFIG_SUPL_CERT CTL_CODE(FILE_DEVICE_UNKNOWN, 0x042, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_RESPOND_NI CTL_CODE(FILE_DEVICE_UNKNOWN, 0x043, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_EXECUTE_CWTEST CTL_CODE(FILE_DEVICE_UNKNOWN, 0x044, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_EXECUTE_SELFTEST CTL_CODE(FILE_DEVICE_UNKNOWN, 0x045, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_GET_CHIPSETINFO CTL_CODE(FILE_DEVICE_UNKNOWN, 0x046, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_LISTEN_NMEA CTL_CODE(FILE_DEVICE_UNKNOWN, 0x047, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_SET_V2UPL_CONFIG CTL_CODE(FILE_DEVICE_UNKNOWN, 0x048, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_GNSS_CREATE_GEOFENCE CTL_CODE(FILE_DEVICE_UNKNOWN, 0x050, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_DELETE_GEOFENCE CTL_CODE(FILE_DEVICE_UNKNOWN, 0x051, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_LISTEN_GEOFENCE_ALERT CTL_CODE(FILE_DEVICE_UNKNOWN, 0x052, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_LISTEN_GEOFENCES_TRACKINGSTATUS CTL_CODE(FILE_DEVICE_UNKNOWN, 0x053, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_GNSS_LISTEN_DRIVER_REQUEST CTL_CODE(FILE_DEVICE_UNKNOWN, 0x060, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_GNSS_START_BREADCRUMBING CTL_CODE(FILE_DEVICE_UNKNOWN, 0x070, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_STOP_BREADCRUMBING CTL_CODE(FILE_DEVICE_UNKNOWN, 0x071, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_LISTEN_BREADCRUMBING_ALERT CTL_CODE(FILE_DEVICE_UNKNOWN, 0x072, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GNSS_POP_BREADCRUMBS CTL_CODE(FILE_DEVICE_UNKNOWN, 0x073, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
//    Define the commonly used AGNSS Formats
//    Some of these formats are supported by specific IHVs only
//

#define GNSS_AGNSSFORMAT_XTRA1 0x01
#define GNSS_AGNSSFORMAT_XTRA2 0x02
#define GNSS_AGNSSFORMAT_LTO 0x04
#define GNSS_AGNSSFORMAT_XTRA3 0x08
#define GNSS_AGNSSFORMAT_XTRA3_1 0x10
#define GNSS_AGNSSFORMAT_XTRA3_2 0x20
#define GNSS_AGNSSFORMAT_XTRA_INT 0x40

//
// Device Interface GUID for GNSS device
//

// {3336e5e4-018a-4669-84c5-bd05f3bd368b}
DEFINE_GUID(GUID_DEVINTERFACE_GNSS, 0x3336e5e4, 0x18a, 0x4669, 0x84, 0xc5, 0xbd, 0x5, 0xf3, 0xbd, 0x36, 0x8b);

//
// Define max server URL name. It is 256 uchars plus some ending spaces.
//
//

#define MAX_SERVER_URL_NAME 260

//
// Define minimum number of geofences supported to be able to use native Geofencing.
//
//

#define MIN_GEOFENCES_REQUIRED 100

//
// Enumerate all versions of Breadcrumbs
//
//

#define BREADCRUMBING_UNSUPPORTED 0
#define BREADCRUMBING_VERSION_1 1

//
// Define the minimum number of breadcrumbs that must be stored in order for a driver
// to be considered conforming to the breadcrumbing portions of the interface
//
//

#define MIN_BREADCRUMBS_SUPPORTED 120

//
// Define structure for SUPL version
//

typedef struct
{
    ULONG MajorVersion;
    ULONG MinorVersion;
} GNSS_SUPL_VERSION, *PGNSS_SUPL_VERSION;

//
// Define structure for SUPL version 2
//

typedef struct
{
    ULONG MajorVersion;
    ULONG MinorVersion;
    ULONG ServiceIndicator;
} GNSS_SUPL_VERSION_2, *PGNSS_SUPL_VERSION_2;

//
// Data structure for Driver Capabilities
//

typedef struct
{
    ULONG Size;

    // The version number of this structure is the DDK version against which the GNSS Driver is developed

    ULONG Version;

    // Session Management

    BOOL SupportMultipleFixSessions;
    BOOL SupportMultipleAppSessions;

    // AGNSS
    BOOL RequireAGnssInjection;
    ULONG AgnssFormatSupported;
    ULONG AgnssFormatPreferred;

    // Tracking support
    BOOL SupportDistanceTracking;
    BOOL SupportContinuousTracking;

    // Reserved fields for future use

    ULONG Reserved1;
    BOOL Reserved2;
    BOOL Reserved3;
    BOOL Reserved4;
    BOOL Reserved5;

    // Geofencing
    ULONG GeofencingSupport;

    // Reserved fields for future use

    BOOL Reserved6;
    BOOL Reserved7;

    // SUPL/CP

    BOOL SupportCpLocation;
    BOOL SupportUplV2;
    BOOL SupportSuplV1;
    BOOL SupportSuplV2;
    GNSS_SUPL_VERSION SupportedSuplVersion; // Location adapter does not refer to this SUPL version provided by GNSS driver today

    // Geofence Parameters
    // Only valid when GeofencingSupport field is non-zero

    ULONG MaxGeofencesSupported;

    // SUPL Parameter
    BOOL SupportMultipleSuplRootCert;

    // Bread crumbing
    ULONG GnssBreadCrumbPayloadVersion; // BREADCRUMBING_UNSUPPORTED or BREADCRUMBING_VERSION_x
    ULONG MaxGnssBreadCrumbFixes;       // Must >= MIN_BREADCRUMBS_SUPPORTED

    BYTE Unused[496];
} GNSS_DEVICE_CAPABILITY, *PGNSS_DEVICE_CAPABILITY;

//
//    Define structure for Platform Capabilities
//

typedef struct
{
    ULONG Size;

    // The version number of this structure is the DDK version against which the HLOS Platform (GNSS Adapter) is developed
    // The HLOS controls the DDK versioning  such that the GNSS Adapter stays backward compatible with all prior versions of the Driver
    // Driver compiled with a new DDK version requires the corresponding HLOS be compiled with the same or more recent DDK version
    //

    ULONG Version;

    BOOL SupportAgnssInjection;
    ULONG AgnssFormatSupported;

    BYTE Unused[516];
} GNSS_PLATFORM_CAPABILITY, *PGNSS_PLATFORM_CAPABILITY;

//
//    Define the driver commands
//

typedef enum
{
    GNSS_SetLocationServiceEnabled = 0x01,
    GNSS_SetLocationNIRequestAllowed = 0x02,
    GNSS_ForceSatelliteSystem = 0x03,
    GNSS_ForceOperationMode = 0x04,
    GNSS_ResetEngine = 0x09,
    GNSS_ClearAgnssData = 0x0A,
    GNSS_SetSuplVersion = 0x0C, // GNSS_SUPL_VERSION
    GNSS_SetNMEALogging = 0x0D,
    GNSS_SetUplServerAccessInterval = 0x0E,
    GNSS_SetNiTimeoutInterval = 0x0F,
    GNSS_ResetGeofencesTracking = 0x10,
    GNSS_SetSuplVersion2 = 0x11, // GNSS_SUPL_VERSION_2
    GNSS_CustomCommand = 0x0100,
} GNSS_DRIVERCOMMAND_TYPE;

//
// Define the data structure for sending driver commands
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    GNSS_DRIVERCOMMAND_TYPE CommandType;
    ULONG Reserved;
    ULONG CommandDataSize;
    BYTE Unused[512];
    BYTE CommandData[ANYSIZE_ARRAY];
} GNSS_DRIVERCOMMAND_PARAM, *PGNSS_DRIVERCOMMAND_PARAM;

//
// Define Satellite System
//

#define GNSS_SATELLITE_ANY 0x00
#define GNSS_SATELLITE_GPS 0x01
#define GNSS_SATELLITE_GLONASS 0x02
#define GNSS_SATELLITE_BEIDOU 0x04
#define GNSS_SATELLITE_GALILEO 0x08

//
// Define Operation Mode
//

#define GNSS_OPERMODE_ANY 0x00
#define GNSS_OPERMODE_MSA 0x01
#define GNSS_OPERMODE_MSB 0x02
#define GNSS_OPERMODE_MSS 0x04
#define GNSS_OPERMODE_CELLID 0x08
#define GNSS_OPERMODE_AFLT 0x10
#define GNSS_OPERMODE_OTDOA 0x20

//
// Define known values for GNSS_SetNMEALogging
//

#define GNSS_NMEALOGGING_NONE 0x00
#define GNSS_NMEALOGGING_ALL 0xFF

//
// Define Fix Session Types
//

typedef enum
{
    GNSS_FixSession_SingleShot = 0x01,
    GNSS_FixSession_DistanceTracking = 0x02,
    GNSS_FixSession_ContinuousTracking = 0x03,
    GNSS_FixSession_LKG = 0x04,
} GNSS_FIXSESSIONTYPE;

//
// Define data structure for Single-shot Fix Session request
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG ResponseTime;

} GNSS_SINGLESHOT_PARAM, *PGNSS_SINGLESHOT_PARAM;

//
// Define data structure for Distance Tracking Fix Session request
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG MovementThreshold;

} GNSS_DISTANCETRACKING_PARAM, *PGNSS_DISTANCETRACKING_PARAM;

//
// Define data structure for Continuous Fix Session request
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG PreferredInterval;

} GNSS_CONTINUOUSTRACKING_PARAM, *PGNSS_CONTINUOUSTRACKING_PARAM;

//
// Define data structure for Last-known-good Fix Session request
//

typedef struct
{
    ULONG Size;
    ULONG Version;

} GNSS_LKGFIX_PARAM, *PGNSS_LKGFIX_PARAM;

//
// Define data structure for Fix Session request
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG FixSessionID;
    GNSS_FIXSESSIONTYPE SessionType;

    // Fix quality preferences

    ULONG HorizontalAccuracy;
    ULONG HorizontalConfidence;

    //  Reserved Fields
    ULONG Reserved[9];

    ULONG FixLevelOfDetails;

    union
    {
        GNSS_SINGLESHOT_PARAM SingleShotParam;
        GNSS_DISTANCETRACKING_PARAM DistanceParam;
        GNSS_CONTINUOUSTRACKING_PARAM ContinuousParam;
        GNSS_LKGFIX_PARAM LkgFixParam;
        BYTE UnusedParam[268];
    };

    BYTE Unused[256];
} GNSS_FIXSESSION_PARAM, *PGNSS_FIXSESSION_PARAM;

//
//    Define data structure for stopping Fix Session
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG FixSessionID;

    BYTE Unused[512];
} GNSS_STOPFIXSESSION_PARAM, *PGNSS_STOPFIXSESSION_PARAM;

//
// Define level of details associated with a fix
//

#define GNSS_FIXDETAIL_BASIC 0x0001
#define GNSS_FIXDETAIL_ACCURACY 0x0002
#define GNSS_FIXDETAIL_SATELLITE 0x0004

//
// Define data structure for basic fix data
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    double Latitude;
    double Longitude;
    double Altitude; // Altitude with respect to mean sea level
    double Speed;
    double Heading;

} GNSS_FIXDATA_BASIC, *PGNSS_FIXDATA_BASIC;

//
// Define data structure for basic fix data version 2
// This data structure is added in GNSS_DRIVER_VERSION_6
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    double Latitude;
    double Longitude;
    double Altitude; // Altitude with respect to mean sea level
    double Speed;
    double Heading;

    double AltitudeEllipsoid; // Altitude with respect to ellipsoid

} GNSS_FIXDATA_BASIC_2, *PGNSS_FIXDATA_BASIC_2;

//
// Define data structure for accuracy-related fix data
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG HorizontalAccuracy;
    ULONG HorizontalErrorMajorAxis;
    ULONG HorizontalErrorMinorAxis;
    ULONG HorizontalErrorAngle;
    ULONG HeadingAccuracy;
    ULONG AltitudeAccuracy;
    ULONG SpeedAccuracy;

    ULONG HorizontalConfidence;
    ULONG HeadingConfidence;
    ULONG AltitudeConfidence;
    ULONG SpeedConfidence;

    float PositionDilutionOfPrecision;
    float HorizontalDilutionOfPrecision;
    float VerticalDilutionOfPrecision;

} GNSS_FIXDATA_ACCURACY, *PGNSS_FIXDATA_ACCURACY;

//
// Define data structure for accuracy-related fix data version 2
// This data structure is added in GNSS_DRIVER_VERSION_6
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    double HorizontalAccuracy;
    double HorizontalErrorMajorAxis;
    double HorizontalErrorMinorAxis;
    double HorizontalErrorAngle;
    double HeadingAccuracy;
    double AltitudeAccuracy;
    double SpeedAccuracy;

    ULONG HorizontalConfidence;
    ULONG HeadingConfidence;
    ULONG AltitudeConfidence;
    ULONG SpeedConfidence;

    double PositionDilutionOfPrecision;
    double HorizontalDilutionOfPrecision;
    double VerticalDilutionOfPrecision;
    double GeometricDilutionOfPrecision;
    double TimeDilutionOfPrecision;

} GNSS_FIXDATA_ACCURACY_2, *PGNSS_FIXDATA_ACCURACY_2;

//
// Define data structure for satellite information
//

typedef struct
{
    ULONG SatelliteId;
    BOOL UsedInPositiong;
    double Elevation;
    double Azimuth;
    double SignalToNoiseRatio;
} GNSS_SATELLITEINFO, *PGNSS_SATELLITEINFO;

//
// Define data structure for satellite related fix data
//

#define GNSS_MAXSATELLITE 64

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG SatelliteCount;

    GNSS_SATELLITEINFO SatelliteArray[GNSS_MAXSATELLITE];

} GNSS_FIXDATA_SATELLITE, *PGNSS_FIXDATA_SATELLITE;

//
// Define data structure for fix data
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG FixSessionID;
    FILETIME FixTimeStamp;
    BOOL IsFinalFix;
    NTSTATUS FixStatus;

    ULONG FixLevelOfDetails;

    GNSS_FIXDATA_BASIC BasicData;
    GNSS_FIXDATA_ACCURACY AccuracyData;
    GNSS_FIXDATA_SATELLITE SatelliteData;

} GNSS_FIXDATA, *PGNSS_FIXDATA;

//
// Define data structure for fix data version 2
// This data structure is added in GNSS_DRIVER_VERSION_6
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG FixSessionID;
    FILETIME FixTimeStamp;
    BOOL IsFinalFix;
    NTSTATUS FixStatus;

    ULONG FixLevelOfDetails;

    GNSS_FIXDATA_BASIC_2 BasicData;
    GNSS_FIXDATA_ACCURACY_2 AccuracyData;
    GNSS_FIXDATA_SATELLITE SatelliteData;

} GNSS_FIXDATA_2, *PGNSS_FIXDATA_2;

//
// The configuration passed into the start of breadcrumbing via IOCTL_GNSS_START_BREADCRUMBING
//
typedef struct
{
    ULONG Size;
    ULONG Version;

    // Any fix with an error radius larger than this shall not be recorded.
    ULONG MaximumHorizontalUncertainty;

    // Only record a fix if the center of it is at least this far apart from center of the last,
    // using a Haversine distance calculation
    ULONG MinDistanceBetweenFixes;

    // If the location of the device is unknown for this duration, an error must be recorded in
    // the breadcrumb data. Errors can be recorded earlier if they were already known.
    ULONG MaximumErrorTimeoutMs;

    BYTE Unused[512];
} GNSS_BREADCRUMBING_PARAM, *PGNSS_BREADCRUMBING_PARAM;

//
// The alert information for when the breadcrumbing buffer has reached a level at which OS read
// operations should be performed.
//
typedef struct
{
    ULONG Size;
    ULONG Version;

    BYTE Unused[512];
} GNSS_BREADCRUMBING_ALERT_DATA, *PGNSS_BREADCRUMBING_ALERT_DATA;

//
// An individual breadcrumb. The order and types of the fields are carefully crafted to pack densely.
//
typedef struct
{
    FILETIME FixTimeStamp;

    double Latitude;
    double Longitude;
    ULONG HorizontalAccuracy;

    unsigned short Speed;
    unsigned short SpeedAccuracy;

    short Altitude;
    unsigned short AltitudeAccuracy;

    short Heading;
    unsigned char HeadingAccuracy;

    unsigned char FixSuccess; // A Boolean type
} GNSS_BREADCRUMB_V1, *PGNSS_BREADCRUMB_V1;

//
// The response to an IOCTL_GNSS_POP_BREADCRUMBS
//
typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG NumCrumbs;
    union
    {
        GNSS_BREADCRUMB_V1 v1[50];
    };

} GNSS_BREADCRUMB_LIST, *PGNSS_BREADCRUMB_LIST;

//
// Define the level of Geofence support by the Driver
//

#define GNSS_GEOFENCESUPPORT_SUPPORTED 0x01 // Supports Geofence Tracking
#define GNSS_GEOFENCESUPPORT_CIRCLE 0x02    // Supports Circular Geofences

//
// Define enumeration for Geofence Shape
//

typedef enum
{
    GNSS_GeoRegion_Circle = 0x01,
} GNSS_GEOREGIONTYPE;

//
// Define data structure for Circle shape
//

typedef struct
{
    double Latitude;
    double Longitude;
    double RadiusInMeters;

} GNSS_GEOREGION_CIRCLE, *PGNSS_GEOREGION_CIRCLE;

//
// Define data structure for Geofence Region
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    GNSS_GEOREGIONTYPE GeoRegionType;

    union
    {
        GNSS_GEOREGION_CIRCLE Circle;
        BYTE Unused[512];
    };

} GNSS_GEOREGION, *PGNSS_GEOREGION;

//
// Define Geofence States and Alert Mask
//

typedef enum
{
    GNSS_GeofenceState_Unknown = 0x00,
    GNSS_GeofenceState_Entered = 0x01,
    GNSS_GeofenceState_Exited = 0x02,
} GNSS_GEOFENCE_STATE;

#define GNSS_GEOFENCEALERTTYPE_ENTRY GNSS_GeofenceState_Entered // Enter Geofence
#define GNSS_GEOFENCEALERTTYPE_EXIT GNSS_GeofenceState_Exited   // Exit Geofence

//
// Define data structure for Geofence Create parameters
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG AlertTypes;                 // Bit-mask to indicate the Geofence alert types (GNSS_GEOFENCEALERTTYPE_xxxx)
    GNSS_GEOFENCE_STATE InitialState; // Initial State of the Geofence, as seen by HLOS

    GNSS_GEOREGION Boundary; // The Geofence boundary

    BYTE Unused[512];
} GNSS_GEOFENCE_CREATE_PARAM, *PGNSS_GEOFENCE_CREATE_PARAM;

//
// Define data structure for Create Geofence Response
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    NTSTATUS CreationStatus;
    ULONG GeofenceID;

    BYTE Unused[512];

} GNSS_GEOFENCE_CREATE_RESPONSE, *PGNSS_GEOFENCE_CREATE_RESPONSE;

//
// Define data structure for Geofence Delete parameters
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG GeofenceID;

    BYTE Unused[512];
} GNSS_GEOFENCE_DELETE_PARAM, *PGNSS_GEOFENCE_DELETE_PARAM;

//
// Define data structure for Geofence Alert Data
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG GeofenceID;
    GNSS_GEOFENCE_STATE GeofenceState;

    GNSS_FIXDATA_BASIC FixBasicData;
    GNSS_FIXDATA_ACCURACY FixAccuracyData;

    BYTE Unused[512];
} GNSS_GEOFENCE_ALERT_DATA, *PGNSS_GEOFENCE_ALERT_DATA;

//
// Define data structure for Tracking Status of all Geofences
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    NTSTATUS Status;
    FILETIME StatusTimeStamp;

    BYTE Unused[512];
} GNSS_GEOFENCES_TRACKINGSTATUS_DATA, *PGNSS_GEOFENCES_TRACKINGSTATUS_DATA;

//
// Define GNSS events
//

typedef enum
{
    GNSS_Event_FixAvailable = 0x0001,
    GNSS_Event_RequireAgnss = 0x0002,
    GNSS_Event_Error = 0x0003,
    GNSS_Event_NiRequest = 0x000C,
    GNSS_Event_NmeaData = 0x000D,
    GNSS_Event_GeofenceAlertData = 0x000E,
    GNSS_Event_GeofencesTrackingStatus = 0x000F,
    GNSS_Event_DriverRequest = 0x0010,
    GNSS_Event_BreadcrumbAlertEvent = 0x0011,
    GNSS_Event_FixAvailable_2 = 0x0012, // Added in GNSS_DRIVER_VERSION_6
    GNSS_Event_Custom = 0x8000
} GNSS_EVENT_TYPE;

//
// Define data structure for error information
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG ErrorCode;
    BOOL IsRecoverable;
    WCHAR ErrorDescription[256];

    BYTE Unused[512];

} GNSS_ERRORINFO, *PGNSS_ERRORINFO;

//
// Define data structure for generic (non-parsed) NMEA data
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    CHAR NmeaSentences[256];

} GNSS_NMEA_DATA, *PGNSS_NMEA_DATA;

//
//    Define AGNSS request type
//

typedef enum
{
    GNSS_AGNSS_TimeInjection = 0x01,
    GNSS_AGNSS_PositionInjection = 0x02,
    GNSS_AGNSS_BlobInjection = 0x03,
} GNSS_AGNSS_REQUEST_TYPE;

//
//    Define AGNSS request patameter
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    GNSS_AGNSS_REQUEST_TYPE RequestType;
    ULONG BlobFormat;

} GNSS_AGNSS_REQUEST_PARAM, *PGNSS_AGNSS_REQUEST_PARAM;

//
// Define Plane type
//

typedef enum
{
    GNSS_NI_SUPL = 0x01,
    GNSS_NI_CP = 0x02,
    GNSS_NI_V2UPL = 0x03,
} GNSS_NI_PLANE_TYPE;

//
// Define Request type
//

typedef enum
{
    GNSS_NI_Request_SingleShot = 0x01,
    GNSS_NI_Request_AreaTrigger = 0x02,
} GNSS_NI_REQUEST_TYPE;

//
// Define Request type
//

typedef enum
{
    GNSS_NI_NoNotifyNoVerify = 0x01,
    GNSS_NI_NotifyOnly = 0x02,
    GNSS_NI_NotifyVerifyDefaultAllow = 0x03,
    GNSS_NI_NotifyVerifyDefaultNotAllow = 0x04,
    GNSS_NI_PrivacyOverride = 0x05
} GNSS_NI_NOTIFICATION_TYPE;

//
// Define data structure for SUPL NI info
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    WCHAR RequestorId[MAX_PATH];
    WCHAR ClientName[MAX_PATH];
    CHAR SuplNiUrl[MAX_SERVER_URL_NAME];

} GNSS_SUPL_NI_INFO, *PGNSS_SUPL_NI_INFO;

//
// Define data structure for CP NI info
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    WCHAR RequestorId[MAX_PATH];
    WCHAR NotificationText[MAX_PATH];

} GNSS_CP_NI_INFO, *PGNSS_CP_NI_INFO;

//
// Define data structure for V2UPL NI info
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    WCHAR RequestorId[MAX_PATH];

} GNSS_V2UPL_NI_INFO, *PGNSS_V2UPL_NI_INFO;

//
// Define data structure for Request parameter
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG RequestId;
    GNSS_NI_REQUEST_TYPE RequestType;
    GNSS_NI_NOTIFICATION_TYPE NotificationType;

    GNSS_NI_PLANE_TYPE RequestPlaneType;
    union
    {
        GNSS_SUPL_NI_INFO SuplNiInfo;
        GNSS_CP_NI_INFO CpNiInfo;
        GNSS_V2UPL_NI_INFO V2UplNiInfo;
    };
    ULONG ResponseTimeInSec;

    BOOL EmergencyLocation;

} GNSS_NI_REQUEST_PARAM, *PGNSS_NI_REQUEST_PARAM;

//
// Define the data requested by the driver
//

typedef enum
{
    SUPL_CONFIG_DATA = 0x01,
} GNSS_DRIVER_REQUEST;

//
// Define data structure for driver requested data
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    GNSS_DRIVER_REQUEST Request;
    ULONG RequestFlag;
} GNSS_DRIVER_REQUEST_DATA;

//
// Define data structure for GNSS events
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    GNSS_EVENT_TYPE EventType;
    ULONG EventDataSize;
    BYTE Unused[512];

    union
    {
        GNSS_FIXDATA FixData;
        GNSS_AGNSS_REQUEST_PARAM AgnssRequest;
        GNSS_NI_REQUEST_PARAM NiRequest;
        GNSS_ERRORINFO ErrorInformation;
        GNSS_NMEA_DATA NmeaData;
        GNSS_GEOFENCE_ALERT_DATA GeofenceAlertData;
        GNSS_BREADCRUMBING_ALERT_DATA BreadcrumbAlertData;
        GNSS_GEOFENCES_TRACKINGSTATUS_DATA GeofencesTrackingStatus;
        GNSS_DRIVER_REQUEST_DATA DriverRequestData;
        BYTE CustomData[ANYSIZE_ARRAY];
    };

} GNSS_EVENT, *PGNSS_EVENT;

//
// Define data structure for GNSS events version 2
// This data structure is added in GNSS_DRIVER_VERSION_6
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    GNSS_EVENT_TYPE EventType;
    ULONG EventDataSize;
    BYTE Unused[512];

    union
    {
        GNSS_FIXDATA FixData;
        GNSS_FIXDATA_2 FixData2;
        GNSS_AGNSS_REQUEST_PARAM AgnssRequest;
        GNSS_NI_REQUEST_PARAM NiRequest;
        GNSS_ERRORINFO ErrorInformation;
        GNSS_NMEA_DATA NmeaData;
        GNSS_GEOFENCE_ALERT_DATA GeofenceAlertData;
        GNSS_BREADCRUMBING_ALERT_DATA BreadcrumbAlertData;
        GNSS_GEOFENCES_TRACKINGSTATUS_DATA GeofencesTrackingStatus;
        GNSS_DRIVER_REQUEST_DATA DriverRequestData;
        BYTE CustomData[ANYSIZE_ARRAY];
    };

} GNSS_EVENT_2, *PGNSS_EVENT_2;

//
// Define data structure for AGNSS time injection
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    FILETIME UtcTime;
    ULONG TimeUncertainty;

} GNSS_AGNSS_INJECTTIME, *PGNSS_AGNSS_INJECTTIME;

//
// Define data structure for AGNSS position injection
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG Age;

    GNSS_FIXDATA_BASIC BasicData;
    GNSS_FIXDATA_ACCURACY AccuracyData;

} GNSS_AGNSS_INJECTPOSITION, *PGNSS_AGNSS_INJECTPOSITION;

//
// Define data structure for AGNSS Blob injection
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG BlobOui;
    ULONG BlobVersion;
    ULONG AgnssFormat;
    ULONG BlobSize;

    BYTE BlobData[ANYSIZE_ARRAY];
} GNSS_AGNSS_INJECTBLOB, *PGNSS_AGNSS_INJECTBLOB;

//
// Define data structure for AGNSS injection
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    GNSS_AGNSS_REQUEST_TYPE InjectionType;
    NTSTATUS InjectionStatus;

    ULONG InjectionDataSize;

    BYTE Unused[512];

    union
    {
        GNSS_AGNSS_INJECTTIME Time;
        GNSS_AGNSS_INJECTPOSITION Position;
        GNSS_AGNSS_INJECTBLOB BlobData;
    };

} GNSS_AGNSS_INJECT, *PGNSS_AGNSS_INJECT;

//
// Define data structure for SUPL H-SLP
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    CHAR SuplHslp[MAX_SERVER_URL_NAME];
    CHAR SuplHslpFromImsi[MAX_SERVER_URL_NAME];
    ULONG Reserved;

    BYTE Unused[512];

} GNSS_SUPL_HSLP_CONFIG, *PGNSS_SUPL_HSLP_CONFIG;

//
// Define actions for SUPL certs
//

typedef enum
{
    GNSS_Supl_Cert_Inject = 0x01,
    GNSS_Supl_Cert_Delete = 0x02,
    GNSS_Supl_Cert_Purge = 0x03,
} GNSS_SUPL_CERT_ACTION;

//
// Define data structure for SUPL Certs
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    GNSS_SUPL_CERT_ACTION CertAction;
    CHAR SuplCertName[MAX_PATH];
    ULONG CertSize;
    BYTE Unused[512];

    BYTE CertData[ANYSIZE_ARRAY];

} GNSS_SUPL_CERT_CONFIG, *PGNSS_SUPL_CERT_CONFIG;

//
// Define data structure for V2UPL Config
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    CHAR MPC[MAX_SERVER_URL_NAME];
    CHAR PDE[MAX_SERVER_URL_NAME];
    BYTE ApplicationTypeIndicator_MR;

    BYTE Unused[512];

} GNSS_V2UPL_CONFIG, *PGNSS_V2UPL_CONFIG;

//
// Define user response type
//

typedef enum
{
    GNSS_Ni_UserResponseAccept = 0x01,
    GNSS_Ni_UserResponseDeny = 0x02,
    GNSS_Ni_UserResponseTimeout = 0x03,
} GNSS_NI_USER_RESPONSE;

//
// Define data structure for NI user response.
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG RequestId;
    GNSS_NI_USER_RESPONSE UserResponse;

} GNSS_NI_RESPONSE, *PGNSS_NI_RESPONSE;

//
// Define data structure for IOCTL_GNSS_EXECUTE_CWTEST
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    NTSTATUS TestResultStatus;
    double SignalToNoiseRatio;
    double Frequency;
    BYTE Unused[512];

} GNSS_CWTESTDATA, *PGNSS_CWTESTDATA;

//
// Define data structures for IOCTL_GNSS_EXECUTE_SELFTEST
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    ULONG TestType;
    BYTE Unused[512];
    ULONG InBufLen;
    BYTE InBuffer[ANYSIZE_ARRAY];

} GNSS_SELFTESTCONFIG, *PGNSS_SELFTESTCONFIG;

typedef struct
{
    ULONG Size;
    ULONG Version;

    NTSTATUS TestResultStatus;
    ULONG Result;
    ULONG PinFailedBitMask;
    BYTE Unused[512];
    ULONG OutBufLen;
    BYTE OutBuffer[ANYSIZE_ARRAY];

} GNSS_SELFTESTRESULT, *PGNSS_SELFTESTRESULT;

//
// Define data structure for IOCTL_GNSS_GET_CHIPSETINFO
//

typedef struct
{
    ULONG Size;
    ULONG Version;

    WCHAR ManufacturerID[25];
    WCHAR HardwareID[25];
    WCHAR FirmwareVersion[20];
    BYTE Unused[512];

} GNSS_CHIPSETINFO, *PGNSS_CHIPSETINFO;

// Restore the previous setting for C4201 compiler warning
#pragma warning(pop)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#endif // NTDDI_VERSION >= NTDDI_WINBLUE
