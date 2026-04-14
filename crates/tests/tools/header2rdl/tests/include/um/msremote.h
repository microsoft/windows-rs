/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
//
// File:        msremote.h
//
// Contents:    MSRemote external constants GUIDS and other things users need
//
// Comments:
//
//-----------------------------------------------------------------------------

#ifndef MSRemote_INCLUDED
#define MSRemote_INCLUDED

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#undef MSREMOTEDECLSPEC
#if _MSC_VER >= 1100 && (!defined(SHx) || (defined(SHx) && _MSC_VER >= 1200))
#define MSREMOTEDECLSPEC __declspec(selectany)
#else
#define MSREMOTEDECLSPEC 
#endif //_MSC_VER

#define MS_REMOTE_PROGID    "MS Remote"
#define MS_REMOTE_FILENAME  "MSDAREM.DLL"
#define MS_REMOTE_WPROGID    L"MS Remote"
#define MS_REMOTE_WFILENAME  L"MSDAREM.DLL"

extern const MSREMOTEDECLSPEC CLSID CLSID_MSRemote  //DSO
 = { 0x27016870, 0x8e02, 0x11d1, { 0x92, 0x4e, 0x0, 0xc0, 0x4f, 0xbb, 0xbf, 0xb3 } };

extern const MSREMOTEDECLSPEC CLSID CLSID_MSRemoteSession
 = { 0x27016871, 0x8e02, 0x11d1, { 0x92, 0x4e, 0x0, 0xc0, 0x4f, 0xbb, 0xbf, 0xb3 } };

extern const MSREMOTEDECLSPEC CLSID CLSID_MSRemoteCommand
 = { 0x27016872, 0x8e02, 0x11d1, { 0x92, 0x4e, 0x0, 0xc0, 0x4f, 0xbb, 0xbf, 0xb3 } };

extern const MSREMOTEDECLSPEC char *PROGID_MSRemote = MS_REMOTE_PROGID;

extern const MSREMOTEDECLSPEC WCHAR *PROGID_WMSRemote = MS_REMOTE_WPROGID;

extern const MSREMOTEDECLSPEC char *PROGID_MSRemote_Version = MS_REMOTE_PROGID ".1";

extern const MSREMOTEDECLSPEC WCHAR *PROGID_WMSRemote_Version = MS_REMOTE_WPROGID L".1";

extern const MSREMOTEDECLSPEC GUID DBPROPSET_MSREMOTE_DBINIT
 = { 0x27016873, 0x8e02, 0x11d1, { 0x92, 0x4e, 0x0, 0xc0, 0x4f, 0xbb, 0xbf, 0xb3 } };

#define DBPROP_MSREMOTE_SERVER             2   //Name="Remote Server", type=VT_BSTR, def=VT_EMPTY
#define DBPROP_MSREMOTE_PROVIDER           3   //Name="Remote Provider", type=VT_BSTR, def=VT_EMPTY
#define DBPROP_MSREMOTE_HANDLER            4   //Name="Handler", type=VT_BSTR, def=VT_EMPTY
#define DBPROP_MSREMOTE_DFMODE             5   //Name="DFMode", type=VT_BSTR, def=VT_EMPTY
#define DBPROP_MSREMOTE_INTERNET_TIMEOUT   6   //Name="Internet Timeout", type=VT_I4, def=VT_EMPTY
#define DBPROP_MSREMOTE_TRANSACT_UPDATES   7   //Name="Transact Updates", type=VT_BOOL, def=VARIANT_FALSE
#define DBPROP_MSREMOTE_COMMAND_PROPERTIES 8   //Name="Command Properties", type=VT_BSTR, def=VT_EMPTY

extern const MSREMOTEDECLSPEC GUID DBPROPSET_MSREMOTE_DATASOURCE
 = { 0x27016874, 0x8e02, 0x11d1, { 0x92, 0x4e, 0x0, 0xc0, 0x4f, 0xbb, 0xbf, 0xb3 } };

#define DBPROP_MSREMOTE_CURRENT_DFMODE  2  //Name="Current DFMode", type=VT_I4, def=21


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // MSRemote_INCLUDED

