

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


#ifndef __audioclientactivationparams_h__
#define __audioclientactivationparams_h__

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
#include "wtypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_audioclientactivationparams_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if (NTDDI_VERSION >= NTDDI_WIN10_FE) 
// Identifier for virtual audio device that supports audio loopback based on
// a process ID instead of the device interface path of a physical audio device.
// Use this for the deviceInterfacePath parameter of ActivateAudioInterfaceAsync when
// AUDIOCLIENT_ACTIVATION_PARAMS::ActivationType is set to AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK.
#define VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK L"VAD\\Process_Loopback"
typedef /* [v1_enum] */ 
enum PROCESS_LOOPBACK_MODE
    {
        PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE	= 0,
        PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE	= 1
    } 	PROCESS_LOOPBACK_MODE;

// This structure is used when creating an IAudioClient using ActivateAudioInterfaceAsync
// for process-based loopback capture. The captured audio either includes or excludes audio rendered
// by the specified process and its child processes, based on how the ProcessLoopbackMode field is set.
typedef struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS
    {
    DWORD TargetProcessId;
    PROCESS_LOOPBACK_MODE ProcessLoopbackMode;
    } 	AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS;

typedef /* [v1_enum] */ 
enum AUDIOCLIENT_ACTIVATION_TYPE
    {
        AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT	= 0,
        AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK	= 1
    } 	AUDIOCLIENT_ACTIVATION_TYPE;

// Activation parameter structure that can be used with ActivateAudioInterfaceAsync
// to create an IAudioClient.
typedef struct AUDIOCLIENT_ACTIVATION_PARAMS
    {
    AUDIOCLIENT_ACTIVATION_TYPE ActivationType;
    union 
        {
        AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS ProcessLoopbackParams;
        } 	DUMMYUNIONNAME;
    } 	AUDIOCLIENT_ACTIVATION_PARAMS;

#endif // (WINVER >= NTDDI_WIN10_FE) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_audioclientactivationparams_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audioclientactivationparams_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


