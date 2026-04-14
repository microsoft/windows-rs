/*****************************************************************************\
*                                                                             *
* custcntl.h -  Custom Control Library header file                            *
*                                                                             *
*               Copyright (c) 1992-1999, Microsoft Corp.  All rights reserved *
*                                                                             *
\*****************************************************************************/

#ifndef _INC_CUSTCNTL
#define _INC_CUSTCNTL

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {            /* Assume C declarations for C++ */
#endif  /* __cplusplus */

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
 * General size defines.
 */
#define CCHCCCLASS          32          // Max chars in a class name.
#define CCHCCDESC           32          // Max chars in a control description.
#define CCHCCTEXT           256         // Max chars in a text field.


/*
 * CCSTYLE - Custom Control Style structure.  This structure is passed
 * tp the Custom Control Style function when the user wants to edit the
 * styles of the custom control.
 */
typedef struct tagCCSTYLEA {
    DWORD   flStyle;                    // Style of the control.
    DWORD   flExtStyle;                 // Extended style of the control.
    CHAR    szText[CCHCCTEXT];          // Text of the control.
    LANGID  lgid;                       // Language Id of the control's dialog.
    WORD    wReserved1;                 // Reserved value.  Do not change.
} CCSTYLEA, *LPCCSTYLEA;

typedef struct tagCCSTYLEW {
    DWORD   flStyle;                    // Style of the control.
    DWORD   flExtStyle;                 // Extended style of the control.
    WCHAR   szText[CCHCCTEXT];          // Text of the control.
    LANGID  lgid;                       // Language Id of the control's dialog.
    WORD    wReserved1;                 // Reserved value.  Do not change.
} CCSTYLEW, *LPCCSTYLEW;

#ifdef UNICODE
#define CCSTYLE     CCSTYLEW
#define LPCCSTYLE   LPCCSTYLEW
#else
#define CCSTYLE     CCSTYLEA
#define LPCCSTYLE   LPCCSTYLEA
#endif // UNICODE


/*
 * The Style function prototype.  This will be called when the user
 * wants to edit the styles of a custom control.  It should display a
 * dialog to edit the styles, update the styles in the pccs structure,
 * then return TRUE for success.  If an error occurs or the user
 * cancels the dialog, FALSE should be returned.
 */
typedef BOOL (CALLBACK* LPFNCCSTYLEA)(HWND hwndParent,  LPCCSTYLEA pccs);
typedef BOOL (CALLBACK* LPFNCCSTYLEW)(HWND hwndParent,  LPCCSTYLEW pccs);

#ifdef UNICODE
#define LPFNCCSTYLE LPFNCCSTYLEW
#else
#define LPFNCCSTYLE LPFNCCSTYLEA
#endif  // UNICODE


/*
 * The SizeToText function prototype.  This will be called if the user
 * requests that the custom control be sized to fit it's text.  It
 * should use the specified styles, text and font to determine how
 * large the control must be to accommodate the text, then return this
 * value in pixels.  The value of -1 should be returned if an error
 * occurs.
 */
typedef INT (CALLBACK* LPFNCCSIZETOTEXTA)(DWORD flStyle, DWORD flExtStyle,
    HFONT hfont, LPSTR pszText);
typedef INT (CALLBACK* LPFNCCSIZETOTEXTW)(DWORD flStyle, DWORD flExtStyle,
    HFONT hfont, LPWSTR pszText);

#ifdef UNICODE
#define LPFNCCSIZETOTEXT    LPFNCCSIZETOTEXTW
#else
#define LPFNCCSIZETOTEXT    LPFNCCSIZETOTEXTA
#endif  // UNICODE


/*
 * CCSTYLEFLAG - Custom Control Style Flag structure.  A table of these
 * structures is used to specify the define strings that match the
 * different styles for a custom control.
 */
typedef struct tagCCSTYLEFLAGA {
    DWORD flStyle;                      // Style bits for this style.
    DWORD flStyleMask;                  // Mask for the style.  Can be zero.
    LPSTR pszStyle;                     // Points to the style define string.
} CCSTYLEFLAGA, *LPCCSTYLEFLAGA;

typedef struct tagCCSTYLEFLAGW {
    DWORD flStyle;                      // Style bits for this style.
    DWORD flStyleMask;                  // Mask for the style.  Can be zero.
    LPWSTR pszStyle;                    // Points to the style define string.
} CCSTYLEFLAGW, *LPCCSTYLEFLAGW;

#ifdef UNICODE
#define CCSTYLEFLAG     CCSTYLEFLAGW
#define LPCCSTYLEFLAG   LPCCSTYLEFLAGW
#else
#define CCSTYLEFLAG     CCSTYLEFLAGA
#define LPCCSTYLEFLAG   LPCCSTYLEFLAGA
#endif // UNICODE


/*
 * CCF_* defines.  These flags are used for the flOptions field of the
 * CCINFO structure, and describe some basic characteristics of the
 * custom control.
 */
#define CCF_NOTEXT          0x00000001  // Control cannot have text.


/*
 * CCINFO - Custom Control Info structure.  This structure provides
 * the dialog editor with information about the control types that the
 * DLL supports.
 */
typedef struct tagCCINFOA {
    CHAR    szClass[CCHCCCLASS];        // Class name for the control.
    DWORD   flOptions;                  // Option flags (CCF_* defines).
    CHAR    szDesc[CCHCCDESC];          // Short, descriptive text for the ctrl.
    UINT    cxDefault;                  // Default width (in dialog units).
    UINT    cyDefault;                  // Default height (in dialog units).
    DWORD   flStyleDefault;             // Default style (WS_CHILD | WS_VISIBLE).
    DWORD   flExtStyleDefault;          // Default extended style.
    DWORD   flCtrlTypeMask;             // Mask for control type styles.
    CHAR    szTextDefault[CCHCCTEXT];   // Default text.
    INT     cStyleFlags;                // Entries in the following style table.
    LPCCSTYLEFLAGA aStyleFlags;         // Points to style flag table.
    LPFNCCSTYLEA lpfnStyle;             // Pointer to the Styles function.
    LPFNCCSIZETOTEXTA lpfnSizeToText;   // Pointer to the SizeToText function.
    DWORD   dwReserved1;                // Reserved.  Must be zero.
    DWORD   dwReserved2;                // Reserved.  Must be zero.
} CCINFOA, *LPCCINFOA;

typedef struct tagCCINFOW {
    WCHAR   szClass[CCHCCCLASS];        // Class name for the control.
    DWORD   flOptions;                  // Option flags (CCF_* defines).
    WCHAR   szDesc[CCHCCDESC];          // Short, descriptive text for the ctrl.
    UINT    cxDefault;                  // Default width (in dialog units).
    UINT    cyDefault;                  // Default height (in dialog units).
    DWORD   flStyleDefault;             // Default style (WS_CHILD | WS_VISIBLE).
    DWORD   flExtStyleDefault;          // Default extended style.
    DWORD   flCtrlTypeMask;             // Mask for control type styles.
    INT     cStyleFlags;                // Entries in the following style table.
    LPCCSTYLEFLAGW aStyleFlags;         // Points to style flag table.
    WCHAR   szTextDefault[CCHCCTEXT];   // Default text.
    LPFNCCSTYLEW lpfnStyle;             // Pointer to the Styles function.
    LPFNCCSIZETOTEXTW lpfnSizeToText;   // Pointer to the SizeToText function.
    DWORD   dwReserved1;                // Reserved.  Must be zero.
    DWORD   dwReserved2;                // Reserved.  Must be zero.
} CCINFOW, *LPCCINFOW;

#ifdef UNICODE
#define CCINFO      CCINFOW
#define LPCCINFO    LPCCINFOW
#else
#define CCINFO      CCINFOA
#define LPCCINFO    LPCCINFOA
#endif // UNICODE


/*
 * The Info function prototype.  This function is the first one
 * called by the dialog editor.  Custom control DLL's must export
 * one or both of the following functions by name (the ordinal
 * used for the export does not matter):
 *
 *  UINT CALLBACK CustomControlInfoA(LPCCINFOA acci)
 *  UINT CALLBACK CustomControlInfoW(LPCCINFOW acci)
 *
 * This function must return the number of controls that the DLL
 * supports, or NULL if an error occurs.  If the acci parameter is
 * not NULL, it will be pointing to an array of CCINFOA or CCINFOW
 * structures that should be filled in with the information about
 * the different control types supported by the DLL.
 *
 * If both functions are present, the CustomControlInfoW function
 * will be used by the dialog editor.
 */
typedef UINT (CALLBACK* LPFNCCINFOA)(LPCCINFOA acci);
typedef UINT (CALLBACK* LPFNCCINFOW)(LPCCINFOW acci);

#ifdef UNICODE
#define LPFNCCINFO  LPFNCCINFOW
#else
#define LPFNCCINFO  LPFNCCINFOA
#endif  // UNICODE


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif  /* __cplusplus */

#endif  /* _INC_CUSTCNTL */
