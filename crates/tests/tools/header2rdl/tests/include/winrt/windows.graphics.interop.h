
//---------------------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
//---------------------------------------------------------------------------

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef __Windows_GRAPHICS_INTEROP_H
#define __Windows_GRAPHICS_INTEROP_H

#include <Windows.graphics.h>
#include <sdkddkver.h>

// Forward declarations
interface ID2D1Geometry;
interface ID2D1Factory;

#if defined(____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__) || defined(MIDL_NS_PREFIX)
namespace ABI {
#endif
namespace Windows {
namespace Graphics {

//------------------------------------------------------------------------------
//
// interface IGeometrySource2DInterop
//
//------------------------------------------------------------------------------

#undef INTERFACE
#define INTERFACE IGeometrySource2DInterop
DECLARE_INTERFACE_IID_(IGeometrySource2DInterop, IUnknown, "0657AF73-53FD-47CF-84FF-C8492D2A80A3")
{
    IFACEMETHOD(GetGeometry)(
        _COM_Outptr_ ID2D1Geometry** value
        ) PURE;

    IFACEMETHOD(TryGetGeometryUsingFactory)(
        _In_ ID2D1Factory* factory,
        _COM_Outptr_result_maybenull_ ID2D1Geometry** value
        ) PURE;
};


} // namespace Graphics
} // namespace Windows
#if defined(____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__) || defined(MIDL_NS_PREFIX)
} // namespace ABI 
#endif

#endif // __Windows_GRAPHICS_INTEROP_H
