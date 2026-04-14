

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


#ifndef __PresentationTypes_h__
#define __PresentationTypes_h__

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

/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "dxgi1_6.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_PresentationTypes_0000_0000 */
/* [local] */ 

typedef struct SystemInterruptTime
    {
    UINT64 value;
    } 	SystemInterruptTime;

typedef struct PresentationTransform
    {
    float M11;
    float M12;
    float M21;
    float M22;
    float M31;
    float M32;
    } 	PresentationTransform;

typedef UINT64 CompositionFrameId;

typedef 
enum PresentStatisticsKind
    {
        PresentStatisticsKind_PresentStatus	= 1,
        PresentStatisticsKind_CompositionFrame	= 2,
        PresentStatisticsKind_IndependentFlipFrame	= 3
    } 	PresentStatisticsKind;



extern RPC_IF_HANDLE __MIDL_itf_PresentationTypes_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_PresentationTypes_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


