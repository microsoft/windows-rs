/*==========================================================================;
 *
 *  Copyright (C) 1994-1998 Microsoft Corporation.  All Rights Reserved.
 *
 *  File:	dmemmgr.h
 *  Content:	Direct Memory Manager include file
 *
 ***************************************************************************/

#ifndef __DMEMMGR_INCLUDED__
#define __DMEMMGR_INCLUDED__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#ifndef __NTDDKCOMP__

/*
 * pointer to video memory
 */
typedef ULONG_PTR FLATPTR;

typedef struct _VIDMEM *LPVIDMEM;

#else

/*
 * pointer to video memory, potentially 64-bit
 */
typedef ULONG_PTR FLATPTR;

typedef struct _VIDEOMEMORY *LPVIDMEM;

#endif

/*
 * Structure for querying extended heap alignment requirements
 */

typedef struct _SURFACEALIGNMENT
{
    union
    {
        struct
        {
            DWORD       dwStartAlignment;
            DWORD       dwPitchAlignment;
            DWORD       dwFlags;
            DWORD       dwReserved2;
        } Linear;
        struct
        {
            DWORD       dwXAlignment;
            DWORD       dwYAlignment;
            DWORD       dwFlags;
            DWORD       dwReserved2;
        } Rectangular;
    };
} SURFACEALIGNMENT;
typedef struct _SURFACEALIGNMENT FAR *LPSURFACEALIGNMENT;

#define SURFACEALIGN_DISCARDABLE	0x00000001l /* Surface can be discarded to   */
                                                    /* make room for another surface */


typedef struct _HEAPALIGNMENT
{
    DWORD                dwSize;
    DDSCAPS              ddsCaps;       /* Indicates which alignment fields are valid.*/
    DWORD                dwReserved;
    SURFACEALIGNMENT     ExecuteBuffer; /* Surfaces tagged with DDSCAPS_EXECUTEBUFFER */
    SURFACEALIGNMENT     Overlay;       /* Surfaces tagged with DDSCAPS_OVERLAY       */
    SURFACEALIGNMENT     Texture;       /* Surfaces tagged with DDSCAPS_TEXTURE       */
    SURFACEALIGNMENT     ZBuffer;       /* Surfaces tagged with DDSCAPS_ZBUFFER       */
    SURFACEALIGNMENT     AlphaBuffer;   /* Surfaces tagged with DDSCAPS_ALPHA         */
    SURFACEALIGNMENT     Offscreen;     /* Surfaces tagged with DDSCAPS_OFFSCREENPLAIN*/
    SURFACEALIGNMENT     FlipTarget;    /* Surfaces whose bits are potential primaries i.e. back buffers*/
} HEAPALIGNMENT;
typedef struct _HEAPALIGNMENT FAR *LPHEAPALIGNMENT;

typedef struct _DD_GETHEAPALIGNMENTDATA
{
    ULONG_PTR                  dwInstance;         // driver context
    DWORD                      dwHeap;             // heap index passed by DirectDraw
    HRESULT                    ddRVal;             // return value
    VOID*                      GetHeapAlignment;   // Unused: Win95 compatibility
    HEAPALIGNMENT              Alignment;          // Filled in by driver.
} DD_GETHEAPALIGNMENTDATA;
typedef struct _DD_GETHEAPALIGNMENTDATA *PDD_GETHEAPALIGNMENTDATA;

/*
 * video memory manager structures
 */
typedef struct _VMEML
{
    struct _VMEML 	FAR *next;
    FLATPTR		ptr;
    DWORD		size;
    BOOL                bDiscardable;
} VMEML, FAR *LPVMEML, FAR * FAR *LPLPVMEML;

typedef struct _VMEMR
{
    struct _VMEMR 	FAR *next;
    struct _VMEMR       FAR *prev;
    /*
     * The pUp, pDown, pLeft and pRight members were removed in DX5
     */
    struct _VMEMR 	FAR *pUp;
    struct _VMEMR 	FAR *pDown;
    struct _VMEMR 	FAR *pLeft;
    struct _VMEMR 	FAR *pRight;
    FLATPTR		ptr;
    DWORD		size;
    DWORD               x;
    DWORD               y;
    DWORD               cx;
    DWORD               cy;
    DWORD		flags;
    FLATPTR             pBits;
    BOOL                bDiscardable;
} VMEMR, FAR *LPVMEMR, FAR * FAR *LPLPVMEMR;

typedef struct _VMEMHEAP
{
    DWORD		        dwFlags;
    DWORD                       stride;
    LPVOID		        freeList;
    LPVOID		        allocList;
    DWORD                       dwTotalSize;
    FLATPTR                     fpGARTLin;      /* AGP: GART linear base of heap (app. visible)   */
    FLATPTR                     fpGARTDev;      /* AGP: GART device base of heap (driver visible) */
    DWORD                       dwCommitedSize; /* AGP: Number of bytes commited to heap          */
    /*
     * Extended alignment data:
     * Filled in by DirectDraw in response to a GetHeapAlignment HAL call.
     */
    DWORD                       dwCoalesceCount;
    HEAPALIGNMENT               Alignment;
    /*
     * These are analogous to VIDMEM.ddsCaps and VIDMEM.ddsCapsAlt. These values are queried from the
     * driver by a GetDriverInfo call. See the documentation for GUID_DDMoreSurfaceCaps
     */
    DDSCAPSEX                   ddsCapsEx;
    DDSCAPSEX                   ddsCapsExAlt;
#ifndef IS_16
    // Full physical address of heap base for NT AGP heaps.
    LARGE_INTEGER               liPhysAGPBase;
#endif
    // hdev for use with VidMemAllocAligned on NT.  Set by the system at
    // initialization time.
    HANDLE                      hdevAGP;
    // Physical reservation handle for NT heaps.
    LPVOID                      pvPhysRsrv;
#if (NTDDI_VERSION >= NTDDI_WINXP)
    BYTE*                       pAgpCommitMask;
    DWORD                       dwAgpCommitMaskSize;
#endif
} VMEMHEAP;

typedef VMEMHEAP FAR *LPVMEMHEAP;

#define VMEMHEAP_LINEAR			0x00000001l /* Heap is linear                    */
#define VMEMHEAP_RECTANGULAR		0x00000002l /* Heap is rectangular               */
#define VMEMHEAP_ALIGNMENT  		0x00000004l /* Heap has extended alignment info  */

/*
 * This legacy export doesn't handle nonlocal heaps
 * This function is not available on Windows NT
 */
#ifndef __NTDDKCOMP__
extern FLATPTR WINAPI VidMemAlloc( LPVMEMHEAP pvmh, DWORD width, DWORD height );
#endif

/*
 * This export can be used by drivers to allocate aligned surfaces from heaps which
 * they have previously exposed to DirectDraw. This function can allocate from nonlocal heaps.
 */
extern FLATPTR WINAPI HeapVidMemAllocAligned(
                LPVIDMEM lpVidMem,
                DWORD dwWidth,
                DWORD dwHeight,
                LPSURFACEALIGNMENT lpAlignment ,
                LPLONG lpNewPitch );

/*
 * This export can free memory allocated via either allocation function
 */
extern void WINAPI VidMemFree( LPVMEMHEAP pvmh, FLATPTR ptr );

#ifdef __cplusplus
};
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
