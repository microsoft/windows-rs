/****************************************************************************

 DCIMAN.H

 Copyright (C) 1993-1999 Microsoft Corporation.  All Rights Reserved.

 DCIMAN 1.0 client interface definitions

 ***************************************************************************/

#ifndef _INC_DCIMAN
#define _INC_DCIMAN

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#ifdef __cplusplus
    #define __inline inline
    extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/****************************************************************************
 ***************************************************************************/

#include "dciddi.h"         // interface to the DCI provider

/****************************************************************************
 ***************************************************************************/

DECLARE_HANDLE(HWINWATCH);  // context handle for WinWatch instance

/****************************************************************************
 ***************************************************************************/

extern HDC WINAPI DCIOpenProvider(void);
extern void WINAPI DCICloseProvider(HDC hdc);

extern int WINAPI DCICreatePrimary(HDC hdc, LPDCISURFACEINFO FAR *lplpSurface);
extern int WINAPI DCICreateOffscreen(HDC hdc, DWORD dwCompression, DWORD dwRedMask,
    DWORD dwGreenMask, DWORD dwBlueMask, DWORD dwWidth, DWORD dwHeight,
    DWORD dwDCICaps, DWORD dwBitCount, LPDCIOFFSCREEN FAR *lplpSurface);
extern int WINAPI DCICreateOverlay(HDC hdc, LPVOID lpOffscreenSurf,
    LPDCIOVERLAY FAR *lplpSurface);
extern int WINAPI DCIEnum(HDC hdc, LPRECT lprDst, LPRECT lprSrc, LPVOID lpFnCallback,
    LPVOID lpContext);
extern DCIRVAL WINAPI DCISetSrcDestClip(LPDCIOFFSCREEN pdci, LPRECT srcrc,
			LPRECT destrc, LPRGNDATA prd );

extern HWINWATCH WINAPI WinWatchOpen(HWND hwnd);
extern void      WINAPI WinWatchClose(HWINWATCH hWW);

// API changed to copy region data instead of return pointer to it
extern UINT	 WINAPI WinWatchGetClipList(HWINWATCH hWW, LPRECT prc,
				UINT size,  LPRGNDATA prd);
extern BOOL      WINAPI WinWatchDidStatusChange(HWINWATCH hWW);

extern DWORD     WINAPI GetWindowRegionData(HWND hwnd, DWORD size, LPRGNDATA prd);
extern DWORD     WINAPI GetDCRegionData(HDC hdc, DWORD size, LPRGNDATA prd);


#define WINWATCHNOTIFY_START        0
#define WINWATCHNOTIFY_STOP         1
#define WINWATCHNOTIFY_DESTROY      2
#define WINWATCHNOTIFY_CHANGING     3
#define WINWATCHNOTIFY_CHANGED      4
typedef void (CALLBACK *WINWATCHNOTIFYPROC)(HWINWATCH hww, HWND hwnd, DWORD code, LPARAM lParam);

extern BOOL WINAPI WinWatchNotify(HWINWATCH hWW, WINWATCHNOTIFYPROC NotifyCallback,
						LPARAM NotifyParam );

#ifdef WIN32
/****************************************************************************
 helper functions to call DCIMAN16.DLL
 ***************************************************************************/
extern void WINAPI DCIEndAccess(LPDCISURFACEINFO pdci);
extern DCIRVAL WINAPI DCIBeginAccess(LPDCISURFACEINFO pdci, int x, int y, int dx, int dy);
extern void WINAPI DCIDestroy(LPDCISURFACEINFO pdci);
extern DCIRVAL WINAPI DCIDraw(LPDCIOFFSCREEN pdci);
extern DCIRVAL WINAPI DCISetClipList(LPDCIOFFSCREEN pdci, LPRGNDATA prd);
extern DCIRVAL WINAPI DCISetDestination(LPDCIOFFSCREEN pdci, LPRECT dst, LPRECT src);


#else

extern int WINAPI DCISendCommand(HDC hdc, VOID FAR *pcmd, int nSize, VOID FAR * FAR * lplpOut);

/****************************************************************************
 helper macros to call DCI callbacks
 ***************************************************************************/
__inline void DCIDestroy(LPDCISURFACEINFO pdci)
{
	if( pdci->DestroySurface != NULL ) {
		pdci->DestroySurface(pdci);
	}
}

__inline void DCIEndAccess(LPDCISURFACEINFO pdci)
{
	if( pdci->EndAccess != NULL ) {
		pdci->EndAccess(pdci);
	}
}

__inline DCIRVAL DCIBeginAccess(LPDCISURFACEINFO pdci, int x, int y, int dx, int dy)
{
    RECT rc;

	if( pdci->BeginAccess != NULL ) {
		rc.left=x;
		rc.top=y;
		rc.right = rc.left+dx;
		rc.bottom = rc.top+dy;
		return pdci->BeginAccess(pdci, &rc);
	} else {
		return DCI_OK;
	}
}

__inline DCIRVAL DCIDraw(LPDCIOFFSCREEN pdci)
{
	if( pdci->Draw != NULL ) {
		return pdci->Draw(pdci);
	} else {
		return DCI_OK;
	}
}

__inline DCIRVAL DCISetClipList(LPDCIOFFSCREEN pdci, LPRGNDATA prd)
{
	if( pdci->SetClipList != NULL ) {
		return pdci->SetClipList(pdci, prd);
	} else {
		return DCI_OK;
	}
}

__inline DCIRVAL DCISetDestination(LPDCIOFFSCREEN pdci, LPRECT dst, LPRECT src)
{
	if( pdci->SetDestination != NULL ) {
		return pdci->SetDestination(pdci, dst, src);
	} else {
		return DCI_OK;
	}
}
#endif

/****************************************************************************
 ***************************************************************************/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
    }
#endif

#endif // _INC_DCIMAN
