//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  File:       NtQuery.h
//
//  Contents:   Main query header; Defines all exported query API
//
//----------------------------------------------------------------------------

#if !defined(__NTQUERY_H__)
#define __NTQUERY_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include "stgprop.h"

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

#if defined(__cplusplus)
extern "C"
{
#endif

#define CI_VERSION_WDS30 0x102 // 258
#define CI_VERSION_WDS40 0x109 // 265
#define CI_VERSION_WIN70 0x700 // 1792

//
// Use this path for the null catalog, one that doesn't have an index.
// Use it to search for properties of files that are not indexed.
//

#define CINULLCATALOG L"::_noindex_::"

//
// Use this path to connect to the server for administration work
// (i.e. DocStoreAdmin.) No catalog is associated with the connection
//

#define CIADMIN L"::_nodocstore_::"

//
// Minimal support for persistent handlers.
//

STDAPI LoadIFilter( PCWSTR pwcsPath,
                    _In_ IUnknown * pUnkOuter,
                    _Outptr_ void ** ppIUnk );

#define LIFF_LOAD_DEFINED_FILTER                   1
#define LIFF_IMPLEMENT_TEXT_FILTER_FALLBACK_POLICY 2
#define LIFF_FORCE_TEXT_FILTER_FALLBACK            3

STDAPI LoadIFilterEx( PCWSTR pwcsPath,
                      DWORD dwFlags,
                      REFIID riid,
                      _Outptr_ void ** ppIUnk );

STDAPI BindIFilterFromStorage(_In_ IStorage * pStg,
                              _In_ IUnknown * pUnkOuter,
                              _Outptr_ void ** ppIUnk );

STDAPI BindIFilterFromStream(_In_ IStream * pStm,
                             _In_ IUnknown * pUnkOuter,
                             _Outptr_ void ** ppIUnk );

// The Index Server Data Source Object CLSID

#define CLSID_INDEX_SERVER_DSO \
    { 0xF9AE8980, 0x7E52, 0x11d0, \
      { 0x89, 0x64, 0x00, 0xC0, 0x4F, 0xD6, 0x11, 0xD7 } }


// The filename PKEY_Filename property set
#define PSGUID_FILENAME \
    { 0x41CF5AE0, 0xF75A, 0x4806, \
      { 0xBD, 0x87, 0x59, 0xC7, 0xD9, 0x24, 0x8E, 0xB9} }
#define PID_FILENAME 100


// File System Content Index Framework property set

#define DBPROPSET_FSCIFRMWRK_EXT \
    { 0xA9BD1526, 0x6A80, 0x11D0, \
      { 0x8C, 0x9D, 0x00, 0x20, 0xAF, 0x1D, 0x74, 0x0E } }

#define DBPROP_CI_CATALOG_NAME     2
#define DBPROP_CI_INCLUDE_SCOPES   3
#define DBPROP_CI_DEPTHS           4 // obsolete
#define DBPROP_CI_SCOPE_FLAGS      4
#define DBPROP_CI_EXCLUDE_SCOPES   5
#define DBPROP_CI_SECURITY_ID      6
#define DBPROP_CI_QUERY_TYPE       7
#define DBPROP_CI_PROVIDER         8

// The VT_UI4 value of DBPROP_CI_PROVIDER

#define CI_PROVIDER_MSSEARCH          1          // Only try MSSearch
#define CI_PROVIDER_INDEXING_SERVICE  2          // Only try Indexing Service
#define CI_PROVIDER_ALL               0xffffffff // Try all -- the default

// Session level Query Extension property set

#define DBPROPSET_SESS_QUERYEXT \
    { 0x63623309, 0x2d8b, 0x4d17, \
      { 0xb1, 0x52, 0x6e, 0x29, 0x56, 0xc2, 0x6a, 0x70 } }

#define DBPROP_DEFAULT_EQUALS_BEHAVIOR   2

// Query Extension property set

#define DBPROPSET_QUERYEXT \
    { 0xA7AC77ED, 0xF8D7, 0x11CE, \
      { 0xA7, 0x98, 0x00, 0x20, 0xF8, 0x00, 0x80, 0x25 } }

#define DBPROP_USECONTENTINDEX                2
#define DBPROP_DEFERNONINDEXEDTRIMMING        3
#define DBPROP_USEEXTENDEDDBTYPES             4
#define DBPROP_IGNORENOISEONLYCLAUSES         5
#define DBPROP_GENERICOPTIONS_STRING          6
#define DBPROP_FIRSTROWS                      7
#define DBPROP_DEFERCATALOGVERIFICATION       8
#define DBPROP_CATALOGLISTID                  9
#define DBPROP_GENERATEPARSETREE             10
#define DBPROP_APPLICATION_NAME              11
#define DBPROP_FREETEXTANYTERM               12
#define DBPROP_FREETEXTUSESTEMMING           13
#define DBPROP_IGNORESBRI                    14
#define DBPROP_DONOTCOMPUTEEXPENSIVEPROPS    15
#define DBPROP_ENABLEROWSETEVENTS            16
#define DBPROP_SESSION_ID                    17
#define DBPROP_QUERY_ID                      18

// Content Index Framework Core property set

#define DBPROPSET_CIFRMWRKCORE_EXT \
    { 0xafafaca5, 0xb5d1, 0x11d0, \
      { 0x8c, 0x62, 0x00, 0xc0, 0x4f, 0xc2, 0xdb, 0x8d } }

#define DBPROP_MACHINE      2
#define DBPROP_CLIENT_CLSID 3

// MSIDXS Rowset property set

#define DBPROPSET_MSIDXS_ROWSETEXT \
    { 0xaa6ee6b0, 0xe828, 0x11d0, \
      { 0xb2, 0x3e, 0x00, 0xaa, 0x00, 0x47, 0xfc, 0x01 } }

#define MSIDXSPROP_ROWSETQUERYSTATUS        2
#define MSIDXSPROP_COMMAND_LOCALE_STRING    3
#define MSIDXSPROP_QUERY_RESTRICTION        4
#define MSIDXSPROP_PARSE_TREE               5
#define MSIDXSPROP_MAX_RANK                 6
#define MSIDXSPROP_RESULTS_FOUND            7
#define MSIDXSPROP_WHEREID                  8
#define MSIDXSPROP_SERVER_VERSION           9
#define MSIDXSPROP_SERVER_WINVER_MAJOR     10
#define MSIDXSPROP_SERVER_WINVER_MINOR     11
#define MSIDXSPROP_SERVER_NLSVERSION       12
#define MSIDXSPROP_SERVER_NLSVER_DEFINED   13
#define MSIDXSPROP_SAME_SORTORDER_USED     14

//
// Query status values returned by MSIDXSPROP_ROWSETQUERYSTATUS
//
// Bits   Effect
// -----  -----------------------------------------------------
// 00-02  Fill Status: How data is being updated, if at all.
// 03-15  Bitfield query reliability: How accurate the result is

#define STAT_BUSY                       ( 0 )
#define STAT_ERROR                      ( 0x1 )
#define STAT_DONE                       ( 0x2 )
#define STAT_REFRESH                    ( 0x3 )
#define QUERY_FILL_STATUS(x)            ( ( x ) & 0x7 )

#define STAT_PARTIAL_SCOPE              ( 0x8 )
#define STAT_NOISE_WORDS                ( 0x10 )
#define STAT_CONTENT_OUT_OF_DATE        ( 0x20 )
#define STAT_REFRESH_INCOMPLETE         ( 0x40 )
#define STAT_CONTENT_QUERY_INCOMPLETE   ( 0x80 )
#define STAT_TIME_LIMIT_EXCEEDED        ( 0x100 )
#define STAT_SHARING_VIOLATION          ( 0x200 )
#define STAT_MISSING_RELDOC             ( 0x400 )
#define STAT_MISSING_PROP_IN_RELDOC     ( 0x800 )
#define STAT_RELDOC_ACCESS_DENIED       ( 0x1000 )
#define STAT_COALESCE_COMP_ALL_NOISE    ( 0x2000 )
#define QUERY_RELIABILITY_STATUS(x)     ( ( x ) & 0xFFF8 )

// Scope flags

#define QUERY_SHALLOW        0
#define QUERY_DEEP           1
#define QUERY_PHYSICAL_PATH  0
#define QUERY_VIRTUAL_PATH   2

// query property set (PSGUID_QUERY) properties not defined in oledb.h

#define PROPID_QUERY_WORKID        5
#define PROPID_QUERY_UNFILTERED    7
#define PROPID_QUERY_VIRTUALPATH   9
#define PROPID_QUERY_LASTSEENTIME 10

//
// Change or get the current state of a catalog specified.
//
#define CICAT_STOPPED     0x1
#define CICAT_READONLY    0x2
#define CICAT_WRITABLE    0x4
#define CICAT_NO_QUERY    0x8
#define CICAT_GET_STATE   0x10
#define CICAT_ALL_OPENED  0x20

STDAPI SetCatalogState ( WCHAR const * pwcsCat,
                         WCHAR const * pwcsMachine,
                         DWORD dwNewState,
                         DWORD * pdwOldState );

//
// Query catalog state
//

#define CI_STATE_SHADOW_MERGE          0x0001    // Index is performing a shadow merge
#define CI_STATE_MASTER_MERGE          0x0002    // Index is performing a master merge
#define CI_STATE_CONTENT_SCAN_REQUIRED 0x0004    // Index is likely corrupt, and a rescan is required
#define CI_STATE_ANNEALING_MERGE       0x0008    // Index is performing an annealing (optimizing) merge
#define CI_STATE_SCANNING              0x0010    // Scans are in-progress
#define CI_STATE_RECOVERING            0x0020    // Index metadata is being recovered
#define CI_STATE_INDEX_MIGRATION_MERGE 0x0040    // Reserved for future use
#define CI_STATE_LOW_MEMORY            0x0080    // Indexing is paused due to low memory availability
#define CI_STATE_HIGH_IO               0x0100    // Indexing is paused due to a high rate of I/O
#define CI_STATE_MASTER_MERGE_PAUSED   0x0200    // Master merge is paused
#define CI_STATE_READ_ONLY             0x0400    // Indexing has been manually paused (read-only)
#define CI_STATE_BATTERY_POWER         0x0800    // Indexing is paused to conserve battery life
#define CI_STATE_USER_ACTIVE           0x1000    // Indexing is paused due to high user activity (keyboard/mouse)
#define CI_STATE_STARTING              0x2000    // Index is still starting up
#define CI_STATE_READING_USNS          0x4000    // USNs on NTFS volumes are being processed
#define CI_STATE_DELETION_MERGE        0x8000    // Index is performing a deletion merge
#define CI_STATE_LOW_DISK             0x10000    // Index is paused due to low disk availability
#define CI_STATE_HIGH_CPU             0x20000    // Index is paused due to high CPU
#define CI_STATE_BATTERY_POLICY       0x40000    // Indexing is paused due to backoff on battery policy

#ifndef CI_STATE_DEFINED
#define CI_STATE_DEFINED
#include <pshpack4.h>
typedef struct  _CI_STATE
    {
    DWORD cbStruct;
    DWORD cWordList;
    DWORD cPersistentIndex;
    DWORD cQueries;
    DWORD cDocuments;
    DWORD cFreshTest;
    DWORD dwMergeProgress;
    DWORD eState;
    DWORD cFilteredDocuments;
    DWORD cTotalDocuments;
    DWORD cPendingScans;
    DWORD dwIndexSize;
    DWORD cUniqueKeys;
    DWORD cSecQDocuments;
    DWORD dwPropCacheSize;
    }   CI_STATE;

#include <poppack.h>
#endif   // CI_STATE_DEFINED

#if defined __ICommand_INTERFACE_DEFINED__

#if defined __ICommandTree_INTERFACE_DEFINED__

#ifndef __propertydef_h__

typedef struct tagCIPROPERTYDEF
{
    LPWSTR wcsFriendlyName;
    DWORD  dbType;
    DBID   dbCol;
} CIPROPERTYDEF;

#endif //__propertydef_h__

#endif  // __ICommandTree_INTERFACE_DEFINED__
#endif  // __ICommand_INTERFACE_DEFINED__

#if defined(__cplusplus)
}
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __NTQUERY_H__

