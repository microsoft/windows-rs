////
// copyright (c) Microsoft Corp.
////

#ifndef BDAIFACE_ENUMS_H
#define BDAIFACE_ENUMS_H

// !!!! do not #pragma once, we use this file twice(once for native and once for mgd) in managed interop
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include "exposeenums2managed.h"

ENUM SmartCardStatusType
{
    CardInserted = 0,
    CardRemoved,
    CardError,
    CardDataChanged,
    CardFirmwareUpgrade
}SmartCardStatusType;

ENUM SmartCardAssociationType
{
    NotAssociated = 0,
    Associated,
    AssociationUnknown
}SmartCardAssociationType;

ENUM LocationCodeSchemeType
{
    SCTE_18 = 0
}LocationCodeSchemeType;

typedef struct EALocationCodeType {
    LocationCodeSchemeType LocationCodeScheme;
    BYTE state_code;
    BYTE county_subdivision;
    WORD county_code;
} EALocationCodeType ;

ENUM EntitlementType
{
    Entitled = 0,
    NotEntitled,
    TechnicalFailure
}EntitlementType;

ENUM UICloseReasonType
{
    NotReady = 0,
    UserClosed,
    SystemClosed,
    DeviceClosed,
    ErrorClosed
}UICloseReasonType;

typedef struct SmartCardApplication {
    ApplicationTypeType ApplicationType;
    USHORT ApplicationVersion;
    BSTR pbstrApplicationName;
    BSTR pbstrApplicationURL;
} SmartCardApplication ;

/*

// this enum is maintained in ehrecvr_enums.h, they need to be kept in sync
// they should also be merged at some point if possible.

ENUM DrmPairingStatus
{
    DrmPairing_Succeeded = 0,
    DrmPairing_HardwareFailure,
    DrmPairing_NeedRevocationData,
    DrmPairing_NeedIndiv,
    DrmPairing_Other,
    DrmPairing_DrmInitFailed,
    DrmPairing_DrmNotPaired,
    DrmPairing_DrmRePairSoon,
    DrmPairing_Aborted,
    DrmPairing_NeedSDKUpdate
}DrmPairingStatus;
*/

ENUM BDA_DrmPairingError
{
    BDA_DrmPairing_Succeeded = 0,
    BDA_DrmPairing_HardwareFailure,
    BDA_DrmPairing_NeedRevocationData,
    BDA_DrmPairing_NeedIndiv,
    BDA_DrmPairing_Other,
    BDA_DrmPairing_DrmInitFailed,
    BDA_DrmPairing_DrmNotPaired,
    BDA_DrmPairing_DrmRePairSoon,
    BDA_DrmPairing_Aborted,
    BDA_DrmPairing_NeedSDKUpdate
}BDA_DrmPairingError;

#include "unexposeenums2managed.h"


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
// end of file
