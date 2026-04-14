/**************************************************************************
*
* Copyright (c) 2000-2003 Microsoft Corporation
*
* Module Name:
*
*   Gdiplus initialization
*
* Abstract:
*
*   GDI+ Startup and Shutdown APIs
*
**************************************************************************/

#ifndef _GDIPLUSINIT_H
#define _GDIPLUSINIT_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

enum DebugEventLevel
{
    DebugEventLevelFatal,
    DebugEventLevelWarning
};

// Callback function that GDI+ can call, on debug builds, for assertions
// and warnings.

typedef VOID (WINAPI *DebugEventProc)(DebugEventLevel level, CHAR *message);

// Notification functions which the user must call appropriately if
// "SuppressBackgroundThread" (below) is set.

typedef Status (WINAPI *NotificationHookProc)(OUT ULONG_PTR *token);
typedef VOID (WINAPI *NotificationUnhookProc)(ULONG_PTR token);

// Input structure for GdiplusStartup()

struct GdiplusStartupInput
{
    UINT32 GdiplusVersion;             // Must be 1  (or 2 for the Ex version)
    DebugEventProc DebugEventCallback; // Ignored on free builds
    BOOL SuppressBackgroundThread;     // FALSE unless you're prepared to call 
                                       // the hook/unhook functions properly
    BOOL SuppressExternalCodecs;       // FALSE unless you want GDI+ only to use
                                       // its internal image codecs.
    
    GdiplusStartupInput(
        DebugEventProc debugEventCallback = NULL,
        BOOL suppressBackgroundThread = FALSE,
        BOOL suppressExternalCodecs = FALSE)
    {
        GdiplusVersion = 1;
        DebugEventCallback = debugEventCallback;
        SuppressBackgroundThread = suppressBackgroundThread;
        SuppressExternalCodecs = suppressExternalCodecs;
    }
};

#if (GDIPVER >= 0x0110)
struct GdiplusStartupInputEx : GdiplusStartupInput
{
    enum class Version : UINT32
    {
        V2 = 2,
        V3 = 3 // Enables Heif and Avif image codecs.  Unlike other functionalities in Gdiplus,
               // these two codecs require COM to be initiailized.
    };

    INT StartupParameters;  // Do we not set the FPU rounding mode

    GdiplusStartupInputEx(
        INT startupParameters = 0,
        DebugEventProc debugEventCallback = NULL,
        BOOL suppressBackgroundThread = FALSE,
        BOOL suppressExternalCodecs = FALSE)
    {
        GdiplusVersion = 2;
        DebugEventCallback = debugEventCallback;
        SuppressBackgroundThread = suppressBackgroundThread;
        SuppressExternalCodecs = suppressExternalCodecs;
        StartupParameters = startupParameters;
    }

    GdiplusStartupInputEx(
        Version gdiplusVersion,
        INT startupParameters = 0,
        DebugEventProc debugEventCallback = NULL,
        BOOL suppressBackgroundThread = FALSE,
        BOOL suppressExternalCodecs = FALSE)
    {
        GdiplusVersion = static_cast<UINT32>(gdiplusVersion);
        DebugEventCallback = debugEventCallback;
        SuppressBackgroundThread = suppressBackgroundThread;
        SuppressExternalCodecs = suppressExternalCodecs;
        StartupParameters = startupParameters;
    }
};

enum GdiplusStartupParams
{
    GdiplusStartupDefault    = 0x00000000,
    GdiplusStartupNoSetRound = 0x00000001,
    GdiplusStartupSetPSValue = 0x00000002,
    GdiplusStartupReserved0  = 0x00000004,
    GdiplusStartupReserved1  = 0x00000008,
    GdiplusStartupReserved2  = 0x00000010,
    GdiplusStartupTransparencyMask = 0xFF000000
};

#endif


// Output structure for GdiplusStartup()

struct GdiplusStartupOutput
{
    // The following 2 fields are NULL if SuppressBackgroundThread is FALSE.
    // Otherwise, they are functions which must be called appropriately to
    // replace the background thread.
    //
    // These should be called on the application's main message loop - i.e.
    // a message loop which is active for the lifetime of GDI+.
    // "NotificationHook" should be called before starting the loop,
    // and "NotificationUnhook" should be called after the loop ends.
    
    NotificationHookProc NotificationHook;
    NotificationUnhookProc NotificationUnhook;
};

// GDI+ initialization. Must not be called from DllMain - can cause deadlock.
//
// Must be called before GDI+ API's or constructors are used.
//
// token  - may not be NULL - accepts a token to be passed in the corresponding
//          GdiplusShutdown call.
// input  - may not be NULL
// output - may be NULL only if input->SuppressBackgroundThread is FALSE.

extern "C" Status WINAPI GdiplusStartup(
    OUT ULONG_PTR *token,
    const GdiplusStartupInput *input,
    OUT GdiplusStartupOutput *output);

// GDI+ termination. Must be called before GDI+ is unloaded. 
// Must not be called from DllMain - can cause deadlock.
//
// GDI+ API's may not be called after GdiplusShutdown. Pay careful attention
// to GDI+ object destructors.

extern "C" VOID WINAPI GdiplusShutdown(ULONG_PTR token);

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
