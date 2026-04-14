/*+--------------------------------------------------------------------------

 Microsoft Windows
 Copyright (c) Microsoft Corporation. All rights reserved.

 File:       aclui.h

 Contents:   Definitions and prototypes for the ACLUI.DLL

---------------------------------------------------------------------------*/

#if (_MSC_VER >= 800)
#if (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif
#pragma warning(disable:4001) /* nonstandard extension : single line comment */
#endif

#ifndef _ACLUI_H_
#define _ACLUI_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <objbase.h>
#include <commctrl.h>   /* for HPROPSHEETPAGE */
#include <accctrl.h>    /* for SE_OBJECT_TYPE */
#include <authz.h>

#if !defined(_ACLUI_)
#define ACLUIAPI    DECLSPEC_IMPORT WINAPI
#else
#define ACLUIAPI    WINAPI
#endif

#ifdef __cplusplus
extern "C" {
#endif  /* __cplusplus */

//
// ISecurityInformation interface
//
//  Methods:
//
//     GetObjectInformation - Allows UI to determine type of object being
//       edited.  Also allows determining if object is a container.
//
//     GetSecurity - Allows retrieving of ACLs from the original object
//                       NOTE: ACLUI will LocalFree the security descriptor
//                       returned by GetSecurity.
//     SetSecurity - Allows setting of the ACLs on the original object
//
//     GetAccessRights - For retrieving the list of rights allowed
//              on this object.
//
//     MapGeneric - For mapping generic rights to standard & specific rights
//
//     GetInheritTypes - For retrieving the list of possible sub-object types
//              for a container.
//
//     PropertySheetCallback - A method which is called back during the various
//              security UI property pages so that specialized work can be
//              done.  Similar to PropSheetPageProc.  If uMsg == PSPCB_CREATE,
//              then any error return value other than E_NOTIMPL will abort
//              the creation of that page.  The type of page being created or
//              destroyed is indicated by the uPage parameter.
//

typedef struct _SI_OBJECT_INFO
{
    DWORD       dwFlags;
    HINSTANCE   hInstance;          // resources (e.g. strings) reside here
    LPWSTR      pszServerName;      // must be present
    LPWSTR      pszObjectName;      // must be present
    LPWSTR      pszPageTitle;       // only valid if SI_PAGE_TITLE is set
    GUID        guidObjectType;     // only valid if SI_OBJECT_GUID is set
} SI_OBJECT_INFO, *PSI_OBJECT_INFO;

// SI_OBJECT_INFO flags
#define SI_EDIT_PERMS               0x00000000L // always implied
#define SI_EDIT_OWNER               0x00000001L
#define SI_EDIT_AUDITS              0x00000002L
#define SI_CONTAINER                0x00000004L
#define SI_READONLY                 0x00000008L
#define SI_ADVANCED                 0x00000010L
#define SI_RESET                    0x00000020L //equals to SI_RESET_DACL|SI_RESET_SACL|SI_RESET_OWNER
#define SI_OWNER_READONLY           0x00000040L
#define SI_EDIT_PROPERTIES          0x00000080L
#define SI_OWNER_RECURSE            0x00000100L
#define SI_NO_ACL_PROTECT           0x00000200L
#define SI_NO_TREE_APPLY            0x00000400L
#define SI_PAGE_TITLE               0x00000800L
#define SI_SERVER_IS_DC             0x00001000L
#define SI_RESET_DACL_TREE          0x00004000L
#define SI_RESET_SACL_TREE          0x00008000L
#define SI_OBJECT_GUID              0x00010000L
#define SI_EDIT_EFFECTIVE           0x00020000L
#define SI_RESET_DACL               0x00040000L
#define SI_RESET_SACL               0x00080000L
#define SI_RESET_OWNER              0x00100000L
#define SI_NO_ADDITIONAL_PERMISSION 0x00200000L
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define SI_VIEW_ONLY                0x00400000L
#define SI_PERMS_ELEVATION_REQUIRED 0x01000000L
#define SI_AUDITS_ELEVATION_REQUIRED 0x02000000L
#define SI_OWNER_ELEVATION_REQUIRED 0x04000000L

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define SI_SCOPE_ELEVATION_REQUIRED 0x08000000L
#endif // NTDDI_VERSION >= NTDDI_WIN8

#endif // (NTDDI_VERSION >= NTDDI_VISTA)
#define SI_MAY_WRITE                0x10000000L //not sure if user can write permission

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define SI_ENABLE_EDIT_ATTRIBUTE_CONDITION 0x20000000L
#define SI_ENABLE_CENTRAL_POLICY    0x40000000L
#define SI_DISABLE_DENY_ACE         0x80000000L
#endif // NTDDI_VERSION >= NTDDI_WIN8

#define SI_EDIT_ALL     (SI_EDIT_PERMS | SI_EDIT_OWNER | SI_EDIT_AUDITS)


typedef struct _SI_ACCESS
{
    const GUID *pguid;
    ACCESS_MASK mask;
    LPCWSTR     pszName;            // may be resource ID
    DWORD       dwFlags;
} SI_ACCESS, *PSI_ACCESS;

// SI_ACCESS flags
#define SI_ACCESS_SPECIFIC  0x00010000L
#define SI_ACCESS_GENERAL   0x00020000L
#define SI_ACCESS_CONTAINER 0x00040000L // general access, container-only
#define SI_ACCESS_PROPERTY  0x00080000L
// ACE inheritance flags (CONTAINER_INHERIT_ACE, etc.) may also be set.
// They will be used as the inheritance when an access is turned on.

typedef struct _SI_INHERIT_TYPE
{
    const GUID *pguid;
    ULONG       dwFlags;
    LPCWSTR     pszName;            // may be resource ID
} SI_INHERIT_TYPE, *PSI_INHERIT_TYPE;

// SI_INHERIT_TYPE flags are a combination of INHERIT_ONLY_ACE,
// CONTAINER_INHERIT_ACE, and OBJECT_INHERIT_ACE.

// For EditSecurityEx/2, the argument actually takes a UINT
// The bottom half is SI_PAGE_TYPE, so the enum value for this
// should never be greater than 0x0000ffff
typedef enum _SI_PAGE_TYPE
{
    SI_PAGE_PERM=0,
    SI_PAGE_ADVPERM,
    SI_PAGE_AUDIT,
    SI_PAGE_OWNER,
    SI_PAGE_EFFECTIVE,
#if (NTDDI_VERSION >= NTDDI_VISTA)
    SI_PAGE_TAKEOWNERSHIP,
#endif // (NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WIN8)
    SI_PAGE_SHARE,
#endif
} SI_PAGE_TYPE;


//
// Page types used by the new advanced ACL UI
//
typedef enum _SI_PAGE_ACTIVATED
{
    SI_SHOW_DEFAULT=0,
    SI_SHOW_PERM_ACTIVATED,
    SI_SHOW_AUDIT_ACTIVATED,
    SI_SHOW_OWNER_ACTIVATED,
    SI_SHOW_EFFECTIVE_ACTIVATED,
    SI_SHOW_SHARE_ACTIVATED,
    SI_SHOW_CENTRAL_POLICY_ACTIVATED,
} SI_PAGE_ACTIVATED;

#define GET_PAGE_TYPE(X)    (UINT)((X) & 0x0000ffff)
#define GET_ACTIVATION_TYPE(Y) (UINT)(((Y) >> 16) & 0x0000ffff)
#define COMBINE_PAGE_ACTIVATION(X,Y) (UINT)(((Y) << 16) | X)


#define DOBJ_RES_CONT           0x00000001L
#define DOBJ_RES_ROOT           0x00000002L
#define DOBJ_VOL_NTACLS         0x00000004L     // NTFS or OFS
#define DOBJ_COND_NTACLS        0x00000008L     // Conditional aces supported.
#define DOBJ_RIBBON_LAUNCH      0x00000010L     // Invoked from explorer ribbon.


// Message to PropertySheetPageCallback (in addition to
// PSPCB_CREATE and PSPCB_RELEASE)
#define PSPCB_SI_INITDIALOG    (WM_USER + 1)


#undef INTERFACE
#define INTERFACE   ISecurityInformation
DECLARE_INTERFACE_IID_(ISecurityInformation, IUnknown, "965FC360-16FF-11d0-91CB-00AA00BBB723")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface)(THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef)(THIS) PURE;
    STDMETHOD_(ULONG, Release)(THIS) PURE;

    // *** ISecurityInformation methods ***
    STDMETHOD(GetObjectInformation) (THIS_ PSI_OBJECT_INFO pObjectInfo ) PURE;
    STDMETHOD(GetSecurity) (THIS_ SECURITY_INFORMATION RequestedInformation,
                            PSECURITY_DESCRIPTOR *ppSecurityDescriptor,
                            BOOL fDefault ) PURE;
    STDMETHOD(SetSecurity) (THIS_ SECURITY_INFORMATION SecurityInformation,
                            PSECURITY_DESCRIPTOR pSecurityDescriptor ) PURE;
    STDMETHOD(GetAccessRights) (THIS_ const GUID* pguidObjectType,
                                DWORD dwFlags, // SI_EDIT_AUDITS, SI_EDIT_PROPERTIES
                                PSI_ACCESS *ppAccess,
                                ULONG *pcAccesses,
                                ULONG *piDefaultAccess ) PURE;
    STDMETHOD(MapGeneric) (THIS_ const GUID *pguidObjectType,
                           UCHAR *pAceFlags,
                           ACCESS_MASK *pMask) PURE;
    STDMETHOD(GetInheritTypes) (THIS_ PSI_INHERIT_TYPE *ppInheritTypes,
                                ULONG *pcInheritTypes ) PURE;
    STDMETHOD(PropertySheetPageCallback)(THIS_ HWND hwnd, UINT uMsg, SI_PAGE_TYPE uPage ) PURE;
};
typedef ISecurityInformation *LPSECURITYINFO;

#undef INTERFACE
#define INTERFACE   ISecurityInformation2
DECLARE_INTERFACE_IID_(ISecurityInformation2, IUnknown, "c3ccfdb4-6f88-11d2-a3ce-00c04fb1782a")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    // *** ISecurityInformation2 methods ***
    STDMETHOD_(BOOL,IsDaclCanonical) (THIS_ IN PACL pDacl) PURE;
    STDMETHOD(LookupSids) (THIS_ IN ULONG cSids, IN PSID *rgpSids, OUT LPDATAOBJECT *ppdo) PURE;
};
typedef ISecurityInformation2 *LPSECURITYINFO2;

// HGLOBAL containing SID_INFO_LIST returned by ISecurityInformation2::LookupSids
#define CFSTR_ACLUI_SID_INFO_LIST   TEXT("CFSTR_ACLUI_SID_INFO_LIST")

// Data structures corresponding to CFSTR_ACLUI_SID_INFO_LIST
typedef struct _SID_INFO
{
    PSID    pSid;
    PWSTR   pwzCommonName;
    PWSTR   pwzClass;       // Used for selecting icon, e.g. "User" or "Group"
    PWSTR   pwzUPN;         // Optional, may be NULL
} SID_INFO, *PSID_INFO;
typedef struct _SID_INFO_LIST
{
    ULONG       cItems;
    SID_INFO    aSidInfo[ANYSIZE_ARRAY];
} SID_INFO_LIST, *PSID_INFO_LIST;

#undef INTERFACE
#define INTERFACE   IEffectivePermission
DECLARE_INTERFACE_IID_(IEffectivePermission, IUnknown, "3853DC76-9F35-407c-88A1-D19344365FBC")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    // *** ISecurityInformation methods ***
    STDMETHOD(GetEffectivePermission) (  THIS_ const GUID* pguidObjectType,
                                         PSID pUserSid,
                                         LPCWSTR pszServerName,
                                         PSECURITY_DESCRIPTOR pSD,
                                         POBJECT_TYPE_LIST *ppObjectTypeList,
                                         ULONG *pcObjectTypeListLength,
                                         PACCESS_MASK *ppGrantedAccessList,
                                         ULONG *pcGrantedAccessListLength) PURE;
};
typedef IEffectivePermission *LPEFFECTIVEPERMISSION;

#undef INTERFACE
#define INTERFACE   ISecurityObjectTypeInfo
DECLARE_INTERFACE_IID_(ISecurityObjectTypeInfo, IUnknown, "FC3066EB-79EF-444b-9111-D18A75EBF2FA")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    // *** ISecurityInformation methods ***
    STDMETHOD(GetInheritSource)(SECURITY_INFORMATION si,
                                PACL pACL,
                                PINHERITED_FROM *ppInheritArray) PURE;
};
typedef ISecurityObjectTypeInfo *LPSecurityObjectTypeInfo;

#if (NTDDI_VERSION >= NTDDI_VISTA)
// Support for separation or read-only ACL viewer and elevated ACL editor
#undef INTERFACE
#define INTERFACE   ISecurityInformation3
DECLARE_INTERFACE_IID_(ISecurityInformation3, IUnknown, "E2CDC9CC-31BD-4f8f-8C8B-B641AF516A1A")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    // *** ISecurityInformation3 methods ***
    STDMETHOD(GetFullResourceName) (THIS_ _Outptr_ LPWSTR *ppszResourceName) PURE;
    STDMETHOD(OpenElevatedEditor) (THIS_ _In_ HWND hWnd, _In_ SI_PAGE_TYPE uPage) PURE;
};
typedef ISecurityInformation3 *LPSECURITYINFO3;
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8)

typedef struct _SECURITY_OBJECT
{
    PWSTR pwszName;
    _Field_size_bytes_ (cbData) PVOID pData;
    DWORD cbData;
    _Field_size_bytes_ (cbData2) PVOID pData2;
    DWORD cbData2;
    DWORD Id;
    BOOLEAN fWellKnown;
} SECURITY_OBJECT, *PSECURITY_OBJECT;

#define SECURITY_OBJECT_ID_OBJECT_SD      1
#define SECURITY_OBJECT_ID_SHARE          2
#define SECURITY_OBJECT_ID_CENTRAL_POLICY 3
#define SECURITY_OBJECT_ID_CENTRAL_ACCESS_RULE 4

typedef struct _EFFPERM_RESULT_LIST
{
    BOOLEAN fEvaluated;
    ULONG cObjectTypeListLength;
    _Field_size_(cObjectTypeListLength)
    OBJECT_TYPE_LIST *pObjectTypeList;
    _Field_size_(cObjectTypeListLength)
    ACCESS_MASK *pGrantedAccessList;
} EFFPERM_RESULT_LIST, *PEFFPERM_RESULT_LIST;

#undef INTERFACE
#define INTERFACE   ISecurityInformation4
DECLARE_INTERFACE_IID_(ISecurityInformation4, IUnknown, "EA961070-CD14-4621-ACE4-F63C03E583E4")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    // *** ISecurityInformation4 methods ***
    STDMETHOD(GetSecondarySecurity) (THIS_ 
                                     _Outptr_result_buffer_(*pSecurityObjectCount) PSECURITY_OBJECT *pSecurityObjects,
                                     _Out_ PULONG pSecurityObjectCount) PURE;
};

typedef ISecurityInformation4 *LPSECURITYINFO4;

#undef INTERFACE
#define INTERFACE   IEffectivePermission
DECLARE_INTERFACE_IID_(IEffectivePermission2, IUnknown, "941FABCA-DD47-4FCA-90BB-B0E10255F20D")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    // *** IEffectivePermission2 methods ***
    //STDMETHOD(GetEffectiveScopePermission) (THIS);
    STDMETHOD(ComputeEffectivePermissionWithSecondarySecurity) (THIS_
        _In_ PSID pSid,
        _In_opt_ PSID pDeviceSid,
        _In_ PCWSTR pszServerName,
        _Inout_updates_(dwSecurityObjectCount) PSECURITY_OBJECT pSecurityObjects,
        _In_ DWORD dwSecurityObjectCount,
        _In_opt_ PTOKEN_GROUPS pUserGroups,
        _When_(pUserGroups != NULL && *pAuthzUserGroupsOperations != AUTHZ_SID_OPERATION_REPLACE_ALL, _In_reads_(pUserGroups->GroupCount))
        _In_opt_ PAUTHZ_SID_OPERATION pAuthzUserGroupsOperations,
        _In_opt_ PTOKEN_GROUPS pDeviceGroups,
        _When_(pDeviceGroups != NULL && *pAuthzDeviceGroupsOperations != AUTHZ_SID_OPERATION_REPLACE_ALL, _In_reads_(pDeviceGroups->GroupCount))
        _In_opt_ PAUTHZ_SID_OPERATION pAuthzDeviceGroupsOperations,
        _In_opt_ PAUTHZ_SECURITY_ATTRIBUTES_INFORMATION pAuthzUserClaims,
        _When_(pAuthzUserClaims != NULL && *pAuthzUserClaimsOperations != AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL, _In_reads_(pAuthzUserClaims->AttributeCount))
        _In_opt_ PAUTHZ_SECURITY_ATTRIBUTE_OPERATION pAuthzUserClaimsOperations,
        _In_opt_ PAUTHZ_SECURITY_ATTRIBUTES_INFORMATION pAuthzDeviceClaims,
        _When_(pAuthzDeviceClaims != NULL && *pAuthzDeviceClaimsOperations != AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL, _In_reads_(pAuthzDeviceClaims->AttributeCount))
        _In_opt_ PAUTHZ_SECURITY_ATTRIBUTE_OPERATION pAuthzDeviceClaimsOperations,
        _Inout_updates_(dwSecurityObjectCount) PEFFPERM_RESULT_LIST pEffpermResultLists);
};
typedef IEffectivePermission2 *LPEFFECTIVEPERMISSION2;
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

// {965FC360-16FF-11d0-91CB-00AA00BBB723}
EXTERN_GUID(IID_ISecurityInformation, 0x965fc360, 0x16ff, 0x11d0, 0x91, 0xcb, 0x0, 0xaa, 0x0, 0xbb, 0xb7, 0x23);
// {c3ccfdb4-6f88-11d2-a3ce-00c04fb1782a}
EXTERN_GUID(IID_ISecurityInformation2, 0xc3ccfdb4, 0x6f88, 0x11d2, 0xa3, 0xce, 0x0, 0xc0, 0x4f, 0xb1, 0x78, 0x2a);
// {3853DC76-9F35-407c-88A1-D19344365FBC}
EXTERN_GUID(IID_IEffectivePermission, 0x3853dc76, 0x9f35, 0x407c, 0x88, 0xa1, 0xd1, 0x93, 0x44, 0x36, 0x5f, 0xbc);
// {FC3066EB-79EF-444b-9111-D18A75EBF2FA}
EXTERN_GUID(IID_ISecurityObjectTypeInfo, 0xfc3066eb, 0x79ef, 0x444b, 0x91, 0x11, 0xd1, 0x8a, 0x75, 0xeb, 0xf2, 0xfa);
#if (NTDDI_VERSION >= NTDDI_VISTA)
// {E2CDC9CC-31BD-4f8f-8C8B-B641AF516A1A}
EXTERN_GUID(IID_ISecurityInformation3, 0xe2cdc9cc, 0x31bd, 0x4f8f, 0x8c, 0x8b, 0xb6, 0x41, 0xaf, 0x51, 0x6a, 0x1a);
#endif // (NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WIN8)
// {EA961070-CD14-4621-ACE4-F63C03E583E4}
EXTERN_GUID(IID_ISecurityInformation4, 0xea961070, 0xcd14, 0x4621, 0xac, 0xe4, 0xf6, 0x3c, 0x3, 0xe5, 0x83, 0xe4);
// {941FABCA-DD47-4FCA-90BB-B0E10255F20D}
EXTERN_GUID(IID_IEffectivePermission2, 0x941fabca, 0xdd47, 0x4fca, 0x90, 0xbb, 0xb0, 0xe1, 0x2, 0x55, 0xf2, 0xd);
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

HPROPSHEETPAGE ACLUIAPI CreateSecurityPage(_In_ LPSECURITYINFO psi );
BOOL ACLUIAPI EditSecurity(_In_ HWND hwndOwner,
                           _In_ LPSECURITYINFO psi );

#if (NTDDI_VERSION >= NTDDI_VISTA)
HRESULT ACLUIAPI EditSecurityAdvanced(_In_ HWND hwndOwner,
                                      _In_ LPSECURITYINFO psi,
                                      _In_ SI_PAGE_TYPE   uSIPage );
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#ifdef __cplusplus
}
#endif  /* __cplusplus */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  /* _ACLUI_H_ */

#if (_MSC_VER >= 800)
#if (_MSC_VER >= 1200)
#pragma warning(pop)
#else
#pragma warning(default:4001)
#endif
#endif
