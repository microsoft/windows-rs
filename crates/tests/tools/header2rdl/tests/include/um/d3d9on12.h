/*==========================================================================;
 *
 *  Copyright (C) Microsoft Corporation.  All Rights Reserved.
 *
 *  File:   d3d9on12.h
 *  Content:    Direct3D include file
 *
 ****************************************************************************/

#ifndef _D3D9ON12_H_
#define _D3D9ON12_H_
#include "d3d9.h"
#include "d3d12.h"

// include this file content only if compiling for DX9 interfaces
#if(DIRECT3D_VERSION >= 0x0900)

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// COM interface stuff to allow 9On12 struct to reference IUnknown
#include <unknwn.h>

#define MAX_D3D9ON12_QUEUES        2

typedef struct _D3D9ON12_ARGS
{
    BOOL Enable9On12;
    IUnknown *pD3D12Device;
    IUnknown *ppD3D12Queues[MAX_D3D9ON12_QUEUES];
    UINT NumQueues;
    UINT NodeMask;
} D3D9ON12_ARGS;

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Entry point interfaces for creating a IDirect3D9 with 9On12 arguments
 */

typedef HRESULT (WINAPI *PFN_Direct3DCreate9On12Ex)(UINT SDKVersion, D3D9ON12_ARGS *pOverrideList, UINT NumOverrideEntries, IDirect3D9Ex** ppOutputInterface);
HRESULT WINAPI Direct3DCreate9On12Ex(UINT SDKVersion, D3D9ON12_ARGS *pOverrideList, UINT NumOverrideEntries, IDirect3D9Ex** ppOutputInterface);

typedef IDirect3D9* (WINAPI *PFN_Direct3DCreate9On12)(UINT SDKVersion, D3D9ON12_ARGS *pOverrideList, UINT NumOverrideEntries);
IDirect3D9* WINAPI Direct3DCreate9On12(UINT SDKVersion, D3D9ON12_ARGS *pOverrideList, UINT NumOverrideEntries);


/* IID_IDirect3DDevice9On12 */
/* {e7fda234-b589-4049-940d-8878977531c8} */
DEFINE_GUID(IID_IDirect3DDevice9On12, 0xe7fda234, 0xb589, 0x4049, 0x94, 0x0d, 0x88, 0x78, 0x97, 0x75, 0x31, 0xc8);

interface DECLSPEC_UUID("e7fda234-b589-4049-940d-8878977531c8") IDirect3DDevice9On12;

#if defined(_COM_SMARTPTR_TYPEDEF)
_COM_SMARTPTR_TYPEDEF(IDirect3DDevice9On12, __uuidof(IDirect3DDevice9On12));
#endif




#undef INTERFACE
#define INTERFACE IDirect3DDevice9On12

DECLARE_INTERFACE_(IDirect3DDevice9On12, IUnknown)
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface)(THIS_ REFIID riid, void** ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef)(THIS) PURE;
    STDMETHOD_(ULONG,Release)(THIS) PURE;

    /*** IDirect3DDevice9On12 methods ***/
    STDMETHOD(GetD3D12Device)(THIS_ REFIID riid,void** ppvDevice) PURE;
    STDMETHOD(UnwrapUnderlyingResource)(THIS_ IDirect3DResource9* pResource,ID3D12CommandQueue* pCommandQueue,REFIID riid,void** ppvResource12) PURE;
    STDMETHOD(ReturnUnderlyingResource)(THIS_ IDirect3DResource9* pResource,UINT NumSync,UINT64* pSignalValues,ID3D12Fence** ppFences) PURE;
};
    
typedef struct IDirect3DDevice9On12 *LPDIRECT3DDEVICE9ON12, *PDIRECT3DDEVICE9ON12;

#if !defined(__cplusplus) || defined(CINTERFACE)
#define IDirect3DDevice9On12_QueryInterface(p,a,b) (p)->lpVtbl->QueryInterface(p,a,b)
#define IDirect3DDevice9On12_AddRef(p) (p)->lpVtbl->AddRef(p)
#define IDirect3DDevice9On12_Release(p) (p)->lpVtbl->Release(p)
#define IDirect3DDevice9On12_GetD3D12Device(p,a,b) (p)->lpVtbl->GetD3D12Device(p,a,b)
#define IDirect3DDevice9On12_UnwrapUnderlyingResource(p,a,b,c,d) (p)->lpVtbl->UnwrapUnderlyingResource(p,a,b,c,d)
#define IDirect3DDevice9On12_ReturnUnderlyingResource(p,a,b,c,d) (p)->lpVtbl->ReturnUnderlyingResource(p,a,b,c,d)
#else
#define IDirect3DDevice9On12_QueryInterface(p,a,b) (p)->QueryInterface(a,b)
#define IDirect3DDevice9On12_AddRef(p) (p)->AddRef()
#define IDirect3DDevice9On12_Release(p) (p)->Release()
#define IDirect3DDevice9On12_GetD3D12Device(p,a,b) (p)->GetD3D12Device(a,b)
#define IDirect3DDevice9On12_UnwrapUnderlyingResource(p,a,b,c,d) (p)->UnwrapUnderlyingResource(a,b,c,d)
#define IDirect3DDevice9On12_ReturnUnderlyingResource(p,a,b,c,d) (p)->ReturnUnderlyingResource(a,b,c,d)
#endif



#ifdef __cplusplus
};
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* (DIRECT3D_VERSION >= 0x0900) */
#endif /* _D3D_H_ */

