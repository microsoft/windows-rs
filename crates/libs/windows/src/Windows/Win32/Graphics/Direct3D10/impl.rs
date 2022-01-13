pub trait ID3D10AsynchronousImpl: Sized + ID3D10DeviceChildImpl {
    fn Begin(&mut self);
    fn End(&mut self);
    fn GetData(&mut self, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()>;
    fn GetDataSize(&mut self) -> u32;
}
impl ID3D10AsynchronousVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10AsynchronousImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10AsynchronousVtbl {
        unsafe extern "system" fn Begin<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin()
        }
        unsafe extern "system" fn End<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End()
        }
        unsafe extern "system" fn GetData<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetData(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&getdataflags)).into()
        }
        unsafe extern "system" fn GetDataSize<Impl: ID3D10AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataSize()
        }
        Self {
            base: ID3D10DeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Begin: Begin::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
            GetData: GetData::<Impl, IMPL_OFFSET>,
            GetDataSize: GetDataSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Asynchronous as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10BlendStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc(&mut self, pdesc: *mut D3D10_BLEND_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10BlendStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10BlendStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10BlendStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10BlendStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D10DeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10BlendState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10BlendState1Impl: Sized + ID3D10DeviceChildImpl + ID3D10BlendStateImpl {
    fn GetDesc1(&mut self, pdesc: *mut D3D10_BLEND_DESC1);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10BlendState1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10BlendState1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10BlendState1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D10BlendState1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D10BlendStateVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10BlendState1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10BufferImpl: Sized + ID3D10DeviceChildImpl + ID3D10ResourceImpl {
    fn Map(&mut self, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Unmap(&mut self);
    fn GetDesc(&mut self, pdesc: *mut D3D10_BUFFER_DESC);
}
impl ID3D10BufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10BufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10BufferVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10BufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Map(::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags), ::core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10BufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10BufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BUFFER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self {
            base: ID3D10ResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Map: Map::<Impl, IMPL_OFFSET>,
            Unmap: Unmap::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Buffer as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10CounterImpl: Sized + ID3D10DeviceChildImpl + ID3D10AsynchronousImpl {
    fn GetDesc(&mut self, pdesc: *mut D3D10_COUNTER_DESC);
}
impl ID3D10CounterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10CounterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10CounterVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10CounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_COUNTER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D10AsynchronousVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Counter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ID3D10DebugImpl: Sized {
    fn SetFeatureMask(&mut self, mask: u32) -> ::windows::core::Result<()>;
    fn GetFeatureMask(&mut self) -> u32;
    fn SetPresentPerRenderOpDelay(&mut self, milliseconds: u32) -> ::windows::core::Result<()>;
    fn GetPresentPerRenderOpDelay(&mut self) -> u32;
    fn SetSwapChain(&mut self, pswapchain: ::core::option::Option<super::Dxgi::IDXGISwapChain>) -> ::windows::core::Result<()>;
    fn GetSwapChain(&mut self) -> ::windows::core::Result<super::Dxgi::IDXGISwapChain>;
    fn Validate(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ID3D10DebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DebugImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10DebugVtbl {
        unsafe extern "system" fn SetFeatureMask<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFeatureMask(::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeatureMask()
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresentPerRenderOpDelay(::core::mem::transmute_copy(&milliseconds)).into()
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresentPerRenderOpDelay()
        }
        unsafe extern "system" fn SetSwapChain<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pswapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSwapChain(::core::mem::transmute(&pswapchain)).into()
        }
        unsafe extern "system" fn GetSwapChain<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSwapChain() {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Impl: ID3D10DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Validate().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetFeatureMask: SetFeatureMask::<Impl, IMPL_OFFSET>,
            GetFeatureMask: GetFeatureMask::<Impl, IMPL_OFFSET>,
            SetPresentPerRenderOpDelay: SetPresentPerRenderOpDelay::<Impl, IMPL_OFFSET>,
            GetPresentPerRenderOpDelay: GetPresentPerRenderOpDelay::<Impl, IMPL_OFFSET>,
            SetSwapChain: SetSwapChain::<Impl, IMPL_OFFSET>,
            GetSwapChain: GetSwapChain::<Impl, IMPL_OFFSET>,
            Validate: Validate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Debug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10DepthStencilStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc(&mut self, pdesc: *mut D3D10_DEPTH_STENCIL_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10DepthStencilStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DepthStencilStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10DepthStencilStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10DepthStencilStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D10DeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10DepthStencilState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10DepthStencilViewImpl: Sized + ID3D10DeviceChildImpl + ID3D10ViewImpl {
    fn GetDesc(&mut self, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10DepthStencilViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DepthStencilViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10DepthStencilViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10DepthStencilViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D10ViewVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10DepthStencilView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10DeviceImpl: Sized {
    fn VSSetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>);
    fn PSSetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>);
    fn PSSetShader(&mut self, ppixelshader: ::core::option::Option<ID3D10PixelShader>);
    fn PSSetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>);
    fn VSSetShader(&mut self, pvertexshader: ::core::option::Option<ID3D10VertexShader>);
    fn DrawIndexed(&mut self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(&mut self, vertexcount: u32, startvertexlocation: u32);
    fn PSSetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>);
    fn IASetInputLayout(&mut self, pinputlayout: ::core::option::Option<ID3D10InputLayout>);
    fn IASetVertexBuffers(&mut self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::core::option::Option<ID3D10Buffer>, pstrides: *const u32, poffsets: *const u32);
    fn IASetIndexBuffer(&mut self, pindexbuffer: ::core::option::Option<ID3D10Buffer>, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32);
    fn DrawIndexedInstanced(&mut self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn DrawInstanced(&mut self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn GSSetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>);
    fn GSSetShader(&mut self, pshader: ::core::option::Option<ID3D10GeometryShader>);
    fn IASetPrimitiveTopology(&mut self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>);
    fn VSSetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>);
    fn SetPredication(&mut self, ppredicate: ::core::option::Option<ID3D10Predicate>, predicatevalue: super::super::Foundation::BOOL);
    fn GSSetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>);
    fn GSSetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>);
    fn OMSetRenderTargets(&mut self, numviews: u32, pprendertargetviews: *const ::core::option::Option<ID3D10RenderTargetView>, pdepthstencilview: ::core::option::Option<ID3D10DepthStencilView>);
    fn OMSetBlendState(&mut self, pblendstate: ::core::option::Option<ID3D10BlendState>, blendfactor: *const f32, samplemask: u32);
    fn OMSetDepthStencilState(&mut self, pdepthstencilstate: ::core::option::Option<ID3D10DepthStencilState>, stencilref: u32);
    fn SOSetTargets(&mut self, numbuffers: u32, ppsotargets: *const ::core::option::Option<ID3D10Buffer>, poffsets: *const u32);
    fn DrawAuto(&mut self);
    fn RSSetState(&mut self, prasterizerstate: ::core::option::Option<ID3D10RasterizerState>);
    fn RSSetViewports(&mut self, numviewports: u32, pviewports: *const D3D10_VIEWPORT);
    fn RSSetScissorRects(&mut self, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn CopySubresourceRegion(&mut self, pdstresource: ::core::option::Option<ID3D10Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::core::option::Option<ID3D10Resource>, srcsubresource: u32, psrcbox: *const D3D10_BOX);
    fn CopyResource(&mut self, pdstresource: ::core::option::Option<ID3D10Resource>, psrcresource: ::core::option::Option<ID3D10Resource>);
    fn UpdateSubresource(&mut self, pdstresource: ::core::option::Option<ID3D10Resource>, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn ClearRenderTargetView(&mut self, prendertargetview: ::core::option::Option<ID3D10RenderTargetView>, colorrgba: *const f32);
    fn ClearDepthStencilView(&mut self, pdepthstencilview: ::core::option::Option<ID3D10DepthStencilView>, clearflags: u32, depth: f32, stencil: u8);
    fn GenerateMips(&mut self, pshaderresourceview: ::core::option::Option<ID3D10ShaderResourceView>);
    fn ResolveSubresource(&mut self, pdstresource: ::core::option::Option<ID3D10Resource>, dstsubresource: u32, psrcresource: ::core::option::Option<ID3D10Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn VSGetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>);
    fn PSGetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>);
    fn PSGetShader(&mut self, pppixelshader: *mut ::core::option::Option<ID3D10PixelShader>);
    fn PSGetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>);
    fn VSGetShader(&mut self, ppvertexshader: *mut ::core::option::Option<ID3D10VertexShader>);
    fn PSGetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>);
    fn IAGetInputLayout(&mut self, ppinputlayout: *mut ::core::option::Option<ID3D10InputLayout>);
    fn IAGetVertexBuffers(&mut self, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::core::option::Option<ID3D10Buffer>, pstrides: *mut u32, poffsets: *mut u32);
    fn IAGetIndexBuffer(&mut self, pindexbuffer: *mut ::core::option::Option<ID3D10Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32);
    fn GSGetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>);
    fn GSGetShader(&mut self, ppgeometryshader: *mut ::core::option::Option<ID3D10GeometryShader>);
    fn IAGetPrimitiveTopology(&mut self, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>);
    fn VSGetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>);
    fn GetPredication(&mut self, pppredicate: *mut ::core::option::Option<ID3D10Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL);
    fn GSGetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>);
    fn GSGetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>);
    fn OMGetRenderTargets(&mut self, numviews: u32, pprendertargetviews: *mut ::core::option::Option<ID3D10RenderTargetView>, ppdepthstencilview: *mut ::core::option::Option<ID3D10DepthStencilView>);
    fn OMGetBlendState(&mut self, ppblendstate: *mut ::core::option::Option<ID3D10BlendState>, blendfactor: *mut f32, psamplemask: *mut u32);
    fn OMGetDepthStencilState(&mut self, ppdepthstencilstate: *mut ::core::option::Option<ID3D10DepthStencilState>, pstencilref: *mut u32);
    fn SOGetTargets(&mut self, numbuffers: u32, ppsotargets: *mut ::core::option::Option<ID3D10Buffer>, poffsets: *mut u32);
    fn RSGetState(&mut self, pprasterizerstate: *mut ::core::option::Option<ID3D10RasterizerState>);
    fn RSGetViewports(&mut self, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT);
    fn RSGetScissorRects(&mut self, numrects: *mut u32, prects: *mut super::super::Foundation::RECT);
    fn GetDeviceRemovedReason(&mut self) -> ::windows::core::Result<()>;
    fn SetExceptionMode(&mut self, raiseflags: u32) -> ::windows::core::Result<()>;
    fn GetExceptionMode(&mut self) -> u32;
    fn GetPrivateData(&mut self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&mut self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&mut self, guid: *const ::windows::core::GUID, pdata: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ClearState(&mut self);
    fn Flush(&mut self);
    fn CreateBuffer(&mut self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Buffer>;
    fn CreateTexture1D(&mut self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture1D>;
    fn CreateTexture2D(&mut self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture2D>;
    fn CreateTexture3D(&mut self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture3D>;
    fn CreateShaderResourceView(&mut self, presource: ::core::option::Option<ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC) -> ::windows::core::Result<ID3D10ShaderResourceView>;
    fn CreateRenderTargetView(&mut self, presource: ::core::option::Option<ID3D10Resource>, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC) -> ::windows::core::Result<ID3D10RenderTargetView>;
    fn CreateDepthStencilView(&mut self, presource: ::core::option::Option<ID3D10Resource>, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC) -> ::windows::core::Result<ID3D10DepthStencilView>;
    fn CreateInputLayout(&mut self, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10InputLayout>;
    fn CreateVertexShader(&mut self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10VertexShader>;
    fn CreateGeometryShader(&mut self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10GeometryShader>;
    fn CreateGeometryShaderWithStreamOutput(&mut self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32) -> ::windows::core::Result<ID3D10GeometryShader>;
    fn CreatePixelShader(&mut self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10PixelShader>;
    fn CreateBlendState(&mut self, pblendstatedesc: *const D3D10_BLEND_DESC) -> ::windows::core::Result<ID3D10BlendState>;
    fn CreateDepthStencilState(&mut self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::Result<ID3D10DepthStencilState>;
    fn CreateRasterizerState(&mut self, prasterizerdesc: *const D3D10_RASTERIZER_DESC) -> ::windows::core::Result<ID3D10RasterizerState>;
    fn CreateSamplerState(&mut self, psamplerdesc: *const D3D10_SAMPLER_DESC) -> ::windows::core::Result<ID3D10SamplerState>;
    fn CreateQuery(&mut self, pquerydesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Query>;
    fn CreatePredicate(&mut self, ppredicatedesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Predicate>;
    fn CreateCounter(&mut self, pcounterdesc: *const D3D10_COUNTER_DESC) -> ::windows::core::Result<ID3D10Counter>;
    fn CheckFormatSupport(&mut self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32>;
    fn CheckMultisampleQualityLevels(&mut self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32>;
    fn CheckCounterInfo(&mut self, pcounterinfo: *mut D3D10_COUNTER_INFO);
    fn CheckCounter(&mut self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetCreationFlags(&mut self) -> u32;
    fn OpenSharedResource(&mut self, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetTextFilterSize(&mut self, width: u32, height: u32);
    fn GetTextFilterSize(&mut self, pwidth: *mut u32, pheight: *mut u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10DeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10DeviceVtbl {
        unsafe extern "system" fn VSSetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSSetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSSetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelshader: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetShader(::core::mem::transmute(&ppixelshader))
        }
        unsafe extern "system" fn PSSetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSSetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvertexshader: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetShader(::core::mem::transmute(&pvertexshader))
        }
        unsafe extern "system" fn DrawIndexed<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexed(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation))
        }
        unsafe extern "system" fn Draw<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Draw(::core::mem::transmute_copy(&vertexcount), ::core::mem::transmute_copy(&startvertexlocation))
        }
        unsafe extern "system" fn PSSetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IASetInputLayout<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputlayout: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetInputLayout(::core::mem::transmute(&pinputlayout))
        }
        unsafe extern "system" fn IASetVertexBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::windows::core::RawPtr, pstrides: *const u32, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetVertexBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IASetIndexBuffer<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: ::windows::core::RawPtr, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetIndexBuffer(::core::mem::transmute(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexedInstanced(::core::mem::transmute_copy(&indexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn DrawInstanced<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawInstanced(::core::mem::transmute_copy(&vertexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startvertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn GSSetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSSetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetShader(::core::mem::transmute(&pshader))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetPrimitiveTopology(::core::mem::transmute_copy(&topology))
        }
        unsafe extern "system" fn VSSetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSSetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn SetPredication<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicate: ::windows::core::RawPtr, predicatevalue: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPredication(::core::mem::transmute(&ppredicate), ::core::mem::transmute_copy(&predicatevalue))
        }
        unsafe extern "system" fn GSSetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSSetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMSetRenderTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetRenderTargets(::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute(&pdepthstencilview))
        }
        unsafe extern "system" fn OMSetBlendState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstate: ::windows::core::RawPtr, blendfactor: *const f32, samplemask: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetBlendState(::core::mem::transmute(&pblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&samplemask))
        }
        unsafe extern "system" fn OMSetDepthStencilState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilstate: ::windows::core::RawPtr, stencilref: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetDepthStencilState(::core::mem::transmute(&pdepthstencilstate), ::core::mem::transmute_copy(&stencilref))
        }
        unsafe extern "system" fn SOSetTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const ::windows::core::RawPtr, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SOSetTargets(::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn DrawAuto<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawAuto()
        }
        unsafe extern "system" fn RSSetState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerstate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetState(::core::mem::transmute(&prasterizerstate))
        }
        unsafe extern "system" fn RSSetViewports<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D10_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetViewports(::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSSetScissorRects<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetScissorRects(::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn CopySubresourceRegion<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D10_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopySubresourceRegion(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox))
        }
        unsafe extern "system" fn CopyResource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyResource(::core::mem::transmute(&pdstresource), ::core::mem::transmute(&psrcresource))
        }
        unsafe extern "system" fn UpdateSubresource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch))
        }
        unsafe extern "system" fn ClearRenderTargetView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertargetview: ::windows::core::RawPtr, colorrgba: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRenderTargetView(::core::mem::transmute(&prendertargetview), ::core::mem::transmute_copy(&colorrgba))
        }
        unsafe extern "system" fn ClearDepthStencilView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilview: ::windows::core::RawPtr, clearflags: u32, depth: f32, stencil: u8) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearDepthStencilView(::core::mem::transmute(&pdepthstencilview), ::core::mem::transmute_copy(&clearflags), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&stencil))
        }
        unsafe extern "system" fn GenerateMips<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderresourceview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateMips(::core::mem::transmute(&pshaderresourceview))
        }
        unsafe extern "system" fn ResolveSubresource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn VSGetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSGetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSGetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppixelshader: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetShader(::core::mem::transmute_copy(&pppixelshader))
        }
        unsafe extern "system" fn PSGetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSGetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvertexshader: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetShader(::core::mem::transmute_copy(&ppvertexshader))
        }
        unsafe extern "system" fn PSGetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IAGetInputLayout<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinputlayout: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetInputLayout(::core::mem::transmute_copy(&ppinputlayout))
        }
        unsafe extern "system" fn IAGetVertexBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::windows::core::RawPtr, pstrides: *mut u32, poffsets: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetVertexBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IAGetIndexBuffer<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::windows::core::RawPtr, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetIndexBuffer(::core::mem::transmute_copy(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn GSGetConstantBuffers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSGetShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetShader(::core::mem::transmute_copy(&ppgeometryshader))
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetPrimitiveTopology(::core::mem::transmute_copy(&ptopology))
        }
        unsafe extern "system" fn VSGetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSGetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn GetPredication<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppredicate: *mut ::windows::core::RawPtr, ppredicatevalue: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPredication(::core::mem::transmute_copy(&pppredicate), ::core::mem::transmute_copy(&ppredicatevalue))
        }
        unsafe extern "system" fn GSGetShaderResources<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSGetSamplers<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMGetRenderTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetRenderTargets(::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview))
        }
        unsafe extern "system" fn OMGetBlendState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblendstate: *mut ::windows::core::RawPtr, blendfactor: *mut f32, psamplemask: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetBlendState(::core::mem::transmute_copy(&ppblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&psamplemask))
        }
        unsafe extern "system" fn OMGetDepthStencilState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut ::windows::core::RawPtr, pstencilref: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetDepthStencilState(::core::mem::transmute_copy(&ppdepthstencilstate), ::core::mem::transmute_copy(&pstencilref))
        }
        unsafe extern "system" fn SOGetTargets<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut ::windows::core::RawPtr, poffsets: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SOGetTargets(::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn RSGetState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetState(::core::mem::transmute_copy(&pprasterizerstate))
        }
        unsafe extern "system" fn RSGetViewports<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetViewports(::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSGetScissorRects<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetScissorRects(::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceRemovedReason().into()
        }
        unsafe extern "system" fn SetExceptionMode<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExceptionMode(::core::mem::transmute_copy(&raiseflags)).into()
        }
        unsafe extern "system" fn GetExceptionMode<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetExceptionMode()
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        unsafe extern "system" fn ClearState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearState()
        }
        unsafe extern "system" fn Flush<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush()
        }
        unsafe extern "system" fn CreateBuffer<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBuffer(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture1D<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture1d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture1D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture1d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture2D<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture2D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture2d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture3D<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture3D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShaderResourceView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsrview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderTargetView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRenderTargetView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprtview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilView<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDepthStencilView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdepthstencilview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInputLayout<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInputLayout(::core::mem::transmute_copy(&pinputelementdescs), ::core::mem::transmute_copy(&numelements), ::core::mem::transmute_copy(&pshaderbytecodewithinputsignature), ::core::mem::transmute_copy(&bytecodelength)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinputlayout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVertexShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvertexshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometryShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgeometryshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometryShaderWithStreamOutput(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute_copy(&psodeclaration), ::core::mem::transmute_copy(&numentries), ::core::mem::transmute_copy(&outputstreamstride)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgeometryshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePixelShader<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePixelShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppixelshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlendState(::core::mem::transmute_copy(&pblendstatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppblendstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDepthStencilState(::core::mem::transmute_copy(&pdepthstencildesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdepthstencilstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRasterizerState(::core::mem::transmute_copy(&prasterizerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprasterizerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSamplerState<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSamplerState(::core::mem::transmute_copy(&psamplerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsamplerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuery<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuery(::core::mem::transmute_copy(&pquerydesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePredicate<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePredicate(::core::mem::transmute_copy(&ppredicatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppredicate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCounter<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCounter(::core::mem::transmute_copy(&pcounterdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcounter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckFormatSupport<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckFormatSupport(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pformatsupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckMultisampleQualityLevels(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&samplecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnumqualitylevels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCounterInfo<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D10_COUNTER_INFO) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckCounterInfo(::core::mem::transmute_copy(&pcounterinfo))
        }
        unsafe extern "system" fn CheckCounter<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckCounter(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pactivecounters), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&pnamelength), ::core::mem::transmute_copy(&szunits), ::core::mem::transmute_copy(&punitslength), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pdescriptionlength)).into()
        }
        unsafe extern "system" fn GetCreationFlags<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCreationFlags()
        }
        unsafe extern "system" fn OpenSharedResource<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenSharedResource(::core::mem::transmute_copy(&hresource), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into()
        }
        unsafe extern "system" fn SetTextFilterSize<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextFilterSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height))
        }
        unsafe extern "system" fn GetTextFilterSize<Impl: ID3D10DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextFilterSize(::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            VSSetConstantBuffers: VSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Impl, IMPL_OFFSET>,
            PSSetShader: PSSetShader::<Impl, IMPL_OFFSET>,
            PSSetSamplers: PSSetSamplers::<Impl, IMPL_OFFSET>,
            VSSetShader: VSSetShader::<Impl, IMPL_OFFSET>,
            DrawIndexed: DrawIndexed::<Impl, IMPL_OFFSET>,
            Draw: Draw::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers: PSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            IASetInputLayout: IASetInputLayout::<Impl, IMPL_OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Impl, IMPL_OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Impl, IMPL_OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Impl, IMPL_OFFSET>,
            DrawInstanced: DrawInstanced::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers: GSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSSetShader: GSSetShader::<Impl, IMPL_OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSSetShaderResources: VSSetShaderResources::<Impl, IMPL_OFFSET>,
            VSSetSamplers: VSSetSamplers::<Impl, IMPL_OFFSET>,
            SetPredication: SetPredication::<Impl, IMPL_OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Impl, IMPL_OFFSET>,
            GSSetSamplers: GSSetSamplers::<Impl, IMPL_OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Impl, IMPL_OFFSET>,
            OMSetBlendState: OMSetBlendState::<Impl, IMPL_OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOSetTargets: SOSetTargets::<Impl, IMPL_OFFSET>,
            DrawAuto: DrawAuto::<Impl, IMPL_OFFSET>,
            RSSetState: RSSetState::<Impl, IMPL_OFFSET>,
            RSSetViewports: RSSetViewports::<Impl, IMPL_OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Impl, IMPL_OFFSET>,
            CopyResource: CopyResource::<Impl, IMPL_OFFSET>,
            UpdateSubresource: UpdateSubresource::<Impl, IMPL_OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Impl, IMPL_OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Impl, IMPL_OFFSET>,
            GenerateMips: GenerateMips::<Impl, IMPL_OFFSET>,
            ResolveSubresource: ResolveSubresource::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers: VSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSGetShaderResources: PSGetShaderResources::<Impl, IMPL_OFFSET>,
            PSGetShader: PSGetShader::<Impl, IMPL_OFFSET>,
            PSGetSamplers: PSGetSamplers::<Impl, IMPL_OFFSET>,
            VSGetShader: VSGetShader::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers: PSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            IAGetInputLayout: IAGetInputLayout::<Impl, IMPL_OFFSET>,
            IAGetVertexBuffers: IAGetVertexBuffers::<Impl, IMPL_OFFSET>,
            IAGetIndexBuffer: IAGetIndexBuffer::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers: GSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSGetShader: GSGetShader::<Impl, IMPL_OFFSET>,
            IAGetPrimitiveTopology: IAGetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSGetShaderResources: VSGetShaderResources::<Impl, IMPL_OFFSET>,
            VSGetSamplers: VSGetSamplers::<Impl, IMPL_OFFSET>,
            GetPredication: GetPredication::<Impl, IMPL_OFFSET>,
            GSGetShaderResources: GSGetShaderResources::<Impl, IMPL_OFFSET>,
            GSGetSamplers: GSGetSamplers::<Impl, IMPL_OFFSET>,
            OMGetRenderTargets: OMGetRenderTargets::<Impl, IMPL_OFFSET>,
            OMGetBlendState: OMGetBlendState::<Impl, IMPL_OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOGetTargets: SOGetTargets::<Impl, IMPL_OFFSET>,
            RSGetState: RSGetState::<Impl, IMPL_OFFSET>,
            RSGetViewports: RSGetViewports::<Impl, IMPL_OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            SetExceptionMode: SetExceptionMode::<Impl, IMPL_OFFSET>,
            GetExceptionMode: GetExceptionMode::<Impl, IMPL_OFFSET>,
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            ClearState: ClearState::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
            CreateBuffer: CreateBuffer::<Impl, IMPL_OFFSET>,
            CreateTexture1D: CreateTexture1D::<Impl, IMPL_OFFSET>,
            CreateTexture2D: CreateTexture2D::<Impl, IMPL_OFFSET>,
            CreateTexture3D: CreateTexture3D::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Impl, IMPL_OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Impl, IMPL_OFFSET>,
            CreateInputLayout: CreateInputLayout::<Impl, IMPL_OFFSET>,
            CreateVertexShader: CreateVertexShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<Impl, IMPL_OFFSET>,
            CreatePixelShader: CreatePixelShader::<Impl, IMPL_OFFSET>,
            CreateBlendState: CreateBlendState::<Impl, IMPL_OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Impl, IMPL_OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Impl, IMPL_OFFSET>,
            CreateSamplerState: CreateSamplerState::<Impl, IMPL_OFFSET>,
            CreateQuery: CreateQuery::<Impl, IMPL_OFFSET>,
            CreatePredicate: CreatePredicate::<Impl, IMPL_OFFSET>,
            CreateCounter: CreateCounter::<Impl, IMPL_OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Impl, IMPL_OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Impl, IMPL_OFFSET>,
            CheckCounter: CheckCounter::<Impl, IMPL_OFFSET>,
            GetCreationFlags: GetCreationFlags::<Impl, IMPL_OFFSET>,
            OpenSharedResource: OpenSharedResource::<Impl, IMPL_OFFSET>,
            SetTextFilterSize: SetTextFilterSize::<Impl, IMPL_OFFSET>,
            GetTextFilterSize: GetTextFilterSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10Device1Impl: Sized + ID3D10DeviceImpl {
    fn CreateShaderResourceView1(&mut self, presource: ::core::option::Option<ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1) -> ::windows::core::Result<ID3D10ShaderResourceView1>;
    fn CreateBlendState1(&mut self, pblendstatedesc: *const D3D10_BLEND_DESC1) -> ::windows::core::Result<ID3D10BlendState1>;
    fn GetFeatureLevel(&mut self) -> D3D10_FEATURE_LEVEL1;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10Device1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Device1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10Device1Vtbl {
        unsafe extern "system" fn CreateShaderResourceView1<Impl: ID3D10Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShaderResourceView1(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsrview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState1<Impl: ID3D10Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlendState1(::core::mem::transmute_copy(&pblendstatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppblendstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureLevel<Impl: ID3D10Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D10_FEATURE_LEVEL1 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeatureLevel()
        }
        Self {
            base: ID3D10DeviceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateShaderResourceView1: CreateShaderResourceView1::<Impl, IMPL_OFFSET>,
            CreateBlendState1: CreateBlendState1::<Impl, IMPL_OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Device1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10DeviceChildImpl: Sized {
    fn GetDevice(&mut self, ppdevice: *mut ::core::option::Option<ID3D10Device>);
    fn GetPrivateData(&mut self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&mut self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&mut self, guid: *const ::windows::core::GUID, pdata: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ID3D10DeviceChildVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10DeviceChildImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10DeviceChildVtbl {
        unsafe extern "system" fn GetDevice<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&ppdevice))
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D10DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectImpl: Sized {
    fn IsValid(&mut self) -> super::super::Foundation::BOOL;
    fn IsPool(&mut self) -> super::super::Foundation::BOOL;
    fn GetDevice(&mut self) -> ::windows::core::Result<ID3D10Device>;
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D10_EFFECT_DESC>;
    fn GetConstantBufferByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn GetConstantBufferByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn GetVariableByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetVariableByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetVariableBySemantic(&mut self, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetTechniqueByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10EffectTechnique>;
    fn GetTechniqueByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectTechnique>;
    fn Optimize(&mut self) -> ::windows::core::Result<()>;
    fn IsOptimized(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsValid()
        }
        unsafe extern "system" fn IsPool<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPool()
        }
        unsafe extern "system" fn GetDevice<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetVariableBySemantic<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableBySemantic(::core::mem::transmute_copy(&semantic))
        }
        unsafe extern "system" fn GetTechniqueByIndex<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectTechnique> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTechniqueByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetTechniqueByName<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectTechnique> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTechniqueByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn Optimize<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Optimize().into()
        }
        unsafe extern "system" fn IsOptimized<Impl: ID3D10EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsOptimized()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsValid: IsValid::<Impl, IMPL_OFFSET>,
            IsPool: IsPool::<Impl, IMPL_OFFSET>,
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Impl, IMPL_OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Impl, IMPL_OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Impl, IMPL_OFFSET>,
            GetVariableByName: GetVariableByName::<Impl, IMPL_OFFSET>,
            GetVariableBySemantic: GetVariableBySemantic::<Impl, IMPL_OFFSET>,
            GetTechniqueByIndex: GetTechniqueByIndex::<Impl, IMPL_OFFSET>,
            GetTechniqueByName: GetTechniqueByName::<Impl, IMPL_OFFSET>,
            Optimize: Optimize::<Impl, IMPL_OFFSET>,
            IsOptimized: IsOptimized::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Effect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectBlendVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetBlendState(&mut self, index: u32) -> ::windows::core::Result<ID3D10BlendState>;
    fn GetBackingStore(&mut self, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectBlendVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectBlendVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectBlendVariableVtbl {
        unsafe extern "system" fn GetBlendState<Impl: ID3D10EffectBlendVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlendState(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppblendstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectBlendVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBackingStore(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pblenddesc)).into()
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetBlendState: GetBlendState::<Impl, IMPL_OFFSET>,
            GetBackingStore: GetBackingStore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectBlendVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectConstantBufferImpl: Sized + ID3D10EffectVariableImpl {
    fn SetConstantBuffer(&mut self, pconstantbuffer: ::core::option::Option<ID3D10Buffer>) -> ::windows::core::Result<()>;
    fn GetConstantBuffer(&mut self) -> ::windows::core::Result<ID3D10Buffer>;
    fn SetTextureBuffer(&mut self, ptexturebuffer: ::core::option::Option<ID3D10ShaderResourceView>) -> ::windows::core::Result<()>;
    fn GetTextureBuffer(&mut self) -> ::windows::core::Result<ID3D10ShaderResourceView>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectConstantBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectConstantBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectConstantBufferVtbl {
        unsafe extern "system" fn SetConstantBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstantbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConstantBuffer(::core::mem::transmute(&pconstantbuffer)).into()
        }
        unsafe extern "system" fn GetConstantBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconstantbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconstantbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextureBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptexturebuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextureBuffer(::core::mem::transmute(&ptexturebuffer)).into()
        }
        unsafe extern "system" fn GetTextureBuffer<Impl: ID3D10EffectConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptexturebuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextureBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *pptexturebuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetConstantBuffer: SetConstantBuffer::<Impl, IMPL_OFFSET>,
            GetConstantBuffer: GetConstantBuffer::<Impl, IMPL_OFFSET>,
            SetTextureBuffer: SetTextureBuffer::<Impl, IMPL_OFFSET>,
            GetTextureBuffer: GetTextureBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectConstantBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectDepthStencilVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetDepthStencilState(&mut self, index: u32) -> ::windows::core::Result<ID3D10DepthStencilState>;
    fn GetBackingStore(&mut self, index: u32) -> ::windows::core::Result<D3D10_DEPTH_STENCIL_DESC>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectDepthStencilVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectDepthStencilVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectDepthStencilVariableVtbl {
        unsafe extern "system" fn GetDepthStencilState<Impl: ID3D10EffectDepthStencilVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDepthStencilState(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdepthstencilstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectDepthStencilVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackingStore(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdepthstencildesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDepthStencilState: GetDepthStencilState::<Impl, IMPL_OFFSET>,
            GetBackingStore: GetBackingStore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectDepthStencilVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectDepthStencilViewVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetDepthStencil(&mut self, presource: ::core::option::Option<ID3D10DepthStencilView>) -> ::windows::core::Result<()>;
    fn GetDepthStencil(&mut self) -> ::windows::core::Result<ID3D10DepthStencilView>;
    fn SetDepthStencilArray(&mut self, ppresources: *const ::core::option::Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetDepthStencilArray(&mut self, ppresources: *mut ::core::option::Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectDepthStencilViewVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectDepthStencilViewVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectDepthStencilViewVariableVtbl {
        unsafe extern "system" fn SetDepthStencil<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDepthStencil(::core::mem::transmute(&presource)).into()
        }
        unsafe extern "system" fn GetDepthStencil<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDepthStencil() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepthStencilArray<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDepthStencilArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetDepthStencilArray<Impl: ID3D10EffectDepthStencilViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDepthStencilArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetDepthStencil: SetDepthStencil::<Impl, IMPL_OFFSET>,
            GetDepthStencil: GetDepthStencil::<Impl, IMPL_OFFSET>,
            SetDepthStencilArray: SetDepthStencilArray::<Impl, IMPL_OFFSET>,
            GetDepthStencilArray: GetDepthStencilArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectDepthStencilViewVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectMatrixVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetMatrix(&mut self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn GetMatrix(&mut self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn SetMatrixArray(&mut self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetMatrixArray(&mut self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn SetMatrixTranspose(&mut self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn GetMatrixTranspose(&mut self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn SetMatrixTransposeArray(&mut self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetMatrixTransposeArray(&mut self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectMatrixVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectMatrixVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectMatrixVariableVtbl {
        unsafe extern "system" fn SetMatrix<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetMatrix<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMatrix(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetMatrixArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetMatrixArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMatrixArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetMatrixTranspose<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixTranspose(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetMatrixTranspose<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMatrixTranspose(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetMatrixTransposeArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixTransposeArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetMatrixTransposeArray<Impl: ID3D10EffectMatrixVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMatrixTransposeArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMatrix: SetMatrix::<Impl, IMPL_OFFSET>,
            GetMatrix: GetMatrix::<Impl, IMPL_OFFSET>,
            SetMatrixArray: SetMatrixArray::<Impl, IMPL_OFFSET>,
            GetMatrixArray: GetMatrixArray::<Impl, IMPL_OFFSET>,
            SetMatrixTranspose: SetMatrixTranspose::<Impl, IMPL_OFFSET>,
            GetMatrixTranspose: GetMatrixTranspose::<Impl, IMPL_OFFSET>,
            SetMatrixTransposeArray: SetMatrixTransposeArray::<Impl, IMPL_OFFSET>,
            GetMatrixTransposeArray: GetMatrixTransposeArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectMatrixVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectPassImpl: Sized {
    fn IsValid(&mut self) -> super::super::Foundation::BOOL;
    fn GetDesc(&mut self, pdesc: *mut D3D10_PASS_DESC) -> ::windows::core::Result<()>;
    fn GetVertexShaderDesc(&mut self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()>;
    fn GetGeometryShaderDesc(&mut self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()>;
    fn GetPixelShaderDesc(&mut self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()>;
    fn GetAnnotationByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn Apply(&mut self, flags: u32) -> ::windows::core::Result<()>;
    fn ComputeStateBlockMask(&mut self) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectPassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectPassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectPassVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsValid()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVertexShaderDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVertexShaderDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetGeometryShaderDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGeometryShaderDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetPixelShaderDesc<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPixelShaderDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAnnotationByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAnnotationByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn Apply<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Apply(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn ComputeStateBlockMask<Impl: ID3D10EffectPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputeStateBlockMask() {
                ::core::result::Result::Ok(ok__) => {
                    *pstateblockmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            IsValid: IsValid::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetVertexShaderDesc: GetVertexShaderDesc::<Impl, IMPL_OFFSET>,
            GetGeometryShaderDesc: GetGeometryShaderDesc::<Impl, IMPL_OFFSET>,
            GetPixelShaderDesc: GetPixelShaderDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName: GetAnnotationByName::<Impl, IMPL_OFFSET>,
            Apply: Apply::<Impl, IMPL_OFFSET>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectPass as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10EffectPoolImpl: Sized {
    fn AsEffect(&mut self) -> ::core::option::Option<ID3D10Effect>;
}
impl ID3D10EffectPoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectPoolImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectPoolVtbl {
        unsafe extern "system" fn AsEffect<Impl: ID3D10EffectPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10Effect> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsEffect()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AsEffect: AsEffect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectPool as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectRasterizerVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetRasterizerState(&mut self, index: u32) -> ::windows::core::Result<ID3D10RasterizerState>;
    fn GetBackingStore(&mut self, index: u32) -> ::windows::core::Result<D3D10_RASTERIZER_DESC>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectRasterizerVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectRasterizerVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectRasterizerVariableVtbl {
        unsafe extern "system" fn GetRasterizerState<Impl: ID3D10EffectRasterizerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRasterizerState(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprasterizerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectRasterizerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackingStore(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *prasterizerdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetRasterizerState: GetRasterizerState::<Impl, IMPL_OFFSET>,
            GetBackingStore: GetBackingStore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectRasterizerVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectRenderTargetViewVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetRenderTarget(&mut self, presource: ::core::option::Option<ID3D10RenderTargetView>) -> ::windows::core::Result<()>;
    fn GetRenderTarget(&mut self) -> ::windows::core::Result<ID3D10RenderTargetView>;
    fn SetRenderTargetArray(&mut self, ppresources: *const ::core::option::Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetRenderTargetArray(&mut self, ppresources: *mut ::core::option::Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectRenderTargetViewVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectRenderTargetViewVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectRenderTargetViewVariableVtbl {
        unsafe extern "system" fn SetRenderTarget<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRenderTarget(::core::mem::transmute(&presource)).into()
        }
        unsafe extern "system" fn GetRenderTarget<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRenderTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderTargetArray<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRenderTargetArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetRenderTargetArray<Impl: ID3D10EffectRenderTargetViewVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRenderTargetArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetRenderTarget: SetRenderTarget::<Impl, IMPL_OFFSET>,
            GetRenderTarget: GetRenderTarget::<Impl, IMPL_OFFSET>,
            SetRenderTargetArray: SetRenderTargetArray::<Impl, IMPL_OFFSET>,
            GetRenderTargetArray: GetRenderTargetArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectRenderTargetViewVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectSamplerVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetSampler(&mut self, index: u32) -> ::windows::core::Result<ID3D10SamplerState>;
    fn GetBackingStore(&mut self, index: u32) -> ::windows::core::Result<D3D10_SAMPLER_DESC>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectSamplerVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectSamplerVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectSamplerVariableVtbl {
        unsafe extern "system" fn GetSampler<Impl: ID3D10EffectSamplerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppsampler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSampler(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsampler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectSamplerVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackingStore(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *psamplerdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSampler: GetSampler::<Impl, IMPL_OFFSET>,
            GetBackingStore: GetBackingStore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectSamplerVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectScalarVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetFloat(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn GetFloat(&mut self) -> ::windows::core::Result<f32>;
    fn SetFloatArray(&mut self, pdata: *const f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetFloatArray(&mut self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn SetInt(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetInt(&mut self) -> ::windows::core::Result<i32>;
    fn SetIntArray(&mut self, pdata: *const i32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetIntArray(&mut self, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn SetBool(&mut self, value: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBool(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetBoolArray(&mut self, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetBoolArray(&mut self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectScalarVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectScalarVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectScalarVariableVtbl {
        unsafe extern "system" fn SetFloat<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFloat(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetFloat<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFloat() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFloatArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFloatArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetFloatArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFloatArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetInt<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInt(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetInt<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInt() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetIntArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIntArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetBool<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBool(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBool<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBool() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoolArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetBoolArray<Impl: ID3D10EffectScalarVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBoolArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetFloat: SetFloat::<Impl, IMPL_OFFSET>,
            GetFloat: GetFloat::<Impl, IMPL_OFFSET>,
            SetFloatArray: SetFloatArray::<Impl, IMPL_OFFSET>,
            GetFloatArray: GetFloatArray::<Impl, IMPL_OFFSET>,
            SetInt: SetInt::<Impl, IMPL_OFFSET>,
            GetInt: GetInt::<Impl, IMPL_OFFSET>,
            SetIntArray: SetIntArray::<Impl, IMPL_OFFSET>,
            GetIntArray: GetIntArray::<Impl, IMPL_OFFSET>,
            SetBool: SetBool::<Impl, IMPL_OFFSET>,
            GetBool: GetBool::<Impl, IMPL_OFFSET>,
            SetBoolArray: SetBoolArray::<Impl, IMPL_OFFSET>,
            GetBoolArray: GetBoolArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectScalarVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectShaderResourceVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetResource(&mut self, presource: ::core::option::Option<ID3D10ShaderResourceView>) -> ::windows::core::Result<()>;
    fn GetResource(&mut self) -> ::windows::core::Result<ID3D10ShaderResourceView>;
    fn SetResourceArray(&mut self, ppresources: *const ::core::option::Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetResourceArray(&mut self, ppresources: *mut ::core::option::Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectShaderResourceVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectShaderResourceVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectShaderResourceVariableVtbl {
        unsafe extern "system" fn SetResource<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResource(::core::mem::transmute(&presource)).into()
        }
        unsafe extern "system" fn GetResource<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResource() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourceArray<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResourceArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetResourceArray<Impl: ID3D10EffectShaderResourceVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResourceArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetResource: SetResource::<Impl, IMPL_OFFSET>,
            GetResource: GetResource::<Impl, IMPL_OFFSET>,
            SetResourceArray: SetResourceArray::<Impl, IMPL_OFFSET>,
            GetResourceArray: GetResourceArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectShaderResourceVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10EffectShaderVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetShaderDesc(&mut self, shaderindex: u32) -> ::windows::core::Result<D3D10_EFFECT_SHADER_DESC>;
    fn GetVertexShader(&mut self, shaderindex: u32) -> ::windows::core::Result<ID3D10VertexShader>;
    fn GetGeometryShader(&mut self, shaderindex: u32) -> ::windows::core::Result<ID3D10GeometryShader>;
    fn GetPixelShader(&mut self, shaderindex: u32) -> ::windows::core::Result<ID3D10PixelShader>;
    fn GetInputSignatureElementDesc(&mut self, shaderindex: u32, element: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
    fn GetOutputSignatureElementDesc(&mut self, shaderindex: u32, element: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10EffectShaderVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectShaderVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectShaderVariableVtbl {
        unsafe extern "system" fn GetShaderDesc<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShaderDesc(::core::mem::transmute_copy(&shaderindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVertexShader<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppvs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVertexShader(::core::mem::transmute_copy(&shaderindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryShader<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppgs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeometryShader(::core::mem::transmute_copy(&shaderindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelShader<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelShader(::core::mem::transmute_copy(&shaderindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSignatureElementDesc<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputSignatureElementDesc(::core::mem::transmute_copy(&shaderindex), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputSignatureElementDesc<Impl: ID3D10EffectShaderVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputSignatureElementDesc(::core::mem::transmute_copy(&shaderindex), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetShaderDesc: GetShaderDesc::<Impl, IMPL_OFFSET>,
            GetVertexShader: GetVertexShader::<Impl, IMPL_OFFSET>,
            GetGeometryShader: GetGeometryShader::<Impl, IMPL_OFFSET>,
            GetPixelShader: GetPixelShader::<Impl, IMPL_OFFSET>,
            GetInputSignatureElementDesc: GetInputSignatureElementDesc::<Impl, IMPL_OFFSET>,
            GetOutputSignatureElementDesc: GetOutputSignatureElementDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectShaderVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectStringVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetString(&mut self) -> ::windows::core::Result<super::super::Foundation::PSTR>;
    fn GetStringArray(&mut self, ppstrings: *mut super::super::Foundation::PSTR, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectStringVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectStringVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectStringVariableVtbl {
        unsafe extern "system" fn GetString<Impl: ID3D10EffectStringVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstring: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringArray<Impl: ID3D10EffectStringVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstrings: *mut super::super::Foundation::PSTR, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStringArray(::core::mem::transmute_copy(&ppstrings), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetString: GetString::<Impl, IMPL_OFFSET>,
            GetStringArray: GetStringArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectStringVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectTechniqueImpl: Sized {
    fn IsValid(&mut self) -> super::super::Foundation::BOOL;
    fn GetDesc(&mut self, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows::core::Result<()>;
    fn GetAnnotationByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetPassByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10EffectPass>;
    fn GetPassByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectPass>;
    fn ComputeStateBlockMask(&mut self) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectTechniqueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectTechniqueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectTechniqueVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsValid()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAnnotationByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAnnotationByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetPassByIndex<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectPass> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPassByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetPassByName<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectPass> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPassByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn ComputeStateBlockMask<Impl: ID3D10EffectTechniqueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputeStateBlockMask() {
                ::core::result::Result::Ok(ok__) => {
                    *pstateblockmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            IsValid: IsValid::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName: GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetPassByIndex: GetPassByIndex::<Impl, IMPL_OFFSET>,
            GetPassByName: GetPassByName::<Impl, IMPL_OFFSET>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectTechnique as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10EffectTypeImpl: Sized {
    fn IsValid(&mut self) -> super::super::Foundation::BOOL;
    fn GetDesc(&mut self, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows::core::Result<()>;
    fn GetMemberTypeByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10EffectType>;
    fn GetMemberTypeByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectType>;
    fn GetMemberTypeBySemantic(&mut self, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectType>;
    fn GetMemberName(&mut self, index: u32) -> super::super::Foundation::PSTR;
    fn GetMemberSemantic(&mut self, index: u32) -> super::super::Foundation::PSTR;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10EffectTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectTypeVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsValid()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetMemberTypeBySemantic<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeBySemantic(::core::mem::transmute_copy(&semantic))
        }
        unsafe extern "system" fn GetMemberName<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberName(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberSemantic<Impl: ID3D10EffectTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberSemantic(::core::mem::transmute_copy(&index))
        }
        Self {
            IsValid: IsValid::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl, IMPL_OFFSET>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl, IMPL_OFFSET>,
            GetMemberTypeBySemantic: GetMemberTypeBySemantic::<Impl, IMPL_OFFSET>,
            GetMemberName: GetMemberName::<Impl, IMPL_OFFSET>,
            GetMemberSemantic: GetMemberSemantic::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectType as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectVariableImpl: Sized {
    fn IsValid(&mut self) -> super::super::Foundation::BOOL;
    fn GetType(&mut self) -> ::core::option::Option<ID3D10EffectType>;
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC>;
    fn GetAnnotationByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetMemberByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&mut self, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetElement(&mut self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&mut self) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&mut self) -> ::core::option::Option<ID3D10EffectScalarVariable>;
    fn AsVector(&mut self) -> ::core::option::Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&mut self) -> ::core::option::Option<ID3D10EffectMatrixVariable>;
    fn AsString(&mut self) -> ::core::option::Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&mut self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&mut self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&mut self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&mut self) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&mut self) -> ::core::option::Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&mut self) -> ::core::option::Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&mut self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&mut self) -> ::core::option::Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&mut self) -> ::core::option::Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&mut self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()>;
    fn GetRawValue(&mut self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectVariableVtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsValid()
        }
        unsafe extern "system" fn GetType<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAnnotationByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAnnotationByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetMemberByIndex<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberByName<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetMemberBySemantic<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberBySemantic(::core::mem::transmute_copy(&semantic))
        }
        unsafe extern "system" fn GetElement<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetElement(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetParentConstantBuffer<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetParentConstantBuffer()
        }
        unsafe extern "system" fn AsScalar<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsScalar()
        }
        unsafe extern "system" fn AsVector<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsVector()
        }
        unsafe extern "system" fn AsMatrix<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsMatrix()
        }
        unsafe extern "system" fn AsString<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsString()
        }
        unsafe extern "system" fn AsShaderResource<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsShaderResource()
        }
        unsafe extern "system" fn AsRenderTargetView<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsRenderTargetView()
        }
        unsafe extern "system" fn AsDepthStencilView<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsDepthStencilView()
        }
        unsafe extern "system" fn AsConstantBuffer<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsConstantBuffer()
        }
        unsafe extern "system" fn AsShader<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsShader()
        }
        unsafe extern "system" fn AsBlend<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsBlend()
        }
        unsafe extern "system" fn AsDepthStencil<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsDepthStencil()
        }
        unsafe extern "system" fn AsRasterizer<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsRasterizer()
        }
        unsafe extern "system" fn AsSampler<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsSampler()
        }
        unsafe extern "system" fn SetRawValue<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRawValue(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&bytecount)).into()
        }
        unsafe extern "system" fn GetRawValue<Impl: ID3D10EffectVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRawValue(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&bytecount)).into()
        }
        Self {
            IsValid: IsValid::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Impl, IMPL_OFFSET>,
            GetAnnotationByName: GetAnnotationByName::<Impl, IMPL_OFFSET>,
            GetMemberByIndex: GetMemberByIndex::<Impl, IMPL_OFFSET>,
            GetMemberByName: GetMemberByName::<Impl, IMPL_OFFSET>,
            GetMemberBySemantic: GetMemberBySemantic::<Impl, IMPL_OFFSET>,
            GetElement: GetElement::<Impl, IMPL_OFFSET>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Impl, IMPL_OFFSET>,
            AsScalar: AsScalar::<Impl, IMPL_OFFSET>,
            AsVector: AsVector::<Impl, IMPL_OFFSET>,
            AsMatrix: AsMatrix::<Impl, IMPL_OFFSET>,
            AsString: AsString::<Impl, IMPL_OFFSET>,
            AsShaderResource: AsShaderResource::<Impl, IMPL_OFFSET>,
            AsRenderTargetView: AsRenderTargetView::<Impl, IMPL_OFFSET>,
            AsDepthStencilView: AsDepthStencilView::<Impl, IMPL_OFFSET>,
            AsConstantBuffer: AsConstantBuffer::<Impl, IMPL_OFFSET>,
            AsShader: AsShader::<Impl, IMPL_OFFSET>,
            AsBlend: AsBlend::<Impl, IMPL_OFFSET>,
            AsDepthStencil: AsDepthStencil::<Impl, IMPL_OFFSET>,
            AsRasterizer: AsRasterizer::<Impl, IMPL_OFFSET>,
            AsSampler: AsSampler::<Impl, IMPL_OFFSET>,
            SetRawValue: SetRawValue::<Impl, IMPL_OFFSET>,
            GetRawValue: GetRawValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectVectorVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetBoolVector(&mut self, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetIntVector(&mut self, pdata: *mut i32) -> ::windows::core::Result<()>;
    fn SetFloatVector(&mut self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn GetBoolVector(&mut self, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetIntVector(&mut self, pdata: *mut i32) -> ::windows::core::Result<()>;
    fn GetFloatVector(&mut self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn SetBoolVectorArray(&mut self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn SetIntVectorArray(&mut self, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn SetFloatVectorArray(&mut self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetBoolVectorArray(&mut self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetIntVectorArray(&mut self, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetFloatVectorArray(&mut self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVectorVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10EffectVectorVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10EffectVectorVariableVtbl {
        unsafe extern "system" fn SetBoolVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoolVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetIntVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetFloatVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFloatVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetBoolVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBoolVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetIntVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIntVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetFloatVector<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFloatVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetBoolVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoolVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetIntVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetFloatVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFloatVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetBoolVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBoolVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetIntVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIntVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetFloatVectorArray<Impl: ID3D10EffectVectorVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFloatVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base: ID3D10EffectVariableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetBoolVector: SetBoolVector::<Impl, IMPL_OFFSET>,
            SetIntVector: SetIntVector::<Impl, IMPL_OFFSET>,
            SetFloatVector: SetFloatVector::<Impl, IMPL_OFFSET>,
            GetBoolVector: GetBoolVector::<Impl, IMPL_OFFSET>,
            GetIntVector: GetIntVector::<Impl, IMPL_OFFSET>,
            GetFloatVector: GetFloatVector::<Impl, IMPL_OFFSET>,
            SetBoolVectorArray: SetBoolVectorArray::<Impl, IMPL_OFFSET>,
            SetIntVectorArray: SetIntVectorArray::<Impl, IMPL_OFFSET>,
            SetFloatVectorArray: SetFloatVectorArray::<Impl, IMPL_OFFSET>,
            GetBoolVectorArray: GetBoolVectorArray::<Impl, IMPL_OFFSET>,
            GetIntVectorArray: GetIntVectorArray::<Impl, IMPL_OFFSET>,
            GetFloatVectorArray: GetFloatVectorArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectVectorVariable as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10GeometryShaderImpl: Sized + ID3D10DeviceChildImpl {}
impl ID3D10GeometryShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10GeometryShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10GeometryShaderVtbl {
        Self { base: ID3D10DeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10GeometryShader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10InfoQueueImpl: Sized {
    fn SetMessageCountLimit(&mut self, messagecountlimit: u64) -> ::windows::core::Result<()>;
    fn ClearStoredMessages(&mut self);
    fn GetMessage(&mut self, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(&mut self) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&mut self) -> u64;
    fn GetNumStoredMessages(&mut self) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(&mut self) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&mut self) -> u64;
    fn GetMessageCountLimit(&mut self) -> u64;
    fn AddStorageFilterEntries(&mut self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetStorageFilter(&mut self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearStorageFilter(&mut self);
    fn PushEmptyStorageFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushCopyOfStorageFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushStorageFilter(&mut self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopStorageFilter(&mut self);
    fn GetStorageFilterStackSize(&mut self) -> u32;
    fn AddRetrievalFilterEntries(&mut self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetRetrievalFilter(&mut self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearRetrievalFilter(&mut self);
    fn PushEmptyRetrievalFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushCopyOfRetrievalFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushRetrievalFilter(&mut self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopRetrievalFilter(&mut self);
    fn GetRetrievalFilterStackSize(&mut self) -> u32;
    fn AddMessage(&mut self, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn AddApplicationMessage(&mut self, severity: D3D10_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn SetBreakOnCategory(&mut self, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnSeverity(&mut self, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnID(&mut self, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBreakOnCategory(&mut self, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&mut self, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&mut self, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&mut self, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10InfoQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10InfoQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10InfoQueueVtbl {
        unsafe extern "system" fn SetMessageCountLimit<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageCountLimit(::core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStoredMessages()
        }
        unsafe extern "system" fn GetMessage<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMessage(::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesAllowedByStorageFilter()
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesDeniedByStorageFilter()
        }
        unsafe extern "system" fn GetNumStoredMessages<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumStoredMessages()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumStoredMessagesAllowedByRetrievalFilter()
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesDiscardedByMessageCountLimit()
        }
        unsafe extern "system" fn GetMessageCountLimit<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMessageCountLimit()
        }
        unsafe extern "system" fn AddStorageFilterEntries<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStorageFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStorageFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStorageFilter()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushEmptyStorageFilter().into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushCopyOfStorageFilter().into()
        }
        unsafe extern "system" fn PushStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushStorageFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopStorageFilter()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStorageFilterStackSize()
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRetrievalFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRetrievalFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRetrievalFilter()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushEmptyRetrievalFilter().into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushCopyOfRetrievalFilter().into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushRetrievalFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopRetrievalFilter()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRetrievalFilterStackSize()
        }
        unsafe extern "system" fn AddMessage<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMessage(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddApplicationMessage(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnCategory(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnSeverity(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnID(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnCategory(::core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnSeverity(::core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnID(::core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMuteDebugOutput(::core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Impl: ID3D10InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMuteDebugOutput()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Impl, IMPL_OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Impl, IMPL_OFFSET>,
            GetMessage: GetMessage::<Impl, IMPL_OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Impl, IMPL_OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Impl, IMPL_OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Impl, IMPL_OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Impl, IMPL_OFFSET>,
            GetStorageFilter: GetStorageFilter::<Impl, IMPL_OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Impl, IMPL_OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Impl, IMPL_OFFSET>,
            PushStorageFilter: PushStorageFilter::<Impl, IMPL_OFFSET>,
            PopStorageFilter: PopStorageFilter::<Impl, IMPL_OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Impl, IMPL_OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Impl, IMPL_OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Impl, IMPL_OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Impl, IMPL_OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Impl, IMPL_OFFSET>,
            AddMessage: AddMessage::<Impl, IMPL_OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Impl, IMPL_OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Impl, IMPL_OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            SetBreakOnID: SetBreakOnID::<Impl, IMPL_OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Impl, IMPL_OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            GetBreakOnID: GetBreakOnID::<Impl, IMPL_OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Impl, IMPL_OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10InfoQueue as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10InputLayoutImpl: Sized + ID3D10DeviceChildImpl {}
impl ID3D10InputLayoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10InputLayoutImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10InputLayoutVtbl {
        Self { base: ID3D10DeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10InputLayout as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10MultithreadImpl: Sized {
    fn Enter(&mut self);
    fn Leave(&mut self);
    fn SetMultithreadProtected(&mut self, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetMultithreadProtected(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10MultithreadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10MultithreadImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10MultithreadVtbl {
        unsafe extern "system" fn Enter<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enter()
        }
        unsafe extern "system" fn Leave<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Leave()
        }
        unsafe extern "system" fn SetMultithreadProtected<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMultithreadProtected(::core::mem::transmute_copy(&bmtprotect))
        }
        unsafe extern "system" fn GetMultithreadProtected<Impl: ID3D10MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMultithreadProtected()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Enter: Enter::<Impl, IMPL_OFFSET>,
            Leave: Leave::<Impl, IMPL_OFFSET>,
            SetMultithreadProtected: SetMultithreadProtected::<Impl, IMPL_OFFSET>,
            GetMultithreadProtected: GetMultithreadProtected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Multithread as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10PixelShaderImpl: Sized + ID3D10DeviceChildImpl {}
impl ID3D10PixelShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10PixelShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10PixelShaderVtbl {
        Self { base: ID3D10DeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10PixelShader as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10PredicateImpl: Sized + ID3D10DeviceChildImpl + ID3D10AsynchronousImpl + ID3D10QueryImpl {}
impl ID3D10PredicateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10PredicateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10PredicateVtbl {
        Self { base: ID3D10QueryVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Predicate as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10QueryImpl: Sized + ID3D10DeviceChildImpl + ID3D10AsynchronousImpl {
    fn GetDesc(&mut self, pdesc: *mut D3D10_QUERY_DESC);
}
impl ID3D10QueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10QueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10QueryVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10QueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_QUERY_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D10AsynchronousVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Query as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10RasterizerStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc(&mut self, pdesc: *mut D3D10_RASTERIZER_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10RasterizerStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10RasterizerStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10RasterizerStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10RasterizerStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RASTERIZER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D10DeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10RasterizerState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10RenderTargetViewImpl: Sized + ID3D10DeviceChildImpl + ID3D10ViewImpl {
    fn GetDesc(&mut self, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10RenderTargetViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10RenderTargetViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10RenderTargetViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10RenderTargetViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D10ViewVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10RenderTargetView as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10ResourceImpl: Sized + ID3D10DeviceChildImpl {
    fn GetType(&mut self, rtype: *mut D3D10_RESOURCE_DIMENSION);
    fn SetEvictionPriority(&mut self, evictionpriority: u32);
    fn GetEvictionPriority(&mut self) -> u32;
}
impl ID3D10ResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ResourceVtbl {
        unsafe extern "system" fn GetType<Impl: ID3D10ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType(::core::mem::transmute_copy(&rtype))
        }
        unsafe extern "system" fn SetEvictionPriority<Impl: ID3D10ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEvictionPriority(::core::mem::transmute_copy(&evictionpriority))
        }
        unsafe extern "system" fn GetEvictionPriority<Impl: ID3D10ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEvictionPriority()
        }
        Self {
            base: ID3D10DeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Resource as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10SamplerStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc(&mut self, pdesc: *mut D3D10_SAMPLER_DESC);
}
impl ID3D10SamplerStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10SamplerStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10SamplerStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10SamplerStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SAMPLER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D10DeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10SamplerState as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10ShaderReflectionImpl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D10_SHADER_DESC>;
    fn GetConstantBufferByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&mut self, resourceindex: u32) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC>;
    fn GetInputParameterDesc(&mut self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
    fn GetOutputParameterDesc(&mut self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10ShaderReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Impl: ID3D10ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Impl, IMPL_OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Impl, IMPL_OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Impl, IMPL_OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10ShaderReflection1Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D10_SHADER_DESC>;
    fn GetConstantBufferByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&mut self, resourceindex: u32) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC>;
    fn GetInputParameterDesc(&mut self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
    fn GetOutputParameterDesc(&mut self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
    fn GetVariableByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&mut self, name: super::super::Foundation::PSTR) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC>;
    fn GetMovInstructionCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetMovcInstructionCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetConversionInstructionCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetBitwiseInstructionCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetGSInputPrimitive(&mut self) -> ::windows::core::Result<super::Direct3D::D3D_PRIMITIVE>;
    fn IsLevel9Shader(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsSampleFrequencyShader(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10ShaderReflection1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflection1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderReflection1Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDescByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMovInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovcInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMovcInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitwiseInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGSInputPrimitive<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprim: *mut super::Direct3D::D3D_PRIMITIVE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGSInputPrimitive() {
                ::core::result::Result::Ok(ok__) => {
                    *pprim = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLevel9Shader<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblevel9shader: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLevel9Shader() {
                ::core::result::Result::Ok(ok__) => {
                    *pblevel9shader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Impl: ID3D10ShaderReflection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsamplefrequency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSampleFrequencyShader() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsamplefrequency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Impl, IMPL_OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Impl, IMPL_OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Impl, IMPL_OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Impl, IMPL_OFFSET>,
            GetVariableByName: GetVariableByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Impl, IMPL_OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Impl, IMPL_OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Impl, IMPL_OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Impl, IMPL_OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Impl, IMPL_OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Impl, IMPL_OFFSET>,
            IsLevel9Shader: IsLevel9Shader::<Impl, IMPL_OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10ShaderReflectionConstantBufferImpl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D10_SHADER_BUFFER_DESC>;
    fn GetVariableByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable>;
    fn GetVariableByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10ShaderReflectionConstantBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionConstantBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderReflectionConstantBufferVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D10ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D10ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        Self {
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Impl, IMPL_OFFSET>,
            GetVariableByName: GetVariableByName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflectionConstantBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10ShaderReflectionTypeImpl: Sized {
    fn GetDesc(&mut self, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows::core::Result<()>;
    fn GetMemberTypeByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeName(&mut self, index: u32) -> super::super::Foundation::PSTR;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10ShaderReflectionTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderReflectionTypeVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D10ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeName(::core::mem::transmute_copy(&index))
        }
        Self {
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl, IMPL_OFFSET>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl, IMPL_OFFSET>,
            GetMemberTypeName: GetMemberTypeName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflectionType as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10ShaderReflectionVariableImpl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D10_SHADER_VARIABLE_DESC>;
    fn GetType(&mut self) -> ::core::option::Option<ID3D10ShaderReflectionType>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10ShaderReflectionVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderReflectionVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderReflectionVariableVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ID3D10ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType()
        }
        Self { GetDesc: GetDesc::<Impl, IMPL_OFFSET>, GetType: GetType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflectionVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceViewImpl: Sized + ID3D10DeviceChildImpl + ID3D10ViewImpl {
    fn GetDesc(&mut self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10ShaderResourceViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderResourceViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderResourceViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderResourceViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D10ViewVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceView1Impl: Sized + ID3D10DeviceChildImpl + ID3D10ViewImpl + ID3D10ShaderResourceViewImpl {
    fn GetDesc1(&mut self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10ShaderResourceView1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ShaderResourceView1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ShaderResourceView1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D10ShaderResourceView1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D10ShaderResourceViewVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10StateBlockImpl: Sized {
    fn Capture(&mut self) -> ::windows::core::Result<()>;
    fn Apply(&mut self) -> ::windows::core::Result<()>;
    fn ReleaseAllDeviceObjects(&mut self) -> ::windows::core::Result<()>;
    fn GetDevice(&mut self) -> ::windows::core::Result<ID3D10Device>;
}
impl ID3D10StateBlockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10StateBlockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10StateBlockVtbl {
        unsafe extern "system" fn Capture<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Capture().into()
        }
        unsafe extern "system" fn Apply<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Apply().into()
        }
        unsafe extern "system" fn ReleaseAllDeviceObjects<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseAllDeviceObjects().into()
        }
        unsafe extern "system" fn GetDevice<Impl: ID3D10StateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Capture: Capture::<Impl, IMPL_OFFSET>,
            Apply: Apply::<Impl, IMPL_OFFSET>,
            ReleaseAllDeviceObjects: ReleaseAllDeviceObjects::<Impl, IMPL_OFFSET>,
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10StateBlock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10SwitchToRefImpl: Sized {
    fn SetUseRef(&mut self, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetUseRef(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10SwitchToRefVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10SwitchToRefImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10SwitchToRefVtbl {
        unsafe extern "system" fn SetUseRef<Impl: ID3D10SwitchToRefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseRef(::core::mem::transmute_copy(&useref))
        }
        unsafe extern "system" fn GetUseRef<Impl: ID3D10SwitchToRefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUseRef()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetUseRef: SetUseRef::<Impl, IMPL_OFFSET>,
            GetUseRef: GetUseRef::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10SwitchToRef as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture1DImpl: Sized + ID3D10DeviceChildImpl + ID3D10ResourceImpl {
    fn Map(&mut self, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Unmap(&mut self, subresource: u32);
    fn GetDesc(&mut self, pdesc: *mut D3D10_TEXTURE1D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture1DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Texture1DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10Texture1DVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10Texture1DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Map(::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags), ::core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10Texture1DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap(::core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10Texture1DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE1D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self {
            base: ID3D10ResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Map: Map::<Impl, IMPL_OFFSET>,
            Unmap: Unmap::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Texture1D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture2DImpl: Sized + ID3D10DeviceChildImpl + ID3D10ResourceImpl {
    fn Map(&mut self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> ::windows::core::Result<D3D10_MAPPED_TEXTURE2D>;
    fn Unmap(&mut self, subresource: u32);
    fn GetDesc(&mut self, pdesc: *mut D3D10_TEXTURE2D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture2DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Texture2DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10Texture2DVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10Texture2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex2d: *mut D3D10_MAPPED_TEXTURE2D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Map(::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmappedtex2d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10Texture2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap(::core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10Texture2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE2D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self {
            base: ID3D10ResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Map: Map::<Impl, IMPL_OFFSET>,
            Unmap: Unmap::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Texture2D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture3DImpl: Sized + ID3D10DeviceChildImpl + ID3D10ResourceImpl {
    fn Map(&mut self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> ::windows::core::Result<D3D10_MAPPED_TEXTURE3D>;
    fn Unmap(&mut self, subresource: u32);
    fn GetDesc(&mut self, pdesc: *mut D3D10_TEXTURE3D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10Texture3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10Texture3DVtbl {
        unsafe extern "system" fn Map<Impl: ID3D10Texture3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex3d: *mut D3D10_MAPPED_TEXTURE3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Map(::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmappedtex3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ID3D10Texture3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap(::core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10Texture3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE3D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self {
            base: ID3D10ResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Map: Map::<Impl, IMPL_OFFSET>,
            Unmap: Unmap::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Texture3D as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10VertexShaderImpl: Sized + ID3D10DeviceChildImpl {}
impl ID3D10VertexShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10VertexShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10VertexShaderVtbl {
        Self { base: ID3D10DeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10VertexShader as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10ViewImpl: Sized + ID3D10DeviceChildImpl {
    fn GetResource(&mut self, ppresource: *mut ::core::option::Option<ID3D10Resource>);
}
impl ID3D10ViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D10ViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D10ViewVtbl {
        unsafe extern "system" fn GetResource<Impl: ID3D10ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResource(::core::mem::transmute_copy(&ppresource))
        }
        Self { base: ID3D10DeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetResource: GetResource::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10View as ::windows::core::Interface>::IID
    }
}
