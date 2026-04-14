/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    qospol.h - QOS policy elements

Abstract:

    This module defines QOS policy elements.

Revision History:

--*/

#ifndef __QOSPOL_H_
#define __QOSPOL_H_

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define PE_TYPE_APPID       3       // policy element contains Application Identity

// Policy Location attribute carries sub application attributes
#define PE_ATTRIB_TYPE_POLICY_LOCATOR            1

#define POLICY_LOCATOR_SUB_TYPE_ASCII_DN         1
#define POLICY_LOCATOR_SUB_TYPE_UNICODE_DN       2
#define POLICY_LOCATOR_SUB_TYPE_ASCII_DN_ENC     3
#define POLICY_LOCATOR_SUB_TYPE_UNICODE_DN_ENC   4


// Credentials attribute carries the application identity
#define PE_ATTRIB_TYPE_CREDENTIAL        2

#define CREDENTIAL_SUB_TYPE_ASCII_ID     1
#define CREDENTIAL_SUB_TYPE_UNICODE_ID   2
#define CREDENTIAL_SUB_TYPE_KERBEROS_TKT 3
#define CREDENTIAL_SUB_TYPE_X509_V3_CERT 4
#define CREDENTIAL_SUB_TYPE_PGP_CERT     5


// Identity Policy Element attribute structure
typedef struct _IDPE_ATTR {

    USHORT  PeAttribLength;
    
    UCHAR   PeAttribType;            // Use the #defines from above
    
    UCHAR   PeAttribSubType;        // Use the #defines from above
    
    UCHAR   PeAttribValue[4];
    
} IDPE_ATTR, *LPIDPE_ATTR;

#define IDPE_ATTR_HDR_LEN    (sizeof(USHORT)+sizeof(UCHAR)+sizeof(UCHAR))

#define RSVP_BYTE_MULTIPLE(x)       (((x+3) / 4) * 4)



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __QOSPOL_H_

