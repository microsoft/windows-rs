//
// copyright (c) Microsoft Corp.
//

#ifndef ENCDEC_ENUMS_H
#define ENCDEC_ENUMS_H

// !!!! do not #pragma once, we use this file twice(once for native and once for mgd) in managed interop
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include "exposeenums2managed.h"

ENUMG(25AEE876-3D61-4486-917E-7C0CB3D9983C) ProtType
{
    PROT_COPY_FREE = 1,
    PROT_COPY_ONCE = 2,
    PROT_COPY_NEVER = 3,
    PROT_COPY_NEVER_REALLY = 4,
    PROT_COPY_NO_MORE = 5,
    PROT_COPY_FREE_CIT = 6,
    PROT_COPY_BF = 7,
    PROT_COPY_CN_RECORDING_STOP = 8,
    PROT_COPY_FREE_SECURE = 9,
    PROT_COPY_INVALID = 50
} ProtType;

// Types of EncDec Events
ENUM EncDecEvents
{
    ENCDEC_CPEVENT = 0,
    ENCDEC_RECORDING_STATUS
} EncDecEvents;

ENUM CPRecordingStatus
{
    RECORDING_STOPPED = 0,
    RECORDING_STARTED = 1
} CPRecordingStatus;

ENUM CPEventBitShift
{
    CPEVENT_BITSHIFT_RATINGS = 0,
    CPEVENT_BITSHIFT_COPP,
    CPEVENT_BITSHIFT_LICENSE,
    CPEVENT_BITSHIFT_ROLLBACK,
    CPEVENT_BITSHIFT_SAC,
    CPEVENT_BITSHIFT_DOWNRES,
    CPEVENT_BITSHIFT_STUBLIB,
    CPEVENT_BITSHIFT_UNTRUSTEDGRAPH,
    CPEVENT_BITSHIFT_PENDING_CERTIFICATE,
    CPEVENT_BITSHIFT_NO_PLAYREADY
} CPEventBitShift;

ENUM CPEvents
{
    CPEVENT_NONE = 0,
    CPEVENT_RATINGS,
    CPEVENT_COPP,
    CPEVENT_LICENSE,
    CPEVENT_ROLLBACK,
    CPEVENT_SAC,
    CPEVENT_DOWNRES,
    CPEVENT_STUBLIB,
    CPEVENT_UNTRUSTEDGRAPH,
    CPEVENT_PROTECTWINDOWED,
} CPEvents;

ENUM RevokedComponent
{
    REVOKED_COPP = 0,
    REVOKED_SAC,
    REVOKED_APP_STUB,
    REVOKED_SECURE_PIPELINE,
    REVOKED_MAX_TYPES
} RevokedComponent;

ENUM EnTag_Mode
{
        EnTag_Remove = 0x0,
        EnTag_Once   = 0x1,
        EnTag_Repeat = 0x2,
} EnTag_Mode;

ENUMG(6F8C2442-2BFB-4180-9EE5-EA1FB47AE35C)  COPPEventBlockReason
{
    COPP_Unknown                = -1,
    COPP_BadDriver              = 0,
    COPP_NoCardHDCPSupport      = 1,
    COPP_NoMonitorHDCPSupport   = 2,
    COPP_BadCertificate         = 3,
    COPP_InvalidBusProtection   = 4,
    COPP_AeroGlassOff           = 5,
    COPP_RogueApp               = 6,
    COPP_ForbiddenVideo         = 7,
    COPP_Activate               = 8,
    COPP_DigitalAudioUnprotected= 9
} COPPEventBlockReason;

ENUMG(57BCA1BE-DF7A-434e-8B89-26D6A0541FDA) LicenseEventBlockReason
{
    LIC_BadLicense      = 0,
    LIC_NeedIndiv       = 1,
    LIC_Expired         = 2,
    LIC_NeedActivation  = 3,
    LIC_ExtenderBlocked = 4
} LicenseEventBlockReason;

ENUMG(D5CC1CDC-EF31-48dc-95B8-AFD34C08036B) DownResEventParam
{
    DOWNRES_Always          = 0,
    DOWNRES_InWindowOnly    = 1,
    DOWNRES_Undefined       = 2    
} DownResEventParam;

#include "unexposeenums2managed.h"


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // ENCDEC_ENUMS_H

// end of file


