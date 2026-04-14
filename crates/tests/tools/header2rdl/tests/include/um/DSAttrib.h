

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __dsattrib_h__
#define __dsattrib_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __IAttributeSet_FWD_DEFINED__
#define __IAttributeSet_FWD_DEFINED__
typedef interface IAttributeSet IAttributeSet;

#endif 	/* __IAttributeSet_FWD_DEFINED__ */


#ifndef __IAttributeGet_FWD_DEFINED__
#define __IAttributeGet_FWD_DEFINED__
typedef interface IAttributeGet IAttributeGet;

#endif 	/* __IAttributeGet_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "objidl.h"
#include "strmif.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_dsattrib_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)




extern RPC_IF_HANDLE __MIDL_itf_dsattrib_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dsattrib_0000_0000_v0_0_s_ifspec;

#ifndef __IAttributeSet_INTERFACE_DEFINED__
#define __IAttributeSet_INTERFACE_DEFINED__

/* interface IAttributeSet */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAttributeSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("583ec3cc-4960-4857-982b-41a33ea0a006")
    IAttributeSet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAttrib( 
            /* [in] */ GUID guidAttribute,
            /* [in] */ __RPC__in BYTE *pbAttribute,
            /* [in] */ DWORD dwAttributeLength) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAttributeSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAttributeSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAttributeSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAttributeSet * This);
        
        DECLSPEC_XFGVIRT(IAttributeSet, SetAttrib)
        HRESULT ( STDMETHODCALLTYPE *SetAttrib )( 
            __RPC__in IAttributeSet * This,
            /* [in] */ GUID guidAttribute,
            /* [in] */ __RPC__in BYTE *pbAttribute,
            /* [in] */ DWORD dwAttributeLength);
        
        END_INTERFACE
    } IAttributeSetVtbl;

    interface IAttributeSet
    {
        CONST_VTBL struct IAttributeSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAttributeSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAttributeSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAttributeSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAttributeSet_SetAttrib(This,guidAttribute,pbAttribute,dwAttributeLength)	\
    ( (This)->lpVtbl -> SetAttrib(This,guidAttribute,pbAttribute,dwAttributeLength) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAttributeSet_INTERFACE_DEFINED__ */


#ifndef __IAttributeGet_INTERFACE_DEFINED__
#define __IAttributeGet_INTERFACE_DEFINED__

/* interface IAttributeGet */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAttributeGet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("52dbd1ec-e48f-4528-9232-f442a68f0ae1")
    IAttributeGet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out LONG *plCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttribIndexed( 
            /* [in] */ LONG lIndex,
            /* [out] */ __RPC__out GUID *pguidAttribute,
            /* [out][in] */ __RPC__inout BYTE *pbAttribute,
            /* [out][in] */ __RPC__inout DWORD *pdwAttributeLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttrib( 
            /* [in] */ GUID guidAttribute,
            /* [out][in] */ __RPC__inout BYTE *pbAttribute,
            /* [out][in] */ __RPC__inout DWORD *pdwAttributeLength) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAttributeGetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAttributeGet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAttributeGet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAttributeGet * This);
        
        DECLSPEC_XFGVIRT(IAttributeGet, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IAttributeGet * This,
            /* [out] */ __RPC__out LONG *plCount);
        
        DECLSPEC_XFGVIRT(IAttributeGet, GetAttribIndexed)
        HRESULT ( STDMETHODCALLTYPE *GetAttribIndexed )( 
            __RPC__in IAttributeGet * This,
            /* [in] */ LONG lIndex,
            /* [out] */ __RPC__out GUID *pguidAttribute,
            /* [out][in] */ __RPC__inout BYTE *pbAttribute,
            /* [out][in] */ __RPC__inout DWORD *pdwAttributeLength);
        
        DECLSPEC_XFGVIRT(IAttributeGet, GetAttrib)
        HRESULT ( STDMETHODCALLTYPE *GetAttrib )( 
            __RPC__in IAttributeGet * This,
            /* [in] */ GUID guidAttribute,
            /* [out][in] */ __RPC__inout BYTE *pbAttribute,
            /* [out][in] */ __RPC__inout DWORD *pdwAttributeLength);
        
        END_INTERFACE
    } IAttributeGetVtbl;

    interface IAttributeGet
    {
        CONST_VTBL struct IAttributeGetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAttributeGet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAttributeGet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAttributeGet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAttributeGet_GetCount(This,plCount)	\
    ( (This)->lpVtbl -> GetCount(This,plCount) ) 

#define IAttributeGet_GetAttribIndexed(This,lIndex,pguidAttribute,pbAttribute,pdwAttributeLength)	\
    ( (This)->lpVtbl -> GetAttribIndexed(This,lIndex,pguidAttribute,pbAttribute,pdwAttributeLength) ) 

#define IAttributeGet_GetAttrib(This,guidAttribute,pbAttribute,pdwAttributeLength)	\
    ( (This)->lpVtbl -> GetAttrib(This,guidAttribute,pbAttribute,pdwAttributeLength) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAttributeGet_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_dsattrib_0000_0002 */
/* [local] */ 

// ------------------------------------------------------
// GUID: DSATTRIB_UDCRTag
// ------------------------------------------------------
typedef struct _UDCR_TAG {
    BYTE        bVersion ;
    BYTE        KID [24 + 1] ;
    ULONGLONG   ullBaseCounter ;
    ULONGLONG   ullBaseCounterRange ;
    BOOL        fScrambled ;
    BYTE        bStreamMark ;
    DWORD       dwReserved1 ;
    DWORD       dwReserved2 ;
} UDCR_TAG ;
// ------------------------------------------------------
// GUID: DSATTRIB_PicSampleSeq
// ------------------------------------------------------
// Older defines with mnemonics specific to MPEG-2 video:
#define SAMPLE_SEQ_SEQUENCE_HEADER      1
#define SAMPLE_SEQ_GOP_HEADER           2
#define SAMPLE_SEQ_PICTURE_HEADER       3
// Latest defines with mnemonics generic to MPEG-2 and H.264 video:
#define SAMPLE_SEQ_SEQUENCE_START       SAMPLE_SEQ_SEQUENCE_HEADER
#define SAMPLE_SEQ_SEEK_POINT           SAMPLE_SEQ_GOP_HEADER
#define SAMPLE_SEQ_FRAME_START          SAMPLE_SEQ_PICTURE_HEADER
// Older defines with mnemonics specific to MPEG-2 video:
#define SAMPLE_SEQ_CONTENT_UNKNOWN      0
#define SAMPLE_SEQ_CONTENT_I_FRAME      1
#define SAMPLE_SEQ_CONTENT_P_FRAME      2
#define SAMPLE_SEQ_CONTENT_B_FRAME      3
// Latest defines with mnemonics generic to MPEG-2 and H.264 video:
#define SAMPLE_SEQ_CONTENT_STANDALONE_FRAME      SAMPLE_SEQ_CONTENT_I_FRAME
#define SAMPLE_SEQ_CONTENT_REF_FRAME             SAMPLE_SEQ_CONTENT_P_FRAME
#define SAMPLE_SEQ_CONTENT_NONREF_FRAME          SAMPLE_SEQ_CONTENT_B_FRAME
typedef struct _PIC_SEQ_SAMPLE {
    DWORD
      Content   : 3,    //  SAMPLE_CONTENT_*
      Start     : 1,    //  '1' if the sample is a PictureSampleSequence start
      FrameRate : 4,    //  see below (h.262, table 6-4)
      InvalidVA  : 1,   //  0 normally. 1 if VA attempted but failed
      Reserved   : 19,  //  reserved; shall have value 0
      Version   : 4 ;   //  1 [0 means InvalidVA defaults to 0]
} PIC_SEQ_SAMPLE, *PPIC_SEQ_SAMPLE ;
//      Frame Rate
//          (see h.262, table 6-4)
//
//          0000    reserved (ignore field value)
//          0001    23.976
//          0010    24
//          0011    25
//          0100    29.97
//          0101    30
//          0110    50
//          0111    59.94
//          1000    60
typedef struct _SAMPLE_SEQ_OFFSET {
    DWORD
      Type      : 4,       //  SAMPLE_SEQ_*; 0 means not used
      Offset    : 20,      //  downstream byte offset; 0xfffff means > 2^20-1
      Reserved  : 8 ;      //  reserved; o
} SAMPLE_SEQ_OFFSET ;
#define OFFSET_MARKER_COUNT(attr_len)   \
    ((attr_len - sizeof (PIC_SEQ_SAMPLE)) / sizeof (SAMPLE_SEQ_OFFSET))
#define PIC_SEQ_SAMPLE_ATTR_LEN(cOffsets)   (sizeof (PIC_SEQ_SAMPLE) + (cOffsets) * sizeof (SAMPLE_SEQ_OFFSET))
// ------------------------------------------------------
// GUID: DSATTRIB_OptionalVideoAttributes
// ------------------------------------------------------

typedef enum VA_VIDEO_FORMAT
{
    // Original video format known:

    VA_VIDEO_COMPONENT = 0,
    VA_VIDEO_PAL = 1,
    VA_VIDEO_NTSC = 2,
    VA_VIDEO_SECAM = 3,
    VA_VIDEO_MAC = 4,

    // Original video format not known:

    VA_VIDEO_UNSPECIFIED = 5

    // The MPEG-2 video and H.264 specs reserve values
    // 6 and 7 for future use. 
} VA_VIDEO_FORMAT;

typedef enum VA_COLOR_PRIMARIES
{
    // 0 is reserved by both H.264 and MPEG-2 video

    VA_PRIMARIES_ITU_R_BT_709 = 1,

    // Use this value if the color primaries system is not
    // known or non-standard:

    VA_PRIMARIES_UNSPECIFIED = 2, 

    // 3 is reserved by both H.264 and MPEG-2 video

    VA_PRIMARIES_ITU_R_BT_470_SYSTEM_M = 4,
    VA_PRIMARIES_ITU_R_BT_470_SYSTEM_B_G = 5,
    VA_PRIMARIES_SMPTE_170M = 6,
    VA_PRIMARIES_SMPTE_240M = 7,
    VA_PRIMARIES_H264_GENERIC_FILM = 8

    //  Values in the range 9 to 255 are reserved by H.264. Values in 
    // the range 8 to 255 are reserved by MPEG-2 video
} VA_COLOR_PRIMARIES;

typedef enum VA_TRANSFER_CHARACTERISTICS
{
    // The value 0 is reserved by both MPEG-2 video and H.264

    VA_TRANSFER_CHARACTERISTICS_ITU_R_BT_709 = 1,

    // Use the value below of the transfer characteristics
    // are unknown or non-standard:

    VA_TRANSFER_CHARACTERISTICS_UNSPECIFIED = 2, 

    // The value 3 is reserved by both MPEG-2 video and H.264

    VA_TRANSFER_CHARACTERISTICS_ITU_R_BT_470_SYSTEM_M = 4,
    VA_TRANSFER_CHARACTERISTICS_ITU_R_BT_470_SYSTEM_B_G = 5,
    VA_TRANSFER_CHARACTERISTICS_SMPTE_170M = 6,
    VA_TRANSFER_CHARACTERISTICS_SMPTE_240M = 7,
    VA_TRANSFER_CHARACTERISTICS_LINEAR = 8,
    VA_TRANSFER_CHARACTERISTICS_H264_LOG_100_TO_1 = 9,
    VA_TRANSFER_CHARACTERISTICS_H264_LOG_316_TO_1 = 10

    // The values 11 to 255 are reserved by H.264. The values
    // 9 to 255 are reserved by MPEG-2 video.
} VA_TRANSFER_CHARACTERISTICS;

typedef enum VA_MATRIX_COEFFICIENTS
{
    VA_MATRIX_COEFF_H264_RGB = 0,
    VA_MATRIX_COEFF_ITU_R_BT_709 = 1,

    // Use the value below if the matrix coefficients are
    // unknown or non-standard:

    VA_MATRIX_COEFF_UNSPECIFIED = 2,

    // The value 3 is reserved by both MPEG-2 video and H.264

    VA_MATRIX_COEFF_FCC = 4,
    VA_MATRIX_COEFF_ITU_R_BT_470_SYSTEM_B_G = 5,
    VA_MATRIX_COEFF_SMPTE_170M = 6,
    VA_MATRIX_COEFF_SMPTE_240M = 7,
    VA_MATRIX_COEFF_H264_YCgCo = 8

    // The values 9 to 255 are reserved by H.264. The values 8 to 255
    // are reserved by MPEG-2 video.
} VA_MATRIX_COEFFICIENTS;

typedef struct VA_OPTIONAL_VIDEO_PROPERTIES
{
    WORD					dwPictureHeight ;
    WORD		 			dwPictureWidth ;
    WORD					dwAspectRatioX ;
    WORD					dwAspectRatioY ;
    VA_VIDEO_FORMAT			VAVideoFormat;
    VA_COLOR_PRIMARIES 		VAColorPrimaries;
    VA_TRANSFER_CHARACTERISTICS VATransferCharacteristics;
    VA_MATRIX_COEFFICIENTS 	VAMatrixCoefficients;
}  VA_OPTIONAL_VIDEO_PROPERTIES;
// ------------------------------------------------------
// GUID: DSATTRIB_TRANSPORT_PROPERTIES
// ------------------------------------------------------
typedef struct _TRANSPORT_PROPERTIES {
    ULONG               PID ;
    REFERENCE_TIME      PCR ;
    union {
        struct _Others {
            LONGLONG 
                TransportScramblingControl : 8 ,
                Reserved : 56 ;
        } Others ;
        LONGLONG Value ;
    } Fields ;
} TRANSPORT_PROPERTIES ;
// ------------------------------------------------------
// GUID: DSATTRIB_PBDATAG_ATTRIBUTE
// ------------------------------------------------------
typedef struct _PBDA_TAG_ATTRIBUTE {
    GUID                TableUUId ;
    BYTE                TableId ;
    WORD                VersionNo ;
    DWORD               TableDataSize ;
    BYTE                TableData [1] ;
} PBDA_TAG_ATTRIBUTE ;
// ------------------------------------------------------
// GUID: DSATTRIB_CAPTURE_STREAMTIME
// ------------------------------------------------------
typedef struct _CAPTURE_STREAMTIME {
    REFERENCE_TIME      StreamTime ;
} CAPTURE_STREAMTIME ;
// ------------------------------------------------------
// GUID: DSATTRIB_DSHOW_STREAM_DESC
// ------------------------------------------------------
typedef struct _DSHOW_STREAM_DESC {
    DWORD   VersionNo ;
    DWORD   StreamId ;
    BOOL    Default ;
    BOOL    Creation ;
    DWORD   Reserved ;
} DSHOW_STREAM_DESC ;
// ------------------------------------------------------
// GUID: DSATTRIB_SAMPLE_LIVE_STREAM_TIME
// ------------------------------------------------------
typedef struct _SAMPLE_LIVE_STREAM_TIME{
    ULONGLONG qwStreamTime;
    ULONGLONG qwLiveTime;
}SAMPLE_LIVE_STREAM_TIME ;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_dsattrib_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dsattrib_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


