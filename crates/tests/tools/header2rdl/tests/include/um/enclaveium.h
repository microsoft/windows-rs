/*++ BUILD Version: 0001     Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    enclaveium.h

Abstract:

    Contains type definitions used by VBS enclaves during communication with trustlets.

Revision History:

--*/

#ifndef _ENCLAVEIUM_
#define _ENCLAVEIUM_

#ifdef __cplusplus
extern "C" {
#endif

#ifndef _IUMTYPES_

typedef ULONGLONG TRUSTLET_IDENTITY, *PTRUSTLET_IDENTITY;


typedef struct _PS_TRUSTLET_TKSESSION_ID {
    ULONGLONG SessionId[256 / 64];
} PS_TRUSTLET_TKSESSION_ID, *PPS_TRUSTLET_TKSESSION_ID;

#endif /* _IUMTYPES_ */

#ifndef _IUMCRYPTO_

#pragma pack(push)
#pragma pack(1)


typedef struct _TRUSTLET_BINDING_DATA
{
    TRUSTLET_IDENTITY TrustletIdentity;
    PS_TRUSTLET_TKSESSION_ID TrustletSessionId;
    ULONG TrustletSvn;
    UINT32 Reserved1;
    UINT64 Reserved2;

} TRUSTLET_BINDING_DATA, *PTRUSTLET_BINDING_DATA;


#pragma pack(pop)

#endif /* _IUMCRYPTO_ */

#ifdef __cplusplus
}
#endif

#endif /* _ENCLAVEIUM_ */

