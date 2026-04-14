//
//    Copyright (C) Microsoft.  All rights reserved.
//
#ifndef __dsquery_h
#define __dsquery_h

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// query handler ID for dsquery.
//

DEFINE_GUID(CLSID_DsQuery, 0x8a23e65e, 0x31c2, 0x11d0, 0x89, 0x1c, 0x0, 0xa0, 0x24, 0xab, 0x2d, 0xbb);

//
// standard forms shipped in dsquery.dll
//

DEFINE_GUID(CLSID_DsFindObjects, 0x83ee3fe1, 0x57d9, 0x11d0, 0xb9, 0x32, 0x0, 0xa0, 0x24, 0xab, 0x2d, 0xbb);
DEFINE_GUID(CLSID_DsFindPeople, 0x83ee3fe2, 0x57d9, 0x11d0, 0xb9, 0x32, 0x0, 0xa0, 0x24, 0xab, 0x2d, 0xbb);
DEFINE_GUID(CLSID_DsFindPrinter, 0xb577f070, 0x7ee2, 0x11d0, 0x91, 0x3f, 0x0, 0xaa, 0x0, 0xc1, 0x6e, 0x65);
DEFINE_GUID(CLSID_DsFindComputer, 0x16006700, 0x87ad, 0x11d0, 0x91, 0x40, 0x0, 0xaa, 0x0, 0xc1, 0x6e, 0x65);
DEFINE_GUID(CLSID_DsFindVolume, 0xc1b3cbf1, 0x886a, 0x11d0, 0x91, 0x40, 0x0, 0xaa, 0x0, 0xc1, 0x6e, 0x65);
DEFINE_GUID(CLSID_DsFindContainer, 0xc1b3cbf2, 0x886a, 0x11d0, 0x91, 0x40, 0x0, 0xaa, 0x0, 0xc1, 0x6e, 0x65);
DEFINE_GUID(CLSID_DsFindAdvanced, 0x83ee3fe3, 0x57d9, 0x11d0, 0xb9, 0x32, 0x0, 0xa0, 0x24, 0xab, 0x2d, 0xbb);

//
// admin forms
//

DEFINE_GUID(CLSID_DsFindDomainController, 0x538c7b7e, 0xd25e, 0x11d0, 0x97, 0x42, 0x0, 0xa0, 0xc9, 0x6, 0xaf, 0x45);
DEFINE_GUID(CLSID_DsFindWriteableDomainController, 0x7cbef079, 0xaa84, 0x444b, 0xbc, 0x70, 0x68, 0xe4, 0x12, 0x83, 0xea, 0xbc);
DEFINE_GUID(CLSID_DsFindFrsMembers, 0x94ce4b18, 0xb3d3, 0x11d1, 0xb9, 0xb4, 0x0, 0xc0, 0x4f, 0xd8, 0xd5, 0xb0);


#ifndef GUID_DEFS_ONLY

//
// DSQUERYINITPARAMS
// -----------------
//  This structured is used when creating a new query view.
//

#define DSQPF_NOSAVE                 0x00000001 // = 1 => remove save verb
#define DSQPF_SAVELOCATION           0x00000002 // = 1 => pSaveLocation contains directory to save queries into
#define DSQPF_SHOWHIDDENOBJECTS      0x00000004 // = 1 => show objects marked as "hidden" in results
#define DSQPF_ENABLEADMINFEATURES    0x00000008 // = 1 => show admin verbs, property pages etc
#define DSQPF_ENABLEADVANCEDFEATURES 0x00000010 // = 1 => set the advanced flag for the property pages
#define DSQPF_HASCREDENTIALS         0x00000020 // = 1 => pServer, pUserName & pPassword are valid
#define DSQPF_NOCHOOSECOLUMNS        0x00000040 // = 1 => remove choose columns from view

typedef struct
{
    DWORD  cbStruct;
    DWORD  dwFlags;
    LPWSTR pDefaultScope;           // -> Active Directory path to use as scope / == NULL for none
    LPWSTR pDefaultSaveLocation;    // -> Directory to save queries into / == NULL default location
    LPWSTR pUserName;               // -> user name to authenticate with
    LPWSTR pPassword;               // -> password for authentication
    LPWSTR pServer;                 // -> server to use for obtaining trusts etc
} DSQUERYINITPARAMS, * LPDSQUERYINITPARAMS;


//
// DSQUERYPARAMS
// -------------
//  The DS query handle takes a packed structure which contains the
//  columns and query to be issued.
//

#define CFSTR_DSQUERYPARAMS         TEXT("DsQueryParameters")

#define DSCOLUMNPROP_ADSPATH        ((LONG)(-1))
#define DSCOLUMNPROP_OBJECTCLASS    ((LONG)(-2))

typedef struct
{
    DWORD dwFlags;                  // flags for this column
    INT   fmt;                      // list view form information
    INT   cx;                       // default column width
    INT   idsName;                  // resource ID for the column dispaly name
    LONG  offsetProperty;           // offset to BSTR defining column ADs property name
    DWORD dwReserved;               // reserved field
} DSCOLUMN, * LPDSCOLUMN;

typedef struct
{
    DWORD     cbStruct;
    DWORD     dwFlags;
    HINSTANCE hInstance;            // instance handle used for string extraction
    LONG      offsetQuery;          // offset to LDAP filter string
    LONG      iColumns;             // column count
    DWORD     dwReserved;           // reserved field for this query
    DSCOLUMN  aColumns[1];          // array of column descriptions
} DSQUERYPARAMS, * LPDSQUERYPARAMS;


//
// CF_DSQUERYSCOPE
// ---------------
//  A clipboard format the puts a string version of the scope into a
//  storage medium via GlobalAlloc.
//
#define CFSTR_DSQUERYSCOPE         TEXT("DsQueryScope")


//
// DSQPM_GETCLASSLIST
// ------------------
//  This page message is sent to the form pages to retrieve the list of classes
//  that the pages are going to query from.  This is used by the feild selector
//  and the property well to build its list of display classes.
//

typedef struct
{
    DWORD   cbStruct;
    LONG    cClasses;               // number of classes in array
    DWORD   offsetClass[1];         // offset to the class names (UNICODE)
} DSQUERYCLASSLIST, * LPDSQUERYCLASSLIST;


#define DSQPM_GETCLASSLIST          (CQPM_HANDLERSPECIFIC+0) // wParam == flags, lParam = LPLPDSQUERYCLASSLIST


//
// DSQPM_HELPTOPICS
// ----------------
//  This page message is sent to the form pages to allow them to handle the
//  "Help Topics" verb.
//

#define DSQPM_HELPTOPICS            (CQPM_HANDLERSPECIFIC+1) // wParam = 0, lParam = hWnd parent



#endif  // GUID_DEFS_ONLY

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
