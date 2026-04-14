// Copyright (c) Microsoft Corporation.  All rights reserved

#if defined(_MSC_VER)
#pragma once
#endif

#include <winapifamily.h>

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#include <inspectable.h>
#include <dxgi.h>

STDAPI CreateDirect3D11DeviceFromDXGIDevice(
    _In_         IDXGIDevice* dxgiDevice,
    _COM_Outptr_ IInspectable** graphicsDevice);

STDAPI CreateDirect3D11SurfaceFromDXGISurface(
    _In_         IDXGISurface* dgxiSurface,
    _COM_Outptr_ IInspectable** graphicsSurface);


#if defined(__cplusplus)

namespace Windows { namespace Graphics { namespace DirectX { namespace Direct3D11 {

struct __declspec(uuid("A9B3D012-3DF2-4EE3-B8D1-8695F457D3C1"))
IDirect3DDxgiInterfaceAccess : public IUnknown
{
    IFACEMETHOD(GetInterface)(REFIID iid, _COM_Outptr_ void** p) = 0;
};

} } } } /* Windows::Graphics::DirectX::Direct3D11 */

#endif /* __cplusplus */

#if defined(__cplusplus_winrt)

#include <wrl.h>

namespace Windows { namespace Graphics { namespace DirectX { namespace Direct3D11 {

    inline IDirect3DDevice^ CreateDirect3DDevice(
        _In_ IDXGIDevice* dxgiDevice)
    {
        using Microsoft::WRL::ComPtr;
        using Platform::Object;

        ComPtr<IInspectable> inspectableDevice;
        __abi_ThrowIfFailed(CreateDirect3D11DeviceFromDXGIDevice(
            dxgiDevice,
            &inspectableDevice));

        Object^ objectDevice = reinterpret_cast<Object^>(inspectableDevice.Get());

        return safe_cast<IDirect3DDevice^>(objectDevice);
    }

    inline IDirect3DSurface^ CreateDirect3DSurface(
        _In_ IDXGISurface* dxgiSurface)
    {
        using Microsoft::WRL::ComPtr;
        using Platform::Object;

        ComPtr<IInspectable> inspectableSurface;
        __abi_ThrowIfFailed(CreateDirect3D11SurfaceFromDXGISurface(
            dxgiSurface,
            &inspectableSurface));

        Object^ objectSurface = reinterpret_cast<Object^>(inspectableSurface.Get());

        return safe_cast<IDirect3DSurface^>(objectSurface);
    }

    inline HRESULT GetDXGIInterfaceFromObject(
        _In_         Platform::Object^ object,
        _In_         REFIID iid,
        _COM_Outptr_ void** p)
    {
        using Microsoft::WRL::ComPtr;

        IInspectable* inspectable = reinterpret_cast<IInspectable*>(object);
        ComPtr<IDirect3DDxgiInterfaceAccess> dxgiInterfaceAccess;
        HRESULT hr = inspectable->QueryInterface(IID_PPV_ARGS(&dxgiInterfaceAccess));
        if (SUCCEEDED(hr))
        {
            hr = dxgiInterfaceAccess->GetInterface(iid, p);
        }
        return hr;
    }

    template<typename DXGI_TYPE>
    HRESULT GetDXGIInterface(
        _In_         IDirect3DDevice^ device,
        _COM_Outptr_ DXGI_TYPE** dxgi)
    {
        return GetDXGIInterfaceFromObject(device, IID_PPV_ARGS(dxgi));
    }

    template<typename DXGI_TYPE>
    HRESULT GetDXGIInterface(
        _In_ IDirect3DSurface^ surface,
        _Out_ DXGI_TYPE** dxgi)
    {
        return GetDXGIInterfaceFromObject(surface, IID_PPV_ARGS(dxgi));
    }

} /* Direct3D11 */ } /* DirectX */ } /* Graphics */ } /* Windows */

#endif /* __cplusplus_winrt */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

