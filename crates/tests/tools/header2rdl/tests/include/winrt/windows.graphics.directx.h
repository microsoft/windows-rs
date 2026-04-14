
#pragma warning( disable: 4049 )  /* more than 64k source lines */

/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include <rpc.h>
#include <rpcndr.h>

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include <windows.h>
#include <ole2.h>
#endif /*COM_NO_WINDOWS_H*/
#ifndef __windows2Egraphics2Edirectx_h__
#define __windows2Egraphics2Edirectx_h__
#ifndef __windows2Egraphics2Edirectx_p_h__
#define __windows2Egraphics2Edirectx_p_h__


#pragma once

//
// Deprecated attribute support
//

#pragma push_macro("DEPRECATED")
#undef DEPRECATED

#if !defined(DISABLE_WINRT_DEPRECATION)
#if defined(__cplusplus)
#if __cplusplus >= 201402
#define DEPRECATED(x) [[deprecated(x)]]
#define DEPRECATEDENUMERATOR(x) [[deprecated(x)]]
#elif defined(_MSC_VER)
#if _MSC_VER >= 1900
#define DEPRECATED(x) [[deprecated(x)]]
#define DEPRECATEDENUMERATOR(x) [[deprecated(x)]]
#else
#define DEPRECATED(x) __declspec(deprecated(x))
#define DEPRECATEDENUMERATOR(x)
#endif // _MSC_VER >= 1900
#else // Not Standard C++ or MSVC, ignore the construct.
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif  // C++ deprecation
#else // C - disable deprecation
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif
#else // Deprecation is disabled
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif  /* DEPRECATED */

// Disable Deprecation for this header, MIDL verifies that cross-type access is acceptable
#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#else
#pragma warning(push)
#pragma warning(disable: 4996)
#endif

// Ensure that the setting of the /ns_prefix command line switch is consistent for all headers.
// If you get an error from the compiler indicating "warning C4005: 'CHECK_NS_PREFIX_STATE': macro redefinition", this
// indicates that you have included two different headers with different settings for the /ns_prefix MIDL command line switch
#if !defined(DISABLE_NS_PREFIX_CHECKS)
#define CHECK_NS_PREFIX_STATE "always"
#endif // !defined(DISABLE_NS_PREFIX_CHECKS)


#pragma push_macro("MIDL_CONST_ID")
#undef MIDL_CONST_ID
#define MIDL_CONST_ID const __declspec(selectany)


//  API Contract Inclusion Definitions
#if !defined(SPECIFIC_API_CONTRACT_DEFINITIONS)
#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
// Parameterized interface forward declarations (C++)

// Collection interface definitions
/*
 *
 * Struct Windows.Graphics.DirectX.DirectXAlphaMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                enum DirectXAlphaMode : int
                {
                    DirectXAlphaMode_Unspecified = 0,
                    DirectXAlphaMode_Premultiplied = 1,
                    DirectXAlphaMode_Straight = 2,
                    DirectXAlphaMode_Ignore = 3,
                };
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Graphics.DirectX.DirectXColorSpace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                enum DirectXColorSpace : int
                {
                    DirectXColorSpace_RgbFullG22NoneP709 = 0,
                    DirectXColorSpace_RgbFullG10NoneP709 = 1,
                    DirectXColorSpace_RgbStudioG22NoneP709 = 2,
                    DirectXColorSpace_RgbStudioG22NoneP2020 = 3,
                    DirectXColorSpace_Reserved = 4,
                    DirectXColorSpace_YccFullG22NoneP709X601 = 5,
                    DirectXColorSpace_YccStudioG22LeftP601 = 6,
                    DirectXColorSpace_YccFullG22LeftP601 = 7,
                    DirectXColorSpace_YccStudioG22LeftP709 = 8,
                    DirectXColorSpace_YccFullG22LeftP709 = 9,
                    DirectXColorSpace_YccStudioG22LeftP2020 = 10,
                    DirectXColorSpace_YccFullG22LeftP2020 = 11,
                    DirectXColorSpace_RgbFullG2084NoneP2020 = 12,
                    DirectXColorSpace_YccStudioG2084LeftP2020 = 13,
                    DirectXColorSpace_RgbStudioG2084NoneP2020 = 14,
                    DirectXColorSpace_YccStudioG22TopLeftP2020 = 15,
                    DirectXColorSpace_YccStudioG2084TopLeftP2020 = 16,
                    DirectXColorSpace_RgbFullG22NoneP2020 = 17,
                    DirectXColorSpace_YccStudioGHlgTopLeftP2020 = 18,
                    DirectXColorSpace_YccFullGHlgTopLeftP2020 = 19,
                    DirectXColorSpace_RgbStudioG24NoneP709 = 20,
                    DirectXColorSpace_RgbStudioG24NoneP2020 = 21,
                    DirectXColorSpace_YccStudioG24LeftP709 = 22,
                    DirectXColorSpace_YccStudioG24LeftP2020 = 23,
                    DirectXColorSpace_YccStudioG24TopLeftP2020 = 24,
                };
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.DirectX.DirectXPixelFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                enum DirectXPixelFormat : int
                {
                    DirectXPixelFormat_Unknown = 0,
                    DirectXPixelFormat_R32G32B32A32Typeless = 1,
                    DirectXPixelFormat_R32G32B32A32Float = 2,
                    DirectXPixelFormat_R32G32B32A32UInt = 3,
                    DirectXPixelFormat_R32G32B32A32Int = 4,
                    DirectXPixelFormat_R32G32B32Typeless = 5,
                    DirectXPixelFormat_R32G32B32Float = 6,
                    DirectXPixelFormat_R32G32B32UInt = 7,
                    DirectXPixelFormat_R32G32B32Int = 8,
                    DirectXPixelFormat_R16G16B16A16Typeless = 9,
                    DirectXPixelFormat_R16G16B16A16Float = 10,
                    DirectXPixelFormat_R16G16B16A16UIntNormalized = 11,
                    DirectXPixelFormat_R16G16B16A16UInt = 12,
                    DirectXPixelFormat_R16G16B16A16IntNormalized = 13,
                    DirectXPixelFormat_R16G16B16A16Int = 14,
                    DirectXPixelFormat_R32G32Typeless = 15,
                    DirectXPixelFormat_R32G32Float = 16,
                    DirectXPixelFormat_R32G32UInt = 17,
                    DirectXPixelFormat_R32G32Int = 18,
                    DirectXPixelFormat_R32G8X24Typeless = 19,
                    DirectXPixelFormat_D32FloatS8X24UInt = 20,
                    DirectXPixelFormat_R32FloatX8X24Typeless = 21,
                    DirectXPixelFormat_X32TypelessG8X24UInt = 22,
                    DirectXPixelFormat_R10G10B10A2Typeless = 23,
                    DirectXPixelFormat_R10G10B10A2UIntNormalized = 24,
                    DirectXPixelFormat_R10G10B10A2UInt = 25,
                    DirectXPixelFormat_R11G11B10Float = 26,
                    DirectXPixelFormat_R8G8B8A8Typeless = 27,
                    DirectXPixelFormat_R8G8B8A8UIntNormalized = 28,
                    DirectXPixelFormat_R8G8B8A8UIntNormalizedSrgb = 29,
                    DirectXPixelFormat_R8G8B8A8UInt = 30,
                    DirectXPixelFormat_R8G8B8A8IntNormalized = 31,
                    DirectXPixelFormat_R8G8B8A8Int = 32,
                    DirectXPixelFormat_R16G16Typeless = 33,
                    DirectXPixelFormat_R16G16Float = 34,
                    DirectXPixelFormat_R16G16UIntNormalized = 35,
                    DirectXPixelFormat_R16G16UInt = 36,
                    DirectXPixelFormat_R16G16IntNormalized = 37,
                    DirectXPixelFormat_R16G16Int = 38,
                    DirectXPixelFormat_R32Typeless = 39,
                    DirectXPixelFormat_D32Float = 40,
                    DirectXPixelFormat_R32Float = 41,
                    DirectXPixelFormat_R32UInt = 42,
                    DirectXPixelFormat_R32Int = 43,
                    DirectXPixelFormat_R24G8Typeless = 44,
                    DirectXPixelFormat_D24UIntNormalizedS8UInt = 45,
                    DirectXPixelFormat_R24UIntNormalizedX8Typeless = 46,
                    DirectXPixelFormat_X24TypelessG8UInt = 47,
                    DirectXPixelFormat_R8G8Typeless = 48,
                    DirectXPixelFormat_R8G8UIntNormalized = 49,
                    DirectXPixelFormat_R8G8UInt = 50,
                    DirectXPixelFormat_R8G8IntNormalized = 51,
                    DirectXPixelFormat_R8G8Int = 52,
                    DirectXPixelFormat_R16Typeless = 53,
                    DirectXPixelFormat_R16Float = 54,
                    DirectXPixelFormat_D16UIntNormalized = 55,
                    DirectXPixelFormat_R16UIntNormalized = 56,
                    DirectXPixelFormat_R16UInt = 57,
                    DirectXPixelFormat_R16IntNormalized = 58,
                    DirectXPixelFormat_R16Int = 59,
                    DirectXPixelFormat_R8Typeless = 60,
                    DirectXPixelFormat_R8UIntNormalized = 61,
                    DirectXPixelFormat_R8UInt = 62,
                    DirectXPixelFormat_R8IntNormalized = 63,
                    DirectXPixelFormat_R8Int = 64,
                    DirectXPixelFormat_A8UIntNormalized = 65,
                    DirectXPixelFormat_R1UIntNormalized = 66,
                    DirectXPixelFormat_R9G9B9E5SharedExponent = 67,
                    DirectXPixelFormat_R8G8B8G8UIntNormalized = 68,
                    DirectXPixelFormat_G8R8G8B8UIntNormalized = 69,
                    DirectXPixelFormat_BC1Typeless = 70,
                    DirectXPixelFormat_BC1UIntNormalized = 71,
                    DirectXPixelFormat_BC1UIntNormalizedSrgb = 72,
                    DirectXPixelFormat_BC2Typeless = 73,
                    DirectXPixelFormat_BC2UIntNormalized = 74,
                    DirectXPixelFormat_BC2UIntNormalizedSrgb = 75,
                    DirectXPixelFormat_BC3Typeless = 76,
                    DirectXPixelFormat_BC3UIntNormalized = 77,
                    DirectXPixelFormat_BC3UIntNormalizedSrgb = 78,
                    DirectXPixelFormat_BC4Typeless = 79,
                    DirectXPixelFormat_BC4UIntNormalized = 80,
                    DirectXPixelFormat_BC4IntNormalized = 81,
                    DirectXPixelFormat_BC5Typeless = 82,
                    DirectXPixelFormat_BC5UIntNormalized = 83,
                    DirectXPixelFormat_BC5IntNormalized = 84,
                    DirectXPixelFormat_B5G6R5UIntNormalized = 85,
                    DirectXPixelFormat_B5G5R5A1UIntNormalized = 86,
                    DirectXPixelFormat_B8G8R8A8UIntNormalized = 87,
                    DirectXPixelFormat_B8G8R8X8UIntNormalized = 88,
                    DirectXPixelFormat_R10G10B10XRBiasA2UIntNormalized = 89,
                    DirectXPixelFormat_B8G8R8A8Typeless = 90,
                    DirectXPixelFormat_B8G8R8A8UIntNormalizedSrgb = 91,
                    DirectXPixelFormat_B8G8R8X8Typeless = 92,
                    DirectXPixelFormat_B8G8R8X8UIntNormalizedSrgb = 93,
                    DirectXPixelFormat_BC6HTypeless = 94,
                    DirectXPixelFormat_BC6H16UnsignedFloat = 95,
                    DirectXPixelFormat_BC6H16Float = 96,
                    DirectXPixelFormat_BC7Typeless = 97,
                    DirectXPixelFormat_BC7UIntNormalized = 98,
                    DirectXPixelFormat_BC7UIntNormalizedSrgb = 99,
                    DirectXPixelFormat_Ayuv = 100,
                    DirectXPixelFormat_Y410 = 101,
                    DirectXPixelFormat_Y416 = 102,
                    DirectXPixelFormat_NV12 = 103,
                    DirectXPixelFormat_P010 = 104,
                    DirectXPixelFormat_P016 = 105,
                    DirectXPixelFormat_Opaque420 = 106,
                    DirectXPixelFormat_Yuy2 = 107,
                    DirectXPixelFormat_Y210 = 108,
                    DirectXPixelFormat_Y216 = 109,
                    DirectXPixelFormat_NV11 = 110,
                    DirectXPixelFormat_AI44 = 111,
                    DirectXPixelFormat_IA44 = 112,
                    DirectXPixelFormat_P8 = 113,
                    DirectXPixelFormat_A8P8 = 114,
                    DirectXPixelFormat_B4G4R4A4UIntNormalized = 115,
                    DirectXPixelFormat_P208 = 130,
                    DirectXPixelFormat_V208 = 131,
                    DirectXPixelFormat_V408 = 132,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DirectXPixelFormat_SamplerFeedbackMinMipOpaque = 189,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DirectXPixelFormat_SamplerFeedbackMipRegionUsedOpaque = 190,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
                    DirectXPixelFormat_A4B4G4R4 = 191,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
                };
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.DirectX.DirectXPrimitiveTopology
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                enum DirectXPrimitiveTopology : int
                {
                    DirectXPrimitiveTopology_Undefined = 0,
                    DirectXPrimitiveTopology_PointList = 1,
                    DirectXPrimitiveTopology_LineList = 2,
                    DirectXPrimitiveTopology_LineStrip = 3,
                    DirectXPrimitiveTopology_TriangleList = 4,
                    DirectXPrimitiveTopology_TriangleStrip = 5,
                };
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#else // !defined(__cplusplus)
/* Forward Declarations */
// Parameterized interface forward declarations (C)

// Collection interface definitions

/*
 *
 * Struct Windows.Graphics.DirectX.DirectXAlphaMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXAlphaMode
{
    DirectXAlphaMode_Unspecified = 0,
    DirectXAlphaMode_Premultiplied = 1,
    DirectXAlphaMode_Straight = 2,
    DirectXAlphaMode_Ignore = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Graphics.DirectX.DirectXColorSpace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXColorSpace
{
    DirectXColorSpace_RgbFullG22NoneP709 = 0,
    DirectXColorSpace_RgbFullG10NoneP709 = 1,
    DirectXColorSpace_RgbStudioG22NoneP709 = 2,
    DirectXColorSpace_RgbStudioG22NoneP2020 = 3,
    DirectXColorSpace_Reserved = 4,
    DirectXColorSpace_YccFullG22NoneP709X601 = 5,
    DirectXColorSpace_YccStudioG22LeftP601 = 6,
    DirectXColorSpace_YccFullG22LeftP601 = 7,
    DirectXColorSpace_YccStudioG22LeftP709 = 8,
    DirectXColorSpace_YccFullG22LeftP709 = 9,
    DirectXColorSpace_YccStudioG22LeftP2020 = 10,
    DirectXColorSpace_YccFullG22LeftP2020 = 11,
    DirectXColorSpace_RgbFullG2084NoneP2020 = 12,
    DirectXColorSpace_YccStudioG2084LeftP2020 = 13,
    DirectXColorSpace_RgbStudioG2084NoneP2020 = 14,
    DirectXColorSpace_YccStudioG22TopLeftP2020 = 15,
    DirectXColorSpace_YccStudioG2084TopLeftP2020 = 16,
    DirectXColorSpace_RgbFullG22NoneP2020 = 17,
    DirectXColorSpace_YccStudioGHlgTopLeftP2020 = 18,
    DirectXColorSpace_YccFullGHlgTopLeftP2020 = 19,
    DirectXColorSpace_RgbStudioG24NoneP709 = 20,
    DirectXColorSpace_RgbStudioG24NoneP2020 = 21,
    DirectXColorSpace_YccStudioG24LeftP709 = 22,
    DirectXColorSpace_YccStudioG24LeftP2020 = 23,
    DirectXColorSpace_YccStudioG24TopLeftP2020 = 24,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.DirectX.DirectXPixelFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat
{
    DirectXPixelFormat_Unknown = 0,
    DirectXPixelFormat_R32G32B32A32Typeless = 1,
    DirectXPixelFormat_R32G32B32A32Float = 2,
    DirectXPixelFormat_R32G32B32A32UInt = 3,
    DirectXPixelFormat_R32G32B32A32Int = 4,
    DirectXPixelFormat_R32G32B32Typeless = 5,
    DirectXPixelFormat_R32G32B32Float = 6,
    DirectXPixelFormat_R32G32B32UInt = 7,
    DirectXPixelFormat_R32G32B32Int = 8,
    DirectXPixelFormat_R16G16B16A16Typeless = 9,
    DirectXPixelFormat_R16G16B16A16Float = 10,
    DirectXPixelFormat_R16G16B16A16UIntNormalized = 11,
    DirectXPixelFormat_R16G16B16A16UInt = 12,
    DirectXPixelFormat_R16G16B16A16IntNormalized = 13,
    DirectXPixelFormat_R16G16B16A16Int = 14,
    DirectXPixelFormat_R32G32Typeless = 15,
    DirectXPixelFormat_R32G32Float = 16,
    DirectXPixelFormat_R32G32UInt = 17,
    DirectXPixelFormat_R32G32Int = 18,
    DirectXPixelFormat_R32G8X24Typeless = 19,
    DirectXPixelFormat_D32FloatS8X24UInt = 20,
    DirectXPixelFormat_R32FloatX8X24Typeless = 21,
    DirectXPixelFormat_X32TypelessG8X24UInt = 22,
    DirectXPixelFormat_R10G10B10A2Typeless = 23,
    DirectXPixelFormat_R10G10B10A2UIntNormalized = 24,
    DirectXPixelFormat_R10G10B10A2UInt = 25,
    DirectXPixelFormat_R11G11B10Float = 26,
    DirectXPixelFormat_R8G8B8A8Typeless = 27,
    DirectXPixelFormat_R8G8B8A8UIntNormalized = 28,
    DirectXPixelFormat_R8G8B8A8UIntNormalizedSrgb = 29,
    DirectXPixelFormat_R8G8B8A8UInt = 30,
    DirectXPixelFormat_R8G8B8A8IntNormalized = 31,
    DirectXPixelFormat_R8G8B8A8Int = 32,
    DirectXPixelFormat_R16G16Typeless = 33,
    DirectXPixelFormat_R16G16Float = 34,
    DirectXPixelFormat_R16G16UIntNormalized = 35,
    DirectXPixelFormat_R16G16UInt = 36,
    DirectXPixelFormat_R16G16IntNormalized = 37,
    DirectXPixelFormat_R16G16Int = 38,
    DirectXPixelFormat_R32Typeless = 39,
    DirectXPixelFormat_D32Float = 40,
    DirectXPixelFormat_R32Float = 41,
    DirectXPixelFormat_R32UInt = 42,
    DirectXPixelFormat_R32Int = 43,
    DirectXPixelFormat_R24G8Typeless = 44,
    DirectXPixelFormat_D24UIntNormalizedS8UInt = 45,
    DirectXPixelFormat_R24UIntNormalizedX8Typeless = 46,
    DirectXPixelFormat_X24TypelessG8UInt = 47,
    DirectXPixelFormat_R8G8Typeless = 48,
    DirectXPixelFormat_R8G8UIntNormalized = 49,
    DirectXPixelFormat_R8G8UInt = 50,
    DirectXPixelFormat_R8G8IntNormalized = 51,
    DirectXPixelFormat_R8G8Int = 52,
    DirectXPixelFormat_R16Typeless = 53,
    DirectXPixelFormat_R16Float = 54,
    DirectXPixelFormat_D16UIntNormalized = 55,
    DirectXPixelFormat_R16UIntNormalized = 56,
    DirectXPixelFormat_R16UInt = 57,
    DirectXPixelFormat_R16IntNormalized = 58,
    DirectXPixelFormat_R16Int = 59,
    DirectXPixelFormat_R8Typeless = 60,
    DirectXPixelFormat_R8UIntNormalized = 61,
    DirectXPixelFormat_R8UInt = 62,
    DirectXPixelFormat_R8IntNormalized = 63,
    DirectXPixelFormat_R8Int = 64,
    DirectXPixelFormat_A8UIntNormalized = 65,
    DirectXPixelFormat_R1UIntNormalized = 66,
    DirectXPixelFormat_R9G9B9E5SharedExponent = 67,
    DirectXPixelFormat_R8G8B8G8UIntNormalized = 68,
    DirectXPixelFormat_G8R8G8B8UIntNormalized = 69,
    DirectXPixelFormat_BC1Typeless = 70,
    DirectXPixelFormat_BC1UIntNormalized = 71,
    DirectXPixelFormat_BC1UIntNormalizedSrgb = 72,
    DirectXPixelFormat_BC2Typeless = 73,
    DirectXPixelFormat_BC2UIntNormalized = 74,
    DirectXPixelFormat_BC2UIntNormalizedSrgb = 75,
    DirectXPixelFormat_BC3Typeless = 76,
    DirectXPixelFormat_BC3UIntNormalized = 77,
    DirectXPixelFormat_BC3UIntNormalizedSrgb = 78,
    DirectXPixelFormat_BC4Typeless = 79,
    DirectXPixelFormat_BC4UIntNormalized = 80,
    DirectXPixelFormat_BC4IntNormalized = 81,
    DirectXPixelFormat_BC5Typeless = 82,
    DirectXPixelFormat_BC5UIntNormalized = 83,
    DirectXPixelFormat_BC5IntNormalized = 84,
    DirectXPixelFormat_B5G6R5UIntNormalized = 85,
    DirectXPixelFormat_B5G5R5A1UIntNormalized = 86,
    DirectXPixelFormat_B8G8R8A8UIntNormalized = 87,
    DirectXPixelFormat_B8G8R8X8UIntNormalized = 88,
    DirectXPixelFormat_R10G10B10XRBiasA2UIntNormalized = 89,
    DirectXPixelFormat_B8G8R8A8Typeless = 90,
    DirectXPixelFormat_B8G8R8A8UIntNormalizedSrgb = 91,
    DirectXPixelFormat_B8G8R8X8Typeless = 92,
    DirectXPixelFormat_B8G8R8X8UIntNormalizedSrgb = 93,
    DirectXPixelFormat_BC6HTypeless = 94,
    DirectXPixelFormat_BC6H16UnsignedFloat = 95,
    DirectXPixelFormat_BC6H16Float = 96,
    DirectXPixelFormat_BC7Typeless = 97,
    DirectXPixelFormat_BC7UIntNormalized = 98,
    DirectXPixelFormat_BC7UIntNormalizedSrgb = 99,
    DirectXPixelFormat_Ayuv = 100,
    DirectXPixelFormat_Y410 = 101,
    DirectXPixelFormat_Y416 = 102,
    DirectXPixelFormat_NV12 = 103,
    DirectXPixelFormat_P010 = 104,
    DirectXPixelFormat_P016 = 105,
    DirectXPixelFormat_Opaque420 = 106,
    DirectXPixelFormat_Yuy2 = 107,
    DirectXPixelFormat_Y210 = 108,
    DirectXPixelFormat_Y216 = 109,
    DirectXPixelFormat_NV11 = 110,
    DirectXPixelFormat_AI44 = 111,
    DirectXPixelFormat_IA44 = 112,
    DirectXPixelFormat_P8 = 113,
    DirectXPixelFormat_A8P8 = 114,
    DirectXPixelFormat_B4G4R4A4UIntNormalized = 115,
    DirectXPixelFormat_P208 = 130,
    DirectXPixelFormat_V208 = 131,
    DirectXPixelFormat_V408 = 132,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DirectXPixelFormat_SamplerFeedbackMinMipOpaque = 189,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DirectXPixelFormat_SamplerFeedbackMipRegionUsedOpaque = 190,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
    DirectXPixelFormat_A4B4G4R4 = 191,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.DirectX.DirectXPrimitiveTopology
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPrimitiveTopology
{
    DirectXPrimitiveTopology_Undefined = 0,
    DirectXPrimitiveTopology_PointList = 1,
    DirectXPrimitiveTopology_LineList = 2,
    DirectXPrimitiveTopology_LineStrip = 3,
    DirectXPrimitiveTopology_TriangleList = 4,
    DirectXPrimitiveTopology_TriangleStrip = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egraphics2Edirectx_p_h__

#endif // __windows2Egraphics2Edirectx_h__
