/************************************************************************
*                                                                       *
*   dmksctrl.h -- Definition of IKsControl                              *
*                                                                       *
*   Copyright (c) Microsoft Corporation.  All rights reserved.          *
*                                                                       *
*   This header file contains the definition of IKsControl, which       *
*   duplicates definitions from ks.h and ksproxy.h. Your code should    *
*   include ks.h and ksproxy.h directly if you have them (they are      *
*   provided in the Windows 98 DDK and will be in the Windows NT 5      *
*   SDK).                                                               *
*                                                                       *
************************************************************************/

#ifndef _DMKSCTRL_
#define _DMKSCTRL_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)   /* Disable warnings on anonymous unions */

#include <pshpack8.h>

#include <objbase.h>

#if !defined(_NTRTL_)
    #ifndef DEFINE_GUIDEX
        #define DEFINE_GUIDEX(name) EXTERN_C const CDECL GUID name
    #endif /* !defined(DEFINE_GUIDEX) */

    #ifndef STATICGUIDOF
        #define STATICGUIDOF(guid) STATIC_##guid
    #endif /* !defined(STATICGUIDOF) */
#endif /* !defined(_NTRTL_) */

#ifndef STATIC_IID_IKsControl
#define STATIC_IID_IKsControl\
    0x28F54685L, 0x06FD, 0x11D2, 0xB2, 0x7A, 0x00, 0xA0, 0xC9, 0x22, 0x31, 0x96
#endif /* STATIC_IID_IKsControl */

/* 
 * Warning: This will prevent the rest of ks.h from being pulled in if ks.h is 
 * included after dmksctrl.h. Make sure you do not include both headers in
 * the same source file.
 */
#ifndef _KS_
#define _KS_

#if (defined(_MSC_EXTENSIONS) || defined(__cplusplus)) && !defined(CINTERFACE)
typedef struct {
    union {
        struct {
            GUID    Set;
            ULONG   Id;
            ULONG   Flags;
        };
        LONGLONG    Alignment;
    };
} KSIDENTIFIER, *PKSIDENTIFIER;
#else
typedef struct {
    union {
        struct {
            GUID    Set;
            ULONG   Id;
            ULONG   Flags;
        } Data;
        LONGLONG    Alignment;
    };
} KSIDENTIFIER, *PKSIDENTIFIER;
#endif

typedef KSIDENTIFIER KSPROPERTY, *PKSPROPERTY, KSMETHOD, *PKSMETHOD, KSEVENT, *PKSEVENT;

#define KSMETHOD_TYPE_NONE                  0x00000000
#define KSMETHOD_TYPE_READ                  0x00000001
#define KSMETHOD_TYPE_WRITE                 0x00000002
#define KSMETHOD_TYPE_MODIFY                0x00000003
#define KSMETHOD_TYPE_SOURCE                0x00000004

#define KSMETHOD_TYPE_SEND                  0x00000001
#define KSMETHOD_TYPE_SETSUPPORT            0x00000100
#define KSMETHOD_TYPE_BASICSUPPORT          0x00000200

#define KSPROPERTY_TYPE_GET                 0x00000001
#define KSPROPERTY_TYPE_SET                 0x00000002
#define KSPROPERTY_TYPE_SETSUPPORT          0x00000100
#define KSPROPERTY_TYPE_BASICSUPPORT        0x00000200
#define KSPROPERTY_TYPE_RELATIONS           0x00000400
#define KSPROPERTY_TYPE_SERIALIZESET        0x00000800
#define KSPROPERTY_TYPE_UNSERIALIZESET      0x00001000
#define KSPROPERTY_TYPE_SERIALIZERAW        0x00002000
#define KSPROPERTY_TYPE_UNSERIALIZERAW      0x00004000
#define KSPROPERTY_TYPE_SERIALIZESIZE       0x00008000
#define KSPROPERTY_TYPE_DEFAULTVALUES       0x00010000

#define KSPROPERTY_TYPE_TOPOLOGY            0x10000000
#endif  /* _KS_ */

#ifndef _IKsControl_
#define _IKsControl_

#ifdef DECLARE_INTERFACE_


#undef INTERFACE
#define INTERFACE IKsControl
DECLARE_INTERFACE_(IKsControl, IUnknown)
{
     /* IUnknown */
    STDMETHOD(QueryInterface)       (THIS_ REFIID, LPVOID FAR *) PURE;
    STDMETHOD_(ULONG,AddRef)        (THIS) PURE;
    STDMETHOD_(ULONG,Release)       (THIS) PURE;

    /*IKsControl*/
    STDMETHOD(KsProperty)(
        THIS_
        _In_reads_bytes_(PropertyLength) PKSPROPERTY Property,
        _In_ ULONG PropertyLength,
        _Inout_updates_bytes_(DataLength) LPVOID PropertyData,
        _In_ ULONG DataLength,
        _Out_ ULONG* BytesReturned
    ) PURE;
    STDMETHOD(KsMethod)(
        THIS_
        _In_reads_bytes_(MethodLength) PKSMETHOD Method,
        _In_ ULONG MethodLength,
        _Inout_updates_bytes_(DataLength)  LPVOID MethodData,
        _In_ ULONG DataLength,
        _Out_ ULONG* BytesReturned
    ) PURE;
    STDMETHOD(KsEvent)(
        THIS_
        _In_reads_bytes_(EventLength) PKSEVENT Event OPTIONAL,
        _In_ ULONG EventLength,
        _Inout_updates_bytes_(DataLength) LPVOID EventData,
        _In_ ULONG DataLength,
        _Out_ ULONG* BytesReturned
    ) PURE;
};

#endif /* DECLARE_INTERFACE_ */
#endif /* _IKsControl_ */

#include <poppack.h>

DEFINE_GUID(IID_IKsControl, 0x28F54685, 0x06FD, 0x11D2, 0xB2, 0x7A, 0x00, 0xA0, 0xC9, 0x22, 0x31, 0x96);

/* These formats are in ksmedia.h
 */
#ifndef _KSMEDIA_

DEFINE_GUID(KSDATAFORMAT_SUBTYPE_MIDI, 0x1D262760L, 0xE957, 0x11CF, 0xA5, 0xD6, 0x28, 0xDB, 0x04, 0xC1, 0x00, 0x00);
DEFINE_GUID(KSDATAFORMAT_SUBTYPE_DIRECTMUSIC, 0x1a82f8bc,  0x3f8b, 0x11d2, 0xb7, 0x74, 0x00, 0x60, 0x08, 0x33, 0x16, 0xc1);

#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* _DMKSCTRL */



