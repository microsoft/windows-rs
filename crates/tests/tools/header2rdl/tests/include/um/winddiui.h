/*++

Copyright (c) 1990-2004  Microsoft Corporation

Module Name:

    WinDDIUI.h

Abstract:

    Header file for the UI portion of printer drivers.

Revision History:

--*/
#ifndef _WINDDIUI_
#define _WINDDIUI_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <compstui.h>

#ifdef __cplusplus
extern "C" {
#endif

#if (NTDDI_VERSION <= NTDDI_WIN2K)

typedef struct _PRINTPROCESSOR_CAPS_1 {
    DWORD    dwLevel;
    DWORD    dwNupOptions;
    DWORD    dwPageOrderFlags;
    DWORD    dwNumberOfCopies;
} PRINTPROCESSOR_CAPS_1, *PPRINTPROCESSOR_CAPS_1;

#endif // (NTDDI_VERSION <= NTDDI_WIN2K)

//
// DrvDevicePropertySheets replace previous version of PrinterProperties
//

LONG WINAPI
DrvDevicePropertySheets(
    _In_opt_ PPROPSHEETUI_INFO   pPSUIInfo,
    LPARAM              lParam
    );

typedef struct _DEVICEPROPERTYHEADER {
    WORD    cbSize;
    WORD    Flags;
    HANDLE  hPrinter;
    LPTSTR  pszPrinterName;
} DEVICEPROPERTYHEADER, *PDEVICEPROPERTYHEADER;

#define DPS_NOPERMISSION    0x0001

//
// For document properties replace DocumentProperties.
//
// Note: if pPSUIInfo is NULL then the call need not to display any dialog
//       boxes (Ignored the DC_PROMPT bit in the fMode, the lParam in this case
//       is a pointer to DOCUMENTPROPERTYHEADER
//

LONG WINAPI
DrvDocumentPropertySheets(
    _In_opt_ PPROPSHEETUI_INFO   pPSUIInfo,
    LPARAM              lParam
    );

typedef struct _DOCUMENTPROPERTYHEADER {
    WORD        cbSize;
    WORD        Reserved;
    HANDLE      hPrinter;
    LPTSTR      pszPrinterName;
    PDEVMODE    pdmIn;
    PDEVMODE    pdmOut;
    DWORD       cbOut;
    DWORD       fMode;
} DOCUMENTPROPERTYHEADER, *PDOCUMENTPROPERTYHEADER;

#define DM_ADVANCED         0x10
#define DM_NOPERMISSION     0x20
#define DM_USER_DEFAULT     0x40

#if (NTDDI_VERSION >= NTDDI_WIN8)
/// Used by DocumentProperties() to determine that when DM_PROMPT is set,
/// the prompt should be non-modal.
#define DM_PROMPT_NON_MODAL         0x40000000
/// Used between driver components to invalidate caches internal to driver.
#define DM_INVALIDATE_DRIVER_CACHE  0x20000000
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define DM_RESERVED         0x80000000
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

// Devmode conversion function used by GetPrinter and SetPrinter
_Success_(return == 1)
BOOL WINAPI
DrvConvertDevMode(
    _In_                     LPTSTR   pPrinterName,
    _In_                     PDEVMODE pdmIn,
    _Out_writes_bytes_(*pcbNeeded) PDEVMODE pdmOut,
    _Inout_                  PLONG    pcbNeeded,
    _In_                     DWORD    fMode
    );

#define CDM_CONVERT         0x01
#define CDM_CONVERT351      0x02
#define CDM_DRIVER_DEFAULT  0x04


//
// This is for DevQueryPrintEx()
//

typedef struct _DEVQUERYPRINT_INFO {
    WORD    cbSize;         // size of this structure in bytes
    WORD    Level;          // Level of this info, 1 for this version
    HANDLE  hPrinter;       // handle to the printer for the query
    DEVMODE *pDevMode;      // pointer to the DEVMODE for this job.
    LPWSTR  pszErrorStr;    // pointer to the error string buffer.
    DWORD   cchErrorStr;    // count characters of pwErrorStr passed.
    DWORD   cchNeeded;      // count characters of pwErrorStr needed.
    } DEVQUERYPRINT_INFO, *PDEVQUERYPRINT_INFO;

BOOL WINAPI
DevQueryPrintEx(
    _Inout_ PDEVQUERYPRINT_INFO pDQPInfo
    );

//
// This for the DrvUpgradePrinter
//

typedef struct _DRIVER_UPGRADE_INFO_1 {
    LPTSTR  pPrinterName;
    LPTSTR  pOldDriverDirectory;
} DRIVER_UPGRADE_INFO_1, *PDRIVER_UPGRADE_INFO_1;

typedef struct _DRIVER_UPGRADE_INFO_2 {
    LPTSTR   pPrinterName;
    LPTSTR   pOldDriverDirectory;
    DWORD    cVersion;
    LPTSTR   pName;
    LPTSTR   pEnvironment;
    LPTSTR   pDriverPath;
    LPTSTR   pDataFile;
    LPTSTR   pConfigFile;
    LPTSTR   pHelpFile;
    LPTSTR   pDependentFiles;
    LPTSTR   pMonitorName;
    LPTSTR   pDefaultDataType;
    LPTSTR   pszzPreviousNames;
} DRIVER_UPGRADE_INFO_2, *PDRIVER_UPGRADE_INFO_2;

BOOL WINAPI
DrvUpgradePrinter(
    _In_range_(1, 2)  DWORD   Level,
    _When_(Level == 1, _In_reads_bytes_opt_(sizeof(DRIVER_UPGRADE_INFO_1)))
    _When_(Level == 2, _In_reads_bytes_opt_(sizeof(DRIVER_UPGRADE_INFO_2)))
                   LPBYTE  pDriverUpgradeInfo
    );

//
// DrvDocumentEvent
//
//
//  Defines and proto-types for hooking GDI printer management functions
//
//  return values: -1 means error, 0 means not supported function
//
//  CreateDCPre must return > 0 or none of the others will be called.
//
//
//  CREATEDCPRE
//      return failure from CreateDC if this fails, CREATEDCPOST not called
//      bIC - TRUE if came from CreateIC
//      output devmode - this is the devmode that actualy gets passed to the
//      server side driver.  Any data needed in EnablePDEV should be passed
//      as part of the DriverExtra.
//
//  CREATEDCPOST
//      return value is ignored
//      the hdc will be 0 if something failed since CREATEDCPRE
//      The input buffer contains a pointer to the devmode returned in the
//      CREATEDCPRE output buffer
//
//  RESETDCPRE
//      return failure from ResetDC if this fails, CREATEDCPOST not called
//
//  RESETDCPOST
//      return value is ignored
//
//  STARTDOCPRE
//      return failure form StartDoc if this fails, driver not called
//
//  STARTDOCPOST
//      return failure form StartDoc if this fails, driver already called.
//      AbortDoc() called.
//
//  STARTPAGE
//      return failure form EndPage if this fails, driver not called
//
//  ENDPAGE
//      return value is ignored, DrvEndPage always called
//
//  ENDDOCPRE
//      return value is ignored, DrvEndDoc always called
//
//  ENDDOCPOST
//      return value is ignored, DrvEndDoc has alreadybeen called
//
//  ABORTDOC
//      return value is ignored
//
//  DELETEDC
//      return value is ignored
//
//  EXTESCAPE
//      return value is ignored
//      The input buffer includes the ExtEscape escape value, size of input
//      buffer to ExtEscape and the input buffer passed in.
//      The output buffer is just the buffer that was passed to ExtEscape
//
//  DOCUMENTEVENT_SPOOLED
//      This flag is added to the iEsc value if the document is being spooled
//      to a metafile rather than going direct.  Note that if this bit is set
//
//

#define DOCUMENTEVENT_EVENT(iEsc) (LOWORD(iEsc))
#define DOCUMENTEVENT_FLAGS(iEsc) (HIWORD(iEsc))

#if (NTDDI_VERSION >= NTDDI_WINXP)

typedef struct _DOCEVENT_FILTER {
    UINT    cbSize;
    UINT    cElementsAllocated;
    UINT    cElementsNeeded;
    UINT    cElementsReturned;
    DWORD   aDocEventCall[ANYSIZE_ARRAY];
} DOCEVENT_FILTER, *PDOCEVENT_FILTER;

//
// Add structures usded for each DocumentEvent calls
//

typedef struct _DOCEVENT_CREATEDCPRE {
    PWSTR       pszDriver;
    PWSTR       pszDevice;
    PDEVMODEW   pdm;
    BOOL        bIC;
} DOCEVENT_CREATEDCPRE, *PDCEVENT_CREATEDCPRE;

typedef struct _DOCEVENT_ESCAPE {
    int    iEscape;
    int    cjInput;
    PVOID  pvInData;
} DOCEVENT_ESCAPE, *PDOCEVENT_ESCAPE;

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

//
// Escape codes for DrvDocumentEvent
//

#define DOCUMENTEVENT_FIRST         1   // Inclusive lower bound
#define DOCUMENTEVENT_CREATEDCPRE   1   // in-pszDriver, pszDevice, pdm, bIC, out-ppdm
#define DOCUMENTEVENT_CREATEDCPOST  2   // in-ppdm
#define DOCUMENTEVENT_RESETDCPRE    3   // in-pszDriver, pszDevice, pdm, out-ppdm
#define DOCUMENTEVENT_RESETDCPOST   4   // in-ppdm
#define DOCUMENTEVENT_STARTDOC      5   // none
#define DOCUMENTEVENT_STARTDOCPRE   5   // none
#define DOCUMENTEVENT_STARTPAGE     6   // none
#define DOCUMENTEVENT_ENDPAGE       7   // none
#define DOCUMENTEVENT_ENDDOC        8   // none
#define DOCUMENTEVENT_ENDDOCPRE     8   // none
#define DOCUMENTEVENT_ABORTDOC      9   // none
#define DOCUMENTEVENT_DELETEDC     10   // none
#define DOCUMENTEVENT_ESCAPE       11   // in-iEsc, cjInBuf, inBuf, out-outBuf
#define DOCUMENTEVENT_ENDDOCPOST   12   // none
#define DOCUMENTEVENT_STARTDOCPOST 13   // none


#if (NTDDI_VERSION >= NTDDI_VISTA)

#define DOCUMENTEVENT_QUERYFILTER  14   // none

//
// Escape code for XPS document events
//
#define DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRE                 1
#define DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRE                         2
#define DOCUMENTEVENT_XPS_ADDFIXEDPAGEEPRE                            3
#define DOCUMENTEVENT_XPS_ADDFIXEDPAGEPOST                            4
#define DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPOST                        5
#define DOCUMENTEVENT_XPS_CANCELJOB                                   6
#define DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPRE      7
#define DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPRE              8
#define DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPRE                  9
#define DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPOST                 10
#define DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPOST             11
#define DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPOST     12
#define DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPOST                13


#define DOCUMENTEVENT_LAST         15   // Non-inclusive upper bound

#elif (NTDDI_VERSION >= NTDDI_WINXP)

#define DOCUMENTEVENT_QUERYFILTER  14   // none
#define DOCUMENTEVENT_LAST         15   // Non-inclusive upper bound

#else // (NTDDI_VERSION >= NTDDI_WINXP)

#define DOCUMENTEVENT_LAST         14   // Non-inclusive upper bound

#endif // NTDDI_VERSION

#define DOCUMENTEVENT_SPOOLED   0x10000

//
// Return values for DrvDocumentEvent
//

#define DOCUMENTEVENT_SUCCESS     1
#define DOCUMENTEVENT_UNSUPPORTED 0
#define DOCUMENTEVENT_FAILURE     -1

int WINAPI
DrvDocumentEvent(
    HANDLE  hPrinter,
    HDC     hdc,
    int     iEsc,
    ULONG   cbIn,
    _In_reads_bytes_(cbIn) PVOID   pvIn,
    ULONG   cbOut,
    _Out_writes_bytes_(cbOut) PVOID   pvOut
);


//
// DrvPrinterEvent
//
//
//    DrvPrinterEvent are called by the print subsystem when events
//    happen that might be of interest to a printer driver
//    The only event which should be implemented in the driver
//    is PRITNER_EVENT_INITIALIZE so that default settings are created
//    for the printer.
//
// PRINTER_EVENT_CONFIGURATION_CHANGE
//        Reserve it for future use.Xerox is already using it.
//
// PRINTER_EVENT_ADD_CONNECTION
//        return value ignored
//        Called after a successful AddPrinterConnection API
//        in the context of the calling app
//        lParam NULL
//
// PRINTER_EVENT_DELETE_CONNECTION
//        return value ignored
//        Called Before DeletePrinterConnect API
//        in the context of the calling app
//        lParam NULL
//
// PRINTER_EVENT_ADD_CONNECTION_NO_UI
//        return value ignored
//        Called after a successful AddPrinterConnection API
//        in the context of the spooler process
//        lParam NULL
//
// PRINTER_EVENT_DELETE_CONNECTION_NO_UI
//        return value ignored
//        Called Before DeletePrinterConnection API
//        in the context of the spooler process
//        lParam NULL
//
// PRINTER_EVENT_INITIALIZE
//        Called when a printer is created for the driver to
//        initialize its registry settings
//        Called in the spooler process
//        lParam NULL
//
// PRINTER_EVENT_DELETE
//        Called when a printer is about to be deleted
//        Called in the spooler process
//        lParam NULL
//
// PRINTER_EVENT_CACHE_REFRESH
//        return value ignored
//        called in spooler process
//        No UI
//        called when spooler detects that something has
//        changed in the workstaion cache or when establishing
//        the cache.
//        allows driver to update any private cache data
//        ( such as font files etc. )
//
// PRINTER_EVENT_CACHE_DELETE
//        return value ignored
//        called in spooler process
//        No UI
//        called when spooler is deleting a cached printer
//        allows printer driver to delete anything it has
//        cached
//
// PRINTER_EVENT_ATTRIBUTES_CHANGED
//        return value ignored
//        No UI
//        Called when the printer attribute bits for a given
//        printer have changed. Allows the driver to respond
//        appropriately.
//        lParam is a pointer to a PRINTER_EVENT_ATTRIBUTES_INFO
//        structure.
//
// PRINTER_EVENT_FLAG_NO_UI
//        Do not bring up UI when this flag it ON
//

#if (NTDDI_VERSION >= NTDDI_VISTA)
// PRINTER_EVENT_CONFIGURATION_UPDATE
//        Called when the printer configuration has changed.
//        lParam is a pointer to a UNICODE string that contains
//        bi-di notification formated according to the Bidi
//        Notificatin Schema.
//
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

//
// DrvPrinterEvent DriverEvent code
//

#if (NTDDI_VERSION >= NTDDI_WINXP)
#define PRINTER_EVENT_CONFIGURATION_CHANGE      0
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#define PRINTER_EVENT_ADD_CONNECTION            1
#define PRINTER_EVENT_DELETE_CONNECTION         2
#define PRINTER_EVENT_INITIALIZE                3
#define PRINTER_EVENT_DELETE                    4
#define PRINTER_EVENT_CACHE_REFRESH             5
#define PRINTER_EVENT_CACHE_DELETE              6

#if (NTDDI_VERSION >= NTDDI_WINXP)
#define PRINTER_EVENT_ATTRIBUTES_CHANGED        7
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define PRINTER_EVENT_CONFIGURATION_UPDATE      8
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define PRINTER_EVENT_ADD_CONNECTION_NO_UI      9
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define PRINTER_EVENT_DELETE_CONNECTION_NO_UI   10
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

//
// DrvPrinterEvent Flags
//

#define PRINTER_EVENT_FLAG_NO_UI        0x00000001

#if (NTDDI_VERSION >= NTDDI_WINXP)

//
// lParam of PRINTER_EVENT_ATTRIBUTES_CHANGED points to this structure.
//
typedef struct _PRINTER_EVENT_ATTRIBUTES_INFO {
    DWORD       cbSize;
    DWORD       dwOldAttributes;
    DWORD       dwNewAttributes;
} PRINTER_EVENT_ATTRIBUTES_INFO, *PPRINTER_EVENT_ATTRIBUTES_INFO;

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

BOOL WINAPI
DrvPrinterEvent(
    _In_ LPWSTR  pPrinterName,
    int     DriverEvent,
    DWORD   Flags,
    LPARAM  lParam
);

//
// DrvDriverEvent is called when any version of the printer driver is deleted.
//
#define DRIVER_EVENT_INITIALIZE        0x00000001
#define DRIVER_EVENT_DELETE            0x00000002

BOOL WINAPI
DrvDriverEvent(
                            DWORD   dwDriverEvent,
    _In_range_(1, 3)        DWORD   dwLevel,
    _When_(dwLevel == 1, _In_reads_bytes_opt_(sizeof(DRIVER_INFO_1))) 
    _When_(dwLevel == 2, _In_reads_bytes_opt_(sizeof(DRIVER_INFO_2)))
    _When_(dwLevel == 3, _In_reads_bytes_opt_(sizeof(DRIVER_INFO_3)))
                            LPBYTE  pDriverInfo,
                            LPARAM  lParam
);

// Print processor capabilities for the driver.
#define BORDER_PRINT                   0x00000000        // default
#define NO_BORDER_PRINT                0x00000001

#if (NTDDI_VERSION <= NTDDI_WIN2K)

#define NORMAL_PRINT                   0x00000000        // default
#define REVERSE_PRINT                  0x00000001

#endif // (NTDDI_VERSION <= NTDDI_WIN2K)

#define BOOKLET_PRINT                  0x00000002

#define NO_COLOR_OPTIMIZATION          0x00000000        // default
#define COLOR_OPTIMIZATION             0x00000001

typedef struct _ATTRIBUTE_INFO_1 {
    DWORD    dwJobNumberOfPagesPerSide;
    DWORD    dwDrvNumberOfPagesPerSide;
    DWORD    dwNupBorderFlags;
    DWORD    dwJobPageOrderFlags;
    DWORD    dwDrvPageOrderFlags;
    DWORD    dwJobNumberOfCopies;
    DWORD    dwDrvNumberOfCopies;
} ATTRIBUTE_INFO_1, *PATTRIBUTE_INFO_1;

typedef struct _ATTRIBUTE_INFO_2 {
    DWORD    dwJobNumberOfPagesPerSide;
    DWORD    dwDrvNumberOfPagesPerSide;
    DWORD    dwNupBorderFlags;
    DWORD    dwJobPageOrderFlags;
    DWORD    dwDrvPageOrderFlags;
    DWORD    dwJobNumberOfCopies;
    DWORD    dwDrvNumberOfCopies;
    DWORD    dwColorOptimization;           // Added for monochrome optimization
} ATTRIBUTE_INFO_2, *PATTRIBUTE_INFO_2;

#ifndef __ATTRIBUTE_INFO_3__
#define __ATTRIBUTE_INFO_3__
typedef struct _ATTRIBUTE_INFO_3 {
    DWORD    dwJobNumberOfPagesPerSide;
    DWORD    dwDrvNumberOfPagesPerSide;
    DWORD    dwNupBorderFlags;
    DWORD    dwJobPageOrderFlags;
    DWORD    dwDrvPageOrderFlags;
    DWORD    dwJobNumberOfCopies;
    DWORD    dwDrvNumberOfCopies;
    DWORD    dwColorOptimization;           // Added for monochrome optimization
    short    dmPrintQuality;                // Added for monochrome optimization
    short    dmYResolution;                 // Added for monochrome optimization
} ATTRIBUTE_INFO_3, *PATTRIBUTE_INFO_3;

#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)

#ifndef __ATTRIBUTE_INFO_4__
#define __ATTRIBUTE_INFO_4__
    typedef struct _ATTRIBUTE_INFO_4 {
        DWORD    dwJobNumberOfPagesPerSide;
        DWORD    dwDrvNumberOfPagesPerSide;
        DWORD    dwNupBorderFlags;
        DWORD    dwJobPageOrderFlags;
        DWORD    dwDrvPageOrderFlags;
        DWORD    dwJobNumberOfCopies;
        DWORD    dwDrvNumberOfCopies;
        DWORD    dwColorOptimization;           // Added for monochrome optimization
        short    dmPrintQuality;                // Added for monochrome optimization
        short    dmYResolution;                 // Added for monochrome optimization

        // _ATTRIBUTE_INFO_4 specific fields.
        DWORD    dwDuplexFlags;
        DWORD    dwNupDirection;
        DWORD    dwBookletFlags;
        DWORD    dwScalingPercentX;            // Scaling percentage in X direction.
        DWORD    dwScalingPercentY;            // Scaling percentage in Y direction.
    } ATTRIBUTE_INFO_4, *PATTRIBUTE_INFO_4;

    //dwDuplexFlags
    // The below flag tells print processor to flip page order within sheet
    // while printing reverse duplex.
    // e.g. Instead of playing pages in order 4,3,2,1, play them 3,4,1,2
    #define REVERSE_PAGES_FOR_REVERSE_DUPLEX ( 0x00000001      )  
    #define DONT_SEND_EXTRA_PAGES_FOR_DUPLEX ( 0x00000001 << 1 )  // 0x00000002

    //Flags for dwNupDirection.
    #define RIGHT_THEN_DOWN                  ( 0x00000001      )  // 0x00000001
    #define DOWN_THEN_RIGHT                  ( 0x00000001 << 1 )  // 0x00000002
    #define LEFT_THEN_DOWN                   ( 0x00000001 << 2 )  // 0x00000004
    #define DOWN_THEN_LEFT                   ( 0x00000001 << 3 )  // 0x00000008

    //dwBookletFlags
    #define BOOKLET_EDGE_LEFT                ( 0x00000000 )
    #define BOOKLET_EDGE_RIGHT               ( 0x00000001 )

#endif

#endif // (NTDDI_VERSION >= NTDDI_VISTA)


//
// DrvQueryJobAttributes is called by the spooler(print processor) to get information
// about the printing options used with the job. These options include N-up and reverse
// order printing.
//
BOOL WINAPI
DrvQueryJobAttributes(
    _In_                HANDLE      hPrinter,
    _In_                PDEVMODE    pDevMode,
    _In_range_(1, 4)    DWORD       dwLevel,
    _When_(dwLevel == 1, _Out_writes_bytes_(sizeof(ATTRIBUTE_INFO_1))) 
    _When_(dwLevel == 2, _Out_writes_bytes_(sizeof(ATTRIBUTE_INFO_2)))
    _When_(dwLevel == 3, _Out_writes_bytes_(sizeof(ATTRIBUTE_INFO_3)))
    _When_(dwLevel == 4, _Out_writes_bytes_(sizeof(ATTRIBUTE_INFO_4)))
                        LPBYTE      lpAttributeInfo
);

//
// DrvQueryColorProfile is called by the GDI (graphics device interface) to get information
// about the default color profile for the given DEVMODE, used with ICM (image color
// management).
//
BOOL WINAPI
DrvQueryColorProfile(
    _In_    HANDLE      hPrinter,
    _In_    PDEVMODEW   pdevmode,
    _In_    ULONG       ulQueryMode,
    _Out_writes_(*pcbProfileData) VOID       *pvProfileData,
    _Inout_ ULONG      *pcbProfileData,
    _Out_   FLONG      *pflProfileData
);

// The value for ulQueryMode
#define QCP_DEVICEPROFILE   0x0000
#define QCP_SOURCEPROFILE   0x0001

// The flags for pflProfileData.
#define QCP_PROFILEMEMORY  0x0001 // The pvProfileData points the color profile data itself.
#define QCP_PROFILEDISK    0x0002 // The pvProfileData points the color profile file name in Unicode.

//
//  User Mode Printer Driver DLL,
//
//  Note on hPrinter passed into DrvSplStartDoc() and subsequent
//  DrvSplxxx calls
//
//
//  A. If you have DrvSplxxxx calls in separate DLL and link it with
//     spoolss.lib.
//
//      * The hPrinter will be valid for any call to the spooler, such as
//        WritePrinter(), GetPrinterData()
//
//      * To do this you must
//
//          1. Have separate DLL for all DrvSplxxx functions.
//          2. Put this DLL name into your dependency files (inf).
//          3. link to spoolss.lib rather than winspool.lib
//          4. Use SetPrinterData() with SPLPRINTER_USER_MODE_PRINTER_DRIVER
//             as key name, and this DLL name as data.
//          5. Call any spooler functions linked from spoolss.lib
//
//
//
//  B. If you have DrvSplxxx calls located in your printer driver UI DLL and
//     linked with winspool.lib
//
//      * The hPrinter is NOT valid for any spooler calls, such as
//        WritePrinter(), GetPrinterData() from within the DrvSplxxx driver
//        functions.
//
//      * To do any spooler call from inside of DrvSplxxxx function you must
//        do the following
//
//          1. hSpoolSS = LoadLibrary("spoolss.dll");
//          2. pfn = GetProcAddress("WritePrinter") or whatever the spooler
//             functions you wish to call
//          3. Call the pfn function pointer returned from GetProcAddress()
//          4. FreeLibrary(hSpoolSS);
//
//
//  The A method is recommended.
//
//
//  If a UserModePrinterDriver DLL is created the following routines are
//  required or optional
//
//  Required Routines
//      DrvSplStartDoc
//      DrvSplWritePrinter
//      DrvSplEndDoc
//      DrvSplClose
//
//
//  Optional Routines
//      DrvSplStart
//      DrvSplEndPage
//      DrvSplAbort
//
//


__drv_preferredFunction("DrvDocumentEvent", "Obsolete")
HANDLE WINAPI
DrvSplStartDoc(
    HANDLE  hPrinter,
    DWORD   JobId
);


__drv_preferredFunction("DrvDocumentEvent", "Obsolete")
BOOL WINAPI
DrvSplWritePrinter(
    HANDLE  hDriver,
    LPVOID  pBuf,
    DWORD   cbBuf,
    LPDWORD pcWritten
);

__drv_preferredFunction("DrvDocumentEvent", "Obsolete")
VOID WINAPI
DrvSplEndDoc(
    HANDLE  hDriver
);


__drv_preferredFunction("DrvDocumentEvent", "Obsolete")
VOID WINAPI
DrvSplClose(
    HANDLE  hDriver
);


__drv_preferredFunction("DrvDocumentEvent", "Obsolete")
BOOL WINAPI
DrvSplStartPage(
    HANDLE  hDriver
);

__drv_preferredFunction("DrvDocumentEvent", "Obsolete")
BOOL WINAPI
DrvSplEndPage(
    HANDLE  hDriver
);

__drv_preferredFunction("DrvDocumentEvent", "Obsolete")
VOID WINAPI
DrvSplAbort(
    HANDLE  hDriver
);

#if (NTDDI_VERSION >= NTDDI_WS03)

DWORD
DrvSplDeviceCaps(
                                                    HANDLE      hPrinter,
    _In_                                            PWSTR       pszDeviceName,
                                                    WORD        Capability,
    _At_((TCHAR*)pOutput, _Out_writes_opt_(cchBufSize)) PVOID   pOutput,
                                                    DWORD       cchBufSize,
    _In_opt_                                        PDEVMODE    pDevmode
    );

DWORD
WINAPI
DrvDeviceCapabilities(
    HANDLE      hPrinter,
    _In_ PWSTR       pszDeviceName,
    WORD        Capability,
    _Out_writes_to_opt_(_Inexpressible_("Size of the buffer is dependent on Capability"),_Inexpressible_("Amount filled is dependent on Capability")) PVOID       pOutput,
    _In_opt_ PDEVMODE    pDevmode
    );

#endif // (NTDDI_VERSION >= NTDDI_WS03)

//
//  Printer Attribute
//  Use with SetPrinterData to define UMPD.DLL
//

#define SPLPRINTER_USER_MODE_PRINTER_DRIVER     TEXT("SPLUserModePrinterDriver")

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  /* !_WINDDIUI_ */
