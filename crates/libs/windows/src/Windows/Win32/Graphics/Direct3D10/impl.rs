pub trait ID3D10AsynchronousImpl: Sized + ID3D10DeviceChildImpl {
    fn Begin();
    fn End();
    fn GetData();
    fn GetDataSize();
}
impl ID3D10AsynchronousVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10AsynchronousImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10AsynchronousVtbl {
        unsafe extern "system" fn Begin<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn End<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetData<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataSize<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, Begin::<Impl, IMPL_OFFSET>, End::<Impl, IMPL_OFFSET>, GetData::<Impl, IMPL_OFFSET>, GetDataSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Asynchronous as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10BlendStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10BlendStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10BlendStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10BlendStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10BlendStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10BlendState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10BlendState1Impl: Sized + ID3D10BlendStateImpl + ID3D10DeviceChildImpl {
    fn GetDesc1();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10BlendState1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10BlendState1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10BlendState1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D10BlendState1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetDesc1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10BlendState1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10BufferImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
impl ID3D10BufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10BufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10BufferVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10BufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10BufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10BufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BUFFER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            SetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetEvictionPriority::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Buffer as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10CounterImpl: Sized + ID3D10AsynchronousImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ID3D10CounterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10CounterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10CounterVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10CounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_COUNTER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, Begin::<Impl, IMPL_OFFSET>, End::<Impl, IMPL_OFFSET>, GetData::<Impl, IMPL_OFFSET>, GetDataSize::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Counter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ID3D10DebugImpl: Sized {
    fn SetFeatureMask();
    fn GetFeatureMask();
    fn SetPresentPerRenderOpDelay();
    fn GetPresentPerRenderOpDelay();
    fn SetSwapChain();
    fn GetSwapChain();
    fn Validate();
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ID3D10DebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DebugImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10DebugVtbl {
        unsafe extern "system" fn SetFeatureMask<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeatureMask<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSwapChain<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pswapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSwapChain<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Validate<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetFeatureMask::<Impl, IMPL_OFFSET>, GetFeatureMask::<Impl, IMPL_OFFSET>, SetPresentPerRenderOpDelay::<Impl, IMPL_OFFSET>, GetPresentPerRenderOpDelay::<Impl, IMPL_OFFSET>, SetSwapChain::<Impl, IMPL_OFFSET>, GetSwapChain::<Impl, IMPL_OFFSET>, Validate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Debug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10DepthStencilStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10DepthStencilStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DepthStencilStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10DepthStencilStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10DepthStencilStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10DepthStencilState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10DepthStencilViewImpl: Sized + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10DepthStencilViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DepthStencilViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10DepthStencilViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10DepthStencilViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10DepthStencilView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10DeviceImpl: Sized {
    fn VSSetConstantBuffers();
    fn PSSetShaderResources();
    fn PSSetShader();
    fn PSSetSamplers();
    fn VSSetShader();
    fn DrawIndexed();
    fn Draw();
    fn PSSetConstantBuffers();
    fn IASetInputLayout();
    fn IASetVertexBuffers();
    fn IASetIndexBuffer();
    fn DrawIndexedInstanced();
    fn DrawInstanced();
    fn GSSetConstantBuffers();
    fn GSSetShader();
    fn IASetPrimitiveTopology();
    fn VSSetShaderResources();
    fn VSSetSamplers();
    fn SetPredication();
    fn GSSetShaderResources();
    fn GSSetSamplers();
    fn OMSetRenderTargets();
    fn OMSetBlendState();
    fn OMSetDepthStencilState();
    fn SOSetTargets();
    fn DrawAuto();
    fn RSSetState();
    fn RSSetViewports();
    fn RSSetScissorRects();
    fn CopySubresourceRegion();
    fn CopyResource();
    fn UpdateSubresource();
    fn ClearRenderTargetView();
    fn ClearDepthStencilView();
    fn GenerateMips();
    fn ResolveSubresource();
    fn VSGetConstantBuffers();
    fn PSGetShaderResources();
    fn PSGetShader();
    fn PSGetSamplers();
    fn VSGetShader();
    fn PSGetConstantBuffers();
    fn IAGetInputLayout();
    fn IAGetVertexBuffers();
    fn IAGetIndexBuffer();
    fn GSGetConstantBuffers();
    fn GSGetShader();
    fn IAGetPrimitiveTopology();
    fn VSGetShaderResources();
    fn VSGetSamplers();
    fn GetPredication();
    fn GSGetShaderResources();
    fn GSGetSamplers();
    fn OMGetRenderTargets();
    fn OMGetBlendState();
    fn OMGetDepthStencilState();
    fn SOGetTargets();
    fn RSGetState();
    fn RSGetViewports();
    fn RSGetScissorRects();
    fn GetDeviceRemovedReason();
    fn SetExceptionMode();
    fn GetExceptionMode();
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn ClearState();
    fn Flush();
    fn CreateBuffer();
    fn CreateTexture1D();
    fn CreateTexture2D();
    fn CreateTexture3D();
    fn CreateShaderResourceView();
    fn CreateRenderTargetView();
    fn CreateDepthStencilView();
    fn CreateInputLayout();
    fn CreateVertexShader();
    fn CreateGeometryShader();
    fn CreateGeometryShaderWithStreamOutput();
    fn CreatePixelShader();
    fn CreateBlendState();
    fn CreateDepthStencilState();
    fn CreateRasterizerState();
    fn CreateSamplerState();
    fn CreateQuery();
    fn CreatePredicate();
    fn CreateCounter();
    fn CheckFormatSupport();
    fn CheckMultisampleQualityLevels();
    fn CheckCounterInfo();
    fn CheckCounter();
    fn GetCreationFlags();
    fn OpenSharedResource();
    fn SetTextFilterSize();
    fn GetTextFilterSize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10DeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10DeviceVtbl {
        unsafe extern "system" fn VSSetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSSetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSSetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelshader: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSSetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSSetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvertexshader: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawIndexed<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Draw<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSSetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IASetInputLayout<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputlayout: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IASetVertexBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::windows::core::RawPtr, pstrides: *const u32, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IASetIndexBuffer<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: ::windows::core::RawPtr, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawIndexedInstanced<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawInstanced<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSSetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSSetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSSetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSSetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPredication<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicate: ::windows::core::RawPtr, predicatevalue: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSSetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSSetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMSetRenderTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMSetBlendState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstate: ::windows::core::RawPtr, blendfactor: *const f32, samplemask: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMSetDepthStencilState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilstate: ::windows::core::RawPtr, stencilref: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SOSetTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const ::windows::core::RawPtr, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawAuto<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSSetState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerstate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSSetViewports<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D10_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSSetScissorRects<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopySubresourceRegion<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D10_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyResource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateSubresource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearRenderTargetView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertargetview: ::windows::core::RawPtr, colorrgba: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearDepthStencilView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilview: ::windows::core::RawPtr, clearflags: u32, depth: f32, stencil: u8) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateMips<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderresourceview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResolveSubresource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSGetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSGetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSGetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppixelshader: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSGetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSGetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvertexshader: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSGetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IAGetInputLayout<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinputlayout: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IAGetVertexBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::windows::core::RawPtr, pstrides: *mut u32, poffsets: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IAGetIndexBuffer<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::windows::core::RawPtr, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSGetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSGetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSGetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSGetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPredication<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppredicate: *mut ::windows::core::RawPtr, ppredicatevalue: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSGetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSGetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMGetRenderTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMGetBlendState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblendstate: *mut ::windows::core::RawPtr, blendfactor: *mut f32, psamplemask: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMGetDepthStencilState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut ::windows::core::RawPtr, pstencilref: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SOGetTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut ::windows::core::RawPtr, poffsets: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSGetState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSGetViewports<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSGetScissorRects<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExceptionMode<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExceptionMode<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBuffer<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTexture1D<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture1d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTexture2D<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTexture3D<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateShaderResourceView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRenderTargetView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDepthStencilView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInputLayout<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVertexShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGeometryShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePixelShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBlendState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDepthStencilState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRasterizerState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSamplerState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateQuery<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePredicate<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCounter<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckFormatSupport<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckCounterInfo<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D10_COUNTER_INFO) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckCounter<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCreationFlags<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenSharedResource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTextFilterSize<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextFilterSize<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            VSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSSetShaderResources::<Impl, IMPL_OFFSET>,
            PSSetShader::<Impl, IMPL_OFFSET>,
            PSSetSamplers::<Impl, IMPL_OFFSET>,
            VSSetShader::<Impl, IMPL_OFFSET>,
            DrawIndexed::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            IASetInputLayout::<Impl, IMPL_OFFSET>,
            IASetVertexBuffers::<Impl, IMPL_OFFSET>,
            IASetIndexBuffer::<Impl, IMPL_OFFSET>,
            DrawIndexedInstanced::<Impl, IMPL_OFFSET>,
            DrawInstanced::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSSetShader::<Impl, IMPL_OFFSET>,
            IASetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSSetShaderResources::<Impl, IMPL_OFFSET>,
            VSSetSamplers::<Impl, IMPL_OFFSET>,
            SetPredication::<Impl, IMPL_OFFSET>,
            GSSetShaderResources::<Impl, IMPL_OFFSET>,
            GSSetSamplers::<Impl, IMPL_OFFSET>,
            OMSetRenderTargets::<Impl, IMPL_OFFSET>,
            OMSetBlendState::<Impl, IMPL_OFFSET>,
            OMSetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOSetTargets::<Impl, IMPL_OFFSET>,
            DrawAuto::<Impl, IMPL_OFFSET>,
            RSSetState::<Impl, IMPL_OFFSET>,
            RSSetViewports::<Impl, IMPL_OFFSET>,
            RSSetScissorRects::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion::<Impl, IMPL_OFFSET>,
            CopyResource::<Impl, IMPL_OFFSET>,
            UpdateSubresource::<Impl, IMPL_OFFSET>,
            ClearRenderTargetView::<Impl, IMPL_OFFSET>,
            ClearDepthStencilView::<Impl, IMPL_OFFSET>,
            GenerateMips::<Impl, IMPL_OFFSET>,
            ResolveSubresource::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSGetShaderResources::<Impl, IMPL_OFFSET>,
            PSGetShader::<Impl, IMPL_OFFSET>,
            PSGetSamplers::<Impl, IMPL_OFFSET>,
            VSGetShader::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            IAGetInputLayout::<Impl, IMPL_OFFSET>,
            IAGetVertexBuffers::<Impl, IMPL_OFFSET>,
            IAGetIndexBuffer::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSGetShader::<Impl, IMPL_OFFSET>,
            IAGetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSGetShaderResources::<Impl, IMPL_OFFSET>,
            VSGetSamplers::<Impl, IMPL_OFFSET>,
            GetPredication::<Impl, IMPL_OFFSET>,
            GSGetShaderResources::<Impl, IMPL_OFFSET>,
            GSGetSamplers::<Impl, IMPL_OFFSET>,
            OMGetRenderTargets::<Impl, IMPL_OFFSET>,
            OMGetBlendState::<Impl, IMPL_OFFSET>,
            OMGetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOGetTargets::<Impl, IMPL_OFFSET>,
            RSGetState::<Impl, IMPL_OFFSET>,
            RSGetViewports::<Impl, IMPL_OFFSET>,
            RSGetScissorRects::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            SetExceptionMode::<Impl, IMPL_OFFSET>,
            GetExceptionMode::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            ClearState::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            CreateBuffer::<Impl, IMPL_OFFSET>,
            CreateTexture1D::<Impl, IMPL_OFFSET>,
            CreateTexture2D::<Impl, IMPL_OFFSET>,
            CreateTexture3D::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView::<Impl, IMPL_OFFSET>,
            CreateDepthStencilView::<Impl, IMPL_OFFSET>,
            CreateInputLayout::<Impl, IMPL_OFFSET>,
            CreateVertexShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShaderWithStreamOutput::<Impl, IMPL_OFFSET>,
            CreatePixelShader::<Impl, IMPL_OFFSET>,
            CreateBlendState::<Impl, IMPL_OFFSET>,
            CreateDepthStencilState::<Impl, IMPL_OFFSET>,
            CreateRasterizerState::<Impl, IMPL_OFFSET>,
            CreateSamplerState::<Impl, IMPL_OFFSET>,
            CreateQuery::<Impl, IMPL_OFFSET>,
            CreatePredicate::<Impl, IMPL_OFFSET>,
            CreateCounter::<Impl, IMPL_OFFSET>,
            CheckFormatSupport::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels::<Impl, IMPL_OFFSET>,
            CheckCounterInfo::<Impl, IMPL_OFFSET>,
            CheckCounter::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            OpenSharedResource::<Impl, IMPL_OFFSET>,
            SetTextFilterSize::<Impl, IMPL_OFFSET>,
            GetTextFilterSize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10Device1Impl: Sized + ID3D10DeviceImpl {
    fn CreateShaderResourceView1();
    fn CreateBlendState1();
    fn GetFeatureLevel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10Device1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Device1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10Device1Vtbl {
        unsafe extern "system" fn CreateShaderResourceView1<Impl: ID3D10Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBlendState1<Impl: ID3D10Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeatureLevel<Impl: ID3D10Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D10_FEATURE_LEVEL1 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            VSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSSetShaderResources::<Impl, IMPL_OFFSET>,
            PSSetShader::<Impl, IMPL_OFFSET>,
            PSSetSamplers::<Impl, IMPL_OFFSET>,
            VSSetShader::<Impl, IMPL_OFFSET>,
            DrawIndexed::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            IASetInputLayout::<Impl, IMPL_OFFSET>,
            IASetVertexBuffers::<Impl, IMPL_OFFSET>,
            IASetIndexBuffer::<Impl, IMPL_OFFSET>,
            DrawIndexedInstanced::<Impl, IMPL_OFFSET>,
            DrawInstanced::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSSetShader::<Impl, IMPL_OFFSET>,
            IASetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSSetShaderResources::<Impl, IMPL_OFFSET>,
            VSSetSamplers::<Impl, IMPL_OFFSET>,
            SetPredication::<Impl, IMPL_OFFSET>,
            GSSetShaderResources::<Impl, IMPL_OFFSET>,
            GSSetSamplers::<Impl, IMPL_OFFSET>,
            OMSetRenderTargets::<Impl, IMPL_OFFSET>,
            OMSetBlendState::<Impl, IMPL_OFFSET>,
            OMSetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOSetTargets::<Impl, IMPL_OFFSET>,
            DrawAuto::<Impl, IMPL_OFFSET>,
            RSSetState::<Impl, IMPL_OFFSET>,
            RSSetViewports::<Impl, IMPL_OFFSET>,
            RSSetScissorRects::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion::<Impl, IMPL_OFFSET>,
            CopyResource::<Impl, IMPL_OFFSET>,
            UpdateSubresource::<Impl, IMPL_OFFSET>,
            ClearRenderTargetView::<Impl, IMPL_OFFSET>,
            ClearDepthStencilView::<Impl, IMPL_OFFSET>,
            GenerateMips::<Impl, IMPL_OFFSET>,
            ResolveSubresource::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSGetShaderResources::<Impl, IMPL_OFFSET>,
            PSGetShader::<Impl, IMPL_OFFSET>,
            PSGetSamplers::<Impl, IMPL_OFFSET>,
            VSGetShader::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            IAGetInputLayout::<Impl, IMPL_OFFSET>,
            IAGetVertexBuffers::<Impl, IMPL_OFFSET>,
            IAGetIndexBuffer::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSGetShader::<Impl, IMPL_OFFSET>,
            IAGetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSGetShaderResources::<Impl, IMPL_OFFSET>,
            VSGetSamplers::<Impl, IMPL_OFFSET>,
            GetPredication::<Impl, IMPL_OFFSET>,
            GSGetShaderResources::<Impl, IMPL_OFFSET>,
            GSGetSamplers::<Impl, IMPL_OFFSET>,
            OMGetRenderTargets::<Impl, IMPL_OFFSET>,
            OMGetBlendState::<Impl, IMPL_OFFSET>,
            OMGetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOGetTargets::<Impl, IMPL_OFFSET>,
            RSGetState::<Impl, IMPL_OFFSET>,
            RSGetViewports::<Impl, IMPL_OFFSET>,
            RSGetScissorRects::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            SetExceptionMode::<Impl, IMPL_OFFSET>,
            GetExceptionMode::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            ClearState::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            CreateBuffer::<Impl, IMPL_OFFSET>,
            CreateTexture1D::<Impl, IMPL_OFFSET>,
            CreateTexture2D::<Impl, IMPL_OFFSET>,
            CreateTexture3D::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView::<Impl, IMPL_OFFSET>,
            CreateDepthStencilView::<Impl, IMPL_OFFSET>,
            CreateInputLayout::<Impl, IMPL_OFFSET>,
            CreateVertexShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShaderWithStreamOutput::<Impl, IMPL_OFFSET>,
            CreatePixelShader::<Impl, IMPL_OFFSET>,
            CreateBlendState::<Impl, IMPL_OFFSET>,
            CreateDepthStencilState::<Impl, IMPL_OFFSET>,
            CreateRasterizerState::<Impl, IMPL_OFFSET>,
            CreateSamplerState::<Impl, IMPL_OFFSET>,
            CreateQuery::<Impl, IMPL_OFFSET>,
            CreatePredicate::<Impl, IMPL_OFFSET>,
            CreateCounter::<Impl, IMPL_OFFSET>,
            CheckFormatSupport::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels::<Impl, IMPL_OFFSET>,
            CheckCounterInfo::<Impl, IMPL_OFFSET>,
            CheckCounter::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            OpenSharedResource::<Impl, IMPL_OFFSET>,
            SetTextFilterSize::<Impl, IMPL_OFFSET>,
            GetTextFilterSize::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView1::<Impl, IMPL_OFFSET>,
            CreateBlendState1::<Impl, IMPL_OFFSET>,
            GetFeatureLevel::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Device1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10DeviceChildImpl: Sized {
    fn GetDevice();
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
}
impl ID3D10DeviceChildVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DeviceChildImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10DeviceChildVtbl {
        unsafe extern "system" fn GetDevice<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectImpl: Sized {
    fn IsValid();
    fn IsPool();
    fn GetDevice();
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetVariableByIndex();
    fn GetVariableByName();
    fn GetVariableBySemantic();
    fn GetTechniqueByIndex();
    fn GetTechniqueByName();
    fn Optimize();
    fn IsOptimized();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPool<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDevice<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableBySemantic<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTechniqueByIndex<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectTechnique> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTechniqueByName<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectTechnique> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Optimize<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOptimized<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            IsValid::<Impl, IMPL_OFFSET>,
            IsPool::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetConstantBufferByIndex::<Impl, IMPL_OFFSET>,
            GetConstantBufferByName::<Impl, IMPL_OFFSET>,
            GetVariableByIndex::<Impl, IMPL_OFFSET>,
            GetVariableByName::<Impl, IMPL_OFFSET>,
            GetVariableBySemantic::<Impl, IMPL_OFFSET>,
            GetTechniqueByIndex::<Impl, IMPL_OFFSET>,
            GetTechniqueByName::<Impl, IMPL_OFFSET>,
            Optimize::<Impl, IMPL_OFFSET>,
            IsOptimized::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Effect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectBlendVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetBlendState();
    fn GetBackingStore();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectBlendVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectBlendVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectBlendVariableVtbl {
        unsafe extern "system" fn GetBlendState<Impl: ID3D10EffectBlendVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectBlendVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            GetBlendState::<Impl, IMPL_OFFSET>,
            GetBackingStore::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectBlendVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectConstantBufferImpl: Sized + ID3D10EffectVariableImpl {
    fn SetConstantBuffer();
    fn GetConstantBuffer();
    fn SetTextureBuffer();
    fn GetTextureBuffer();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectConstantBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectConstantBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectConstantBufferVtbl {
        unsafe extern "system" fn SetConstantBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstantbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstantBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconstantbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTextureBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptexturebuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextureBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptexturebuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            SetConstantBuffer::<Impl, IMPL_OFFSET>,
            GetConstantBuffer::<Impl, IMPL_OFFSET>,
            SetTextureBuffer::<Impl, IMPL_OFFSET>,
            GetTextureBuffer::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectConstantBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectDepthStencilVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetDepthStencilState();
    fn GetBackingStore();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectDepthStencilVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectDepthStencilVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectDepthStencilVariableVtbl {
        unsafe extern "system" fn GetDepthStencilState<Impl: ID3D10EffectDepthStencilVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectDepthStencilVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            GetDepthStencilState::<Impl, IMPL_OFFSET>,
            GetBackingStore::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectDepthStencilVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectDepthStencilViewVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetDepthStencil();
    fn GetDepthStencil();
    fn SetDepthStencilArray();
    fn GetDepthStencilArray();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectDepthStencilViewVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectDepthStencilViewVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectDepthStencilViewVariableVtbl {
        unsafe extern "system" fn SetDepthStencil<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDepthStencil<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDepthStencilArray<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDepthStencilArray<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            SetDepthStencil::<Impl, IMPL_OFFSET>,
            GetDepthStencil::<Impl, IMPL_OFFSET>,
            SetDepthStencilArray::<Impl, IMPL_OFFSET>,
            GetDepthStencilArray::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectDepthStencilViewVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectMatrixVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetMatrix();
    fn GetMatrix();
    fn SetMatrixArray();
    fn GetMatrixArray();
    fn SetMatrixTranspose();
    fn GetMatrixTranspose();
    fn SetMatrixTransposeArray();
    fn GetMatrixTransposeArray();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectMatrixVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectMatrixVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectMatrixVariableVtbl {
        unsafe extern "system" fn SetMatrix<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatrix<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMatrixArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatrixArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMatrixTranspose<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatrixTranspose<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMatrixTransposeArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatrixTransposeArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            SetMatrix::<Impl, IMPL_OFFSET>,
            GetMatrix::<Impl, IMPL_OFFSET>,
            SetMatrixArray::<Impl, IMPL_OFFSET>,
            GetMatrixArray::<Impl, IMPL_OFFSET>,
            SetMatrixTranspose::<Impl, IMPL_OFFSET>,
            GetMatrixTranspose::<Impl, IMPL_OFFSET>,
            SetMatrixTransposeArray::<Impl, IMPL_OFFSET>,
            GetMatrixTransposeArray::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectMatrixVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectPassImpl: Sized {
    fn IsValid();
    fn GetDesc();
    fn GetVertexShaderDesc();
    fn GetGeometryShaderDesc();
    fn GetPixelShaderDesc();
    fn GetAnnotationByIndex();
    fn GetAnnotationByName();
    fn Apply();
    fn ComputeStateBlockMask();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectPassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectPassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectPassVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVertexShaderDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGeometryShaderDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelShaderDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Apply<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputeStateBlockMask<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(IsValid::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetVertexShaderDesc::<Impl, IMPL_OFFSET>, GetGeometryShaderDesc::<Impl, IMPL_OFFSET>, GetPixelShaderDesc::<Impl, IMPL_OFFSET>, GetAnnotationByIndex::<Impl, IMPL_OFFSET>, GetAnnotationByName::<Impl, IMPL_OFFSET>, Apply::<Impl, IMPL_OFFSET>, ComputeStateBlockMask::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectPass as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10EffectPoolImpl: Sized {
    fn AsEffect();
}
impl ID3D10EffectPoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectPoolImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectPoolVtbl {
        unsafe extern "system" fn AsEffect<Impl: ID3D10EffectPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10Effect> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AsEffect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectPool as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectRasterizerVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetRasterizerState();
    fn GetBackingStore();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectRasterizerVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectRasterizerVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectRasterizerVariableVtbl {
        unsafe extern "system" fn GetRasterizerState<Impl: ID3D10EffectRasterizerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectRasterizerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            GetRasterizerState::<Impl, IMPL_OFFSET>,
            GetBackingStore::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectRasterizerVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectRenderTargetViewVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetRenderTarget();
    fn GetRenderTarget();
    fn SetRenderTargetArray();
    fn GetRenderTargetArray();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectRenderTargetViewVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectRenderTargetViewVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectRenderTargetViewVariableVtbl {
        unsafe extern "system" fn SetRenderTarget<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRenderTarget<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRenderTargetArray<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRenderTargetArray<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            SetRenderTarget::<Impl, IMPL_OFFSET>,
            GetRenderTarget::<Impl, IMPL_OFFSET>,
            SetRenderTargetArray::<Impl, IMPL_OFFSET>,
            GetRenderTargetArray::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectRenderTargetViewVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectSamplerVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetSampler();
    fn GetBackingStore();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectSamplerVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectSamplerVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectSamplerVariableVtbl {
        unsafe extern "system" fn GetSampler<Impl: ID3D10EffectSamplerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppsampler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectSamplerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            GetSampler::<Impl, IMPL_OFFSET>,
            GetBackingStore::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectSamplerVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectScalarVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetFloat();
    fn GetFloat();
    fn SetFloatArray();
    fn GetFloatArray();
    fn SetInt();
    fn GetInt();
    fn SetIntArray();
    fn GetIntArray();
    fn SetBool();
    fn GetBool();
    fn SetBoolArray();
    fn GetBoolArray();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectScalarVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectScalarVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectScalarVariableVtbl {
        unsafe extern "system" fn SetFloat<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFloat<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFloatArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFloatArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInt<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInt<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIntArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIntArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBool<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBool<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBoolArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoolArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            SetFloat::<Impl, IMPL_OFFSET>,
            GetFloat::<Impl, IMPL_OFFSET>,
            SetFloatArray::<Impl, IMPL_OFFSET>,
            GetFloatArray::<Impl, IMPL_OFFSET>,
            SetInt::<Impl, IMPL_OFFSET>,
            GetInt::<Impl, IMPL_OFFSET>,
            SetIntArray::<Impl, IMPL_OFFSET>,
            GetIntArray::<Impl, IMPL_OFFSET>,
            SetBool::<Impl, IMPL_OFFSET>,
            GetBool::<Impl, IMPL_OFFSET>,
            SetBoolArray::<Impl, IMPL_OFFSET>,
            GetBoolArray::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectScalarVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectShaderResourceVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetResource();
    fn GetResource();
    fn SetResourceArray();
    fn GetResourceArray();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectShaderResourceVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectShaderResourceVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectShaderResourceVariableVtbl {
        unsafe extern "system" fn SetResource<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResource<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResourceArray<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceArray<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            SetResource::<Impl, IMPL_OFFSET>,
            GetResource::<Impl, IMPL_OFFSET>,
            SetResourceArray::<Impl, IMPL_OFFSET>,
            GetResourceArray::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectShaderResourceVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10EffectShaderVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetShaderDesc();
    fn GetVertexShader();
    fn GetGeometryShader();
    fn GetPixelShader();
    fn GetInputSignatureElementDesc();
    fn GetOutputSignatureElementDesc();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10EffectShaderVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectShaderVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectShaderVariableVtbl {
        unsafe extern "system" fn GetShaderDesc<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVertexShader<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppvs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGeometryShader<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppgs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelShader<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputSignatureElementDesc<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputSignatureElementDesc<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            GetShaderDesc::<Impl, IMPL_OFFSET>,
            GetVertexShader::<Impl, IMPL_OFFSET>,
            GetGeometryShader::<Impl, IMPL_OFFSET>,
            GetPixelShader::<Impl, IMPL_OFFSET>,
            GetInputSignatureElementDesc::<Impl, IMPL_OFFSET>,
            GetOutputSignatureElementDesc::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectShaderVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectStringVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetString();
    fn GetStringArray();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectStringVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectStringVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectStringVariableVtbl {
        unsafe extern "system" fn GetString<Impl: ID3D10EffectStringVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstring: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringArray<Impl: ID3D10EffectStringVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstrings: *mut super::super::Foundation::PSTR, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            GetString::<Impl, IMPL_OFFSET>,
            GetStringArray::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectStringVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectTechniqueImpl: Sized {
    fn IsValid();
    fn GetDesc();
    fn GetAnnotationByIndex();
    fn GetAnnotationByName();
    fn GetPassByIndex();
    fn GetPassByName();
    fn ComputeStateBlockMask();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectTechniqueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectTechniqueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectTechniqueVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPassByIndex<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectPass> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPassByName<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectPass> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputeStateBlockMask<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(IsValid::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetAnnotationByIndex::<Impl, IMPL_OFFSET>, GetAnnotationByName::<Impl, IMPL_OFFSET>, GetPassByIndex::<Impl, IMPL_OFFSET>, GetPassByName::<Impl, IMPL_OFFSET>, ComputeStateBlockMask::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectTechnique as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10EffectTypeImpl: Sized {
    fn IsValid();
    fn GetDesc();
    fn GetMemberTypeByIndex();
    fn GetMemberTypeByName();
    fn GetMemberTypeBySemantic();
    fn GetMemberName();
    fn GetMemberSemantic();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10EffectTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectTypeVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberTypeBySemantic<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberName<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberSemantic<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(IsValid::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetMemberTypeByIndex::<Impl, IMPL_OFFSET>, GetMemberTypeByName::<Impl, IMPL_OFFSET>, GetMemberTypeBySemantic::<Impl, IMPL_OFFSET>, GetMemberName::<Impl, IMPL_OFFSET>, GetMemberSemantic::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectType as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectVariableImpl: Sized {
    fn IsValid();
    fn GetType();
    fn GetDesc();
    fn GetAnnotationByIndex();
    fn GetAnnotationByName();
    fn GetMemberByIndex();
    fn GetMemberByName();
    fn GetMemberBySemantic();
    fn GetElement();
    fn GetParentConstantBuffer();
    fn AsScalar();
    fn AsVector();
    fn AsMatrix();
    fn AsString();
    fn AsShaderResource();
    fn AsRenderTargetView();
    fn AsDepthStencilView();
    fn AsConstantBuffer();
    fn AsShader();
    fn AsBlend();
    fn AsDepthStencil();
    fn AsRasterizer();
    fn AsSampler();
    fn SetRawValue();
    fn GetRawValue();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectVariableVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberByIndex<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberByName<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberBySemantic<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetElement<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParentConstantBuffer<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsScalar<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsVector<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsMatrix<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsString<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsShaderResource<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsRenderTargetView<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsDepthStencilView<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsConstantBuffer<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsShader<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsBlend<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsDepthStencil<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsRasterizer<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsSampler<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRawValue<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRawValue<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectVectorVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetBoolVector();
    fn SetIntVector();
    fn SetFloatVector();
    fn GetBoolVector();
    fn GetIntVector();
    fn GetFloatVector();
    fn SetBoolVectorArray();
    fn SetIntVectorArray();
    fn SetFloatVectorArray();
    fn GetBoolVectorArray();
    fn GetIntVectorArray();
    fn GetFloatVectorArray();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVectorVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectVectorVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectVectorVariableVtbl {
        unsafe extern "system" fn SetBoolVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIntVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFloatVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoolVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIntVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFloatVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBoolVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIntVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFloatVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoolVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIntVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFloatVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            IsValid::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar::<Impl, IMPL_OFFSET>,
            AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix::<Impl, IMPL_OFFSET>,
            AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader::<Impl, IMPL_OFFSET>,
            AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue::<Impl, IMPL_OFFSET>,
            SetBoolVector::<Impl, IMPL_OFFSET>,
            SetIntVector::<Impl, IMPL_OFFSET>,
            SetFloatVector::<Impl, IMPL_OFFSET>,
            GetBoolVector::<Impl, IMPL_OFFSET>,
            GetIntVector::<Impl, IMPL_OFFSET>,
            GetFloatVector::<Impl, IMPL_OFFSET>,
            SetBoolVectorArray::<Impl, IMPL_OFFSET>,
            SetIntVectorArray::<Impl, IMPL_OFFSET>,
            SetFloatVectorArray::<Impl, IMPL_OFFSET>,
            GetBoolVectorArray::<Impl, IMPL_OFFSET>,
            GetIntVectorArray::<Impl, IMPL_OFFSET>,
            GetFloatVectorArray::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectVectorVariable as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10GeometryShaderImpl: Sized + ID3D10DeviceChildImpl {}
impl ID3D10GeometryShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10GeometryShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10GeometryShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10GeometryShader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10InfoQueueImpl: Sized {
    fn SetMessageCountLimit();
    fn ClearStoredMessages();
    fn GetMessage();
    fn GetNumMessagesAllowedByStorageFilter();
    fn GetNumMessagesDeniedByStorageFilter();
    fn GetNumStoredMessages();
    fn GetNumStoredMessagesAllowedByRetrievalFilter();
    fn GetNumMessagesDiscardedByMessageCountLimit();
    fn GetMessageCountLimit();
    fn AddStorageFilterEntries();
    fn GetStorageFilter();
    fn ClearStorageFilter();
    fn PushEmptyStorageFilter();
    fn PushCopyOfStorageFilter();
    fn PushStorageFilter();
    fn PopStorageFilter();
    fn GetStorageFilterStackSize();
    fn AddRetrievalFilterEntries();
    fn GetRetrievalFilter();
    fn ClearRetrievalFilter();
    fn PushEmptyRetrievalFilter();
    fn PushCopyOfRetrievalFilter();
    fn PushRetrievalFilter();
    fn PopRetrievalFilter();
    fn GetRetrievalFilterStackSize();
    fn AddMessage();
    fn AddApplicationMessage();
    fn SetBreakOnCategory();
    fn SetBreakOnSeverity();
    fn SetBreakOnID();
    fn GetBreakOnCategory();
    fn GetBreakOnSeverity();
    fn GetBreakOnID();
    fn SetMuteDebugOutput();
    fn GetMuteDebugOutput();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10InfoQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10InfoQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10InfoQueueVtbl {
        unsafe extern "system" fn SetMessageCountLimit<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearStoredMessages<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMessage<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumStoredMessages<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMessageCountLimit<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStorageFilterEntries<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddMessage<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddApplicationMessage<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBreakOnCategory<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBreakOnID<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBreakOnCategory<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBreakOnSeverity<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBreakOnID<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMuteDebugOutput<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMuteDebugOutput<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetMessageCountLimit::<Impl, IMPL_OFFSET>,
            ClearStoredMessages::<Impl, IMPL_OFFSET>,
            GetMessage::<Impl, IMPL_OFFSET>,
            GetNumMessagesAllowedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDeniedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumStoredMessages::<Impl, IMPL_OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit::<Impl, IMPL_OFFSET>,
            GetMessageCountLimit::<Impl, IMPL_OFFSET>,
            AddStorageFilterEntries::<Impl, IMPL_OFFSET>,
            GetStorageFilter::<Impl, IMPL_OFFSET>,
            ClearStorageFilter::<Impl, IMPL_OFFSET>,
            PushEmptyStorageFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfStorageFilter::<Impl, IMPL_OFFSET>,
            PushStorageFilter::<Impl, IMPL_OFFSET>,
            PopStorageFilter::<Impl, IMPL_OFFSET>,
            GetStorageFilterStackSize::<Impl, IMPL_OFFSET>,
            AddRetrievalFilterEntries::<Impl, IMPL_OFFSET>,
            GetRetrievalFilter::<Impl, IMPL_OFFSET>,
            ClearRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushEmptyRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushRetrievalFilter::<Impl, IMPL_OFFSET>,
            PopRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetRetrievalFilterStackSize::<Impl, IMPL_OFFSET>,
            AddMessage::<Impl, IMPL_OFFSET>,
            AddApplicationMessage::<Impl, IMPL_OFFSET>,
            SetBreakOnCategory::<Impl, IMPL_OFFSET>,
            SetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            SetBreakOnID::<Impl, IMPL_OFFSET>,
            GetBreakOnCategory::<Impl, IMPL_OFFSET>,
            GetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            GetBreakOnID::<Impl, IMPL_OFFSET>,
            SetMuteDebugOutput::<Impl, IMPL_OFFSET>,
            GetMuteDebugOutput::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10InfoQueue as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10InputLayoutImpl: Sized + ID3D10DeviceChildImpl {}
impl ID3D10InputLayoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10InputLayoutImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10InputLayoutVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10InputLayout as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10MultithreadImpl: Sized {
    fn Enter();
    fn Leave();
    fn SetMultithreadProtected();
    fn GetMultithreadProtected();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10MultithreadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10MultithreadImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10MultithreadVtbl {
        unsafe extern "system" fn Enter<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Leave<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMultithreadProtected<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMultithreadProtected<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Enter::<Impl, IMPL_OFFSET>, Leave::<Impl, IMPL_OFFSET>, SetMultithreadProtected::<Impl, IMPL_OFFSET>, GetMultithreadProtected::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Multithread as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10PixelShaderImpl: Sized + ID3D10DeviceChildImpl {}
impl ID3D10PixelShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10PixelShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10PixelShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10PixelShader as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10PredicateImpl: Sized + ID3D10QueryImpl + ID3D10AsynchronousImpl + ID3D10DeviceChildImpl {}
impl ID3D10PredicateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10PredicateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10PredicateVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, Begin::<Impl, IMPL_OFFSET>, End::<Impl, IMPL_OFFSET>, GetData::<Impl, IMPL_OFFSET>, GetDataSize::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Predicate as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10QueryImpl: Sized + ID3D10AsynchronousImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ID3D10QueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10QueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10QueryVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10QueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_QUERY_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, Begin::<Impl, IMPL_OFFSET>, End::<Impl, IMPL_OFFSET>, GetData::<Impl, IMPL_OFFSET>, GetDataSize::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Query as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10RasterizerStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10RasterizerStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10RasterizerStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10RasterizerStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10RasterizerStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RASTERIZER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10RasterizerState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10RenderTargetViewImpl: Sized + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10RenderTargetViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10RenderTargetViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10RenderTargetViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10RenderTargetViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10RenderTargetView as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10ResourceImpl: Sized + ID3D10DeviceChildImpl {
    fn GetType();
    fn SetEvictionPriority();
    fn GetEvictionPriority();
}
impl ID3D10ResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ResourceVtbl {
        unsafe extern "system" fn GetType<Impl: ID3D10ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEvictionPriority<Impl: ID3D10ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEvictionPriority<Impl: ID3D10ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, SetEvictionPriority::<Impl, IMPL_OFFSET>, GetEvictionPriority::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Resource as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10SamplerStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ID3D10SamplerStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10SamplerStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10SamplerStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10SamplerStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SAMPLER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10SamplerState as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10ShaderReflectionImpl: Sized {
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetResourceBindingDesc();
    fn GetInputParameterDesc();
    fn GetOutputParameterDesc();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10ShaderReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputParameterDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputParameterDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetConstantBufferByIndex::<Impl, IMPL_OFFSET>, GetConstantBufferByName::<Impl, IMPL_OFFSET>, GetResourceBindingDesc::<Impl, IMPL_OFFSET>, GetInputParameterDesc::<Impl, IMPL_OFFSET>, GetOutputParameterDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10ShaderReflection1Impl: Sized {
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetResourceBindingDesc();
    fn GetInputParameterDesc();
    fn GetOutputParameterDesc();
    fn GetVariableByName();
    fn GetResourceBindingDescByName();
    fn GetMovInstructionCount();
    fn GetMovcInstructionCount();
    fn GetConversionInstructionCount();
    fn GetBitwiseInstructionCount();
    fn GetGSInputPrimitive();
    fn IsLevel9Shader();
    fn IsSampleFrequencyShader();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10ShaderReflection1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflection1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderReflection1Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputParameterDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputParameterDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMovInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMovcInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversionInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGSInputPrimitive<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprim: *mut super::Direct3D::D3D_PRIMITIVE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLevel9Shader<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblevel9shader: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsamplefrequency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetConstantBufferByIndex::<Impl, IMPL_OFFSET>,
            GetConstantBufferByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDesc::<Impl, IMPL_OFFSET>,
            GetInputParameterDesc::<Impl, IMPL_OFFSET>,
            GetOutputParameterDesc::<Impl, IMPL_OFFSET>,
            GetVariableByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDescByName::<Impl, IMPL_OFFSET>,
            GetMovInstructionCount::<Impl, IMPL_OFFSET>,
            GetMovcInstructionCount::<Impl, IMPL_OFFSET>,
            GetConversionInstructionCount::<Impl, IMPL_OFFSET>,
            GetBitwiseInstructionCount::<Impl, IMPL_OFFSET>,
            GetGSInputPrimitive::<Impl, IMPL_OFFSET>,
            IsLevel9Shader::<Impl, IMPL_OFFSET>,
            IsSampleFrequencyShader::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10ShaderReflectionConstantBufferImpl: Sized {
    fn GetDesc();
    fn GetVariableByIndex();
    fn GetVariableByName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10ShaderReflectionConstantBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionConstantBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderReflectionConstantBufferVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D10ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D10ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(GetDesc::<Impl, IMPL_OFFSET>, GetVariableByIndex::<Impl, IMPL_OFFSET>, GetVariableByName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflectionConstantBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10ShaderReflectionTypeImpl: Sized {
    fn GetDesc();
    fn GetMemberTypeByIndex();
    fn GetMemberTypeByName();
    fn GetMemberTypeName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10ShaderReflectionTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderReflectionTypeVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(GetDesc::<Impl, IMPL_OFFSET>, GetMemberTypeByIndex::<Impl, IMPL_OFFSET>, GetMemberTypeByName::<Impl, IMPL_OFFSET>, GetMemberTypeName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflectionType as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10ShaderReflectionVariableImpl: Sized {
    fn GetDesc();
    fn GetType();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10ShaderReflectionVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderReflectionVariableVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: ID3D10ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(GetDesc::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflectionVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceViewImpl: Sized + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10ShaderResourceViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderResourceViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderResourceViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderResourceViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceView1Impl: Sized + ID3D10ShaderResourceViewImpl + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc1();
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10ShaderResourceView1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderResourceView1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderResourceView1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D10ShaderResourceView1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetDesc1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10StateBlockImpl: Sized {
    fn Capture();
    fn Apply();
    fn ReleaseAllDeviceObjects();
    fn GetDevice();
}
impl ID3D10StateBlockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10StateBlockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10StateBlockVtbl {
        unsafe extern "system" fn Capture<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Apply<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseAllDeviceObjects<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDevice<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Capture::<Impl, IMPL_OFFSET>, Apply::<Impl, IMPL_OFFSET>, ReleaseAllDeviceObjects::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10StateBlock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10SwitchToRefImpl: Sized {
    fn SetUseRef();
    fn GetUseRef();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10SwitchToRefVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10SwitchToRefImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10SwitchToRefVtbl {
        unsafe extern "system" fn SetUseRef<Impl: ID3D10SwitchToRefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUseRef<Impl: ID3D10SwitchToRefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetUseRef::<Impl, IMPL_OFFSET>, GetUseRef::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10SwitchToRef as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture1DImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture1DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Texture1DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10Texture1DVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10Texture1DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10Texture1DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10Texture1DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE1D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            SetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetEvictionPriority::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Texture1D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture2DImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture2DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Texture2DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10Texture2DVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10Texture2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex2d: *mut D3D10_MAPPED_TEXTURE2D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10Texture2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10Texture2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE2D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            SetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetEvictionPriority::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Texture2D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture3DImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Texture3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10Texture3DVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10Texture3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex3d: *mut D3D10_MAPPED_TEXTURE3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10Texture3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10Texture3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE3D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            SetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetEvictionPriority::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Texture3D as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10VertexShaderImpl: Sized + ID3D10DeviceChildImpl {}
impl ID3D10VertexShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10VertexShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10VertexShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10VertexShader as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10ViewImpl: Sized + ID3D10DeviceChildImpl {
    fn GetResource();
}
impl ID3D10ViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ViewVtbl {
        unsafe extern "system" fn GetResource<Impl: ID3D10ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10View as ::windows::core::Interface>::IID
    }
}
