/*++

Copyright (c) 2007 Microsoft Corporation


Module Name:

    winbio_types.h

Abstract:

    Type definitions, constants, and structures used
    by Windows Biometrics components.


Environment:

    User or Kernel mode.

Revision History:

--*/

#ifndef _WINBIO_TYPES_H_712486DB_3EF5_41da_937A_55DCB7B66A53_
#define _WINBIO_TYPES_H_712486DB_3EF5_41da_937A_55DCB7B66A53_

#if (NTDDI_VERSION >= NTDDI_WIN7)

#pragma warning( push )
#pragma warning( disable : 4324 ) // structure-padding message

#ifdef __cplusplus
extern "C"{
#endif

///////////////////////////////////////////////////////////////////////////////
//
// Types used throughout WinBio
//
///////////////////////////////////////////////////////////////////////////////

typedef ULONG WINBIO_UNIT_ID, *PWINBIO_UNIT_ID;
typedef ULONG WINBIO_SESSION_HANDLE, *PWINBIO_SESSION_HANDLE;
typedef WINBIO_SESSION_HANDLE WINBIO_FRAMEWORK_HANDLE, *PWINBIO_FRAMEWORK_HANDLE;

//
// A GUID
//
typedef GUID WINBIO_UUID, *PWINBIO_UUID;

//
// Represents a NULL-terminated Unicode character
// string inside a fixed-length buffer.
//
#define WINBIO_MAX_STRING_LEN 256
typedef WCHAR WINBIO_STRING[WINBIO_MAX_STRING_LEN];
typedef WINBIO_STRING *PWINBIO_STRING;

//
// Version
//
typedef struct _WINBIO_VERSION {
    DWORD MajorVersion;
    DWORD MinorVersion;
} WINBIO_VERSION, *PWINBIO_VERSION;

///////////////////////////////////////////////////////////////////////////////
//
// Enumeration for template identity types...
//
typedef ULONG WINBIO_IDENTITY_TYPE, *PWINBIO_IDENTITY_TYPE;

#ifdef MIDL_PASS

const WINBIO_IDENTITY_TYPE  WINBIO_ID_TYPE_NULL      = (WINBIO_IDENTITY_TYPE)0;  // The Identity structure is empty.
const WINBIO_IDENTITY_TYPE  WINBIO_ID_TYPE_WILDCARD  = (WINBIO_IDENTITY_TYPE)1;  // The Identity matches "all identities".
const WINBIO_IDENTITY_TYPE  WINBIO_ID_TYPE_GUID      = (WINBIO_IDENTITY_TYPE)2;  // A GUID identifies the template.
const WINBIO_IDENTITY_TYPE  WINBIO_ID_TYPE_SID       = (WINBIO_IDENTITY_TYPE)3;  // An account SID identifies the template.
const WINBIO_IDENTITY_TYPE  WINBIO_ID_TYPE_SECURE_ID = (WINBIO_IDENTITY_TYPE)4;  // A secure ID identifies the template.

const ULONG SECURITY_MAX_SID_SIZE          = (ULONG)68;
const ULONG WINBIO_IDENTITY_SECURE_ID_SIZE = (ULONG)32;

typedef union switch(WINBIO_IDENTITY_TYPE Type) _WINBIO_IDENTITY {
    case WINBIO_ID_TYPE_NULL:       ULONG Null;
    case WINBIO_ID_TYPE_WILDCARD:   ULONG Wildcard;
    case WINBIO_ID_TYPE_GUID:       GUID TemplateGuid;
    case WINBIO_ID_TYPE_SID:        struct {
                                        ULONG Size;
                                        UCHAR Data[SECURITY_MAX_SID_SIZE];
                                    } AccountSid;
    case WINBIO_ID_TYPE_SECURE_ID:  UCHAR SecureId[WINBIO_IDENTITY_SECURE_ID_SIZE];
} WINBIO_IDENTITY;

#else // MIDL_PASS

#define WINBIO_ID_TYPE_NULL      ((WINBIO_IDENTITY_TYPE)0)  // The Identity structure is empty.
#define WINBIO_ID_TYPE_WILDCARD  ((WINBIO_IDENTITY_TYPE)1)  // The Identity matches "all identities"
#define WINBIO_ID_TYPE_GUID      ((WINBIO_IDENTITY_TYPE)2)  // A GUID identifies the template.
#define WINBIO_ID_TYPE_SID       ((WINBIO_IDENTITY_TYPE)3)  // An account SID identifies the template.
#define WINBIO_ID_TYPE_SECURE_ID ((WINBIO_IDENTITY_TYPE)4)  // A secure ID identifies the template.

//
// Structure that contains the identity value associated
// with a biometric template.
//
//#ifndef SECURITY_MAX_SID_SIZE
//#define SECURITY_MAX_SID_SIZE 68
//#endif
#define WINBIO_IDENTITY_SECURE_ID_SIZE ((ULONG)32)
typedef struct _WINBIO_IDENTITY {
    WINBIO_IDENTITY_TYPE Type;
    union {
        ULONG Null;
        ULONG Wildcard;
        GUID TemplateGuid;
        struct {
            ULONG Size;
            UCHAR Data[SECURITY_MAX_SID_SIZE];
        } AccountSid;
#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
        UCHAR SecureId[WINBIO_IDENTITY_SECURE_ID_SIZE];
#endif
    } Value;
} WINBIO_IDENTITY;

#endif // MIDL_PASS

typedef WINBIO_IDENTITY *PWINBIO_IDENTITY;

#define WINBIO_IDENTITY_WILDCARD  ((ULONG)0x25066282)


///////////////////////////////////////////////////////////////////////////////
//
// Bitmask describing the supported set of biometric types (factors).
//
typedef ULONG32 WINBIO_BIOMETRIC_TYPE, *PWINBIO_BIOMETRIC_TYPE;

#ifdef MIDL_PASS

const WINBIO_BIOMETRIC_TYPE WINBIO_STANDARD_TYPE_MASK      = (WINBIO_BIOMETRIC_TYPE)0x00FFFFFF;

const WINBIO_BIOMETRIC_TYPE WINBIO_NO_TYPE_AVAILABLE       = (WINBIO_BIOMETRIC_TYPE)0x00000000;

//
// Standard biometric types (from NISTIR 6529-A)
//
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_MULTIPLE            = (WINBIO_BIOMETRIC_TYPE)0x00000001;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_FACIAL_FEATURES     = (WINBIO_BIOMETRIC_TYPE)0x00000002;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_VOICE               = (WINBIO_BIOMETRIC_TYPE)0x00000004;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_FINGERPRINT         = (WINBIO_BIOMETRIC_TYPE)0x00000008;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_IRIS                = (WINBIO_BIOMETRIC_TYPE)0x00000010;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_RETINA              = (WINBIO_BIOMETRIC_TYPE)0x00000020;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_HAND_GEOMETRY       = (WINBIO_BIOMETRIC_TYPE)0x00000040;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_SIGNATURE_DYNAMICS  = (WINBIO_BIOMETRIC_TYPE)0x00000080;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_KEYSTROKE_DYNAMICS  = (WINBIO_BIOMETRIC_TYPE)0x00000100;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_LIP_MOVEMENT        = (WINBIO_BIOMETRIC_TYPE)0x00000200;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_THERMAL_FACE_IMAGE  = (WINBIO_BIOMETRIC_TYPE)0x00000400;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_THERMAL_HAND_IMAGE  = (WINBIO_BIOMETRIC_TYPE)0x00000800;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_GAIT                = (WINBIO_BIOMETRIC_TYPE)0x00001000;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_SCENT               = (WINBIO_BIOMETRIC_TYPE)0x00002000;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_DNA                 = (WINBIO_BIOMETRIC_TYPE)0x00004000;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_EAR_SHAPE           = (WINBIO_BIOMETRIC_TYPE)0x00008000;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_FINGER_GEOMETRY     = (WINBIO_BIOMETRIC_TYPE)0x00010000;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_PALM_PRINT          = (WINBIO_BIOMETRIC_TYPE)0x00020000;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_VEIN_PATTERN        = (WINBIO_BIOMETRIC_TYPE)0x00040000;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_FOOT_PRINT          = (WINBIO_BIOMETRIC_TYPE)0x00080000;
//
// WinBio extended types
//
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_OTHER               = (WINBIO_BIOMETRIC_TYPE)0x40000000;
const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_PASSWORD            = (WINBIO_BIOMETRIC_TYPE)0x80000000;

const WINBIO_BIOMETRIC_TYPE WINBIO_TYPE_ANY                 = (WINBIO_BIOMETRIC_TYPE)(WINBIO_STANDARD_TYPE_MASK | WINBIO_TYPE_OTHER | WINBIO_TYPE_PASSWORD);

#else   // MIDL_PASS

#define WINBIO_STANDARD_TYPE_MASK           ((WINBIO_BIOMETRIC_TYPE)0x00FFFFFF)

#define WINBIO_NO_TYPE_AVAILABLE            ((WINBIO_BIOMETRIC_TYPE)0x00000000)
//
// Standard biometric types (from NISTIR 6529-A)
//
#define WINBIO_TYPE_MULTIPLE                ((WINBIO_BIOMETRIC_TYPE)0x00000001)
#define WINBIO_TYPE_FACIAL_FEATURES         ((WINBIO_BIOMETRIC_TYPE)0x00000002)
#define WINBIO_TYPE_VOICE                   ((WINBIO_BIOMETRIC_TYPE)0x00000004)
#define WINBIO_TYPE_FINGERPRINT             ((WINBIO_BIOMETRIC_TYPE)0x00000008)
#define WINBIO_TYPE_IRIS                    ((WINBIO_BIOMETRIC_TYPE)0x00000010)
#define WINBIO_TYPE_RETINA                  ((WINBIO_BIOMETRIC_TYPE)0x00000020)
#define WINBIO_TYPE_HAND_GEOMETRY           ((WINBIO_BIOMETRIC_TYPE)0x00000040)
#define WINBIO_TYPE_SIGNATURE_DYNAMICS      ((WINBIO_BIOMETRIC_TYPE)0x00000080)
#define WINBIO_TYPE_KEYSTROKE_DYNAMICS      ((WINBIO_BIOMETRIC_TYPE)0x00000100)
#define WINBIO_TYPE_LIP_MOVEMENT            ((WINBIO_BIOMETRIC_TYPE)0x00000200)
#define WINBIO_TYPE_THERMAL_FACE_IMAGE      ((WINBIO_BIOMETRIC_TYPE)0x00000400)
#define WINBIO_TYPE_THERMAL_HAND_IMAGE      ((WINBIO_BIOMETRIC_TYPE)0x00000800)
#define WINBIO_TYPE_GAIT                    ((WINBIO_BIOMETRIC_TYPE)0x00001000)
#define WINBIO_TYPE_SCENT                   ((WINBIO_BIOMETRIC_TYPE)0x00002000)
#define WINBIO_TYPE_DNA                     ((WINBIO_BIOMETRIC_TYPE)0x00004000)
#define WINBIO_TYPE_EAR_SHAPE               ((WINBIO_BIOMETRIC_TYPE)0x00008000)
#define WINBIO_TYPE_FINGER_GEOMETRY         ((WINBIO_BIOMETRIC_TYPE)0x00010000)
#define WINBIO_TYPE_PALM_PRINT              ((WINBIO_BIOMETRIC_TYPE)0x00020000)
#define WINBIO_TYPE_VEIN_PATTERN            ((WINBIO_BIOMETRIC_TYPE)0x00040000)
#define WINBIO_TYPE_FOOT_PRINT              ((WINBIO_BIOMETRIC_TYPE)0x00080000)
//
// WinBio extended types
//
#define WINBIO_TYPE_OTHER                   ((WINBIO_BIOMETRIC_TYPE)0x40000000)
#define WINBIO_TYPE_PASSWORD                ((WINBIO_BIOMETRIC_TYPE)0x80000000)

#define WINBIO_TYPE_ANY                     ((WINBIO_BIOMETRIC_TYPE)(WINBIO_STANDARD_TYPE_MASK |    \
                                                                     WINBIO_TYPE_OTHER |            \
                                                                     WINBIO_TYPE_PASSWORD))

#endif  // MIDL_PASS

//
// WinBio sensor sub-types.  These are defined per Biometric type, and are
// defined only for fingerprints in this version.
//
typedef ULONG WINBIO_BIOMETRIC_SENSOR_SUBTYPE, *PWINBIO_BIOMETRIC_SENSOR_SUBTYPE;

#define WINBIO_SENSOR_SUBTYPE_UNKNOWN       ((WINBIO_BIOMETRIC_SENSOR_SUBTYPE)0x00000000)

#define WINBIO_FP_SENSOR_SUBTYPE_SWIPE      ((WINBIO_BIOMETRIC_SENSOR_SUBTYPE)0x00000001)
#define WINBIO_FP_SENSOR_SUBTYPE_TOUCH      ((WINBIO_BIOMETRIC_SENSOR_SUBTYPE)0x00000002)

//
// Bitmask of sensor capabilities
//
typedef ULONG WINBIO_CAPABILITIES, *PWINBIO_CAPABILITIES;

#define WINBIO_CAPABILITY_SENSOR            ((WINBIO_CAPABILITIES)0x00000001)
#define WINBIO_CAPABILITY_MATCHING          ((WINBIO_CAPABILITIES)0x00000002)
#define WINBIO_CAPABILITY_DATABASE          ((WINBIO_CAPABILITIES)0x00000004)
#define WINBIO_CAPABILITY_PROCESSING        ((WINBIO_CAPABILITIES)0x00000008)
#define WINBIO_CAPABILITY_ENCRYPTION        ((WINBIO_CAPABILITIES)0x00000010)
#define WINBIO_CAPABILITY_NAVIGATION        ((WINBIO_CAPABILITIES)0x00000020)
#define WINBIO_CAPABILITY_INDICATOR         ((WINBIO_CAPABILITIES)0x00000040)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define WINBIO_CAPABILITY_VIRTUAL_SENSOR    ((WINBIO_CAPABILITIES)0x00000080)

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)

#define WINBIO_CAPABILITY_SECURE_SENSOR     ((WINBIO_CAPABILITIES)0x00000100)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

// Secure Connection Protocol (SCP) V1 --> Secure Fingerprints
#define WINBIO_CAPABILITY_SCP_V1            ((WINBIO_CAPABILITIES)0x00000200)

// Modern standby support
#define WINBIO_CAPABILITY_WAKE              ((WINBIO_CAPABILITIES)0x00000400)

//
// Constants for the SCP V1 protocol
//
typedef USHORT WINBIO_SCP_VERSION;
#define WINBIO_SCP_VERSION_1 1

// Sizes for the fixed V1 cipher suite
#define WINBIO_SCP_RANDOM_SIZE_V1           32
#define WINBIO_SCP_DIGEST_SIZE_V1           32 // SHA256
#define WINBIO_SCP_CURVE_FIELD_SIZE_V1      32 // NIST P256
#define WINBIO_SCP_PUBLIC_KEY_SIZE_V1       65 // 0x04||x||y
#define WINBIO_SCP_PRIVATE_KEY_SIZE_V1      32 // log_2(n)/8
#define WINBIO_SCP_SIGNATURE_SIZE_V1        64 // r||s
#define WINBIO_SCP_ENCRYPTION_BLOCK_SIZE_V1 16 // AES
#define WINBIO_SCP_ENCRYPTION_KEY_SIZE_V1   32 // AES256

typedef USHORT WINBIO_SCP_FLAGS;
#define WINBIO_SCP_FLAG_RECONNECT           ((WINBIO_SCP_FLAGS)0x0001)

//
// Secure Connection Protocol structures
//
typedef struct _WINBIO_SECURE_CONNECTION_PARAMS {
    DWORD PayloadSize;
    WINBIO_SCP_VERSION Version; // WINBIO_SCP_VERSION_1
    WINBIO_SCP_FLAGS Flags;
    // Required fields:
    //   HostRandom[WINBIO_SCP_RANDOM_SIZE_V1];
    // Fields omitted for reconnection:
    //   PublicKey[WINBIO_SCP_PUBLIC_KEY_SIZE_V1]
} WINBIO_SECURE_CONNECTION_PARAMS, *PWINBIO_SECURE_CONNECTION_PARAMS;

typedef struct _WINBIO_SECURE_CONNECTION_DATA {
    DWORD Size;
    WINBIO_SCP_VERSION Version; // WINBIO_SCP_VERSION_1
    WINBIO_SCP_FLAGS Flags;
    DWORD ModelCertificateSize;
    DWORD IntermediateCA1Size;
    DWORD IntermediateCA2Size;
    // Required fields:
    //   Mac[WINBIO_SCP_DIGEST_SIZE_V1];
    // Fields omitted for reconnection:
    //   DeviceRandom[WINBIO_SCP_RANDOM_SIZE_V1]
    //   ModelCertificate[ModelCertificateSize]
    //   DevicePublicKey[WINBIO_SCP_PUBLIC_KEY_SIZE_V1]
    //   FirmwarePublicKey[WINBIO_SPC_PUBLIC_KEY_SIZE_V1]
    //   FirmwareHash[WINBIO_SCP_DIGEST_SIZE_V1]
    //   ModelSignature[WINBIO_SCP_SIGNATURE_SIZE_V1]
    //   DeviceSignature[WINBIO_SCP_SIGNATURE_SIZE_V1]
    // Field required the driver needs to append for full connection:
    //   IntermediateCA1[IntermediateCA1Size]
    //   IntermediateCA2[IntermediateCA2Size]
} WINBIO_SECURE_CONNECTION_DATA, *PWINBIO_SECURE_CONNECTION_DATA;

//
// Values representing the reason the sensor woke the host
//
typedef ULONG WINBIO_WAKE_REASON, *PWINBIO_WAKE_REASON;

#define WINBIO_WAKE_REASON_UNKNOWN ((WINBIO_WAKE_REASON)0)
#define WINBIO_WAKE_REASON_TOUCH   ((WINBIO_WAKE_REASON)1)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS4)

//
// Values representing the operating status of a sensor
//
typedef ULONG WINBIO_SENSOR_STATUS, *PWINBIO_SENSOR_STATUS;

#define WINBIO_SENSOR_STATUS_UNKNOWN        ((WINBIO_SENSOR_STATUS)0)
#define WINBIO_SENSOR_ACCEPT                ((WINBIO_SENSOR_STATUS)1)
#define WINBIO_SENSOR_REJECT                ((WINBIO_SENSOR_STATUS)2)
#define WINBIO_SENSOR_READY                 ((WINBIO_SENSOR_STATUS)3)
#define WINBIO_SENSOR_BUSY                  ((WINBIO_SENSOR_STATUS)4)
#define WINBIO_SENSOR_NOT_CALIBRATED        ((WINBIO_SENSOR_STATUS)5)
#define WINBIO_SENSOR_FAILURE               ((WINBIO_SENSOR_STATUS)6)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

//
// The following only apply to VIRTUAL_SENSOR units
//
#define WINBIO_SENSOR_AVAILABLE             ((WINBIO_SENSOR_STATUS)7)
#define WINBIO_SENSOR_UNAVAILABLE           ((WINBIO_SENSOR_STATUS)8)

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

//
// Values used to set the indicator on or off
//
// By default, sensors will not have a light on.
// Applications can use these values to enable
// or disable indicator lights on the sensor.
// WINBIO_SENSOR_STATUS will provide more detail
// about the status of the light when it is "on."
//
typedef DWORD WINBIO_INDICATOR_STATUS, *PWINBIO_INDICATOR_STATUS;

#define WINBIO_INDICATOR_ON     ((WINBIO_INDICATOR_STATUS)1)
#define WINBIO_INDICATOR_OFF    ((WINBIO_INDICATOR_STATUS)2)

///////////////////////////////////////////////////////////////////////////////
//
// Sensor operating modes
//
typedef ULONG WINBIO_SENSOR_MODE, *PWINBIO_SENSOR_MODE;

#define WINBIO_SENSOR_UNKNOWN_MODE      ((WINBIO_SENSOR_MODE)0)
#define WINBIO_SENSOR_BASIC_MODE        ((WINBIO_SENSOR_MODE)1)
#define WINBIO_SENSOR_ADVANCED_MODE     ((WINBIO_SENSOR_MODE)2)
#define WINBIO_SENSOR_NAVIGATION_MODE   ((WINBIO_SENSOR_MODE)3)
#define WINBIO_SENSOR_SLEEP_MODE        ((WINBIO_SENSOR_MODE)4)

///////////////////////////////////////////////////////////////////////////////
//
// Factor-specific value giving additional information about
// a biometric measurement (e.g., *which* finger a fingerprint
// sample was taken from).
//
typedef UCHAR WINBIO_BIOMETRIC_SUBTYPE, *PWINBIO_BIOMETRIC_SUBTYPE;

#define WINBIO_SUBTYPE_NO_INFORMATION       ((WINBIO_BIOMETRIC_SUBTYPE)0x00)
#define WINBIO_SUBTYPE_ANY                  ((WINBIO_BIOMETRIC_SUBTYPE)0xFF)
//
// If the biometric type is WINBIO_TYPE_FINGERPRINT, WinBio uses
// 'WINBIO_ANSI_381_POS_xyz' constants to represent fingerprint sub-type
// information. (These are defined below in the WINBIO_BDB_ANSI_381_RECORD.)
//
// If the biometric type is WINBIO_TYPE_FACIAL_FEATURES, WinBio uses
// 'WINBIO_ANSI_385_xyz' constants to represent facial image sub-type
// information.
//

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

///////////////////////////////////////////////////////////////////////////////
//
// Biometric unit security level
//
typedef ULONG WINBIO_UNIT_SECURITY_LEVEL, *PWINBIO_UNIT_SECURITY_LEVEL;

#define WINBIO_UNIT_SECURITY_LEVEL_NORMAL   ((WINBIO_UNIT_SECURITY_LEVEL)0)
#define WINBIO_UNIT_SECURITY_LEVEL_VBS      ((WINBIO_UNIT_SECURITY_LEVEL)1)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

///////////////////////////////////////////////////////////////////////////////
//
// Factor-specific value that describes the reason a
// biometric sampling operation failed.
//
typedef ULONG WINBIO_REJECT_DETAIL, *PWINBIO_REJECT_DETAIL;

#ifndef WINBIO_REJECT_DETAILS_DEFINED
#define WINBIO_REJECT_DETAILS_DEFINED

//
// Reject detail values for WINBIO_TYPE_FINGERPRINT
//
#define WINBIO_FP_TOO_HIGH          ((WINBIO_REJECT_DETAIL)1)
#define WINBIO_FP_TOO_LOW           ((WINBIO_REJECT_DETAIL)2)
#define WINBIO_FP_TOO_LEFT          ((WINBIO_REJECT_DETAIL)3)
#define WINBIO_FP_TOO_RIGHT         ((WINBIO_REJECT_DETAIL)4)
#define WINBIO_FP_TOO_FAST          ((WINBIO_REJECT_DETAIL)5)
#define WINBIO_FP_TOO_SLOW          ((WINBIO_REJECT_DETAIL)6)
#define WINBIO_FP_POOR_QUALITY      ((WINBIO_REJECT_DETAIL)7)
#define WINBIO_FP_TOO_SKEWED        ((WINBIO_REJECT_DETAIL)8)
#define WINBIO_FP_TOO_SHORT         ((WINBIO_REJECT_DETAIL)9)
#define WINBIO_FP_MERGE_FAILURE     ((WINBIO_REJECT_DETAIL)10)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

//
// REJECT_DETAIL values for IRIS and FACIAL_FEATURES contain multiple fields:
//
//  - Flags requesting proof-of-liveness behaviors from the user            [0xFF000000]
//  - Flags indicating of position errors                                   [0x00FF0000]
//  - A (single) enumerated value explaining the reason for the rejection.  [0x0000FFFF]
//
// This mask covers the upper 8 bits of the reject detail value, where
// the proof-of-liveness behaviors are located.
//
#define WINBIO_REJECT_DETAIL_ANTI_SPOOF_MASK    ((WINBIO_REJECT_DETAIL)0xFF000000)

//
// Anti-spoofing behaviors...
//
#define WINBIO_ANTI_SPOOF_TURN_SIDE_TO_SIDE     ((WINBIO_REJECT_DETAIL)0x01000000)

//
// This mask covers the range of bits devoted to position errors...
//
#define WINBIO_REJECT_DETAIL_POSITION_MASK      ((WINBIO_REJECT_DETAIL)0x00FF0000)

//
// This mask covers the lower 16 bits, where the enumerated reason for the
// rejection is located.
//
#define WINBIO_REJECT_DETAIL_REASON_MASK        ((WINBIO_REJECT_DETAIL)0x0000FFFF)

//
// Reject detail values for WINBIO_TYPE_IRIS
//
#define WINBIO_IRIS_POOR_QUALITY            ((WINBIO_REJECT_DETAIL)1)
#define WINBIO_IRIS_TOO_BRIGHT              ((WINBIO_REJECT_DETAIL)2)
#define WINBIO_IRIS_TOO_DARK                ((WINBIO_REJECT_DETAIL)3)
#define WINBIO_IRIS_SPOOF_DETECTED          ((WINBIO_REJECT_DETAIL)4)
#define WINBIO_IRIS_TOO_SKEWED              ((WINBIO_REJECT_DETAIL)5)
#define WINBIO_IRIS_TOO_CLOSED              ((WINBIO_REJECT_DETAIL)6)
#define WINBIO_IRIS_GLARE                   ((WINBIO_REJECT_DETAIL)7)
#define WINBIO_IRIS_DIRTY_LENS              ((WINBIO_REJECT_DETAIL)8)
#define WINBIO_IRIS_POOR_FOCUS              ((WINBIO_REJECT_DETAIL)9)

// Camera orientation doesn't match mandatory value in EXTENDED_SENSOR_INFO
#define WINBIO_IRIS_WRONG_ORIENTATION       ((WINBIO_REJECT_DETAIL)10)

//
// Positioning errors...
//
#define WINBIO_IRIS_TOO_HIGH                ((WINBIO_REJECT_DETAIL)0x00010000)
#define WINBIO_IRIS_TOO_LOW                 ((WINBIO_REJECT_DETAIL)0x00020000)
#define WINBIO_IRIS_TOO_LEFT                ((WINBIO_REJECT_DETAIL)0x00040000)
#define WINBIO_IRIS_TOO_RIGHT               ((WINBIO_REJECT_DETAIL)0x00080000)
#define WINBIO_IRIS_TOO_NEAR                ((WINBIO_REJECT_DETAIL)0x00100000)
#define WINBIO_IRIS_TOO_FAR                 ((WINBIO_REJECT_DETAIL)0x00200000)

//
// Reject detail values for WINBIO_TYPE_FACIAL_FEATURES
//
#define WINBIO_FACE_POOR_QUALITY            ((WINBIO_REJECT_DETAIL)1)
#define WINBIO_FACE_TOO_BRIGHT              ((WINBIO_REJECT_DETAIL)2)
#define WINBIO_FACE_TOO_DARK                ((WINBIO_REJECT_DETAIL)3)
//
// SPOOF_DETECTED indicates that the recognition component believes the
// face is not live, but is coming from a replayed video feed, a photo,
// or a 3-D sculpture.
//
#define WINBIO_FACE_SPOOF_DETECTED          ((WINBIO_REJECT_DETAIL)4)
#define WINBIO_FACE_AMBIGUOUS_TARGET        ((WINBIO_REJECT_DETAIL)5)
// WINBIO_FACE_EYES_OCCLUDED is deprecated. Use WINBIO_FACE_OCCLUDED instead
#define WINBIO_FACE_EYES_OCCLUDED           ((WINBIO_REJECT_DETAIL)6)
// WINBIO_FACE_OCCLUDED indicates that the user's eyes, mouth or nose are occluded.
// This is create to replace WINBIO_FACE_EYES_OCCLUDED
#define WINBIO_FACE_OCCLUDED                ((WINBIO_REJECT_DETAIL)6)

// Camera orientation doesn't match mandatory value in EXTENDED_SENSOR_INFO
#define WINBIO_FACE_WRONG_ORIENTATION       ((WINBIO_REJECT_DETAIL)7)

//
// Positioning errors...
//
#define WINBIO_FACE_TOO_HIGH                ((WINBIO_REJECT_DETAIL)0x00010000)
#define WINBIO_FACE_TOO_LOW                 ((WINBIO_REJECT_DETAIL)0x00020000)
#define WINBIO_FACE_TOO_LEFT                ((WINBIO_REJECT_DETAIL)0x00040000)
#define WINBIO_FACE_TOO_RIGHT               ((WINBIO_REJECT_DETAIL)0x00080000)
#define WINBIO_FACE_TOO_NEAR                ((WINBIO_REJECT_DETAIL)0x00100000)
#define WINBIO_FACE_TOO_FAR                 ((WINBIO_REJECT_DETAIL)0x00200000)

//
// Reject detail values for WINBIO_TYPE_VOICE
//
#define WINBIO_VOICE_POOR_QUALITY           ((WINBIO_REJECT_DETAIL)1)
#define WINBIO_VOICE_TOO_SLOW               ((WINBIO_REJECT_DETAIL)2)
#define WINBIO_VOICE_TOO_FAST               ((WINBIO_REJECT_DETAIL)3)
#define WINBIO_VOICE_NO_KEYWORD             ((WINBIO_REJECT_DETAIL)4)
#define WINBIO_VOICE_PROCESSING_ERROR       ((WINBIO_REJECT_DETAIL)5)

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#endif // WINBIO_REJECT_DETAILS_DEFINED


///////////////////////////////////////////////////////////////////////////////
//
// Biometric Information Record (BIR)
//
///////////////////////////////////////////////////////////////////////////////
//
//      +---------------------------------------+
// 1.   |   WINBIO_BIR                          |
//      +---------------------------------------+
// 2.   |   WINBIO_BIR_HEADER                   |
//      +---------------------------------------+
// 3.   |   Standard Data Block (optional)      |
//      |                                       |
//      |       WINBIO_BDB_ANSI_381_HEADER      |
//      |       [0] WINBIO_BDB_ANSI_381_RECORD  |
//      |           :                           |
//      |       [N] WINBIO_BDB_ANSI_381_RECORD  |
//      +---------------------------------------+
// 4.   |   Vendor Data Block (optional)        |
//      +---------------------------------------+
// 5.   |   Signature Block (optional)          |
//      +---------------------------------------+
//
// NOTES:
//      - The format of the Standard Data Block is determined
//      by the 'BiometricDataFormat' field of WINBIO_BIR_HEADER.
//
//      - Currently, the only supported format for the Standard
//      Data Block is ANSI 381 fingerprint image data. Data in
//      any other form must go into the Vendor Data Block instead.
//
//      - It's vital that BIR structures and sub-structures be aligned
//      on 8-byte boundaries. This means both C++ and MIDL compilation
//      with at least /Zp8 alignment. This also applies to BIRs that
//      are allocated dynamically from the heap.
//
#define WINBIO_BIR_ALIGN_SIZE   (8)
//
// Also define legacy typo to be synonymous
//
#define WINBIO_BIR_ALGIN_SIZE   WINBIO_BIR_ALIGN_SIZE

//
// The following gives the location and size of a block
// in a BIR. The offset is measured from the beginning of
// the WINBIO_BIR structure.
//
typedef struct _WINBIO_BIR_DATA {
    ULONG Size;
    ULONG Offset;
} WINBIO_BIR_DATA;

typedef WINBIO_BIR_DATA *PWINBIO_BIR_DATA;

//
// Top-level structure contains offset/size
// information needed to find other items.
//
typedef struct _WINBIO_BIR {
    WINBIO_BIR_DATA HeaderBlock;
    WINBIO_BIR_DATA StandardDataBlock;
    WINBIO_BIR_DATA VendorDataBlock;
    WINBIO_BIR_DATA SignatureBlock;
} WINBIO_BIR;

typedef WINBIO_BIR *PWINBIO_BIR;

///////////////////////////////////////////////////////////////////////////////
//
// Elements used in a BIR header
//
///////////////////////////////////////////////////////////////////////////////
//
// BIR 'ValidFields' flags...
//
#define WINBIO_BIR_FIELD_SUBHEAD_COUNT          ((USHORT)0x0001)
#define WINBIO_BIR_FIELD_PRODUCT_ID             ((USHORT)0x0002)
#define WINBIO_BIR_FIELD_PATRON_ID              ((USHORT)0x0004)
#define WINBIO_BIR_FIELD_INDEX                  ((USHORT)0x0008)

#define WINBIO_BIR_FIELD_CREATION_DATE          ((USHORT)0x0010)
#define WINBIO_BIR_FIELD_VALIDITY_PERIOD        ((USHORT)0x0020)
#define WINBIO_BIR_FIELD_BIOMETRIC_TYPE         ((USHORT)0x0040)
#define WINBIO_BIR_FIELD_BIOMETRIC_SUBTYPE      ((USHORT)0x0080)

#define WINBIO_BIR_FIELD_CBEFF_HEADER_VERSION   ((USHORT)0x0100)
#define WINBIO_BIR_FIELD_PATRON_HEADER_VERSION  ((USHORT)0x0200)
#define WINBIO_BIR_FIELD_BIOMETRIC_PURPOSE      ((USHORT)0x0400)
#define WINBIO_BIR_FIELD_BIOMETRIC_CONDITION    ((USHORT)0x0800)

#define WINBIO_BIR_FIELD_QUALITY                ((USHORT)0x1000)
#define WINBIO_BIR_FIELD_CREATOR                ((USHORT)0x2000)
#define WINBIO_BIR_FIELD_CHALLENGE              ((USHORT)0x4000)
#define WINBIO_BIR_FIELD_PAYLOAD                ((USHORT)0x8000)

//
// The following collection of optional fields will NEVER be
// part of a WinBio BIR. If any of these bits are asserted,
// the BIR is malformed.
//
#define WINBIO_BIR_FIELD_NEVER_VALID    (WINBIO_BIR_FIELD_SUBHEAD_COUNT |   \
                                         WINBIO_BIR_FIELD_PATRON_ID |       \
                                         WINBIO_BIR_FIELD_INDEX |           \
                                         WINBIO_BIR_FIELD_CHALLENGE |       \
                                         WINBIO_BIR_FIELD_PAYLOAD )

///////////////////////////////////////////////////////////////////////////////
//
// BIR 'HeaderVersion' and 'PatronHeaderVersion' fields:
//
// Versions are represented as 8-bit values of the
// form: 0xNM, where 'N' is the major version and 'M'
// is the minor version.
//
typedef UCHAR WINBIO_BIR_VERSION, *PWINBIO_BIR_VERSION;

#define WINBIO_CBEFF_HEADER_VERSION     ((WINBIO_BIR_VERSION)0x11)
#define WINBIO_PATRON_HEADER_VERSION    ((WINBIO_BIR_VERSION)0x11)

///////////////////////////////////////////////////////////////////////////////
//
// BIR 'DataFlags' field:
//      * Security and integrity-checking options
//          PRIVACY     - BDB is encrypted
//          INTEGRITY   - BDB is signed or MAC'ed
//          SIGNED      - 1 -> BDB is signed; 0 -> BDB is MAC'ed
//      * Processing level of the data
//
typedef UCHAR WINBIO_BIR_DATA_FLAGS, *PWINBIO_BIR_DATA_FLAGS;

#define WINBIO_DATA_FLAG_PRIVACY                ((UCHAR)0x02)
#define WINBIO_DATA_FLAG_INTEGRITY              ((UCHAR)0x01)
#define WINBIO_DATA_FLAG_SIGNED                 ((UCHAR)0x04)

#define WINBIO_DATA_FLAG_RAW                    ((UCHAR)0x20)
#define WINBIO_DATA_FLAG_INTERMEDIATE           ((UCHAR)0x40)
#define WINBIO_DATA_FLAG_PROCESSED              ((UCHAR)0x80)

#define WINBIO_DATA_FLAG_OPTION_MASK_PRESENT    ((UCHAR)0x08)   // Always '1'.

///////////////////////////////////////////////////////////////////////////////
//
// BIR 'Purpose' field:
//
// A value defining the purpose for which the BIR
//
//  - is intended, when used as input to a WinBio function
//
//  - is suitable, when used as output from a WinBio function
//  or within a BIR header.
//
// NOTE:
//  In a WINBIO BIR, the 'Purpose' field is defined as a set of flag bits
//  rather than an enumerated type (as specified in NISTIR 6529-A). Transferring
//  a WINBIO BIR to another environment (e.g., BioAPI) will require conversion.
//
//  The suggested way to handle this conversion is to generate a set of nested
//  BIRs for any WINBIO BIRs that have multiple 'Purpose' bits set.
//
typedef UCHAR WINBIO_BIR_PURPOSE, *PWINBIO_BIR_PURPOSE;

#define WINBIO_NO_PURPOSE_AVAILABLE                     ((WINBIO_BIR_PURPOSE)0x00)
#define WINBIO_PURPOSE_VERIFY                           ((WINBIO_BIR_PURPOSE)0x01)
#define WINBIO_PURPOSE_IDENTIFY                         ((WINBIO_BIR_PURPOSE)0x02)
#define WINBIO_PURPOSE_ENROLL                           ((WINBIO_BIR_PURPOSE)0x04)
#define WINBIO_PURPOSE_ENROLL_FOR_VERIFICATION          ((WINBIO_BIR_PURPOSE)0x08)
#define WINBIO_PURPOSE_ENROLL_FOR_IDENTIFICATION        ((WINBIO_BIR_PURPOSE)0x10)
#define WINBIO_PURPOSE_AUDIT                            ((WINBIO_BIR_PURPOSE)0x80)

///////////////////////////////////////////////////////////////////////////////
//
// BIR 'DataQuality' field:
//
// FIndicates the relative quality of the biometric
// data in the BIR.
//
// Quality measurements are represented as signed
// integers in the range 0-100, except:
//
//      -1  Quality measurements are supported by the
//          BIR creator, but no value is set in the BIR.
//
//      -2  Quality measurements are not supported
//          by the BIR creator.
//
typedef CHAR WINBIO_BIR_QUALITY, *PWINBIO_BIR_QUALITY;

#define WINBIO_DATA_QUALITY_NOT_SET         ((WINBIO_BIR_QUALITY)-1)
#define WINBIO_DATA_QUALITY_NOT_SUPPORTED   ((WINBIO_BIR_QUALITY)-2)

///////////////////////////////////////////////////////////////////////////////
//
// BIR 'BiometricDataFormat' and 'ProductId' fields:
//
// Identifies a registered data format as a pair consisting of
// an IBIA-assigned owner value plus an owner-assigned format-type
// value.
//
typedef struct _WINBIO_REGISTERED_FORMAT {
    USHORT Owner;
    USHORT Type;
} WINBIO_REGISTERED_FORMAT, *PWINBIO_REGISTERED_FORMAT;

#define WINBIO_NO_FORMAT_OWNER_AVAILABLE    ((USHORT)0)
#define WINBIO_NO_FORMAT_TYPE_AVAILABLE     ((USHORT)0)

///////////////////////////////////////////////////////////////////////////////
//
// NISTIR 6529-A -- Common Biometric Exchange Formats Framework (CBEFF)
// April 5, 2004
//
// CBEFF Patron Format A: Standard Biometric Header Block
//
// NOTE:
//      This structure is COMPATIBLE with CBEFF Patron Format A in that
//      it can be transformed to/from a fully-conformant Format A record.
//      The following fields are NOT part of this structure:
//          - Subheader -- WinBio doesn't support nested CBEFF structures
//          - Patron format owner/type -- used only for nested CBEFF structures
//          - Index
//          - Creator
//          - Challenge/response
//          - Payload
//
///////////////////////////////////////////////////////////////////////////////
typedef struct _WINBIO_BIR_HEADER {
    //
    // Mask indicating which fields are valid
    //
    // Annex A -- Note (1) to Table A.1
    //
    USHORT ValidFields;

    //
    // CBEFF Header version
    //
    // SECTION 5.2.1.3
    //
    WINBIO_BIR_VERSION HeaderVersion;   // = WINBIO_CBEFF_HEADER_VERSION

    //
    // Patron header version
    //
    // SECTION 5.2.1.4
    //
    WINBIO_BIR_VERSION PatronHeaderVersion; // = WINBIO_PATRON_HEADER_VERSION

    //
    // 'DataFlags' is a combination of the 'Security
    // Options' field and the 'Biometric Data Type'
    // (RAW, INTERMEDIATE,PROCESSED) field.
    //
    // SECTION 5.2.1.1, 5.2.1.2, and 5.2.1.7
    //
    WINBIO_BIR_DATA_FLAGS DataFlags;

    //
    // Biometric type
    //
    // SECTION 5.2.1.5
    //
    WINBIO_BIOMETRIC_TYPE Type;

    //
    // Biometric subtype
    //
    // SECTION 5.2.1.6
    //
    WINBIO_BIOMETRIC_SUBTYPE Subtype;

    //
    // Intended use of the data
    //
    // SECTION 5.2.1.8
    //
    WINBIO_BIR_PURPOSE Purpose;

    //
    // Biometric data quality
    //
    // SECTION 5.2.1.9
    //
    WINBIO_BIR_QUALITY DataQuality;

    //
    // Creation date and time of this BIR (in UTC)
    //
    // SECTION 5.2.1.10
    //
    LARGE_INTEGER CreationDate;

    //
    // Validity period of this BIR (in UTC)
    //
    // SECTION 5.2.1.11
    //
    struct {
        LARGE_INTEGER BeginDate;
        LARGE_INTEGER EndDate;
    } ValidityPeriod;

    ///////////////////////////////////////////////////////////////////////////
    //
    // Data format of Standard Data Block.
    //
    // NOTE: If BIR doesn't contain 'StandardDataBlock'
    // element, this is set to "no owner/type available".
    //
    WINBIO_REGISTERED_FORMAT BiometricDataFormat;

    //
    // Product identifier for the component that
    // generated the 'StandardDataBlock' element.
    //
    // NOTE: If BIR doesn't contain 'StandardDataBlock'
    // element, this is set to "no owner/type available".
    //
    WINBIO_REGISTERED_FORMAT ProductId;

} WINBIO_BIR_HEADER;

typedef WINBIO_BIR_HEADER *PWINBIO_BIR_HEADER;


///////////////////////////////////////////////////////////////////////////////
//
// ANSI INCITS 381-2004 -- Finger Image-Based Data Interchange Format
//
///////////////////////////////////////////////////////////////////////////////
//
// SECTION 7 -- Registered Format
//
// WINBIO_BIR_HEADER.BiometricDataFormat.Owner = WINBIO_ANSI_381_FORMAT_OWNER
// WINBIO_BIR_HEADER.BiometricDataFormat.Type  = WINBIO_ANSI_381_FORMAT_TYPE
//
#define WINBIO_ANSI_381_FORMAT_OWNER    ((USHORT)0x001B)    // INCITS Technical Committee M1
#define WINBIO_ANSI_381_FORMAT_TYPE     ((USHORT)0x0401)    // ANSI-381

//
// SECTION 7.1 -- General Record Header
//
typedef struct _WINBIO_BDB_ANSI_381_HEADER {
    ULONG64 RecordLength;               // Only the low 6 bytes of this number are valid.
                                        // sizeof(WINBIO_BDB_ANSI_381_HEADER)
                                        // + sizeof( all WINBIO_BDB_ANSI_381_RECORD records)

    ULONG FormatIdentifier;             // Must be 0x46495200 (ASCII: 'F' 'I' 'R' 0x0)
    ULONG VersionNumber;                // Must be 0x30313000 (ASCII: '0' '1' '0' 0x0)

    WINBIO_REGISTERED_FORMAT ProductId;

    USHORT CaptureDeviceId;
    USHORT ImageAcquisitionLevel;
    USHORT HorizontalScanResolution;
    USHORT VerticalScanResolution;
    USHORT HorizontalImageResolution;
    USHORT VerticalImageResolution;

    UCHAR ElementCount;                 // Number of finger/palm records in the block
    UCHAR ScaleUnits;                   // cm or inch
    UCHAR PixelDepth;                   // 1-16 bits per pixel (2-64K gray levels)
    UCHAR ImageCompressionAlg;

    USHORT Reserved;
    // 38 bytes (because of unused 2 bytes in ULONG64) vs. 36 bytes defined in ANSI INCITS spec

} WINBIO_BDB_ANSI_381_HEADER;

typedef WINBIO_BDB_ANSI_381_HEADER *PWINBIO_BDB_ANSI_381_HEADER;

//
// SECTION 7.1.6 Image acquistion level
//
// Table 1 -- Image acquistion setting levels
//
#define WINBIO_ANSI_381_IMG_ACQ_LEVEL_10    ((USHORT)10)
#define WINBIO_ANSI_381_IMG_ACQ_LEVEL_20    ((USHORT)20)
#define WINBIO_ANSI_381_IMG_ACQ_LEVEL_30    ((USHORT)30)
#define WINBIO_ANSI_381_IMG_ACQ_LEVEL_31    ((USHORT)31)
#define WINBIO_ANSI_381_IMG_ACQ_LEVEL_40    ((USHORT)40)
#define WINBIO_ANSI_381_IMG_ACQ_LEVEL_41    ((USHORT)41)

//
// SECTION 7.1.8 -- Scale units
//
#define WINBIO_ANSI_381_PIXELS_PER_INCH     ((UCHAR)0x01)
#define WINBIO_ANSI_381_PIXELS_PER_CM       ((UCHAR)0x02)

//
// SECTION 7.1.14 -- Image compression algorithm
//
// Table 3 -- Compression algorithm codes
//
#define WINBIO_ANSI_381_IMG_UNCOMPRESSED            ((UCHAR)0)
#define WINBIO_ANSI_381_IMG_BIT_PACKED              ((UCHAR)1)
#define WINBIO_ANSI_381_IMG_COMPRESSED_WSQ          ((UCHAR)2)
#define WINBIO_ANSI_381_IMG_COMPRESSED_JPEG         ((UCHAR)3)
#define WINBIO_ANSI_381_IMG_COMPRESSED_JPEG2000     ((UCHAR)4)
#define WINBIO_ANSI_381_IMG_COMPRESSED_PNG          ((UCHAR)5)

//
// SECTION 7.2 -- Finger Record Header
//
typedef struct _WINBIO_BDB_ANSI_381_RECORD {
    ULONG BlockLength;                  // sizeof(WINBIO_BDB_ANSI_381_RECORD)
                                        // + sizeof(image data)
    USHORT HorizontalLineLength;        // Number of pixels in a horizontal line
    USHORT VerticalLineLength;          // Number of horizontal lines in the image
    WINBIO_BIOMETRIC_SUBTYPE Position;
    UCHAR CountOfViews;                 // Must be set to one (1)
    UCHAR ViewNumber;                   // Must be set to one (1)
    UCHAR ImageQuality;                 // Reserved -- must be set to 254 (0xFE)
    UCHAR ImpressionType;
    UCHAR Reserved;                     // Must be set to zero
    // 14 bytes of header data
} WINBIO_BDB_ANSI_381_RECORD;
//
// Followed immediately by compressed/uncompressed image data.
// (Image data must be < 43 * 10^8 bytes in length.)
//

typedef WINBIO_BDB_ANSI_381_RECORD *PWINBIO_BDB_ANSI_381_RECORD;

//
// SECTION 7.2.2 -- Finger position codes
//
// Table 5 -- Finger position codes, areas, and maximum dimensions
//
#define WINBIO_ANSI_381_POS_UNKNOWN             ((WINBIO_BIOMETRIC_SUBTYPE)0)
#define WINBIO_ANSI_381_POS_RH_THUMB            ((WINBIO_BIOMETRIC_SUBTYPE)1)
#define WINBIO_ANSI_381_POS_RH_INDEX_FINGER     ((WINBIO_BIOMETRIC_SUBTYPE)2)
#define WINBIO_ANSI_381_POS_RH_MIDDLE_FINGER    ((WINBIO_BIOMETRIC_SUBTYPE)3)
#define WINBIO_ANSI_381_POS_RH_RING_FINGER      ((WINBIO_BIOMETRIC_SUBTYPE)4)
#define WINBIO_ANSI_381_POS_RH_LITTLE_FINGER    ((WINBIO_BIOMETRIC_SUBTYPE)5)
#define WINBIO_ANSI_381_POS_LH_THUMB            ((WINBIO_BIOMETRIC_SUBTYPE)6)
#define WINBIO_ANSI_381_POS_LH_INDEX_FINGER     ((WINBIO_BIOMETRIC_SUBTYPE)7)
#define WINBIO_ANSI_381_POS_LH_MIDDLE_FINGER    ((WINBIO_BIOMETRIC_SUBTYPE)8)
#define WINBIO_ANSI_381_POS_LH_RING_FINGER      ((WINBIO_BIOMETRIC_SUBTYPE)9)
#define WINBIO_ANSI_381_POS_LH_LITTLE_FINGER    ((WINBIO_BIOMETRIC_SUBTYPE)10)

#define WINBIO_ANSI_381_POS_RH_FOUR_FINGERS     ((WINBIO_BIOMETRIC_SUBTYPE)13)
#define WINBIO_ANSI_381_POS_LH_FOUR_FINGERS     ((WINBIO_BIOMETRIC_SUBTYPE)14)
#define WINBIO_ANSI_381_POS_TWO_THUMBS          ((WINBIO_BIOMETRIC_SUBTYPE)15)
//
// Table 6 -- Palm codes, areas, and maximum dimensions
//
#define WINBIO_ANSI_381_POS_UNKNOWN_PALM        ((WINBIO_BIOMETRIC_SUBTYPE)20)
#define WINBIO_ANSI_381_POS_RH_FULL_PALM        ((WINBIO_BIOMETRIC_SUBTYPE)21)
#define WINBIO_ANSI_381_POS_RH_WRITERS_PALM     ((WINBIO_BIOMETRIC_SUBTYPE)22)
#define WINBIO_ANSI_381_POS_LH_FULL_PALM        ((WINBIO_BIOMETRIC_SUBTYPE)23)
#define WINBIO_ANSI_381_POS_LH_WRITERS_PALM     ((WINBIO_BIOMETRIC_SUBTYPE)24)
#define WINBIO_ANSI_381_POS_RH_LOWER_PALM       ((WINBIO_BIOMETRIC_SUBTYPE)25)
#define WINBIO_ANSI_381_POS_RH_UPPER_PALM       ((WINBIO_BIOMETRIC_SUBTYPE)26)
#define WINBIO_ANSI_381_POS_LH_LOWER_PALM       ((WINBIO_BIOMETRIC_SUBTYPE)27)
#define WINBIO_ANSI_381_POS_LH_UPPER_PALM       ((WINBIO_BIOMETRIC_SUBTYPE)28)
#define WINBIO_ANSI_381_POS_RH_OTHER            ((WINBIO_BIOMETRIC_SUBTYPE)29)
#define WINBIO_ANSI_381_POS_LH_OTHER            ((WINBIO_BIOMETRIC_SUBTYPE)30)
#define WINBIO_ANSI_381_POS_RH_INTERDIGITAL     ((WINBIO_BIOMETRIC_SUBTYPE)31)
#define WINBIO_ANSI_381_POS_RH_THENAR           ((WINBIO_BIOMETRIC_SUBTYPE)32)
#define WINBIO_ANSI_381_POS_RH_HYPOTHENAR       ((WINBIO_BIOMETRIC_SUBTYPE)33)
#define WINBIO_ANSI_381_POS_LH_INTERDIGITAL     ((WINBIO_BIOMETRIC_SUBTYPE)34)
#define WINBIO_ANSI_381_POS_LH_THENAR           ((WINBIO_BIOMETRIC_SUBTYPE)35)
#define WINBIO_ANSI_381_POS_LH_HYPOTHENAR       ((WINBIO_BIOMETRIC_SUBTYPE)36)

//
// SECTION 7.2.6 -- Impression-Type Codes
//
// Table 7 -- Finger and palm impression types
//
#define WINBIO_ANSI_381_IMP_TYPE_LIVE_SCAN_PLAIN        ((UCHAR)0)
#define WINBIO_ANSI_381_IMP_TYPE_LIVE_SCAN_ROLLED       ((UCHAR)1)
#define WINBIO_ANSI_381_IMP_TYPE_NONLIVE_SCAN_PLAIN     ((UCHAR)2)
#define WINBIO_ANSI_381_IMP_TYPE_NONLIVE_SCAN_ROLLED    ((UCHAR)3)
#define WINBIO_ANSI_381_IMP_TYPE_LATENT                 ((UCHAR)7)
#define WINBIO_ANSI_381_IMP_TYPE_SWIPE                  ((UCHAR)8)
#define WINBIO_ANSI_381_IMP_TYPE_LIVE_SCAN_CONTACTLESS  ((UCHAR)9)


#if (NTDDI_VERSION >= NTDDI_WIN9)

//
// Sub-factor values for untagged fingerprints.
// Microsoft-specific extension -- not part of ANSI/ISO standard
//
#define WINBIO_FINGER_UNSPECIFIED_POS_01        ((WINBIO_BIOMETRIC_SUBTYPE)0xF5)
#define WINBIO_FINGER_UNSPECIFIED_POS_02        ((WINBIO_BIOMETRIC_SUBTYPE)0xF6)
#define WINBIO_FINGER_UNSPECIFIED_POS_03        ((WINBIO_BIOMETRIC_SUBTYPE)0xF7)
#define WINBIO_FINGER_UNSPECIFIED_POS_04        ((WINBIO_BIOMETRIC_SUBTYPE)0xF8)
#define WINBIO_FINGER_UNSPECIFIED_POS_05        ((WINBIO_BIOMETRIC_SUBTYPE)0xF9)
#define WINBIO_FINGER_UNSPECIFIED_POS_06        ((WINBIO_BIOMETRIC_SUBTYPE)0xFA)
#define WINBIO_FINGER_UNSPECIFIED_POS_07        ((WINBIO_BIOMETRIC_SUBTYPE)0xFB)
#define WINBIO_FINGER_UNSPECIFIED_POS_08        ((WINBIO_BIOMETRIC_SUBTYPE)0xFC)
#define WINBIO_FINGER_UNSPECIFIED_POS_09        ((WINBIO_BIOMETRIC_SUBTYPE)0xFD)
#define WINBIO_FINGER_UNSPECIFIED_POS_10        ((WINBIO_BIOMETRIC_SUBTYPE)0xFE)

#endif // (NTDDI_VERSION >= NTDDI_WIN9)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
//
// Sub-factor values for face recognition, taken from:
//
//      ANSI/INCITS 385-2004: "Face Recognition Format for Data Interchange"
//
// 385-2004 defines two types of frontal face images: full resolution and low
// resolution. In practice, WinBio will use only full resolution images for
// face recognition.
//
#define WINBIO_ANSI_385_FACE_TYPE_UNKNOWN       ((WINBIO_BIOMETRIC_SUBTYPE)0)
#define WINBIO_ANSI_385_FACE_FRONTAL_FULL       ((WINBIO_BIOMETRIC_SUBTYPE)1)
#define WINBIO_ANSI_385_FACE_FRONTAL_TOKEN      ((WINBIO_BIOMETRIC_SUBTYPE)2)

//
// Sub-factor values for iris recognition.
//
// Microsoft-specific extension -- not part of ANSI/ISO standards
// (ANSI/INCITS 379-2004 doesn't define any left/right eye values)
//
#define WINBIO_IRIS_TYPE_UNKNOWN                ((WINBIO_BIOMETRIC_SUBTYPE)0)
#define WINBIO_IRIS_LEFT_EYE                    ((WINBIO_BIOMETRIC_SUBTYPE)0xF5)
#define WINBIO_IRIS_RIGHT_EYE                   ((WINBIO_BIOMETRIC_SUBTYPE)0xF6)
#define WINBIO_IRIS_UNSPECIFIED_POS_01          ((WINBIO_BIOMETRIC_SUBTYPE)0xF7)
#define WINBIO_IRIS_UNSPECIFIED_POS_02          ((WINBIO_BIOMETRIC_SUBTYPE)0xF8)
#define WINBIO_IRIS_BOTH_EYES                   ((WINBIO_BIOMETRIC_SUBTYPE)0xF9)
#define WINBIO_IRIS_EITHER_EYE                  ((WINBIO_BIOMETRIC_SUBTYPE)0xFA)

//
// Sub-factor values for voice recognition.
//
// Microsoft-specific extension. Each "sub-factor" is a separate utterance.
//
// An enrollment appication can create multiple templates for a single SID,
// each template corresponding to a unique "utterance". The enrollment's
// SubFactor value is used to identify a specific utterance. The SubFactor
// can be any value between WINBIO_VOICE_MIN_UTTERANCE and WINBIO_VOICE_MAX_UTTERANCE.
// After enrollment, WinBioIdentify will return the utterance ID through the
// SubFactor return value.
//
#define WINBIO_VOICE_TYPE_UNKNOWN               ((WINBIO_BIOMETRIC_SUBTYPE)0)

#define WINBIO_VOICE_MIN_UTTERANCE              ((WINBIO_BIOMETRIC_SUBTYPE)1)
#define WINBIO_VOICE_MAX_UTTERANCE              ((WINBIO_BIOMETRIC_SUBTYPE)32)

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

///////////////////////////////////////////////////////////////////////////////
//
// Validation header placed at the beginning of every secure buffer.
//
///////////////////////////////////////////////////////////////////////////////
typedef struct _WINBIO_SECURE_BUFFER_HEADER_V1 {
    ULONG Type;                 // WINBIO_SECURE_BUFFER_TYPE_V1
    ULONG Size;                 // sizeof(this structure)
    ULONG Flags;                // Buffer creator must set to zero
    ULONGLONG ValidationTag;    // Buffer creator must set
} WINBIO_SECURE_BUFFER_HEADER_V1, *PWINBIO_SECURE_BUFFER_HEADER_V1;

#define WINBIO_SECURE_BUFFER_TYPE_V1         ((ULONG)0xB9BE0001)

#endif  // NTDDI_VERSION >= NTDDI_WIN10_RS2

//
// Sensor pool type identifiers
//
typedef ULONG WINBIO_POOL_TYPE, *PWINBIO_POOL_TYPE;

#define WINBIO_POOL_UNKNOWN         ((WINBIO_POOL_TYPE)0)
#define WINBIO_POOL_SYSTEM          ((WINBIO_POOL_TYPE)1)
#define WINBIO_POOL_PRIVATE         ((WINBIO_POOL_TYPE)2)
#define WINBIO_POOL_UNASSIGNED      ((WINBIO_POOL_TYPE)3) // Reserved for Microsoft - do not use.

//
// Specific access rights for Biometric Units
//
#define BIO_UNIT_RAW                ((USHORT)0x0001)
#define BIO_UNIT_MAINTENANCE        ((USHORT)0x0002)
#define BIO_UNIT_OPEN_SESSION       ((USHORT)0x0004)
#define BIO_UNIT_EXTENDED_ACCESS    ((USHORT)0x0008)
#define BIO_UNIT_ENROLL             ((USHORT)0x0010)

#if (NTDDI_VERSION >= NTDDI_WIN9)

#define BIO_UNIT_DELETE_TEMPLATE    ((USHORT)0x0020)
#define BIO_UNIT_CONTROL_UNIT       ((USHORT)0x0040)

#endif // (NTDDI_VERSION >= NTDDI_WIN9)

//
// OpenSession parameter that controls session attributes...
//
typedef ULONG WINBIO_SESSION_FLAGS, *PWINBIO_SESSION_FLAGS;
//
// Sensor configuration flags...
//
#define WINBIO_FLAG_DEFAULT         ((WINBIO_SESSION_FLAGS)0x00000000)
#define WINBIO_FLAG_BASIC           ((WINBIO_SESSION_FLAGS)((WINBIO_SENSOR_BASIC_MODE & 0xFFFF)<<16))
#define WINBIO_FLAG_ADVANCED        ((WINBIO_SESSION_FLAGS)((WINBIO_SENSOR_ADVANCED_MODE & 0xFFFF)<<16))
//
// Desired access flags...
//
#define WINBIO_FLAG_RAW             ((WINBIO_SESSION_FLAGS)BIO_UNIT_RAW)
#define WINBIO_FLAG_MAINTENANCE     ((WINBIO_SESSION_FLAGS)BIO_UNIT_MAINTENANCE)

//
// Well-known database IDs used by WinBioOpenSession
//
#define WINBIO_DB_DEFAULT           ((GUID *)1)
#define WINBIO_DB_BOOTSTRAP         ((GUID *)2)
#define WINBIO_DB_ONCHIP            ((GUID *)3)

//
// Identify one of the components in a Biometric Unit's
// processing pipeline (for ControlUnit operations)...
//
typedef ULONG WINBIO_COMPONENT, *PWINBIO_COMPONENT;

#define WINBIO_COMPONENT_SENSOR     ((WINBIO_COMPONENT)1)
#define WINBIO_COMPONENT_ENGINE     ((WINBIO_COMPONENT)2)
#define WINBIO_COMPONENT_STORAGE    ((WINBIO_COMPONENT)3)

//
// Event Monitor stuff...
//
typedef ULONG WINBIO_EVENT_TYPE, *PWINBIO_EVENT_TYPE;

#ifdef MIDL_PASS

//
// Generic events
//
const WINBIO_EVENT_TYPE WINBIO_EVENT_ERROR                  = (WINBIO_EVENT_TYPE)0xFFFFFFFF;

//
// Fingerprint events.
//
const WINBIO_EVENT_TYPE WINBIO_EVENT_FP_UNCLAIMED           = (WINBIO_EVENT_TYPE)0x00000001;
const WINBIO_EVENT_TYPE WINBIO_EVENT_FP_UNCLAIMED_IDENTIFY  = (WINBIO_EVENT_TYPE)0x00000002;

typedef union switch(WINBIO_EVENT_TYPE Type) _WINBIO_EVENT {

    case WINBIO_EVENT_FP_UNCLAIMED:             struct {
                                                    WINBIO_UNIT_ID UnitId;
                                                    WINBIO_REJECT_DETAIL RejectDetail;
                                                } Unclaimed;

    case WINBIO_EVENT_FP_UNCLAIMED_IDENTIFY:    struct {
                                                    WINBIO_UNIT_ID UnitId;
                                                    WINBIO_IDENTITY Identity;
                                                    WINBIO_BIOMETRIC_SUBTYPE SubFactor;
                                                    WINBIO_REJECT_DETAIL RejectDetail;
                                                } UnclaimedIdentify;

    case WINBIO_EVENT_ERROR:                    struct {
                                                    HRESULT ErrorCode;
                                                } Error;
} WINBIO_EVENT, *PWINBIO_EVENT;

#else // MIDL_PASS

//
// Generic events
//
#define WINBIO_EVENT_ERROR                  ((WINBIO_EVENT_TYPE)0xFFFFFFFF)

//
// Fingerprint events.
//
#define WINBIO_EVENT_FP_UNCLAIMED           ((WINBIO_EVENT_TYPE)0x00000001)
#define WINBIO_EVENT_FP_UNCLAIMED_IDENTIFY  ((WINBIO_EVENT_TYPE)0x00000002)


//
// Event structure passed back to applications.
//
typedef struct _WINBIO_EVENT {
    WINBIO_EVENT_TYPE Type;
    union {
        struct {
            WINBIO_UNIT_ID UnitId;
            WINBIO_REJECT_DETAIL RejectDetail;
        } Unclaimed;
        struct {
            WINBIO_UNIT_ID UnitId;
            WINBIO_IDENTITY Identity;
            WINBIO_BIOMETRIC_SUBTYPE SubFactor;
            WINBIO_REJECT_DETAIL RejectDetail;
        } UnclaimedIdentify;
        struct {
            HRESULT ErrorCode;
        } Error;
    } Parameters;
} WINBIO_EVENT, *PWINBIO_EVENT;

#endif // MIDL_PASS

#if (NTDDI_VERSION >= NTDDI_WIN9)
//
// Tickets used by data protection and credential release APIs
//
typedef ULONG64 WINBIO_PROTECTION_TICKET, *PWINBIO_PROTECTION_TICKET;

#endif // (NTDDI_VERSION >= NTDDI_WIN9)

#define WINBIO_OPAQUE_ENGINE_DATA_ITEM_COUNT    ((ULONG)78)  // Number of ULONG slots in the array

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

///////////////////////////////////////////////////////////////////////////////
//
// Items required by the presence-monitoring components
//
///////////////////////////////////////////////////////////////////////////////
//
// Types of presence-change events recognized by WinBio
//
typedef ULONG WINBIO_PRESENCE_CHANGE, *PWINBIO_PRESENCE_CHANGE;

#define WINBIO_PRESENCE_CHANGE_TYPE_UNKNOWN         ((WINBIO_PRESENCE_CHANGE)0)
#define WINBIO_PRESENCE_CHANGE_TYPE_UPDATE_ALL      ((WINBIO_PRESENCE_CHANGE)1)
#define WINBIO_PRESENCE_CHANGE_TYPE_ARRIVE          ((WINBIO_PRESENCE_CHANGE)2)
#define WINBIO_PRESENCE_CHANGE_TYPE_RECOGNIZE       ((WINBIO_PRESENCE_CHANGE)3)
#define WINBIO_PRESENCE_CHANGE_TYPE_DEPART          ((WINBIO_PRESENCE_CHANGE)4)
#define WINBIO_PRESENCE_CHANGE_TYPE_TRACK           ((WINBIO_PRESENCE_CHANGE)5)

#ifdef MIDL_PASS

//
// Factor-specific extended information about a single presence
//
typedef [switch_type(WINBIO_BIOMETRIC_TYPE)] union _WINBIO_PRESENCE_PROPERTIES {

    [case(WINBIO_NO_TYPE_AVAILABLE)]
        ULONG32 Null;

    [case(WINBIO_TYPE_FACIAL_FEATURES)]
        struct {
            RECT BoundingBox;
            LONG Distance;
            struct {
                WINBIO_UUID AdapterId;
                ULONG Data[WINBIO_OPAQUE_ENGINE_DATA_ITEM_COUNT];
            } OpaqueEngineData;
        // [...] other face-specific properties
        } FacialFeatures;

    [case(WINBIO_TYPE_IRIS)]
        struct {
            RECT EyeBoundingBox_1;
            RECT EyeBoundingBox_2;
            POINT PupilCenter_1;
            POINT PupilCenter_2;
            LONG Distance;
        } Iris;
} WINBIO_PRESENCE_PROPERTIES, *PWINBIO_PRESENCE_PROPERTIES;

//
// A single individual...
//
typedef struct _WINBIO_PRESENCE {
    WINBIO_BIOMETRIC_TYPE Factor;
    WINBIO_BIOMETRIC_SUBTYPE SubFactor;
    HRESULT Status;
    WINBIO_REJECT_DETAIL RejectDetail;
    WINBIO_IDENTITY Identity;
    ULONGLONG TrackingId;
    WINBIO_PROTECTION_TICKET Ticket;
    [switch_is(Factor)] WINBIO_PRESENCE_PROPERTIES Properties;
    struct {
        ULONG Size;
        UCHAR Data[32];
    } Authorization;
} WINBIO_PRESENCE, *PWINBIO_PRESENCE;

#else   // MIDL_PASS

typedef union _WINBIO_PRESENCE_PROPERTIES {
    struct {
        RECT BoundingBox;
        LONG Distance;
        struct {
            WINBIO_UUID AdapterId;
            ULONG Data[WINBIO_OPAQUE_ENGINE_DATA_ITEM_COUNT];
        } OpaqueEngineData;
        // [...] other face-specific properties
    } FacialFeatures;

    struct {
        RECT EyeBoundingBox_1;
        RECT EyeBoundingBox_2;
        POINT PupilCenter_1;
        POINT PupilCenter_2;
        LONG Distance;
    } Iris;

    /*
    struct {
        WINBIO_COORDINATES Location;
        // [...] other voice-specific properties
    } Voice;
    */
} WINBIO_PRESENCE_PROPERTIES, *PWINBIO_PRESENCE_PROPERTIES;

typedef struct _WINBIO_PRESENCE {
    WINBIO_BIOMETRIC_TYPE Factor;
    WINBIO_BIOMETRIC_SUBTYPE SubFactor;
    HRESULT Status;
    WINBIO_REJECT_DETAIL RejectDetail;
    WINBIO_IDENTITY Identity;
    ULONGLONG TrackingId;
    WINBIO_PROTECTION_TICKET Ticket;
    WINBIO_PRESENCE_PROPERTIES Properties;
    struct {
        ULONG Size;
        UCHAR Data[32];
    } Authorization;
} WINBIO_PRESENCE, *PWINBIO_PRESENCE;

#endif  // MIDL_PASS

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

///////////////////////////////////////////////////////////////////////////////
//
// Schemata returned by enumeration APIs...
//
///////////////////////////////////////////////////////////////////////////////
//
// Results from EnumServiceProviders...
//
typedef struct _WINBIO_BSP_SCHEMA {
    WINBIO_BIOMETRIC_TYPE BiometricFactor;
    WINBIO_UUID BspId;
    WINBIO_STRING Description;
    WINBIO_STRING Vendor;
    WINBIO_VERSION Version;
} WINBIO_BSP_SCHEMA, *PWINBIO_BSP_SCHEMA;

//
// Results from EnumBiometricUnits...
//
typedef struct _WINBIO_UNIT_SCHEMA {
    WINBIO_UNIT_ID UnitId;
    WINBIO_POOL_TYPE PoolType;
    WINBIO_BIOMETRIC_TYPE BiometricFactor;
    WINBIO_BIOMETRIC_SENSOR_SUBTYPE SensorSubType;
    WINBIO_CAPABILITIES Capabilities;
    WINBIO_STRING DeviceInstanceId;
    WINBIO_STRING Description;
    WINBIO_STRING Manufacturer;
    WINBIO_STRING Model;
    WINBIO_STRING SerialNumber;
    WINBIO_VERSION FirmwareVersion;
} WINBIO_UNIT_SCHEMA, *PWINBIO_UNIT_SCHEMA;

///////////////////////////////////////////////////////////////////////////////
//
// Definitions for the 'Attributes' database value...
//
///////////////////////////////////////////////////////////////////////////////
#define WINBIO_DATABASE_TYPE_MASK       ((ULONG)0x0000FFFF)
//
// Type values...
//
#define WINBIO_DATABASE_TYPE_FILE       ((ULONG)0x00000001)
#define WINBIO_DATABASE_TYPE_DBMS       ((ULONG)0x00000002)
#define WINBIO_DATABASE_TYPE_ONCHIP     ((ULONG)0x00000003)
#define WINBIO_DATABASE_TYPE_SMARTCARD  ((ULONG)0x00000004)

#define WINBIO_DATABASE_FLAG_MASK       ((ULONG)0xFFFF0000)
//
// Flag values...
//
#define WINBIO_DATABASE_FLAG_REMOVABLE  ((ULONG)0x00010000)
#define WINBIO_DATABASE_FLAG_REMOTE     ((ULONG)0x00020000)

//
// Results from EnumDatabases...
//
typedef struct _WINBIO_STORAGE_SCHEMA {
    WINBIO_BIOMETRIC_TYPE BiometricFactor;
    WINBIO_UUID DatabaseId;
    WINBIO_UUID DataFormat;
    ULONG Attributes;
    WINBIO_STRING FilePath;
    WINBIO_STRING ConnectionString;
} WINBIO_STORAGE_SCHEMA, *PWINBIO_STORAGE_SCHEMA;

///////////////////////////////////////////////////////////////////////////////
//
// Bit flags for WinBio framework change notification...
//
typedef ULONG WINBIO_FRAMEWORK_CHANGE_TYPE, *PWINBIO_FRAMEWORK_CHANGE_TYPE;

#define WINBIO_FRAMEWORK_CHANGE_UNIT            ((WINBIO_FRAMEWORK_CHANGE_TYPE)0x00000001)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#define WINBIO_FRAMEWORK_CHANGE_UNIT_STATUS     ((WINBIO_FRAMEWORK_CHANGE_TYPE)0x00000002)

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS2

///////////////////////////////////////////////////////////////////////////////
//
// Maximum number of bytes in a single capture...
//

#ifdef MIDL_PASS

const ULONG WINBIO_MAX_SAMPLE_BUFFER_SIZE = 0x7FFFFFFF;

#else // MIDL_PASS

#define WINBIO_MAX_SAMPLE_BUFFER_SIZE ((ULONG)0x7FFFFFFF)

#endif // MIDL_PASS

///////////////////////////////////////////////////////////////////////////////
//
// Results of asynchronous operations...
//
///////////////////////////////////////////////////////////////////////////////
//
// Enumeration specifying what kind of operation is being completed...
//
typedef ULONG WINBIO_OPERATION_TYPE, *PWINBIO_OPERATION_TYPE;

//
// Session-based operations...
//
#define WINBIO_OPERATION_NONE                               ((WINBIO_OPERATION_TYPE)0)
#define WINBIO_OPERATION_OPEN                               ((WINBIO_OPERATION_TYPE)1)
#define WINBIO_OPERATION_CLOSE                              ((WINBIO_OPERATION_TYPE)2)
#define WINBIO_OPERATION_VERIFY                             ((WINBIO_OPERATION_TYPE)3)
#define WINBIO_OPERATION_IDENTIFY                           ((WINBIO_OPERATION_TYPE)4)
#define WINBIO_OPERATION_LOCATE_SENSOR                      ((WINBIO_OPERATION_TYPE)5)
#define WINBIO_OPERATION_ENROLL_BEGIN                       ((WINBIO_OPERATION_TYPE)6)
#define WINBIO_OPERATION_ENROLL_CAPTURE                     ((WINBIO_OPERATION_TYPE)7)
#define WINBIO_OPERATION_ENROLL_COMMIT                      ((WINBIO_OPERATION_TYPE)8)
#define WINBIO_OPERATION_ENROLL_DISCARD                     ((WINBIO_OPERATION_TYPE)9)
#define WINBIO_OPERATION_ENUM_ENROLLMENTS                   ((WINBIO_OPERATION_TYPE)10)
#define WINBIO_OPERATION_DELETE_TEMPLATE                    ((WINBIO_OPERATION_TYPE)11)
#define WINBIO_OPERATION_CAPTURE_SAMPLE                     ((WINBIO_OPERATION_TYPE)12)
#define WINBIO_OPERATION_GET_PROPERTY                       ((WINBIO_OPERATION_TYPE)13)
#define WINBIO_OPERATION_SET_PROPERTY                       ((WINBIO_OPERATION_TYPE)14)
#define WINBIO_OPERATION_GET_EVENT                          ((WINBIO_OPERATION_TYPE)15)
#define WINBIO_OPERATION_LOCK_UNIT                          ((WINBIO_OPERATION_TYPE)16)
#define WINBIO_OPERATION_UNLOCK_UNIT                        ((WINBIO_OPERATION_TYPE)17)
#define WINBIO_OPERATION_CONTROL_UNIT                       ((WINBIO_OPERATION_TYPE)18)
#define WINBIO_OPERATION_CONTROL_UNIT_PRIVILEGED            ((WINBIO_OPERATION_TYPE)19)

//
// Framework operations...
//
#define WINBIO_OPERATION_OPEN_FRAMEWORK             ((WINBIO_OPERATION_TYPE)20)
#define WINBIO_OPERATION_CLOSE_FRAMEWORK            ((WINBIO_OPERATION_TYPE)21)
#define WINBIO_OPERATION_ENUM_SERVICE_PROVIDERS     ((WINBIO_OPERATION_TYPE)22)
#define WINBIO_OPERATION_ENUM_BIOMETRIC_UNITS       ((WINBIO_OPERATION_TYPE)23)
#define WINBIO_OPERATION_ENUM_DATABASES             ((WINBIO_OPERATION_TYPE)24)
#define WINBIO_OPERATION_UNIT_ARRIVAL               ((WINBIO_OPERATION_TYPE)25)
#define WINBIO_OPERATION_UNIT_REMOVAL               ((WINBIO_OPERATION_TYPE)26)

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////


///////////////////////////////////////////////////////////////////////////////
//
// Things related to get/set property operations
//
typedef ULONG32 WINBIO_PROPERTY_TYPE, *PWINBIO_PROPERTY_TYPE;

#define WINBIO_PROPERTY_TYPE_SESSION    ((WINBIO_PROPERTY_TYPE)1)
#define WINBIO_PROPERTY_TYPE_UNIT       ((WINBIO_PROPERTY_TYPE)2)
#define WINBIO_PROPERTY_TYPE_TEMPLATE   ((WINBIO_PROPERTY_TYPE)3)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define WINBIO_PROPERTY_TYPE_ACCOUNT    ((WINBIO_PROPERTY_TYPE)4)

#ifdef MIDL_PASS

//
// Limit the size of an ingoing property buffer for the server
//
const ULONG WINBIO_MAX_SET_PROPERTY_BUFFER_SIZE = 0x1000;

#else // MIDL_PASS

#define WINBIO_MAX_SET_PROPERTY_BUFFER_SIZE ((ULONG)0x1000)

#endif // MIDL_PASS

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

typedef ULONG32 WINBIO_PROPERTY_ID, *PWINBIO_PROPERTY_ID;


///////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_TYPE_SESSION properties...
//
///////////////////////////////////////////////////////////////////////////
//
// (None)
//


///////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_TYPE_UNIT properties...
//
///////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_SAMPLE_HINT
//
// Description:
//      Estimate of the maximum number of good biometric
//      samples required to complete an enrollment template.
//
// Access:
//      Read-only
//
// Inputs:
//      SessionHandle - must be valid
//      UnitId - must be valid
//      Identity - must be NULL
//      SubFactor - must be WINBIO_SUBTYPE_NO_INFORMATION
//
// Outputs:
//      PropertyBuffer - points to a ULONG buffer containing the hint
//      PropertyBufferSize - points to a SIZE_T variable containing sizeof(ULONG)
//
#define WINBIO_PROPERTY_SAMPLE_HINT                 ((WINBIO_PROPERTY_ID)1)


#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

///////////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_EXTENDED_SENSOR_INFO
//
// Description:
//      Returns extended information about the capabilities
//      and attributes of the Sensor component connected to
//      a specific biometric unit.
//
// Access:
//      Read-only
//
// Inputs:
//      SessionHandle - must be valid
//      UnitId - must be valid
//      Identity - must be NULL
//      SubFactor - must be WINBIO_SUBTYPE_NO_INFORMATION
//
// Outputs:
//      PropertyBuffer - points to a buffer containing a WINBIO_EXTENDED_SENSOR_INFO structure
//      PropertyBufferSize - points to a SIZE_T variable containing sizeof(WINBIO_EXTENDED_SENSOR_INFO)
//
#define WINBIO_PROPERTY_EXTENDED_SENSOR_INFO        ((WINBIO_PROPERTY_ID)2)

//
// Enumeration for mandatory orientation...
//
typedef ULONG WINBIO_ORIENTATION, *PWINBIO_ORIENTATION;

#define WINBIO_ORIENTATION_UNSPECIFIED      ((WINBIO_ORIENTATION)0)
#define WINBIO_ORIENTATION_LANDSCAPE        ((WINBIO_ORIENTATION)1)
#define WINBIO_ORIENTATION_PORTRAIT         ((WINBIO_ORIENTATION)2)
#define WINBIO_ORIENTATION_ANY              ((WINBIO_ORIENTATION)3)

#ifdef MIDL_PASS

//
// Factor-specific extended information about a single adapter
//
typedef struct _WINBIO_EXTENDED_SENSOR_INFO {
    WINBIO_CAPABILITIES GenericSensorCapabilities;

    WINBIO_BIOMETRIC_TYPE Factor;
    [switch_is(Factor)] union
    {
        [case(WINBIO_NO_TYPE_AVAILABLE)]
            ULONG32 Null;

        [case(WINBIO_TYPE_FACIAL_FEATURES)]
            struct {
                RECT FrameSize;     // Camera frame size, in pixels; (top, left) fields always (0,0)
                POINT FrameOffset;  // Offset of face camera frame from video camera, in pixels. (0, 0) indicates complete overlap
                WINBIO_ORIENTATION MandatoryOrientation;
                struct {
                    WCHAR ColorSensorId[260];
                    WCHAR InfraredSensorId[260];
                    UINT32 InfraredSensorRotationAngle;
                } HardwareInfo;
            } FacialFeatures;

        [case(WINBIO_TYPE_FINGERPRINT)]
            struct {
                ULONG32 Reserved;   // Reserved for future expansion
            } Fingerprint;

        [case(WINBIO_TYPE_IRIS)]
            struct {
                RECT FrameSize;     // Iris camera frame size, in pixels; (top, left) fields always (0,0)
                POINT FrameOffset;  // Offset of iris camera frame from video camera, in pixels. (0, 0) indicates complete overlap
                WINBIO_ORIENTATION MandatoryOrientation;
            } Iris;

        [case(WINBIO_TYPE_VOICE)]
            struct {
                ULONG32 Reserved;   // Reserved for future expansion
            } Voice;
    } Specific;
} WINBIO_EXTENDED_SENSOR_INFO, *PWINBIO_EXTENDED_SENSOR_INFO;

#else   // MIDL_PASS

typedef struct _WINBIO_EXTENDED_SENSOR_INFO {
    WINBIO_CAPABILITIES GenericSensorCapabilities;

    WINBIO_BIOMETRIC_TYPE Factor;
    union
    {
        ULONG32 Null;

        struct {
            RECT FrameSize;     // Camera frame size, in pixels; (top, left) fields always (0,0)
            POINT FrameOffset;  // Offset of face camera frame from video camera, in pixels. (0, 0) indicates complete overlap
            WINBIO_ORIENTATION MandatoryOrientation;
            struct {
                WCHAR ColorSensorId[260];
                WCHAR InfraredSensorId[260];
                UINT32 InfraredSensorRotationAngle;
            } HardwareInfo;
        } FacialFeatures;

        struct {
            ULONG32 Reserved;   // Reserved for future expansion
        } Fingerprint;

        struct {
            RECT FrameSize;     // Iris camera frame size, in pixels; (top, left) fields always (0,0)
            POINT FrameOffset;  // Offset of iris camera frame from video camera, in pixels. (0, 0) indicates complete overlap
            WINBIO_ORIENTATION MandatoryOrientation;
        } Iris;

        struct {
            ULONG32 Reserved;   // Reserved for future expansion
        } Voice;
    } Specific;
} WINBIO_EXTENDED_SENSOR_INFO, *PWINBIO_EXTENDED_SENSOR_INFO;

#endif  // MIDL_PASS

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

///////////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_EXTENDED_ENGINE_INFO
//
// Description:
//      Returns extended information about the capabilities
//      and attributes of the Engine component connected to
//      a specific biometric unit.
//
// Access:
//      Read-only
//
// Inputs:
//      SessionHandle - must be valid
//      UnitId - must be valid
//      Identity - must be NULL
//      SubFactor - must be WINBIO_SUBTYPE_NO_INFORMATION
//
// Outputs:
//      PropertyBuffer - points to a buffer containing a WINBIO_EXTENDED_ENGINE_INFO structure
//      PropertyBufferSize - points to a SIZE_T variable containing sizeof(WINBIO_EXTENDED_ENGINE_INFO)
//
#define WINBIO_PROPERTY_EXTENDED_ENGINE_INFO        ((WINBIO_PROPERTY_ID)3)

//
// Generic Engine Capabilities
//
#define WINBIO_ENG_CAP_ITERATIVE_IMPROVEMENT    ((WINBIO_CAPABILITIES)0x00000001)
#define WINBIO_ENG_CAP_SPOOF_DETECTION          ((WINBIO_CAPABILITIES)0x00000002)

#ifdef MIDL_PASS

typedef struct _WINBIO_EXTENDED_ENGINE_INFO {
    WINBIO_CAPABILITIES GenericEngineCapabilities;

    WINBIO_BIOMETRIC_TYPE Factor;
    [switch_is(Factor)] union
    {
        [case(WINBIO_NO_TYPE_AVAILABLE)]
            ULONG32 Null;

        [case(WINBIO_TYPE_FACIAL_FEATURES)]
            struct {
                WINBIO_CAPABILITIES Capabilities;   // Reserved for future expansion
                struct {
                    ULONG32 Null;
                } EnrollmentRequirements;
            } FacialFeatures;

        [case(WINBIO_TYPE_FINGERPRINT)]
            struct {
                WINBIO_CAPABILITIES Capabilities;   // Reserved for future expansion
                struct {
                    ULONG GeneralSamples;
                    ULONG Center;
                    ULONG TopEdge;
                    ULONG BottomEdge;
                    ULONG LeftEdge;
                    ULONG RightEdge;
                } EnrollmentRequirements;
            } Fingerprint;

        [case(WINBIO_TYPE_IRIS)]
            struct {
                WINBIO_CAPABILITIES Capabilities;   // Reserved for future expansion
                struct {
                    ULONG32 Null;
                } EnrollmentRequirements;
            } Iris;

        [case(WINBIO_TYPE_VOICE)]
            struct {
                WINBIO_CAPABILITIES Capabilities;   // Reserved for future expansion
                struct {
                    ULONG32 Null;
                } EnrollmentRequirements;
            } Voice;
    } Specific;
} WINBIO_EXTENDED_ENGINE_INFO, *PWINBIO_EXTENDED_ENGINE_INFO;

#else   // MIDL_PASS

typedef struct _WINBIO_EXTENDED_ENGINE_INFO {
    WINBIO_CAPABILITIES GenericEngineCapabilities;

    WINBIO_BIOMETRIC_TYPE Factor;
    union
    {
        ULONG32 Null;

        struct {
            WINBIO_CAPABILITIES Capabilities;   // Reserved for future expansion
            struct {
                ULONG32 Null;
            } EnrollmentRequirements;
        } FacialFeatures;

        struct {
            WINBIO_CAPABILITIES Capabilities;   // Reserved for future expansion
            struct {
                ULONG GeneralSamples;
                ULONG Center;
                ULONG TopEdge;
                ULONG BottomEdge;
                ULONG LeftEdge;
                ULONG RightEdge;
            } EnrollmentRequirements;
        } Fingerprint;

        struct {
            WINBIO_CAPABILITIES Capabilities;   // Reserved for future expansion
            struct {
                ULONG32 Null;
            } EnrollmentRequirements;
        } Iris;

        struct {
            WINBIO_CAPABILITIES Capabilities;   // Reserved for future expansion
            struct {
                ULONG32 Null;
            } EnrollmentRequirements;
        } Voice;
    } Specific;
} WINBIO_EXTENDED_ENGINE_INFO, *PWINBIO_EXTENDED_ENGINE_INFO;

#endif  // MIDL_PASS

///////////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_EXTENDED_STORAGE_INFO
//
// Description:
//      Returns extended information about the capabilities
//      and attributes of the Storage component connected to
//      a specific biometric unit.
//
// Access:
//      Read-only
//
// Inputs:
//      SessionHandle - must be valid
//      UnitId - must be valid
//      Identity - must be NULL
//      SubFactor - must be WINBIO_SUBTYPE_NO_INFORMATION
//
// Outputs:
//      PropertyBuffer - points to a buffer containing a WINBIO_EXTENDED_STORAGE_INFO structure
//      PropertyBufferSize - points to a SIZE_T variable containing sizeof(WINBIO_EXTENDED_STORAGE_INFO)
//
#define WINBIO_PROPERTY_EXTENDED_STORAGE_INFO       ((WINBIO_PROPERTY_ID)4)

#ifdef MIDL_PASS

typedef struct _WINBIO_EXTENDED_STORAGE_INFO {
    WINBIO_CAPABILITIES GenericStorageCapabilities;

    WINBIO_BIOMETRIC_TYPE Factor;
    [switch_is(Factor)] union
    {
        [case(WINBIO_NO_TYPE_AVAILABLE)]
            ULONG32 Null;

        [case(WINBIO_TYPE_FACIAL_FEATURES)]
            struct {
                WINBIO_CAPABILITIES Capabilities;
                // Reserved for future expansion
            } FacialFeatures;

        [case(WINBIO_TYPE_FINGERPRINT)]
            struct {
                WINBIO_CAPABILITIES Capabilities;
                // Reserved for future expansion
            } Fingerprint;

        [case(WINBIO_TYPE_IRIS)]
            struct {
                WINBIO_CAPABILITIES Capabilities;
                // Reserved for future expansion
            } Iris;

        [case(WINBIO_TYPE_VOICE)]
            struct {
                WINBIO_CAPABILITIES Capabilities;
                // Reserved for future expansion
            } Voice;
    } Specific;
} WINBIO_EXTENDED_STORAGE_INFO, *PWINBIO_EXTENDED_STORAGE_INFO;

#else   // MIDL_PASS

typedef struct _WINBIO_EXTENDED_STORAGE_INFO {
    WINBIO_CAPABILITIES GenericStorageCapabilities;

    WINBIO_BIOMETRIC_TYPE Factor;
    union
    {
        ULONG32 Null;

        struct {
            WINBIO_CAPABILITIES Capabilities;
            // Reserved for future expansion
        } FacialFeatures;

        struct {
            WINBIO_CAPABILITIES Capabilities;
            // Reserved for future expansion
        } Fingerprint;

        struct {
            WINBIO_CAPABILITIES Capabilities;
            // Reserved for future expansion
        } Iris;

        struct {
            WINBIO_CAPABILITIES Capabilities;
            // Reserved for future expansion
        } Voice;
    } Specific;
} WINBIO_EXTENDED_STORAGE_INFO, *PWINBIO_EXTENDED_STORAGE_INFO;

#endif  // MIDL_PASS

///////////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_EXTENDED_ENROLLMENT_STATUS
//
// Description:
//      Returns extended information about the status of an
//      in-progress enrollment on a particular biometric unit.
//      If no enrollment is in progress on the BU, it returns
//      WINBIO_E_INVALID_OPERATION in the 'TemplateStatus' field.
//
// Access:
//      Read-only
//
// Inputs:
//      SessionHandle - must be valid
//      UnitId - must be valid
//      Identity - must be NULL
//      SubFactor - must be WINBIO_SUBTYPE_NO_INFORMATION
//
// Outputs:
//      PropertyBuffer - points to a buffer containing a WINBIO_EXTENDED_ENROLLMENT_STATUS structure
//      PropertyBufferSize - points to a SIZE_T variable containing sizeof(WINBIO_EXTENDED_ENROLLMENT_STATUS)
//
#define WINBIO_PROPERTY_EXTENDED_ENROLLMENT_STATUS  ((WINBIO_PROPERTY_ID)5)

#ifdef MIDL_PASS

typedef struct _WINBIO_EXTENDED_ENROLLMENT_STATUS {
    HRESULT TemplateStatus;
    WINBIO_REJECT_DETAIL RejectDetail;
    ULONG PercentComplete;

    WINBIO_BIOMETRIC_TYPE Factor;
    WINBIO_BIOMETRIC_SUBTYPE SubFactor;
    [switch_is(Factor)] union
    {
        [case(WINBIO_NO_TYPE_AVAILABLE)]
            ULONG32 Null;

        [case(WINBIO_TYPE_FACIAL_FEATURES)]
            struct {
                RECT BoundingBox;
                LONG Distance;
                struct {
                    WINBIO_UUID AdapterId;
                    ULONG Data[WINBIO_OPAQUE_ENGINE_DATA_ITEM_COUNT];
                } OpaqueEngineData;
            } FacialFeatures;

        [case(WINBIO_TYPE_FINGERPRINT)]
            struct {
                ULONG GeneralSamples;
                ULONG Center;
                ULONG TopEdge;
                ULONG BottomEdge;
                ULONG LeftEdge;
                ULONG RightEdge;
            } Fingerprint;

        [case(WINBIO_TYPE_IRIS)]
            struct {
                RECT EyeBoundingBox_1;
                RECT EyeBoundingBox_2;
                POINT PupilCenter_1;
                POINT PupilCenter_2;
                LONG Distance;
                ULONG GridPointCompletionPercent;
                UINT16 GridPointIndex;
                struct
                {
                    double X;
                    double Y;
                    double Z;
                } Point3D;
                BOOL StopCaptureAndShowCriticalFeedback;
            } Iris;

        [case(WINBIO_TYPE_VOICE)]
            struct {
                ULONG32 Reserved;           // Reserved for future expansion
            } Voice;
    } Specific;
} WINBIO_EXTENDED_ENROLLMENT_STATUS, *PWINBIO_EXTENDED_ENROLLMENT_STATUS;

#else   // MIDL_PASS

typedef struct _WINBIO_EXTENDED_ENROLLMENT_STATUS {
    HRESULT TemplateStatus;
    WINBIO_REJECT_DETAIL RejectDetail;
    ULONG PercentComplete;

    WINBIO_BIOMETRIC_TYPE Factor;
    WINBIO_BIOMETRIC_SUBTYPE SubFactor;
    union
    {
        ULONG32 Null;

        struct {
            RECT BoundingBox;
            LONG Distance;
            struct {
                WINBIO_UUID AdapterId;
                ULONG Data[WINBIO_OPAQUE_ENGINE_DATA_ITEM_COUNT];
            } OpaqueEngineData;
        } FacialFeatures;

        struct {
            ULONG GeneralSamples;
            ULONG Center;
            ULONG TopEdge;
            ULONG BottomEdge;
            ULONG LeftEdge;
            ULONG RightEdge;
        } Fingerprint;

        struct {
            RECT EyeBoundingBox_1;
            RECT EyeBoundingBox_2;
            POINT PupilCenter_1;
            POINT PupilCenter_2;
            LONG Distance;
            ULONG GridPointCompletionPercent;
            UINT16 GridPointIndex;
            struct
            {
                double X;
                double Y;
                double Z;
            } Point3D;
            BOOL StopCaptureAndShowCriticalFeedback;
        } Iris;

        struct {
            ULONG32 Reserved;               // Reserved for future expansion
        } Voice;
    } Specific;
} WINBIO_EXTENDED_ENROLLMENT_STATUS, *PWINBIO_EXTENDED_ENROLLMENT_STATUS;

#endif  // MIDL_PASS

///////////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_EXTENDED_UNIT_STATUS
//
// Description:
//      Returns extended information about the current status
//      of a specific biometric unit.
//
// Access:
//      Read-only
//
// Inputs:
//      SessionHandle - must be valid
//      UnitId - must be valid
//      Identity - must be NULL
//      SubFactor - must be WINBIO_SUBTYPE_NO_INFORMATION
//
// Outputs:
//      PropertyBuffer - points to a buffer containing a WINBIO_EXTENDED_UNIT_STATUS structure
//      PropertyBufferSize - points to a SIZE_T variable containing sizeof(WINBIO_EXTENDED_UNIT_STATUS)
//
#define WINBIO_PROPERTY_EXTENDED_UNIT_STATUS        ((WINBIO_PROPERTY_ID)6)

typedef struct _WINBIO_EXTENDED_UNIT_STATUS {
    WINBIO_SENSOR_STATUS Availability;
    ULONG ReasonCode;
} WINBIO_EXTENDED_UNIT_STATUS, *PWINBIO_EXTENDED_UNIT_STATUS;

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

///////////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_UNIT_SECURITY_LEVEL
//
// Description:
//      Returns the security level a specific biometric unit
//      is running as.
//
// Access:
//      Read-only
//
// Inputs:
//      SessionHandle - must be valid
//      UnitId - must be valid
//      Identity - must be NULL
//      SubFactor - must be WINBIO_SUBTYPE_NO_INFORMATION
//
// Outputs:
//      PropertyBuffer - points to a WINBIO_UNIT_SECURITY_LEVEL buffer containing the level
//      PropertyBufferSize - points to a SIZE_T variable containing sizeof(WINBIO_UNIT_SECURITY_LEVEL)
//
#define WINBIO_PROPERTY_UNIT_SECURITY_LEVEL         ((WINBIO_PROPERTY_ID)7)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)

///////////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_FP_BU_STATE
//
// Description:
//      Returns information on whether a FP sensor is currently
//      attached to the machine, in addition to any relevant FP
//      biometric unit (BU) creation error codes.
//
// Access:
//      Read-only
//
// Inputs:
//      SessionHandle - must be valid
//      UnitId - must be zero
//      Identity - must be NULL
//      SubFactor - must be WINBIO_SUBTYPE_NO_INFORMATION
//
// Outputs:
//      PropertyBuffer - points to buffer containing a WINBIO_FP_BU_STATE structure
//      PropertyBufferSize - points to a SIZE_T variable containing sizeof(WINBIO_FP_BU_STATE)
//
#define WINBIO_PROPERTY_FP_BU_STATE        ((WINBIO_PROPERTY_ID)8)

typedef struct _WINBIO_FP_BU_STATE {
    BOOL SensorAttached;
    HRESULT CreationResult;
} WINBIO_FP_BU_STATE, *PWINBIO_FP_BU_STATE;

#endif // (NTDDI_VERSION >= NTDDI_WIN10_19H1)

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)

///////////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_FP_IS_IMPROVING
//
// Description:
//      Returns information on whether a FP sensor is currently
//      set to improve recognition through extra enrollment templates.
//      Duplicate enrollment records are permitted while this is set.
//
// Access:
//      Read-only
//
// Inputs:
//      SessionHandle - must be valid
//      UnitId - must be valid
//      Identity - must be NULL
//      SubFactor - must be WINBIO_SUBTYPE_NO_INFORMATION
//
// Outputs:
//      PropertyBuffer - points to BOOL buffer containing the improvement status
//      PropertyBufferSize - points to a SIZE_T variable containing sizeof(BOOL)
//
#define WINBIO_PROPERTY_FP_IS_IMPROVING             ((WINBIO_PROPERTY_ID)9)

#define WINBIO_OPERATION_IMPROVE_BEGIN              ((WINBIO_OPERATION_TYPE)35)
#define WINBIO_OPERATION_IMPROVE_END                ((WINBIO_OPERATION_TYPE)36)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_VB)

///////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_TYPE_TEMPLATE properties...
//
///////////////////////////////////////////////////////////////////////////
//
// (None)
//


///////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_TYPE_ACCOUNT properties...
//
///////////////////////////////////////////////////////////////////////////
//
// WINBIO_PROPERTY_ANTI_SPOOF_POLICY
//
// Description:
//      Gets or sets the values of the anti-spoofing policy for a
//      specific user account.
//
//      Performing a 'Get' operation with the wildcard identity returns
//      the system default value of this policy.
//
// Access:
//      Read/Write
//
//      Non-privileged users can only modify their own policy setting. If
//      the 'Identity' argument refers to another account -- or contains a
//      wildcard identifier value -- WinBioSetProperty will fail.
//
// Inputs:
//      SessionHandle - must be valid
//      UnitId - must be zero
//      Identity - account SID to be queried or changed. If NULL, the
//                 SID associated with SessionHandle will be used.
//      SubFactor - must be WINBIO_SUBTYPE_NO_INFORMATION
//
// Outputs:
//      PropertyBuffer - points to a buffer containing a WINBIO_ANTI_SPOOF_POLICY structure
//      PropertyBufferSize - points to a SIZE_T variable containing sizeof(WINBIO_ANTI_SPOOF_POLICY)
//
#define WINBIO_PROPERTY_ANTI_SPOOF_POLICY           ((WINBIO_PROPERTY_ID)1)

typedef enum _WINBIO_ANTI_SPOOF_POLICY_ACTION
{
    WINBIO_ANTI_SPOOF_DISABLE   = 0x00000000,
    WINBIO_ANTI_SPOOF_ENABLE    = 0x00000001,
    WINBIO_ANTI_SPOOF_REMOVE    = 0x00000002,
} WINBIO_ANTI_SPOOF_POLICY_ACTION, *PWINBIO_ANTI_SPOOF_POLICY_ACTION;

typedef enum _WINBIO_POLICY_SOURCE
{
    WINBIO_POLICY_UNKNOWN   = 0x00000000,
    WINBIO_POLICY_DEFAULT   = 0x00000001,
    WINBIO_POLICY_LOCAL     = 0x00000002,
    WINBIO_POLICY_ADMIN     = 0x00000003,
} WINBIO_POLICY_SOURCE, *PWINBIO_POLICY_SOURCE;


typedef struct _WINBIO_ANTI_SPOOF_POLICY {
    WINBIO_ANTI_SPOOF_POLICY_ACTION Action;
    WINBIO_POLICY_SOURCE Source;
} WINBIO_ANTI_SPOOF_POLICY, *PWINBIO_ANTI_SPOOF_POLICY;


#if (NTDDI_VERSION >= NTDDI_WIN9)

///////////////////////////////////////////////////////////////////////////////
//
// Async ticket operations...
//
///////////////////////////////////////////////////////////////////////////////
#define WINBIO_OPERATION_IDENTIFY_AND_RELEASE_TICKET    ((WINBIO_OPERATION_TYPE)27)
#define WINBIO_OPERATION_VERIFY_AND_RELEASE_TICKET      ((WINBIO_OPERATION_TYPE)28)

#endif // (NTDDI_VERSION >= NTDDI_WIN9)


#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

///////////////////////////////////////////////////////////////////////////////
//
// Facial features operations...
//
///////////////////////////////////////////////////////////////////////////////
#define WINBIO_OPERATION_MONITOR_PRESENCE           ((WINBIO_OPERATION_TYPE)29)
#define WINBIO_OPERATION_ENROLL_SELECT              ((WINBIO_OPERATION_TYPE)30)

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

///////////////////////////////////////////////////////////////////////////////
//
// Credential management
//
///////////////////////////////////////////////////////////////////////////////

typedef enum _WINBIO_CREDENTIAL_TYPE
{
    WINBIO_CREDENTIAL_PASSWORD  = 0x00000001,
    WINBIO_CREDENTIAL_ALL = 0xffffffff,
} WINBIO_CREDENTIAL_TYPE;

typedef enum _WINBIO_CREDENTIAL_FORMAT
{
    WINBIO_PASSWORD_GENERIC     = 0x00000001,
    WINBIO_PASSWORD_PACKED      = 0x00000002,
    WINBIO_PASSWORD_PROTECTED   = 0x00000003,
} WINBIO_CREDENTIAL_FORMAT;

typedef enum _WINBIO_CREDENTIAL_STATE {
    WINBIO_CREDENTIAL_NOT_SET   = 0x00000001,
    WINBIO_CREDENTIAL_SET       = 0x00000002,
} WINBIO_CREDENTIAL_STATE, *PWINBIO_CREDENTIAL_STATE;

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////


///////////////////////////////////////////////////////////////////////////////
//
// Settings
//
///////////////////////////////////////////////////////////////////////////////

typedef ULONG32 WINBIO_SETTING_SOURCE_TYPE, *PWINBIO_SETTING_SOURCE_TYPE;

#define WINBIO_SETTING_SOURCE_INVALID    ((WINBIO_SETTING_SOURCE_TYPE)0)
#define WINBIO_SETTING_SOURCE_DEFAULT    ((WINBIO_SETTING_SOURCE_TYPE)1)
#define WINBIO_SETTING_SOURCE_POLICY     ((WINBIO_SETTING_SOURCE_TYPE)2)
#define WINBIO_SETTING_SOURCE_LOCAL      ((WINBIO_SETTING_SOURCE_TYPE)3)

typedef ULONG32 WINBIO_SETTING_TYPE, *PWINBIO_SETTING_TYPE;

#define WINBIO_SETTING_TYPE_PERIPHERALS_WITH_ESS ((WINBIO_SETTING_TYPE)0)
#define WINBIO_SETTING_TYPE_ESS_ENABLED ((WINBIO_SETTING_TYPE)1)

typedef struct _WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    SIZE_T Size;
    WINBIO_BIOMETRIC_SUBTYPE SubFactor;
} WINBIO_EXTENDED_ENROLLMENT_PARAMETERS, *PWINBIO_EXTENDED_ENROLLMENT_PARAMETERS;

typedef struct _WINBIO_ACCOUNT_POLICY {
    WINBIO_IDENTITY Identity;
    WINBIO_ANTI_SPOOF_POLICY_ACTION AntiSpoofBehavior;
} WINBIO_ACCOUNT_POLICY, *PWINBIO_ACCOUNT_POLICY;


#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#define WINBIO_OPERATION_ENROLL_AUTHORIZE                   ((WINBIO_OPERATION_TYPE)31)
#define WINBIO_OPERATION_ENROLL_REVOKE                      ((WINBIO_OPERATION_TYPE)32)
#define WINBIO_OPERATION_GET_PROTECTION_POLICY              ((WINBIO_OPERATION_TYPE)33)

typedef struct _WINBIO_PROTECTION_POLICY
{
    ULONG Version;
    WINBIO_IDENTITY Identity;
    WINBIO_UUID DatabaseId;
    ULONGLONG UserState;
    SIZE_T PolicySize;
    UCHAR Policy[128];
} WINBIO_PROTECTION_POLICY, *PWINBIO_PROTECTION_POLICY;

typedef ULONG32 WINBIO_MATCH_TYPE, *PWINBIO_MATCH_TYPE;

#define WINBIO_MATCH_SOFTWARE                      ((WINBIO_MATCH_TYPE)1)
#define WINBIO_MATCH_TRUSTED_EXECUTION_ENVIRONMENT ((WINBIO_MATCH_TYPE)2)
#define WINBIO_MATCH_ON_CHIP                       ((WINBIO_MATCH_TYPE)3)

typedef ULONG32 WINBIO_PROTECTION_TYPE, *PWINBIO_PROTECTION_TYPE;

#define WINBIO_PROTECTION_SOFTWARE                      ((WINBIO_PROTECTION_TYPE)1)
#define WINBIO_PROTECTION_TRUSTED_EXECUTION_ENVIRONMENT ((WINBIO_PROTECTION_TYPE)2)

typedef struct _WINBIO_GESTURE_METADATA
{
    SIZE_T Size;
    WINBIO_BIOMETRIC_TYPE BiometricType;
    WINBIO_MATCH_TYPE MatchType;
    WINBIO_PROTECTION_TYPE ProtectionType;
} WINBIO_GESTURE_METADATA, *PWINBIO_GESTURE_METADATA;

//
// Framework operation - async unit status update notification...
//
#define WINBIO_OPERATION_NOTIFY_UNIT_STATUS_CHANGE      ((WINBIO_OPERATION_TYPE)34)

typedef ULONG32 WINBIO_TELEMETRY_TYPE, *PWINBIO_TELEMETRY_TYPE;

#define WINBIO_TELEMETRY_AUTH                         ((WINBIO_TELEMETRY_TYPE)1)
#define WINBIO_TELEMETRY_ENROLLMENT                   ((WINBIO_TELEMETRY_TYPE)2)


#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

#ifdef MIDL_PASS

//
// Limit the size of vendor-specific type info about a sensor to 4K
//
const ULONG WINBIO_MAX_PRIVATE_SENSOR_TYPE_INFO_BUFFER_SIZE = 0x1000;

#else // MIDL_PASS

#define WINBIO_MAX_PRIVATE_SENSOR_TYPE_INFO_BUFFER_SIZE ((ULONG)0x1000)

#endif // MIDL_PASS

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)

# if (NTDDI_VERSION >= NTDDI_WIN11_GE)

///////////////////////////////////////////////////////////////////////////////
//
// Ess state query
//
///////////////////////////////////////////////////////////////////////////////

typedef ULONG64 WINBIO_ESS_STATE, *PWINBIO_ESS_STATE;

enum WINBIO_ESS_STATE_FLAGS
{
    // This PC does not have TPM 2.0
    WINBIO_ESS_REQUIRES_TPM2                            = 0x00000001,

    // This PC is not VBS capable.
    WINBIO_ESS_REQUIRES_VBS_CAPABLE                     = 0x00000002,

    // This PC has a Windows Hello container not backed by VBS.
    WINBIO_ESS_REQUIRES_NON_VBS_WINDOWS_HELLO_ABSENCE   = 0x00000004,

    // This PC does not have a Windows Hello container backed by VBS.
    WINBIO_ESS_REQUIRES_VBS_WINDOWS_HELLO               = 0x00000008,

    // This PC does not have VBS running.
    WINBIO_ESS_REQUIRES_VBS_RUNNING                     = 0x00000010,

    // This PC is missing VBS encryption keys.
    WINBIO_ESS_REQUIRES_VBS_ENCRYPTION_KEY              = 0x00000020,

    // On this PC, ESS is disabled either using a "Enhanced Sign-in Security" toggle in Accounts -> Sign-in options under "Additional settings" or
    // using a policy.
    // By default following flag will not be set.
    WINBIO_ESS_REQUIRES_ENABLEMENT                      = 0x00000040,

    // On this PC, ESS is managed using a policy available through group policy or MDM policy.
    // MDM policy: ./Device/Vendor/MSFT/PassportForWork/Biometrics/EnableESSwithSupportedPeripherals
    // Group policy: Computer Configuration\Administrative Templates\Windows Components\Windows Hello for Business\Enable ESS with Supported Peripherals
    WINBIO_ESS_MANAGED_BY_POLICY                        = 0x00000080,

    // This PC contains enrollments performed using biometric sensors that are not ESS-capable.
    WINBIO_ESS_REQUIRES_NON_VBS_BIOMETRIC_ENROLLMENT_ABSENCE = 0x00000100,

    // This PC does not have enrollments performed using ESS capable biometric sensors.
    WINBIO_ESS_REQUIRES_VBS_BIOMETRIC_ENROLLMENT        = 0x00000200,

    // This PC does not have a ESS capable face camera sensor. 
    WINBIO_ESS_REQUIRES_FACE_SENSOR                     = 0x00000400,

    // This PC does not have a ESS FPR sensor.
    WINBIO_ESS_REQUIRES_FPR_SENSOR                      = 0x00000800,

    // This PC could not start biometrics trustlet needed for ESS.
    WINBIO_ESS_REQUIRES_ISOLATED_PROCESS                = 0x00001000,

    // This PC blocked non-ESS capable fingerprint sensor.
    WINBIO_ESS_BLOCKED_NON_ESS_FPR                      = 0x00002000,

    // This PC blocked non-ESS capable face camera sensor.
    WINBIO_ESS_BLOCKED_NON_ESS_CAMERA                   = 0x00004000,

    // This PC's ESS state is determined based on if they have non ESS enrollments
    WINBIO_ESS_SOURCE_DEFAULT                           = 0x00008000
};

// Returns biometric capable sensor type connected to the PC.
typedef struct _WINBIO_CONNECTED_SENSOR
{
    WINBIO_BIOMETRIC_TYPE biometricType;
    BOOL isEnhancedSignInSecurityCapable;
} WINBIO_CONNECTED_SENSOR, *PWINBIO_CONNECTED_SENSOR;

#endif // (NTDDI_VERSION >= NTDDI_WIN11_GE)

#ifdef __cplusplus
} // extern "C"
#endif

#pragma warning( pop )

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#endif // _WINBIO_TYPES_H_712486DB_3EF5_41da_937A_55DCB7B66A53_

