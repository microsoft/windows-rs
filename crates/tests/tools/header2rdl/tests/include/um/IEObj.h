//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation.  All Rights Reserved.
//
//  File:       ieobj.h
//
//  Description:
//      Defines the IEFRAME API.
//
//----------------------------------------------------------------------------
#ifndef _IEOBJ_
#define _IEOBJ_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// Internet Explorer suppresses script-initiated dialog boxes and other UI
// elements if the calling thread is not the thread for the active tab or is not
// associated with the thread for the active tab.
//
// This method enables any thread to be associated with the specified tab thread
// so that the dialog suppression and other logic treats code running on the
// specified thread the same way that it does for the tab thread.
//
// Parameters:
//
//   dwTabThreadID - ID of the tab thread
//   dwAssociatedThreadID - ID of the thread to associate with the tab thread.
//
STDAPI IEAssociateThreadWithTab(_In_ DWORD dwTabThreadID,
                                _In_ DWORD dwAssociatedThreadID);

//
// This method unassociates a thread previously associated with IEAssociateThreadWithTab.
//
// Parameters:
//
//   dwTabThreadID - ID of the tab thread
//   dwAssociatedThreadID - ID of the thread to disassociate with the tab thread.
//
STDAPI IEDisassociateThreadWithTab(_In_ DWORD dwTabThreadID,
                                   _In_ DWORD dwAssociatedThreadID);


// This method returns Internet Explorer's InPrivate Browsing status.
//
// Return values:
// TRUE- InPrivate Browsing is enabled.
// FALSE - InPrivate Browsing is not enabled.
//
BOOL IEIsInPrivateBrowsing();

// This method returns Internet Explorer's InPrivate Blocking status.
//
// Return values:
// TRUE- InPrivate Blocking is enabled.
// FALSE - InPrivate Blocking is not enabled.
//
BOOL IEInPrivateFilteringEnabled();

// This method returns Internet Explorer's Tracking Protection status.
//
// Return values:
// TRUE- Tracking Protection is enabled.
// FALSE - Tracking Protection is not enabled.
//
BOOL IETrackingProtectionEnabled();


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //_IEOBJ_
