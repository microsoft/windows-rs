/*==========================================================================;
 *
 *  Copyright (C) Microsoft Corporation.  All Rights Reserved.
 *
 *  File:       dxmini.h
 *  Content:    Miniport support for DirectDraw DXAPI.  This file is
 *              analagous to Win95's ddkmmini.h.
 *
 ***************************************************************************/

#include <winapifamily.h>

#ifndef __DXMINI_INCLUDED__
#define __DXMINI_INCLUDED__

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

DEFINE_GUID(GUID_DxApi, 0x8a79bef0, 0xb915, 0x11d0, 0x91, 0x44, 0x08, 0x00, 0x36, 0xd2, 0xef, 0x02);

#ifndef GUID_DEFS_ONLY
/*============================================================================
 *
 * MDL structure for handling pagelocked memory.  This is copied from WDM.H
 *
 *==========================================================================*/

#ifndef MDL_MAPPING_FLAGS

    typedef struct _MDL {
        struct _MDL *MdlNext;
        short MdlSize;
        short MdlFlags;
        struct _EPROCESS *Process;
        ULONG *lpMappedSystemVa;
        ULONG *lpStartVa;
        ULONG ByteCount;
        ULONG ByteOffset;
    } MDL;
    typedef MDL *PMDL;

    #define MDL_MAPPED_TO_SYSTEM_VA     0x0001
    #define MDL_PAGES_LOCKED            0x0002
    #define MDL_SOURCE_IS_NONPAGED_POOL 0x0004
    #define MDL_ALLOCATED_FIXED_SIZE    0x0008
    #define MDL_PARTIAL                 0x0010
    #define MDL_PARTIAL_HAS_BEEN_MAPPED 0x0020
    #define MDL_IO_PAGE_READ            0x0040
    #define MDL_WRITE_OPERATION         0x0080
    #define MDL_PARENT_MAPPED_SYSTEM_VA 0x0100
    #define MDL_LOCK_HELD               0x0200
    #define MDL_SCATTER_GATHER_VA       0x0400
    #define MDL_IO_SPACE                0x0800
    #define MDL_NETWORK_HEADER          0x1000
    #define MDL_MAPPING_CAN_FAIL        0x2000
    #define MDL_ALLOCATED_MUST_SUCCEED  0x4000
    #define MDL_64_BIT_VA               0x8000

    #define MDL_MAPPING_FLAGS (MDL_MAPPED_TO_SYSTEM_VA     | \
                           MDL_PAGES_LOCKED            | \
                           MDL_SOURCE_IS_NONPAGED_POOL | \
                           MDL_PARTIAL_HAS_BEEN_MAPPED | \
                           MDL_PARENT_MAPPED_SYSTEM_VA | \
                           MDL_LOCK_HELD               | \
                           MDL_SYSTEM_VA               | \
                           MDL_IO_SPACE )
#endif

/*============================================================================
 *
 * Error values that may be returned by the miniport
 *
 *==========================================================================*/

#define DX_OK                                   0x0
#define DXERR_UNSUPPORTED                       0x80004001
#define DXERR_GENERIC                           0x80004005
#define DXERR_OUTOFCAPS                         0x88760168

/*============================================================================
 *
 * Structures maintained by DirectDraw
 *
 *==========================================================================*/

#define DDOVER_AUTOFLIP                       	0x00100000l
#define DDOVER_BOB                       	0x00200000l
#define DDOVER_OVERRIDEBOBWEAVE			0x00400000l
#define DDOVER_INTERLEAVED			0x00800000l

//
// Data for every DXAPI surface
//

typedef struct _DDSURFACEDATA {
    DWORD       ddsCaps;                // Ring 3 creation caps
    DWORD       dwSurfaceOffset;        // Offset in frame buffer of surface
    ULONG_PTR    fpLockPtr;              // Surface lock ptr
    DWORD       dwWidth;                // Surface width
    DWORD       dwHeight;               // Surface height
    LONG        lPitch;                 // Surface pitch
    DWORD       dwOverlayFlags;         // DDOVER_XX flags
    DWORD       dwOverlayOffset;        // Offset in frame buffer of overlay
    DWORD       dwOverlaySrcWidth;	// Src width of overlay
    DWORD       dwOverlaySrcHeight;	// Src height of overlay
    DWORD       dwOverlayDestWidth;	// Dest width of overlay
    DWORD       dwOverlayDestHeight;	// Dest height of overlay
    DWORD	dwVideoPortId;		// ID of video port (-1 if not connected to a video port)
    DWORD       dwFormatFlags;
    DWORD       dwFormatFourCC;
    DWORD       dwFormatBitCount;
    DWORD       dwRBitMask;
    DWORD       dwGBitMask;
    DWORD       dwBBitMask;
    ULONG       dwDriverReserved1;      // Reserved for the HAL/Miniport
    ULONG       dwDriverReserved2;      // Reserved for the HAL/Miniport
    ULONG       dwDriverReserved3;      // Reserved for the HAL/Miniport
    ULONG       dwDriverReserved4;      // Reserved for the HAL/Miniport
} DDSURFACEDATA, *LPDDSURFACEDATA;

//
// Data for every DXAPI video port
//

typedef struct DDVIDEOPORTDATA {
    DWORD       dwVideoPortId;          // ID of video port (0 - MaxVideoPorts-1)
    DWORD       dwVPFlags;              // Video port DDVP_ option flags
    DWORD       dwOriginOffset;         // Start address relative to surface
    DWORD       dwHeight;               // Height of total video region (per field)
    DWORD       dwVBIHeight;            // Height of VBI region (per field)
    ULONG       dwDriverReserved1;      // Reserved for the HAL/Miniport
    ULONG       dwDriverReserved2;      // Reserved for the HAL/Miniport
    ULONG       dwDriverReserved3;      // Reserved for the HAL/Miniport
} DDVIDEOPORTDATA, *LPDDVIDEOPORTDATA;


/*============================================================================
 *
 * Structures used to communicate with the Miniport
 *
 *==========================================================================*/

typedef struct _DX_IRQDATA {
    DWORD       dwIrqFlags;             // DDIRQ_ flags ORed in by miniport
} DX_IRQDATA, *PDX_IRQDATA;

typedef VOID (*PDX_IRQCALLBACK)(PDX_IRQDATA pIrqData);

#define DDIRQ_DISPLAY_VSYNC			0x00000001l
#define DDIRQ_BUSMASTER                         0x00000002l
#define DDIRQ_VPORT0_VSYNC			0x00000004l
#define DDIRQ_VPORT0_LINE			0x00000008l
#define DDIRQ_VPORT1_VSYNC			0x00000010l
#define DDIRQ_VPORT1_LINE			0x00000020l
#define DDIRQ_VPORT2_VSYNC			0x00000040l
#define DDIRQ_VPORT2_LINE			0x00000080l
#define DDIRQ_VPORT3_VSYNC			0x00000100l
#define DDIRQ_VPORT3_LINE			0x00000200l
#define DDIRQ_VPORT4_VSYNC			0x00000400l
#define DDIRQ_VPORT4_LINE			0x00000800l
#define DDIRQ_VPORT5_VSYNC			0x00001000l
#define DDIRQ_VPORT5_LINE			0x00002000l
#define DDIRQ_VPORT6_VSYNC			0x00004000l
#define DDIRQ_VPORT6_LINE			0x00008000l
#define DDIRQ_VPORT7_VSYNC			0x00010000l
#define DDIRQ_VPORT7_LINE			0x00020000l
#define DDIRQ_VPORT8_VSYNC			0x00040000l
#define DDIRQ_VPORT8_LINE			0x00080000l
#define DDIRQ_VPORT9_VSYNC			0x00010000l
#define DDIRQ_VPORT9_LINE			0x00020000l

// output from DxGetIrqInfo
typedef struct _DDGETIRQINFO {
    DWORD	dwFlags;
} DDGETIRQINFO, *PDDGETIRQINFO;
#define IRQINFO_HANDLED		0x01	// Miniport is managing IRQ
#define IRQINFO_NOTHANDLED	0x02	// Not supported on NT

// input to DxEnableIrq
typedef struct _DDENABLEIRQINFO {
    DWORD           dwIRQSources;
    DWORD           dwLine;             // Line for DDIRQ_VPORTx_LINE interrupt
    PDX_IRQCALLBACK IRQCallback;	// Miniport calls this when IRQ happens
    PDX_IRQDATA     lpIRQData;          // Parameter to be passed to IRQCallback
} DDENABLEIRQINFO, *PDDENABLEIRQINFO;

// input to DxSkipNextField
typedef struct _DDSKIPNEXTFIELDINFO {
    LPDDVIDEOPORTDATA   lpVideoPortData;
    DWORD               dwSkipFlags;
} DDSKIPNEXTFIELDINFO, *PDDSKIPNEXTFIELDINFO;

#define DDSKIP_SKIPNEXT                 1
#define DDSKIP_ENABLENEXT               2

// intput to DxBobNextField
typedef struct _DDBOBNEXTFIELDINFO {
    LPDDSURFACEDATA     lpSurface;
} DDBOBNEXTFIELDINFO, *PDDBOBNEXTFIELDINFO;

// intput to DxSetState
typedef struct _DDSETSTATEININFO {
    LPDDSURFACEDATA     lpSurfaceData;
    LPDDVIDEOPORTDATA   lpVideoPortData;
} DDSETSTATEININFO, *PDDSETSTATEININFO;

// output from DxSetState
typedef struct _DDSETSTATEOUTINFO {
    BOOL                bSoftwareAutoflip;
    DWORD               dwSurfaceIndex;
    DWORD               dwVBISurfaceIndex;
} DDSETSTATEOUTINFO, *PDDSETSTATEOUTINFO;

// input to DxLock
typedef struct _DDLOCKININFO {
    LPDDSURFACEDATA     lpSurfaceData;
} DDLOCKININFO, *PDDLOCKININFO;

// output from DxLock
typedef struct _DDLOCKOUTINFO {
    ULONG_PTR            dwSurfacePtr;
} DDLOCKOUTINFO, *PDDLOCKOUTINFO;

// input to DxFlipOverlay
typedef struct _DDFLIPOVERLAYINFO {
    LPDDSURFACEDATA     lpCurrentSurface;
    LPDDSURFACEDATA     lpTargetSurface;
    DWORD               dwFlags;
} DDFLIPOVERLAYINFO, *PDDFLIPOVERLAYINFO;

// intput to DxFlipVideoPort
typedef struct _DDFLIPVIDEOPORTINFO {
    LPDDVIDEOPORTDATA   lpVideoPortData;
    LPDDSURFACEDATA     lpCurrentSurface;
    LPDDSURFACEDATA     lpTargetSurface;
    DWORD               dwFlipVPFlags;
} DDFLIPVIDEOPORTINFO, *PDDFLIPVIDEOPORTINFO;

#define DDVPFLIP_VIDEO                  0x00000001l
#define DDVPFLIP_VBI                    0x00000002l

// input to DxGetPolarity
typedef struct _DDGETPOLARITYININFO {
    LPDDVIDEOPORTDATA   lpVideoPortData;
} DDGETPOLARITYININFO, *PDDGETPOLARITYININFO;

// output from DxGetPolarity
typedef struct _DDGETPOLARITYOUTINFO {
    DWORD               bPolarity;
} DDGETPOLARITYOUTINFO, *PDDGETPOLARITYOUTINFO;

// input to DxGetCurrentAutoflipSurface
typedef struct _DDGETCURRENTAUTOFLIPININFO {
    LPDDVIDEOPORTDATA   lpVideoPortData;
} DDGETCURRENTAUTOFLIPININFO, *PDDGETCURRENTAUTOFLIPININFO;

// output from DxGetCurrentAutoflipSurface
typedef struct _DDGETCURRENTAUTOFLIPOUTINFO {
    DWORD               dwSurfaceIndex;
    DWORD               dwVBISurfaceIndex;
} DDGETCURRENTAUTOFLIPOUTINFO, *PDDGETCURRENTAUTOFLIPOUTINFO;

// input to DxGetPreviousAutoflipSurface
typedef struct _DDGETPREVIOUSAUTOFLIPININFO {
    LPDDVIDEOPORTDATA   lpVideoPortData;
} DDGETPREVIOUSAUTOFLIPININFO, *PDDGETPREVIOUSAUTOFLIPININFO;

// output from DxGetPreviousAutoflipSurface
typedef struct _DDGETPREVIOUSAUTOFLIPOUTINFO {
    DWORD               dwSurfaceIndex;
    DWORD               dwVBISurfaceIndex;
} DDGETPREVIOUSAUTOFLIPOUTINFO, *PDDGETPREVIOUSAUTOFLIPOUTINFO;

// intput to DxTransfer
typedef struct _DDTRANSFERININFO {
    LPDDSURFACEDATA	lpSurfaceData;
    DWORD		dwStartLine;
    DWORD 		dwEndLine;
    ULONG_PTR   dwTransferID;
    DWORD 		dwTransferFlags;
    PMDL  		lpDestMDL;
} DDTRANSFERININFO, *PDDTRANSFERININFO;

#define DDTRANSFER_SYSTEMMEMORY		0x00000001
#define DDTRANSFER_NONLOCALVIDMEM	0x00000002
#define DDTRANSFER_INVERT		0x00000004
#define DDTRANSFER_CANCEL		0x00000080
#define DDTRANSFER_HALFLINES		0x00000100

// output from DxTransfer
typedef struct _DDTRANSFEROUTINFO {
    DWORD dwBufferPolarity;
} DDTRANSFEROUTINFO, *PDDTRANSFEROUTINFO;

// output from DxGetTransferStatus
typedef struct _DDGETTRANSFERSTATUSOUTINFO {
    DWORD_PTR dwTransferID;
} DDGETTRANSFERSTATUSOUTINFO, *PDDGETTRANSFEROUTINFO;

/*============================================================================
 *
 * DXAPI function prototypes
 *
 *==========================================================================*/

typedef DWORD (*PDX_GETIRQINFO)(PVOID,PVOID,PDDGETIRQINFO);
typedef DWORD (*PDX_ENABLEIRQ)(PVOID,PDDENABLEIRQINFO,PVOID);
typedef DWORD (*PDX_SKIPNEXTFIELD)(PVOID,PDDSKIPNEXTFIELDINFO,PVOID);
typedef DWORD (*PDX_BOBNEXTFIELD)(PVOID,PDDBOBNEXTFIELDINFO,PVOID);
typedef DWORD (*PDX_SETSTATE)(PVOID,PDDSETSTATEININFO,PDDSETSTATEOUTINFO);
typedef DWORD (*PDX_LOCK)(PVOID,PDDLOCKININFO,PDDLOCKOUTINFO);
typedef DWORD (*PDX_FLIPOVERLAY)(PVOID,PDDFLIPOVERLAYINFO,PVOID);
typedef DWORD (*PDX_FLIPVIDEOPORT)(PVOID,PDDFLIPVIDEOPORTINFO,PVOID);
typedef DWORD (*PDX_GETPOLARITY)(PVOID,PDDGETPOLARITYININFO,PDDGETPOLARITYOUTINFO);
typedef DWORD (*PDX_GETCURRENTAUTOFLIP)(PVOID,PDDGETCURRENTAUTOFLIPININFO,PDDGETCURRENTAUTOFLIPOUTINFO);
typedef DWORD (*PDX_GETPREVIOUSAUTOFLIP)(PVOID,PDDGETPREVIOUSAUTOFLIPININFO,PDDGETPREVIOUSAUTOFLIPOUTINFO);
typedef DWORD (*PDX_TRANSFER)(PVOID,PDDTRANSFERININFO,PDDTRANSFEROUTINFO);
typedef DWORD (*PDX_GETTRANSFERSTATUS)(PVOID,PVOID,PDDGETTRANSFEROUTINFO);

/*============================================================================
 *
 * HAL table filled in by the miniport and called by DirectDraw
 *
 *==========================================================================*/

#define DXAPI_HALVERSION 0x0001

typedef struct _DXAPI_INTERFACE {

    USHORT                  Size;
    USHORT                  Version;
    PVOID                   Context;
    PVOID                   InterfaceReference;
    PVOID                   InterfaceDereference;
    PDX_GETIRQINFO          DxGetIrqInfo;
    PDX_ENABLEIRQ           DxEnableIrq;
    PDX_SKIPNEXTFIELD       DxSkipNextField;
    PDX_BOBNEXTFIELD        DxBobNextField;
    PDX_SETSTATE            DxSetState;
    PDX_LOCK                DxLock;
    PDX_FLIPOVERLAY         DxFlipOverlay;
    PDX_FLIPVIDEOPORT       DxFlipVideoPort;
    PDX_GETPOLARITY         DxGetPolarity;
    PDX_GETCURRENTAUTOFLIP  DxGetCurrentAutoflip;
    PDX_GETPREVIOUSAUTOFLIP DxGetPreviousAutoflip;
    PDX_TRANSFER	    DxTransfer;
    PDX_GETTRANSFERSTATUS   DxGetTransferStatus;

} DXAPI_INTERFACE, *PDXAPI_INTERFACE;

#endif  // GUID_DEFS_ONLY

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
