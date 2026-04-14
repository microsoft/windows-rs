//-----------------------------------------------------------------------------
//
// GPEDIT.H - Definitions and prototypes for Group Policy
//
// Copyright 1997-2000, Microsoft Corporation
//
//-----------------------------------------------------------------------------

//
// Terminology
//
// Group Policy Editor  -  The tool to view a Group Policy Object.
//
// Group Policy Object  -  A collection of administrator defined policies.
//                         Each Group Policy Object (GPO) has both file system
//                         and Active Directory storage available to it.
//
// IGPEInformation      -  The interface MMC Snapin Extensions use to
//                         talk to the Group Policy Editor.
//
// IGroupPolicyObject   -  The interface used to create/edit a GPO directly
//                         without going through the Group Policy Editor
//


//
// Group Policy Editor MMC SnapIn GUID
//
// {8FC0B734-A0E1-11d1-A7D3-0000F87571E3}

DEFINE_GUID(CLSID_GPESnapIn, 0x8fc0b734, 0xa0e1, 0x11d1, 0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3);


//
// Group Policy Editor node ids
//

//
// Computer Configuration\Windows Settings
// {8FC0B737-A0E1-11d1-A7D3-0000F87571E3}
//

DEFINE_GUID(NODEID_Machine, 0x8fc0b737, 0xa0e1, 0x11d1, 0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3);


//
// Computer Configuration\Software Settings
// {8FC0B73A-A0E1-11d1-A7D3-0000F87571E3}
//

DEFINE_GUID(NODEID_MachineSWSettings, 0x8fc0b73a, 0xa0e1, 0x11d1, 0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3);


//
// User Configuration\Windows Settings
// {8FC0B738-A0E1-11d1-A7D3-0000F87571E3}
//

DEFINE_GUID(NODEID_User, 0x8fc0b738, 0xa0e1, 0x11d1, 0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3);


//
// User Configuration\Software Settings
// {8FC0B73C-A0E1-11d1-A7D3-0000F87571E3}
//

DEFINE_GUID(NODEID_UserSWSettings, 0x8fc0b73c, 0xa0e1, 0x11d1, 0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3);


//
// IGPEInformation interface id
//
// {8FC0B735-A0E1-11d1-A7D3-0000F87571E3}

DEFINE_GUID(IID_IGPEInformation, 0x8fc0b735, 0xa0e1, 0x11d1, 0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3);


//
// Group Policy Object class id
//
// {EA502722-A23D-11d1-A7D3-0000F87571E3}

DEFINE_GUID(CLSID_GroupPolicyObject, 0xea502722, 0xa23d, 0x11d1, 0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3);


//
// Group Policy Object interface id
//
// {EA502723-A23D-11d1-A7D3-0000F87571E3}

DEFINE_GUID(IID_IGroupPolicyObject, 0xea502723, 0xa23d, 0x11d1, 0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3);


//
// GUID that identifies the registry extension
//

#define REGISTRY_EXTENSION_GUID  { 0x35378EAC, 0x683F, 0x11D2, 0xA8, 0x9A, 0x00, 0xC0, 0x4F, 0xBB, 0xCF, 0xA2 }


//
// GUID that identifies the comments extension
//

#define ADMXCOMMENTS_EXTENSION_GUID  { 0x6C5A2A86, 0x9EB3, 0x42b9, 0xAA, 0x83, 0xA7, 0x37, 0x1B, 0xA0, 0x11, 0xB9 }


//========================================================================================
//
// Resultant Set of Policy node ids
//
//========================================================================================

//
// Resultant Set of Policy MMC SnapIn GUID
//
// {6DC3804B-7212-458D-ADB0-9A07E2AE1FA2}

DEFINE_GUID(CLSID_RSOPSnapIn, 0x6dc3804b, 0x7212, 0x458d, 0xad, 0xb0, 0x9a, 0x07, 0xe2, 0xae, 0x1f, 0xa2);


//
// Computer Configuration\Windows Settings
// {BD4C1A2E-0B7A-4A62-A6B0-C0577539C97E}
//

DEFINE_GUID(NODEID_RSOPMachine, 0xbd4c1a2e, 0x0b7a, 0x4a62, 0xa6, 0xb0, 0xc0, 0x57, 0x75, 0x39, 0xc9, 0x7e);


//
// Computer Configuration\Software Settings
// {6A76273E-EB8E-45DB-94C5-25663A5f2C1A}
//

DEFINE_GUID(NODEID_RSOPMachineSWSettings, 0x6a76273e, 0xeb8e, 0x45db, 0x94, 0xc5, 0x25, 0x66, 0x3a, 0x5f, 0x2c, 0x1a);


//
// User Configuration\Windows Settings
// {AB87364F-0CEC-4CD8-9BF8-898F34628FB8}
//

DEFINE_GUID(NODEID_RSOPUser, 0xab87364f, 0x0cec, 0x4cd8, 0x9b, 0xf8, 0x89, 0x8f, 0x34, 0x62, 0x8f, 0xb8);


//
// User Configuration\Software Settings
// {E52C5CE3-FD27-4402-84DE-D9A5F2858910}
//

DEFINE_GUID(NODEID_RSOPUserSWSettings, 0xe52c5ce3, 0xfd27, 0x4402, 0x84, 0xde, 0xd9, 0xa5, 0xf2, 0x85, 0x89, 0x10);


//
// IRSOPInformation interface id
//
// {9A5A81B5-D9C7-49EF-9D11-DDF50968C48D}

DEFINE_GUID(IID_IRSOPInformation, 0x9a5a81b5, 0xd9c7, 0x49ef, 0x9d, 0x11, 0xdd, 0xf5, 0x09, 0x68, 0xc4, 0x8d);


#ifndef _GPEDIT_H_
#define _GPEDIT_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



//
// Define API decoration for direct importing of DLL references.
//

#if !defined(_GPEDIT_)
#define GPEDITAPI DECLSPEC_IMPORT
#else
#define GPEDITAPI
#endif


#ifdef __cplusplus
extern "C" {
#endif


#include <objbase.h>


//
// Group Policy Object Section flags
//

#define GPO_SECTION_ROOT                 0  // Root
#define GPO_SECTION_USER                 1  // User
#define GPO_SECTION_MACHINE              2  // Machine


//
// Group Policy Object types
//

#ifndef GROUP_POLICY_OBJECT_TYPE_ENUM_DEFINED
#define GROUP_POLICY_OBJECT_TYPE_ENUM_DEFINED

typedef enum _GROUP_POLICY_OBJECT_TYPE {
    GPOTypeLocal = 0,                       // Default GPO on the local machine
    GPOTypeRemote,                          // GPO on a remote machine
    GPOTypeDS,                              // GPO in the Active Directory
    GPOTypeLocalUser,                       // User-specific GPO on the local machine 
    GPOTypeLocalGroup                       // Group-specific GPO on the local machine 
} GROUP_POLICY_OBJECT_TYPE, *PGROUP_POLICY_OBJECT_TYPE;

#endif // GROUP_POLICY_OBJECT_TYPE_ENUM_DEFINED

//
// Group Policy Hint types
//

typedef enum _GROUP_POLICY_HINT_TYPE {
    GPHintUnknown = 0,                      // No link information available
    GPHintMachine,                          // GPO linked to a machine (local or remote)
    GPHintSite,                             // GPO linked to a site
    GPHintDomain,                           // GPO linked to a domain
    GPHintOrganizationalUnit,               // GPO linked to a organizational unit
} GROUP_POLICY_HINT_TYPE, *PGROUP_POLICY_HINT_TYPE;


#undef INTERFACE
#define INTERFACE   IGPEInformation
DECLARE_INTERFACE_(IGPEInformation, IUnknown)
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, LPVOID * ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;


    // *** IGPEInformation methods ***

    //
    // Returns the unique Group Policy Object name (a GUID)
    //
    // pszName contains the name on return
    // cchMaxLength is the max number of characters that can be stored in pszName
    //

    STDMETHOD(GetName) (THIS_ _Out_writes_(cchMaxLength) LPOLESTR pszName, int cchMaxLength) PURE;


    //
    // Returns the friendly display name for this Group Policy Object
    //
    // pszName contains the name on return
    // cchMaxLength is the max number of characters that can be stored in pszName
    //

    STDMETHOD(GetDisplayName) (THIS_ _Out_writes_(cchMaxLength) LPOLESTR pszName, int cchMaxLength) PURE;


    //
    // Returns a registry key handle for the requested section.  The returned
    // key is the root of the registry, not the Policies subkey.  To set / read
    // a value in the Policies subkey, you will need to call RegOpenKeyEx to
    // open Software\Policies subkey first.
    //
    // The handle has been opened with ALL ACCESS rights.  Call RegCloseKey
    // on the handle when finished.
    //
    // dwSection is either GPO_SECTION_USER or GPO_SECTION_MACHINE
    // hKey contains the registry key on return
    //

    STDMETHOD(GetRegistryKey) (THIS_ DWORD dwSection, HKEY *hKey) PURE;


    //
    // Returns the Active Directory path to the root of the request section.
    // The path is in ADSI name format.
    //
    // dwSection is one of the GPO_SECTION_* flags
    // pszPath contains the path on return
    // cchMaxPath is the max number of characters that can be stored in pszPath
    //

    STDMETHOD(GetDSPath) (THIS_ DWORD dwSection, _Out_writes_(cchMaxPath) LPOLESTR pszPath, int cchMaxPath) PURE;


    //
    // Returns the UNC path to the root of the requested section.
    //
    // dwSection is one of the GPO_SECTION_* flags
    // pszPath contains the path on return
    // cchMaxPath is the number of characters that can be stored in pszPath.
    //

    STDMETHOD(GetFileSysPath) (THIS_ DWORD dwSection, _Out_writes_(cchMaxPath) LPOLESTR pszPath, int cchMaxPath) PURE;


    //
    // Returns the user preferences (options)
    //
    // Currently, there are no options defined.  This is reserved for future use.
    //
    // dwOptions receives a bitmask value
    //

    STDMETHOD(GetOptions) (THIS_ DWORD *dwOptions) PURE;


    //
    // Returns the type of GPO being edited.
    //
    // The three types are:  a GPO in the Active Directory, the GPO on the local machine,
    // and the GPO on a remote machine.
    //
    // Machine GPOs only have file system storage (no Active Directory storage available).
    // If GetDSPath is called for a machine GPO, the function will succeed
    // and the returned buffer will be the empty string ""
    //
    // Active Directory GPOs have both file system and Active Directory storage available to them.
    //
    // gpoType receives one of the type flags listed above.
    //

    STDMETHOD(GetType) (THIS_ GROUP_POLICY_OBJECT_TYPE *gpoType) PURE;


    //
    // Returns the type of Active Directory object (or machine) that could be linked to
    // this GPO
    //
    // This is a hint api only.  The GPE does not know which Active Directory objects are
    // linked to a particular GPO, but it can offer a hint based upon how the
    // user started the GPE.
    //
    // Use this method with great caution.  Some extensions might want to
    // customize their user interface based upon the scoping for this GPO,
    // but it is easy to offer the wrong namespace.  Best advice is to
    // always offer your full user interface, but if you choose to use this
    // method, always offer your full user interface if you recieve the
    // unknown hint back.
    //
    // gpHint receives one of the hint flags listed above.
    //

    STDMETHOD(GetHint) (THIS_ GROUP_POLICY_HINT_TYPE *gpHint) PURE;


    //
    // Informs the Group Policy Editor that policy settings have changed.
    // Extensions MUST call this methold every time a change is made
    // to a Group Policy Object.
    //
    // bMachine specifies if machine or user policy has changed.
    // bAdd specifies whether this is an add or delete.
    // pGuidExtension is the guid or unique name of extension that
    //    will process this GPO.
    // pGuidSnapin is the guid or unique name of snapin that is making
    //    this call
    //

    STDMETHOD(PolicyChanged) (THIS_ BOOL bMachine, BOOL bAdd, GUID *pGuidExtension, GUID *pGuidSnapin ) PURE;
};
typedef IGPEInformation *LPGPEINFORMATION;


//
// Group Policy Object open / creation flags
//

#define GPO_OPEN_LOAD_REGISTRY      0x00000001  // Load the registry files
#define GPO_OPEN_READ_ONLY          0x00000002  // Open the GPO as read only



//
// Group Policy Object option flags
//

#define GPO_OPTION_DISABLE_USER     0x00000001  // The user portion of this GPO is disabled
#define GPO_OPTION_DISABLE_MACHINE  0x00000002  // The machine portion of this GPO is disabled


#undef INTERFACE
#define INTERFACE   IGroupPolicyObject
DECLARE_INTERFACE_(IGroupPolicyObject, IUnknown)
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, LPVOID * ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;


    // *** IGroupPolicyObject methods ***

    //
    // Creates a new GPO in the Active Directory with the given friendly name
    // and opens it via OpenDSGPO().  If pszDomainName contains a domain
    // controller name, the GPO will be created on that DC.  If it does not
    // specify a domain controller name, the method will select a DC on
    // the callers behalf.
    //
    // pszDomainName contains the ADSI path of the domain root
    // pszDisplayName contains the friendly display name
    // dwFlags is a bitmask of GPO open / creation flags listed above
    //

    STDMETHOD(New) (THIS_ _In_ LPOLESTR pszDomainName, _In_opt_ LPOLESTR pszDisplayName,
                    DWORD dwFlags) PURE;


    //
    // Opens the specified Group Policy Object in the Active Directory
    // based upon the passed in flags.  If pszPath contains a domain
    // controller name, the GPO will be opened on that DC.  If it does
    // not contain a domain controller name, the method will select a
    // DC on the callers behalf.  If the registry is not loaded,
    // GetRegistryKey() will return E_FAIL.
    //
    // pszPath contains the ADSI path to the GPO to open
    // dwFlags is a bitmask of GPO open / creation flags listed above
    //

    STDMETHOD(OpenDSGPO) (THIS_ _In_ LPOLESTR pszPath, DWORD dwFlags) PURE;


    //
    // Opens the default Group Policy Object on this machine with the
    // dwFlags options listed above.  If the registry is not loaded,
    // GetRegistryKey() will return E_FAIL.
    //
    // dwFlags is a bitmask of GPO open / creation flags listed above
    //

    STDMETHOD(OpenLocalMachineGPO) (THIS_ DWORD dwFlags) PURE;


    //
    // Opens the default Group Policy Object on a remote machine with the
    // dwFlags options listed above.  If the registry is not loaded,
    // GetRegistryKey() will return E_FAIL.
    //
    // pszComputerName contains the machine name in \\machine format
    // dwFlags is a bitmask of GPO open / creation flags listed above
    //

    STDMETHOD(OpenRemoteMachineGPO) (THIS_ _In_ LPOLESTR pszComputerName, DWORD dwFlags) PURE;


    //
    // Flushes the registry settings to disk and updates the revision
    // number of the GPO.
    //
    // bMachine specifies if machine or user should be saved.
    // bAdd specifies whether this is an add or delete.
    // pGuidExtension is the guid or unique name of extension that
    //    will process this GPO.
    // pGuid is a guid
    //

    STDMETHOD(Save) (THIS_ BOOL bMachine, BOOL bAdd, GUID *pGuidExtension, GUID *pGuid ) PURE;


    //
    // Deletes this Group Policy Object.
    //
    // After calling this method, no other methods are valid to call
    // since the data will have been deleted.
    //

    STDMETHOD(Delete) (THIS) PURE;


    //
    // Returns the unique Group Policy Object name
    //
    // For Active Directory policy objects, this is a GUID
    // For the local policy object, it is the string "Local"
    // For remote policy objects, it is the computername
    //
    // pszName contains the name on return
    // cchMaxLength is the max number of characters that can be stored in pszName
    //

    STDMETHOD(GetName) (THIS_ _Out_writes_(cchMaxLength) LPOLESTR pszName, int cchMaxLength) PURE;


    //
    // Returns the friendly display name for this Group Policy Object
    //
    // pszName contains the name on return
    // cchMaxLength is the max number of characters that can be stored in pszName
    //

    STDMETHOD(GetDisplayName) (THIS_ _Out_writes_(cchMaxLength) LPOLESTR pszName, int cchMaxLength) PURE;


    //
    // Sets the friendly display name for this Group Policy Object
    //
    // pszName is the new display name
    //

    STDMETHOD(SetDisplayName) (THIS_ _In_ LPOLESTR pszName) PURE;


    //
    // Returns the path to the Group Policy Object
    //
    //
    // If the GPO is an Active Directory object, the path is in ADSI name format.
    // If the GPO is a machine object, it is a file system path
    //
    // pszPath contains the path on return
    // cchMaxPath is the max number of characters that can be stored in pszPath
    //

    STDMETHOD(GetPath) (THIS_ _Out_writes_(cchMaxLength) LPOLESTR pszPath, int cchMaxLength) PURE;


    //
    // Returns the Active Directory path to the root of the request section.
    // The path is in DN name format.
    //
    // dwSection is one of the GPO_SECTION_* flags
    // pszPath contains the path on return
    // cchMaxPath is the max number of characters that can be stored in pszPath
    //

    STDMETHOD(GetDSPath) (THIS_ DWORD dwSection, _Out_writes_(cchMaxPath) LPOLESTR pszPath, int cchMaxPath) PURE;


    //
    // Returns the UNC path to the root of the requested section.
    //
    // dwSection is one of the GPO_SECTION_* flags
    // pszPath contains the path on return
    // cchMaxPath is the number of characters that can be stored in pszPath.
    //

    STDMETHOD(GetFileSysPath) (THIS_ DWORD dwSection, _Out_writes_(cchMaxPath) LPOLESTR pszPath, int cchMaxPath) PURE;


    //
    // Returns a registry key handle for the requested section.  The returned
    // key is the root of the registry, not the Policies subkey.  To set / read
    // a value in the Policies subkey, you will need to call RegOpenKeyEx to
    // open Software\Policies subkey first.
    //
    // The handle has been opened with ALL ACCESS rights.  Call RegCloseKey
    // on the handle when finished.
    //
    // If the GPO was loaded / created without the registry being loaded
    // this method will return E_FAIL.
    //
    // dwSection is either GPO_SECTION_USER or GPO_SECTION_MACHINE
    // hKey contains the registry key on return
    //

    STDMETHOD(GetRegistryKey) (THIS_ DWORD dwSection, HKEY *hKey) PURE;


    //
    // Returns any options for this Group Policy Object
    //
    // dwOptions receives the GPO_OPTION_* flags
    //

    STDMETHOD(GetOptions) (THIS_ DWORD *dwOptions) PURE;


    //
    // Sets any options for this Group Policy Object
    //
    // This method sets any options for this GPO.  To change
    // an option, that flag must be set in the mask field.
    // If the flag is in the mask field, then the dwOptions
    // field is read for the current state.
    //
    // For example:  to disable the GPO, make this call
    //
    //    SetOptions (GPO_OPTION_DISABLED, GPO_OPTION_DISABLED);
    //
    // dwOptions specifies one or more GPO_OPTION_* flags
    // dwMask specificies which of the dwOptions to change
    //

    STDMETHOD(SetOptions) (THIS_ DWORD dwOptions, DWORD dwMask) PURE;


    //
    // Returns the type of GPO being edited.
    //
    // The three types are:  a GPO in the Active Directory, the GPO on the local machine,
    // and the GPO on a remote machine.
    //
    // Machine GPOs only have file system storage (no Active Directory storage available).
    // If GetDSPath is called for a machine GPO, the function will succeed
    // and the returned buffer will be the empty string ""
    //
    // Active Directory GPOs have both file system and Active Directory storage available to them.
    //
    // gpoType receives one of the type flags
    //

    STDMETHOD(GetType) (THIS_ GROUP_POLICY_OBJECT_TYPE *gpoType) PURE;


    //
    // Returns the machine name of the remote GPO
    //
    // This method returns the name passed to OpenRemoteMachineGPO.
    //
    // pszName contains the name on return
    // cchMaxLength is the max number of characters that can be stored in pszName
    //

    STDMETHOD(GetMachineName) (THIS_ _Out_writes_(cchMaxLength) LPOLESTR pszName, int cchMaxLength) PURE;


    //
    // Returns an array of property sheet pages and the number of pages
    // in the array
    //
    // Note, this method will allocate memory for the array with
    // LocalAlloc.  When finished, the caller should free the array
    // with LocalFree
    //
    // hPages address of the pointer for the array of property sheet pages
    // uPageCount receives the number of pages in the array
    //

    STDMETHOD(GetPropertySheetPages) (THIS_ HPROPSHEETPAGE **hPages, UINT *uPageCount) PURE;
};
typedef IGroupPolicyObject *LPGROUPPOLICYOBJECT;


//
// RSOP flags
//

#define RSOP_INFO_FLAG_DIAGNOSTIC_MODE  0x00000001      // Running in diagnostic mode vs planning mode

#undef INTERFACE
#define INTERFACE   IRSOPInformation
DECLARE_INTERFACE_(IRSOPInformation, IUnknown)
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, LPVOID * ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;


    // *** IRSOPInformation methods ***

    //
    // Returns the namespace for the RSOP data
    //
    // dwSection is either GPO_SECTION_USER or GPO_SECTION_MACHINE
    // pszName contains the namespace on return
    // cchMaxLength is the max number of characters that can be stored in pszName
    //

    STDMETHOD(GetNamespace) (THIS_ DWORD dwSection, _Out_writes_(cchMaxLength) LPOLESTR pszName, int cchMaxLength) PURE;


    //
    // Returns information about the RSOP session
    //
    // pdwFlags points to a DWORD which contains the flags on return
    //

    STDMETHOD(GetFlags) (THIS_ DWORD * pdwFlags) PURE;


    //
    // Returns the event log text for a specific entry
    //
    // lpEventSource - event log source name
    // lpEventLogName - event log name
    // lpEventTime - event log time in WMI datetime format
    // dwEventID - event ID
    // lpText - Receives a pointer to a buffer containing the text.
    //          The caller should free this buffer with CoTaskMemFree.
    //

    STDMETHOD(GetEventLogEntryText) (THIS_ _In_ LPOLESTR pszEventSource, _In_ LPOLESTR pszEventLogName,
                                           _In_ LPOLESTR pszEventTime, DWORD dwEventID,  _Outptr_ LPOLESTR *ppszText) PURE;

};
typedef IRSOPInformation *LPRSOPINFORMATION;


//=============================================================================
//
// CreateGPOLink
//
// Creates a link to a GPO for the specified Site, Domain, or Organizational Unit
//
// lpGPO         - ADSI path to the GPO
// lpContainer   - ADSI path to the Site, Domain, or Organizational Unit
// fHighPriority - Create the link as the highest or lowest priority
//
// Returns:  S_OK if successful
//
//=============================================================================

GPEDITAPI
HRESULT
WINAPI
CreateGPOLink(
    _In_ LPOLESTR lpGPO,
    _In_ LPOLESTR lpContainer,
         BOOL fHighPriority);


//=============================================================================
//
// DeleteGPOLink
//
// Deletes a link to a GPO for the specified Site, Domain, or Organizational Unit
//
// lpGPO         - ADSI path to the GPO
// lpContainer   - ADSI path to the Site, Domain, or Organizational Unit
//
// Returns:  S_OK if successful
//
//=============================================================================

GPEDITAPI
HRESULT
WINAPI
DeleteGPOLink(
    _In_ LPOLESTR lpGPO,
    _In_ LPOLESTR lpContainer);


//=============================================================================
//
// DeleteAllGPOLinks
//
// Deletes all GPO links for the specified Site, Domain, or Organizational Unit
//
// lpContainer   - ADSI path to the Site, Domain, or Organizational Unit
//
// Returns:  S_OK if successful
//
//=============================================================================

GPEDITAPI
HRESULT
WINAPI
DeleteAllGPOLinks(
    _In_ LPOLESTR lpContainer);


//=============================================================================
//
// BrowseForGPO
//
// Displays the GPO browser dialog
//
// lpBrowseInfo   - Address of a GPOBROWSEINFO structure
//
// Returns:  S_OK if successful
//
//=============================================================================

//
// Flags passed in the dwFlags field of the GPOBROWSEINFO structure
//

#define GPO_BROWSE_DISABLENEW           0x00000001   // Disables the New GPO functionality on all pages except "All"
#define GPO_BROWSE_NOCOMPUTERS          0x00000002   // Removes the Computers tab
#define GPO_BROWSE_NODSGPOS             0x00000004   // Removes the Domain/OU and Sites tabs
#define GPO_BROWSE_OPENBUTTON           0x00000008   // Change the Ok button to say Open
#define GPO_BROWSE_INITTOALL            0x00000010   // Initialize the dialog focused on the All pane
#define GPO_BROWSE_NOUSERGPOS           0x00000020   // Removes the Users tab
#define GPO_BROWSE_SENDAPPLYONEDIT      0x00000040   // Send the apply event if edit menu is selected

typedef struct tag_GPOBROWSEINFO
{
    DWORD       dwSize;                   // [in] Initialized to the size of this structure
    DWORD       dwFlags;                  // [in] Flags defined above
    HWND        hwndOwner;                // [in] Parent window handle (can be NULL)
    LPOLESTR    lpTitle;                  // [in] Title bar text.  If NULL, "Browse for a Group Policy Object" will be the default text
    LPOLESTR    lpInitialOU;              // [in] Initial Domain/Organizational Unit to open focus on
    _Field_size_(dwDSPathSize) LPOLESTR    lpDSPath;                 // [in/out] Pointer to the buffer that receives the Active Directory GPO path
    DWORD       dwDSPathSize;             // [in] Size in characters of buffer given in lpDSPath
    _Field_size_bytes_( dwNameSize) LPOLESTR    lpName;                   // [in/out] Pointer to a buffer that receives either the computer name or
                                          //      the friendly name of the GPO (can be NULL)
    DWORD       dwNameSize;               // [in] Size in characters of buffer given in lpName
    GROUP_POLICY_OBJECT_TYPE    gpoType;  // [out] Specifies the type of GPO
    GROUP_POLICY_HINT_TYPE      gpoHint;  // [out] Specifies a hint of the GPO association
} GPOBROWSEINFO, *LPGPOBROWSEINFO;


GPEDITAPI
HRESULT
WINAPI
BrowseForGPO(
    LPGPOBROWSEINFO lpBrowseInfo);


//=============================================================================
//
// ImportRSoPData
//
// Imports a data file generated by ExportRSoPData
//
// lpNameSpace   - Namespace to place the data in
// lpFileName    - Filename containing the data
//
// Returns:  S_OK if successful
//
// Notes:    The namespace specified in lpNameSpace must exist prior to calling
//           this function.
//
//=============================================================================

GPEDITAPI
HRESULT
WINAPI
ImportRSoPData(
    _In_ LPOLESTR lpNameSpace,
    _In_ LPOLESTR lpFileName);


//=============================================================================
//
// ExportRSoPData
//
// Exports a WBEM namespace containing RSoP information to a data file.
// This data file can be imported to a WBEM namespace using ImportRSoPData.
//
// lpNameSpace   - Namespace to read the data from
// lpFileName    - Filename to receive the data
//
// Returns:  S_OK if successful
//
// Notes:    This function should be called twice.  Once for the user data
//           and once for the computer data.
//
//=============================================================================

GPEDITAPI
HRESULT
WINAPI
ExportRSoPData(
    _In_ LPOLESTR lpNameSpace,
    _In_ LPOLESTR lpFileName);

#ifdef __cplusplus
}
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  /* _GPEDIT_H_ */
