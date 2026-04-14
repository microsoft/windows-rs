/***
*rtcapi.h - declarations and definitions for RTC use
*
*       Copyright (c) Microsoft Corporation. All rights reserved.
*
*Purpose:
*       Contains the declarations and definitions for all RunTime Check
*       support.
*
****/

#ifndef _INC_RTCAPI
#define _INC_RTCAPI

#if defined _RTC || !defined _VCRT_BUILD

#include <vcruntime.h>

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)

#pragma pack(push,_CRT_PACKING)

/*
Previous versions of this header included declarations of functions used by RTC but
not intended for use by end-users. These functions are now declared deprecated by default
and may be removed in a future version.
*/
#ifndef _CRT_ENABLE_RTC_INTERNALS
#define _RTCINTERNAL_DEPRECATED _CRT_DEPRECATE_TEXT("These internal RTC functions are obsolete and should not be used")
#else  /* _CRT_ENABLE_RTC_INTERNALS */
#define _RTCINTERNAL_DEPRECATED
#endif  /* _CRT_ENABLE_RTC_INTERNALS */



#ifdef __cplusplus

extern "C" {

#endif  /* __cplusplus */

    /* General User API */

typedef enum _RTC_ErrorNumber {
    _RTC_CHKSTK = 0,
    _RTC_CVRT_LOSS_INFO,
    _RTC_CORRUPT_STACK,
    _RTC_UNINIT_LOCAL_USE,
    _RTC_CORRUPTED_ALLOCA,
    _RTC_ILLEGAL
} _RTC_ErrorNumber;

#   define _RTC_ERRTYPE_IGNORE -1
#   define _RTC_ERRTYPE_ASK    -2

    typedef int (__cdecl *_RTC_error_fn)(int, const char *, int, const char *, const char *, ...);
    typedef int (__cdecl *_RTC_error_fnW)(int, const wchar_t *, int, const wchar_t *, const wchar_t *, ...);

    /* User API */
    int           __cdecl _RTC_NumErrors(void);
    const char *  __cdecl _RTC_GetErrDesc(_RTC_ErrorNumber  _Errnum);
    int           __cdecl _RTC_SetErrorType(_RTC_ErrorNumber  _Errnum, int _ErrType);
    _RTC_error_fn __cdecl _RTC_SetErrorFunc(_RTC_error_fn);
    _RTC_error_fnW __cdecl _RTC_SetErrorFuncW(_RTC_error_fnW);

    /* Power User/library API */


    /* Init functions */

    /* These functions all call _CRT_RTC_INIT */
    void __cdecl _RTC_Initialize(void);
    void __cdecl _RTC_Terminate(void);

    /*
     * If you're not using the CRT, you have to implement _CRT_RTC_INIT
     * Just return either null, or your error reporting function
     * *** Don't mess with res0/res1/res2/res3/res4 - YOU'VE BEEN WARNED! ***
     */
    _RTC_error_fn __cdecl _CRT_RTC_INIT(void *_Res0, void **_Res1, int _Res2, int _Res3, int _Res4);
    _RTC_error_fnW __cdecl _CRT_RTC_INITW(void *_Res0, void **_Res1, int _Res2, int _Res3, int _Res4);

    /* Compiler generated calls (unlikely to be used, even by power users) */
    /* Types */
    typedef struct _RTC_vardesc {
        int addr;
        int size;
        char *name;
    } _RTC_vardesc;

    typedef struct _RTC_framedesc {
        int varCount;
        _RTC_vardesc *variables;
    } _RTC_framedesc;

    /* NOTE:
        Changing this structure requires a matching compiler backend
        update, because the offsets are hardcoded inside there.
    */
#pragma pack(push, 1)
    /*  Structure padded under 32-bit x86, to get consistent
        execution between 32/64 targets.
    */
    typedef struct _RTC_ALLOCA_NODE {
        __int32 guard1;
        struct _RTC_ALLOCA_NODE *next;
#if defined (_M_IX86) || defined (_M_ARM)
        __int32 dummypad;
#endif  /* defined (_M_IX86) || defined (_M_ARM) */
        size_t allocaSize;
#if defined (_M_IX86) || defined (_M_ARM)
        __int32 dummypad2;
#endif  /* defined (_M_IX86) || defined (_M_ARM) */
        __int32 guard2[3];
    } _RTC_ALLOCA_NODE;
#pragma pack(pop)

#if !defined (_M_CEE) && !defined (_M_CEE_PURE)
    /* These unsupported functions are deprecated in native mode and not supported at all in /clr mode */

    /* Shortening convert checks - name indicates src bytes to target bytes */
    /* Signedness is NOT checked */
    _RTCINTERNAL_DEPRECATED char   __fastcall _RTC_Check_2_to_1(short _Src);
    _RTCINTERNAL_DEPRECATED char   __fastcall _RTC_Check_4_to_1(int _Src);
    _RTCINTERNAL_DEPRECATED char   __fastcall _RTC_Check_8_to_1(__int64 _Src);
    _RTCINTERNAL_DEPRECATED short  __fastcall _RTC_Check_4_to_2(int _Src);
    _RTCINTERNAL_DEPRECATED short  __fastcall _RTC_Check_8_to_2(__int64 _Src);
    _RTCINTERNAL_DEPRECATED int    __fastcall _RTC_Check_8_to_4(__int64 _Src);
#endif  /* !defined (_M_CEE) && !defined (_M_CEE_PURE) */


    /* Stack Checking Calls */
#if defined (_M_IX86)
    void   __cdecl     _RTC_CheckEsp(void);
#endif  /* defined (_M_IX86) */

#if !defined (_M_CEE) && !defined (_M_CEE_PURE)
    /* These unsupported functions are deprecated in native mode and not supported at all in /clr mode */

    _RTCINTERNAL_DEPRECATED  void   __fastcall _RTC_CheckStackVars(void *_Esp, _RTC_framedesc *_Fd);
    _RTCINTERNAL_DEPRECATED  void   __fastcall _RTC_CheckStackVars2(void *_Esp, _RTC_framedesc *_Fd, _RTC_ALLOCA_NODE *_AllocaList);
    _RTCINTERNAL_DEPRECATED  void   __fastcall _RTC_AllocaHelper(_RTC_ALLOCA_NODE *_PAllocaBase, size_t _CbSize, _RTC_ALLOCA_NODE **_PAllocaInfoList);
#endif  /* !defined (_M_CEE) && !defined (_M_CEE_PURE) */

    /* Uninitialized Local call */
    void   __cdecl     _RTC_UninitUse(const char *_Varname);

    /* Subsystem initialization stuff */
    void    __cdecl    _RTC_Shutdown(void);
    void    __cdecl    _RTC_InitBase(void);


#ifdef __cplusplus

    void* _ReturnAddress();
}

#endif  /* __cplusplus */

#pragma pack(pop)

#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS

#endif  /* defined(_RTC) || !defined(_CRTBLD) */

#endif  /* _INC_RTCAPI */
