
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
#ifndef __windows2Egraphics2Edirectx2Edirect3d11_h__
#define __windows2Egraphics2Edirectx2Edirect3d11_h__
#ifndef __windows2Egraphics2Edirectx2Edirect3d11_p_h__
#define __windows2Egraphics2Edirectx2Edirect3d11_p_h__


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
#include "Windows.Graphics.DirectX.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    interface IDirect3DDevice;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    interface IDirect3DSurface;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IClosable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIClosable ABI::Windows::Foundation::IClosable

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                typedef enum DirectXPixelFormat : int DirectXPixelFormat;
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    typedef struct Direct3DMultisampleDescription Direct3DMultisampleDescription;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    typedef struct Direct3DSurfaceDescription Direct3DSurfaceDescription;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Graphics.DirectX.Direct3D11.Direct3DBindings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    enum Direct3DBindings : unsigned int
                    {
                        Direct3DBindings_VertexBuffer = 0x1,
                        Direct3DBindings_IndexBuffer = 0x2,
                        Direct3DBindings_ConstantBuffer = 0x4,
                        Direct3DBindings_ShaderResource = 0x8,
                        Direct3DBindings_StreamOutput = 0x10,
                        Direct3DBindings_RenderTarget = 0x20,
                        Direct3DBindings_DepthStencil = 0x40,
                        Direct3DBindings_UnorderedAccess = 0x80,
                        Direct3DBindings_Decoder = 0x200,
                        Direct3DBindings_VideoEncoder = 0x400,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(Direct3DBindings)
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.DirectX.Direct3D11.Direct3DUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    enum Direct3DUsage : int
                    {
                        Direct3DUsage_Default = 0,
                        Direct3DUsage_Immutable = 1,
                        Direct3DUsage_Dynamic = 2,
                        Direct3DUsage_Staging = 3,
                    };
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.DirectX.Direct3D11.Direct3DMultisampleDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    struct Direct3DMultisampleDescription
                    {
                        INT32 Count;
                        INT32 Quality;
                    };
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.DirectX.Direct3D11.Direct3DSurfaceDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    struct Direct3DSurfaceDescription
                    {
                        INT32 Width;
                        INT32 Height;
                        ABI::Windows::Graphics::DirectX::DirectXPixelFormat Format;
                        ABI::Windows::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription MultisampleDescription;
                    };
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.DirectX.Direct3D11.IDirect3DDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_DirectX_Direct3D11_IDirect3DDevice[] = L"Windows.Graphics.DirectX.Direct3D11.IDirect3DDevice";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    MIDL_INTERFACE("a37624ab-8d5f-4650-9d3e-9eae3d9bc670")
                    IDirect3DDevice : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Trim(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDirect3DDevice = __uuidof(IDirect3DDevice);
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_DirectX_Direct3D11_IDirect3DSurface[] = L"Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    MIDL_INTERFACE("0bf4a146-13c1-4694-bee3-7abf15eaf586")
                    IDirect3DSurface : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            ABI::Windows::Graphics::DirectX::Direct3D11::Direct3DSurfaceDescription* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDirect3DSurface = __uuidof(IDirect3DSurface);
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat;

typedef struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DMultisampleDescription __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DMultisampleDescription;

typedef struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DSurfaceDescription __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DSurfaceDescription;

/*
 *
 * Struct Windows.Graphics.DirectX.Direct3D11.Direct3DBindings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DBindings
{
    Direct3DBindings_VertexBuffer = 0x1,
    Direct3DBindings_IndexBuffer = 0x2,
    Direct3DBindings_ConstantBuffer = 0x4,
    Direct3DBindings_ShaderResource = 0x8,
    Direct3DBindings_StreamOutput = 0x10,
    Direct3DBindings_RenderTarget = 0x20,
    Direct3DBindings_DepthStencil = 0x40,
    Direct3DBindings_UnorderedAccess = 0x80,
    Direct3DBindings_Decoder = 0x200,
    Direct3DBindings_VideoEncoder = 0x400,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.DirectX.Direct3D11.Direct3DUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DUsage
{
    Direct3DUsage_Default = 0,
    Direct3DUsage_Immutable = 1,
    Direct3DUsage_Dynamic = 2,
    Direct3DUsage_Staging = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.DirectX.Direct3D11.Direct3DMultisampleDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DMultisampleDescription
{
    INT32 Count;
    INT32 Quality;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.DirectX.Direct3D11.Direct3DSurfaceDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DSurfaceDescription
{
    INT32 Width;
    INT32 Height;
    enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat Format;
    struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DMultisampleDescription MultisampleDescription;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.DirectX.Direct3D11.IDirect3DDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_DirectX_Direct3D11_IDirect3DDevice[] = L"Windows.Graphics.DirectX.Direct3D11.IDirect3DDevice";
typedef struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Trim)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* This);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDeviceVtbl;

interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_Trim(This) \
    ((This)->lpVtbl->Trim(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_DirectX_Direct3D11_IDirect3DSurface[] = L"Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface";
typedef struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurfaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* This,
        struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DSurfaceDescription* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurfaceVtbl;

interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurfaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egraphics2Edirectx2Edirect3d11_p_h__

#endif // __windows2Egraphics2Edirectx2Edirect3d11_h__
