// Copyright (C) Microsoft Corporation. All rights reserved.
#ifndef _DWMAPI_H_
#define _DWMAPI_H_

#include <winapifamily.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifndef DWMAPI
#if !defined(_DWMAPI_)
#define DWMAPI          EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define DWMAPI_(type)   EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#else
#define DWMAPI          STDAPI
#define DWMAPI_(type)   STDAPI_(type)
#endif /* _DWMAPI_ */
#endif /* DWMAPI */

#ifndef MILCORE_KERNEL_COMPONENT
#include <wtypes.h>
#include <uxtheme.h>
#endif

#include <pshpack1.h>

// Blur behind data structures
#define DWM_BB_ENABLE                 0x00000001  // fEnable has been specified
#define DWM_BB_BLURREGION             0x00000002  // hRgnBlur has been specified
#define DWM_BB_TRANSITIONONMAXIMIZED  0x00000004  // fTransitionOnMaximized has been specified

typedef struct _DWM_BLURBEHIND
{
    DWORD dwFlags;
    BOOL fEnable;
    HRGN hRgnBlur;
    BOOL fTransitionOnMaximized;
} DWM_BLURBEHIND, *PDWM_BLURBEHIND;

// Window attributes
enum DWMWINDOWATTRIBUTE
{
    DWMWA_NCRENDERING_ENABLED = 1,              // [get] Is non-client rendering enabled/disabled
    DWMWA_NCRENDERING_POLICY,                   // [set] DWMNCRENDERINGPOLICY - Non-client rendering policy
    DWMWA_TRANSITIONS_FORCEDISABLED,            // [set] Potentially enable/forcibly disable transitions
    DWMWA_ALLOW_NCPAINT,                        // [set] Allow contents rendered in the non-client area to be visible on the DWM-drawn frame.
    DWMWA_CAPTION_BUTTON_BOUNDS,                // [get] Bounds of the caption button area in window-relative space.
    DWMWA_NONCLIENT_RTL_LAYOUT,                 // [set] Is non-client content RTL mirrored
    DWMWA_FORCE_ICONIC_REPRESENTATION,          // [set] Force this window to display iconic thumbnails.
    DWMWA_FLIP3D_POLICY,                        // [set] Designates how Flip3D will treat the window.
    DWMWA_EXTENDED_FRAME_BOUNDS,                // [get] Gets the extended frame bounds rectangle in screen space
    DWMWA_HAS_ICONIC_BITMAP,                    // [set] Indicates an available bitmap when there is no better thumbnail representation.
    DWMWA_DISALLOW_PEEK,                        // [set] Don't invoke Peek on the window.
    DWMWA_EXCLUDED_FROM_PEEK,                   // [set] LivePreview exclusion information
    DWMWA_CLOAK,                                // [set] Cloak or uncloak the window
    DWMWA_CLOAKED,                              // [get] Gets the cloaked state of the window
    DWMWA_FREEZE_REPRESENTATION,                // [set] BOOL, Force this window to freeze the thumbnail without live update
    DWMWA_PASSIVE_UPDATE_MODE,                  // [set] BOOL, Updates the window only when desktop composition runs for other reasons
    DWMWA_USE_HOSTBACKDROPBRUSH,                // [set] BOOL, Allows the use of host backdrop brushes for the window.
    DWMWA_USE_IMMERSIVE_DARK_MODE = 20,         // [set] BOOL, Allows a window to either use the accent color, or dark, according to the user Color Mode preferences.
    DWMWA_WINDOW_CORNER_PREFERENCE = 33,        // [set] WINDOW_CORNER_PREFERENCE, Controls the policy that rounds top-level window corners
    DWMWA_BORDER_COLOR,                         // [set] COLORREF, The color of the thin border around a top-level window
    DWMWA_CAPTION_COLOR,                        // [set] COLORREF, The color of the caption
    DWMWA_TEXT_COLOR,                           // [set] COLORREF, The color of the caption text
    DWMWA_VISIBLE_FRAME_BORDER_THICKNESS,       // [get] UINT, width of the visible border around a thick frame window
    DWMWA_SYSTEMBACKDROP_TYPE,                  // [get, set] SYSTEMBACKDROP_TYPE, Controls the system-drawn backdrop material of a window, including behind the non-client area.
    DWMWA_REDIRECTIONBITMAP_ALPHA,              // [set] BOOL, GDI redirection bitmap contains premultiplied alpha
    DWMWA_BORDER_MARGINS,                       // [set] FRAME_MARGIN, Override location of window border (distance from each edge)
    DWMWA_LAST
};

typedef enum {
    /*
     * Let the system decide whether or not to round window corners
     */
    DWMWCP_DEFAULT                                 = 0,

    /*
     * Never round window corners
     */
    DWMWCP_DONOTROUND                              = 1,

    /*
     * Round the corners if appropriate
     */
    DWMWCP_ROUND                                   = 2,

    /*
     * Round the corners if appropriate, with a small radius
     */
    DWMWCP_ROUNDSMALL                              = 3

} DWM_WINDOW_CORNER_PREFERENCE;

// Use this constant to reset any window part colors to the system default behavior
#define DWMWA_COLOR_DEFAULT 0xFFFFFFFF

// Use this constant to specify that a window part should not be rendered
#define DWMWA_COLOR_NONE    0xFFFFFFFE

// Types used with DWMWA_SYSTEMBACKDROP_TYPE
enum DWM_SYSTEMBACKDROP_TYPE
{
    DWMSBT_AUTO,             // [Default] Let DWM automatically decide the system-drawn backdrop for this window.
    DWMSBT_NONE,             // Do not draw any system backdrop.
    DWMSBT_MAINWINDOW,       // Draw the backdrop material effect corresponding to a long-lived window.
    DWMSBT_TRANSIENTWINDOW,  // Draw the backdrop material effect corresponding to a transient window.
    DWMSBT_TABBEDWINDOW,     // Draw the backdrop material effect corresponding to a window with a tabbed title bar.
};


// Non-client rendering policy attribute values
enum DWMNCRENDERINGPOLICY
{
    DWMNCRP_USEWINDOWSTYLE, // Enable/disable non-client rendering based on window style
    DWMNCRP_DISABLED,       // Disabled non-client rendering; window style is ignored
    DWMNCRP_ENABLED,        // Enabled non-client rendering; window style is ignored
    DWMNCRP_LAST
};

// Values designating how Flip3D treats a given window.
enum DWMFLIP3DWINDOWPOLICY
{
    DWMFLIP3D_DEFAULT,      // Hide or include the window in Flip3D based on window style and visibility.
    DWMFLIP3D_EXCLUDEBELOW, // Display the window under Flip3D and disabled.
    DWMFLIP3D_EXCLUDEABOVE, // Display the window above Flip3D and enabled.
    DWMFLIP3D_LAST
};

// Cloaked flags describing why a window is cloaked.
#define DWM_CLOAKED_APP         0x00000001
#define DWM_CLOAKED_SHELL       0x00000002
#define DWM_CLOAKED_INHERITED   0x00000004

#pragma region Thumbnails
typedef HANDLE      HTHUMBNAIL;
typedef HTHUMBNAIL* PHTHUMBNAIL;

#pragma region Flags for DWM_THUMBNAIL_PROPERTIES
#define DWM_TNP_RECTDESTINATION                  0x00000001 // A value for the "rcDestination" member has been specified.
#define DWM_TNP_RECTSOURCE                       0x00000002 // A value for the "rcSource" member has been specified.
#define DWM_TNP_OPACITY                          0x00000004 // A value for the "opacity" member has been specified.
#define DWM_TNP_VISIBLE                          0x00000008 // A value for the "fVisible" member has been specified.
#define DWM_TNP_SOURCECLIENTAREAONLY             0x00000010 // A value for the "fSourceClientAreaOnly" member has been specified.
#pragma endregion

typedef struct _DWM_THUMBNAIL_PROPERTIES
{
    DWORD dwFlags;              // Specifies which members of this struct have been specified
    RECT rcDestination;         // The area in the destination window where the thumbnail will be rendered
    RECT rcSource;              // The region of the source window to use as the thumbnail.  By default, the entire window is used as the thumbnail
    BYTE opacity;               // The opacity with which to render the thumbnail.  0 is fully transparent, while 255 is fully opaque.  The default value is 255
    BOOL fVisible;              // Whether the thumbnail should be visible.  The default is FALSE
    BOOL fSourceClientAreaOnly; // Whether only the client area of the source window should be included in the thumbnail.  The default is FALSE
} DWM_THUMBNAIL_PROPERTIES, *PDWM_THUMBNAIL_PROPERTIES;
#pragma endregion

// Video enabling apis

typedef ULONGLONG DWM_FRAME_COUNT;
typedef ULONGLONG QPC_TIME;

typedef  struct _UNSIGNED_RATIO
{
    UINT32 uiNumerator;
    UINT32 uiDenominator;
} UNSIGNED_RATIO;

typedef  struct _DWM_TIMING_INFO
{
    UINT32          cbSize;

    // Data on DWM composition overall

    // Monitor refresh rate
    UNSIGNED_RATIO  rateRefresh;

    // Actual period
    QPC_TIME        qpcRefreshPeriod;

    // composition rate
    UNSIGNED_RATIO  rateCompose;

    // QPC time at a VSync interupt
    QPC_TIME        qpcVBlank;

    // DWM refresh count of the last vsync
    // DWM refresh count is a 64bit number where zero is
    // the first refresh the DWM woke up to process
    DWM_FRAME_COUNT cRefresh;

    // DX refresh count at the last Vsync Interupt
    // DX refresh count is a 32bit number with zero
    // being the first refresh after the card was initialized
    // DX increments a counter when ever a VSync ISR is processed
    // It is possible for DX to miss VSyncs
    //
    // There is not a fixed mapping between DX and DWM refresh counts
    // because the DX will rollover and may miss VSync interupts
    UINT cDXRefresh;

    // QPC time at a compose time.
    QPC_TIME        qpcCompose;

    // Frame number that was composed at qpcCompose
    DWM_FRAME_COUNT cFrame;

    // The present number DX uses to identify renderer frames
    UINT            cDXPresent;

    // Refresh count of the frame that was composed at qpcCompose
    DWM_FRAME_COUNT cRefreshFrame;


    // DWM frame number that was last submitted
    DWM_FRAME_COUNT cFrameSubmitted;

    // DX Present number that was last submitted
    UINT cDXPresentSubmitted;

    // DWM frame number that was last confirmed presented
    DWM_FRAME_COUNT cFrameConfirmed;

    // DX Present number that was last confirmed presented
    UINT cDXPresentConfirmed;

    // The target refresh count of the last
    // frame confirmed completed by the GPU
    DWM_FRAME_COUNT cRefreshConfirmed;

    // DX refresh count when the frame was confirmed presented
    UINT cDXRefreshConfirmed;

    // Number of frames the DWM presented late
    // AKA Glitches
    DWM_FRAME_COUNT          cFramesLate;

    // the number of composition frames that
    // have been issued but not confirmed completed
    UINT          cFramesOutstanding;


    // Following fields are only relavent when an HWND is specified
    // Display frame


    // Last frame displayed
    DWM_FRAME_COUNT cFrameDisplayed;

    // QPC time of the composition pass when the frame was displayed
    QPC_TIME        qpcFrameDisplayed;

    // Count of the VSync when the frame should have become visible
    DWM_FRAME_COUNT cRefreshFrameDisplayed;

    // Complete frames: DX has notified the DWM that the frame is done rendering

    // ID of the the last frame marked complete (starts at 0)
    DWM_FRAME_COUNT cFrameComplete;

    // QPC time when the last frame was marked complete
    QPC_TIME        qpcFrameComplete;

    // Pending frames:
    // The application has been submitted to DX but not completed by the GPU

    // ID of the the last frame marked pending (starts at 0)
    DWM_FRAME_COUNT cFramePending;

    // QPC time when the last frame was marked pending
    QPC_TIME        qpcFramePending;

    // number of unique frames displayed
    DWM_FRAME_COUNT cFramesDisplayed;

    // number of new completed frames that have been received
    DWM_FRAME_COUNT cFramesComplete;

     // number of new frames submitted to DX but not yet complete
    DWM_FRAME_COUNT cFramesPending;

    // number of frames available but not displayed, used or dropped
    DWM_FRAME_COUNT cFramesAvailable;

    // number of rendered frames that were never
    // displayed because composition occured too late
    DWM_FRAME_COUNT cFramesDropped;

    // number of times an old frame was composed
    // when a new frame should have been used
    // but was not available
    DWM_FRAME_COUNT cFramesMissed;

    // the refresh at which the next frame is
    // scheduled to be displayed
    DWM_FRAME_COUNT cRefreshNextDisplayed;

    // the refresh at which the next DX present is
    // scheduled to be displayed
    DWM_FRAME_COUNT cRefreshNextPresented;

    // The total number of refreshes worth of content
    // for this HWND that have been displayed by the DWM
    // since DwmSetPresentParameters was called
    DWM_FRAME_COUNT cRefreshesDisplayed;

    // The total number of refreshes worth of content
    // that have been presented by the application
    // since DwmSetPresentParameters was called
    DWM_FRAME_COUNT cRefreshesPresented;


    // The actual refresh # when content for this
    // window started to be displayed
    // it may be different than that requested
    // DwmSetPresentParameters
    DWM_FRAME_COUNT cRefreshStarted;

    // Total number of pixels DX redirected
    // to the DWM.
    // If Queueing is used the full buffer
    // is transfered on each present.
    // If not queuing it is possible only
    // a dirty region is updated
    ULONGLONG  cPixelsReceived;

    // Total number of pixels drawn.
    // Does not take into account if
    // if the window is only partial drawn
    // do to clipping or dirty rect management
    ULONGLONG  cPixelsDrawn;

    // The number of buffers in the flipchain
    // that are empty.   An application can
    // present that number of times and guarantee
    // it won't be blocked waiting for a buffer to
    // become empty to present to
    DWM_FRAME_COUNT      cBuffersEmpty;

} DWM_TIMING_INFO;

typedef enum
{
    // Use the first source frame that
    // includes the first refresh of the output frame
    DWM_SOURCE_FRAME_SAMPLING_POINT,

    // use the source frame that includes the most
    // refreshes of out the output frame
    // in case of multiple source frames with the
    // same coverage the last will be used
    DWM_SOURCE_FRAME_SAMPLING_COVERAGE,

       // Sentinel value
    DWM_SOURCE_FRAME_SAMPLING_LAST
} DWM_SOURCE_FRAME_SAMPLING;

EXTERN_C __declspec(selectany) const UINT c_DwmMaxQueuedBuffers = 8;
EXTERN_C __declspec(selectany) const UINT c_DwmMaxMonitors = 16;
EXTERN_C __declspec(selectany) const UINT c_DwmMaxAdapters = 16;

#pragma warning(push)
#pragma warning(disable:4201)
typedef struct _DWM_PRESENT_PARAMETERS
{
    UINT32          cbSize;
    BOOL            fQueue;
    DWM_FRAME_COUNT cRefreshStart;
    UINT            cBuffer;
    BOOL            fUseSourceRate;
    UNSIGNED_RATIO  rateSource;
    UINT            cRefreshesPerFrame;
    DWM_SOURCE_FRAME_SAMPLING  eSampling;
} DWM_PRESENT_PARAMETERS;
#pragma warning(pop)



#define DWM_FRAME_DURATION_DEFAULT -1

DWMAPI_(BOOL)
DwmDefWindowProc(
    _In_ HWND hWnd,
    UINT msg,
    WPARAM wParam,
    LPARAM lParam,
    _Out_ LRESULT *plResult
    );

DWMAPI
DwmEnableBlurBehindWindow(
    HWND hWnd,
    _In_ const DWM_BLURBEHIND* pBlurBehind
    );

#define DWM_EC_DISABLECOMPOSITION         0
#define DWM_EC_ENABLECOMPOSITION          1


DWMAPI
DwmEnableComposition(
    UINT uCompositionAction
    );

#if NTDDI_VERSION >= NTDDI_WIN8
#pragma deprecated (DwmEnableComposition)
#endif

DWMAPI
DwmEnableMMCSS(
    BOOL fEnableMMCSS
    );

DWMAPI
DwmExtendFrameIntoClientArea(
    HWND hWnd,
    _In_ const MARGINS* pMarInset
    );

DWMAPI
DwmGetColorizationColor(
    _Out_ DWORD* pcrColorization,
    _Out_ BOOL* pfOpaqueBlend
    );

DWMAPI
DwmGetCompositionTimingInfo(
    HWND hwnd,
    _Out_ DWM_TIMING_INFO* pTimingInfo
    );


DWMAPI
DwmGetWindowAttribute(
    HWND hwnd,
    DWORD dwAttribute,
    _Out_writes_bytes_(cbAttribute) PVOID pvAttribute,
    DWORD cbAttribute
    );

DWMAPI
DwmIsCompositionEnabled(
    _Out_ BOOL* pfEnabled
    );

DWMAPI
DwmModifyPreviousDxFrameDuration(
    HWND hwnd,
    INT cRefreshes,
    BOOL fRelative
    );

DWMAPI
DwmQueryThumbnailSourceSize(
    HTHUMBNAIL hThumbnail,
    _Out_ PSIZE pSize
    );

DWMAPI
DwmRegisterThumbnail(
    HWND hwndDestination,
    HWND hwndSource,
    _Out_ PHTHUMBNAIL phThumbnailId
    );

DWMAPI
DwmSetDxFrameDuration(
    HWND hwnd,
    INT cRefreshes
    );

DWMAPI
DwmSetPresentParameters(
    HWND hwnd,
   _Inout_ DWM_PRESENT_PARAMETERS* pPresentParams
    );

DWMAPI
DwmSetWindowAttribute(
    HWND hwnd,
    DWORD dwAttribute,
    _In_reads_bytes_(cbAttribute) LPCVOID pvAttribute,
    DWORD cbAttribute
    );

DWMAPI
DwmUnregisterThumbnail(
    HTHUMBNAIL hThumbnailId
    );

DWMAPI
DwmUpdateThumbnailProperties(
    HTHUMBNAIL hThumbnailId,
    _In_ const DWM_THUMBNAIL_PROPERTIES* ptnProperties
    );

#if(_WIN32_WINNT >= 0x0601)
#define DWM_SIT_DISPLAYFRAME    0x00000001  // Display a window frame around the provided bitmap

DWMAPI DwmSetIconicThumbnail(
    HWND hwnd,
    HBITMAP hbmp,
    DWORD dwSITFlags
    );

DWMAPI DwmSetIconicLivePreviewBitmap(
    HWND hwnd,
    HBITMAP hbmp,
    _In_opt_ POINT *pptClient,
    DWORD dwSITFlags
    );

DWMAPI DwmInvalidateIconicBitmaps(
    HWND hwnd
    );

#endif /* _WIN32_WINNT >= 0x0601 */

DWMAPI
DwmAttachMilContent(
    _In_ HWND hwnd
    );

DWMAPI
DwmDetachMilContent(
    _In_ HWND hwnd
    );

DWMAPI
DwmFlush();

#ifndef MILCORE_KERNEL_COMPONENT
#ifndef _MIL_MATRIX3X2D_DEFINED

typedef struct _MilMatrix3x2D
{
    DOUBLE S_11;
    DOUBLE S_12;
    DOUBLE S_21;
    DOUBLE S_22;
    DOUBLE DX;
    DOUBLE DY;
} MilMatrix3x2D;

#define _MIL_MATRIX3X2D_DEFINED

#endif // _MIL_MATRIX3X2D_DEFINED

#ifndef MILCORE_MIL_MATRIX3X2D_COMPAT_TYPEDEF
// Compatibility for Vista dwm api.
typedef MilMatrix3x2D MIL_MATRIX3X2D;
#define MILCORE_MIL_MATRIX3X2D_COMPAT_TYPEDEF
#endif // MILCORE_MIL_MATRIX3X2D_COMPAT_TYPEDEF

DWMAPI
DwmGetGraphicsStreamTransformHint(
    UINT uIndex,
    _Out_ MilMatrix3x2D *pTransform
    );

DWMAPI
DwmGetGraphicsStreamClient(
    UINT uIndex,
    _Out_ UUID *pClientUuid
    );
#endif // MILCORE_KERNEL_COMPONENT

DWMAPI
DwmGetTransportAttributes(
    _Out_writes_(1) BOOL *pfIsRemoting,
    _Out_writes_(1) BOOL *pfIsConnected,
    _Out_writes_(1) DWORD *pDwGeneration
    );


enum DWMTRANSITION_OWNEDWINDOW_TARGET
{
    DWMTRANSITION_OWNEDWINDOW_NULL          = -1,
    DWMTRANSITION_OWNEDWINDOW_REPOSITION    = 0,
};

DWMAPI
DwmTransitionOwnedWindow(
    HWND hwnd,
    enum DWMTRANSITION_OWNEDWINDOW_TARGET target
    );


#if (NTDDI_VERSION >= NTDDI_WIN8)

enum GESTURE_TYPE
{
    GT_PEN_TAP                  = 0,
    GT_PEN_DOUBLETAP            = 1,
    GT_PEN_RIGHTTAP             = 2,
    GT_PEN_PRESSANDHOLD         = 3,
    GT_PEN_PRESSANDHOLDABORT    = 4,
    GT_TOUCH_TAP                = 5,
    GT_TOUCH_DOUBLETAP          = 6,
    GT_TOUCH_RIGHTTAP           = 7,
    GT_TOUCH_PRESSANDHOLD       = 8,
    GT_TOUCH_PRESSANDHOLDABORT  = 9,
    GT_TOUCH_PRESSANDTAP        = 10,
};

DWMAPI
DwmRenderGesture(
    _In_ enum GESTURE_TYPE gt,
    _In_ UINT cContacts,
    _In_reads_(cContacts) const DWORD *pdwPointerID,
    _In_reads_(cContacts) const POINT *pPoints
    );

DWMAPI
DwmTetherContact(
    DWORD dwPointerID,
    BOOL fEnable,
    POINT ptTether
    );

enum DWM_SHOWCONTACT
{
    DWMSC_DOWN      = 0x00000001,
    DWMSC_UP        = 0x00000002,
    DWMSC_DRAG      = 0x00000004,
    DWMSC_HOLD      = 0x00000008,
    DWMSC_PENBARREL = 0x00000010,

    DWMSC_NONE      = 0x00000000,
    DWMSC_ALL       = 0xFFFFFFFF
};
DEFINE_ENUM_FLAG_OPERATORS(DWM_SHOWCONTACT);

DWMAPI
DwmShowContact(
    DWORD dwPointerID,
    enum DWM_SHOWCONTACT eShowContact
    );
#endif // NTDDI_WIN8


#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

enum DWM_TAB_WINDOW_REQUIREMENTS
{
    // This result means the window meets all requirements requested.
    DWMTWR_NONE                     = 0x0000,

    // In some configurations, admin/user setting or mode of the system means that windows won't be tabbed
    // This requirement says that the system/mode must implement tabbing and if it does not
    // nothing can be done to change this.
    DWMTWR_IMPLEMENTED_BY_SYSTEM    = 0x0001,

    // The window has an owner or parent so is ineligible for tabbing.
    DWMTWR_WINDOW_RELATIONSHIP      = 0x0002,

    // The window has styles that make it ineligible for tabbing.
    // To be eligible windows must:
    // Have the WS_OVERLAPPEDWINDOW (WS_CAPTION, WS_THICKFRAME, etc.) styles set.
    // Not have WS_POPUP, WS_CHILD or WS_DLGFRAME set.
    // Not have WS_EX_TOPMOST or WS_EX_TOOLWINDOW set.
    DWMTWR_WINDOW_STYLES            = 0x0004,

    // The window has a region (set using SetWindowRgn) making it ineligible.
    DWMTWR_WINDOW_REGION            = 0x0008,

    // The window is ineligible due to its Dwm configuration.
    // It must not extended its client area into the title bar using DwmExtendFrameIntoClientArea
    // It must not have DWMWA_NCRENDERING_POLICY set to DWMNCRP_ENABLED
    DWMTWR_WINDOW_DWM_ATTRIBUTES    = 0x0010,

    // The window is ineligible due to it's margins, most likely due to custom handling in WM_NCCALCSIZE.
    // The window must use the default window margins for the non-client area.
    DWMTWR_WINDOW_MARGINS           = 0x0020,

    // The window has been explicitly opted out by setting DWMWA_TABBING_ENABLED to FALSE.
    DWMTWR_TABBING_ENABLED          = 0x0040,

    // The user has configured this application to not participate in tabbing.
    DWMTWR_USER_POLICY              = 0x0080,

    // The group policy has configured this application to not participate in tabbing.
    DWMTWR_GROUP_POLICY             = 0x0100,

    // This is set if app compat has blocked tabs for this window. Can be overridden per window by setting
    // DWMWA_TABBING_ENABLED to TRUE. That does not override any other tabbing requirements.
    DWMTWR_APP_COMPAT               = 0x0200
};
DEFINE_ENUM_FLAG_OPERATORS(DWM_TAB_WINDOW_REQUIREMENTS);

// Checks the requirements needed to get tabs in the application title bar.
DWMAPI DwmGetUnmetTabRequirements(_In_opt_ HWND appWindow, _Out_ enum DWM_TAB_WINDOW_REQUIREMENTS* value);

#endif // NTDDI_WIN10_RS4


#include <poppack.h>

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#endif // _DWMAPI_H_
