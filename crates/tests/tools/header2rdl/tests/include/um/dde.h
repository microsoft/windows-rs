/*****************************************************************************\
*                                                                             *
* dde.h -       Dynamic Data Exchange structures and definitions              *
*                                                                             *
* Copyright (c) 1993-1999, Microsoft Corp.	All rights reserved	      *
*                                                                             *
\*****************************************************************************/
#ifndef _DDEHEADER_INCLUDED_
#define _DDEHEADER_INCLUDED_

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef _WINDEF_
#include <windef.h>
#endif

#include <winapifamily.h>

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// begin_r_dde

/* DDE window messages */

#define WM_DDE_FIRST	    0x03E0
#define WM_DDE_INITIATE     (WM_DDE_FIRST)
#define WM_DDE_TERMINATE    (WM_DDE_FIRST+1)
#define WM_DDE_ADVISE	    (WM_DDE_FIRST+2)
#define WM_DDE_UNADVISE     (WM_DDE_FIRST+3)
#define WM_DDE_ACK	        (WM_DDE_FIRST+4)
#define WM_DDE_DATA	        (WM_DDE_FIRST+5)
#define WM_DDE_REQUEST	    (WM_DDE_FIRST+6)
#define WM_DDE_POKE	        (WM_DDE_FIRST+7)
#define WM_DDE_EXECUTE	    (WM_DDE_FIRST+8)
#define WM_DDE_LAST	        (WM_DDE_FIRST+8)

// end_r_dde

/*----------------------------------------------------------------------------
|       DDEACK structure
|
|	Structure of wStatus (LOWORD(lParam)) in WM_DDE_ACK message
|       sent in response to a WM_DDE_DATA, WM_DDE_REQUEST, WM_DDE_POKE,
|       WM_DDE_ADVISE, or WM_DDE_UNADVISE message.
|
----------------------------------------------------------------------------*/

typedef struct {
#ifndef _MAC
        unsigned short bAppReturnCode:8,
                 reserved:6,
                 fBusy:1,
		 fAck:1;
#else
        unsigned short usFlags;
#endif
} DDEACK;


/*----------------------------------------------------------------------------
|       DDEADVISE structure
|
|	WM_DDE_ADVISE parameter structure for hOptions (LOWORD(lParam))
|
----------------------------------------------------------------------------*/

typedef struct {
#ifndef _MAC
        unsigned short reserved:14,
                 fDeferUpd:1,
		 fAckReq:1;
#else
        unsigned short usFlags;
#endif
	short     cfFormat;
} DDEADVISE;


/*----------------------------------------------------------------------------
|       DDEDATA structure
|
|       WM_DDE_DATA parameter structure for hData (LOWORD(lParam)).
|       The actual size of this structure depends on the size of
|       the Value array.
|
----------------------------------------------------------------------------*/

typedef struct {
#ifndef _MAC
	unsigned short unused:12,
                 fResponse:1,
                 fRelease:1,
                 reserved:1,
                 fAckReq:1;
#else
	unsigned short usFlags;
#endif
	short	 cfFormat;
	BYTE	 Value[1];
} DDEDATA;


/*----------------------------------------------------------------------------
|	DDEPOKE structure
|
|	WM_DDE_POKE parameter structure for hData (LOWORD(lParam)).
|       The actual size of this structure depends on the size of
|       the Value array.
|
----------------------------------------------------------------------------*/

typedef struct {
#ifndef _MAC
	unsigned short unused:13,  /* Earlier versions of DDE.H incorrectly */
                             /* 12 unused bits.                       */
		 fRelease:1,
		 fReserved:2;
#else
	unsigned short usFlags;
#endif
	short    cfFormat;
	BYTE	 Value[1];  /* This member was named rgb[1] in previous */
                            /* versions of DDE.H                        */

} DDEPOKE;

/*----------------------------------------------------------------------------
The following typedef's were used in previous versions of the Windows SDK.
They are still valid.  The above typedef's define exactly the same structures
as those below.  The above typedef names are recommended, however, as they
are more meaningful.

Note that the DDEPOKE structure typedef'ed in earlier versions of DDE.H did
not correctly define the bit positions.
----------------------------------------------------------------------------*/

typedef struct {
#ifndef _MAC
        unsigned short unused:13,
                 fRelease:1,
                 fDeferUpd:1,
		 fAckReq:1;
#else
	unsigned short usFlags;
#endif
	short	 cfFormat;
} DDELN;

typedef struct {
#ifndef _MAC
	unsigned short unused:12,
                 fAck:1,
                 fRelease:1,
                 fReserved:1,
                 fAckReq:1;
#else
	unsigned short usFlags;
#endif
	short	 cfFormat;
	BYTE	 rgb[1];
} DDEUP;


/*
 * DDE SECURITY
 */

BOOL
WINAPI
DdeSetQualityOfService(
    HWND hwndClient,
    CONST SECURITY_QUALITY_OF_SERVICE *pqosNew,
    PSECURITY_QUALITY_OF_SERVICE pqosPrev);

BOOL
WINAPI
ImpersonateDdeClientWindow(
    HWND hWndClient,
    HWND hWndServer);

/*
 * DDE message packing APIs
 */
LPARAM APIENTRY PackDDElParam(_In_ UINT msg, _In_ UINT_PTR uiLo, _In_ UINT_PTR uiHi);
BOOL   APIENTRY UnpackDDElParam(_In_ UINT msg, _In_ LPARAM lParam, _Out_ PUINT_PTR puiLo, _Out_ PUINT_PTR puiHi);
BOOL   APIENTRY FreeDDElParam(_In_ UINT msg, _In_ LPARAM lParam);
LPARAM APIENTRY ReuseDDElParam(LPARAM lParam, UINT msgIn, UINT msgOut, UINT_PTR uiLo, UINT_PTR uiHi);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif // _DDEHEADER_INCLUDED_
