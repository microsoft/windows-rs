/*++

Copyright (c) 1998-2006  Microsoft Corporation

Module Name:

    mxdc.h

Abstract:

    Header file for the Microsoft XPS Document Converter (MXDC),
    which is the conversion component of the
    Microsoft XPS Digital Writer (MXDW).

Environment:

    Windows NT printer drivers

Revision History:


--*/


#ifndef _MXDC_H_
#define _MXDC_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



#ifdef __cplusplus
extern "C" {
#endif

#if (NTDDI_VERSION >= NTDDI_VISTASP1)

#define MXDC_IMAGEABLE_AREA_PROP_NAME_WSTR          (L"MxdcImageableArea")
#define MXDC_IMAGE_COMPRESSION_TYPE_PROP_NAME_WSTR  (L"MxdcImageCompressionType")
#define MXDC_DOTS_PER_INCH_PROP_NAME_WSTR           (L"MxdcDotsPerInch")

#define MXDC_LANDSCAPE_ROTATION_PROP_NAME_WSTR      (L"MxdcLandscapeRotation")

#define MXDC_PHYSICAL_PAPER_DIMENSIONS_NAME_WSTR    (L"MxdcPhysicalPaperDimensions")


typedef enum tagMxdcLandscapeRotationEnums 
{
    MXDC_LANDSCAPE_ROTATE_COUNTERCLOCKWISE_90_DEGREES   =   90,  // 90 Degrees rotation counter-clockwise
    MXDC_LANDSCAPE_ROTATE_NONE         =    0,  // No Rotation
    MXDC_LANDSCAPE_ROTATE_COUNTERCLOCKWISE_270_DEGREES  =  -90   // 270 Degrees rotation counter-clockwise
} MXDC_LANDSCAPE_ROTATION_ENUMS;

/*
 * Microsoft XPS Document Converter Image Types
 */
typedef enum tagMxdcImageTypeEnums
{
    MXDC_IMAGETYPE_JPEGHIGH_COMPRESSION     = 1,  // Lowest Quality, and smallest file size
    MXDC_IMAGETYPE_JPEGMEDIUM_COMPRESSION   = 2,  // Medium Quality, and medium file size
    MXDC_IMAGETYPE_JPEGLOW_COMPRESSION      = 3,  // High Quality, and large file size
    MXDC_IMAGETYPE_PNG                      = 4   // Highest Quality, and largest file size
} MXDC_IMAGE_TYPE_ENUMS;

//
// MxdcGetPDEVAdjustment is exported by XPSDrv configuration module and called by the
// Microsoft XPS Document Converter (MXDC) to supply printer configuration data in the form of a property bag.
//
HRESULT WINAPI
MxdcGetPDEVAdjustment(
    _In_ HANDLE                                                           hPrinter,
    ULONG                                                                 cbDevMode,
    _In_reads_bytes_(cbDevMode) const DEVMODE                                  *pDevMode,
    ULONG                                                                 cbIn,
    _In_reads_bytes_opt_(cbIn) const VOID                                      *pvIn,
    ULONG                                                                 cbPrintPropertiesCollection,
    _Inout_updates_bytes_(cbPrintPropertiesCollection) PrintPropertiesCollection *pPrintPropertiesCollection
    );

#endif // (NTDDI_VERSION >= NTDDI_VISTASP1)

#define MXDC_ESCAPE                        4122

/*
 * Operation Codes used with the MXDC Escape (MXDC_ESCAPE)
 */

 /* MXDC Destination File Name Query Escape */
#define MXDCOP_GET_FILENAME                   14


/* MXDC Document Sequence PrintTicket Escape */
#define MXDCOP_PRINTTICKET_FIXED_DOC_SEQ      22

/* MXDC Fixed Document PrintTicket Escape */
#define MXDCOP_PRINTTICKET_FIXED_DOC          24

/* MXDC Fixed Page PrintTicket Escape */
#define MXDCOP_PRINTTICKET_FIXED_PAGE         26

/* MXDC S0 Page Pass-through Escape */
#define MXDCOP_SET_S0PAGE                     28

/* MXDC S0 Page Resource Pass-through */
#define MXDCOP_SET_S0PAGE_RESOURCE            30

/* MXDC Full XPS Pass-through Mode */
#define MXDCOP_SET_XPSPASSTHRU_MODE           32



#pragma pack(1)

/*
 * All of the MXDC's Escapes must start with this structure.
 */
typedef struct tagMxdcEscapeHeader
{
    ULONG cbInput;
    ULONG cbOutput;
    ULONG opCode;
} MXDC_ESCAPE_HEADER_T, * P_MXDC_ESCAPE_HEADER_T;


typedef struct tagMxdcGetFileNameData
{
    ULONG   cbOutput;
    wchar_t wszData[1];
} MXDC_GET_FILENAME_DATA_T, * P_MXDC_GET_FILENAME_DATA_T;

/*
 * Passthrough and entire S0 Page
 */
typedef struct tagMxdcS0PageData
{
    DWORD  dwSize;
    BYTE   bData[1];
} MXDC_S0PAGE_DATA_T, * P_MXDC_S0PAGE_DATA_T;


/*
 * Microsoft XPS Document Writer Resource Enumerators
 */
typedef enum tagMxdcS0PageEnums
{
    MXDC_RESOURCE_TTF            = 0, /* TrueType (OpenType) font                */
    MXDC_RESOURCE_JPEG           = 1, /* JPEG Image                              */
    MXDC_RESOURCE_PNG            = 2, /* PNG Image                               */
    MXDC_RESOURCE_TIFF           = 3, /* TIFF Image                              */
    MXDC_RESOURCE_WDP            = 4, /* Windows Media Photo Image               */
    MXDC_RESOURCE_DICTIONARY     = 5, /* Remote Resource Dictionary Pass-through */
    MXDC_RESOURCE_ICC_PROFILE    = 6, /* ICC Profile Pass-through                */
    MXDC_RESOURCE_JPEG_THUMBNAIL = 7, /* JPEG Thumbnail Pass-through             */
    MXDC_RESOURCE_PNG_THUMBNAIL  = 8, /* PNG Thumbnail Pass-through              */
    MXDC_RESOURCE_MAX                 /* Maximum Resource Count for validation   */
} MXDC_S0_PAGE_ENUMS;


typedef struct tagMxdcXpsS0PageResource
{
    DWORD dwSize;                /* Size of the structue and data        */
    DWORD dwResourceType;        /* Resource Type, "MXDC_S0_PAGE_ENUMS"  */
    BYTE  szUri[MAX_PATH];       /* URI sting of the resource            */
    DWORD dwDataSize;            /* The resource's data byte size        */
    BYTE  bData[1];              /* The resource's data                  */
} MXDC_XPS_S0PAGE_RESOURCE_T, * P_MXDC_XPS_S0PAGE_RESOURCE_T;


typedef struct tagMxdcPrintTicketPassthrough
{
    DWORD dwDataSize;            /* The Print Ticket's data byte size  */
    BYTE  bData[1];              /* The Print Ticket's data            */
} MXDC_PRINTTICKET_DATA_T, * P_MXDC_PRINTTICKET_DATA_T;


/***********************************************************
 * MXDC Escape Data Structures
 *
 */

typedef struct tagMxdcPrintTicketEscape
{
    MXDC_ESCAPE_HEADER_T     mxdcEscape;
    MXDC_PRINTTICKET_DATA_T  printTicketData;
} MXDC_PRINTTICKET_ESCAPE_T, * P_MXDC_PRINTTICKET_ESCAPE_T;

/*
 * The application will use this to pass the entire
 * XPS Page through the MXDC.
 */
typedef struct tagMxdcS0PagePassthroughEscape
{
    MXDC_ESCAPE_HEADER_T  mxdcEscape;
    MXDC_S0PAGE_DATA_T    xpsS0PageData;
} MXDC_S0PAGE_PASSTHROUGH_ESCAPE_T, * P_MXDC_S0PAGE_PASSTHROUGH_ESCAPE_T;

/*
 * This works in concert with MXDC_S0PAGE_PASSTHROUGH_ESCAPE_T.
 * This structure allows the respective resource of the
 * passed through XPS page to be saved. Some of the resource types
 * are as follows: JPEGs, PNGs, TIFFs, WDPs, Thumbnails, TTFs, etc..
 * The full list of resource types are described in the enum,
 * MXDC_S0_PAGE_ENUMS.
 */
typedef struct tagMxdcS0PageResourceEscape
{
    MXDC_ESCAPE_HEADER_T        mxdcEscape;
    MXDC_XPS_S0PAGE_RESOURCE_T  xpsS0PageResourcePassthrough;
} MXDC_S0PAGE_RESOURCE_ESCAPE_T, * P_MXDC_S0PAGE_RESOURCE_ESCAPE_T;


#pragma pack()





#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //_MXDC_H_







