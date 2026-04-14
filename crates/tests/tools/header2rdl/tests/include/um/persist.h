/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
//
// File:        persist.h
//
// Contents:    MSPersist external constants GUIDS and other things users need
//
// Comments:
//
//-----------------------------------------------------------------------------

#ifndef PERSIST_H
#define PERSIST_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#undef PERSISTDECLSPEC
#if _MSC_VER >= 1100 && (!defined(SHx) || (defined(SHx) && _MSC_VER >= 1200))
#define PERSISTDECLSPEC __declspec(selectany)
#else
#define PERSISTDECLSPEC 
#endif //_MSC_VER

// Persist Properties
#define DBPROPFLAGS_PERSIST 0x2000

#define DBPROPVAL_PERSIST_ADTG 0
#define DBPROPVAL_PERSIST_XML 1
#define DBPROP_PersistFormat 2
#define DBPROP_PersistSchema 3
#define DBPROP_HCHAPTER 4
#define DBPROP_MAINTAINPROPS 5

// XML writing only properties
#define DBPROP_Unicode  6

// internal property (read-only)
#define DBPROP_INTERLEAVEDROWS 8

extern const PERSISTDECLSPEC CLSID CLSID_MSPersist
 = { 0x7c07e0d0, 0x4418, 0x11d2, { 0x92, 0x12, 0x0, 0xc0, 0x4f, 0xbb, 0xbf, 0xb3 } };

// {4D7839A0-5B8E-11d1-A6B3-00A0C9138C66}
extern const PERSISTDECLSPEC GUID   DBPROPSET_PERSIST
 = { 0x4d7839a0, 0x5b8e, 0x11d1, { 0xa6, 0xb3, 0x0, 0xa0, 0xc9, 0x13, 0x8c, 0x66 } };

#define MS_PERSIST_PROGID "MSPersist"

extern const PERSISTDECLSPEC char *PROGID_MSPersist = MS_PERSIST_PROGID;

extern const PERSISTDECLSPEC WCHAR *PROGID_MSPersist_W = L"MSPersist";

extern const PERSISTDECLSPEC char *PROGID_MSPersist_Version = MS_PERSIST_PROGID ".1";

extern const PERSISTDECLSPEC WCHAR *PROGID_MSPersist_Version_W = L"MSPersist.1";


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // PERSIST_H

