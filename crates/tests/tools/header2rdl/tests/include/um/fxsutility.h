/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND, 
EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED 
WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

Module Name:

    FxsSnd.h

Abstract:

    This header file contains prototypes for "Send to Fax Recipient" functionality.

--*/

#ifndef _FXS_UTILITY_H_
#define _FXS_UTILITY_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
	  SEND_TO_FAX_RECIPIENT_ATTACHMENT
} SendToMode;

BOOL WINAPI CanSendToFaxRecipient();
DWORD WINAPI SendToFaxRecipient(SendToMode sndMode, LPCWSTR lpFileName);

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif


