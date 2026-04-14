/*==========================================================================;
 *
 *  Copyright (C) Microsoft Corporation.  All Rights Reserved.
 *
 *  File:	ddkmapi.h
 *  Content:	Kernel mode APIs for accessing DirectDraw support.
 *
 ***************************************************************************/

#ifndef __DDKMAPI_INCLUDED__
#define __DDKMAPI_INCLUDED__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
 * API entry point
 */
DWORD
FAR PASCAL
DxApi(
    DWORD  dwFunctionNum,
    LPVOID lpvInBuffer,
    DWORD  cbInBuffer,
    LPVOID lpvOutBuffer,
    DWORD  cbOutBuffer
);

typedef
DWORD
(FAR PASCAL *LPDXAPI)(
    DWORD   dwFunctionNum,
    LPVOID  lpvInBuffer,
    DWORD   cbInBuffer,
    LPVOID  lpvOutBuffer,
    DWORD   cbOutBuffer
);

#define DXAPI_MAJORVERSION		1
#define DXAPI_MINORVERSION              0

#define DD_FIRST_DXAPI					0x500

typedef ULONG (FAR PASCAL *LPDD_NOTIFYCALLBACK)(DWORD dwFlags, PVOID pContext, DWORD dwParam1, DWORD dwParam2);

/*
 * Queries the DXAPI version number.
 *
 * Input:  Null
 * Output: LPDDGETVERSIONNUMBER
 */
#define DD_DXAPI_GETVERSIONNUMBER                       (DD_FIRST_DXAPI)

    typedef struct _DDGETVERSIONNUMBER
    {
        DWORD   ddRVal;
        DWORD   dwMajorVersion;
        DWORD   dwMinorVersion;
    } DDGETVERSIONNUMBER, FAR *LPDDGETVERSIONNUMBER;

/*
 * Closes the kernel mode handle.
 *
 * Input:  LPDDCLOSEHANDLE
 * Output: DWORD DirectDraw return value
 */
#define DD_DXAPI_CLOSEHANDLE                            (DD_FIRST_DXAPI+1)

    typedef struct _DDCLOSEHANDLE
    {
        HANDLE  hHandle;
    } DDCLOSEHANDLE, FAR *LPDDCLOSEHANDLE;

/*
 * Opens the DirectDraw object and returns a kernel mode handle.
 *
 * Input:  LPDDOPENDIRECTDRAWIN
 * Output: LPDDOPENDIRECTDRAWOUT
 */
#define DD_DXAPI_OPENDIRECTDRAW                         (DD_FIRST_DXAPI+2)

    typedef struct _DDOPENDIRECTDRAWIN
    {
        ULONG_PTR            dwDirectDrawHandle;
        LPDD_NOTIFYCALLBACK pfnDirectDrawClose;
        PVOID               pContext;
    } DDOPENDIRECTDRAWIN, FAR *LPDDOPENDIRECTDRAWIN;

    typedef struct _DDOPENDIRECTDRAWOUT
    {
        DWORD   ddRVal;
        HANDLE  hDirectDraw;
    } DDOPENDIRECTDRAWOUT, FAR *LPDDOPENDIRECTDRAWOUT;

/*
 * Opens the surface and returns a kernel mode handle.
 *
 * Input:  LPDDOPENSURFACEIN
 * Output: LPDDOPENSURFACEOUT
 */
#define DD_DXAPI_OPENSURFACE                            (DD_FIRST_DXAPI+3)

    typedef struct _DDOPENSURFACEIN
    {
	HANDLE	            hDirectDraw;
        ULONG_PTR            dwSurfaceHandle;
        LPDD_NOTIFYCALLBACK pfnSurfaceClose;
        PVOID               pContext;
    } DDOPENSURFACEIN, FAR *LPDDOPENSURFACEIN;

    typedef struct _DDOPENSURFACEOUT
    {
        DWORD   ddRVal;
        HANDLE  hSurface;
    } DDOPENSURFACEOUT, FAR *LPDDOPENSURFACEOUT;

/*
 * Opens the VideoPort and returns a kernel mode handle.
 *
 * Input:  LPDDOPENVIDEOPORTIN
 * Output: LPDDOPENVIDEOPORTOUT
 */
#define DD_DXAPI_OPENVIDEOPORT                          (DD_FIRST_DXAPI+4)

    typedef struct _DDOPENVIDEOPORTIN
    {
	HANDLE		    hDirectDraw;
        ULONG               dwVideoPortHandle;
        LPDD_NOTIFYCALLBACK pfnVideoPortClose;
        PVOID               pContext;
    } DDOPENVIDEOPORTIN, FAR *LPDDOPENVIDEOPORTIN;

    typedef struct _DDOPENVIDEOPORTOUT
    {
        DWORD   ddRVal;
        HANDLE  hVideoPort;
    } DDOPENVIDEOPORTOUT, FAR *LPDDOPENVIDEOPORTOUT;

/*
 * Returns the kernel mode capabilities supported by the device
 *
 * Input:  HANDLE hDirectDraw
 * Output: LPDDGETKERNELCAPSOUT
 */
#define DD_DXAPI_GETKERNELCAPS                          (DD_FIRST_DXAPI+5)

    typedef struct _DDGETKERNELCAPSOUT
    {
	DWORD	ddRVal;
        DWORD	dwCaps;
	DWORD	dwIRQCaps;
    } DDGETKERNELCAPSOUT, FAR *LPDDGETKERNELCAPSOUT;

/*
 * Gets the current field number
 *
 * Input:  LPDDGETFIELDNUMIN
 * Output: LPDDGETFIELDNUMOUT
 */
#define DD_DXAPI_GET_VP_FIELD_NUMBER			(DD_FIRST_DXAPI+6)

    typedef struct _DDGETFIELDNUMIN
    {
	HANDLE	hDirectDraw;
	HANDLE	hVideoPort;
    } DDGETFIELDNUMIN, FAR *LPDDGETFIELDNUMIN;

    typedef struct _DDGETFIELDNUMOUT
    {
	DWORD	ddRVal;
	DWORD	dwFieldNum;
    } DDGETFIELDNUMOUT, FAR *LPDDGETFIELDNUMOUT;

/*
 * Sets the current field number
 *
 * Input:  LPDDSETFIELDNUM
 * Output: DWORD DirectDraw return value
 */
#define DD_DXAPI_SET_VP_FIELD_NUMBER			(DD_FIRST_DXAPI+7)

    typedef struct _DDSETFIELDNUM
    {
	HANDLE	hDirectDraw;
	HANDLE	hVideoPort;
	DWORD	dwFieldNum;
    } DDSETFIELDNUM, FAR *LPDDSETFIELDNUM;

/*
 * Indicates which fields should be skipped to undo the 3:2 pulldown.
 *
 * Input:  LPDDSETSKIPFIELD
 * Output: DWORD DirectDraw return value
 */
#define DD_DXAPI_SET_VP_SKIP_FIELD			(DD_FIRST_DXAPI+8)

    typedef struct _DDSETSKIPFIELD
    {
	HANDLE	hDirectDraw;
	HANDLE	hVideoPort;
	DWORD	dwStartField;
    } DDSETSKIPFIELD, FAR *LPDDSETSKIPFIELD;

/*
 * Notifies whether the surface is in bob or weave mode.
 *
 * Input:  LPDDGETSURFACESTATEIN
 * Output: LPDDGETSURFACESTATEOUT
 */
#define DD_DXAPI_GET_SURFACE_STATE			(DD_FIRST_DXAPI+9)

    typedef struct _DDGETSURFACESTATEIN
    {
	HANDLE	hDirectDraw;
	HANDLE	hSurface;
    } DDGETSURFACESTATEIN, FAR *LPDDGETSURFACESTATEIN;

    typedef struct _DDGETSURFACESTATEOUT
    {
	DWORD	ddRVal;
	DWORD	dwStateCaps;
	DWORD	dwStateStatus;
    } DDGETSURFACESTATEOUT, FAR *LPDDGETSURFACESTATEOUT;

/*
 * Changes the surface between bob and weave mode.
 *
 * Input:  LPDDSETSURFACESTATE
 * Output: DWORD DirectDraw return value
 */
#define DD_DXAPI_SET_SURFACE_STATE			(DD_FIRST_DXAPI+10)

    typedef struct _DDSETSURFACETATE
    {
	HANDLE	hDirectDraw;
	HANDLE	hSurface;
	DWORD	dwState;
	DWORD	dwStartField;
    } DDSETSURFACESTATE, FAR *LPDDSETSURFACESTATE;

/*
 * Allows direct access to the surface memory
 *
 * Input:  LPDDLOCKIN
 * Output: LPDDLOCKOUT
 */
#define DD_DXAPI_LOCK					(DD_FIRST_DXAPI+11)

    typedef struct _DDLOCKIN
    {
	HANDLE	hDirectDraw;
	HANDLE	hSurface;
    } DDLOCKIN, FAR *LPDDLOCKIN;

    typedef struct _DDLOCKOUT
    {
	DWORD	ddRVal;
	DWORD	dwSurfHeight;
	DWORD	dwSurfWidth;
	LONG	lSurfPitch;
	PVOID	lpSurface;
	DWORD	SurfaceCaps;
	DWORD	dwFormatFlags;
	DWORD	dwFormatFourCC;
	DWORD	dwFormatBitCount;
	union
	{
	    DWORD	dwRBitMask;
	    DWORD	dwYBitMask;
    	};
    	union
    	{
	    DWORD	dwGBitMask;
	    DWORD	dwUBitMask;
	};
	union
	{
	    DWORD	dwBBitMask;
	    DWORD	dwVBitMask;
	};
    } DDLOCKOUT, FAR *LPDDLOCKOUT;

/*
 * Flips the overlay surface
 *
 * Input:  LPDDFLIPOVERLAY
 * Output: DWORD DirectDraw return value
 */
#define DD_DXAPI_FLIP_OVERLAY				(DD_FIRST_DXAPI+12)

    typedef struct _DDFLIPOVERLAY
    {
	HANDLE	hDirectDraw;
	HANDLE	hCurrentSurface;
	HANDLE	hTargetSurface;
	DWORD	dwFlags;
    } DDFLIPOVERLAY, FAR *LPDDFLIPOVERLAY;

/*
 * Flips the video port
 *
 * Input:  LPDDFLIPOVERLAY
 * Output: DWORD DirectDraw return value
 */
#define DD_DXAPI_FLIP_VP				(DD_FIRST_DXAPI+13)

    typedef struct _DDFLIPVIDEOPORT
    {
	HANDLE	hDirectDraw;
	HANDLE	hVideoPort;
	HANDLE	hCurrentSurface;
	HANDLE	hTargetSurface;
	DWORD	dwFlags;
    } DDFLIPVIDEOPORT, FAR *LPDDFLIPVIDEOPORT;

/*
 * Returns the current surface receiving the data while autoflipping
 *
 * Input:  LPDDGETAUTOFLIPIN
 * Output: LPDDGETAUTOFLIPOUT
 */
#define DD_DXAPI_GET_CURRENT_VP_AUTOFLIP_SURFACE	(DD_FIRST_DXAPI+14)

    typedef struct _DDGETAUTOFLIPIN
    {
	HANDLE	hDirectDraw;
	HANDLE	hVideoPort;
    } DDGETAUTOFLIPIN, FAR *LPDDGETAUTOFLIPIN;

    typedef struct _DDGETAUTOFLIPOUT
    {
	DWORD	ddRVal;
	HANDLE	hVideoSurface;
	HANDLE	hVBISurface;
	BOOL	bPolarity;
    } DDGETAUTOFLIPOUT, FAR *LPDDGETAUTOFLIPOUT;

/*
 * Returns the surface that received the previous field of data (could
 * be the same as current if video is interleaved)
 *
 * Input:  LPDDGETAUTOFLIPIN
 * Output: LPDDGETAUTOFLIPOUT
 */
#define DD_DXAPI_GET_LAST_VP_AUTOFLIP_SURFACE		(DD_FIRST_DXAPI+15)

/*
 * Registers a callback for when various events occur.
 *
 * Input:  LPDDREGISTERCALLBACK
 * Output: DWORD DirectDraw return value
 */
#define DD_DXAPI_REGISTER_CALLBACK			(DD_FIRST_DXAPI+16)

    typedef struct _DDREGISTERCALLBACK
    {
	HANDLE 	            hDirectDraw;
	ULONG	            dwEvents;
	LPDD_NOTIFYCALLBACK pfnCallback;
	ULONG_PTR            dwParam1;
	ULONG_PTR            dwParam2;
	PVOID	            pContext;
    } DDREGISTERCALLBACK, FAR *LPDDREGISTERCALLBACK;

/*
 * Unregisters a callback for when various events occur.
 *
 * Input:  LPDDREGISTERCALLBACK
 * Output: DWORD DirectDraw return value
 */
#define DD_DXAPI_UNREGISTER_CALLBACK			(DD_FIRST_DXAPI+17)

/*
 * Returns the polarity (odd/even) of the current field
 *
 * Input:  LPDDGETPOLARITYIN
 * Output: LPDDGETPOLARITYOUT
 */
#define DD_DXAPI_GET_POLARITY				(DD_FIRST_DXAPI+18)

    typedef struct _DDGETPOLARITYIN
    {
	HANDLE 	hDirectDraw;
	HANDLE	hVideoPort;
    } DDGETPOLARITYIN, FAR *LPDDGETPOLARITYIN;

    typedef struct _DDGETPOLARITYOUT
    {
	DWORD 	ddRVal;
	BOOL	bPolarity;
    } DDGETPOLARITYOUT, FAR *LPDDGETPOLARITYOUT;

/*
 * Opens the device for capture
 *
 * Input:  LPDDOPENCAPTUREDEVICEIN
 * Output: LPDDOPENCAPTUREDEVICEOUT
 */
#define DD_DXAPI_OPENVPCAPTUREDEVICE			(DD_FIRST_DXAPI+19)

    typedef struct _DDOPENVPCAPTUREDEVICEIN
    {
	HANDLE  hDirectDraw;
	HANDLE  hVideoPort;
	DWORD   dwStartLine;
	DWORD   dwEndLine;
	DWORD   dwCaptureEveryNFields;
	LPDD_NOTIFYCALLBACK pfnCaptureClose;
	PVOID   pContext;
	DWORD	dwFlags;
    } DDOPENVPCAPTUREDEVICEIN, FAR * LPDDOPENVPCAPTUREDEVICEIN;

    typedef struct _DDOPENVPCAPTUREDEVICEOUT
    {
	DWORD	ddRVal;
	HANDLE	hCapture;
    } DDOPENVPCAPTUREDEVICEOUT, FAR * LPDDOPENVPCAPTUREDEVICEOUT;

    #define DDOPENCAPTURE_VIDEO	0x0001	// Capture from the video stream
    #define DDOPENCAPTURE_VBI	0x0002	// Capture from the VBI stream

/*
 * Adds a capture buffer to the internal video port capture queue
 *
 * Input:  LPDDADDVPCAPTUREBUFF
 * Output: DWORD DirectDraw return value
 */
#define DD_DXAPI_ADDVPCAPTUREBUFFER			(DD_FIRST_DXAPI+20)

    typedef struct _DDCAPBUFFINFO
    {
	DWORD   	dwFieldNumber;
	DWORD   	bPolarity;
	LARGE_INTEGER	liTimeStamp;
	DWORD   	ddRVal;
    } DDCAPBUFFINFO, FAR * LPDDCAPBUFFINFO;

    typedef struct _DDADDVPCAPTUREBUFF
    {
	HANDLE  hCapture;
	DWORD   dwFlags;
	PMDL    pMDL;
	PKEVENT pKEvent;
	LPDDCAPBUFFINFO lpBuffInfo;
    } DDADDVPCAPTUREBUFF, FAR * LPDDADDVPCAPTUREBUFF;

    #define DDADDBUFF_SYSTEMMEMORY	0x0001	// lpBuffer points to sys mem
    #define DDADDBUFF_NONLOCALVIDMEM	0x0002	// lpBuffer points to AGP mem
    #define DDADDBUFF_INVERT		0x0004	// invert the buffer during capture

/*
 * Flushes the internal video port capture queue
 *
 * Input:  HANDLE to capture device
 * Output: DWORD DirectDraw return value
 */
#define DD_DXAPI_FLUSHVPCAPTUREBUFFERS			(DD_FIRST_DXAPI+21)


/*
 * State flags returned by DSVXD_DXAPI_DD_GET_SURFACE_STATE
 */
#define DDSTATE_BOB				0x0001
#define DDSTATE_WEAVE				0x0002
#define DDSTATE_EXPLICITLY_SET			0x0004
#define DDSTATE_SOFTWARE_AUTOFLIP		0x0008
#define DDSTATE_SKIPEVENFIELDS			0x0010

/*
 * Event flags - passed into RegisterCallback
 */
#define DDEVENT_DISPLAY_VSYNC			0x0001
#define DDEVENT_VP_VSYNC			0x0002
#define DDEVENT_VP_LINE				0x0004
#define DDEVENT_PRERESCHANGE			0x0008
#define DDEVENT_POSTRESCHANGE			0x0010
#define DDEVENT_PREDOSBOX			0x0020
#define DDEVENT_POSTDOSBOX			0x0040

/*
 * Notification flags - passed to the notification proc
 */
#define DDNOTIFY_DISPLAY_VSYNC			0x0001	// dwParam1 = hDirectDraw
#define DDNOTIFY_VP_VSYNC			0x0002	// dwParam1 = hVideoPort
#define DDNOTIFY_VP_LINE	  		0x0004	// dwParam1 = hVideoPort
#define DDNOTIFY_PRERESCHANGE			0x0008	// dwParam1 = hDirectDraw
#define DDNOTIFY_POSTRESCHANGE			0x0010	// dwParam1 = hDirectDraw
#define DDNOTIFY_PREDOSBOX			0x0020  // dwParam1 = hDirectDraw
#define DDNOTIFY_POSTDOSBOX			0x0040  // dwParam1 = hDirectDraw
#define DDNOTIFY_CLOSEDIRECTDRAW		0x0080  // dwParam1 = hDirectDraw
#define DDNOTIFY_CLOSESURFACE			0x0100  // dwParam1 = hSurface
#define DDNOTIFY_CLOSEVIDEOPORT			0x0200  // dwParam1 = hVideoPort
#define DDNOTIFY_CLOSECAPTURE			0x0400  // dwParam1 = hCapture

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif



