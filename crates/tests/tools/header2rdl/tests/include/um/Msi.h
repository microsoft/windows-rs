
/*****************************************************************************\
*                                                                             *
* msi.h - - Interface for external access to Installer Service                *
*                                                                             *
* Version 4.0                                                                 *
*                                                                             *
* NOTES:  All buffers sizes are TCHAR count, null included only on input      *
*         Return argument pointers may be null if not interested in value     *
*                                                                             *
* Copyright (c) Microsoft Corporation. All rights reserved.                   *
*                                                                             *
\*****************************************************************************/

#ifndef _MSI_H_
#define _MSI_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifndef NTDDI_WIN2K
#define NTDDI_WIN2K                         0x05000000
#endif
#ifndef NTDDI_WINXP
#define NTDDI_WINXP                         0x05010000
#endif
#ifndef NTDDI_WINXPSP2
#define NTDDI_WINXPSP2                      0x05010200
#endif
#ifndef NTDDI_WS03SP1
#define NTDDI_WS03SP1                       0x05020100
#endif
#ifndef NTDDI_VISTA
#define NTDDI_VISTA                         0x06000000
#endif
#ifndef NTDDI_VISTASP1
#define NTDDI_VISTASP1                      0x06000100
#endif
#ifndef NTDDI_WIN7
#define NTDDI_WIN7                          0x06010000
#endif
#ifndef NTDDI_WIN8
#define NTDDI_WIN8                          0x06020000
#endif
#ifndef NTDDI_WINBLUE
#define NTDDI_WINBLUE                       0x06030000
#endif
#ifndef NTDDI_WINTHRESHOLD
#define NTDDI_WINTHRESHOLD                  0x06040000
#endif

#ifndef _WIN32_MSI
#if (_WIN32_WINNT >= 0x0601 || (defined(NTDDI_VERSION) && NTDDI_VERSION >= NTDDI_WIN7))
  #define _WIN32_MSI   500
#elif (_WIN32_WINNT >= 0x0501 || (defined(NTDDI_VERSION) && NTDDI_VERSION >= NTDDI_WINXP))
  #if (defined(NTDDI_VERSION) && NTDDI_VERSION >= NTDDI_VISTASP1)
    #define _WIN32_MSI   450
  #elif (defined(NTDDI_VERSION) && NTDDI_VERSION >= NTDDI_VISTA)
    #define _WIN32_MSI   400
  #elif (defined(NTDDI_VERSION) && NTDDI_VERSION >= NTDDI_WS03SP1)
    #define _WIN32_MSI   310
  #elif (defined(NTDDI_VERSION) && NTDDI_VERSION >= NTDDI_WINXPSP2)
    #define _WIN32_MSI   300
  #else
    #define _WIN32_MSI   200
  #endif
#elif (_WIN32_WINNT >= 0x0500 || (defined(NTDDI_VERSION) && NTDDI_VERSION >= NTDDI_WIN2K))
  #define _WIN32_MSI   110
#else
  #define _WIN32_MSI   100
#endif //_WIN32_WINNT
#endif // !_WIN32_MSI

#include "specstrings.h"
#define MAX_GUID_CHARS  38


#if (_WIN32_MSI >= 150)
#ifndef _MSI_NO_CRYPTO
#include "wincrypt.h"
#endif // _MSI_NO_CRYPTO
#endif //(_WIN32_MSI >= 150)

// --------------------------------------------------------------------------
// Installer generic handle definitions
// --------------------------------------------------------------------------

typedef unsigned long MSIHANDLE;     // abstract generic handle, 0 == no handle

#ifdef __cplusplus
extern "C" {
#endif

// Close a open handle of any type
// All handles obtained from API calls must be closed when no longer needed
// Normally succeeds, returning TRUE. 

UINT WINAPI MsiCloseHandle(MSIHANDLE hAny);

// Close all handles open in the process, a diagnostic call
// This should NOT be used as a cleanup mechanism -- use PMSIHANDLE class
// Can be called at termination to assure that all handles have been closed
// Returns 0 if all handles have been close, else number of open handles

UINT WINAPI MsiCloseAllHandles();

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus

// C++ wrapper object to automatically free handle when going out of scope

class PMSIHANDLE
{
	MSIHANDLE m_h;
 public:
	PMSIHANDLE():m_h(0){}
	PMSIHANDLE(MSIHANDLE h):m_h(h){}
  ~PMSIHANDLE(){if (m_h!=0) MsiCloseHandle(m_h);}
	void operator =(MSIHANDLE h) {if (m_h) MsiCloseHandle(m_h); m_h=h;}
	operator MSIHANDLE() {return m_h;}
	MSIHANDLE* operator &() {if (m_h) MsiCloseHandle(m_h); m_h = 0; return &m_h;}
};
#endif  //__cplusplus

// Install message type for callback is a combination of the following:
//  A message box style:      MB_*, where MB_OK is the default
//  A message box icon type:  MB_ICON*, where no icon is the default
//  A default button:         MB_DEFBUTTON?, where MB_DEFBUTTON1 is the default
//  One of the following install message types, no default
typedef enum tagINSTALLMESSAGE
{
	INSTALLMESSAGE_FATALEXIT      = 0x00000000L, // premature termination, possibly fatal OOM
	INSTALLMESSAGE_ERROR          = 0x01000000L, // formatted error message
	INSTALLMESSAGE_WARNING        = 0x02000000L, // formatted warning message
	INSTALLMESSAGE_USER           = 0x03000000L, // user request message
	INSTALLMESSAGE_INFO           = 0x04000000L, // informative message for log
	INSTALLMESSAGE_FILESINUSE     = 0x05000000L, // list of files in use that need to be replaced
	INSTALLMESSAGE_RESOLVESOURCE  = 0x06000000L, // request to determine a valid source location
	INSTALLMESSAGE_OUTOFDISKSPACE = 0x07000000L, // insufficient disk space message
	INSTALLMESSAGE_ACTIONSTART    = 0x08000000L, // start of action: action name & description
	INSTALLMESSAGE_ACTIONDATA     = 0x09000000L, // formatted data associated with individual action item
	INSTALLMESSAGE_PROGRESS       = 0x0A000000L, // progress gauge info: units so far, total
	INSTALLMESSAGE_COMMONDATA     = 0x0B000000L, // product info for dialog: language Id, dialog caption
	INSTALLMESSAGE_INITIALIZE     = 0x0C000000L, // sent prior to UI initialization, no string data
	INSTALLMESSAGE_TERMINATE      = 0x0D000000L, // sent after UI termination, no string data
	INSTALLMESSAGE_SHOWDIALOG     = 0x0E000000L, // sent prior to display or authored dialog or wizard
#if (_WIN32_MSI >= 500)
	INSTALLMESSAGE_PERFORMANCE    = 0x0F000000L, // log only, to log performance number like action time
#endif // (_WIN32_MSI >= 500)
#if (_WIN32_MSI >= 400)
	INSTALLMESSAGE_RMFILESINUSE   = 0x19000000L, // the list of apps that the user can request Restart Manager to shut down and restart
#endif // (_WIN32_MSI >= 400)
#if (_WIN32_MSI >= 450)
	INSTALLMESSAGE_INSTALLSTART   = 0x1A000000L, // sent prior to server-side install of a product
	INSTALLMESSAGE_INSTALLEND     = 0x1B000000L, // sent after server-side install
#endif // (_WIN32_MSI >= 450)
} INSTALLMESSAGE;

// external error handler supplied to installation API functions
typedef int (WINAPI *INSTALLUI_HANDLERA)(LPVOID pvContext, UINT iMessageType, LPCSTR szMessage);
// external error handler supplied to installation API functions
typedef int (WINAPI *INSTALLUI_HANDLERW)(LPVOID pvContext, UINT iMessageType, LPCWSTR szMessage);
#ifdef UNICODE
#define INSTALLUI_HANDLER  INSTALLUI_HANDLERW
#else
#define INSTALLUI_HANDLER  INSTALLUI_HANDLERA
#endif // !UNICODE
#if (_WIN32_MSI >= 310)
typedef int (WINAPI *INSTALLUI_HANDLER_RECORD)(LPVOID pvContext, UINT iMessageType, MSIHANDLE hRecord);
typedef INSTALLUI_HANDLER_RECORD* PINSTALLUI_HANDLER_RECORD;
#endif // (_WIN32_MSI >= 310)

typedef enum tagINSTALLUILEVEL
{
	INSTALLUILEVEL_NOCHANGE = 0,    // UI level is unchanged
	INSTALLUILEVEL_DEFAULT  = 1,    // default UI is used
	INSTALLUILEVEL_NONE     = 2,    // completely silent installation
	INSTALLUILEVEL_BASIC    = 3,    // simple progress and error handling
	INSTALLUILEVEL_REDUCED  = 4,    // authored UI, wizard dialogs suppressed
	INSTALLUILEVEL_FULL     = 5,    // authored UI with wizards, progress, errors
	INSTALLUILEVEL_ENDDIALOG    = 0x80, // display success/failure dialog at end of install
	INSTALLUILEVEL_PROGRESSONLY = 0x40, // display only progress dialog
	INSTALLUILEVEL_HIDECANCEL   = 0x20, // do not display the cancel button in basic UI
	INSTALLUILEVEL_SOURCERESONLY = 0x100, // force display of source resolution even if quiet
#if (_WIN32_MSI >= 500)
	INSTALLUILEVEL_UACONLY = 0x200, // show UAC prompt even if quiet
#endif // (_WIN32_MSI >= 500)
} INSTALLUILEVEL;

typedef enum tagINSTALLSTATE
{
	INSTALLSTATE_NOTUSED      = -7,  // component disabled
	INSTALLSTATE_BADCONFIG    = -6,  // configuration data corrupt
	INSTALLSTATE_INCOMPLETE   = -5,  // installation suspended or in progress
	INSTALLSTATE_SOURCEABSENT = -4,  // run from source, source is unavailable
	INSTALLSTATE_MOREDATA     = -3,  // return buffer overflow
	INSTALLSTATE_INVALIDARG   = -2,  // invalid function argument
	INSTALLSTATE_UNKNOWN      = -1,  // unrecognized product or feature
	INSTALLSTATE_BROKEN       =  0,  // broken
	INSTALLSTATE_ADVERTISED   =  1,  // advertised feature
	INSTALLSTATE_REMOVED      =  1,  // component being removed (action state, not settable)
	INSTALLSTATE_ABSENT       =  2,  // uninstalled (or action state absent but clients remain)
	INSTALLSTATE_LOCAL        =  3,  // installed on local drive
	INSTALLSTATE_SOURCE       =  4,  // run from source, CD or net
	INSTALLSTATE_DEFAULT      =  5,  // use default, local or source
} INSTALLSTATE;

typedef enum tagUSERINFOSTATE
{
	USERINFOSTATE_MOREDATA   = -3,  // return buffer overflow
	USERINFOSTATE_INVALIDARG = -2,  // invalid function argument
	USERINFOSTATE_UNKNOWN    = -1,  // unrecognized product
	USERINFOSTATE_ABSENT     =  0,  // user info and PID not initialized
	USERINFOSTATE_PRESENT    =  1,  // user info and PID initialized
} USERINFOSTATE;

typedef enum tagINSTALLLEVEL
{
	INSTALLLEVEL_DEFAULT = 0,      // install authored default
	INSTALLLEVEL_MINIMUM = 1,      // install only required features
	INSTALLLEVEL_MAXIMUM = 0xFFFF, // install all features
} INSTALLLEVEL;                   // intermediate levels dependent on authoring

typedef enum tagREINSTALLMODE  // bit flags
{
	REINSTALLMODE_REPAIR           = 0x00000001,  // Reserved bit - currently ignored
	REINSTALLMODE_FILEMISSING      = 0x00000002,  // Reinstall only if file is missing
	REINSTALLMODE_FILEOLDERVERSION = 0x00000004,  // Reinstall if file is missing, or older version
	REINSTALLMODE_FILEEQUALVERSION = 0x00000008,  // Reinstall if file is missing, or equal or older version
	REINSTALLMODE_FILEEXACT        = 0x00000010,  // Reinstall if file is missing, or not exact version
	REINSTALLMODE_FILEVERIFY       = 0x00000020,  // checksum executables, reinstall if missing or corrupt
	REINSTALLMODE_FILEREPLACE      = 0x00000040,  // Reinstall all files, regardless of version
	REINSTALLMODE_MACHINEDATA      = 0x00000080,  // insure required machine reg entries
	REINSTALLMODE_USERDATA         = 0x00000100,  // insure required user reg entries
	REINSTALLMODE_SHORTCUT         = 0x00000200,  // validate shortcuts items
	REINSTALLMODE_PACKAGE          = 0x00000400,  // use re-cache source install package
} REINSTALLMODE;

typedef enum tagINSTALLOGMODE  // bit flags for use with MsiEnableLog and MsiSetExternalUI
{
	INSTALLLOGMODE_FATALEXIT      = (1 << (INSTALLMESSAGE_FATALEXIT      >> 24)),
	INSTALLLOGMODE_ERROR          = (1 << (INSTALLMESSAGE_ERROR          >> 24)),
	INSTALLLOGMODE_WARNING        = (1 << (INSTALLMESSAGE_WARNING        >> 24)),
	INSTALLLOGMODE_USER           = (1 << (INSTALLMESSAGE_USER           >> 24)),
	INSTALLLOGMODE_INFO           = (1 << (INSTALLMESSAGE_INFO           >> 24)),
	INSTALLLOGMODE_RESOLVESOURCE  = (1 << (INSTALLMESSAGE_RESOLVESOURCE  >> 24)),
	INSTALLLOGMODE_OUTOFDISKSPACE = (1 << (INSTALLMESSAGE_OUTOFDISKSPACE >> 24)),
	INSTALLLOGMODE_ACTIONSTART    = (1 << (INSTALLMESSAGE_ACTIONSTART    >> 24)),
	INSTALLLOGMODE_ACTIONDATA     = (1 << (INSTALLMESSAGE_ACTIONDATA     >> 24)),
	INSTALLLOGMODE_COMMONDATA     = (1 << (INSTALLMESSAGE_COMMONDATA     >> 24)),
	INSTALLLOGMODE_PROPERTYDUMP   = (1 << (INSTALLMESSAGE_PROGRESS       >> 24)), // log only
	INSTALLLOGMODE_VERBOSE        = (1 << (INSTALLMESSAGE_INITIALIZE     >> 24)), // log only
	INSTALLLOGMODE_EXTRADEBUG     = (1 << (INSTALLMESSAGE_TERMINATE      >> 24)), // log only
	INSTALLLOGMODE_LOGONLYONERROR = (1 << (INSTALLMESSAGE_SHOWDIALOG     >> 24)), // log only	
#if (_WIN32_MSI >= 500)
	INSTALLLOGMODE_LOGPERFORMANCE = (1 << (INSTALLMESSAGE_PERFORMANCE    >> 24)), // log only	
#endif // (_WIN32_MSI >= 500)
	INSTALLLOGMODE_PROGRESS       = (1 << (INSTALLMESSAGE_PROGRESS       >> 24)), // external handler only
	INSTALLLOGMODE_INITIALIZE     = (1 << (INSTALLMESSAGE_INITIALIZE     >> 24)), // external handler only
	INSTALLLOGMODE_TERMINATE      = (1 << (INSTALLMESSAGE_TERMINATE      >> 24)), // external handler only
	INSTALLLOGMODE_SHOWDIALOG     = (1 << (INSTALLMESSAGE_SHOWDIALOG     >> 24)), // external handler only
	INSTALLLOGMODE_FILESINUSE     = (1 << (INSTALLMESSAGE_FILESINUSE     >> 24)), // external handler only
#if (_WIN32_MSI >= 400)
	INSTALLLOGMODE_RMFILESINUSE   = (1 << (INSTALLMESSAGE_RMFILESINUSE   >> 24)), // external handler only
#endif // (_WIN32_MSI >= 400)
#if (_WIN32_MSI >= 450)
	INSTALLLOGMODE_INSTALLSTART   = (1 << (INSTALLMESSAGE_INSTALLSTART   >> 24)), // external/embedded only
	INSTALLLOGMODE_INSTALLEND     = (1 << (INSTALLMESSAGE_INSTALLEND     >> 24)), // external/embedded only
#endif // (_WIN32_MSI >= 450)
} INSTALLLOGMODE;

typedef enum tagINSTALLLOGATTRIBUTES // flag attributes for MsiEnableLog
{
	INSTALLLOGATTRIBUTES_APPEND            = (1 << 0),
	INSTALLLOGATTRIBUTES_FLUSHEACHLINE     = (1 << 1),
} INSTALLLOGATTRIBUTES;

typedef enum tagINSTALLFEATUREATTRIBUTE // bit flags
{
	INSTALLFEATUREATTRIBUTE_FAVORLOCAL             = 1 << 0,
	INSTALLFEATUREATTRIBUTE_FAVORSOURCE            = 1 << 1,
	INSTALLFEATUREATTRIBUTE_FOLLOWPARENT           = 1 << 2,
	INSTALLFEATUREATTRIBUTE_FAVORADVERTISE         = 1 << 3,
	INSTALLFEATUREATTRIBUTE_DISALLOWADVERTISE      = 1 << 4,
	INSTALLFEATUREATTRIBUTE_NOUNSUPPORTEDADVERTISE = 1 << 5,
} INSTALLFEATUREATTRIBUTE;

typedef enum tagINSTALLMODE
{
#if (_WIN32_MSI >=  150)
	INSTALLMODE_NODETECTION_ANY      = -4,  // provide any, if available, supported internally for MsiProvideAssembly
#endif
	INSTALLMODE_NOSOURCERESOLUTION   = -3,  // skip source resolution
	INSTALLMODE_NODETECTION          = -2,  // skip detection
	INSTALLMODE_EXISTING             = -1,  // provide, if available
	INSTALLMODE_DEFAULT              =  0,  // install, if absent
} INSTALLMODE;

#if (_WIN32_MSI >=  300)
typedef enum tagMSIPATCHSTATE
{
	MSIPATCHSTATE_INVALID     =  0,  // broken
	MSIPATCHSTATE_APPLIED     =  1,  // applied patch
	MSIPATCHSTATE_SUPERSEDED  =  2,  // applied but superseded
	MSIPATCHSTATE_OBSOLETED   =  4,  // applied but obsoleted
	MSIPATCHSTATE_REGISTERED  =  8,   // registered only - Not applied
	MSIPATCHSTATE_ALL         =  (MSIPATCHSTATE_APPLIED | MSIPATCHSTATE_SUPERSEDED | MSIPATCHSTATE_OBSOLETED | MSIPATCHSTATE_REGISTERED)
} MSIPATCHSTATE;

typedef enum tagMSIINSTALLCONTEXT
{
        MSIINSTALLCONTEXT_FIRSTVISIBLE   =  0,  // product visible to the current user
        MSIINSTALLCONTEXT_NONE           =  0,  // Invalid context for a product
        MSIINSTALLCONTEXT_USERMANAGED    =  1,  // user managed install context
        MSIINSTALLCONTEXT_USERUNMANAGED  =  2,  // user non-managed context
        MSIINSTALLCONTEXT_MACHINE        =  4,  // per-machine context
        MSIINSTALLCONTEXT_ALL            =  (MSIINSTALLCONTEXT_USERMANAGED | MSIINSTALLCONTEXT_USERUNMANAGED | MSIINSTALLCONTEXT_MACHINE),	// All contexts. OR of all valid values
        MSIINSTALLCONTEXT_ALLUSERMANAGED =  8,  // all user-managed contexts
} MSIINSTALLCONTEXT;

typedef enum tagMSIPATCHDATATYPE
{
	MSIPATCH_DATATYPE_PATCHFILE = 0,
	MSIPATCH_DATATYPE_XMLPATH   = 1,
	MSIPATCH_DATATYPE_XMLBLOB   = 2,
} MSIPATCHDATATYPE, * PMSIPATCHDATATYPE;

typedef struct tagMSIPATCHSEQUENCEINFOA
{
	LPCSTR   szPatchData;
	MSIPATCHDATATYPE ePatchDataType;
	DWORD   dwOrder;
	UINT    uStatus;
} MSIPATCHSEQUENCEINFOA, * PMSIPATCHSEQUENCEINFOA;
typedef struct tagMSIPATCHSEQUENCEINFOW
{
	LPCWSTR  szPatchData;
	MSIPATCHDATATYPE ePatchDataType;
	DWORD   dwOrder;
	UINT    uStatus;
} MSIPATCHSEQUENCEINFOW, * PMSIPATCHSEQUENCEINFOW;
#ifdef UNICODE
typedef MSIPATCHSEQUENCEINFOW MSIPATCHSEQUENCEINFO;
typedef PMSIPATCHSEQUENCEINFOW PMSIPATCHSEQUENCEINFO;
#else
typedef MSIPATCHSEQUENCEINFOA MSIPATCHSEQUENCEINFO;
typedef PMSIPATCHSEQUENCEINFOA PMSIPATCHSEQUENCEINFO;
#endif // UNICODE

#endif //(_WIN32_MSI >=  300)



#define MAX_FEATURE_CHARS  38   // maximum chars in feature name (same as string GUID)


// Product info attributes: advertised information

#define INSTALLPROPERTY_PACKAGENAME           __TEXT("PackageName")
#define INSTALLPROPERTY_TRANSFORMS            __TEXT("Transforms")
#define INSTALLPROPERTY_LANGUAGE              __TEXT("Language")
#define INSTALLPROPERTY_PRODUCTNAME           __TEXT("ProductName")
#define INSTALLPROPERTY_ASSIGNMENTTYPE        __TEXT("AssignmentType")
#if (_WIN32_MSI >= 150)
#define INSTALLPROPERTY_INSTANCETYPE          __TEXT("InstanceType")
#endif //(_WIN32_MSI >= 150)
#if (_WIN32_MSI >= 300)
#define INSTALLPROPERTY_AUTHORIZED_LUA_APP    __TEXT("AuthorizedLUAApp")
#endif //(_WIN32_MSI >= 300)


#define INSTALLPROPERTY_PACKAGECODE           __TEXT("PackageCode")
#define INSTALLPROPERTY_VERSION               __TEXT("Version")
#if (_WIN32_MSI >=  110)
#define INSTALLPROPERTY_PRODUCTICON           __TEXT("ProductIcon")
#endif //(_WIN32_MSI >=  110)

// Product info attributes: installed information

#define INSTALLPROPERTY_INSTALLEDPRODUCTNAME  __TEXT("InstalledProductName")
#define INSTALLPROPERTY_VERSIONSTRING         __TEXT("VersionString")
#define INSTALLPROPERTY_HELPLINK              __TEXT("HelpLink")
#define INSTALLPROPERTY_HELPTELEPHONE         __TEXT("HelpTelephone")
#define INSTALLPROPERTY_INSTALLLOCATION       __TEXT("InstallLocation")
#define INSTALLPROPERTY_INSTALLSOURCE         __TEXT("InstallSource")
#define INSTALLPROPERTY_INSTALLDATE           __TEXT("InstallDate")
#define INSTALLPROPERTY_PUBLISHER             __TEXT("Publisher")
#define INSTALLPROPERTY_LOCALPACKAGE          __TEXT("LocalPackage")
#define INSTALLPROPERTY_URLINFOABOUT          __TEXT("URLInfoAbout")
#define INSTALLPROPERTY_URLUPDATEINFO         __TEXT("URLUpdateInfo")
#define INSTALLPROPERTY_VERSIONMINOR          __TEXT("VersionMinor")
#define INSTALLPROPERTY_VERSIONMAJOR          __TEXT("VersionMajor")
#define INSTALLPROPERTY_PRODUCTID             __TEXT("ProductID")
#define INSTALLPROPERTY_REGCOMPANY            __TEXT("RegCompany")
#define INSTALLPROPERTY_REGOWNER              __TEXT("RegOwner")
#if (_WIN32_MSI >= 500)
#define INSTALLPROPERTY_INSTALLEDLANGUAGE     __TEXT("InstalledLanguage")
#endif //(_WIN32_MSI >= 500)

#if (_WIN32_MSI >= 300)
#define	INSTALLPROPERTY_UNINSTALLABLE         __TEXT("Uninstallable")
#define	INSTALLPROPERTY_PRODUCTSTATE          __TEXT("State")
#define	INSTALLPROPERTY_PATCHSTATE            __TEXT("State")
#define INSTALLPROPERTY_PATCHTYPE             __TEXT("PatchType")
#define INSTALLPROPERTY_LUAENABLED            __TEXT("LUAEnabled")
#define INSTALLPROPERTY_DISPLAYNAME           __TEXT("DisplayName")
#define INSTALLPROPERTY_MOREINFOURL           __TEXT("MoreInfoURL")

// Source List Info attributes: Advertised information
#define INSTALLPROPERTY_LASTUSEDSOURCE        __TEXT("LastUsedSource")
#define INSTALLPROPERTY_LASTUSEDTYPE          __TEXT("LastUsedType")
#define INSTALLPROPERTY_MEDIAPACKAGEPATH      __TEXT("MediaPackagePath")
#define INSTALLPROPERTY_DISKPROMPT            __TEXT("DiskPrompt")
#endif // (_WIN32_MSI >= 300)

typedef enum tagSCRIPTFLAGS
{
	SCRIPTFLAGS_CACHEINFO                = 0x00000001L,   // set if the icons need to be created/ removed
	SCRIPTFLAGS_SHORTCUTS                = 0x00000004L,   // set if the shortcuts needs to be created/ deleted
	SCRIPTFLAGS_MACHINEASSIGN            = 0x00000008L,   // set if product to be assigned to machine
	SCRIPTFLAGS_REGDATA_CNFGINFO         = 0x00000020L,   // set if the product cnfg mgmt. registry data needs to be written/ removed
	SCRIPTFLAGS_VALIDATE_TRANSFORMS_LIST = 0x00000040L,
#if (_WIN32_MSI >=  110)
	SCRIPTFLAGS_REGDATA_CLASSINFO        = 0x00000080L,   // set if COM classes related app info needs to be  created/ deleted
	SCRIPTFLAGS_REGDATA_EXTENSIONINFO    = 0x00000100L,   // set if extension related app info needs to be  created/ deleted
	SCRIPTFLAGS_REGDATA_APPINFO          = SCRIPTFLAGS_REGDATA_CLASSINFO | SCRIPTFLAGS_REGDATA_EXTENSIONINFO,  // for source level backward compatibility
#else //_WIN32_MSI == 100
	SCRIPTFLAGS_REGDATA_APPINFO          = 0x00000010L,
#endif //(_WIN32_MSI >=  110)
	SCRIPTFLAGS_REGDATA                  = SCRIPTFLAGS_REGDATA_APPINFO | SCRIPTFLAGS_REGDATA_CNFGINFO, // for source level backward compatibility
}SCRIPTFLAGS;


typedef enum tagADVERTISEFLAGS
{
	ADVERTISEFLAGS_MACHINEASSIGN   =    0,   // set if the product is to be machine assigned
	ADVERTISEFLAGS_USERASSIGN      =    1,   // set if the product is to be user assigned
}ADVERTISEFLAGS;

typedef enum tagINSTALLTYPE
{
	INSTALLTYPE_DEFAULT            =    0,   // set to indicate default behavior
	INSTALLTYPE_NETWORK_IMAGE      =    1,   // set to indicate network install
	INSTALLTYPE_SINGLE_INSTANCE    =    2,   // set to indicate a particular instance 
}INSTALLTYPE;

#if (_WIN32_MSI >=  150)

typedef struct _MSIFILEHASHINFO {
	ULONG dwFileHashInfoSize;
	ULONG dwData [ 4 ];
} MSIFILEHASHINFO, *PMSIFILEHASHINFO;

typedef enum tagMSIARCHITECTUREFLAGS
{
	MSIARCHITECTUREFLAGS_X86   = 0x00000001L, // set if creating the script for i386 platform
	MSIARCHITECTUREFLAGS_IA64  = 0x00000002L, // set if creating the script for IA64 platform
	MSIARCHITECTUREFLAGS_AMD64 = 0x00000004L, // set if creating the script for AMD64 platform
	MSIARCHITECTUREFLAGS_ARM   = 0x00000008L //set if creating the script for ARM platform
}MSIARCHITECTUREFLAGS;

typedef enum tagMSIOPENPACKAGEFLAGS
{
	MSIOPENPACKAGEFLAGS_IGNOREMACHINESTATE = 0x00000001L, // ignore the machine state when creating the engine
}MSIOPENPACKAGEFLAGS;

typedef enum tagMSIADVERTISEOPTIONFLAGS
{
	MSIADVERTISEOPTIONFLAGS_INSTANCE = 0x00000001L, // set if advertising a new instance
}MSIADVERTISEOPTIONFLAGS;

#endif //(_WIN32_MSI >=  150)


#if (_WIN32_MSI >= 300)

typedef enum tagMSISOURCETYPE
{
	MSISOURCETYPE_UNKNOWN = 0x00000000L,
	MSISOURCETYPE_NETWORK = 0x00000001L, // network source
	MSISOURCETYPE_URL     = 0x00000002L, // URL source
	MSISOURCETYPE_MEDIA   = 0x00000004L  // media source
}MSISOURCETYPE;

typedef enum tagMSICODE
{
	MSICODE_PRODUCT = 0x00000000L, // product code provided
	MSICODE_PATCH   = 0x40000000L  // patch code provided
}MSICODE;

#if (_WIN32_MSI >= 450)

typedef enum tagMSITRANSACTION
{
	MSITRANSACTION_CHAIN_EMBEDDEDUI = 0x00000001L,
	MSITRANSACTION_JOIN_EXISTING_EMBEDDEDUI = 0x00000002L,
}MSITRANSACTION;

typedef enum tagMSITRANSACTIONSTATE
{
	MSITRANSACTIONSTATE_ROLLBACK = 0x00000000L,
	MSITRANSACTIONSTATE_COMMIT   = 0x00000001L,

}MSITRANSACTIONSTATE;

#endif //(_WIN32_MSI >= 450)


#endif //(_WIN32_MSI >= 300)


#ifdef __cplusplus
extern "C" {
#endif


// --------------------------------------------------------------------------
// Functions to set the UI handling and logging. The UI will be used for error,
// progress, and log messages for all subsequent calls to Installer Service
// API functions that require UI.
// --------------------------------------------------------------------------

// Enable internal UI

INSTALLUILEVEL WINAPI MsiSetInternalUI(
	_In_ INSTALLUILEVEL  dwUILevel,     // UI level
	_Inout_opt_ HWND  *phWnd);          // handle of owner window

// Enable external UI handling, returns any previous handler or NULL if none.
// Messages are designated with a combination of bits from INSTALLLOGMODE enum.

INSTALLUI_HANDLERA WINAPI MsiSetExternalUIA(
	_In_opt_ INSTALLUI_HANDLERA puiHandler,  // for progress and error handling 
	_In_ DWORD              dwMessageFilter, // bit flags designating messages to handle
	_In_opt_ LPVOID             pvContext);  // application context
INSTALLUI_HANDLERW WINAPI MsiSetExternalUIW(
	_In_opt_ INSTALLUI_HANDLERW puiHandler,  // for progress and error handling 
	_In_ DWORD              dwMessageFilter, // bit flags designating messages to handle
	_In_opt_ LPVOID             pvContext);  // application context
#ifdef UNICODE
#define MsiSetExternalUI  MsiSetExternalUIW
#else
#define MsiSetExternalUI  MsiSetExternalUIA
#endif // !UNICODE

#if (_WIN32_MSI >= 310)

UINT WINAPI MsiSetExternalUIRecord(
	_In_opt_ INSTALLUI_HANDLER_RECORD puiHandler,         // for progress and error handling 
	_In_ DWORD              dwMessageFilter,              // bit flags designating messages to handle
	_In_opt_ LPVOID             pvContext,                // application context
	_Out_opt_ PINSTALLUI_HANDLER_RECORD ppuiPrevHandler); // pointer to the previously set handler
	
#endif // (_WIN32_MSI >= 310)

// Enable logging to a file for all install sessions for the client process,
// with control over which log messages are passed to the specified log file.
// Messages are designated with a combination of bits from INSTALLLOGMODE enum.

UINT WINAPI MsiEnableLogA(
	_In_ DWORD     dwLogMode,           // bit flags designating operations to report
	_In_opt_ LPCSTR  szLogFile,       // log file, or NULL to disable logging
	_In_ DWORD     dwLogAttributes);    // INSTALLLOGATTRIBUTES flags
UINT WINAPI MsiEnableLogW(
	_In_ DWORD     dwLogMode,           // bit flags designating operations to report
	_In_opt_ LPCWSTR  szLogFile,       // log file, or NULL to disable logging
	_In_ DWORD     dwLogAttributes);    // INSTALLLOGATTRIBUTES flags
#ifdef UNICODE
#define MsiEnableLog  MsiEnableLogW
#else
#define MsiEnableLog  MsiEnableLogA
#endif // !UNICODE

// --------------------------------------------------------------------------
// Functions to query and configure a product as a whole.
// --------------------------------------------------------------------------

// Return the installed state for a product

INSTALLSTATE WINAPI MsiQueryProductStateA(
	_In_ LPCSTR  szProduct);
INSTALLSTATE WINAPI MsiQueryProductStateW(
	_In_ LPCWSTR  szProduct);
#ifdef UNICODE
#define MsiQueryProductState  MsiQueryProductStateW
#else
#define MsiQueryProductState  MsiQueryProductStateA
#endif // !UNICODE

// Return product info

UINT WINAPI MsiGetProductInfoA(
	_In_ LPCSTR   szProduct,                              // product code
	_In_ LPCSTR   szAttribute,                            // attribute name, case-sensitive
	_Out_writes_opt_(*pcchValueBuf) LPSTR lpValueBuf,     // returned value, NULL if not desired
	_Inout_opt_                     LPDWORD pcchValueBuf);  // in/out buffer character count
UINT WINAPI MsiGetProductInfoW(
	_In_ LPCWSTR   szProduct,                              // product code
	_In_ LPCWSTR   szAttribute,                            // attribute name, case-sensitive
	_Out_writes_opt_(*pcchValueBuf) LPWSTR lpValueBuf,     // returned value, NULL if not desired
	_Inout_opt_                     LPDWORD pcchValueBuf);  // in/out buffer character count
#ifdef UNICODE
#define MsiGetProductInfo  MsiGetProductInfoW
#else
#define MsiGetProductInfo  MsiGetProductInfoA
#endif // !UNICODE

#if (_WIN32_MSI >= 300)
// Retrieve product info for a particular product instance

UINT WINAPI MsiGetProductInfoExA(
		_In_ LPCSTR szProductCode,                  // Information is queried on this product
		_In_opt_ LPCSTR szUserSid,                  // Account of this product instance
		_In_ MSIINSTALLCONTEXT dwContext,             // Context of this product instance
		_In_ LPCSTR szProperty,                     // Property being queried
		_Out_writes_opt_(*pcchValue) LPSTR szValue, // Value read
		_Inout_opt_ LPDWORD pcchValue);               // size of the szValue buffer
UINT WINAPI MsiGetProductInfoExW(
		_In_ LPCWSTR szProductCode,                  // Information is queried on this product
		_In_opt_ LPCWSTR szUserSid,                  // Account of this product instance
		_In_ MSIINSTALLCONTEXT dwContext,             // Context of this product instance
		_In_ LPCWSTR szProperty,                     // Property being queried
		_Out_writes_opt_(*pcchValue) LPWSTR szValue, // Value read
		_Inout_opt_ LPDWORD pcchValue);               // size of the szValue buffer
#ifdef UNICODE
#define MsiGetProductInfoEx  MsiGetProductInfoExW
#else
#define MsiGetProductInfoEx  MsiGetProductInfoExA
#endif // !UNICODE
#endif //(_WIN32_MSI >= 300)

// Install a new product.
// Either may be NULL, but the DATABASE property must be specfied

UINT WINAPI MsiInstallProductA(
	_In_ LPCSTR      szPackagePath,        // location of package to install
	_In_opt_ LPCSTR      szCommandLine);   // command line <property settings>
UINT WINAPI MsiInstallProductW(
	_In_ LPCWSTR      szPackagePath,        // location of package to install
	_In_opt_ LPCWSTR      szCommandLine);   // command line <property settings>
#ifdef UNICODE
#define MsiInstallProduct  MsiInstallProductW
#else
#define MsiInstallProduct  MsiInstallProductA
#endif // !UNICODE

// Install/uninstall an advertised or installed product
// No action if installed and INSTALLSTATE_DEFAULT specified

UINT WINAPI MsiConfigureProductA(
	_In_ LPCSTR      szProduct,       // product code
	_In_ int          iInstallLevel,    // how much of the product to install
	_In_ INSTALLSTATE eInstallState);   // local/source/default/absent/lock/uncache
UINT WINAPI MsiConfigureProductW(
	_In_ LPCWSTR      szProduct,       // product code
	_In_ int          iInstallLevel,    // how much of the product to install
	_In_ INSTALLSTATE eInstallState);   // local/source/default/absent/lock/uncache
#ifdef UNICODE
#define MsiConfigureProduct  MsiConfigureProductW
#else
#define MsiConfigureProduct  MsiConfigureProductA
#endif // !UNICODE

// Install/uninstall an advertised or installed product
// No action if installed and INSTALLSTATE_DEFAULT specified

UINT WINAPI MsiConfigureProductExA(
	_In_ LPCSTR      szProduct,            // product code
	_In_ int          iInstallLevel,         // how much of the product to install
	_In_ INSTALLSTATE eInstallState,         // local/source/default/absent/lock/uncache
	_In_opt_ LPCSTR      szCommandLine);   // command line <property settings>
UINT WINAPI MsiConfigureProductExW(
	_In_ LPCWSTR      szProduct,            // product code
	_In_ int          iInstallLevel,         // how much of the product to install
	_In_ INSTALLSTATE eInstallState,         // local/source/default/absent/lock/uncache
	_In_opt_ LPCWSTR      szCommandLine);   // command line <property settings>
#ifdef UNICODE
#define MsiConfigureProductEx  MsiConfigureProductExW
#else
#define MsiConfigureProductEx  MsiConfigureProductExA
#endif // !UNICODE

// Reinstall product, used to validate or correct problems

UINT WINAPI MsiReinstallProductA(
	_In_ LPCSTR      szProduct,        // product code
	_In_ DWORD         szReinstallMode); // one or more REINSTALLMODE modes
UINT WINAPI MsiReinstallProductW(
	_In_ LPCWSTR      szProduct,        // product code
	_In_ DWORD         szReinstallMode); // one or more REINSTALLMODE modes
#ifdef UNICODE
#define MsiReinstallProduct  MsiReinstallProductW
#else
#define MsiReinstallProduct  MsiReinstallProductA
#endif // !UNICODE

#if (_WIN32_MSI >=  150)

// Output reg and shortcut info to script file for specified architecture for Assign or Publish
// If dwPlatform is 0, then the script is created based on the current platform (behavior of MsiAdvertiseProduct)
// If dwPlatform specifies a platform, then the script is created as if the current platform is the
//    platform specified in dwPlatform 
// If dwOptions includes MSIADVERTISEOPTIONFLAGS_INSTANCE, then a new instance is advertised. Use of
//    this option requires that szTransforms include the instance transform that changes the product code

UINT WINAPI MsiAdvertiseProductExA(
	_In_ LPCSTR	szPackagePath,      // location of package
	_In_opt_ LPCSTR    szScriptfilePath,   // if NULL, product is locally advertised
	_In_opt_ LPCSTR    szTransforms,       // list of transforms to be applied
	_In_ LANGID      lgidLanguage,       // install language
	_In_ DWORD       dwPlatform,         // the MSIARCHITECTUREFLAGS that control for which platform
	                                //   to create the script, ignored if szScriptfilePath is NULL
	_In_ DWORD       dwOptions);         // the MSIADVERTISEOPTIONFLAGS that specify extra advertise parameters
UINT WINAPI MsiAdvertiseProductExW(
	_In_ LPCWSTR	szPackagePath,      // location of package
	_In_opt_ LPCWSTR    szScriptfilePath,   // if NULL, product is locally advertised
	_In_opt_ LPCWSTR    szTransforms,       // list of transforms to be applied
	_In_ LANGID      lgidLanguage,       // install language
	_In_ DWORD       dwPlatform,         // the MSIARCHITECTUREFLAGS that control for which platform
	                                //   to create the script, ignored if szScriptfilePath is NULL
	_In_ DWORD       dwOptions);         // the MSIADVERTISEOPTIONFLAGS that specify extra advertise parameters
#ifdef UNICODE
#define MsiAdvertiseProductEx  MsiAdvertiseProductExW
#else
#define MsiAdvertiseProductEx  MsiAdvertiseProductExA
#endif // !UNICODE

#endif // (_WIN32_MSI >= 150)

// Output reg and shortcut info to script file for Assign or Publish

UINT WINAPI MsiAdvertiseProductA(
	_In_ LPCSTR      szPackagePath,    // location of package
	_In_opt_ LPCSTR      szScriptfilePath,  // if NULL, product is locally advertised
	_In_opt_ LPCSTR      szTransforms,      // list of transforms to be applied
	_In_ LANGID        lgidLanguage);     // install language
UINT WINAPI MsiAdvertiseProductW(
	_In_ LPCWSTR      szPackagePath,    // location of package
	_In_opt_ LPCWSTR      szScriptfilePath,  // if NULL, product is locally advertised
	_In_opt_ LPCWSTR      szTransforms,      // list of transforms to be applied
	_In_ LANGID        lgidLanguage);     // install language
#ifdef UNICODE
#define MsiAdvertiseProduct  MsiAdvertiseProductW
#else
#define MsiAdvertiseProduct  MsiAdvertiseProductA
#endif // !UNICODE


#if (_WIN32_MSI >=  150)

// Process advertise script file into supplied locations
// If an icon folder is specified, icon files will be placed there
// If an registry key is specified, registry data will be mapped under it
// If fShortcuts is TRUE, shortcuts will be created. If a special folder is
//    returned by SHGetSpecialFolderLocation(?), it will hold the shortcuts.
// if fRemoveItems is TRUE, items that are present will be removed

UINT WINAPI MsiProcessAdvertiseScriptA(
	_In_ LPCSTR      szScriptFile,  // path to script from MsiAdvertiseProduct
	_In_opt_ LPCSTR      szIconFolder,  // optional path to folder for icon files and transforms
	_In_opt_ HKEY         hRegData,      // optional parent registry key
	_In_ BOOL         fShortcuts,    // TRUE if shortcuts output to special folder
	_In_ BOOL         fRemoveItems); // TRUE if specified items are to be removed
UINT WINAPI MsiProcessAdvertiseScriptW(
	_In_ LPCWSTR      szScriptFile,  // path to script from MsiAdvertiseProduct
	_In_opt_ LPCWSTR      szIconFolder,  // optional path to folder for icon files and transforms
	_In_opt_ HKEY         hRegData,      // optional parent registry key
	_In_ BOOL         fShortcuts,    // TRUE if shortcuts output to special folder
	_In_ BOOL         fRemoveItems); // TRUE if specified items are to be removed
#ifdef UNICODE
#define MsiProcessAdvertiseScript  MsiProcessAdvertiseScriptW
#else
#define MsiProcessAdvertiseScript  MsiProcessAdvertiseScriptA
#endif // !UNICODE

#endif // (_WIN32_MSI >= 150)


// Process advertise script file using the supplied dwFlags control flags
// if fRemoveItems is TRUE, items that are present will be removed

UINT WINAPI MsiAdvertiseScriptA(
	_In_ LPCSTR      szScriptFile,  // path to script from MsiAdvertiseProduct
	_In_ DWORD         dwFlags,       // the SCRIPTFLAGS bit flags that control the script execution
	_In_opt_ PHKEY         phRegData,     // optional parent registry key
	_In_ BOOL          fRemoveItems); // TRUE if specified items are to be removed
UINT WINAPI MsiAdvertiseScriptW(
	_In_ LPCWSTR      szScriptFile,  // path to script from MsiAdvertiseProduct
	_In_ DWORD         dwFlags,       // the SCRIPTFLAGS bit flags that control the script execution
	_In_opt_ PHKEY         phRegData,     // optional parent registry key
	_In_ BOOL          fRemoveItems); // TRUE if specified items are to be removed
#ifdef UNICODE
#define MsiAdvertiseScript  MsiAdvertiseScriptW
#else
#define MsiAdvertiseScript  MsiAdvertiseScriptA
#endif // !UNICODE

// Return product info from an installer script file:
//   product code, language, version, readable name, path to package
// Returns TRUE is success, FALSE if szScriptFile is not a valid script file

UINT WINAPI MsiGetProductInfoFromScriptA(
	_In_                               LPCSTR  szScriptFile,    // path to installer script file
	_Out_writes_opt_(MAX_GUID_CHARS+1) LPSTR   lpProductBuf39,  // buffer for product code string GUID, 39 chars
	_Out_opt_                          LANGID   *plgidLanguage,  // return language Id
	_Out_opt_                          LPDWORD   pdwVersion,     // return version: Maj:Min:Build <8:8:16>
	_Out_writes_opt_(*pcchNameBuf)     LPSTR   lpNameBuf,       // buffer to return readable product name
	_Inout_opt_                        LPDWORD   pcchNameBuf,    // in/out name buffer character count
	_Out_writes_opt_(*pcchPackageBuf)  LPSTR   lpPackageBuf,   // buffer for path to product package
	_Inout_opt_                        LPDWORD   pcchPackageBuf);// in/out path buffer character count
UINT WINAPI MsiGetProductInfoFromScriptW(
	_In_                               LPCWSTR  szScriptFile,    // path to installer script file
	_Out_writes_opt_(MAX_GUID_CHARS+1) LPWSTR   lpProductBuf39,  // buffer for product code string GUID, 39 chars
	_Out_opt_                          LANGID   *plgidLanguage,  // return language Id
	_Out_opt_                          LPDWORD   pdwVersion,     // return version: Maj:Min:Build <8:8:16>
	_Out_writes_opt_(*pcchNameBuf)     LPWSTR   lpNameBuf,       // buffer to return readable product name
	_Inout_opt_                        LPDWORD   pcchNameBuf,    // in/out name buffer character count
	_Out_writes_opt_(*pcchPackageBuf)  LPWSTR   lpPackageBuf,   // buffer for path to product package
	_Inout_opt_                        LPDWORD   pcchPackageBuf);// in/out path buffer character count
#ifdef UNICODE
#define MsiGetProductInfoFromScript  MsiGetProductInfoFromScriptW
#else
#define MsiGetProductInfoFromScript  MsiGetProductInfoFromScriptA
#endif // !UNICODE

// Return the product code for a registered component, called once by apps

UINT WINAPI MsiGetProductCodeA(
	_In_ LPCSTR   szComponent,   // component Id registered for this product
	_Out_writes_(MAX_GUID_CHARS+1) LPSTR lpBuf39);   // returned string GUID, sized for 39 characters
UINT WINAPI MsiGetProductCodeW(
	_In_ LPCWSTR   szComponent,   // component Id registered for this product
	_Out_writes_(MAX_GUID_CHARS+1) LPWSTR lpBuf39);   // returned string GUID, sized for 39 characters
#ifdef UNICODE
#define MsiGetProductCode  MsiGetProductCodeW
#else
#define MsiGetProductCode  MsiGetProductCodeA
#endif // !UNICODE

// Return the registered user information for an installed product

USERINFOSTATE WINAPI MsiGetUserInfoA(
	_In_                                   LPCSTR  szProduct,    // product code, string GUID
	_Out_writes_opt_(*pcchUserNameBuf) LPSTR   lpUserNameBuf,    // return user name           
	_Inout_opt_                        LPDWORD   pcchUserNameBuf, // in/out buffer character count
	_Out_writes_opt_(*pcchOrgNameBuf)  LPSTR   lpOrgNameBuf,     // return company name           
	_Inout_opt_                        LPDWORD   pcchOrgNameBuf,  // in/out buffer character count
	_Out_writes_opt_(*pcchSerialBuf)   LPSTR   lpSerialBuf,      // return product serial number
	_Inout_opt_                        LPDWORD   pcchSerialBuf);  // in/out buffer character count
USERINFOSTATE WINAPI MsiGetUserInfoW(
	_In_                                   LPCWSTR  szProduct,    // product code, string GUID
	_Out_writes_opt_(*pcchUserNameBuf) LPWSTR   lpUserNameBuf,    // return user name           
	_Inout_opt_                        LPDWORD   pcchUserNameBuf, // in/out buffer character count
	_Out_writes_opt_(*pcchOrgNameBuf)  LPWSTR   lpOrgNameBuf,     // return company name           
	_Inout_opt_                        LPDWORD   pcchOrgNameBuf,  // in/out buffer character count
	_Out_writes_opt_(*pcchSerialBuf)   LPWSTR   lpSerialBuf,      // return product serial number
	_Inout_opt_                        LPDWORD   pcchSerialBuf);  // in/out buffer character count
#ifdef UNICODE
#define MsiGetUserInfo  MsiGetUserInfoW
#else
#define MsiGetUserInfo  MsiGetUserInfoA
#endif // !UNICODE

// Obtain and store user info and PID from installation wizard (first run)

UINT WINAPI MsiCollectUserInfoA(
	_In_ LPCSTR  szProduct);     // product code, string GUID
UINT WINAPI MsiCollectUserInfoW(
	_In_ LPCWSTR  szProduct);     // product code, string GUID
#ifdef UNICODE
#define MsiCollectUserInfo  MsiCollectUserInfoW
#else
#define MsiCollectUserInfo  MsiCollectUserInfoA
#endif // !UNICODE

// --------------------------------------------------------------------------
// Functions to query, apply, and remove patches to a product.
// --------------------------------------------------------------------------

// Patch all possible installed products.

UINT WINAPI MsiApplyPatchA(
	_In_ LPCSTR szPatchPackage,       // location of patch package
	_In_opt_ LPCSTR szInstallPackage, // location of package for install to patch <optional>
	_In_ INSTALLTYPE eInstallType,      // type of install to patch
	_In_opt_ LPCSTR szCommandLine);   // command line <property settings>
UINT WINAPI MsiApplyPatchW(
	_In_ LPCWSTR szPatchPackage,       // location of patch package
	_In_opt_ LPCWSTR szInstallPackage, // location of package for install to patch <optional>
	_In_ INSTALLTYPE eInstallType,      // type of install to patch
	_In_opt_ LPCWSTR szCommandLine);   // command line <property settings>
#ifdef UNICODE
#define MsiApplyPatch  MsiApplyPatchW
#else
#define MsiApplyPatch  MsiApplyPatchA
#endif // !UNICODE

// Return patch info for first visible patch

UINT WINAPI MsiGetPatchInfoA(
	_In_ LPCSTR szPatch,                              // patch code
	_In_ LPCSTR szAttribute,                          // attribute name, case-sensitive
	_Out_writes_opt_(*pcchValueBuf) LPSTR lpValueBuf, // returned value, NULL if not desired
	_Inout_opt_ LPDWORD pcchValueBuf);                  // in/out buffer character count
UINT WINAPI MsiGetPatchInfoW(
	_In_ LPCWSTR szPatch,                              // patch code
	_In_ LPCWSTR szAttribute,                          // attribute name, case-sensitive
	_Out_writes_opt_(*pcchValueBuf) LPWSTR lpValueBuf, // returned value, NULL if not desired
	_Inout_opt_ LPDWORD pcchValueBuf);                  // in/out buffer character count
#ifdef UNICODE
#define MsiGetPatchInfo  MsiGetPatchInfoW
#else
#define MsiGetPatchInfo  MsiGetPatchInfoA
#endif // !UNICODE

// Enumerate all patches for a product in contexts visible to caller

UINT WINAPI MsiEnumPatchesA(
	_In_ LPCSTR szProduct,
	_In_ DWORD iPatchIndex,
	_Out_writes_(MAX_GUID_CHARS+1)   LPSTR  lpPatchBuf,
	_Out_writes_(*pcchTransformsBuf) LPSTR  lpTransformsBuf,
	_Inout_ LPDWORD  pcchTransformsBuf);
UINT WINAPI MsiEnumPatchesW(
	_In_ LPCWSTR szProduct,
	_In_ DWORD iPatchIndex,
	_Out_writes_(MAX_GUID_CHARS+1)   LPWSTR  lpPatchBuf,
	_Out_writes_(*pcchTransformsBuf) LPWSTR  lpTransformsBuf,
	_Inout_ LPDWORD  pcchTransformsBuf);
#ifdef UNICODE
#define MsiEnumPatches  MsiEnumPatchesW
#else
#define MsiEnumPatches  MsiEnumPatchesA
#endif // !UNICODE
	
#if (_WIN32_MSI >= 300)
// Remove one or more patches from the specified product.

UINT WINAPI MsiRemovePatchesA(
	_In_ LPCSTR szPatchList,         // semi-colon delimited list of patches to remove; patch can be referenced by patch package path or Patch GUID
	_In_ LPCSTR szProductCode,       // ProductCode GUID of product with patch to remove
	_In_ INSTALLTYPE eUninstallType,   // type of patch uninstall to perform. Must be INSTALLTYPE_SINGLE_INSTANCE.
	_In_opt_ LPCSTR szPropertyList); // command line property settings using PROPERTY=VALUE pairs
UINT WINAPI MsiRemovePatchesW(
	_In_ LPCWSTR szPatchList,         // semi-colon delimited list of patches to remove; patch can be referenced by patch package path or Patch GUID
	_In_ LPCWSTR szProductCode,       // ProductCode GUID of product with patch to remove
	_In_ INSTALLTYPE eUninstallType,   // type of patch uninstall to perform. Must be INSTALLTYPE_SINGLE_INSTANCE.
	_In_opt_ LPCWSTR szPropertyList); // command line property settings using PROPERTY=VALUE pairs
#ifdef UNICODE
#define MsiRemovePatches  MsiRemovePatchesW
#else
#define MsiRemovePatches  MsiRemovePatchesA
#endif // !UNICODE

// Extract XML data from the patch

UINT WINAPI MsiExtractPatchXMLDataA(
	_In_ LPCSTR szPatchPath,                         // Patch file to open
	_Reserved_ DWORD dwReserved,                       // Reserved
	_Out_writes_opt_(*pcchXMLData) LPSTR szXMLData,  // Buffer that gets the XML data
	_Inout_opt_ LPDWORD pcchXMLData);                  // in/out XML data buffer character count
UINT WINAPI MsiExtractPatchXMLDataW(
	_In_ LPCWSTR szPatchPath,                         // Patch file to open
	_Reserved_ DWORD dwReserved,                       // Reserved
	_Out_writes_opt_(*pcchXMLData) LPWSTR szXMLData,  // Buffer that gets the XML data
	_Inout_opt_ LPDWORD pcchXMLData);                  // in/out XML data buffer character count
#ifdef UNICODE
#define MsiExtractPatchXMLData  MsiExtractPatchXMLDataW
#else
#define MsiExtractPatchXMLData  MsiExtractPatchXMLDataA
#endif // !UNICODE

// Retrieve extended patch info for a particular patch applied
// to a particular product instance.

UINT WINAPI MsiGetPatchInfoExA(
        _In_ LPCSTR szPatchCode,                    // target patch to query
        _In_ LPCSTR szProductCode,                  // target product of patch application
        _In_opt_ LPCSTR szUserSid,                  // Account of this product instance
        _In_ MSIINSTALLCONTEXT dwContext,             // context to query for product and patch
        _In_ LPCSTR szProperty,                     // property of patch to retrieve
        _Out_writes_opt_(*pcchValue) LPSTR lpValue, // address buffer for data
        _Inout_opt_ LPDWORD pcchValue);               // in/out value buffer character count
UINT WINAPI MsiGetPatchInfoExW(
        _In_ LPCWSTR szPatchCode,                    // target patch to query
        _In_ LPCWSTR szProductCode,                  // target product of patch application
        _In_opt_ LPCWSTR szUserSid,                  // Account of this product instance
        _In_ MSIINSTALLCONTEXT dwContext,             // context to query for product and patch
        _In_ LPCWSTR szProperty,                     // property of patch to retrieve
        _Out_writes_opt_(*pcchValue) LPWSTR lpValue, // address buffer for data
        _Inout_opt_ LPDWORD pcchValue);               // in/out value buffer character count
#ifdef UNICODE
#define MsiGetPatchInfoEx  MsiGetPatchInfoExW
#else
#define MsiGetPatchInfoEx  MsiGetPatchInfoExA
#endif // !UNICODE

// Apply multiple patches to the specified product or to all eligible products
// on the machine

UINT WINAPI MsiApplyMultiplePatchesA(
        _In_ LPCSTR szPatchPackages,       // Patches to apply
        _In_opt_ LPCSTR szProductCode,     // target product of patch application
        _In_opt_ LPCSTR szPropertiesList); // Properties settings
UINT WINAPI MsiApplyMultiplePatchesW(
        _In_ LPCWSTR szPatchPackages,       // Patches to apply
        _In_opt_ LPCWSTR szProductCode,     // target product of patch application
        _In_opt_ LPCWSTR szPropertiesList); // Properties settings
#ifdef UNICODE
#define MsiApplyMultiplePatches  MsiApplyMultiplePatchesW
#else
#define MsiApplyMultiplePatches  MsiApplyMultiplePatchesA
#endif // !UNICODE

// Determines which patches apply and in what sequence for the provided set of
// patch files, XML files, and/or XML blobs. Patches already applied to the product
// are taken into account

UINT WINAPI MsiDeterminePatchSequenceA(
	_In_ LPCSTR szProductCode,                                   // Product code GUID of an installed product
	_In_opt_ LPCSTR szUserSid,                                   // User account we're interested in
	_In_ MSIINSTALLCONTEXT dwContext,                              // Installation context we're interested in
	_In_ DWORD cPatchInfo,                                         // Number of patches in the array
	_Inout_updates_(cPatchInfo) PMSIPATCHSEQUENCEINFOA pPatchInfo); // Array of patch sequence information data
UINT WINAPI MsiDeterminePatchSequenceW(
	_In_ LPCWSTR szProductCode,                                   // Product code GUID of an installed product
	_In_opt_ LPCWSTR szUserSid,                                   // User account we're interested in
	_In_ MSIINSTALLCONTEXT dwContext,                              // Installation context we're interested in
	_In_ DWORD cPatchInfo,                                         // Number of patches in the array
	_Inout_updates_(cPatchInfo) PMSIPATCHSEQUENCEINFOW pPatchInfo); // Array of patch sequence information data
#ifdef UNICODE
#define MsiDeterminePatchSequence  MsiDeterminePatchSequenceW
#else
#define MsiDeterminePatchSequence  MsiDeterminePatchSequenceA
#endif // !UNICODE

// Determines which patches apply and what sequence for the provided set of patch files,
// XML files, and/or XML blobs. Does not take into account either products or patches
// already installed on the machine.

UINT WINAPI MsiDetermineApplicablePatchesA(
	_In_ LPCSTR szProductPackagePath,                            // Fully qualified path to a product MSI database
	_In_ DWORD cPatchInfo,                                         // Number of patches in the array
	_Inout_updates_(cPatchInfo) PMSIPATCHSEQUENCEINFOA pPatchInfo); // Array of patch sequence information data
UINT WINAPI MsiDetermineApplicablePatchesW(
	_In_ LPCWSTR szProductPackagePath,                            // Fully qualified path to a product MSI database
	_In_ DWORD cPatchInfo,                                         // Number of patches in the array
	_Inout_updates_(cPatchInfo) PMSIPATCHSEQUENCEINFOW pPatchInfo); // Array of patch sequence information data
#ifdef UNICODE
#define MsiDetermineApplicablePatches  MsiDetermineApplicablePatchesW
#else
#define MsiDetermineApplicablePatches  MsiDetermineApplicablePatchesA
#endif // !UNICODE

// Enumerates all patches applied for a single product or all products based upon
// enumeration context and filter.

UINT WINAPI MsiEnumPatchesExA(
	_In_opt_ LPCSTR szProductCode,                                   // Enumerate patches on instances of this product
	_In_opt_ LPCSTR szUserSid,                                       // Account for enumeration
	_In_ DWORD dwContext,                                              // Contexts for enumeration
	_In_ DWORD dwFilter,                                               // Filter for enumeration
	_In_ DWORD dwIndex,                                                // Index for enumeration
	_Out_writes_opt_(MAX_GUID_CHARS+1) CHAR szPatchCode[39],         // Enumerated patch code
	_Out_writes_opt_(MAX_GUID_CHARS+1) CHAR szTargetProductCode[39], // Enumerated patch's product code
	_Out_opt_ MSIINSTALLCONTEXT *pdwTargetProductContext,              //Enumerated patch's context
	_Out_writes_opt_(*pcchTargetUserSid) LPSTR  szTargetUserSid,     // Enumerated patch's user account
	_Inout_opt_ LPDWORD pcchTargetUserSid);                            // in/out character count of szTargetUserSid
UINT WINAPI MsiEnumPatchesExW(
	_In_opt_ LPCWSTR szProductCode,                                   // Enumerate patches on instances of this product
	_In_opt_ LPCWSTR szUserSid,                                       // Account for enumeration
	_In_ DWORD dwContext,                                              // Contexts for enumeration
	_In_ DWORD dwFilter,                                               // Filter for enumeration
	_In_ DWORD dwIndex,                                                // Index for enumeration
	_Out_writes_opt_(MAX_GUID_CHARS+1) WCHAR szPatchCode[39],         // Enumerated patch code
	_Out_writes_opt_(MAX_GUID_CHARS+1) WCHAR szTargetProductCode[39], // Enumerated patch's product code
	_Out_opt_ MSIINSTALLCONTEXT *pdwTargetProductContext,              //Enumerated patch's context
	_Out_writes_opt_(*pcchTargetUserSid) LPWSTR  szTargetUserSid,     // Enumerated patch's user account
	_Inout_opt_ LPDWORD pcchTargetUserSid);                            // in/out character count of szTargetUserSid
#ifdef UNICODE
#define MsiEnumPatchesEx  MsiEnumPatchesExW
#else
#define MsiEnumPatchesEx  MsiEnumPatchesExA
#endif // !UNICODE

#endif //(_WIN32_MSI >= 300)

// --------------------------------------------------------------------------
// Functions to query and configure a feature within a product.
// --------------------------------------------------------------------------

// Return the installed state for a product feature

INSTALLSTATE WINAPI MsiQueryFeatureStateA(
	_In_ LPCSTR  szProduct,
	_In_ LPCSTR  szFeature);
INSTALLSTATE WINAPI MsiQueryFeatureStateW(
	_In_ LPCWSTR  szProduct,
	_In_ LPCWSTR  szFeature);
#ifdef UNICODE
#define MsiQueryFeatureState  MsiQueryFeatureStateW
#else
#define MsiQueryFeatureState  MsiQueryFeatureStateA
#endif // !UNICODE

#if (_WIN32_MSI >= 300)	
// Retrieve the installed state for a product feature in a particular product instance

UINT WINAPI MsiQueryFeatureStateExA(
		_In_ LPCSTR szProductCode,        // Information is queried on this product
		_In_opt_ LPCSTR szUserSid,        // Account of this product instance
		_In_ MSIINSTALLCONTEXT dwContext,   // Context of this product instance
		_In_ LPCSTR szFeature,            // Name of the feature being queried
		_Out_opt_ INSTALLSTATE *pdwState);  // State value
UINT WINAPI MsiQueryFeatureStateExW(
		_In_ LPCWSTR szProductCode,        // Information is queried on this product
		_In_opt_ LPCWSTR szUserSid,        // Account of this product instance
		_In_ MSIINSTALLCONTEXT dwContext,   // Context of this product instance
		_In_ LPCWSTR szFeature,            // Name of the feature being queried
		_Out_opt_ INSTALLSTATE *pdwState);  // State value
#ifdef UNICODE
#define MsiQueryFeatureStateEx  MsiQueryFeatureStateExW
#else
#define MsiQueryFeatureStateEx  MsiQueryFeatureStateExA
#endif // !UNICODE
#endif //(_WIN32_MSI >= 300)

// Indicate intent to use a product feature, increments usage count
// Prompts for CD if not loaded, does not install feature

INSTALLSTATE WINAPI MsiUseFeatureA(
	_In_ LPCSTR  szProduct,
	_In_ LPCSTR  szFeature);
INSTALLSTATE WINAPI MsiUseFeatureW(
	_In_ LPCWSTR  szProduct,
	_In_ LPCWSTR  szFeature);
#ifdef UNICODE
#define MsiUseFeature  MsiUseFeatureW
#else
#define MsiUseFeature  MsiUseFeatureA
#endif // !UNICODE

// Indicate intent to use a product feature, increments usage count
// Prompts for CD if not loaded, does not install feature
// Allows for bypassing component detection where performance is critical

INSTALLSTATE WINAPI MsiUseFeatureExA(
	_In_ LPCSTR  szProduct,          // product code
	_In_ LPCSTR  szFeature,          // feature ID
	_In_ DWORD     dwInstallMode,      // INSTALLMODE_NODETECTION, else 0
	_Reserved_ DWORD     dwReserved);  // reserved, must be 0
INSTALLSTATE WINAPI MsiUseFeatureExW(
	_In_ LPCWSTR  szProduct,          // product code
	_In_ LPCWSTR  szFeature,          // feature ID
	_In_ DWORD     dwInstallMode,      // INSTALLMODE_NODETECTION, else 0
	_Reserved_ DWORD     dwReserved);  // reserved, must be 0
#ifdef UNICODE
#define MsiUseFeatureEx  MsiUseFeatureExW
#else
#define MsiUseFeatureEx  MsiUseFeatureExA
#endif // !UNICODE

// Return the usage metrics for a product feature

UINT WINAPI MsiGetFeatureUsageA(
	_In_ LPCSTR      szProduct,            // product code
	_In_ LPCSTR      szFeature,            // feature ID
	_Out_opt_ LPDWORD       pdwUseCount,     // returned use count
	_Out_opt_ LPWORD        pwDateUsed);     // last date used (DOS date format)
UINT WINAPI MsiGetFeatureUsageW(
	_In_ LPCWSTR      szProduct,            // product code
	_In_ LPCWSTR      szFeature,            // feature ID
	_Out_opt_ LPDWORD       pdwUseCount,     // returned use count
	_Out_opt_ LPWORD        pwDateUsed);     // last date used (DOS date format)
#ifdef UNICODE
#define MsiGetFeatureUsage  MsiGetFeatureUsageW
#else
#define MsiGetFeatureUsage  MsiGetFeatureUsageA
#endif // !UNICODE

// Force the installed state for a product feature

UINT WINAPI MsiConfigureFeatureA(
	_In_ LPCSTR  szProduct,
	_In_ LPCSTR  szFeature,
	_In_ INSTALLSTATE eInstallState);   // local/source/default/absent/lock/uncache
UINT WINAPI MsiConfigureFeatureW(
	_In_ LPCWSTR  szProduct,
	_In_ LPCWSTR  szFeature,
	_In_ INSTALLSTATE eInstallState);   // local/source/default/absent/lock/uncache
#ifdef UNICODE
#define MsiConfigureFeature  MsiConfigureFeatureW
#else
#define MsiConfigureFeature  MsiConfigureFeatureA
#endif // !UNICODE


// Reinstall feature, used to validate or correct problems

UINT WINAPI MsiReinstallFeatureA(
	_In_ LPCSTR      szProduct,        // product code
	_In_ LPCSTR      szFeature,        // feature ID, NULL for entire product
	_In_ DWORD         dwReinstallMode); // one or more REINSTALLMODE modes
UINT WINAPI MsiReinstallFeatureW(
	_In_ LPCWSTR      szProduct,        // product code
	_In_ LPCWSTR      szFeature,        // feature ID, NULL for entire product
	_In_ DWORD         dwReinstallMode); // one or more REINSTALLMODE modes
#ifdef UNICODE
#define MsiReinstallFeature  MsiReinstallFeatureW
#else
#define MsiReinstallFeature  MsiReinstallFeatureA
#endif // !UNICODE

// --------------------------------------------------------------------------
// Functions to query a component. For functions that return a path to
// a particular component, the state of the feature being used should
// have been checked previously.
// --------------------------------------------------------------------------

// Return full component path, performing any necessary installation
// calls MsiQueryFeatureState to detect that all components are installed
// then calls MsiConfigureFeature if any of its components are uninstalled
// then calls MsiLocateComponent to obtain the path the its key file

UINT WINAPI MsiProvideComponentA(
	_In_ LPCSTR     szProduct,                               // product code in case install required
	_In_ LPCSTR     szFeature,                               // feature ID in case install required
	_In_ LPCSTR     szComponent,                             // component ID
	_In_ DWORD        dwInstallMode,                           // either of type INSTALLMODE or a combination of the REINSTALLMODE flags
	_Out_writes_opt_(*pcchPathBuf)  LPSTR      lpPathBuf,    // returned path, NULL if not desired
	_Inout_opt_                     LPDWORD      pcchPathBuf); // in/out buffer character count
UINT WINAPI MsiProvideComponentW(
	_In_ LPCWSTR     szProduct,                               // product code in case install required
	_In_ LPCWSTR     szFeature,                               // feature ID in case install required
	_In_ LPCWSTR     szComponent,                             // component ID
	_In_ DWORD        dwInstallMode,                           // either of type INSTALLMODE or a combination of the REINSTALLMODE flags
	_Out_writes_opt_(*pcchPathBuf)  LPWSTR      lpPathBuf,    // returned path, NULL if not desired
	_Inout_opt_                     LPDWORD      pcchPathBuf); // in/out buffer character count
#ifdef UNICODE
#define MsiProvideComponent  MsiProvideComponentW
#else
#define MsiProvideComponent  MsiProvideComponentA
#endif // !UNICODE

// Return full component path for a qualified component, performing any necessary installation. 
// Prompts for source if necessary and increments the usage count for the feature.

UINT WINAPI MsiProvideQualifiedComponentA(
	_In_ LPCSTR     szCategory,                           // component category ID
	_In_ LPCSTR     szQualifier,                          // specifies which component to access
	_In_ DWORD        dwInstallMode,                        // either of type INSTALLMODE or a combination of the REINSTALLMODE flags
	_Out_writes_opt_(*pcchPathBuf)  LPSTR  lpPathBuf,     // returned path, NULL if not desired
	_Inout_opt_                     LPDWORD  pcchPathBuf);  // in/out buffer character count
UINT WINAPI MsiProvideQualifiedComponentW(
	_In_ LPCWSTR     szCategory,                           // component category ID
	_In_ LPCWSTR     szQualifier,                          // specifies which component to access
	_In_ DWORD        dwInstallMode,                        // either of type INSTALLMODE or a combination of the REINSTALLMODE flags
	_Out_writes_opt_(*pcchPathBuf)  LPWSTR  lpPathBuf,     // returned path, NULL if not desired
	_Inout_opt_                     LPDWORD  pcchPathBuf);  // in/out buffer character count
#ifdef UNICODE
#define MsiProvideQualifiedComponent  MsiProvideQualifiedComponentW
#else
#define MsiProvideQualifiedComponent  MsiProvideQualifiedComponentA
#endif // !UNICODE

// Return full component path for a qualified component, performing any necessary installation. 
// Prompts for source if necessary and increments the usage count for the feature.
// The szProduct parameter specifies the product to match that has published the qualified
// component. If null, this API works the same as MsiProvideQualifiedComponent. 

UINT WINAPI MsiProvideQualifiedComponentExA(
	_In_ LPCSTR     szCategory,                           // component category ID
	_In_ LPCSTR     szQualifier,                          // specifies which component to access
	_In_ DWORD        dwInstallMode,                        // either of type INSTALLMODE or a combination of the REINSTALLMODE flags
	_In_opt_ LPCSTR     szProduct,                        // the product code 
	_Reserved_ DWORD        dwUnused1,                      // not used, must be zero
	_Reserved_ DWORD        dwUnused2,                      // not used, must be zero
	_Out_writes_opt_(*pcchPathBuf)  LPSTR  lpPathBuf,     // returned path, NULL if not desired
	_Inout_opt_                     LPDWORD  pcchPathBuf);  // in/out buffer character count
UINT WINAPI MsiProvideQualifiedComponentExW(
	_In_ LPCWSTR     szCategory,                           // component category ID
	_In_ LPCWSTR     szQualifier,                          // specifies which component to access
	_In_ DWORD        dwInstallMode,                        // either of type INSTALLMODE or a combination of the REINSTALLMODE flags
	_In_opt_ LPCWSTR     szProduct,                        // the product code 
	_Reserved_ DWORD        dwUnused1,                      // not used, must be zero
	_Reserved_ DWORD        dwUnused2,                      // not used, must be zero
	_Out_writes_opt_(*pcchPathBuf)  LPWSTR  lpPathBuf,     // returned path, NULL if not desired
	_Inout_opt_                     LPDWORD  pcchPathBuf);  // in/out buffer character count
#ifdef UNICODE
#define MsiProvideQualifiedComponentEx  MsiProvideQualifiedComponentExW
#else
#define MsiProvideQualifiedComponentEx  MsiProvideQualifiedComponentExA
#endif // !UNICODE

// Return full path to an installed component

INSTALLSTATE WINAPI MsiGetComponentPathA(
	_In_ LPCSTR   szProduct,                        // product code for client product
	_In_ LPCSTR   szComponent,                      // component Id, string GUID
	_Out_writes_opt_(*pcchBuf)  LPSTR  lpPathBuf,   // returned path
	_Inout_opt_                 LPDWORD  pcchBuf);    // in/out buffer character count
INSTALLSTATE WINAPI MsiGetComponentPathW(
	_In_ LPCWSTR   szProduct,                        // product code for client product
	_In_ LPCWSTR   szComponent,                      // component Id, string GUID
	_Out_writes_opt_(*pcchBuf)  LPWSTR  lpPathBuf,   // returned path
	_Inout_opt_                 LPDWORD  pcchBuf);    // in/out buffer character count
#ifdef UNICODE
#define MsiGetComponentPath  MsiGetComponentPathW
#else
#define MsiGetComponentPath  MsiGetComponentPathA
#endif // !UNICODE

#if (_WIN32_MSI >= 500)
// Return full path to an installed component for a given user
// and context

INSTALLSTATE WINAPI MsiGetComponentPathExA(		
	_In_					LPCSTR		szProductCode,		// ProductCode
	_In_					LPCSTR		szComponentCode,	// ComponentCode
	_In_opt_				LPCSTR		szUserSid,		// User account
	_In_opt_				MSIINSTALLCONTEXT	dwContext,		// Install context
	_Out_writes_opt_(*pcchOutPathBuffer)	LPSTR			lpOutPathBuffer,	// returned path
	_Inout_opt_				LPDWORD			pcchOutPathBuffer);	// in/out buffer character count
INSTALLSTATE WINAPI MsiGetComponentPathExW(		
	_In_					LPCWSTR		szProductCode,		// ProductCode
	_In_					LPCWSTR		szComponentCode,	// ComponentCode
	_In_opt_				LPCWSTR		szUserSid,		// User account
	_In_opt_				MSIINSTALLCONTEXT	dwContext,		// Install context
	_Out_writes_opt_(*pcchOutPathBuffer)	LPWSTR			lpOutPathBuffer,	// returned path
	_Inout_opt_				LPDWORD			pcchOutPathBuffer);	// in/out buffer character count
#ifdef UNICODE
#define MsiGetComponentPathEx  MsiGetComponentPathExW
#else
#define MsiGetComponentPathEx  MsiGetComponentPathExA
#endif // !UNICODE
#endif // (_WIN32_MSI >= 500)		

#if (_WIN32_MSI >= 150)

#define MSIASSEMBLYINFO_NETASSEMBLY   0 // Net assemblies
#define MSIASSEMBLYINFO_WIN32ASSEMBLY 1 // Win32 assemblies

// Return full component path for an assembly installed via the WI, performing any necessary installation. 
// Prompts for source if necessary and increments the usage count for the feature.
// The szAssemblyName parameter specifies the stringized assembly name. 
// The szAppContext is the full path to the .cfg file or the app exe to which the assembly being requested 
// has been privatised to, which is null for global assemblies

UINT WINAPI MsiProvideAssemblyA(
	_In_ LPCSTR    szAssemblyName,                       // stringized assembly name
	_In_opt_ LPCSTR    szAppContext,                     // specifies the full path to the parent asm's .cfg file, null for global assemblies
	_In_ DWORD       dwInstallMode,                        // either of type INSTALLMODE or a combination of the REINSTALLMODE flags
	_In_ DWORD       dwAssemblyInfo,                       // assembly info, including assembly type
	_Out_writes_opt_(*pcchPathBuf)  LPSTR  lpPathBuf,    // returned path, NULL if not desired
	_Inout_opt_                     LPDWORD  pcchPathBuf); // in/out buffer character count
UINT WINAPI MsiProvideAssemblyW(
	_In_ LPCWSTR    szAssemblyName,                       // stringized assembly name
	_In_opt_ LPCWSTR    szAppContext,                     // specifies the full path to the parent asm's .cfg file, null for global assemblies
	_In_ DWORD       dwInstallMode,                        // either of type INSTALLMODE or a combination of the REINSTALLMODE flags
	_In_ DWORD       dwAssemblyInfo,                       // assembly info, including assembly type
	_Out_writes_opt_(*pcchPathBuf)  LPWSTR  lpPathBuf,    // returned path, NULL if not desired
	_Inout_opt_                     LPDWORD  pcchPathBuf); // in/out buffer character count
#ifdef UNICODE
#define MsiProvideAssembly  MsiProvideAssemblyW
#else
#define MsiProvideAssembly  MsiProvideAssemblyA
#endif // !UNICODE

#endif //(_WIN32_MSI >=  150)

#if (_WIN32_MSI >= 300)
// Query the installed state of a component in the specified product instance

UINT WINAPI MsiQueryComponentStateA(
		_In_ LPCSTR szProductCode,       // Information is queried on this product
		_In_opt_ LPCSTR szUserSid,       // Account of this product instance
		_In_ MSIINSTALLCONTEXT dwContext,  // Context of this product instance
		_In_ LPCSTR szComponentCode,     // Name of the component being queried
		_Out_opt_ INSTALLSTATE *pdwState); // State value
UINT WINAPI MsiQueryComponentStateW(
		_In_ LPCWSTR szProductCode,       // Information is queried on this product
		_In_opt_ LPCWSTR szUserSid,       // Account of this product instance
		_In_ MSIINSTALLCONTEXT dwContext,  // Context of this product instance
		_In_ LPCWSTR szComponentCode,     // Name of the component being queried
		_Out_opt_ INSTALLSTATE *pdwState); // State value
#ifdef UNICODE
#define MsiQueryComponentState  MsiQueryComponentStateW
#else
#define MsiQueryComponentState  MsiQueryComponentStateA
#endif // !UNICODE
#endif //(_WIN32_MSI >= 300)


// --------------------------------------------------------------------------
// Functions to iterate registered products, features, and components.
// As with reg keys, they accept a 0-based index into the enumeration.
// --------------------------------------------------------------------------

// Enumerate the registered products, either installed or advertised

UINT WINAPI MsiEnumProductsA(
	_In_ DWORD     iProductIndex,                               // 0-based index into registered products
	_Out_writes_(MAX_GUID_CHARS+1)  LPSTR   lpProductBuf);    // buffer of char count: 39 (size of string GUID)
UINT WINAPI MsiEnumProductsW(
	_In_ DWORD     iProductIndex,                               // 0-based index into registered products
	_Out_writes_(MAX_GUID_CHARS+1)  LPWSTR   lpProductBuf);    // buffer of char count: 39 (size of string GUID)
#ifdef UNICODE
#define MsiEnumProducts  MsiEnumProductsW
#else
#define MsiEnumProducts  MsiEnumProductsA
#endif // !UNICODE

#if (_WIN32_MSI >= 300)
// Enumerate the registered products, either installed or advertised in
// the specified context
UINT WINAPI MsiEnumProductsExA(
		_In_opt_ LPCSTR szProductCode,                                      // Enumerate instances of this product
		_In_opt_ LPCSTR szUserSid,                                          // Account for enumeration
		_In_ DWORD dwContext,                                                 // Contexts for enumeration
		_In_ DWORD dwIndex,                                                   // Index for enumeration
		_Out_writes_opt_(MAX_GUID_CHARS+1) CHAR szInstalledProductCode[39], // Enumerated product's ProductCode
		_Out_opt_ MSIINSTALLCONTEXT *pdwInstalledContext,                     // Enumerated product's context
		_Out_writes_opt_(*pcchSid) LPSTR  szSid,                            // Enumerated product's user account
		_Inout_opt_ LPDWORD pcchSid);                                         // in/out character count of szSid 
// Enumerate the registered products, either installed or advertised in
// the specified context
UINT WINAPI MsiEnumProductsExW(
		_In_opt_ LPCWSTR szProductCode,                                      // Enumerate instances of this product
		_In_opt_ LPCWSTR szUserSid,                                          // Account for enumeration
		_In_ DWORD dwContext,                                                 // Contexts for enumeration
		_In_ DWORD dwIndex,                                                   // Index for enumeration
		_Out_writes_opt_(MAX_GUID_CHARS+1) WCHAR szInstalledProductCode[39], // Enumerated product's ProductCode
		_Out_opt_ MSIINSTALLCONTEXT *pdwInstalledContext,                     // Enumerated product's context
		_Out_writes_opt_(*pcchSid) LPWSTR  szSid,                            // Enumerated product's user account
		_Inout_opt_ LPDWORD pcchSid);                                         // in/out character count of szSid 
#ifdef UNICODE
#define MsiEnumProductsEx  MsiEnumProductsExW
#else
#define MsiEnumProductsEx  MsiEnumProductsExA
#endif // !UNICODE
#endif //(_WIN32_MSI >= 300)

#if (_WIN32_MSI >=  110)

// Enumerate products with given upgrade code

UINT WINAPI MsiEnumRelatedProductsA(
	_In_ LPCSTR  lpUpgradeCode,                               // upgrade code of products to enumerate
	_Reserved_ DWORD     dwReserved,                            // reserved, must be 0
	_In_ DWORD     iProductIndex,                               // 0-based index into registered products
	_Out_writes_(MAX_GUID_CHARS+1)  LPSTR   lpProductBuf);    // buffer of char count: 39 (size of string GUID)
UINT WINAPI MsiEnumRelatedProductsW(
	_In_ LPCWSTR  lpUpgradeCode,                               // upgrade code of products to enumerate
	_Reserved_ DWORD     dwReserved,                            // reserved, must be 0
	_In_ DWORD     iProductIndex,                               // 0-based index into registered products
	_Out_writes_(MAX_GUID_CHARS+1)  LPWSTR   lpProductBuf);    // buffer of char count: 39 (size of string GUID)
#ifdef UNICODE
#define MsiEnumRelatedProducts  MsiEnumRelatedProductsW
#else
#define MsiEnumRelatedProducts  MsiEnumRelatedProductsA
#endif // !UNICODE

#endif //(_WIN32_MSI >=  110)

// Enumerate the advertised features for a given product.
// If parent is not required, supplying NULL will improve performance.

UINT WINAPI MsiEnumFeaturesA(
	_In_ LPCSTR  szProduct,
	_In_ DWORD     iFeatureIndex,                                    // 0-based index into published features
	_Out_writes_(MAX_FEATURE_CHARS+1)      LPSTR   lpFeatureBuf,   // feature name buffer,   size=MAX_FEATURE_CHARS+1
	_Out_writes_opt_(MAX_FEATURE_CHARS+1)  LPSTR   lpParentBuf);   // parent feature buffer, size=MAX_FEATURE_CHARS+1
UINT WINAPI MsiEnumFeaturesW(
	_In_ LPCWSTR  szProduct,
	_In_ DWORD     iFeatureIndex,                                    // 0-based index into published features
	_Out_writes_(MAX_FEATURE_CHARS+1)      LPWSTR   lpFeatureBuf,   // feature name buffer,   size=MAX_FEATURE_CHARS+1
	_Out_writes_opt_(MAX_FEATURE_CHARS+1)  LPWSTR   lpParentBuf);   // parent feature buffer, size=MAX_FEATURE_CHARS+1
#ifdef UNICODE
#define MsiEnumFeatures  MsiEnumFeaturesW
#else
#define MsiEnumFeatures  MsiEnumFeaturesA
#endif // !UNICODE

// Enumerate the installed components for all products

UINT WINAPI MsiEnumComponentsA(
	_In_ DWORD    iComponentIndex,                              // 0-based index into installed components
	_Out_writes_(MAX_GUID_CHARS+1)  LPSTR   lpComponentBuf);  // buffer of char count: 39 (size of string GUID)
UINT WINAPI MsiEnumComponentsW(
	_In_ DWORD    iComponentIndex,                              // 0-based index into installed components
	_Out_writes_(MAX_GUID_CHARS+1)  LPWSTR   lpComponentBuf);  // buffer of char count: 39 (size of string GUID)
#ifdef UNICODE
#define MsiEnumComponents  MsiEnumComponentsW
#else
#define MsiEnumComponents  MsiEnumComponentsA
#endif // !UNICODE

#if (_WIN32_MSI >=  500)
// Enumerate installed components for all products for a given users 
// and context

UINT WINAPI MsiEnumComponentsExA(	
	_In_opt_				LPCSTR		szUserSid,			// Account for enumeration
	_In_					DWORD			dwContext,			// Contexts for enumeration
	_In_					DWORD			dwIndex,			// 0-based index into installed components
	_Out_writes_opt_(MAX_GUID_CHARS+1)	CHAR			szInstalledComponentCode[39],	// Enumerated component's ComponentCode
	_Out_opt_				MSIINSTALLCONTEXT*	pdwInstalledContext,		// Enumerated component's context
	_Out_writes_opt_(*pcchSid)		LPSTR			szSid,				// Enumerated component's user account
	_Inout_opt_				LPDWORD			pcchSid);			// in/out character count of szSid 
UINT WINAPI MsiEnumComponentsExW(	
	_In_opt_				LPCWSTR		szUserSid,			// Account for enumeration
	_In_					DWORD			dwContext,			// Contexts for enumeration
	_In_					DWORD			dwIndex,			// 0-based index into installed components
	_Out_writes_opt_(MAX_GUID_CHARS+1)	WCHAR			szInstalledComponentCode[39],	// Enumerated component's ComponentCode
	_Out_opt_				MSIINSTALLCONTEXT*	pdwInstalledContext,		// Enumerated component's context
	_Out_writes_opt_(*pcchSid)		LPWSTR			szSid,				// Enumerated component's user account
	_Inout_opt_				LPDWORD			pcchSid);			// in/out character count of szSid 
#ifdef UNICODE
#define MsiEnumComponentsEx  MsiEnumComponentsExW
#else
#define MsiEnumComponentsEx  MsiEnumComponentsExA
#endif // !UNICODE
	
#endif // (_WIN32_MSI >= 500)

// Enumerate the client products for a component

UINT WINAPI MsiEnumClientsA(
	_In_ LPCSTR  szComponent,
	_In_ DWORD     iProductIndex,                               // 0-based index into client products
	_Out_writes_(MAX_GUID_CHARS+1)  LPSTR   lpProductBuf);    // buffer of char count: 39 (size of string GUID)
UINT WINAPI MsiEnumClientsW(
	_In_ LPCWSTR  szComponent,
	_In_ DWORD     iProductIndex,                               // 0-based index into client products
	_Out_writes_(MAX_GUID_CHARS+1)  LPWSTR   lpProductBuf);    // buffer of char count: 39 (size of string GUID)
#ifdef UNICODE
#define MsiEnumClients  MsiEnumClientsW
#else
#define MsiEnumClients  MsiEnumClientsA
#endif // !UNICODE

#if (_WIN32_MSI >=  500)
// Enumerate the client products for a component for a given users 
// and context

UINT WINAPI MsiEnumClientsExA (	
	_In_					LPCSTR		szComponent,		// ComponentCode for enumeration
	_In_opt_				LPCSTR		szUserSid,		// Account for enumeration
	_In_					DWORD			dwContext,		// Contexts for enumeration
	_In_					DWORD			dwProductIndex,		// 0-based index into component clients
	_Out_writes_opt_(MAX_GUID_CHARS+1)	CHAR			szProductBuf[39],	// Enumerated component's client ProductCode
	_Out_opt_				MSIINSTALLCONTEXT	*pdwInstalledContext,	// Enumerated component's client product's context
	_Out_writes_opt_(*pcchSid)		LPSTR			szSid,			// Enumerated user account
	_Inout_opt_				LPDWORD			pcchSid);		// in/out character count of szSid 
UINT WINAPI MsiEnumClientsExW (	
	_In_					LPCWSTR		szComponent,		// ComponentCode for enumeration
	_In_opt_				LPCWSTR		szUserSid,		// Account for enumeration
	_In_					DWORD			dwContext,		// Contexts for enumeration
	_In_					DWORD			dwProductIndex,		// 0-based index into component clients
	_Out_writes_opt_(MAX_GUID_CHARS+1)	WCHAR			szProductBuf[39],	// Enumerated component's client ProductCode
	_Out_opt_				MSIINSTALLCONTEXT	*pdwInstalledContext,	// Enumerated component's client product's context
	_Out_writes_opt_(*pcchSid)		LPWSTR			szSid,			// Enumerated user account
	_Inout_opt_				LPDWORD			pcchSid);		// in/out character count of szSid 
#ifdef UNICODE
#define MsiEnumClientsEx  MsiEnumClientsExW
#else
#define MsiEnumClientsEx  MsiEnumClientsExA
#endif // !UNICODE
#endif // (_WIN32_MSI >= 500)									

// Enumerate the qualifiers for an advertised component.

UINT WINAPI MsiEnumComponentQualifiersA(
	_In_                                       LPCSTR   szComponent,             // generic component ID that is qualified
	_In_                                       DWORD      iIndex,	               // 0-based index into qualifiers
	_Out_writes_(*pcchQualifierBuf)            LPSTR    lpQualifierBuf,          // qualifier buffer
	_Inout_                                    LPDWORD    pcchQualifierBuf,        // in/out qualifier buffer character count
	_Out_writes_opt_(*pcchApplicationDataBuf)  LPSTR    lpApplicationDataBuf,    // description buffer
	_Inout_opt_                                LPDWORD    pcchApplicationDataBuf); // in/out description buffer character count
UINT WINAPI MsiEnumComponentQualifiersW(
	_In_                                       LPCWSTR   szComponent,             // generic component ID that is qualified
	_In_                                       DWORD      iIndex,	               // 0-based index into qualifiers
	_Out_writes_(*pcchQualifierBuf)            LPWSTR    lpQualifierBuf,          // qualifier buffer
	_Inout_                                    LPDWORD    pcchQualifierBuf,        // in/out qualifier buffer character count
	_Out_writes_opt_(*pcchApplicationDataBuf)  LPWSTR    lpApplicationDataBuf,    // description buffer
	_Inout_opt_                                LPDWORD    pcchApplicationDataBuf); // in/out description buffer character count
#ifdef UNICODE
#define MsiEnumComponentQualifiers  MsiEnumComponentQualifiersW
#else
#define MsiEnumComponentQualifiers  MsiEnumComponentQualifiersA
#endif // !UNICODE

// --------------------------------------------------------------------------
// Functions to obtain product or package information.
// --------------------------------------------------------------------------

// Open the installation for a product to obtain detailed information

UINT WINAPI MsiOpenProductA(
	_In_ LPCSTR   szProduct,     // product code
	_Out_ MSIHANDLE  *hProduct);   // returned product handle, must be closed
UINT WINAPI MsiOpenProductW(
	_In_ LPCWSTR   szProduct,     // product code
	_Out_ MSIHANDLE  *hProduct);   // returned product handle, must be closed
#ifdef UNICODE
#define MsiOpenProduct  MsiOpenProductW
#else
#define MsiOpenProduct  MsiOpenProductA
#endif // !UNICODE

// Open a product package in order to access product properties

UINT WINAPI MsiOpenPackageA(
	_In_ LPCSTR    szPackagePath,      // path to package, or database handle: #nnnn
	_Out_ MSIHANDLE  *hProduct);         // returned product handle, must be closed
UINT WINAPI MsiOpenPackageW(
	_In_ LPCWSTR    szPackagePath,      // path to package, or database handle: #nnnn
	_Out_ MSIHANDLE  *hProduct);         // returned product handle, must be closed
#ifdef UNICODE
#define MsiOpenPackage  MsiOpenPackageW
#else
#define MsiOpenPackage  MsiOpenPackageA
#endif // !UNICODE

#if (_WIN32_MSI >=  150)

// Open a product package in order to access product properties
// Option to create a "safe" engine that does not look at machine state
//  and does not allow for modification of machine state

UINT WINAPI MsiOpenPackageExA(
	_In_ LPCSTR   szPackagePath,  // path to package, or database handle: #nnnn
	_In_ DWORD      dwOptions,      // options flags to indicate whether or not to ignore machine state
	_Out_ MSIHANDLE *hProduct);     // returned product handle, must be closed
UINT WINAPI MsiOpenPackageExW(
	_In_ LPCWSTR   szPackagePath,  // path to package, or database handle: #nnnn
	_In_ DWORD      dwOptions,      // options flags to indicate whether or not to ignore machine state
	_Out_ MSIHANDLE *hProduct);     // returned product handle, must be closed
#ifdef UNICODE
#define MsiOpenPackageEx  MsiOpenPackageExW
#else
#define MsiOpenPackageEx  MsiOpenPackageExA
#endif // !UNICODE

#endif //(_WIN32_MSI >= 150)

#if (_WIN32_MSI >= 400)

UINT WINAPI MsiGetPatchFileListA(
	_In_  LPCSTR    szProductCode,
	_In_  LPCSTR    szPatchPackages,
	_Out_ LPDWORD      pcFiles,
	_Outptr_result_buffer_maybenull_(*pcFiles)MSIHANDLE** pphFileRecords);
UINT WINAPI MsiGetPatchFileListW(
	_In_  LPCWSTR    szProductCode,
	_In_  LPCWSTR    szPatchPackages,
	_Out_ LPDWORD      pcFiles,
	_Outptr_result_buffer_maybenull_(*pcFiles)MSIHANDLE** pphFileRecords);
#ifdef UNICODE
#define MsiGetPatchFileList  MsiGetPatchFileListW
#else
#define MsiGetPatchFileList  MsiGetPatchFileListA
#endif // !UNICODE

#endif // #if (_WIN32_MSI >= 400)

// Provide the value for an installation property.

UINT WINAPI MsiGetProductPropertyA(
	_In_ MSIHANDLE   hProduct,                                   // product handle obtained from MsiOpenProduct
	_In_ LPCSTR    szProperty,                                 // property name, case-sensitive
	_Out_writes_opt_(*pcchValueBuf)  LPSTR     lpValueBuf,     // returned value, NULL if not desired
	_Inout_opt_                      LPDWORD     pcchValueBuf);  // in/out buffer character count
UINT WINAPI MsiGetProductPropertyW(
	_In_ MSIHANDLE   hProduct,                                   // product handle obtained from MsiOpenProduct
	_In_ LPCWSTR    szProperty,                                 // property name, case-sensitive
	_Out_writes_opt_(*pcchValueBuf)  LPWSTR     lpValueBuf,     // returned value, NULL if not desired
	_Inout_opt_                      LPDWORD     pcchValueBuf);  // in/out buffer character count
#ifdef UNICODE
#define MsiGetProductProperty  MsiGetProductPropertyW
#else
#define MsiGetProductProperty  MsiGetProductPropertyA
#endif // !UNICODE


// Determine whether a file is a package
// Returns ERROR_SUCCESS if file is a package.

UINT WINAPI MsiVerifyPackageA(
	_In_ LPCSTR      szPackagePath);   // location of package
UINT WINAPI MsiVerifyPackageW(
	_In_ LPCWSTR      szPackagePath);   // location of package
#ifdef UNICODE
#define MsiVerifyPackage  MsiVerifyPackageW
#else
#define MsiVerifyPackage  MsiVerifyPackageA
#endif // !UNICODE


// Provide descriptive information for product feature: title and description.
// Returns the install level for the feature, or -1 if feature is unknown.
//   0 = feature is not available on this machine
//   1 = highest priority, feature installed if parent is installed
//  >1 = decreasing priority, feature installation based on InstallLevel property

UINT WINAPI MsiGetFeatureInfoA(
	_In_                             MSIHANDLE   hProduct,      // product handle obtained from MsiOpenProduct
	_In_                             LPCSTR    szFeature,     // feature name
	_Out_opt_                        LPDWORD     lpAttributes,  // attribute flags for the feature, using INSTALLFEATUREATTRIBUTE
	_Out_writes_opt_(*pcchTitleBuf)  LPSTR     lpTitleBuf,    // returned localized name, NULL if not desired
	_Inout_opt_                      LPDWORD     pcchTitleBuf,  // in/out buffer character count
	_Out_writes_opt_(*pcchHelpBuf)   LPSTR     lpHelpBuf,     // returned description, NULL if not desired
	_Inout_opt_                      LPDWORD     pcchHelpBuf);  // in/out buffer character count
UINT WINAPI MsiGetFeatureInfoW(
	_In_                             MSIHANDLE   hProduct,      // product handle obtained from MsiOpenProduct
	_In_                             LPCWSTR    szFeature,     // feature name
	_Out_opt_                        LPDWORD     lpAttributes,  // attribute flags for the feature, using INSTALLFEATUREATTRIBUTE
	_Out_writes_opt_(*pcchTitleBuf)  LPWSTR     lpTitleBuf,    // returned localized name, NULL if not desired
	_Inout_opt_                      LPDWORD     pcchTitleBuf,  // in/out buffer character count
	_Out_writes_opt_(*pcchHelpBuf)   LPWSTR     lpHelpBuf,     // returned description, NULL if not desired
	_Inout_opt_                      LPDWORD     pcchHelpBuf);  // in/out buffer character count
#ifdef UNICODE
#define MsiGetFeatureInfo  MsiGetFeatureInfoW
#else
#define MsiGetFeatureInfo  MsiGetFeatureInfoA
#endif // !UNICODE

// --------------------------------------------------------------------------
// Functions to access or install missing components and files.
// These should be used as a last resort.
// --------------------------------------------------------------------------

// Install a component unexpectedly missing, provided only for error recovery
// This would typically occur due to failue to establish feature availability
// The product feature having the smallest incremental cost is installed

UINT WINAPI MsiInstallMissingComponentA(
	_In_ LPCSTR      szProduct,        // product code
	_In_ LPCSTR      szComponent,      // component Id, string GUID
	_In_ INSTALLSTATE eInstallState);    // local/source/default, absent invalid
UINT WINAPI MsiInstallMissingComponentW(
	_In_ LPCWSTR      szProduct,        // product code
	_In_ LPCWSTR      szComponent,      // component Id, string GUID
	_In_ INSTALLSTATE eInstallState);    // local/source/default, absent invalid
#ifdef UNICODE
#define MsiInstallMissingComponent  MsiInstallMissingComponentW
#else
#define MsiInstallMissingComponent  MsiInstallMissingComponentA
#endif // !UNICODE

// Install a file unexpectedly missing, provided only for error recovery
// This would typically occur due to failue to establish feature availability
// The missing component is determined from the product's File table, then
// the product feature having the smallest incremental cost is installed

UINT WINAPI MsiInstallMissingFileA(
	_In_ LPCSTR      szProduct,        // product code
	_In_ LPCSTR      szFile);          // file name, without path
UINT WINAPI MsiInstallMissingFileW(
	_In_ LPCWSTR      szProduct,        // product code
	_In_ LPCWSTR      szFile);          // file name, without path
#ifdef UNICODE
#define MsiInstallMissingFile  MsiInstallMissingFileW
#else
#define MsiInstallMissingFile  MsiInstallMissingFileA
#endif // !UNICODE

// Return full path to an installed component without a product code
// This function attempts to determine the product using MsiGetProductCode
// but is not guaranteed to find the correct product for the caller.
// MsiGetComponentPath should always be called when possible.

INSTALLSTATE WINAPI MsiLocateComponentA(
	_In_ LPCSTR szComponent,                         // component Id, string GUID
	_Out_writes_opt_(*pcchBuf)  LPSTR  lpPathBuf,    // returned path
	_Inout_opt_                 LPDWORD  pcchBuf);     // in/out buffer character count
INSTALLSTATE WINAPI MsiLocateComponentW(
	_In_ LPCWSTR szComponent,                         // component Id, string GUID
	_Out_writes_opt_(*pcchBuf)  LPWSTR  lpPathBuf,    // returned path
	_Inout_opt_                 LPDWORD  pcchBuf);     // in/out buffer character count
#ifdef UNICODE
#define MsiLocateComponent  MsiLocateComponentW
#else
#define MsiLocateComponent  MsiLocateComponentA
#endif // !UNICODE

#if (_WIN32_MSI >=  110)

// --------------------------------------------------------------------------
// Functions used to manage the list of valid sources.
// --------------------------------------------------------------------------

// Opens the list of sources for the specified user's install of the product
// and removes all network sources from the list. A NULL or empty value for
// the user name indicates the per-machine install.

UINT WINAPI MsiSourceListClearAllA(
	_In_ LPCSTR szProduct,          // product code
	_In_opt_ LPCSTR szUserName,     // user name or NULL/empty for per-machine
	_Reserved_  DWORD    dwReserved); // reserved - must be 0
UINT WINAPI MsiSourceListClearAllW(
	_In_ LPCWSTR szProduct,          // product code
	_In_opt_ LPCWSTR szUserName,     // user name or NULL/empty for per-machine
	_Reserved_  DWORD    dwReserved); // reserved - must be 0
#ifdef UNICODE
#define MsiSourceListClearAll  MsiSourceListClearAllW
#else
#define MsiSourceListClearAll  MsiSourceListClearAllA
#endif // !UNICODE

// Opens the list of sources for the specified user's install of the product
// and adds the provided source as a new network source. A NULL or empty 
// value for the user name indicates the per-machine install.

UINT WINAPI MsiSourceListAddSourceA(
	_In_ LPCSTR szProduct,          // product code
	_In_opt_ LPCSTR szUserName,     // user name or NULL/empty for per-machine
	_Reserved_ DWORD    dwReserved,   // reserved - must be 0
	_In_ LPCSTR szSource);          // new source
UINT WINAPI MsiSourceListAddSourceW(
	_In_ LPCWSTR szProduct,          // product code
	_In_opt_ LPCWSTR szUserName,     // user name or NULL/empty for per-machine
	_Reserved_ DWORD    dwReserved,   // reserved - must be 0
	_In_ LPCWSTR szSource);          // new source
#ifdef UNICODE
#define MsiSourceListAddSource  MsiSourceListAddSourceW
#else
#define MsiSourceListAddSource  MsiSourceListAddSourceA
#endif // !UNICODE

// Forces the installer to reevaluate the list of sources the next time that
// the specified product needs a source.

UINT WINAPI MsiSourceListForceResolutionA(
	_In_ LPCSTR szProduct,          // product code
	_In_opt_ LPCSTR szUserName,     // user name or NULL/empty for per-machine
	_Reserved_ DWORD    dwReserved);  // reserved - must be 0
UINT WINAPI MsiSourceListForceResolutionW(
	_In_ LPCWSTR szProduct,          // product code
	_In_opt_ LPCWSTR szUserName,     // user name or NULL/empty for per-machine
	_Reserved_ DWORD    dwReserved);  // reserved - must be 0
#ifdef UNICODE
#define MsiSourceListForceResolution  MsiSourceListForceResolutionW
#else
#define MsiSourceListForceResolution  MsiSourceListForceResolutionA
#endif // !UNICODE
	
#endif //(_WIN32_MSI >=  110)

#if (_WIN32_MSI >= 300)
// Adds a source or moves a source within the list of sources for the 
// specified product or patch instance. The source can be network/url type.

UINT WINAPI MsiSourceListAddSourceExA(
	_In_ LPCSTR szProductCodeOrPatchCode,// Source will be added to this product or patch
	_In_opt_ LPCSTR szUserSid,           // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,      // Context of this product/patch instance
	_In_ DWORD dwOptions,                  // Additional qualifiers
	_In_ LPCSTR szSource,                // source to be added/re-ordered
	_In_ DWORD dwIndex);                   // new index for the source
UINT WINAPI MsiSourceListAddSourceExW(
	_In_ LPCWSTR szProductCodeOrPatchCode,// Source will be added to this product or patch
	_In_opt_ LPCWSTR szUserSid,           // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,      // Context of this product/patch instance
	_In_ DWORD dwOptions,                  // Additional qualifiers
	_In_ LPCWSTR szSource,                // source to be added/re-ordered
	_In_ DWORD dwIndex);                   // new index for the source
#ifdef UNICODE
#define MsiSourceListAddSourceEx  MsiSourceListAddSourceExW
#else
#define MsiSourceListAddSourceEx  MsiSourceListAddSourceExA
#endif // !UNICODE

// Add or update a media source for the specified product or patch instance

UINT WINAPI MsiSourceListAddMediaDiskA(
	_In_ LPCSTR szProductCodeOrPatchCode,// Disk will be added to this product or patch
	_In_opt_ LPCSTR szUserSid,           // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,      // Context of this product/patch instance
	_In_ DWORD dwOptions,                  // Additional qualifiers
	_In_ DWORD dwDiskId,                   // ID of the disk to be added/updated
	_In_opt_ LPCSTR szVolumeLabel,       // Volume Label of the disk
	_In_opt_ LPCSTR szDiskPrompt);       // Disk Prompt of the disk
UINT WINAPI MsiSourceListAddMediaDiskW(
	_In_ LPCWSTR szProductCodeOrPatchCode,// Disk will be added to this product or patch
	_In_opt_ LPCWSTR szUserSid,           // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,      // Context of this product/patch instance
	_In_ DWORD dwOptions,                  // Additional qualifiers
	_In_ DWORD dwDiskId,                   // ID of the disk to be added/updated
	_In_opt_ LPCWSTR szVolumeLabel,       // Volume Label of the disk
	_In_opt_ LPCWSTR szDiskPrompt);       // Disk Prompt of the disk
#ifdef UNICODE
#define MsiSourceListAddMediaDisk  MsiSourceListAddMediaDiskW
#else
#define MsiSourceListAddMediaDisk  MsiSourceListAddMediaDiskA
#endif // !UNICODE

// Remove an existing source for the specified product or patch instance

UINT WINAPI MsiSourceListClearSourceA(
	_In_ LPCSTR szProductCodeOrPatchCode, // Source will be cleared for this product or patch
	_In_opt_ LPCSTR szUserSid,            // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,       // Context of this product/patch instance
	_In_ DWORD dwOptions,                   // Additional qualifiers
	_In_ LPCSTR szSource);                // source to be removed
UINT WINAPI MsiSourceListClearSourceW(
	_In_ LPCWSTR szProductCodeOrPatchCode, // Source will be cleared for this product or patch
	_In_opt_ LPCWSTR szUserSid,            // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,       // Context of this product/patch instance
	_In_ DWORD dwOptions,                   // Additional qualifiers
	_In_ LPCWSTR szSource);                // source to be removed
#ifdef UNICODE
#define MsiSourceListClearSource  MsiSourceListClearSourceW
#else
#define MsiSourceListClearSource  MsiSourceListClearSourceA
#endif // !UNICODE

// Remove an existing registered disk under the media source for the specified product or
// patch instance

UINT WINAPI MsiSourceListClearMediaDiskA(
	_In_ LPCSTR szProductCodeOrPatchCode, // Disk will be cleared for this product or patch
	_In_opt_ LPCSTR szUserSid,            // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,       // Context of this product/patch instance
	_In_ DWORD dwOptions,                   // Additional qualifiers
	_In_ DWORD dwDiskId);                   // DiskID to be removed
UINT WINAPI MsiSourceListClearMediaDiskW(
	_In_ LPCWSTR szProductCodeOrPatchCode, // Disk will be cleared for this product or patch
	_In_opt_ LPCWSTR szUserSid,            // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,       // Context of this product/patch instance
	_In_ DWORD dwOptions,                   // Additional qualifiers
	_In_ DWORD dwDiskId);                   // DiskID to be removed
#ifdef UNICODE
#define MsiSourceListClearMediaDisk  MsiSourceListClearMediaDiskW
#else
#define MsiSourceListClearMediaDisk  MsiSourceListClearMediaDiskA
#endif // !UNICODE

// Remove all existing sources of the given source type for the specified product or patch
// instance

UINT WINAPI MsiSourceListClearAllExA(
	_In_ LPCSTR szProductCodeOrPatchCode, // All sources will be cleared for this product or patch
	_In_opt_ LPCSTR szUserSid,            // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,       // Context of this product/patch instance
	_In_ DWORD dwOptions);                  // Additional qualifiers
UINT WINAPI MsiSourceListClearAllExW(
	_In_ LPCWSTR szProductCodeOrPatchCode, // All sources will be cleared for this product or patch
	_In_opt_ LPCWSTR szUserSid,            // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,       // Context of this product/patch instance
	_In_ DWORD dwOptions);                  // Additional qualifiers
#ifdef UNICODE
#define MsiSourceListClearAllEx  MsiSourceListClearAllExW
#else
#define MsiSourceListClearAllEx  MsiSourceListClearAllExA
#endif // !UNICODE

// Forces the installer to reevaluate the list of sources the next time that
// the specified product or patch instance needs a source.

UINT WINAPI MsiSourceListForceResolutionExA(
	_In_ LPCSTR szProductCodeOrPatchCode,  // Source Resolution will be forced for this product or patch
	_In_opt_ LPCSTR szUserSid,             // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,        // Context of this product/patch instance
	_In_ DWORD dwOptions);                   // Additional qualifiers
UINT WINAPI MsiSourceListForceResolutionExW(
	_In_ LPCWSTR szProductCodeOrPatchCode,  // Source Resolution will be forced for this product or patch
	_In_opt_ LPCWSTR szUserSid,             // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,        // Context of this product/patch instance
	_In_ DWORD dwOptions);                   // Additional qualifiers
#ifdef UNICODE
#define MsiSourceListForceResolutionEx  MsiSourceListForceResolutionExW
#else
#define MsiSourceListForceResolutionEx  MsiSourceListForceResolutionExA
#endif // !UNICODE

// Set the source list property to the provided value for the specified product or patch instance

UINT WINAPI MsiSourceListSetInfoA(
	_In_ LPCSTR szProductCodeOrPatchCode, // Source Info will be set for this product or patch
	_In_opt_ LPCSTR szUserSid,            // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,       // Context of this product/patch instance
	_In_ DWORD dwOptions,                   // Additional qualifiers
	_In_ LPCSTR szProperty,               // Property being set/modified
	_In_ LPCSTR szValue);                 // Value to be set
UINT WINAPI MsiSourceListSetInfoW(
	_In_ LPCWSTR szProductCodeOrPatchCode, // Source Info will be set for this product or patch
	_In_opt_ LPCWSTR szUserSid,            // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,       // Context of this product/patch instance
	_In_ DWORD dwOptions,                   // Additional qualifiers
	_In_ LPCWSTR szProperty,               // Property being set/modified
	_In_ LPCWSTR szValue);                 // Value to be set
#ifdef UNICODE
#define MsiSourceListSetInfo  MsiSourceListSetInfoW
#else
#define MsiSourceListSetInfo  MsiSourceListSetInfoA
#endif // !UNICODE

// Query the given source list property for the specified product or patch instance

UINT WINAPI MsiSourceListGetInfoA(
	_In_ LPCSTR szProductCodeOrPatchCode,        // Source Info will be queried for this produt or patch
	_In_opt_ LPCSTR szUserSid,                   // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,              // Context of this product/patch instance
	_In_ DWORD dwOptions,                          // Additional qualifiers
	_In_ LPCSTR szProperty,                      // Property being queried
	_Out_writes_opt_(*pcchValue)  LPSTR szValue, // Buffer to get the value
	_Inout_opt_ LPDWORD pcchValue);                // in/out character count of szValue
UINT WINAPI MsiSourceListGetInfoW(
	_In_ LPCWSTR szProductCodeOrPatchCode,        // Source Info will be queried for this produt or patch
	_In_opt_ LPCWSTR szUserSid,                   // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,              // Context of this product/patch instance
	_In_ DWORD dwOptions,                          // Additional qualifiers
	_In_ LPCWSTR szProperty,                      // Property being queried
	_Out_writes_opt_(*pcchValue)  LPWSTR szValue, // Buffer to get the value
	_Inout_opt_ LPDWORD pcchValue);                // in/out character count of szValue
#ifdef UNICODE
#define MsiSourceListGetInfo  MsiSourceListGetInfoW
#else
#define MsiSourceListGetInfo  MsiSourceListGetInfoA
#endif // !UNICODE

// Enumerate all sources for the specified product or patch instance

UINT WINAPI MsiSourceListEnumSourcesA(
	_In_ LPCSTR szProductCodeOrPatchCode,             // Sources will be enumerated for this product or patch
	_In_opt_ LPCSTR szUserSid,                        // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,                   // Context of this product/patch instance
	_In_ DWORD dwOptions,                               // Additional qualifiers
	_In_ DWORD dwIndex,                                 // Index value for enumeration
	_Out_writes_opt_(*pcchSource)  LPSTR szSource,    // Buffer to receive the enumerated source
	_Inout_opt_                    LPDWORD pcchSource); // in/out character count of szSource
UINT WINAPI MsiSourceListEnumSourcesW(
	_In_ LPCWSTR szProductCodeOrPatchCode,             // Sources will be enumerated for this product or patch
	_In_opt_ LPCWSTR szUserSid,                        // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,                   // Context of this product/patch instance
	_In_ DWORD dwOptions,                               // Additional qualifiers
	_In_ DWORD dwIndex,                                 // Index value for enumeration
	_Out_writes_opt_(*pcchSource)  LPWSTR szSource,    // Buffer to receive the enumerated source
	_Inout_opt_                    LPDWORD pcchSource); // in/out character count of szSource
#ifdef UNICODE
#define MsiSourceListEnumSources  MsiSourceListEnumSourcesW
#else
#define MsiSourceListEnumSources  MsiSourceListEnumSourcesA
#endif // !UNICODE

// Enumerate all registered media disks for the specified product or patch instance

UINT WINAPI MsiSourceListEnumMediaDisksA(
	_In_ LPCSTR szProductCodeOrPatchCode,                      // Disks will be enumerated for this product or patch
	_In_opt_ LPCSTR szUserSid,                                 // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,                            // Context of this product/patch instance
	_In_ DWORD dwOptions,                                        // Additional qualifiers
	_In_ DWORD dwIndex,                                          // Index value for enumeration
	_Out_opt_ LPDWORD pdwDiskId,                                 // ID of the disk being enumerated
	_Out_writes_opt_(*pcchVolumeLabel)  LPSTR szVolumeLabel,   // Volume label of the disk being enumerated
	_Inout_opt_                         LPDWORD pcchVolumeLabel, // in/out character count of szVolumeLabel
	_Out_writes_opt_(*pcchDiskPrompt)   LPSTR szDiskPrompt,    // DiskPrompt of the disk being enumerated
	_Inout_opt_                         LPDWORD pcchDiskPrompt); // in/out character count of szDiskPrompt
UINT WINAPI MsiSourceListEnumMediaDisksW(
	_In_ LPCWSTR szProductCodeOrPatchCode,                      // Disks will be enumerated for this product or patch
	_In_opt_ LPCWSTR szUserSid,                                 // Account of this product/patch instance
	_In_ MSIINSTALLCONTEXT dwContext,                            // Context of this product/patch instance
	_In_ DWORD dwOptions,                                        // Additional qualifiers
	_In_ DWORD dwIndex,                                          // Index value for enumeration
	_Out_opt_ LPDWORD pdwDiskId,                                 // ID of the disk being enumerated
	_Out_writes_opt_(*pcchVolumeLabel)  LPWSTR szVolumeLabel,   // Volume label of the disk being enumerated
	_Inout_opt_                         LPDWORD pcchVolumeLabel, // in/out character count of szVolumeLabel
	_Out_writes_opt_(*pcchDiskPrompt)   LPWSTR szDiskPrompt,    // DiskPrompt of the disk being enumerated
	_Inout_opt_                         LPDWORD pcchDiskPrompt); // in/out character count of szDiskPrompt
#ifdef UNICODE
#define MsiSourceListEnumMediaDisks  MsiSourceListEnumMediaDisksW
#else
#define MsiSourceListEnumMediaDisks  MsiSourceListEnumMediaDisksA
#endif // !UNICODE

#endif //(_WIN32_MSI >= 300)

// --------------------------------------------------------------------------
// Utility functions
// --------------------------------------------------------------------------

// Give the version string and language for a specified file

UINT WINAPI MsiGetFileVersionA(
	_In_                               LPCSTR    szFilePath,       // path to the file
	_Out_writes_opt_(*pcchVersionBuf)  LPSTR     lpVersionBuf,     // returned version string
	_Inout_opt_                        LPDWORD     pcchVersionBuf,   // in/out buffer byte count
	_Out_writes_opt_(*pcchLangBuf)     LPSTR     lpLangBuf,        // returned language string
	_Inout_opt_                        LPDWORD     pcchLangBuf);     // in/out buffer byte count
UINT WINAPI MsiGetFileVersionW(
	_In_                               LPCWSTR    szFilePath,       // path to the file
	_Out_writes_opt_(*pcchVersionBuf)  LPWSTR     lpVersionBuf,     // returned version string
	_Inout_opt_                        LPDWORD     pcchVersionBuf,   // in/out buffer byte count
	_Out_writes_opt_(*pcchLangBuf)     LPWSTR     lpLangBuf,        // returned language string
	_Inout_opt_                        LPDWORD     pcchLangBuf);     // in/out buffer byte count
#ifdef UNICODE
#define MsiGetFileVersion  MsiGetFileVersionW
#else
#define MsiGetFileVersion  MsiGetFileVersionA
#endif // !UNICODE


#if (_WIN32_MSI >=  150)

UINT WINAPI MsiGetFileHashA(
	_In_ LPCSTR         szFilePath,  // path to the file
	_In_ DWORD            dwOptions,   // options
	_Inout_ PMSIFILEHASHINFO pHash);     // returned file hash info
UINT WINAPI MsiGetFileHashW(
	_In_ LPCWSTR         szFilePath,  // path to the file
	_In_ DWORD            dwOptions,   // options
	_Inout_ PMSIFILEHASHINFO pHash);     // returned file hash info
#ifdef UNICODE
#define MsiGetFileHash  MsiGetFileHashW
#else
#define MsiGetFileHash  MsiGetFileHashA
#endif // !UNICODE

#endif //(_WIN32_MSI >=  150)

#if (_WIN32_MSI >= 150)
#ifndef _MSI_NO_CRYPTO

HRESULT WINAPI MsiGetFileSignatureInformationA(
	_In_ LPCSTR szSignedObjectPath,                     // path to the signed object
	_In_ DWORD dwFlags,                                   // special extra error case flags
	_Outptr_ PCCERT_CONTEXT  *ppcCertContext,          // returned signer cert context
	_Out_writes_bytes_opt_(*pcbHashData) LPBYTE   pbHashData,   // returned hash buffer, NULL if not desired
	_Inout_opt_ LPDWORD pcbHashData);                     // in/out buffer byte count
HRESULT WINAPI MsiGetFileSignatureInformationW(
	_In_ LPCWSTR szSignedObjectPath,                     // path to the signed object
	_In_ DWORD dwFlags,                                   // special extra error case flags
	_Outptr_ PCCERT_CONTEXT  *ppcCertContext,          // returned signer cert context
	_Out_writes_bytes_opt_(*pcbHashData) LPBYTE   pbHashData,   // returned hash buffer, NULL if not desired
	_Inout_opt_ LPDWORD pcbHashData);                     // in/out buffer byte count
#ifdef UNICODE
#define MsiGetFileSignatureInformation  MsiGetFileSignatureInformationW
#else
#define MsiGetFileSignatureInformation  MsiGetFileSignatureInformationA
#endif // !UNICODE

// By default, when only requesting the certificate context, an invalid hash
// in the digital signature is not a fatal error.  Set this flag in the dwFlags
// parameter to make the TRUST_E_BAD_DIGEST error fatal.
#define MSI_INVALID_HASH_IS_FATAL 0x1

#endif// _MSI_NO_CRYPTO
#endif //(_WIN32_MSI >= 150)

#if (_WIN32_MSI >=  110)

// examine a shortcut, and retrieve its descriptor information 
// if available.

UINT WINAPI MsiGetShortcutTargetA(
	_In_                                   LPCSTR    szShortcutPath,    // full file path for the shortcut
	_Out_writes_opt_(MAX_GUID_CHARS+1)     LPSTR     szProductCode,     // returned product code   - GUID
	_Out_writes_opt_(MAX_FEATURE_CHARS+1)  LPSTR     szFeatureId,       // returned Feature Id.
	_Out_writes_opt_(MAX_GUID_CHARS+1)     LPSTR     szComponentCode);  // returned component code - GUID
UINT WINAPI MsiGetShortcutTargetW(
	_In_                                   LPCWSTR    szShortcutPath,    // full file path for the shortcut
	_Out_writes_opt_(MAX_GUID_CHARS+1)     LPWSTR     szProductCode,     // returned product code   - GUID
	_Out_writes_opt_(MAX_FEATURE_CHARS+1)  LPWSTR     szFeatureId,       // returned Feature Id.
	_Out_writes_opt_(MAX_GUID_CHARS+1)     LPWSTR     szComponentCode);  // returned component code - GUID
#ifdef UNICODE
#define MsiGetShortcutTarget  MsiGetShortcutTargetW
#else
#define MsiGetShortcutTarget  MsiGetShortcutTargetA
#endif // !UNICODE

#endif //(_WIN32_MSI >=  110)

#if (_WIN32_MSI >=  110)

// checks to see if a product is managed
// checks per-machine if called from system context, per-user if from
// user context
UINT WINAPI MsiIsProductElevatedA(
	_In_ LPCSTR szProduct,  // product code
	_Out_ BOOL *pfElevated);  // result   
// checks to see if a product is managed
// checks per-machine if called from system context, per-user if from
// user context
UINT WINAPI MsiIsProductElevatedW(
	_In_ LPCWSTR szProduct,  // product code
	_Out_ BOOL *pfElevated);  // result   
#ifdef UNICODE
#define MsiIsProductElevated  MsiIsProductElevatedW
#else
#define MsiIsProductElevated  MsiIsProductElevatedA
#endif // !UNICODE

#endif //(_WIN32_MSI >=  110)


#if (_WIN32_MSI >= 310)

// Caller notifies us of a user who's been moved and results in a sid change.

UINT WINAPI MsiNotifySidChangeA(_In_ LPCSTR pOldSid,
                                _In_ LPCSTR pNewSid);
UINT WINAPI MsiNotifySidChangeW(_In_ LPCWSTR pOldSid,
                                _In_ LPCWSTR pNewSid);
#ifdef UNICODE
#define MsiNotifySidChange  MsiNotifySidChangeW
#else
#define MsiNotifySidChange  MsiNotifySidChangeA
#endif // !UNICODE

#else // _WIN32_MSI < 310


#endif // (_WIN32_MSI >= 310)


#if (_WIN32_MSI >=  450)

// Start a multi package transaction
UINT WINAPI MsiBeginTransactionA(
	_In_ LPCSTR szName,                         // A name to identify the transaction
	_In_ DWORD dwTransactionAttributes,           // Attributes to control the nature of the transaction
	_Out_ MSIHANDLE* phTransactionHandle,         // Transaction handle
	_Out_ HANDLE* phChangeOfOwnerEvent);          // Change Of Owner Event handle
// Start a multi package transaction
UINT WINAPI MsiBeginTransactionW(
	_In_ LPCWSTR szName,                         // A name to identify the transaction
	_In_ DWORD dwTransactionAttributes,           // Attributes to control the nature of the transaction
	_Out_ MSIHANDLE* phTransactionHandle,         // Transaction handle
	_Out_ HANDLE* phChangeOfOwnerEvent);          // Change Of Owner Event handle
#ifdef UNICODE
#define MsiBeginTransaction  MsiBeginTransactionW
#else
#define MsiBeginTransaction  MsiBeginTransactionA
#endif // !UNICODE

// End a multi package transaction
UINT WINAPI MsiEndTransaction(
	_In_ DWORD dwTransactionState);      // Flags to indicate how to end a transaction

// Join an already created multi package transaction
UINT WINAPI MsiJoinTransaction(
	_In_ MSIHANDLE hTransactionHandle,   //Transaction handle returned by MsiBeginTransaction
	_In_ DWORD dwTransactionAttributes,  //Transaction attributes
	_Out_ HANDLE* phChangeOfOwnerEvent); //Change Of Owner Event handle


#endif // #if (_WIN32_MSI >= 450)


#ifdef __cplusplus
}
#endif

// --------------------------------------------------------------------------
// Error codes for installer access functions
// --------------------------------------------------------------------------

#ifndef ERROR_INSTALL_FAILURE
#define ERROR_INSTALL_USEREXIT      1602L  // User cancel installation.
#define ERROR_INSTALL_FAILURE       1603L  // Fatal error during installation.
#define ERROR_INSTALL_SUSPEND       1604L  // Installation suspended, incomplete.
// LOCALIZE BEGIN:
#define ERROR_UNKNOWN_PRODUCT       1605L  // This action is only valid for products that are currently installed.
// LOCALIZE END
#define ERROR_UNKNOWN_FEATURE       1606L  // Feature ID not registered.
#define ERROR_UNKNOWN_COMPONENT     1607L  // Component ID not registered.
#define ERROR_UNKNOWN_PROPERTY      1608L  // Unknown property.
#define ERROR_INVALID_HANDLE_STATE  1609L  // Handle is in an invalid state.
// LOCALIZE BEGIN:
#define ERROR_BAD_CONFIGURATION     1610L  // The configuration data for this product is corrupt.  Contact your support personnel.
// LOCALIZE END:
#define ERROR_INDEX_ABSENT          1611L  // Component qualifier not present.
// LOCALIZE BEGIN:
#define ERROR_INSTALL_SOURCE_ABSENT 1612L  // The installation source for this product is not available.  Verify that the source exists and that you can access it.
// LOCALIZE END
#define ERROR_PRODUCT_UNINSTALLED   1614L  // Product is uninstalled.
#define ERROR_BAD_QUERY_SYNTAX      1615L  // SQL query syntax invalid or unsupported.
#define ERROR_INVALID_FIELD         1616L  // Record field does not exist.
#endif

// LOCALIZE BEGIN:
#ifndef ERROR_INSTALL_SERVICE_FAILURE
#define ERROR_INSTALL_SERVICE_FAILURE      1601L // The Windows Installer Service could not be accessed. This can occur if the Windows Installer is not correctly installed. Contact your support personnel for assistance.
#define ERROR_INSTALL_PACKAGE_VERSION      1613L // This installation package cannot be installed by the Windows Installer service.  You must install a Windows service pack that contains a newer version of the Windows Installer service.
#define ERROR_INSTALL_ALREADY_RUNNING      1618L // Another program is being installed. Please wait until that installation is complete, and then try installing this software again.
#define ERROR_INSTALL_PACKAGE_OPEN_FAILED  1619L // This installation package could not be opened.  Verify that the package exists and that you can access it, or contact the application vendor to verify that this is a valid Windows Installer package.
#define ERROR_INSTALL_PACKAGE_INVALID      1620L // This installation package could not be opened.  Contact the application vendor to verify that this is a valid Windows Installer package.
#define ERROR_INSTALL_UI_FAILURE           1621L // There was an error starting the Windows Installer service user interface.  Contact your support personnel.
#define ERROR_INSTALL_LOG_FAILURE          1622L // Error opening installation log file.  Verify that the specified log file location exists and is writable.
#define ERROR_INSTALL_LANGUAGE_UNSUPPORTED 1623L // This language of this installation package is not supported by your system.
#define ERROR_INSTALL_PACKAGE_REJECTED     1625L // The system administrator has set policies to prevent this installation.
// LOCALIZE END

#define ERROR_FUNCTION_NOT_CALLED          1626L // Function could not be executed.
#define ERROR_FUNCTION_FAILED              1627L // Function failed during execution.
#define ERROR_INVALID_TABLE                1628L // Invalid or unknown table specified.
#define ERROR_DATATYPE_MISMATCH            1629L // Data supplied is of wrong type.
#define ERROR_UNSUPPORTED_TYPE             1630L // Data of this type is not supported.
// LOCALIZE BEGIN:
#define ERROR_CREATE_FAILED                1631L // The Windows Installer service failed to start.  Contact your support personnel.
// LOCALIZE END:
#endif

// LOCALIZE BEGIN:
#ifndef ERROR_INSTALL_TEMP_UNWRITABLE      
#define ERROR_INSTALL_TEMP_UNWRITABLE      1632L // The Temp folder is on a drive that is full or is inaccessible. Free up space on the drive or verify that you have write permission on the Temp folder.
#endif

#ifndef ERROR_INSTALL_PLATFORM_UNSUPPORTED
#define ERROR_INSTALL_PLATFORM_UNSUPPORTED 1633L // This installation package is not supported by this processor type. Contact your product vendor.
#endif
// LOCALIZE END

#ifndef ERROR_INSTALL_NOTUSED
#define ERROR_INSTALL_NOTUSED              1634L // Component not used on this machine
#endif

// LOCALIZE BEGIN:
#ifndef ERROR_INSTALL_TRANSFORM_FAILURE
#define ERROR_INSTALL_TRANSFORM_FAILURE     1624L // Error applying transforms.  Verify that the specified transform paths are valid.
#endif

#ifndef ERROR_PATCH_PACKAGE_OPEN_FAILED
#define ERROR_PATCH_PACKAGE_OPEN_FAILED    1635L // This patch package could not be opened.  Verify that the patch package exists and that you can access it, or contact the application vendor to verify that this is a valid Windows Installer patch package.
#define ERROR_PATCH_PACKAGE_INVALID        1636L // This patch package could not be opened.  Contact the application vendor to verify that this is a valid Windows Installer patch package.
#define ERROR_PATCH_PACKAGE_UNSUPPORTED    1637L // This patch package cannot be processed by the Windows Installer service.  You must install a Windows service pack that contains a newer version of the Windows Installer service.
#endif

#ifndef ERROR_PRODUCT_VERSION
#define ERROR_PRODUCT_VERSION              1638L // Another version of this product is already installed.  Installation of this version cannot continue.  To configure or remove the existing version of this product, use Add/Remove Programs on the Control Panel.
#endif

#ifndef ERROR_INVALID_COMMAND_LINE
#define ERROR_INVALID_COMMAND_LINE         1639L // Invalid command line argument.  Consult the Windows Installer SDK for detailed command line help.
#endif

// The following three error codes are not returned from MSI version 1.0

#ifndef ERROR_INSTALL_REMOTE_DISALLOWED
#define ERROR_INSTALL_REMOTE_DISALLOWED    1640L // Only administrators have permission to add, remove, or configure server software during a Terminal services remote session. If you want to install or configure software on the server, contact your network administrator.
#endif

// LOCALIZE END

#ifndef ERROR_SUCCESS_REBOOT_INITIATED
#define ERROR_SUCCESS_REBOOT_INITIATED     1641L // The requested operation completed successfully.  The system will be restarted so the changes can take effect.
#endif

// LOCALIZE BEGIN:
#ifndef ERROR_PATCH_TARGET_NOT_FOUND
#define ERROR_PATCH_TARGET_NOT_FOUND       1642L // The upgrade patch cannot be installed by the Windows Installer service because the program to be upgraded may be missing, or the upgrade patch may update a different version of the program. Verify that the program to be upgraded exists on your computer and that you have the correct upgrade patch.
#endif
// LOCALIZE END

// The following two error codes are not returned from MSI version 1.0, 1.1. or 1.2

// LOCALIZE BEGIN:
#ifndef ERROR_PATCH_PACKAGE_REJECTED
#define ERROR_PATCH_PACKAGE_REJECTED       1643L // The patch package is not permitted by software restriction policy.
#endif

#ifndef ERROR_INSTALL_TRANSFORM_REJECTED
#define ERROR_INSTALL_TRANSFORM_REJECTED   1644L // One or more customizations are not permitted by software restriction policy.
#endif
// LOCALIZE END

// The following error code is returned only from MSI post version 2.0

// LOCALIZE BEGIN:
#ifndef ERROR_INSTALL_REMOTE_PROHIBITED
#define ERROR_INSTALL_REMOTE_PROHIBITED    1645L // The Windows Installer does not permit installation from a Remote Desktop Connection.
#endif
// LOCALIZE END

// The following error codes are returned from MSI 3.0 and later.

// LOCALIZE BEGIN:
#ifndef ERROR_PATCH_REMOVAL_UNSUPPORTED
#define ERROR_PATCH_REMOVAL_UNSUPPORTED  1646L // Uninstallation of the patch package is not supported.
#endif
// LOCALIZE END

// LOCALIZE BEGIN:
#ifndef ERROR_UNKNOWN_PATCH
#define ERROR_UNKNOWN_PATCH                1647L // The patch is not applied to this product.
#endif
// LOCALIZE END

// LOCALIZE BEGIN:
#ifndef ERROR_PATCH_NO_SEQUENCE
#define ERROR_PATCH_NO_SEQUENCE            1648L // No valid sequence could be found for the set of patches.
#endif
// LOCALIZE END

// LOCALIZE BEGIN:
#ifndef ERROR_PATCH_REMOVAL_DISALLOWED
#define ERROR_PATCH_REMOVAL_DISALLOWED      1649L // Patch removal was disallowed by policy.
#endif
// LOCALIZE END

// LOCALIZE BEGIN:
#ifndef ERROR_INVALID_PATCH_XML
#define ERROR_INVALID_PATCH_XML             1650L // The XML patch data is invalid.
#endif
// LOCALIZE END

// LOCALIZE BEGIN:
#ifndef ERROR_PATCH_MANAGED_ADVERTISED_PRODUCT
#define ERROR_PATCH_MANAGED_ADVERTISED_PRODUCT 1651L // Windows Installer does not permit patching of managed advertised products. At least one feature of the product must be installed before applying the patch.
#endif
// LOCALIZE END

// The following error codes are returned from MSI 4.0 and later

// LOCALIZE BEGIN:
#ifndef ERROR_INSTALL_SERVICE_SAFEBOOT
#define ERROR_INSTALL_SERVICE_SAFEBOOT     1652L // The Windows Installer service is not accessible in Safe Mode. Please try again when your computer is not in Safe Mode or you can use System Restore to return your machine to a previous good state.
#endif
// LOCALIZE END

// The following error codes are returned from MSI 4.5 and later

// LOCALIZE BEGIN:
#ifndef ERROR_ROLLBACK_DISABLED
#define ERROR_ROLLBACK_DISABLED     1653L // Multi-package transaction cannot be run when rollback is disabled.
#endif
// LOCALIZE END

// The following error codes are returned from MSI 5.0 and later
// LOCALIZE BEGIN:
#ifndef ERROR_INSTALL_REJECTED
#define ERROR_INSTALL_REJECTED      1654L // The app that you are trying to run is not supported on this version of Windows
#endif
// LOCALIZE END


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MSI_H_

