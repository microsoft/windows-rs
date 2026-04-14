/*==========================================================================;
 *
 *  Copyright (C) Microsoft Corporation.  All Rights Reserved.
 *
 *  File:	dvp.h
 *  Content:	DirectDrawVideoPort include file
 *
 ***************************************************************************/

#ifndef __DVP_INCLUDED__
#define __DVP_INCLUDED__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


/*
 * GUIDS used by DirectDrawVideoPort objects
 */
#if defined( _WIN32 ) && (!defined( _NO_COM ) || defined( DEFINE_GUID ))
DEFINE_GUID( IID_IDDVideoPortContainer,		0x6C142760,0xA733,0x11CE,0xA5,0x21,0x00,0x20,0xAF,0x0B,0xE5,0x60 );
DEFINE_GUID( IID_IDirectDrawVideoPort,		0xB36D93E0,0x2B43,0x11CF,0xA2,0xDE,0x00,0xAA,0x00,0xB9,0x33,0x56 );
DEFINE_GUID( IID_IDirectDrawVideoPortNotify,    0xA655FB94,0x0589,0x4E57,0xB3,0x33,0x56,0x7A,0x89,0x46,0x8C,0x88);



DEFINE_GUID( DDVPTYPE_E_HREFH_VREFH, 0x54F39980L,0xDA60,0x11CF,0x9B,0x06,0x00,0xA0,0xC9,0x03,0xA3,0xB8);
DEFINE_GUID( DDVPTYPE_E_HREFH_VREFL, 0x92783220L,0xDA60,0x11CF,0x9B,0x06,0x00,0xA0,0xC9,0x03,0xA3,0xB8);
DEFINE_GUID( DDVPTYPE_E_HREFL_VREFH, 0xA07A02E0L,0xDA60,0x11CF,0x9B,0x06,0x00,0xA0,0xC9,0x03,0xA3,0xB8);
DEFINE_GUID( DDVPTYPE_E_HREFL_VREFL, 0xE09C77E0L,0xDA60,0x11CF,0x9B,0x06,0x00,0xA0,0xC9,0x03,0xA3,0xB8);
DEFINE_GUID( DDVPTYPE_CCIR656,	     0xFCA326A0L,0xDA60,0x11CF,0x9B,0x06,0x00,0xA0,0xC9,0x03,0xA3,0xB8);
DEFINE_GUID( DDVPTYPE_BROOKTREE,     0x1352A560L,0xDA61,0x11CF,0x9B,0x06,0x00,0xA0,0xC9,0x03,0xA3,0xB8);
DEFINE_GUID( DDVPTYPE_PHILIPS,	     0x332CF160L,0xDA61,0x11CF,0x9B,0x06,0x00,0xA0,0xC9,0x03,0xA3,0xB8);
#endif

#ifndef GUID_DEFS_ONLY

#if defined( _WIN32 )  && !defined( _NO_COM )
#define COM_NO_WINDOWS_H
#include <objbase.h>
#else
#define IUnknown	    void
#endif

/*
 * These definitions are required to allow polymorphic structure members (i.e. those
 * that are referred to both as DWORDs and as pointers) to resolve into a type
 * of correct size to hold the largest of those two types (i.e. pointer) on 64 bit
 * systems. For 32 bit environments, ULONG_PTR resolves to a DWORD.
 */
#ifndef MAXULONG_PTR
#define ULONG_PTR    DWORD
#endif //MAXULONG_PTR

#ifdef __cplusplus
extern "C" {
#endif

/*============================================================================
 *
 * DirectDraw Structures
 *
 * Various structures used to invoke DirectDraw.
 *
 *==========================================================================*/

struct IDirectDraw;
struct IDirectDrawSurface;
struct IDirectDrawPalette;
struct IDirectDrawClipper;

typedef struct IDDVideoPortContainer            FAR *LPDDVIDEOPORTCONTAINER;
typedef struct IDirectDrawVideoPort             FAR *LPDIRECTDRAWVIDEOPORT;
typedef struct IDirectDrawVideoPortNotify       FAR *LPDIRECTDRAWVIDEOPORTNOTIFY;

typedef struct _DDVIDEOPORTCONNECT              FAR *LPDDVIDEOPORTCONNECT;
typedef struct _DDVIDEOPORTCAPS                 FAR *LPDDVIDEOPORTCAPS;
typedef struct _DDVIDEOPORTDESC                 FAR *LPDDVIDEOPORTDESC;
typedef struct _DDVIDEOPORTINFO                 FAR *LPDDVIDEOPORTINFO;
typedef struct _DDVIDEOPORTBANDWIDTH            FAR *LPDDVIDEOPORTBANDWIDTH;
typedef struct _DDVIDEOPORTSTATUS               FAR *LPDDVIDEOPORTSTATUS;
typedef struct _DDVIDEOPORTNOTIFY               FAR *LPDDVIDEOPORTNOTIFY;

typedef struct IDDVideoPortContainerVtbl        DDVIDEOPORTCONTAINERCALLBACKS;
typedef struct IDirectDrawVideoPortVtbl         DIRECTDRAWVIDEOPORTCALLBACKS;
typedef struct IDirectDrawVideoPortNotifyVtbl   DIRECTDRAWVIDEOPORTNOTIFYCALLBACKS;


/*
 * API's
 */
typedef HRESULT (FAR PASCAL * LPDDENUMVIDEOCALLBACK)(LPDDVIDEOPORTCAPS, LPVOID);


/*
 * INTERACES FOLLOW:
 *	IDirectDrawVideoPort
 *	IVideoPort
 */

/*
 * IDirectDrawVideoPortContainer
 */
#if defined( _WIN32 ) && !defined( _NO_COM )
#undef INTERFACE
#define INTERFACE IDDVideoPortContainer
DECLARE_INTERFACE_( IDDVideoPortContainer, IUnknown )
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, LPVOID FAR * ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;
    /*** IDirectDrawVideoPort methods ***/
    STDMETHOD(CreateVideoPort)(THIS_ DWORD, LPDDVIDEOPORTDESC, LPDIRECTDRAWVIDEOPORT FAR *, IUnknown FAR *) PURE;
    STDMETHOD(EnumVideoPorts)(THIS_ DWORD, LPDDVIDEOPORTCAPS, LPVOID,LPDDENUMVIDEOCALLBACK ) PURE;
    STDMETHOD(GetVideoPortConnectInfo)(THIS_ DWORD, _Inout_ LPDWORD pcInfo, _Out_writes_to_opt_(*pcInfo, *pcInfo) LPDDVIDEOPORTCONNECT ) PURE;
    STDMETHOD(QueryVideoPortStatus)(THIS_ DWORD, LPDDVIDEOPORTSTATUS ) PURE;
};

#if !defined(__cplusplus) || defined(CINTERFACE)
#define IVideoPortContainer_QueryInterface(p, a, b)         (p)->lpVtbl->QueryInterface(p, a, b)
#define IVideoPortContainer_AddRef(p)                       (p)->lpVtbl->AddRef(p)
#define IVideoPortContainer_Release(p)                      (p)->lpVtbl->Release(p)
#define IVideoPortContainer_CreateVideoPort(p, a, b, c, d)  (p)->lpVtbl->CreateVideoPort(p, a, b, c, d)
#define IVideoPortContainer_EnumVideoPorts(p, a, b, c, d)   (p)->lpVtbl->EnumVideoPorts(p, a, b, c, d)
#define IVideoPortContainer_GetVideoPortConnectInfo(p, a, b, c) (p)->lpVtbl->GetVideoPortConnectInfo(p, a, b, c)
#define IVideoPortContainer_QueryVideoPortStatus(p, a, b)   (p)->lpVtbl->QueryVideoPortStatus(p, a, b)
#else
#define IVideoPortContainer_QueryInterface(p, a, b)         (p)->QueryInterface(a, b)
#define IVideoPortContainer_AddRef(p)                       (p)->AddRef()
#define IVideoPortContainer_Release(p)                      (p)->Release()
#define IVideoPortContainer_CreateVideoPort(p, a, b, c, d)  (p)->CreateVideoPort(a, b, c, d)
#define IVideoPortContainer_EnumVideoPorts(p, a, b, c, d)   (p)->EnumVideoPorts(a, b, c, d)
#define IVideoPortContainer_GetVideoPortConnectInfo(p, a, b, c) (p)->GetVideoPortConnectInfo(a, b, c)
#define IVideoPortContainer_QueryVideoPortStatus(p, a, b)   (p)->QueryVideoPortStatus(a, b)
#endif

#endif


/*
 * IDirectDrawVideoPort
 */
#if defined( _WIN32 ) && !defined( _NO_COM )
#undef INTERFACE
#define INTERFACE IDirectDrawVideoPort
DECLARE_INTERFACE_( IDirectDrawVideoPort, IUnknown )
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, LPVOID FAR * ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;
    /*** IVideoPort methods ***/
    STDMETHOD(Flip)(THIS_ LPDIRECTDRAWSURFACE, DWORD) PURE;
    STDMETHOD(GetBandwidthInfo)(THIS_ LPDDPIXELFORMAT, DWORD, DWORD, DWORD, LPDDVIDEOPORTBANDWIDTH) PURE;
    STDMETHOD(GetColorControls)(THIS_ LPDDCOLORCONTROL) PURE;
    STDMETHOD(GetInputFormats)(THIS_ LPDWORD lpNumFormats, _Out_writes_to_opt_(*lpNumFormats, *lpNumFormats) LPDDPIXELFORMAT, DWORD) PURE;
    STDMETHOD(GetOutputFormats)(THIS_ LPDDPIXELFORMAT, LPDWORD lpNumFormats, _Out_writes_to_opt_(*lpNumFormats, *lpNumFormats) LPDDPIXELFORMAT, DWORD) PURE;
    STDMETHOD(GetFieldPolarity)(THIS_ LPBOOL) PURE;
    STDMETHOD(GetVideoLine)(THIS_ LPDWORD) PURE;
    STDMETHOD(GetVideoSignalStatus)(THIS_ LPDWORD) PURE;
    STDMETHOD(SetColorControls)(THIS_ LPDDCOLORCONTROL) PURE;
    STDMETHOD(SetTargetSurface)(THIS_ LPDIRECTDRAWSURFACE, DWORD) PURE;
    STDMETHOD(StartVideo)(THIS_ LPDDVIDEOPORTINFO) PURE;
    STDMETHOD(StopVideo)(THIS) PURE;
    STDMETHOD(UpdateVideo)(THIS_ LPDDVIDEOPORTINFO) PURE;
    STDMETHOD(WaitForSync)(THIS_ DWORD, DWORD, DWORD) PURE;
};

#if !defined(__cplusplus) || defined(CINTERFACE)
#define IVideoPort_QueryInterface(p,a,b)        (p)->lpVtbl->QueryInterface(p,a,b)
#define IVideoPort_AddRef(p)                    (p)->lpVtbl->AddRef(p)
#define IVideoPort_Release(p)                   (p)->lpVtbl->Release(p)
#define IVideoPort_SetTargetSurface(p,a,b)	(p)->lpVtbl->SetTargetSurface(p,a,b)
#define IVideoPort_Flip(p,a,b)			(p)->lpVtbl->Flip(p,a,b)
#define IVideoPort_GetBandwidthInfo(p,a,b,c,d,e) (p)->lpVtbl->GetBandwidthInfo(p,a,b,c,d,e)
#define IVideoPort_GetColorControls(p,a) 	(p)->lpVtbl->GetColorControls(p,a)
#define IVideoPort_GetInputFormats(p,a,b,c)	(p)->lpVtbl->GetInputFormats(p,a,b,c)
#define IVideoPort_GetOutputFormats(p,a,b,c,d)	(p)->lpVtbl->GetOutputFormats(p,a,b,c,d)
#define IVideoPort_GetFieldPolarity(p,a)	(p)->lpVtbl->GetFieldPolarity(p,a)
#define IVideoPort_GetVideoLine(p,a)		(p)->lpVtbl->GetVideoLine(p,a)
#define IVideoPort_GetVideoSignalStatus(p,a)	(p)->lpVtbl->GetVideoSignalStatus(p,a)
#define IVideoPort_SetColorControls(p,a)	(p)->lpVtbl->SetColorControls(p,a)
#define IVideoPort_StartVideo(p,a)		(p)->lpVtbl->StartVideo(p,a)
#define IVideoPort_StopVideo(p)			(p)->lpVtbl->StopVideo(p)
#define IVideoPort_UpdateVideo(p,a)		(p)->lpVtbl->UpdateVideo(p,a)
#define IVideoPort_WaitForSync(p,a,b,c)		(p)->lpVtbl->WaitForSync(p,a,b,c)
#else
#define IVideoPort_QueryInterface(p,a,b)        (p)->QueryInterface(a,b)
#define IVideoPort_AddRef(p)                    (p)->AddRef()
#define IVideoPort_Release(p)                   (p)->Release()
#define IVideoPort_SetTargetSurface(p,a,b)	(p)->SetTargetSurface(a,b)
#define IVideoPort_Flip(p,a,b)			(p)->Flip(a,b)
#define IVideoPort_GetBandwidthInfo(p,a,b,c,d,e) (p)->GetBandwidthInfo(a,b,c,d,e)
#define IVideoPort_GetColorControls(p,a) 	(p)->GetColorControls(a)
#define IVideoPort_GetInputFormats(p,a,b,c)	(p)->GetInputFormats(a,b,c)
#define IVideoPort_GetOutputFormats(p,a,b,c,d)	(p)->GetOutputFormats(a,b,c,d)
#define IVideoPort_GetFieldPolarity(p,a)	(p)->GetFieldPolarity(a)
#define IVideoPort_GetVideoLine(p,a)		(p)->GetVideoLine(a)
#define IVideoPort_GetVideoSignalStatus(p,a)	(p)->GetVideoSignalStatus(a)
#define IVideoPort_SetColorControls(p,a)	(p)->SetColorControls(a)
#define IVideoPort_StartVideo(p,a)		(p)->StartVideo(a)
#define IVideoPort_StopVideo(p)			(p)->StopVideo()
#define IVideoPort_UpdateVideo(p,a)		(p)->UpdateVideo(a)
#define IVideoPort_WaitForSync(p,a,b,c)		(p)->WaitForSync(a,b,c)
#endif

#endif

/*
 * IDirectDrawVideoPort
 */
#if (_WIN32_WINNT >= _WIN32_WINNT_WINXP)
#if defined( _WIN32 ) && !defined( _NO_COM )
#undef INTERFACE
#define INTERFACE IDirectDrawVideoPortNotify
DECLARE_INTERFACE_( IDirectDrawVideoPortNotify, IUnknown )
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, LPVOID FAR * ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;
    /*** IVideoPort methods ***/
    STDMETHOD(AcquireNotification)(THIS_ HANDLE *, LPDDVIDEOPORTNOTIFY) PURE;
    STDMETHOD(ReleaseNotification)(THIS_ HANDLE) PURE;
};

#if !defined(__cplusplus) || defined(CINTERFACE)
#define IVideoPortNotify_QueryInterface(p,a,b)      (p)->lpVtbl->QueryInterface(p,a,b)
#define IVideoPortNotify_AddRef(p)                  (p)->lpVtbl->AddRef(p)
#define IVideoPortNotify_Release(p)                 (p)->lpVtbl->Release(p)
#define IVideoPortNotify_AcquireNotification(p,a,b) (p)->lpVtbl->AcquireNotification(p,a,b)
#define IVideoPortNotify_ReleaseNotification(p,a)   (p)->lpVtbl->ReleaseNotification(p,a)
#else
#define IVideoPortNotify_QueryInterface(p,a,b)      (p)->QueryInterface(a,b)
#define IVideoPortNotify_AddRef(p)                  (p)->AddRef()
#define IVideoPortNotify_Release(p)                 (p)->Release()
#define IVideoPortNotify_AcquireNotification(p,a,b) (p)->lpVtbl->AcquireNotification(a,b)
#define IVideoPortNotify_ReleaseNotification(p,a)   (p)->lpVtbl->ReleaseNotification(a)
#endif

#endif
#endif

/*
 * DDVIDEOPORTCONNECT
 */
typedef struct _DDVIDEOPORTCONNECT
{
    DWORD dwSize;           // size of the DDVIDEOPORTCONNECT structure
    DWORD dwPortWidth;      // Width of the video port
    GUID  guidTypeID;       // Description of video port connection
    DWORD dwFlags;          // Connection flags
    ULONG_PTR dwReserved1;      // Reserved, set to zero.
} DDVIDEOPORTCONNECT;


/*
 * DDVIDEOPORTCAPS
 */
typedef struct _DDVIDEOPORTCAPS
{
    DWORD dwSize;			// size of the DDVIDEOPORTCAPS structure
    DWORD dwFlags;			// indicates which fields contain data
    DWORD dwMaxWidth;			// max width of the video port field
    DWORD dwMaxVBIWidth;		// max width of the VBI data
    DWORD dwMaxHeight; 			// max height of the video port field
    DWORD dwVideoPortID;		// Video port ID (0 - (dwMaxVideoPorts -1))
    DWORD dwCaps;			// Video port capabilities
    DWORD dwFX;				// More video port capabilities
    DWORD dwNumAutoFlipSurfaces;	// Max number of autoflippable surfaces allowed
    DWORD dwAlignVideoPortBoundary;	// Byte restriction of placement within the surface
    DWORD dwAlignVideoPortPrescaleWidth;// Byte restriction of width after prescaling
    DWORD dwAlignVideoPortCropBoundary;	// Byte restriction of left cropping
    DWORD dwAlignVideoPortCropWidth;	// Byte restriction of cropping width
    DWORD dwPreshrinkXStep;		// Width can be shrunk in steps of 1/x
    DWORD dwPreshrinkYStep;		// Height can be shrunk in steps of 1/x
    DWORD dwNumVBIAutoFlipSurfaces;	// Max number of VBI autoflippable surfaces allowed
    DWORD dwNumPreferredAutoflip;	// Optimal number of autoflippable surfaces for hardware
    WORD  wNumFilterTapsX;              // Number of taps the prescaler uses in the X direction (0 - no prescale, 1 - replication, etc.)
    WORD  wNumFilterTapsY;              // Number of taps the prescaler uses in the Y direction (0 - no prescale, 1 - replication, etc.)
} DDVIDEOPORTCAPS;

/*
 * The dwMaxWidth and dwMaxVBIWidth members are valid
 */
#define DDVPD_WIDTH		0x00000001l

/*
 * The dwMaxHeight member is valid
 */
#define DDVPD_HEIGHT		0x00000002l

/*
 * The dwVideoPortID member is valid
 */
#define DDVPD_ID		0x00000004l

/*
 * The dwCaps member is valid
 */
#define DDVPD_CAPS		0x00000008l

/*
 * The dwFX member is valid
 */
#define DDVPD_FX		0x00000010l

/*
 * The dwNumAutoFlipSurfaces member is valid
 */
#define DDVPD_AUTOFLIP		0x00000020l

/*
 * All of the alignment members are valid
 */
#define DDVPD_ALIGN		0x00000040l

/*
 * The dwNumPreferredAutoflip member is valid
 */
#define DDVPD_PREFERREDAUTOFLIP 0x00000080l

/*
 * The wNumFilterTapsX and wNumFilterTapsY fields are valid
 */
#define DDVPD_FILTERQUALITY     0x00000100l

/*
 * DDVIDEOPORTDESC
 */
typedef struct _DDVIDEOPORTDESC
{
    DWORD dwSize;			// size of the DDVIDEOPORTDESC structure
    DWORD dwFieldWidth;			// width of the video port field
    DWORD dwVBIWidth;			// width of the VBI data
    DWORD dwFieldHeight;		// height of the video port field
    DWORD dwMicrosecondsPerField;	// Microseconds per video field
    DWORD dwMaxPixelsPerSecond;		// Maximum pixel rate per second
    DWORD dwVideoPortID;		// Video port ID (0 - (dwMaxVideoPorts -1))
    DWORD dwReserved1;			// Reserved for future use - set to zero (struct padding)
    DDVIDEOPORTCONNECT VideoPortType; 	// Description of video port connection
    ULONG_PTR dwReserved2;		// Reserved for future use - set to zero
    ULONG_PTR dwReserved3;		// Reserved for future use - set to zero
} DDVIDEOPORTDESC;


/*
 * DDVIDEOPORTINFO
 */
typedef struct _DDVIDEOPORTINFO
{
    DWORD dwSize;			// Size of the structure
    DWORD dwOriginX;			// Placement of the video data within the surface.
    DWORD dwOriginY;			// Placement of the video data within the surface.
    DWORD dwVPFlags;			// Video port options
    RECT rCrop;				// Cropping rectangle (optional).
    DWORD dwPrescaleWidth;		// Determines pre-scaling/zooming in the X direction (optional).
    DWORD dwPrescaleHeight;		// Determines pre-scaling/zooming in the Y direction (optional).
    LPDDPIXELFORMAT lpddpfInputFormat;	// Video format written to the video port
    LPDDPIXELFORMAT lpddpfVBIInputFormat; // Input format of the VBI data
    LPDDPIXELFORMAT lpddpfVBIOutputFormat;// Output format of the data
    DWORD dwVBIHeight;			// Specifies the number of lines of data within the vertical blanking interval.
    ULONG_PTR dwReserved1;		// Reserved for future use - set to zero
    ULONG_PTR dwReserved2;		// Reserved for future use - set to zero
} DDVIDEOPORTINFO;


/*
 * DDVIDEOPORTBANDWIDTH
 */
typedef struct _DDVIDEOPORTBANDWIDTH
{
    DWORD dwSize;			// Size of the structure
    DWORD dwCaps;
    DWORD dwOverlay;           		// Zoom factor at which overlay is supported
    DWORD dwColorkey;			// Zoom factor at which overlay w/ colorkey is supported
    DWORD dwYInterpolate;		// Zoom factor at which overlay w/ Y interpolation is supported
    DWORD dwYInterpAndColorkey;		// Zoom factor at which ovelray w/ Y interpolation and colorkeying is supported
    ULONG_PTR dwReserved1;		// Reserved for future use - set to zero
    ULONG_PTR dwReserved2;		// Reserved for future use - set to zero
} DDVIDEOPORTBANDWIDTH;


/*
 * DDVIDEOPORTSTATUS
 */
typedef struct _DDVIDEOPORTSTATUS
{
    DWORD dwSize;			// Size of the structure
    BOOL  bInUse;			// TRUE if video port is currently being used
    DWORD dwFlags;           		// Currently not used
    DWORD dwReserved1;			// Reserved for future use
    DDVIDEOPORTCONNECT VideoPortType;	// Information about the connection
    ULONG_PTR dwReserved2;		// Reserved for future use
    ULONG_PTR dwReserved3;		// Reserved for future use
} DDVIDEOPORTSTATUS;

/*
 * DDVIDEOPORTNOTIFY
 */
typedef struct _DDVIDEOPORTNOTIFY
{
    LARGE_INTEGER ApproximateTimeStamp;	// Timestamp in the event notification
    LONG lField;                        // 0 if even, 1 if odd, -1 if unknown
    UINT dwSurfaceIndex;                // Index in the surface chain of the surface that received the sample
    LONG lDone;                         // Call InterlockedIncrement on this when done with sample
} DDVIDEOPORTNOTIFY;


/*============================================================================
 *
 * Video Port Flags
 *
 * All flags are bit flags.
 *
 *==========================================================================*/

/****************************************************************************
 *
 * VIDEOPORT DDVIDEOPORTCONNECT FLAGS
 *
 ****************************************************************************/

/*
 * When this is set by the driver and passed to the client, this
 * indicates that the video port is capable of double clocking the data.
 * When this is set by the client, this indicates that the video port
 * should enable double clocking.  This flag is only valid with external
 * syncs.
 */
#define DDVPCONNECT_DOUBLECLOCK			0x00000001l

/*
 * When this is set by the driver and passed to the client, this
 * indicates that the video port is capable of using an external VACT
 * signal. When this is set by the client, this indicates that the
 * video port should use the external VACT signal.
 */
#define DDVPCONNECT_VACT			0x00000002l

/*
 * When this is set by the driver and passed to the client, this
 * indicates that the video port is capable of treating even fields
 * like odd fields and visa versa.  When this is set by the client,
 * this indicates that the video port should treat even fields like odd
 * fields.
 */
#define DDVPCONNECT_INVERTPOLARITY		0x00000004l

/*
 * Indicates that any data written to the video port during the VREF
 * period will not be written into the frame buffer. This flag is read only.
 */
#define DDVPCONNECT_DISCARDSVREFDATA		0x00000008l

/*
 * When this is set be the driver and passed to the client, this
 * indicates that the device will write half lines into the frame buffer
 * if half lines are provided by the decoder.  If this is set by the client,
 * this indicates that the decoder will be supplying half lines.
 */
#define DDVPCONNECT_HALFLINE			0x00000010l

/*
 * Indicates that the signal is interlaced. This flag is only
 * set by the client.
 */
#define DDVPCONNECT_INTERLACED			0x00000020l

/*
 * Indicates that video port is shareable and that this video port
 * will use the even fields.  This flag is only set by the client.
 */
#define DDVPCONNECT_SHAREEVEN			0x00000040l

/*
 * Indicates that video port is shareable and that this video port
 * will use the odd fields.  This flag is only set by the client.
 */
#define DDVPCONNECT_SHAREODD			0x00000080l

/****************************************************************************
 *
 * VIDEOPORT DDVIDEOPORTDESC CAPS
 *
 ****************************************************************************/

/*
 * Flip can be performed automatically to avoid tearing.
 */
#define DDVPCAPS_AUTOFLIP			0x00000001l

/*
 * Supports interlaced video
 */
#define DDVPCAPS_INTERLACED			0x00000002l

/*
 * Supports non-interlaced video
 */
#define DDVPCAPS_NONINTERLACED			0x00000004l

/*
 * Indicates that the device can return whether the current field
 * of an interlaced signal is even or odd.
 */
#define DDVPCAPS_READBACKFIELD			0x00000008l

/*
 * Indicates that the device can return the current line of video
 * being written into the frame buffer.
 */
#define DDVPCAPS_READBACKLINE			0x00000010l

/*
 * Allows two gen-locked video streams to share a single video port,
 * where one stream uses the even fields and the other uses the odd
 * fields. Separate parameters (including address, scaling,
 * cropping, etc.) are maintained for both fields.)
 */
#define DDVPCAPS_SHAREABLE			0x00000020l

/*
 * Even fields of video can be automatically discarded.
 */
#define DDVPCAPS_SKIPEVENFIELDS			0x00000040l

/*
 * Odd fields of video can be automatically discarded.
 */
#define DDVPCAPS_SKIPODDFIELDS			0x00000080l

/*
 * Indicates that the device is capable of driving the graphics
 * VSYNC with the video port VSYNC.
 */
#define DDVPCAPS_SYNCMASTER			0x00000100l

/*
 * Indicates that data within the vertical blanking interval can
 * be written to a different surface.
 */
#define DDVPCAPS_VBISURFACE			0x00000200l

/*
 * Indicates that the video port can perform color operations
 * on the incoming data before it is written to the frame buffer.
 */
#define DDVPCAPS_COLORCONTROL			0x00000400l

/*
 * Indicates that the video port can accept VBI data in a different
 * width or format than the regular video data.
 */
#define DDVPCAPS_OVERSAMPLEDVBI			0x00000800l

/*
 * Indicates that the video port can write data directly to system memory
 */
#define DDVPCAPS_SYSTEMMEMORY			0x00001000l

/*
 * Indicates that the VBI and video portions of the video stream can
 * be controlled by an independent processes.
 */
#define DDVPCAPS_VBIANDVIDEOINDEPENDENT		0x00002000l

/*
 * Indicates that the video port contains high quality hardware
 * de-interlacing hardware that should be used instead of the
 * bob/weave algorithms.
 */
#define DDVPCAPS_HARDWAREDEINTERLACE		0x00004000l


/****************************************************************************
 *
 * VIDEOPORT DDVIDEOPORTDESC FX
 *
 ****************************************************************************/

/*
 * Limited cropping is available to crop out the vertical interval data.
 */
#define DDVPFX_CROPTOPDATA			0x00000001l

/*
 * Incoming data can be cropped in the X direction before it is written
 * to the surface.
 */
#define DDVPFX_CROPX				0x00000002l

/*
 * Incoming data can be cropped in the Y direction before it is written
 * to the surface.
 */
#define DDVPFX_CROPY				0x00000004l

/*
 * Supports interleaving interlaced fields in memory.
 */
#define DDVPFX_INTERLEAVE			0x00000008l

/*
 * Supports mirroring left to right as the video data is written
 * into the frame buffer.
 */
#define DDVPFX_MIRRORLEFTRIGHT			0x00000010l

/*
 * Supports mirroring top to bottom as the video data is written
 * into the frame buffer.
 */
#define DDVPFX_MIRRORUPDOWN			0x00000020l

/*
 * Data can be arbitrarily shrunk in the X direction before it
 * is written to the surface.
 */
#define DDVPFX_PRESHRINKX			0x00000040l

/*
 * Data can be arbitrarily shrunk in the Y direction before it
 * is written to the surface.
 */
#define DDVPFX_PRESHRINKY			0x00000080l

/*
 * Data can be binary shrunk (1/2, 1/4, 1/8, etc.) in the X
 * direction before it is written to the surface.
 */
#define DDVPFX_PRESHRINKXB			0x00000100l

/*
 * Data can be binary shrunk (1/2, 1/4, 1/8, etc.) in the Y
 * direction before it is written to the surface.
 */
#define DDVPFX_PRESHRINKYB			0x00000200l

/*
 * Data can be shrunk in increments of 1/x in the X direction
 * (where X is specified in the DDVIDEOPORTCAPS.dwPreshrinkXStep)
 * before it is written to the surface.
 */
#define DDVPFX_PRESHRINKXS			0x00000400l

/*
 * Data can be shrunk in increments of 1/x in the Y direction
 * (where X is specified in the DDVIDEOPORTCAPS.dwPreshrinkYStep)
 * before it is written to the surface.
 */
#define DDVPFX_PRESHRINKYS			0x00000800l

/*
 * Data can be arbitrarily stretched in the X direction before
 * it is written to the surface.
 */
#define DDVPFX_PRESTRETCHX			0x00001000l

/*
 * Data can be arbitrarily stretched in the Y direction before
 * it is written to the surface.
 */
#define DDVPFX_PRESTRETCHY			0x00002000l

/*
 * Data can be integer stretched in the X direction before it is
 * written to the surface.
 */
#define DDVPFX_PRESTRETCHXN			0x00004000l

/*
 * Data can be integer stretched in the Y direction before it is
 * written to the surface.
 */
#define DDVPFX_PRESTRETCHYN			0x00008000l

/*
 * Indicates that data within the vertical blanking interval can
 * be converted independently of the remaining video data.
 */
#define DDVPFX_VBICONVERT			0x00010000l

/*
 * Indicates that scaling can be disabled for data within the
 * vertical blanking interval.
 */
#define DDVPFX_VBINOSCALE			0x00020000l

/*
 * Indicates that the video data can ignore the left and right
 * cropping coordinates when cropping oversampled VBI data.
 */
#define DDVPFX_IGNOREVBIXCROP			0x00040000l

/*
 * Indicates that interleaving can be disabled for data within the
 * vertical blanking interval.
 */
#define DDVPFX_VBINOINTERLEAVE			0x00080000l


/****************************************************************************
 *
 * VIDEOPORT DDVIDEOPORTINFO FLAGS
 *
 ****************************************************************************/

/*
 * Perform automatic flipping.   Auto-flipping is performed between
 * the overlay surface that was attached to the video port using
 * IDirectDrawVideoPort::AttachSurface and the overlay surfaces that
 * are attached to the surface via the IDirectDrawSurface::AttachSurface
 * method.  The flip order is the order in which the overlay surfaces
 * were. attached.
 */
#define DDVP_AUTOFLIP				0x00000001l

/*
 * Perform conversion using the ddpfOutputFormat information.
 */
#define DDVP_CONVERT				0x00000002l

/*
 * Perform cropping using the specified rectangle.
 */
#define DDVP_CROP				0x00000004l

/*
 * Indicates that interlaced fields should be interleaved in memory.
 */
#define DDVP_INTERLEAVE				0x00000008l

/*
 * Indicates that the data should be mirrored left to right as it's
 * written into the frame buffer.
 */
#define DDVP_MIRRORLEFTRIGHT			0x00000010l

/*
 * Indicates that the data should be mirrored top to bottom as it's
 * written into the frame buffer.
 */
#define DDVP_MIRRORUPDOWN			0x00000020l

/*
 * Perform pre-scaling/zooming based on the pre-scale parameters.
 */
#define DDVP_PRESCALE				0x00000040l

/*
 * Ignore input of even fields.
 */
#define DDVP_SKIPEVENFIELDS			0x00000080l

/*
 * Ignore input of odd fields.
 */
#define DDVP_SKIPODDFIELDS			0x00000100l

/*
 * Drive the graphics VSYNCs using the video port VYSNCs.
 */
#define DDVP_SYNCMASTER				0x00000200l

/*
 * The ddpfVBIOutputFormatFormat member contains data that should be used
 * to convert the data within the vertical blanking interval.
 */
#define DDVP_VBICONVERT				0x00000400l

/*
 * Indicates that data within the vertical blanking interval
 * should not be scaled.
 */
#define DDVP_VBINOSCALE				0x00000800l

/*
 * Indicates that these bob/weave decisions should not be
 * overriden by other interfaces.
 */
#define DDVP_OVERRIDEBOBWEAVE			0x00001000l

/*
 * Indicates that the video data should ignore the left and right
 * cropping coordinates when cropping the VBI data.
 */
#define DDVP_IGNOREVBIXCROP			0x00002000l

/*
 * Indicates that interleaving can be disabled for data within the
 * vertical blanking interval.
 */
#define DDVP_VBINOINTERLEAVE			0x00004000l

/*
 * Indicates that the video port should use the hardware
 * de-interlacing hardware.
 */
#define DDVP_HARDWAREDEINTERLACE		0x00008000l

/****************************************************************************
 *
 * DIRIRECTDRAWVIDEOPORT GETINPUTFORMAT/GETOUTPUTFORMAT FLAGS
 *
 ****************************************************************************/

/*
 * Return formats for the video data
 */
#define DDVPFORMAT_VIDEO			0x00000001l

/*
 * Return formats for the VBI data
 */
#define DDVPFORMAT_VBI				0x00000002l

/****************************************************************************
 *
 * DIRIRECTDRAWVIDEOPORT SETTARGETSURFACE FLAGS
 *
 ****************************************************************************/

/*
 * Surface should receive video data (and VBI data if a surface
 * is not explicitly attached for that purpose)
 */
#define DDVPTARGET_VIDEO			0x00000001l

/*
 * Surface should receive VBI data
 */
#define DDVPTARGET_VBI				0x00000002l


/****************************************************************************
 *
 * DIRIRECTDRAWVIDEOPORT WAITFORSYNC FLAGS
 *
 ****************************************************************************/

/*
 * Waits until the beginning of the next VSYNC
 */
#define DDVPWAIT_BEGIN				0x00000001l

/*
 * Waits until the end of the next/current VSYNC
 */
#define DDVPWAIT_END				0x00000002l

/*
 * Waits until the beginning of the specified line
 */
#define DDVPWAIT_LINE				0x00000003l

/****************************************************************************
 *
 * DIRECTDRAWVIDEOPORT FLIP FLAGS
 *
 ****************************************************************************/

/*
 * Flips the normal video surface
 */
#define DDVPFLIP_VIDEO				0x00000001l

/*
 * Flips the VBI surface
 */
#define DDVPFLIP_VBI				0x00000002l

/****************************************************************************
 *
 * DIRIRECTDRAWVIDEOPORT GETVIDEOSIGNALSTATUS VALUES
 *
 ****************************************************************************/

/*
 * No video signal is present at the video port
 */
#define DDVPSQ_NOSIGNAL				0x00000001l

/*
 * A valid video signal is present at the video port
 */
#define DDVPSQ_SIGNALOK				0x00000002l

/****************************************************************************
 *
 * VIDEOPORTBANDWIDTH Flags
 *
 ****************************************************************************/

/*
 * The specified height/width refer to the size of the video port data
 * written into memory, after prescaling has occured.
 */
#define DDVPB_VIDEOPORT				0x00000001l

/*
 * The specified height/width refer to the source size of the overlay.
 */
#define DDVPB_OVERLAY				0x00000002l

/*
 * This is a query for the device to return which caps this device requires.
 */
#define DDVPB_TYPE				0x00000004l

/****************************************************************************
 *
 * VIDEOPORTBANDWIDTH Caps
 *
 ****************************************************************************/

/*
 * The bandwidth for this device is dependant on the overlay source size.
 */
#define DDVPBCAPS_SOURCE			0x00000001l

/*
 * The bandwidth for this device is dependant on the overlay destination
 * size.
 */
#define DDVPBCAPS_DESTINATION			0x00000002l

/****************************************************************************
 *
 * DDVIDEOPORTCONTAINER CreateVideoPort flags
 *
 ****************************************************************************/

/*
 * The process only wants to control the VBI portion of the video stream.
 */
#define DDVPCREATE_VBIONLY			0x00000001l

/*
 * The process only wants to control the non-VBI (video) portion of
 * the video stream.
 */
#define DDVPCREATE_VIDEOONLY			0x00000002l

/****************************************************************************
 *
 * DDVIDEOPORTSTATUS flags
 *
 ****************************************************************************/

/*
 * The video port interface is only controlling the VBI portion of the
 * video stream
 */
#define DDVPSTATUS_VBIONLY			0x00000001l

/*
 * The video port interface is only controlling the video portion of the
 * video stream
 */
#define DDVPSTATUS_VIDEOONLY			0x00000002l


#ifdef __cplusplus
};
#endif

#endif  // GUID_DEFS_ONLY


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

