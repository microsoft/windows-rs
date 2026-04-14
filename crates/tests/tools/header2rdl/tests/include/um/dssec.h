//
//    Copyright (C) Microsoft.  All rights reserved.
//
#ifndef _DSSEC_H_
#define _DSSEC_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <aclui.h>  // LPSECURITYINFO


//+---------------------------------------------------------------------------
//
//  Function:   PFNREADOBJECTSECURITY
//
//  Synopsis:   Reads the security descriptor of a DS object
//
//  Arguments:  [IN  LPCWSTR]               --  ADS path of DS Object
//              [IN  SECURITY_INFORMATION]  --  Which SD parts to read
//              [OUT PSECURITY_DESCRIPTOR*] --  Return SD here. Caller frees with LocalFree
//              [IN  LPARAM]                --  Context param
//
//  Return:     HRESULT
//
//----------------------------------------------------------------------------
//
//  Function:   PFNWRITEOBJECTSECURITY
//
//  Synopsis:   Writes a security descriptor to a DS object
//
//  Arguments:  [IN  LPCWSTR]               --  ADS path of DS Object
//              [IN  SECURITY_INFORMATION]  --  Which SD parts to write
//              [OUT PSECURITY_DESCRIPTOR]  --  Security descriptor to write
//              [IN  LPARAM]                --  Context param
//
//  Return:     HRESULT
//
//----------------------------------------------------------------------------
typedef HRESULT (WINAPI *PFNREADOBJECTSECURITY)(LPCWSTR, SECURITY_INFORMATION, PSECURITY_DESCRIPTOR*, LPARAM);
typedef HRESULT (WINAPI *PFNWRITEOBJECTSECURITY)(LPCWSTR, SECURITY_INFORMATION, PSECURITY_DESCRIPTOR, LPARAM);

//+---------------------------------------------------------------------------
//
//  Function:   DSCreateISecurityInfoObject
//
//  Synopsis:   Instantiates an ISecurityInfo interface for a DS object
//
//  Arguments:  [IN  pwszObjectPath]    --  Full ADS path of DS object
//              [IN  pwszObjectClass]   --  Class of the object (optional)
//              [IN  dwFlags]           --  Combination of DSSI_* flags
//              [OUT ppSI]              --  Interface pointer returned here
//              [IN  pfnReadSD]         --  Optional function for reading SD
//              [IN  pfnWriteSD]        --  Optional function for writing SD
//              [IN  LPARAM]            --  Passed to pfnReadSD/pfnWriteSD
//
//  Return:     HRESULT
//
//----------------------------------------------------------------------------
STDAPI
DSCreateISecurityInfoObject(LPCWSTR pwszObjectPath,
                            LPCWSTR pwszObjectClass,
                            DWORD dwFlags,
                            _Out_ LPSECURITYINFO *ppSI,
                            _In_opt_ PFNREADOBJECTSECURITY pfnReadSD,
                            _In_opt_ PFNWRITEOBJECTSECURITY pfnWriteSD,
                            LPARAM lpContext);

// Flags for DSCreateISecurityInfoObject
#define DSSI_READ_ONLY          0x00000001
#define DSSI_NO_ACCESS_CHECK    0x00000002
#define DSSI_NO_EDIT_SACL       0x00000004
#define DSSI_NO_EDIT_OWNER      0x00000008
#define DSSI_IS_ROOT            0x00000010
#define DSSI_NO_FILTER          0x00000020
#define DSSI_NO_READONLY_MESSAGE          0x00000040

//
// Same as above, with optional server, user & password arguments.
// If use & password are not provided, ADSI defaults are used.
// If the server is not provided, it is obtained from the object
// path or DsGetDcName.
//
STDAPI
DSCreateISecurityInfoObjectEx(LPCWSTR pwszObjectPath,
                              LPCWSTR pwszObjectClass,
                              LPCWSTR pwszServer,
                              LPCWSTR pwszUserName,
                              LPCWSTR pwszPassword,
                              DWORD   dwFlags,
                              _Out_ LPSECURITYINFO *ppSI,
                              _In_opt_ PFNREADOBJECTSECURITY  pfnReadSD,
                              _In_opt_ PFNWRITEOBJECTSECURITY pfnWriteSD,
                              LPARAM lpContext);


//+---------------------------------------------------------------------------
//
//  Function:   DSCreateSecurityPage
//
//  Synopsis:   Creates a Security property page for a DS object
//
//  Arguments:  [IN  pwszObjectPath]    --  Full ADS path of DS object
//              [IN  pwszObjectClass]   --  Class of the object (optional)
//              [IN  dwFlags]           --  Combination of DSSI_* flags
//              [OUT phPage]            --  HPROPSHEETPAGE returned here
//              [IN  pfnReadSD]         --  Optional function for reading SD
//              [IN  pfnWriteSD]        --  Optional function for writing SD
//              [IN  LPARAM]            --  Passed to pfnReadSD/pfnWriteSD
//
//  Return:     HRESULT
//
//----------------------------------------------------------------------------
STDAPI
DSCreateSecurityPage(LPCWSTR pwszObjectPath,
                     LPCWSTR pwszObjectClass,
                     DWORD dwFlags,
                     _Out_ HPROPSHEETPAGE *phPage,
                     _In_opt_ PFNREADOBJECTSECURITY pfnReadSD,
                     _In_opt_ PFNWRITEOBJECTSECURITY pfnWriteSD,
                     LPARAM lpContext);

//+---------------------------------------------------------------------------
//
//  Function:   DSEditSecurity
//
//  Synopsis:   Displays a modal dialog for editing security on a DS object
//
//  Arguments:  [IN  hwndOwner]         --  Dialog owner window
//              [IN  pwszObjectPath]    --  Full ADS path of DS object
//              [IN  pwszObjectClass]   --  Class of the object (optional)
//              [IN  dwFlags]           --  Combination of DSSI_* flags
//              [IN  pwszCaption]       --  Optional dialog caption
//              [IN  pfnReadSD]         --  Optional function for reading SD
//              [IN  pfnWriteSD]        --  Optional function for writing SD
//              [IN  LPARAM]            --  Passed to pfnReadSD/pfnWriteSD
//
//  Return:     HRESULT
//
//----------------------------------------------------------------------------
STDAPI
DSEditSecurity(HWND hwndOwner,
               LPCWSTR pwszObjectPath,
               LPCWSTR pwszObjectClass,
               DWORD dwFlags,
               LPCWSTR pwszCaption,
               _In_opt_ PFNREADOBJECTSECURITY pfnReadSD,
               _In_opt_ PFNWRITEOBJECTSECURITY pfnWriteSD,
               LPARAM lpContext);


typedef HRESULT (WINAPI *PFNDSCREATEISECINFO)(LPCWSTR,
                                              LPCWSTR,
                                              DWORD,
                                              LPSECURITYINFO*,
                                              PFNREADOBJECTSECURITY,
                                              PFNWRITEOBJECTSECURITY,
                                              LPARAM);

typedef HRESULT (WINAPI *PFNDSCREATEISECINFOEX)(LPCWSTR,
                                                LPCWSTR,
                                                LPCWSTR,
                                                LPCWSTR,
                                                LPCWSTR,
                                                DWORD,
                                                LPSECURITYINFO*,
                                                PFNREADOBJECTSECURITY,
                                                PFNWRITEOBJECTSECURITY,
                                                LPARAM);

typedef HRESULT (WINAPI *PFNDSCREATESECPAGE)(LPCWSTR,
                                             LPCWSTR,
                                             DWORD,
                                             HPROPSHEETPAGE*,
                                             PFNREADOBJECTSECURITY,
                                             PFNWRITEOBJECTSECURITY,
                                             LPARAM);

typedef HRESULT (WINAPI *PFNDSEDITSECURITY)(HWND,
                                            LPCWSTR,
                                            LPCWSTR,
                                            DWORD,
                                            LPCWSTR,
                                            PFNREADOBJECTSECURITY,
                                            PFNWRITEOBJECTSECURITY,
                                            LPARAM);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  /* _DSSEC_H_ */
