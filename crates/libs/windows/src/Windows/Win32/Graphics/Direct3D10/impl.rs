pub trait ID3D10AsynchronousImpl: Sized + ID3D10DeviceChildImpl {
    fn Begin();
    fn End();
    fn GetData();
    fn GetDataSize();
}
impl ::windows::core::RuntimeName for ID3D10Asynchronous {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Asynchronous";
}
impl ID3D10AsynchronousVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10AsynchronousImpl, const OFFSET: isize>() -> ID3D10AsynchronousVtbl {
        unsafe extern "system" fn Begin<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin().into()
        }
        unsafe extern "system" fn End<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End().into()
        }
        unsafe extern "system" fn GetData<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData(::core::mem::transmute_copy(&pdata), datasize, getdataflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataSize<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10Asynchronous>, ::windows::core::GetTrustLevel, Begin::<Impl, OFFSET>, End::<Impl, OFFSET>, GetData::<Impl, OFFSET>, GetDataSize::<Impl, OFFSET>)
    }
}
pub trait ID3D10BlendStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10BlendState {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10BlendState";
}
impl ID3D10BlendStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10BlendStateImpl, const OFFSET: isize>() -> ID3D10BlendStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10BlendStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10BlendState>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10BlendState1Impl: Sized + ID3D10BlendStateImpl + ID3D10DeviceChildImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D10BlendState1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10BlendState1";
}
impl ID3D10BlendState1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10BlendState1Impl, const OFFSET: isize>() -> ID3D10BlendState1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D10BlendState1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10BlendState1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait ID3D10BufferImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10Buffer {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Buffer";
}
impl ID3D10BufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10BufferImpl, const OFFSET: isize>() -> ID3D10BufferVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10BufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Map(maptype, mapflags, ::core::mem::transmute_copy(&ppdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10BufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap().into()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10BufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BUFFER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10Buffer>, ::windows::core::GetTrustLevel, Map::<Impl, OFFSET>, Unmap::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10CounterImpl: Sized + ID3D10AsynchronousImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10Counter {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Counter";
}
impl ID3D10CounterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10CounterImpl, const OFFSET: isize>() -> ID3D10CounterVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10CounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_COUNTER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10Counter>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10DebugImpl: Sized {
    fn SetFeatureMask();
    fn GetFeatureMask();
    fn SetPresentPerRenderOpDelay();
    fn GetPresentPerRenderOpDelay();
    fn SetSwapChain();
    fn GetSwapChain();
    fn Validate();
}
impl ::windows::core::RuntimeName for ID3D10Debug {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Debug";
}
impl ID3D10DebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DebugImpl, const OFFSET: isize>() -> ID3D10DebugVtbl {
        unsafe extern "system" fn SetFeatureMask<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFeatureMask(mask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureMask<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeatureMask() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPresentPerRenderOpDelay(milliseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPresentPerRenderOpDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSwapChain<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pswapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSwapChain(&*(&pswapchain as *const <super::Dxgi::IDXGISwapChain as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGISwapChain as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSwapChain<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSwapChain(::core::mem::transmute_copy(&ppswapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Validate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10Debug>,
            ::windows::core::GetTrustLevel,
            SetFeatureMask::<Impl, OFFSET>,
            GetFeatureMask::<Impl, OFFSET>,
            SetPresentPerRenderOpDelay::<Impl, OFFSET>,
            GetPresentPerRenderOpDelay::<Impl, OFFSET>,
            SetSwapChain::<Impl, OFFSET>,
            GetSwapChain::<Impl, OFFSET>,
            Validate::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D10DepthStencilStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10DepthStencilState {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10DepthStencilState";
}
impl ID3D10DepthStencilStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DepthStencilStateImpl, const OFFSET: isize>() -> ID3D10DepthStencilStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10DepthStencilStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10DepthStencilState>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10DepthStencilViewImpl: Sized + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10DepthStencilView {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10DepthStencilView";
}
impl ID3D10DepthStencilViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DepthStencilViewImpl, const OFFSET: isize>() -> ID3D10DepthStencilViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10DepthStencilViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10DepthStencilView>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D10Device {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Device";
}
impl ID3D10DeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DeviceImpl, const OFFSET: isize>() -> ID3D10DeviceVtbl {
        unsafe extern "system" fn VSSetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetConstantBuffers(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D10Buffer as ::windows::core::Abi>::Abi as *const <ID3D10Buffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PSSetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetShaderResources(startslot, numviews, &*(&ppshaderresourceviews as *const <ID3D10ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D10ShaderResourceView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PSSetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelshader: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetShader(&*(&ppixelshader as *const <ID3D10PixelShader as ::windows::core::Abi>::Abi as *const <ID3D10PixelShader as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PSSetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetSamplers(startslot, numsamplers, &*(&ppsamplers as *const <ID3D10SamplerState as ::windows::core::Abi>::Abi as *const <ID3D10SamplerState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VSSetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvertexshader: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetShader(&*(&pvertexshader as *const <ID3D10VertexShader as ::windows::core::Abi>::Abi as *const <ID3D10VertexShader as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawIndexed<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexed(indexcount, startindexlocation, basevertexlocation).into()
        }
        unsafe extern "system" fn Draw<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Draw(vertexcount, startvertexlocation).into()
        }
        unsafe extern "system" fn PSSetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetConstantBuffers(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D10Buffer as ::windows::core::Abi>::Abi as *const <ID3D10Buffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IASetInputLayout<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputlayout: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetInputLayout(&*(&pinputlayout as *const <ID3D10InputLayout as ::windows::core::Abi>::Abi as *const <ID3D10InputLayout as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IASetVertexBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::windows::core::RawPtr, pstrides: *const u32, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetVertexBuffers(startslot, numbuffers, &*(&ppvertexbuffers as *const <ID3D10Buffer as ::windows::core::Abi>::Abi as *const <ID3D10Buffer as ::windows::core::DefaultType>::DefaultType), pstrides, poffsets).into()
        }
        unsafe extern "system" fn IASetIndexBuffer<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: ::windows::core::RawPtr, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetIndexBuffer(&*(&pindexbuffer as *const <ID3D10Buffer as ::windows::core::Abi>::Abi as *const <ID3D10Buffer as ::windows::core::DefaultType>::DefaultType), format, offset).into()
        }
        unsafe extern "system" fn DrawIndexedInstanced<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexedInstanced(indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation).into()
        }
        unsafe extern "system" fn DrawInstanced<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawInstanced(vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation).into()
        }
        unsafe extern "system" fn GSSetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetConstantBuffers(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D10Buffer as ::windows::core::Abi>::Abi as *const <ID3D10Buffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GSSetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetShader(&*(&pshader as *const <ID3D10GeometryShader as ::windows::core::Abi>::Abi as *const <ID3D10GeometryShader as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetPrimitiveTopology(topology).into()
        }
        unsafe extern "system" fn VSSetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetShaderResources(startslot, numviews, &*(&ppshaderresourceviews as *const <ID3D10ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D10ShaderResourceView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VSSetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetSamplers(startslot, numsamplers, &*(&ppsamplers as *const <ID3D10SamplerState as ::windows::core::Abi>::Abi as *const <ID3D10SamplerState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetPredication<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicate: ::windows::core::RawPtr, predicatevalue: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPredication(&*(&ppredicate as *const <ID3D10Predicate as ::windows::core::Abi>::Abi as *const <ID3D10Predicate as ::windows::core::DefaultType>::DefaultType), &*(&predicatevalue as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GSSetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetShaderResources(startslot, numviews, &*(&ppshaderresourceviews as *const <ID3D10ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D10ShaderResourceView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GSSetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetSamplers(startslot, numsamplers, &*(&ppsamplers as *const <ID3D10SamplerState as ::windows::core::Abi>::Abi as *const <ID3D10SamplerState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OMSetRenderTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetRenderTargets(numviews, &*(&pprendertargetviews as *const <ID3D10RenderTargetView as ::windows::core::Abi>::Abi as *const <ID3D10RenderTargetView as ::windows::core::DefaultType>::DefaultType), &*(&pdepthstencilview as *const <ID3D10DepthStencilView as ::windows::core::Abi>::Abi as *const <ID3D10DepthStencilView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OMSetBlendState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstate: ::windows::core::RawPtr, blendfactor: *const f32, samplemask: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetBlendState(&*(&pblendstate as *const <ID3D10BlendState as ::windows::core::Abi>::Abi as *const <ID3D10BlendState as ::windows::core::DefaultType>::DefaultType), blendfactor, samplemask).into()
        }
        unsafe extern "system" fn OMSetDepthStencilState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilstate: ::windows::core::RawPtr, stencilref: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetDepthStencilState(&*(&pdepthstencilstate as *const <ID3D10DepthStencilState as ::windows::core::Abi>::Abi as *const <ID3D10DepthStencilState as ::windows::core::DefaultType>::DefaultType), stencilref).into()
        }
        unsafe extern "system" fn SOSetTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const ::windows::core::RawPtr, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SOSetTargets(numbuffers, &*(&ppsotargets as *const <ID3D10Buffer as ::windows::core::Abi>::Abi as *const <ID3D10Buffer as ::windows::core::DefaultType>::DefaultType), poffsets).into()
        }
        unsafe extern "system" fn DrawAuto<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawAuto().into()
        }
        unsafe extern "system" fn RSSetState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerstate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetState(&*(&prasterizerstate as *const <ID3D10RasterizerState as ::windows::core::Abi>::Abi as *const <ID3D10RasterizerState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RSSetViewports<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D10_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetViewports(numviewports, &*(&pviewports as *const <D3D10_VIEWPORT as ::windows::core::Abi>::Abi as *const <D3D10_VIEWPORT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RSSetScissorRects<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetScissorRects(numrects, &*(&prects as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopySubresourceRegion<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D10_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CopySubresourceRegion(&*(&pdstresource as *const <ID3D10Resource as ::windows::core::Abi>::Abi as *const <ID3D10Resource as ::windows::core::DefaultType>::DefaultType), dstsubresource, dstx, dsty, dstz, &*(&psrcresource as *const <ID3D10Resource as ::windows::core::Abi>::Abi as *const <ID3D10Resource as ::windows::core::DefaultType>::DefaultType), srcsubresource, &*(&psrcbox as *const <D3D10_BOX as ::windows::core::Abi>::Abi as *const <D3D10_BOX as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn CopyResource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyResource(&*(&pdstresource as *const <ID3D10Resource as ::windows::core::Abi>::Abi as *const <ID3D10Resource as ::windows::core::DefaultType>::DefaultType), &*(&psrcresource as *const <ID3D10Resource as ::windows::core::Abi>::Abi as *const <ID3D10Resource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateSubresource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .UpdateSubresource(&*(&pdstresource as *const <ID3D10Resource as ::windows::core::Abi>::Abi as *const <ID3D10Resource as ::windows::core::DefaultType>::DefaultType), dstsubresource, &*(&pdstbox as *const <D3D10_BOX as ::windows::core::Abi>::Abi as *const <D3D10_BOX as ::windows::core::DefaultType>::DefaultType), &*(&psrcdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), srcrowpitch, srcdepthpitch)
                .into()
        }
        unsafe extern "system" fn ClearRenderTargetView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertargetview: ::windows::core::RawPtr, colorrgba: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRenderTargetView(&*(&prendertargetview as *const <ID3D10RenderTargetView as ::windows::core::Abi>::Abi as *const <ID3D10RenderTargetView as ::windows::core::DefaultType>::DefaultType), colorrgba).into()
        }
        unsafe extern "system" fn ClearDepthStencilView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilview: ::windows::core::RawPtr, clearflags: u32, depth: f32, stencil: u8) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearDepthStencilView(&*(&pdepthstencilview as *const <ID3D10DepthStencilView as ::windows::core::Abi>::Abi as *const <ID3D10DepthStencilView as ::windows::core::DefaultType>::DefaultType), clearflags, depth, stencil).into()
        }
        unsafe extern "system" fn GenerateMips<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderresourceview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateMips(&*(&pshaderresourceview as *const <ID3D10ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D10ShaderResourceView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResolveSubresource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveSubresource(&*(&pdstresource as *const <ID3D10Resource as ::windows::core::Abi>::Abi as *const <ID3D10Resource as ::windows::core::DefaultType>::DefaultType), dstsubresource, &*(&psrcresource as *const <ID3D10Resource as ::windows::core::Abi>::Abi as *const <ID3D10Resource as ::windows::core::DefaultType>::DefaultType), srcsubresource, format).into()
        }
        unsafe extern "system" fn VSGetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetConstantBuffers(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers)).into()
        }
        unsafe extern "system" fn PSGetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetShaderResources(startslot, numviews, ::core::mem::transmute_copy(&ppshaderresourceviews)).into()
        }
        unsafe extern "system" fn PSGetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppixelshader: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetShader(::core::mem::transmute_copy(&pppixelshader)).into()
        }
        unsafe extern "system" fn PSGetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetSamplers(startslot, numsamplers, ::core::mem::transmute_copy(&ppsamplers)).into()
        }
        unsafe extern "system" fn VSGetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvertexshader: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetShader(::core::mem::transmute_copy(&ppvertexshader)).into()
        }
        unsafe extern "system" fn PSGetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetConstantBuffers(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers)).into()
        }
        unsafe extern "system" fn IAGetInputLayout<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinputlayout: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetInputLayout(::core::mem::transmute_copy(&ppinputlayout)).into()
        }
        unsafe extern "system" fn IAGetVertexBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::windows::core::RawPtr, pstrides: *mut u32, poffsets: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetVertexBuffers(startslot, numbuffers, ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets)).into()
        }
        unsafe extern "system" fn IAGetIndexBuffer<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::windows::core::RawPtr, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetIndexBuffer(::core::mem::transmute_copy(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn GSGetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetConstantBuffers(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers)).into()
        }
        unsafe extern "system" fn GSGetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetShader(::core::mem::transmute_copy(&ppgeometryshader)).into()
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetPrimitiveTopology(::core::mem::transmute_copy(&ptopology)).into()
        }
        unsafe extern "system" fn VSGetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetShaderResources(startslot, numviews, ::core::mem::transmute_copy(&ppshaderresourceviews)).into()
        }
        unsafe extern "system" fn VSGetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetSamplers(startslot, numsamplers, ::core::mem::transmute_copy(&ppsamplers)).into()
        }
        unsafe extern "system" fn GetPredication<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppredicate: *mut ::windows::core::RawPtr, ppredicatevalue: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPredication(::core::mem::transmute_copy(&pppredicate), ::core::mem::transmute_copy(&ppredicatevalue)).into()
        }
        unsafe extern "system" fn GSGetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetShaderResources(startslot, numviews, ::core::mem::transmute_copy(&ppshaderresourceviews)).into()
        }
        unsafe extern "system" fn GSGetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetSamplers(startslot, numsamplers, ::core::mem::transmute_copy(&ppsamplers)).into()
        }
        unsafe extern "system" fn OMGetRenderTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetRenderTargets(numviews, ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview)).into()
        }
        unsafe extern "system" fn OMGetBlendState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblendstate: *mut ::windows::core::RawPtr, blendfactor: *mut f32, psamplemask: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetBlendState(::core::mem::transmute_copy(&ppblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&psamplemask)).into()
        }
        unsafe extern "system" fn OMGetDepthStencilState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut ::windows::core::RawPtr, pstencilref: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetDepthStencilState(::core::mem::transmute_copy(&ppdepthstencilstate), ::core::mem::transmute_copy(&pstencilref)).into()
        }
        unsafe extern "system" fn SOGetTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut ::windows::core::RawPtr, poffsets: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SOGetTargets(numbuffers, ::core::mem::transmute_copy(&ppsotargets), ::core::mem::transmute_copy(&poffsets)).into()
        }
        unsafe extern "system" fn RSGetState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetState(::core::mem::transmute_copy(&pprasterizerstate)).into()
        }
        unsafe extern "system" fn RSGetViewports<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetViewports(numviewports, ::core::mem::transmute_copy(&pviewports)).into()
        }
        unsafe extern "system" fn RSGetScissorRects<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetScissorRects(numrects, ::core::mem::transmute_copy(&prects)).into()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceRemovedReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExceptionMode<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExceptionMode(raiseflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExceptionMode<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExceptionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: &::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pdatasize, ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: &::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), datasize, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: &::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateDataInterface(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearState().into()
        }
        unsafe extern "system" fn Flush<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        unsafe extern "system" fn CreateBuffer<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBuffer(&*(&pdesc as *const <D3D10_BUFFER_DESC as ::windows::core::Abi>::Abi as *const <D3D10_BUFFER_DESC as ::windows::core::DefaultType>::DefaultType), &*(&pinitialdata as *const <D3D10_SUBRESOURCE_DATA as ::windows::core::Abi>::Abi as *const <D3D10_SUBRESOURCE_DATA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture1D<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture1d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture1D(&*(&pdesc as *const <D3D10_TEXTURE1D_DESC as ::windows::core::Abi>::Abi as *const <D3D10_TEXTURE1D_DESC as ::windows::core::DefaultType>::DefaultType), &*(&pinitialdata as *const <D3D10_SUBRESOURCE_DATA as ::windows::core::Abi>::Abi as *const <D3D10_SUBRESOURCE_DATA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptexture1d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture2D<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture2D(&*(&pdesc as *const <D3D10_TEXTURE2D_DESC as ::windows::core::Abi>::Abi as *const <D3D10_TEXTURE2D_DESC as ::windows::core::DefaultType>::DefaultType), &*(&pinitialdata as *const <D3D10_SUBRESOURCE_DATA as ::windows::core::Abi>::Abi as *const <D3D10_SUBRESOURCE_DATA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptexture2d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture3D<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture3D(&*(&pdesc as *const <D3D10_TEXTURE3D_DESC as ::windows::core::Abi>::Abi as *const <D3D10_TEXTURE3D_DESC as ::windows::core::DefaultType>::DefaultType), &*(&pinitialdata as *const <D3D10_SUBRESOURCE_DATA as ::windows::core::Abi>::Abi as *const <D3D10_SUBRESOURCE_DATA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptexture3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShaderResourceView(&*(&presource as *const <ID3D10Resource as ::windows::core::Abi>::Abi as *const <ID3D10Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc as *const <D3D10_SHADER_RESOURCE_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D10_SHADER_RESOURCE_VIEW_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsrview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderTargetView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRenderTargetView(&*(&presource as *const <ID3D10Resource as ::windows::core::Abi>::Abi as *const <ID3D10Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc as *const <D3D10_RENDER_TARGET_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D10_RENDER_TARGET_VIEW_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprtview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDepthStencilView(&*(&presource as *const <ID3D10Resource as ::windows::core::Abi>::Abi as *const <ID3D10Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc as *const <D3D10_DEPTH_STENCIL_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D10_DEPTH_STENCIL_VIEW_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdepthstencilview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInputLayout<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInputLayout(&*(&pinputelementdescs as *const <D3D10_INPUT_ELEMENT_DESC as ::windows::core::Abi>::Abi as *const <D3D10_INPUT_ELEMENT_DESC as ::windows::core::DefaultType>::DefaultType), numelements, &*(&pshaderbytecodewithinputsignature as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, ::core::mem::transmute_copy(&ppinputlayout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVertexShader(&*(&pshaderbytecode as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, ::core::mem::transmute_copy(&ppvertexshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometryShader(&*(&pshaderbytecode as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, ::core::mem::transmute_copy(&ppgeometryshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometryShaderWithStreamOutput(&*(&pshaderbytecode as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, &*(&psodeclaration as *const <D3D10_SO_DECLARATION_ENTRY as ::windows::core::Abi>::Abi as *const <D3D10_SO_DECLARATION_ENTRY as ::windows::core::DefaultType>::DefaultType), numentries, outputstreamstride, ::core::mem::transmute_copy(&ppgeometryshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePixelShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePixelShader(&*(&pshaderbytecode as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, ::core::mem::transmute_copy(&pppixelshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlendState(&*(&pblendstatedesc as *const <D3D10_BLEND_DESC as ::windows::core::Abi>::Abi as *const <D3D10_BLEND_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppblendstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDepthStencilState(&*(&pdepthstencildesc as *const <D3D10_DEPTH_STENCIL_DESC as ::windows::core::Abi>::Abi as *const <D3D10_DEPTH_STENCIL_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdepthstencilstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRasterizerState(&*(&prasterizerdesc as *const <D3D10_RASTERIZER_DESC as ::windows::core::Abi>::Abi as *const <D3D10_RASTERIZER_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprasterizerstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSamplerState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSamplerState(&*(&psamplerdesc as *const <D3D10_SAMPLER_DESC as ::windows::core::Abi>::Abi as *const <D3D10_SAMPLER_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsamplerstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuery<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuery(&*(&pquerydesc as *const <D3D10_QUERY_DESC as ::windows::core::Abi>::Abi as *const <D3D10_QUERY_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePredicate<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePredicate(&*(&ppredicatedesc as *const <D3D10_QUERY_DESC as ::windows::core::Abi>::Abi as *const <D3D10_QUERY_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppredicate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCounter<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCounter(&*(&pcounterdesc as *const <D3D10_COUNTER_DESC as ::windows::core::Abi>::Abi as *const <D3D10_COUNTER_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcounter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckFormatSupport<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckFormatSupport(format, ::core::mem::transmute_copy(&pformatsupport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckMultisampleQualityLevels(format, samplecount, ::core::mem::transmute_copy(&pnumqualitylevels)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCounterInfo<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D10_COUNTER_INFO) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckCounterInfo(::core::mem::transmute_copy(&pcounterinfo)).into()
        }
        unsafe extern "system" fn CheckCounter<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckCounter(&*(&pdesc as *const <D3D10_COUNTER_DESC as ::windows::core::Abi>::Abi as *const <D3D10_COUNTER_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pactivecounters), ::core::mem::transmute_copy(&szname), pnamelength, ::core::mem::transmute_copy(&szunits), punitslength, ::core::mem::transmute_copy(&szdescription), pdescriptionlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCreationFlags<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreationFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedResource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: &::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenSharedResource(&*(&hresource as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&returnedinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextFilterSize<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextFilterSize(width, height).into()
        }
        unsafe extern "system" fn GetTextFilterSize<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextFilterSize(::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10Device>,
            ::windows::core::GetTrustLevel,
            VSSetConstantBuffers::<Impl, OFFSET>,
            PSSetShaderResources::<Impl, OFFSET>,
            PSSetShader::<Impl, OFFSET>,
            PSSetSamplers::<Impl, OFFSET>,
            VSSetShader::<Impl, OFFSET>,
            DrawIndexed::<Impl, OFFSET>,
            Draw::<Impl, OFFSET>,
            PSSetConstantBuffers::<Impl, OFFSET>,
            IASetInputLayout::<Impl, OFFSET>,
            IASetVertexBuffers::<Impl, OFFSET>,
            IASetIndexBuffer::<Impl, OFFSET>,
            DrawIndexedInstanced::<Impl, OFFSET>,
            DrawInstanced::<Impl, OFFSET>,
            GSSetConstantBuffers::<Impl, OFFSET>,
            GSSetShader::<Impl, OFFSET>,
            IASetPrimitiveTopology::<Impl, OFFSET>,
            VSSetShaderResources::<Impl, OFFSET>,
            VSSetSamplers::<Impl, OFFSET>,
            SetPredication::<Impl, OFFSET>,
            GSSetShaderResources::<Impl, OFFSET>,
            GSSetSamplers::<Impl, OFFSET>,
            OMSetRenderTargets::<Impl, OFFSET>,
            OMSetBlendState::<Impl, OFFSET>,
            OMSetDepthStencilState::<Impl, OFFSET>,
            SOSetTargets::<Impl, OFFSET>,
            DrawAuto::<Impl, OFFSET>,
            RSSetState::<Impl, OFFSET>,
            RSSetViewports::<Impl, OFFSET>,
            RSSetScissorRects::<Impl, OFFSET>,
            CopySubresourceRegion::<Impl, OFFSET>,
            CopyResource::<Impl, OFFSET>,
            UpdateSubresource::<Impl, OFFSET>,
            ClearRenderTargetView::<Impl, OFFSET>,
            ClearDepthStencilView::<Impl, OFFSET>,
            GenerateMips::<Impl, OFFSET>,
            ResolveSubresource::<Impl, OFFSET>,
            VSGetConstantBuffers::<Impl, OFFSET>,
            PSGetShaderResources::<Impl, OFFSET>,
            PSGetShader::<Impl, OFFSET>,
            PSGetSamplers::<Impl, OFFSET>,
            VSGetShader::<Impl, OFFSET>,
            PSGetConstantBuffers::<Impl, OFFSET>,
            IAGetInputLayout::<Impl, OFFSET>,
            IAGetVertexBuffers::<Impl, OFFSET>,
            IAGetIndexBuffer::<Impl, OFFSET>,
            GSGetConstantBuffers::<Impl, OFFSET>,
            GSGetShader::<Impl, OFFSET>,
            IAGetPrimitiveTopology::<Impl, OFFSET>,
            VSGetShaderResources::<Impl, OFFSET>,
            VSGetSamplers::<Impl, OFFSET>,
            GetPredication::<Impl, OFFSET>,
            GSGetShaderResources::<Impl, OFFSET>,
            GSGetSamplers::<Impl, OFFSET>,
            OMGetRenderTargets::<Impl, OFFSET>,
            OMGetBlendState::<Impl, OFFSET>,
            OMGetDepthStencilState::<Impl, OFFSET>,
            SOGetTargets::<Impl, OFFSET>,
            RSGetState::<Impl, OFFSET>,
            RSGetViewports::<Impl, OFFSET>,
            RSGetScissorRects::<Impl, OFFSET>,
            GetDeviceRemovedReason::<Impl, OFFSET>,
            SetExceptionMode::<Impl, OFFSET>,
            GetExceptionMode::<Impl, OFFSET>,
            GetPrivateData::<Impl, OFFSET>,
            SetPrivateData::<Impl, OFFSET>,
            SetPrivateDataInterface::<Impl, OFFSET>,
            ClearState::<Impl, OFFSET>,
            Flush::<Impl, OFFSET>,
            CreateBuffer::<Impl, OFFSET>,
            CreateTexture1D::<Impl, OFFSET>,
            CreateTexture2D::<Impl, OFFSET>,
            CreateTexture3D::<Impl, OFFSET>,
            CreateShaderResourceView::<Impl, OFFSET>,
            CreateRenderTargetView::<Impl, OFFSET>,
            CreateDepthStencilView::<Impl, OFFSET>,
            CreateInputLayout::<Impl, OFFSET>,
            CreateVertexShader::<Impl, OFFSET>,
            CreateGeometryShader::<Impl, OFFSET>,
            CreateGeometryShaderWithStreamOutput::<Impl, OFFSET>,
            CreatePixelShader::<Impl, OFFSET>,
            CreateBlendState::<Impl, OFFSET>,
            CreateDepthStencilState::<Impl, OFFSET>,
            CreateRasterizerState::<Impl, OFFSET>,
            CreateSamplerState::<Impl, OFFSET>,
            CreateQuery::<Impl, OFFSET>,
            CreatePredicate::<Impl, OFFSET>,
            CreateCounter::<Impl, OFFSET>,
            CheckFormatSupport::<Impl, OFFSET>,
            CheckMultisampleQualityLevels::<Impl, OFFSET>,
            CheckCounterInfo::<Impl, OFFSET>,
            CheckCounter::<Impl, OFFSET>,
            GetCreationFlags::<Impl, OFFSET>,
            OpenSharedResource::<Impl, OFFSET>,
            SetTextFilterSize::<Impl, OFFSET>,
            GetTextFilterSize::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D10Device1Impl: Sized + ID3D10DeviceImpl {
    fn CreateShaderResourceView1();
    fn CreateBlendState1();
    fn GetFeatureLevel();
}
impl ::windows::core::RuntimeName for ID3D10Device1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Device1";
}
impl ID3D10Device1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Device1Impl, const OFFSET: isize>() -> ID3D10Device1Vtbl {
        unsafe extern "system" fn CreateShaderResourceView1<Impl: ID3D10Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShaderResourceView1(&*(&presource as *const <ID3D10Resource as ::windows::core::Abi>::Abi as *const <ID3D10Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc as *const <D3D10_SHADER_RESOURCE_VIEW_DESC1 as ::windows::core::Abi>::Abi as *const <D3D10_SHADER_RESOURCE_VIEW_DESC1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsrview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState1<Impl: ID3D10Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlendState1(&*(&pblendstatedesc as *const <D3D10_BLEND_DESC1 as ::windows::core::Abi>::Abi as *const <D3D10_BLEND_DESC1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppblendstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureLevel<Impl: ID3D10Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D10_FEATURE_LEVEL1 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeatureLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10Device1>, ::windows::core::GetTrustLevel, CreateShaderResourceView1::<Impl, OFFSET>, CreateBlendState1::<Impl, OFFSET>, GetFeatureLevel::<Impl, OFFSET>)
    }
}
pub trait ID3D10DeviceChildImpl: Sized {
    fn GetDevice();
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
}
impl ::windows::core::RuntimeName for ID3D10DeviceChild {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10DeviceChild";
}
impl ID3D10DeviceChildVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DeviceChildImpl, const OFFSET: isize>() -> ID3D10DeviceChildVtbl {
        unsafe extern "system" fn GetDevice<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)).into()
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: &::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pdatasize, ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: &::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), datasize, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: &::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateDataInterface(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10DeviceChild>, ::windows::core::GetTrustLevel, GetDevice::<Impl, OFFSET>, GetPrivateData::<Impl, OFFSET>, SetPrivateData::<Impl, OFFSET>, SetPrivateDataInterface::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D10Effect {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Effect";
}
impl ID3D10EffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectImpl, const OFFSET: isize>() -> ID3D10EffectVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPool<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPool() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableBySemantic<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableBySemantic(&*(&semantic as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTechniqueByIndex<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectTechnique> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTechniqueByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTechniqueByName<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectTechnique> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTechniqueByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Optimize<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Optimize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptimized<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOptimized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10Effect>,
            ::windows::core::GetTrustLevel,
            IsValid::<Impl, OFFSET>,
            IsPool::<Impl, OFFSET>,
            GetDevice::<Impl, OFFSET>,
            GetDesc::<Impl, OFFSET>,
            GetConstantBufferByIndex::<Impl, OFFSET>,
            GetConstantBufferByName::<Impl, OFFSET>,
            GetVariableByIndex::<Impl, OFFSET>,
            GetVariableByName::<Impl, OFFSET>,
            GetVariableBySemantic::<Impl, OFFSET>,
            GetTechniqueByIndex::<Impl, OFFSET>,
            GetTechniqueByName::<Impl, OFFSET>,
            Optimize::<Impl, OFFSET>,
            IsOptimized::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D10EffectBlendVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetBlendState();
    fn GetBackingStore();
}
impl ::windows::core::RuntimeName for ID3D10EffectBlendVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectBlendVariable";
}
impl ID3D10EffectBlendVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectBlendVariableImpl, const OFFSET: isize>() -> ID3D10EffectBlendVariableVtbl {
        unsafe extern "system" fn GetBlendState<Impl: ID3D10EffectBlendVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlendState(index, ::core::mem::transmute_copy(&ppblendstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectBlendVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackingStore(index, &*(&pblenddesc as *const <D3D10_BLEND_DESC as ::windows::core::Abi>::Abi as *const <D3D10_BLEND_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10EffectBlendVariable>, ::windows::core::GetTrustLevel, GetBlendState::<Impl, OFFSET>, GetBackingStore::<Impl, OFFSET>)
    }
}
pub trait ID3D10EffectConstantBufferImpl: Sized + ID3D10EffectVariableImpl {
    fn SetConstantBuffer();
    fn GetConstantBuffer();
    fn SetTextureBuffer();
    fn GetTextureBuffer();
}
impl ::windows::core::RuntimeName for ID3D10EffectConstantBuffer {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectConstantBuffer";
}
impl ID3D10EffectConstantBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>() -> ID3D10EffectConstantBufferVtbl {
        unsafe extern "system" fn SetConstantBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstantbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConstantBuffer(&*(&pconstantbuffer as *const <ID3D10Buffer as ::windows::core::Abi>::Abi as *const <ID3D10Buffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconstantbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBuffer(::core::mem::transmute_copy(&ppconstantbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextureBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptexturebuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTextureBuffer(&*(&ptexturebuffer as *const <ID3D10ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D10ShaderResourceView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextureBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptexturebuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextureBuffer(::core::mem::transmute_copy(&pptexturebuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10EffectConstantBuffer>, ::windows::core::GetTrustLevel, SetConstantBuffer::<Impl, OFFSET>, GetConstantBuffer::<Impl, OFFSET>, SetTextureBuffer::<Impl, OFFSET>, GetTextureBuffer::<Impl, OFFSET>)
    }
}
pub trait ID3D10EffectDepthStencilVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetDepthStencilState();
    fn GetBackingStore();
}
impl ::windows::core::RuntimeName for ID3D10EffectDepthStencilVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectDepthStencilVariable";
}
impl ID3D10EffectDepthStencilVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectDepthStencilVariableImpl, const OFFSET: isize>() -> ID3D10EffectDepthStencilVariableVtbl {
        unsafe extern "system" fn GetDepthStencilState<Impl: ID3D10EffectDepthStencilVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDepthStencilState(index, ::core::mem::transmute_copy(&ppdepthstencilstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectDepthStencilVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackingStore(index, ::core::mem::transmute_copy(&pdepthstencildesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10EffectDepthStencilVariable>, ::windows::core::GetTrustLevel, GetDepthStencilState::<Impl, OFFSET>, GetBackingStore::<Impl, OFFSET>)
    }
}
pub trait ID3D10EffectDepthStencilViewVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetDepthStencil();
    fn GetDepthStencil();
    fn SetDepthStencilArray();
    fn GetDepthStencilArray();
}
impl ::windows::core::RuntimeName for ID3D10EffectDepthStencilViewVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectDepthStencilViewVariable";
}
impl ID3D10EffectDepthStencilViewVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>() -> ID3D10EffectDepthStencilViewVariableVtbl {
        unsafe extern "system" fn SetDepthStencil<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDepthStencil(&*(&presource as *const <ID3D10DepthStencilView as ::windows::core::Abi>::Abi as *const <ID3D10DepthStencilView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDepthStencil<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDepthStencil(::core::mem::transmute_copy(&ppresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepthStencilArray<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDepthStencilArray(&*(&ppresources as *const <ID3D10DepthStencilView as ::windows::core::Abi>::Abi as *const <ID3D10DepthStencilView as ::windows::core::DefaultType>::DefaultType), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDepthStencilArray<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDepthStencilArray(::core::mem::transmute_copy(&ppresources), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10EffectDepthStencilViewVariable>, ::windows::core::GetTrustLevel, SetDepthStencil::<Impl, OFFSET>, GetDepthStencil::<Impl, OFFSET>, SetDepthStencilArray::<Impl, OFFSET>, GetDepthStencilArray::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D10EffectMatrixVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectMatrixVariable";
}
impl ID3D10EffectMatrixVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>() -> ID3D10EffectMatrixVariableVtbl {
        unsafe extern "system" fn SetMatrix<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrix(pdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatrix<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatrix(pdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrixArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrixArray(pdata, offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatrixArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatrixArray(pdata, offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrixTranspose<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrixTranspose(pdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatrixTranspose<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatrixTranspose(pdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrixTransposeArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrixTransposeArray(pdata, offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatrixTransposeArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatrixTransposeArray(pdata, offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10EffectMatrixVariable>,
            ::windows::core::GetTrustLevel,
            SetMatrix::<Impl, OFFSET>,
            GetMatrix::<Impl, OFFSET>,
            SetMatrixArray::<Impl, OFFSET>,
            GetMatrixArray::<Impl, OFFSET>,
            SetMatrixTranspose::<Impl, OFFSET>,
            GetMatrixTranspose::<Impl, OFFSET>,
            SetMatrixTransposeArray::<Impl, OFFSET>,
            GetMatrixTransposeArray::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for ID3D10EffectPass {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectPass";
}
impl ID3D10EffectPassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectPassImpl, const OFFSET: isize>() -> ID3D10EffectPassVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(&*(&pdesc as *const <D3D10_PASS_DESC as ::windows::core::Abi>::Abi as *const <D3D10_PASS_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVertexShaderDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVertexShaderDesc(&*(&pdesc as *const <D3D10_PASS_SHADER_DESC as ::windows::core::Abi>::Abi as *const <D3D10_PASS_SHADER_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryShaderDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeometryShaderDesc(&*(&pdesc as *const <D3D10_PASS_SHADER_DESC as ::windows::core::Abi>::Abi as *const <D3D10_PASS_SHADER_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelShaderDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelShaderDesc(&*(&pdesc as *const <D3D10_PASS_SHADER_DESC as ::windows::core::Abi>::Abi as *const <D3D10_PASS_SHADER_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Apply<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Apply(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeStateBlockMask<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputeStateBlockMask(::core::mem::transmute_copy(&pstateblockmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10EffectPass>,
            ::windows::core::GetTrustLevel,
            IsValid::<Impl, OFFSET>,
            GetDesc::<Impl, OFFSET>,
            GetVertexShaderDesc::<Impl, OFFSET>,
            GetGeometryShaderDesc::<Impl, OFFSET>,
            GetPixelShaderDesc::<Impl, OFFSET>,
            GetAnnotationByIndex::<Impl, OFFSET>,
            GetAnnotationByName::<Impl, OFFSET>,
            Apply::<Impl, OFFSET>,
            ComputeStateBlockMask::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D10EffectPoolImpl: Sized {
    fn AsEffect();
}
impl ::windows::core::RuntimeName for ID3D10EffectPool {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectPool";
}
impl ID3D10EffectPoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectPoolImpl, const OFFSET: isize>() -> ID3D10EffectPoolVtbl {
        unsafe extern "system" fn AsEffect<Impl: ID3D10EffectPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10Effect> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10EffectPool>, ::windows::core::GetTrustLevel, AsEffect::<Impl, OFFSET>)
    }
}
pub trait ID3D10EffectRasterizerVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetRasterizerState();
    fn GetBackingStore();
}
impl ::windows::core::RuntimeName for ID3D10EffectRasterizerVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectRasterizerVariable";
}
impl ID3D10EffectRasterizerVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectRasterizerVariableImpl, const OFFSET: isize>() -> ID3D10EffectRasterizerVariableVtbl {
        unsafe extern "system" fn GetRasterizerState<Impl: ID3D10EffectRasterizerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRasterizerState(index, ::core::mem::transmute_copy(&pprasterizerstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectRasterizerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackingStore(index, ::core::mem::transmute_copy(&prasterizerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10EffectRasterizerVariable>, ::windows::core::GetTrustLevel, GetRasterizerState::<Impl, OFFSET>, GetBackingStore::<Impl, OFFSET>)
    }
}
pub trait ID3D10EffectRenderTargetViewVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetRenderTarget();
    fn GetRenderTarget();
    fn SetRenderTargetArray();
    fn GetRenderTargetArray();
}
impl ::windows::core::RuntimeName for ID3D10EffectRenderTargetViewVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectRenderTargetViewVariable";
}
impl ID3D10EffectRenderTargetViewVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>() -> ID3D10EffectRenderTargetViewVariableVtbl {
        unsafe extern "system" fn SetRenderTarget<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRenderTarget(&*(&presource as *const <ID3D10RenderTargetView as ::windows::core::Abi>::Abi as *const <ID3D10RenderTargetView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRenderTarget<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRenderTarget(::core::mem::transmute_copy(&ppresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderTargetArray<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRenderTargetArray(&*(&ppresources as *const <ID3D10RenderTargetView as ::windows::core::Abi>::Abi as *const <ID3D10RenderTargetView as ::windows::core::DefaultType>::DefaultType), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRenderTargetArray<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRenderTargetArray(::core::mem::transmute_copy(&ppresources), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10EffectRenderTargetViewVariable>, ::windows::core::GetTrustLevel, SetRenderTarget::<Impl, OFFSET>, GetRenderTarget::<Impl, OFFSET>, SetRenderTargetArray::<Impl, OFFSET>, GetRenderTargetArray::<Impl, OFFSET>)
    }
}
pub trait ID3D10EffectSamplerVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetSampler();
    fn GetBackingStore();
}
impl ::windows::core::RuntimeName for ID3D10EffectSamplerVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectSamplerVariable";
}
impl ID3D10EffectSamplerVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectSamplerVariableImpl, const OFFSET: isize>() -> ID3D10EffectSamplerVariableVtbl {
        unsafe extern "system" fn GetSampler<Impl: ID3D10EffectSamplerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppsampler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSampler(index, ::core::mem::transmute_copy(&ppsampler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectSamplerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackingStore(index, ::core::mem::transmute_copy(&psamplerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10EffectSamplerVariable>, ::windows::core::GetTrustLevel, GetSampler::<Impl, OFFSET>, GetBackingStore::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D10EffectScalarVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectScalarVariable";
}
impl ID3D10EffectScalarVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>() -> ID3D10EffectScalarVariableVtbl {
        unsafe extern "system" fn SetFloat<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFloat(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFloat<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFloat(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFloatArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFloatArray(pdata, offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFloatArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFloatArray(::core::mem::transmute_copy(&pdata), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInt<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInt(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIntArray(pdata, offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntArray(::core::mem::transmute_copy(&pdata), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBool<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBool(&*(&value as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBool<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBool(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBoolArray(&*(&pdata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoolArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoolArray(::core::mem::transmute_copy(&pdata), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10EffectScalarVariable>,
            ::windows::core::GetTrustLevel,
            SetFloat::<Impl, OFFSET>,
            GetFloat::<Impl, OFFSET>,
            SetFloatArray::<Impl, OFFSET>,
            GetFloatArray::<Impl, OFFSET>,
            SetInt::<Impl, OFFSET>,
            GetInt::<Impl, OFFSET>,
            SetIntArray::<Impl, OFFSET>,
            GetIntArray::<Impl, OFFSET>,
            SetBool::<Impl, OFFSET>,
            GetBool::<Impl, OFFSET>,
            SetBoolArray::<Impl, OFFSET>,
            GetBoolArray::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D10EffectShaderResourceVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetResource();
    fn GetResource();
    fn SetResourceArray();
    fn GetResourceArray();
}
impl ::windows::core::RuntimeName for ID3D10EffectShaderResourceVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectShaderResourceVariable";
}
impl ID3D10EffectShaderResourceVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>() -> ID3D10EffectShaderResourceVariableVtbl {
        unsafe extern "system" fn SetResource<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetResource(&*(&presource as *const <ID3D10ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D10ShaderResourceView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResource<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResource(::core::mem::transmute_copy(&ppresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourceArray<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetResourceArray(&*(&ppresources as *const <ID3D10ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D10ShaderResourceView as ::windows::core::DefaultType>::DefaultType), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceArray<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceArray(::core::mem::transmute_copy(&ppresources), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10EffectShaderResourceVariable>, ::windows::core::GetTrustLevel, SetResource::<Impl, OFFSET>, GetResource::<Impl, OFFSET>, SetResourceArray::<Impl, OFFSET>, GetResourceArray::<Impl, OFFSET>)
    }
}
pub trait ID3D10EffectShaderVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetShaderDesc();
    fn GetVertexShader();
    fn GetGeometryShader();
    fn GetPixelShader();
    fn GetInputSignatureElementDesc();
    fn GetOutputSignatureElementDesc();
}
impl ::windows::core::RuntimeName for ID3D10EffectShaderVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectShaderVariable";
}
impl ID3D10EffectShaderVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>() -> ID3D10EffectShaderVariableVtbl {
        unsafe extern "system" fn GetShaderDesc<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShaderDesc(shaderindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVertexShader<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppvs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVertexShader(shaderindex, ::core::mem::transmute_copy(&ppvs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryShader<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppgs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeometryShader(shaderindex, ::core::mem::transmute_copy(&ppgs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelShader<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelShader(shaderindex, ::core::mem::transmute_copy(&ppps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSignatureElementDesc<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputSignatureElementDesc(shaderindex, element, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputSignatureElementDesc<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputSignatureElementDesc(shaderindex, element, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10EffectShaderVariable>,
            ::windows::core::GetTrustLevel,
            GetShaderDesc::<Impl, OFFSET>,
            GetVertexShader::<Impl, OFFSET>,
            GetGeometryShader::<Impl, OFFSET>,
            GetPixelShader::<Impl, OFFSET>,
            GetInputSignatureElementDesc::<Impl, OFFSET>,
            GetOutputSignatureElementDesc::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D10EffectStringVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetString();
    fn GetStringArray();
}
impl ::windows::core::RuntimeName for ID3D10EffectStringVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectStringVariable";
}
impl ID3D10EffectStringVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectStringVariableImpl, const OFFSET: isize>() -> ID3D10EffectStringVariableVtbl {
        unsafe extern "system" fn GetString<Impl: ID3D10EffectStringVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstring: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(::core::mem::transmute_copy(&ppstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringArray<Impl: ID3D10EffectStringVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstrings: *mut super::super::Foundation::PSTR, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringArray(::core::mem::transmute_copy(&ppstrings), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10EffectStringVariable>, ::windows::core::GetTrustLevel, GetString::<Impl, OFFSET>, GetStringArray::<Impl, OFFSET>)
    }
}
pub trait ID3D10EffectTechniqueImpl: Sized {
    fn IsValid();
    fn GetDesc();
    fn GetAnnotationByIndex();
    fn GetAnnotationByName();
    fn GetPassByIndex();
    fn GetPassByName();
    fn ComputeStateBlockMask();
}
impl ::windows::core::RuntimeName for ID3D10EffectTechnique {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectTechnique";
}
impl ID3D10EffectTechniqueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>() -> ID3D10EffectTechniqueVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(&*(&pdesc as *const <D3D10_TECHNIQUE_DESC as ::windows::core::Abi>::Abi as *const <D3D10_TECHNIQUE_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPassByIndex<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectPass> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPassByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPassByName<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectPass> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPassByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeStateBlockMask<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputeStateBlockMask(::core::mem::transmute_copy(&pstateblockmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10EffectTechnique>,
            ::windows::core::GetTrustLevel,
            IsValid::<Impl, OFFSET>,
            GetDesc::<Impl, OFFSET>,
            GetAnnotationByIndex::<Impl, OFFSET>,
            GetAnnotationByName::<Impl, OFFSET>,
            GetPassByIndex::<Impl, OFFSET>,
            GetPassByName::<Impl, OFFSET>,
            ComputeStateBlockMask::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D10EffectTypeImpl: Sized {
    fn IsValid();
    fn GetDesc();
    fn GetMemberTypeByIndex();
    fn GetMemberTypeByName();
    fn GetMemberTypeBySemantic();
    fn GetMemberName();
    fn GetMemberSemantic();
}
impl ::windows::core::RuntimeName for ID3D10EffectType {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectType";
}
impl ID3D10EffectTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectTypeImpl, const OFFSET: isize>() -> ID3D10EffectTypeVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(&*(&pdesc as *const <D3D10_EFFECT_TYPE_DESC as ::windows::core::Abi>::Abi as *const <D3D10_EFFECT_TYPE_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeBySemantic<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeBySemantic(&*(&semantic as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberName<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberName(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberSemantic<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberSemantic(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10EffectType>,
            ::windows::core::GetTrustLevel,
            IsValid::<Impl, OFFSET>,
            GetDesc::<Impl, OFFSET>,
            GetMemberTypeByIndex::<Impl, OFFSET>,
            GetMemberTypeByName::<Impl, OFFSET>,
            GetMemberTypeBySemantic::<Impl, OFFSET>,
            GetMemberName::<Impl, OFFSET>,
            GetMemberSemantic::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for ID3D10EffectVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectVariable";
}
impl ID3D10EffectVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectVariableImpl, const OFFSET: isize>() -> ID3D10EffectVariableVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberByName<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberBySemantic(&*(&semantic as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElement(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentConstantBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsScalar<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsScalar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsVector<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsVector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsMatrix<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsString<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsShaderResource<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsShaderResource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsRenderTargetView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsDepthStencilView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsConstantBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsShader<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsShader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsBlend<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsBlend() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsDepthStencil<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsDepthStencil() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsRasterizer<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsRasterizer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsSampler<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsSampler() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRawValue<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRawValue(&*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), offset, bytecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRawValue<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRawValue(::core::mem::transmute_copy(&pdata), offset, bytecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10EffectVariable>,
            ::windows::core::GetTrustLevel,
            IsValid::<Impl, OFFSET>,
            GetType::<Impl, OFFSET>,
            GetDesc::<Impl, OFFSET>,
            GetAnnotationByIndex::<Impl, OFFSET>,
            GetAnnotationByName::<Impl, OFFSET>,
            GetMemberByIndex::<Impl, OFFSET>,
            GetMemberByName::<Impl, OFFSET>,
            GetMemberBySemantic::<Impl, OFFSET>,
            GetElement::<Impl, OFFSET>,
            GetParentConstantBuffer::<Impl, OFFSET>,
            AsScalar::<Impl, OFFSET>,
            AsVector::<Impl, OFFSET>,
            AsMatrix::<Impl, OFFSET>,
            AsString::<Impl, OFFSET>,
            AsShaderResource::<Impl, OFFSET>,
            AsRenderTargetView::<Impl, OFFSET>,
            AsDepthStencilView::<Impl, OFFSET>,
            AsConstantBuffer::<Impl, OFFSET>,
            AsShader::<Impl, OFFSET>,
            AsBlend::<Impl, OFFSET>,
            AsDepthStencil::<Impl, OFFSET>,
            AsRasterizer::<Impl, OFFSET>,
            AsSampler::<Impl, OFFSET>,
            SetRawValue::<Impl, OFFSET>,
            GetRawValue::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for ID3D10EffectVectorVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10EffectVectorVariable";
}
impl ID3D10EffectVectorVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>() -> ID3D10EffectVectorVariableVtbl {
        unsafe extern "system" fn SetBoolVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBoolVector(&*(&pdata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIntVector(pdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFloatVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFloatVector(pdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoolVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoolVector(&*(&pdata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntVector(pdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFloatVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFloatVector(pdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBoolVectorArray(&*(&pdata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIntVectorArray(pdata, offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFloatVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFloatVectorArray(pdata, offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoolVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoolVectorArray(&*(&pdata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntVectorArray(pdata, offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFloatVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFloatVectorArray(pdata, offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10EffectVectorVariable>,
            ::windows::core::GetTrustLevel,
            SetBoolVector::<Impl, OFFSET>,
            SetIntVector::<Impl, OFFSET>,
            SetFloatVector::<Impl, OFFSET>,
            GetBoolVector::<Impl, OFFSET>,
            GetIntVector::<Impl, OFFSET>,
            GetFloatVector::<Impl, OFFSET>,
            SetBoolVectorArray::<Impl, OFFSET>,
            SetIntVectorArray::<Impl, OFFSET>,
            SetFloatVectorArray::<Impl, OFFSET>,
            GetBoolVectorArray::<Impl, OFFSET>,
            GetIntVectorArray::<Impl, OFFSET>,
            GetFloatVectorArray::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D10GeometryShaderImpl: Sized + ID3D10DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D10GeometryShader {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10GeometryShader";
}
impl ID3D10GeometryShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10GeometryShaderImpl, const OFFSET: isize>() -> ID3D10GeometryShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10GeometryShader>, ::windows::core::GetTrustLevel)
    }
}
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
impl ::windows::core::RuntimeName for ID3D10InfoQueue {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10InfoQueue";
}
impl ID3D10InfoQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10InfoQueueImpl, const OFFSET: isize>() -> ID3D10InfoQueueVtbl {
        unsafe extern "system" fn SetMessageCountLimit<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMessageCountLimit(messagecountlimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStoredMessages<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStoredMessages().into()
        }
        unsafe extern "system" fn GetMessage<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(messageindex, ::core::mem::transmute_copy(&pmessage), pmessagebytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesAllowedByStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesDeniedByStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumStoredMessages<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumStoredMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumStoredMessagesAllowedByRetrievalFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesDiscardedByMessageCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageCountLimit<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStorageFilterEntries<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStorageFilterEntries(&*(&pfilter as *const <D3D10_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D10_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFilter(::core::mem::transmute_copy(&pfilter), pfilterbytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStorageFilter().into()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushEmptyStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushCopyOfStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushStorageFilter(&*(&pfilter as *const <D3D10_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D10_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopStorageFilter().into()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFilterStackSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRetrievalFilterEntries(&*(&pfilter as *const <D3D10_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D10_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRetrievalFilter(::core::mem::transmute_copy(&pfilter), pfilterbytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRetrievalFilter().into()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushEmptyRetrievalFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushCopyOfRetrievalFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushRetrievalFilter(&*(&pfilter as *const <D3D10_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D10_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopRetrievalFilter().into()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRetrievalFilterStackSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMessage<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMessage(category, severity, id, &*(&pdescription as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddApplicationMessage<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddApplicationMessage(severity, &*(&pdescription as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnCategory<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnCategory(category, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnSeverity<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnSeverity(severity, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnID<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnID(id, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnCategory<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnCategory(category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnSeverity<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnSeverity(severity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnID<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnID(id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMuteDebugOutput<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMuteDebugOutput(&*(&bmute as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetMuteDebugOutput<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMuteDebugOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10InfoQueue>,
            ::windows::core::GetTrustLevel,
            SetMessageCountLimit::<Impl, OFFSET>,
            ClearStoredMessages::<Impl, OFFSET>,
            GetMessage::<Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter::<Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter::<Impl, OFFSET>,
            GetNumStoredMessages::<Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter::<Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit::<Impl, OFFSET>,
            GetMessageCountLimit::<Impl, OFFSET>,
            AddStorageFilterEntries::<Impl, OFFSET>,
            GetStorageFilter::<Impl, OFFSET>,
            ClearStorageFilter::<Impl, OFFSET>,
            PushEmptyStorageFilter::<Impl, OFFSET>,
            PushCopyOfStorageFilter::<Impl, OFFSET>,
            PushStorageFilter::<Impl, OFFSET>,
            PopStorageFilter::<Impl, OFFSET>,
            GetStorageFilterStackSize::<Impl, OFFSET>,
            AddRetrievalFilterEntries::<Impl, OFFSET>,
            GetRetrievalFilter::<Impl, OFFSET>,
            ClearRetrievalFilter::<Impl, OFFSET>,
            PushEmptyRetrievalFilter::<Impl, OFFSET>,
            PushCopyOfRetrievalFilter::<Impl, OFFSET>,
            PushRetrievalFilter::<Impl, OFFSET>,
            PopRetrievalFilter::<Impl, OFFSET>,
            GetRetrievalFilterStackSize::<Impl, OFFSET>,
            AddMessage::<Impl, OFFSET>,
            AddApplicationMessage::<Impl, OFFSET>,
            SetBreakOnCategory::<Impl, OFFSET>,
            SetBreakOnSeverity::<Impl, OFFSET>,
            SetBreakOnID::<Impl, OFFSET>,
            GetBreakOnCategory::<Impl, OFFSET>,
            GetBreakOnSeverity::<Impl, OFFSET>,
            GetBreakOnID::<Impl, OFFSET>,
            SetMuteDebugOutput::<Impl, OFFSET>,
            GetMuteDebugOutput::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D10InputLayoutImpl: Sized + ID3D10DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D10InputLayout {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10InputLayout";
}
impl ID3D10InputLayoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10InputLayoutImpl, const OFFSET: isize>() -> ID3D10InputLayoutVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10InputLayout>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D10MultithreadImpl: Sized {
    fn Enter();
    fn Leave();
    fn SetMultithreadProtected();
    fn GetMultithreadProtected();
}
impl ::windows::core::RuntimeName for ID3D10Multithread {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Multithread";
}
impl ID3D10MultithreadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10MultithreadImpl, const OFFSET: isize>() -> ID3D10MultithreadVtbl {
        unsafe extern "system" fn Enter<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enter().into()
        }
        unsafe extern "system" fn Leave<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Leave().into()
        }
        unsafe extern "system" fn SetMultithreadProtected<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMultithreadProtected(&*(&bmtprotect as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMultithreadProtected<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMultithreadProtected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10Multithread>, ::windows::core::GetTrustLevel, Enter::<Impl, OFFSET>, Leave::<Impl, OFFSET>, SetMultithreadProtected::<Impl, OFFSET>, GetMultithreadProtected::<Impl, OFFSET>)
    }
}
pub trait ID3D10PixelShaderImpl: Sized + ID3D10DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D10PixelShader {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10PixelShader";
}
impl ID3D10PixelShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10PixelShaderImpl, const OFFSET: isize>() -> ID3D10PixelShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10PixelShader>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D10PredicateImpl: Sized + ID3D10QueryImpl + ID3D10AsynchronousImpl + ID3D10DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D10Predicate {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Predicate";
}
impl ID3D10PredicateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10PredicateImpl, const OFFSET: isize>() -> ID3D10PredicateVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10Predicate>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D10QueryImpl: Sized + ID3D10AsynchronousImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10Query {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Query";
}
impl ID3D10QueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10QueryImpl, const OFFSET: isize>() -> ID3D10QueryVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10QueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_QUERY_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10Query>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10RasterizerStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10RasterizerState {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10RasterizerState";
}
impl ID3D10RasterizerStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10RasterizerStateImpl, const OFFSET: isize>() -> ID3D10RasterizerStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10RasterizerStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RASTERIZER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10RasterizerState>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10RenderTargetViewImpl: Sized + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10RenderTargetView {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10RenderTargetView";
}
impl ID3D10RenderTargetViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10RenderTargetViewImpl, const OFFSET: isize>() -> ID3D10RenderTargetViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10RenderTargetViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10RenderTargetView>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10ResourceImpl: Sized + ID3D10DeviceChildImpl {
    fn GetType();
    fn SetEvictionPriority();
    fn GetEvictionPriority();
}
impl ::windows::core::RuntimeName for ID3D10Resource {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Resource";
}
impl ID3D10ResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ResourceImpl, const OFFSET: isize>() -> ID3D10ResourceVtbl {
        unsafe extern "system" fn GetType<Impl: ID3D10ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType(::core::mem::transmute_copy(&rtype)).into()
        }
        unsafe extern "system" fn SetEvictionPriority<Impl: ID3D10ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEvictionPriority(evictionpriority).into()
        }
        unsafe extern "system" fn GetEvictionPriority<Impl: ID3D10ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEvictionPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10Resource>, ::windows::core::GetTrustLevel, GetType::<Impl, OFFSET>, SetEvictionPriority::<Impl, OFFSET>, GetEvictionPriority::<Impl, OFFSET>)
    }
}
pub trait ID3D10SamplerStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10SamplerState {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10SamplerState";
}
impl ID3D10SamplerStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10SamplerStateImpl, const OFFSET: isize>() -> ID3D10SamplerStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10SamplerStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SAMPLER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10SamplerState>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10ShaderReflectionImpl: Sized {
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetResourceBindingDesc();
    fn GetInputParameterDesc();
    fn GetOutputParameterDesc();
}
impl ::windows::core::RuntimeName for ID3D10ShaderReflection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10ShaderReflection";
}
impl ID3D10ShaderReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>() -> ID3D10ShaderReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(resourceindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputParameterDesc(parameterindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputParameterDesc(parameterindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10ShaderReflection>,
            ::windows::core::GetTrustLevel,
            GetDesc::<Impl, OFFSET>,
            GetConstantBufferByIndex::<Impl, OFFSET>,
            GetConstantBufferByName::<Impl, OFFSET>,
            GetResourceBindingDesc::<Impl, OFFSET>,
            GetInputParameterDesc::<Impl, OFFSET>,
            GetOutputParameterDesc::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for ID3D10ShaderReflection1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10ShaderReflection1";
}
impl ID3D10ShaderReflection1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>() -> ID3D10ShaderReflection1Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(resourceindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputParameterDesc(parameterindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputParameterDesc(parameterindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDescByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMovInstructionCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovcInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMovcInstructionCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionInstructionCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitwiseInstructionCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGSInputPrimitive<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprim: *mut super::Direct3D::D3D_PRIMITIVE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGSInputPrimitive(::core::mem::transmute_copy(&pprim)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLevel9Shader<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblevel9shader: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLevel9Shader(::core::mem::transmute_copy(&pblevel9shader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsamplefrequency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSampleFrequencyShader(::core::mem::transmute_copy(&pbsamplefrequency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D10ShaderReflection1>,
            ::windows::core::GetTrustLevel,
            GetDesc::<Impl, OFFSET>,
            GetConstantBufferByIndex::<Impl, OFFSET>,
            GetConstantBufferByName::<Impl, OFFSET>,
            GetResourceBindingDesc::<Impl, OFFSET>,
            GetInputParameterDesc::<Impl, OFFSET>,
            GetOutputParameterDesc::<Impl, OFFSET>,
            GetVariableByName::<Impl, OFFSET>,
            GetResourceBindingDescByName::<Impl, OFFSET>,
            GetMovInstructionCount::<Impl, OFFSET>,
            GetMovcInstructionCount::<Impl, OFFSET>,
            GetConversionInstructionCount::<Impl, OFFSET>,
            GetBitwiseInstructionCount::<Impl, OFFSET>,
            GetGSInputPrimitive::<Impl, OFFSET>,
            IsLevel9Shader::<Impl, OFFSET>,
            IsSampleFrequencyShader::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D10ShaderReflectionConstantBufferImpl: Sized {
    fn GetDesc();
    fn GetVariableByIndex();
    fn GetVariableByName();
}
impl ::windows::core::RuntimeName for ID3D10ShaderReflectionConstantBuffer {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10ShaderReflectionConstantBuffer";
}
impl ID3D10ShaderReflectionConstantBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionConstantBufferImpl, const OFFSET: isize>() -> ID3D10ShaderReflectionConstantBufferVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D10ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D10ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10ShaderReflectionConstantBuffer>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>, GetVariableByIndex::<Impl, OFFSET>, GetVariableByName::<Impl, OFFSET>)
    }
}
pub trait ID3D10ShaderReflectionTypeImpl: Sized {
    fn GetDesc();
    fn GetMemberTypeByIndex();
    fn GetMemberTypeByName();
    fn GetMemberTypeName();
}
impl ::windows::core::RuntimeName for ID3D10ShaderReflectionType {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10ShaderReflectionType";
}
impl ID3D10ShaderReflectionTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>() -> ID3D10ShaderReflectionTypeVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(&*(&pdesc as *const <D3D10_SHADER_TYPE_DESC as ::windows::core::Abi>::Abi as *const <D3D10_SHADER_TYPE_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeName(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10ShaderReflectionType>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>, GetMemberTypeByIndex::<Impl, OFFSET>, GetMemberTypeByName::<Impl, OFFSET>, GetMemberTypeName::<Impl, OFFSET>)
    }
}
pub trait ID3D10ShaderReflectionVariableImpl: Sized {
    fn GetDesc();
    fn GetType();
}
impl ::windows::core::RuntimeName for ID3D10ShaderReflectionVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10ShaderReflectionVariable";
}
impl ID3D10ShaderReflectionVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionVariableImpl, const OFFSET: isize>() -> ID3D10ShaderReflectionVariableVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ID3D10ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10ShaderReflectionVariable>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>, GetType::<Impl, OFFSET>)
    }
}
pub trait ID3D10ShaderResourceViewImpl: Sized + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10ShaderResourceView {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10ShaderResourceView";
}
impl ID3D10ShaderResourceViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderResourceViewImpl, const OFFSET: isize>() -> ID3D10ShaderResourceViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderResourceViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10ShaderResourceView>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10ShaderResourceView1Impl: Sized + ID3D10ShaderResourceViewImpl + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D10ShaderResourceView1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10ShaderResourceView1";
}
impl ID3D10ShaderResourceView1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderResourceView1Impl, const OFFSET: isize>() -> ID3D10ShaderResourceView1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D10ShaderResourceView1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10ShaderResourceView1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait ID3D10StateBlockImpl: Sized {
    fn Capture();
    fn Apply();
    fn ReleaseAllDeviceObjects();
    fn GetDevice();
}
impl ::windows::core::RuntimeName for ID3D10StateBlock {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10StateBlock";
}
impl ID3D10StateBlockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10StateBlockImpl, const OFFSET: isize>() -> ID3D10StateBlockVtbl {
        unsafe extern "system" fn Capture<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Apply<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Apply() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseAllDeviceObjects<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseAllDeviceObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10StateBlock>, ::windows::core::GetTrustLevel, Capture::<Impl, OFFSET>, Apply::<Impl, OFFSET>, ReleaseAllDeviceObjects::<Impl, OFFSET>, GetDevice::<Impl, OFFSET>)
    }
}
pub trait ID3D10SwitchToRefImpl: Sized {
    fn SetUseRef();
    fn GetUseRef();
}
impl ::windows::core::RuntimeName for ID3D10SwitchToRef {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10SwitchToRef";
}
impl ID3D10SwitchToRefVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10SwitchToRefImpl, const OFFSET: isize>() -> ID3D10SwitchToRefVtbl {
        unsafe extern "system" fn SetUseRef<Impl: ID3D10SwitchToRefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseRef(&*(&useref as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUseRef<Impl: ID3D10SwitchToRefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUseRef() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10SwitchToRef>, ::windows::core::GetTrustLevel, SetUseRef::<Impl, OFFSET>, GetUseRef::<Impl, OFFSET>)
    }
}
pub trait ID3D10Texture1DImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10Texture1D {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Texture1D";
}
impl ID3D10Texture1DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Texture1DImpl, const OFFSET: isize>() -> ID3D10Texture1DVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10Texture1DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Map(subresource, maptype, mapflags, ::core::mem::transmute_copy(&ppdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10Texture1DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap(subresource).into()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10Texture1DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE1D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10Texture1D>, ::windows::core::GetTrustLevel, Map::<Impl, OFFSET>, Unmap::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10Texture2DImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10Texture2D {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Texture2D";
}
impl ID3D10Texture2DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Texture2DImpl, const OFFSET: isize>() -> ID3D10Texture2DVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10Texture2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex2d: *mut D3D10_MAPPED_TEXTURE2D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Map(subresource, maptype, mapflags, ::core::mem::transmute_copy(&pmappedtex2d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10Texture2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap(subresource).into()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10Texture2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE2D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10Texture2D>, ::windows::core::GetTrustLevel, Map::<Impl, OFFSET>, Unmap::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10Texture3DImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D10Texture3D {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10Texture3D";
}
impl ID3D10Texture3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Texture3DImpl, const OFFSET: isize>() -> ID3D10Texture3DVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10Texture3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex3d: *mut D3D10_MAPPED_TEXTURE3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Map(subresource, maptype, mapflags, ::core::mem::transmute_copy(&pmappedtex3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10Texture3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap(subresource).into()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10Texture3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE3D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10Texture3D>, ::windows::core::GetTrustLevel, Map::<Impl, OFFSET>, Unmap::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D10VertexShaderImpl: Sized + ID3D10DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D10VertexShader {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10VertexShader";
}
impl ID3D10VertexShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10VertexShaderImpl, const OFFSET: isize>() -> ID3D10VertexShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10VertexShader>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D10ViewImpl: Sized + ID3D10DeviceChildImpl {
    fn GetResource();
}
impl ::windows::core::RuntimeName for ID3D10View {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D10.ID3D10View";
}
impl ID3D10ViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ViewImpl, const OFFSET: isize>() -> ID3D10ViewVtbl {
        unsafe extern "system" fn GetResource<Impl: ID3D10ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResource(::core::mem::transmute_copy(&ppresource)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D10View>, ::windows::core::GetTrustLevel, GetResource::<Impl, OFFSET>)
    }
}
