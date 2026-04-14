//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation.  All Rights Reserved.
//
//  THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
//  ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
//  THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
//  PARTICULAR PURPOSE.
//
//  File:       msiehost.h
//
//  Contents:   Definitions for Web Browser OC host (not automatable)
//
//=--------------------------------------------------------------------------=

// Definitions for IOleCommandTarget IDs


#ifndef _MSIEHOST_H
#define _MSIEHOST_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


EXTERN_C const GUID CGID_InternetExplorer;

// CGID_InternetExplorer CMDID definitions
#define IECMDID_CLEAR_AUTOCOMPLETE_FOR_FORMS        0
#define IECMDID_SETID_AUTOCOMPLETE_FOR_FORMS        1

#define IECMDID_BEFORENAVIGATE_GETSHELLBROWSE     2
#define IECMDID_BEFORENAVIGATE_DOEXTERNALBROWSE   3
#define IECMDID_BEFORENAVIGATE_GETIDLIST          4

//  Takes a VARIANT of type VT_BOOL for pvarargIn
//  to set it.
#define IECMDID_SET_INVOKE_DEFAULT_BROWSER_ON_NEW_WINDOW    5

//  Sets a VARIANT of type VT_BOOL for pvarargOut
#define IECMDID_GET_INVOKE_DEFAULT_BROWSER_ON_NEW_WINDOW    6

// Values for first parameter of IEID_CLEAR_AUTOCOMPLETE_FOR_FORMS
#define IECMDID_ARG_CLEAR_FORMS_ALL                 0
#define IECMDID_ARG_CLEAR_FORMS_ALL_BUT_PASSWORDS   1
#define IECMDID_ARG_CLEAR_FORMS_PASSWORDS_ONLY      2


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif


