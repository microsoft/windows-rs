//////////////////////////////////////////////////////////////////////////////
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  File:       D3DCompiler.inl
//  Content:    D3D Compilation Inline Functions
//
//////////////////////////////////////////////////////////////////////////////

#ifndef __D3DCOMPILER_INL__
#define __D3DCOMPILER_INL__

#include "d3dcompiler.h"

//////////////////////////////////////////////////////////////////////////////
// APIs //////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

//----------------------------------------------------------------------------
// Wrappers to retrieve specific reflection interfaces.
//----------------------------------------------------------------------------

FORCEINLINE HRESULT
D3D12Reflect(_In_reads_bytes_(SrcDataSize) LPCVOID pSrcData,
             _In_ SIZE_T SrcDataSize,
             _Out_ ID3D11ShaderReflection** ppReflector)
{
    return D3DReflect(pSrcData, SrcDataSize,
                      IID_ID3D12ShaderReflection, (void**)ppReflector);
}

FORCEINLINE HRESULT
D3D12ReflectLibrary(_In_reads_bytes_(SrcDataSize) LPCVOID pSrcData,
                    _In_ SIZE_T SrcDataSize,
                    _Out_ ID3D12LibraryReflection ** ppReflector)
{
    return D3DReflectLibrary(pSrcData, SrcDataSize,
                             IID_ID3D12LibraryReflection, (void**)ppReflector);
}

#endif // #ifndef __D3DCOMPILER_INL__
