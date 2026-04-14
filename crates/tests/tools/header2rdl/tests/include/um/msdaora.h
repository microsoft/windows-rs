/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
// File:            msdaora.h
//
// Contents:        Internal GUIDS
//
// Comments:        
//
//-----------------------------------------------------------------------------

#ifndef __MSDAORAGUIDS_DEFINED__
#define __MSDAORAGUIDS_DEFINED__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#undef MSDAORADECLSPEC
#if _MSC_VER >= 1100 && (!defined(SHx) || (defined(SHx) && _MSC_VER >= 1200))
#define MSDAORADECLSPEC __declspec(selectany)
#else
#define MSDAORADECLSPEC 
#endif //_MSC_VER

EXTERN_C const MSDAORADECLSPEC CLSID CLSID_MSDAORA        = {0xE8CC4CBE,0xFDFF,0x11D0,{0xB8,0x65,0x00,0xA0,0xC9,0x08,0x1C,0x1D}};
EXTERN_C const MSDAORADECLSPEC CLSID CLSID_MSDAORA_ERROR  = {0xE8CC4CBF,0xFDFF,0x11D0,{0xB8,0x65,0x00,0xA0,0xC9,0x08,0x1C,0x1D}};

EXTERN_C const MSDAORADECLSPEC CLSID CLSID_MSDAORA8       = {0x7f06a373,0xdd6a,0x43db,{0xb4,0xe0,0x1f,0xc1,0x21,0xe5,0xe6,0x2b}};
EXTERN_C const MSDAORADECLSPEC CLSID CLSID_MSDAORA8_ERROR = {0x7f06a374,0xdd6a,0x43db,{0xb4,0xe0,0x1f,0xc1,0x21,0xe5,0xe6,0x2b}};

#ifdef __cplusplus

class DECLSPEC_UUID("E8CC4CBE-FDFF-11D0-B865-00A0C9081C1D")
MSDAORA;

class DECLSPEC_UUID("E8CC4CBF-FDFF-11D0-B865-00A0C9081C1D")
MSDAORA_ERROR;

class DECLSPEC_UUID("7F06A373-DD6A-43db-B4E0-1FC121E5E62B")
MSDAORA8;

class DECLSPEC_UUID("7F06A374-DD6A-43db-B4E0-1FC121E5E62B")
MSDAORA8_ERROR;

#endif // __cplusplus

//----------------------------------------------------------------------------
// MSDAORA specific properties
//

extern const MSDAORADECLSPEC GUID DBPROPSET_MSDAORA_ROWSET  = {0xE8CC4CBD,0xFDFF,0x11D0,{0xB8,0x65,0x00,0xA0,0xC9,0x08,0x1C,0x1D}};

extern const MSDAORADECLSPEC GUID DBPROPSET_MSDAORA8_ROWSET = {0x7f06a375,0xdd6a,0x43db,{0xb4,0xe0,0x1f,0xc1,0x21,0xe5,0xe6,0x2b}};

// PropIds under DBPROPSET_MSDAORA_ROWSET 
#define DBPROP_MSDAORA_DETERMINEKEYCOLUMNS  1
#define DBPROP_MSDAORA8_DETERMINEKEYCOLUMNS 2


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __MSDAORAGUIDS_DEFINED__
