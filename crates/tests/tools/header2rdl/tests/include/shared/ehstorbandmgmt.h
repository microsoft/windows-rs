/*++
    Copyright (c) Microsoft Corporation. All rights reserved.

    Module Name:
        EhStorBandMgmt.h

    Abstract:
        Header file for Enhanced Storage Band Management IOCTLs.

    Environment:
        Kernel mode only

    Revision History:
        08-18-09 : Created
--*/

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#define _EHSTOR_INTERFACE_

#pragma warning(push)
// nonstandard extension used : nameless struct/union
#pragma warning(disable:4201)

//
// IOCTL_EHSTOR_BANDMGMT_QUERY_CAPABILITIES parameters.
//
typedef struct _BAND_MANAGEMENT_CAPABILITIES
{
    ULONG       StructSize;
    ULONG       Capabilities;
    ULONGLONG   KeyProtectionMechanism;
    ULONG       MinAuthKeyLength;
    ULONG       MaxAuthKeyLength;
    ULONG       MaxBandCount;
    ULONG       MaxSimultaneousReencryptionCount;
    ULONG       BandMetadataSize;
} BAND_MANAGEMENT_CAPABILITIES, *PBAND_MANAGEMENT_CAPABILITIES;

#define CAPS_ACTIVATED                  0x00000001
#define CAPS_BANDCROSSING_SUPPORTED     0x00000002
#define CAPS_SID_SECURED                0x00000004

#define MEDIAKEY_PROTECTEDBY_VENDORSCHEME 0x1
#define MEDIAKEY_PROTECTEDBY_AUTHKEY      0x2

//
// IOCTL_EHSTOR_BANDMGMT_ACTIVATE or IOCTL_EHSTOR_BANDMGMT_REVERT parameters.
//
typedef struct _ACTIVATE_REVERT_PARAMETERS
{
    ULONG           StructSize;
    ULONG           Flags;
    ULONG           AuthKeyOffset;
    //
    // This structure is followed by a variable-length AUTH_KEY structure
    // referenced by the AuthKeyOffset field.
    //
} ACTIVATE_REVERT_PARAMETERS, *PACTIVATE_REVERT_PARAMETERS;

#define ACTIVATE_DISABLE_SID            0x00000001
#define ACTIVATE_IGNORE_POLICY          0x00000002
#define REVERT_PSID_AUTHKEY             0x00000001

typedef struct _AUTH_KEY
{
    ULONG   KeySize;
    UCHAR   Key[ANYSIZE_ARRAY];
} AUTH_KEY, *PAUTH_KEY;

#define SIZE_AUTH_KEY_HEADER    FIELD_OFFSET(AUTH_KEY, Key)

// Special key offset value for "no key" or "default key"
#define EHSTOR_BANDMGR_NO_KEY           0

//
// IOCTL_EHSTOR_BANDMGMT_ENUMERATE_BANDS parameters.
//
typedef struct _ENUMERATE_BANDS_PARAMETERS
{
    ULONG           StructSize;
    ULONG           Flags;
    ULONG           Reserved;
    ULONG           BandId;
    LARGE_INTEGER   BandStart;
    LARGE_INTEGER   BandSize;
} ENUMERATE_BANDS_PARAMETERS, *PENUMERATE_BANDS_PARAMETERS;

#define ENUMBANDS_ENUM_ALL_BANDS        0x00000001
#define ENUMBANDS_REPORT_CRYPTO_ALGO    0x00000002

typedef struct _BAND_LOCATION_INFO
{
    ULONG           StructSize;
    ULONG           Reserved;
    LARGE_INTEGER   BandStart;
    LARGE_INTEGER   BandSize;
    BYTE            Metadata[32];
} BAND_LOCATION_INFO, *PBAND_LOCATION_INFO;

typedef enum
{
    INVALID_LOCK_STATE = 0,
    PERSISTENT_UNLOCK,      //dev is unlocked and remains unlocked on pwr reset.
    NONPERSISTENT_UNLOCK,   //dev is unlocked but gets locked on pwr reset.
    PERSISTENT_LOCK         //dev is locked and remains locked on pwr reset.
} LOCKSTATE;

typedef enum
{
    AlgoIdTypeInvalid = 0,
    AlgoIdTypeOidString,
    AlgoIdTypeNumeric
} ALGOIDTYPE;

typedef struct _BAND_SECURITY_INFO
{
    ULONG       StructSize;
    LOCKSTATE   ReadLock;
    LOCKSTATE   WriteLock;
    ALGOIDTYPE  CryptoAlgoIdType;
    union
    {
        struct
        {
            ULONG   Offset;
            ULONG   Length;
        } CryptoAlgoOidString;
        ULONG   CryptoAlgoNumericId;
    };
    BYTE        Metadata[32];
} BAND_SECURITY_INFO, *PBAND_SECURITY_INFO;

typedef struct _BAND_TABLE_ENTRY
{
    ULONG               BandId;
    BAND_LOCATION_INFO  LocationInfo;
    BAND_SECURITY_INFO  SecurityInfo;
} BAND_TABLE_ENTRY, *PBAND_TABLE_ENTRY;

typedef struct _BAND_TABLE
{
    ULONG   StructSize;
    ULONG   BandTableOffset;
    ULONG   BandTableEntryCount;
    ULONG   BandTableEntrySize;
    //
    // This structure is followed by a variable-length array
    // of BAND_TABLE_ENTRY structures referenced by BandTableOffset
    //
} BAND_TABLE, *PBAND_TABLE;

//
// IOCTL_EHSTOR_BANDMGMT_CREATE_BAND parameters.
//
typedef struct _CREATE_BAND_PARAMETERS
{
    ULONG   StructSize;
    ULONG   Flags;
    ULONG   BandLocationInfoOffset;
    ULONG   BandSecurityInfoOffset;
    ULONG   AuthKeyOffset;
    //
    // This structure is followed by BAND_LOCATION_INFO, BAND_SECURITY_INFO
    // and AUTH_KEY structures referenced by the BandLocationInfoOffset,
    // BandSecurityInfoOffset and AuthKeyOffset fields.
    //
} CREATE_BAND_PARAMETERS, *PCREATE_BAND_PARAMETERS;

#define CREATEBAND_AUTHKEY_CACHING_ENABLED  0x00000001

//
// IOCTL_EHSTOR_BANDMGMT_SET_BAND_LOCATION parameters.
//
typedef struct _SET_BAND_LOCATION_PARAMETERS
{
    ULONG           StructSize;
    ULONG           BandId;
    LARGE_INTEGER   BandStart;
    ULONG           AuthKeyOffset;
    ULONG           BandLocationInfoOffset;
    //
    // This structure is followed by a BAND_LOCATION_INFO structure
    // and a variable-length AUTH_KEY structure referenced by the
    // BandLocationInfoOffset and AuthKeyOffset fields, respectively.
    //
} SET_BAND_LOCATION_PARAMETERS, *PSET_BAND_LOCATION_PARAMETERS;

//
// IOCTL_EHSTOR_BANDMGMT_SET_BAND_SECURITY parameters.
//
typedef struct _SET_BAND_SECURITY_PARAMETERS
{
    ULONG           StructSize;
    ULONG           Flags;
    ULONG           Reserved;
    ULONG           BandId;
    LARGE_INTEGER   BandStart;
    ULONG           CurrentAuthKeyOffset;
    ULONG           NewAuthKeyOffset;
    ULONG           BandSecurityInfoOffset;
    //
    // This structure is followed by variable-length AUTH_KEY and
    // BAND_SECURITY_INFO structures referenced by the CurrentAuthKeyOffset,
    // NewAuthKeyOffset and BandSecurityInfoOffset fields.
    //
} SET_BAND_SECURITY_PARAMETERS, *PSET_BAND_SECURITY_PARAMETERS;

#define SETBANDSEC_AUTHKEY_CACHING_ENABLED  0x00000001

//
// IOCTL_EHSTOR_BANDMGMT_GET_BAND_METADATA parameters.
//
typedef struct _GET_BAND_METADATA_PARAMETERS
{
    ULONG           StructSize;
    ULONG           BandId;
    LARGE_INTEGER   BandStart;
    ULONG           MetadataOffset;
    ULONG           MetadataSize;
} GET_BAND_METADATA_PARAMETERS, *PGET_BAND_METADATA_PARAMETERS;

//
// IOCTL_EHSTOR_BANDMGMT_SET_BAND_METADATA parameters.
//
typedef struct _SET_BAND_METADATA_PARAMETERS
{
    ULONG           StructSize;
    ULONG           BandId;
    LARGE_INTEGER   BandStart;
    ULONG           MetadataOffset;
    ULONG           MetadataSize;
    ULONG           BufferOffset;
    ULONG           AuthKeyOffset;
    //
    // This structure is followed by a buffer referenced by the BufferOffset
    // and MetadataSize fields, and optionally a variable-length AUTH_KEY
    // structure referenced by the AuthKeyOffset field.
    //
} SET_BAND_METADATA_PARAMETERS, *PSET_BAND_METADATA_PARAMETERS;

//
// IOCTL_EHSTOR_BANDMGMT_DELETE_BAND parameters.
//
typedef struct _DELETE_BAND_PARAMETERS
{
    ULONG           StructSize;
    ULONG           Flags;
    ULONG           Reserved;
    ULONG           BandId;
    LARGE_INTEGER   BandStart;
    ULONG           AuthKeyOffset;
    //
    // This structure is followed by a variable-length AUTH_KEY
    // structure referenced by AuthKeyOffset.
    //
} DELETE_BAND_PARAMETERS, *PDELETE_BAND_PARAMETERS;

#define DELBAND_ERASE_BEFORE_DELETE     0x00000001

//
// IOCTL_EHSTOR_BANDMGMT_ERASE_BAND parameters.
//
typedef struct _ERASE_BAND_PARAMETERS
{
    ULONG           StructSize;
    ULONG           Flags;
    ULONG           Reserved;
    ULONG           BandId;
    LARGE_INTEGER   BandStart;
    ULONG           NewAuthKeyOffset;
    //
    // This structure is followed by a variable-length AUTH_KEY
    // structure referenced by NewAuthKeyOffset.
    //
} ERASE_BAND_PARAMETERS, *PERASE_BAND_PARAMETERS;

#define ERASEBAND_AUTHKEY_CACHING_ENABLED   0x00000001

//
// IOCTL_EHSTOR_BANDMGMT_SET_SID parameters.
//
typedef struct _SET_SID_PARAMETERS
{
    ULONG   StructSize;
    ULONG   CurrentSidPinOffset;
    ULONG   NewSidPinOffset;
    //
    // This structure is followed by two variable-length AUTH_KEY structures
    // referenced by CurrentSidPinOffset and NewSidPinOffset fields.
    //
} SET_SID_PARAMETERS, *PSET_SID_PARAMETERS;

//
// Band Control IOCTLs
//
#define IOCTL_EHSTOR_BANDMGMT_QUERY_CAPABILITIES \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x520, METHOD_BUFFERED, \
                     FILE_ANY_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_ACTIVATE          \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x521, METHOD_BUFFERED, \
                     FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_REVERT            \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x522, METHOD_BUFFERED, \
                     FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_ENUMERATE_BANDS   \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x523, METHOD_BUFFERED, \
                     FILE_ANY_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_CREATE_BAND       \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x524, METHOD_BUFFERED, \
                     FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_SET_BAND_LOCATION \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x525, METHOD_BUFFERED, \
                     FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_SET_BAND_SECURITY \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x526, METHOD_BUFFERED, \
                     FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_GET_BAND_METADATA \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x527, METHOD_BUFFERED, \
                     FILE_ANY_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_SET_BAND_METADATA \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x528, METHOD_BUFFERED, \
                     FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_DELETE_BAND       \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x529, METHOD_BUFFERED, \
                     FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_ERASE_BAND        \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x52a, METHOD_BUFFERED, \
                     FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_SET_SID           \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x52b, METHOD_BUFFERED, \
                     FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_BANDMGMT_ERASE_ALL_BANDS   \
            CTL_CODE(IOCTL_STORAGE_BASE, 0x52c, METHOD_BUFFERED, \
                     FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// TCG Storage Silo Driver specific IOCTLs
//
#define IOCTL_EHSTOR_TCGDRV_RELINQUISH_SILO \
            CTL_CODE(FILE_DEVICE_EHSTOR, 0x200, METHOD_BUFFERED, \
                     FILE_READ_ACCESS | FILE_WRITE_ACCESS)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

