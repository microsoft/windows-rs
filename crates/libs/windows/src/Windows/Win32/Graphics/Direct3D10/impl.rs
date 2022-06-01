pub trait ID3D10Asynchronous_Impl: Sized + ID3D10DeviceChild_Impl {
    fn Begin(&self);
    fn End(&self);
    fn GetData(&self, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()>;
    fn GetDataSize(&self) -> u32;
}
impl ::windows::core::RuntimeName for ID3D10Asynchronous {}
impl ID3D10Asynchronous_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Asynchronous_Impl, const OFFSET: isize>() -> ID3D10Asynchronous_Vtbl {
        unsafe extern "system" fn Begin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin()
        }
        unsafe extern "system" fn End<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.End()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetData(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&getdataflags)).into()
        }
        unsafe extern "system" fn GetDataSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataSize()
        }
        Self {
            base__: ID3D10DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            Begin: Begin::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDataSize: GetDataSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Asynchronous as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10BlendState_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_BLEND_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10BlendState {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10BlendState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10BlendState_Impl, const OFFSET: isize>() -> ID3D10BlendState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10BlendState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10BlendState as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10BlendState1_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10BlendState_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D10_BLEND_DESC1);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10BlendState1 {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10BlendState1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10BlendState1_Impl, const OFFSET: isize>() -> ID3D10BlendState1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10BlendState1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10BlendState_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10BlendState1 as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10BlendState as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10Buffer_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10Resource_Impl {
    fn Map(&self, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Unmap(&self);
    fn GetDesc(&self, pdesc: *mut D3D10_BUFFER_DESC);
}
impl ::windows::core::RuntimeName for ID3D10Buffer {}
impl ID3D10Buffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Buffer_Impl, const OFFSET: isize>() -> ID3D10Buffer_Vtbl {
        unsafe extern "system" fn Map<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Buffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Map(::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags), ::core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn Unmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Buffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unmap()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Buffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BUFFER_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Buffer as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10Resource as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10Counter_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_COUNTER_DESC);
}
impl ::windows::core::RuntimeName for ID3D10Counter {}
impl ID3D10Counter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Counter_Impl, const OFFSET: isize>() -> ID3D10Counter_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Counter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_COUNTER_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10Asynchronous_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Counter as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10Asynchronous as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ID3D10Debug_Impl: Sized {
    fn SetFeatureMask(&self, mask: u32) -> ::windows::core::Result<()>;
    fn GetFeatureMask(&self) -> u32;
    fn SetPresentPerRenderOpDelay(&self, milliseconds: u32) -> ::windows::core::Result<()>;
    fn GetPresentPerRenderOpDelay(&self) -> u32;
    fn SetSwapChain(&self, pswapchain: &::core::option::Option<super::Dxgi::IDXGISwapChain>) -> ::windows::core::Result<()>;
    fn GetSwapChain(&self) -> ::windows::core::Result<super::Dxgi::IDXGISwapChain>;
    fn Validate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ::windows::core::RuntimeName for ID3D10Debug {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ID3D10Debug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: isize>() -> ID3D10Debug_Vtbl {
        unsafe extern "system" fn SetFeatureMask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFeatureMask(::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFeatureMask()
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPresentPerRenderOpDelay(::core::mem::transmute_copy(&milliseconds)).into()
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPresentPerRenderOpDelay()
        }
        unsafe extern "system" fn SetSwapChain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pswapchain: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSwapChain(::core::mem::transmute(&pswapchain)).into()
        }
        unsafe extern "system" fn GetSwapChain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSwapChain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Validate().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetFeatureMask: SetFeatureMask::<Identity, Impl, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, Impl, OFFSET>,
            SetPresentPerRenderOpDelay: SetPresentPerRenderOpDelay::<Identity, Impl, OFFSET>,
            GetPresentPerRenderOpDelay: GetPresentPerRenderOpDelay::<Identity, Impl, OFFSET>,
            SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Debug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10DepthStencilState_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10DepthStencilState {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10DepthStencilState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10DepthStencilState_Impl, const OFFSET: isize>() -> ID3D10DepthStencilState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10DepthStencilState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10DepthStencilState as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10DepthStencilView_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows::core::RuntimeName for ID3D10DepthStencilView {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10DepthStencilView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10DepthStencilView_Impl, const OFFSET: isize>() -> ID3D10DepthStencilView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10DepthStencilView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10DepthStencilView as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10View as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10Device_Impl: Sized {
    fn VSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>);
    fn PSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>);
    fn PSSetShader(&self, ppixelshader: &::core::option::Option<ID3D10PixelShader>);
    fn PSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>);
    fn VSSetShader(&self, pvertexshader: &::core::option::Option<ID3D10VertexShader>);
    fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(&self, vertexcount: u32, startvertexlocation: u32);
    fn PSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>);
    fn IASetInputLayout(&self, pinputlayout: &::core::option::Option<ID3D10InputLayout>);
    fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::core::option::Option<ID3D10Buffer>, pstrides: *const u32, poffsets: *const u32);
    fn IASetIndexBuffer(&self, pindexbuffer: &::core::option::Option<ID3D10Buffer>, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32);
    fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn GSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>);
    fn GSSetShader(&self, pshader: &::core::option::Option<ID3D10GeometryShader>);
    fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>);
    fn VSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>);
    fn SetPredication(&self, ppredicate: &::core::option::Option<ID3D10Predicate>, predicatevalue: super::super::Foundation::BOOL);
    fn GSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>);
    fn GSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>);
    fn OMSetRenderTargets(&self, numviews: u32, pprendertargetviews: *const ::core::option::Option<ID3D10RenderTargetView>, pdepthstencilview: &::core::option::Option<ID3D10DepthStencilView>);
    fn OMSetBlendState(&self, pblendstate: &::core::option::Option<ID3D10BlendState>, blendfactor: *const f32, samplemask: u32);
    fn OMSetDepthStencilState(&self, pdepthstencilstate: &::core::option::Option<ID3D10DepthStencilState>, stencilref: u32);
    fn SOSetTargets(&self, numbuffers: u32, ppsotargets: *const ::core::option::Option<ID3D10Buffer>, poffsets: *const u32);
    fn DrawAuto(&self);
    fn RSSetState(&self, prasterizerstate: &::core::option::Option<ID3D10RasterizerState>);
    fn RSSetViewports(&self, numviewports: u32, pviewports: *const D3D10_VIEWPORT);
    fn RSSetScissorRects(&self, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn CopySubresourceRegion(&self, pdstresource: &::core::option::Option<ID3D10Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: &::core::option::Option<ID3D10Resource>, srcsubresource: u32, psrcbox: *const D3D10_BOX);
    fn CopyResource(&self, pdstresource: &::core::option::Option<ID3D10Resource>, psrcresource: &::core::option::Option<ID3D10Resource>);
    fn UpdateSubresource(&self, pdstresource: &::core::option::Option<ID3D10Resource>, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn ClearRenderTargetView(&self, prendertargetview: &::core::option::Option<ID3D10RenderTargetView>, colorrgba: *const f32);
    fn ClearDepthStencilView(&self, pdepthstencilview: &::core::option::Option<ID3D10DepthStencilView>, clearflags: u32, depth: f32, stencil: u8);
    fn GenerateMips(&self, pshaderresourceview: &::core::option::Option<ID3D10ShaderResourceView>);
    fn ResolveSubresource(&self, pdstresource: &::core::option::Option<ID3D10Resource>, dstsubresource: u32, psrcresource: &::core::option::Option<ID3D10Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn VSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>);
    fn PSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>);
    fn PSGetShader(&self, pppixelshader: *mut ::core::option::Option<ID3D10PixelShader>);
    fn PSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>);
    fn VSGetShader(&self, ppvertexshader: *mut ::core::option::Option<ID3D10VertexShader>);
    fn PSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>);
    fn IAGetInputLayout(&self, ppinputlayout: *mut ::core::option::Option<ID3D10InputLayout>);
    fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::core::option::Option<ID3D10Buffer>, pstrides: *mut u32, poffsets: *mut u32);
    fn IAGetIndexBuffer(&self, pindexbuffer: *mut ::core::option::Option<ID3D10Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32);
    fn GSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>);
    fn GSGetShader(&self, ppgeometryshader: *mut ::core::option::Option<ID3D10GeometryShader>);
    fn IAGetPrimitiveTopology(&self, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>);
    fn VSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>);
    fn GetPredication(&self, pppredicate: *mut ::core::option::Option<ID3D10Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL);
    fn GSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>);
    fn GSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>);
    fn OMGetRenderTargets(&self, numviews: u32, pprendertargetviews: *mut ::core::option::Option<ID3D10RenderTargetView>, ppdepthstencilview: *mut ::core::option::Option<ID3D10DepthStencilView>);
    fn OMGetBlendState(&self, ppblendstate: *mut ::core::option::Option<ID3D10BlendState>, blendfactor: *mut f32, psamplemask: *mut u32);
    fn OMGetDepthStencilState(&self, ppdepthstencilstate: *mut ::core::option::Option<ID3D10DepthStencilState>, pstencilref: *mut u32);
    fn SOGetTargets(&self, numbuffers: u32, ppsotargets: *mut ::core::option::Option<ID3D10Buffer>, poffsets: *mut u32);
    fn RSGetState(&self, pprasterizerstate: *mut ::core::option::Option<ID3D10RasterizerState>);
    fn RSGetViewports(&self, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT);
    fn RSGetScissorRects(&self, numrects: *mut u32, prects: *mut super::super::Foundation::RECT);
    fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()>;
    fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()>;
    fn GetExceptionMode(&self) -> u32;
    fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const ::windows::core::GUID, pdata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ClearState(&self);
    fn Flush(&self);
    fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Buffer>;
    fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture1D>;
    fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture2D>;
    fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture3D>;
    fn CreateShaderResourceView(&self, presource: &::core::option::Option<ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC) -> ::windows::core::Result<ID3D10ShaderResourceView>;
    fn CreateRenderTargetView(&self, presource: &::core::option::Option<ID3D10Resource>, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC) -> ::windows::core::Result<ID3D10RenderTargetView>;
    fn CreateDepthStencilView(&self, presource: &::core::option::Option<ID3D10Resource>, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC) -> ::windows::core::Result<ID3D10DepthStencilView>;
    fn CreateInputLayout(&self, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10InputLayout>;
    fn CreateVertexShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10VertexShader>;
    fn CreateGeometryShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10GeometryShader>;
    fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32) -> ::windows::core::Result<ID3D10GeometryShader>;
    fn CreatePixelShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10PixelShader>;
    fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC) -> ::windows::core::Result<ID3D10BlendState>;
    fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::Result<ID3D10DepthStencilState>;
    fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC) -> ::windows::core::Result<ID3D10RasterizerState>;
    fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC) -> ::windows::core::Result<ID3D10SamplerState>;
    fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Query>;
    fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Predicate>;
    fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC) -> ::windows::core::Result<ID3D10Counter>;
    fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32>;
    fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32>;
    fn CheckCounterInfo(&self, pcounterinfo: *mut D3D10_COUNTER_INFO);
    fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: *mut u32, szunits: ::windows::core::PSTR, punitslength: *mut u32, szdescription: ::windows::core::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetCreationFlags(&self) -> u32;
    fn OpenSharedResource(&self, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetTextFilterSize(&self, width: u32, height: u32);
    fn GetTextFilterSize(&self, pwidth: *mut u32, pheight: *mut u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for ID3D10Device {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10Device_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>() -> ID3D10Device_Vtbl {
        unsafe extern "system" fn VSSetConstantBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSSetShaderResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSSetShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelshader: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PSSetShader(::core::mem::transmute(&ppixelshader))
        }
        unsafe extern "system" fn PSSetSamplers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSSetShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvertexshader: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VSSetShader(::core::mem::transmute(&pvertexshader))
        }
        unsafe extern "system" fn DrawIndexed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawIndexed(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation))
        }
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Draw(::core::mem::transmute_copy(&vertexcount), ::core::mem::transmute_copy(&startvertexlocation))
        }
        unsafe extern "system" fn PSSetConstantBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IASetInputLayout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputlayout: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IASetInputLayout(::core::mem::transmute(&pinputlayout))
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const *mut ::core::ffi::c_void, pstrides: *const u32, poffsets: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IASetVertexBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IASetIndexBuffer(::core::mem::transmute(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawIndexedInstanced(::core::mem::transmute_copy(&indexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn DrawInstanced<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawInstanced(::core::mem::transmute_copy(&vertexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startvertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn GSSetConstantBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSSetShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GSSetShader(::core::mem::transmute(&pshader))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IASetPrimitiveTopology(::core::mem::transmute_copy(&topology))
        }
        unsafe extern "system" fn VSSetShaderResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSSetSamplers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn SetPredication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicate: *mut ::core::ffi::c_void, predicatevalue: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPredication(::core::mem::transmute(&ppredicate), ::core::mem::transmute_copy(&predicatevalue))
        }
        unsafe extern "system" fn GSSetShaderResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSSetSamplers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const *mut ::core::ffi::c_void, pdepthstencilview: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OMSetRenderTargets(::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute(&pdepthstencilview))
        }
        unsafe extern "system" fn OMSetBlendState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstate: *mut ::core::ffi::c_void, blendfactor: *const f32, samplemask: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OMSetBlendState(::core::mem::transmute(&pblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&samplemask))
        }
        unsafe extern "system" fn OMSetDepthStencilState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilstate: *mut ::core::ffi::c_void, stencilref: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OMSetDepthStencilState(::core::mem::transmute(&pdepthstencilstate), ::core::mem::transmute_copy(&stencilref))
        }
        unsafe extern "system" fn SOSetTargets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const *mut ::core::ffi::c_void, poffsets: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SOSetTargets(::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn DrawAuto<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawAuto()
        }
        unsafe extern "system" fn RSSetState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerstate: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RSSetState(::core::mem::transmute(&prasterizerstate))
        }
        unsafe extern "system" fn RSSetViewports<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D10_VIEWPORT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RSSetViewports(::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RSSetScissorRects(::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn CopySubresourceRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D10_BOX) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopySubresourceRegion(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox))
        }
        unsafe extern "system" fn CopyResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, psrcresource: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyResource(::core::mem::transmute(&pdstresource), ::core::mem::transmute(&psrcresource))
        }
        unsafe extern "system" fn UpdateSubresource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch))
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertargetview: *mut ::core::ffi::c_void, colorrgba: *const f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearRenderTargetView(::core::mem::transmute(&prendertargetview), ::core::mem::transmute_copy(&colorrgba))
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilview: *mut ::core::ffi::c_void, clearflags: u32, depth: f32, stencil: u8) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearDepthStencilView(::core::mem::transmute(&pdepthstencilview), ::core::mem::transmute_copy(&clearflags), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&stencil))
        }
        unsafe extern "system" fn GenerateMips<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderresourceview: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateMips(::core::mem::transmute(&pshaderresourceview))
        }
        unsafe extern "system" fn ResolveSubresource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResolveSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn VSGetConstantBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSGetShaderResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSGetShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppixelshader: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PSGetShader(::core::mem::transmute_copy(&pppixelshader))
        }
        unsafe extern "system" fn PSGetSamplers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSGetShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvertexshader: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VSGetShader(::core::mem::transmute_copy(&ppvertexshader))
        }
        unsafe extern "system" fn PSGetConstantBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IAGetInputLayout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinputlayout: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IAGetInputLayout(::core::mem::transmute_copy(&ppinputlayout))
        }
        unsafe extern "system" fn IAGetVertexBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut *mut ::core::ffi::c_void, pstrides: *mut u32, poffsets: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IAGetVertexBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IAGetIndexBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut *mut ::core::ffi::c_void, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IAGetIndexBuffer(::core::mem::transmute_copy(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn GSGetConstantBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSGetShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GSGetShader(::core::mem::transmute_copy(&ppgeometryshader))
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IAGetPrimitiveTopology(::core::mem::transmute_copy(&ptopology))
        }
        unsafe extern "system" fn VSGetShaderResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSGetSamplers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn GetPredication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppredicate: *mut *mut ::core::ffi::c_void, ppredicatevalue: *mut super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPredication(::core::mem::transmute_copy(&pppredicate), ::core::mem::transmute_copy(&ppredicatevalue))
        }
        unsafe extern "system" fn GSGetShaderResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSGetSamplers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMGetRenderTargets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut *mut ::core::ffi::c_void, ppdepthstencilview: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OMGetRenderTargets(::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview))
        }
        unsafe extern "system" fn OMGetBlendState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblendstate: *mut *mut ::core::ffi::c_void, blendfactor: *mut f32, psamplemask: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OMGetBlendState(::core::mem::transmute_copy(&ppblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&psamplemask))
        }
        unsafe extern "system" fn OMGetDepthStencilState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut *mut ::core::ffi::c_void, pstencilref: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OMGetDepthStencilState(::core::mem::transmute_copy(&ppdepthstencilstate), ::core::mem::transmute_copy(&pstencilref))
        }
        unsafe extern "system" fn SOGetTargets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut *mut ::core::ffi::c_void, poffsets: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SOGetTargets(::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn RSGetState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RSGetState(::core::mem::transmute_copy(&pprasterizerstate))
        }
        unsafe extern "system" fn RSGetViewports<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RSGetViewports(::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSGetScissorRects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RSGetScissorRects(::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceRemovedReason().into()
        }
        unsafe extern "system" fn SetExceptionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExceptionMode(::core::mem::transmute_copy(&raiseflags)).into()
        }
        unsafe extern "system" fn GetExceptionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExceptionMode()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        unsafe extern "system" fn ClearState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearState()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flush()
        }
        unsafe extern "system" fn CreateBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBuffer(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture1D<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture1d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTexture1D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexture1d, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture2D<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture2d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTexture2D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexture2d, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture3D<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture3d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTexture3D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexture3d, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateShaderResourceView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsrview, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRenderTargetView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprtview, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDepthStencilView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdepthstencilview, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInputLayout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInputLayout(::core::mem::transmute_copy(&pinputelementdescs), ::core::mem::transmute_copy(&numelements), ::core::mem::transmute_copy(&pshaderbytecodewithinputsignature), ::core::mem::transmute_copy(&bytecodelength)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinputlayout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateVertexShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvertexshader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGeometryShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgeometryshader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGeometryShaderWithStreamOutput(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute_copy(&psodeclaration), ::core::mem::transmute_copy(&numentries), ::core::mem::transmute_copy(&outputstreamstride)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgeometryshader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePixelShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppixelshader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBlendState(::core::mem::transmute_copy(&pblendstatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppblendstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDepthStencilState(::core::mem::transmute_copy(&pdepthstencildesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdepthstencilstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRasterizerState(::core::mem::transmute_copy(&prasterizerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprasterizerstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSamplerState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSamplerState(::core::mem::transmute_copy(&psamplerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsamplerstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateQuery(::core::mem::transmute_copy(&pquerydesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppquery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePredicate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePredicate(::core::mem::transmute_copy(&ppredicatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppredicate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCounter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCounter(::core::mem::transmute_copy(&pcounterdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcounter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckFormatSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckFormatSupport(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformatsupport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckMultisampleQualityLevels(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&samplecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumqualitylevels, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCounterInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D10_COUNTER_INFO) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckCounterInfo(::core::mem::transmute_copy(&pcounterinfo))
        }
        unsafe extern "system" fn CheckCounter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: *mut u32, szunits: ::windows::core::PSTR, punitslength: *mut u32, szdescription: ::windows::core::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckCounter(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pactivecounters), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&pnamelength), ::core::mem::transmute_copy(&szunits), ::core::mem::transmute_copy(&punitslength), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pdescriptionlength)).into()
        }
        unsafe extern "system" fn GetCreationFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCreationFlags()
        }
        unsafe extern "system" fn OpenSharedResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenSharedResource(::core::mem::transmute_copy(&hresource), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into()
        }
        unsafe extern "system" fn SetTextFilterSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextFilterSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height))
        }
        unsafe extern "system" fn GetTextFilterSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextFilterSize(::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            VSSetConstantBuffers: VSSetConstantBuffers::<Identity, Impl, OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Identity, Impl, OFFSET>,
            PSSetShader: PSSetShader::<Identity, Impl, OFFSET>,
            PSSetSamplers: PSSetSamplers::<Identity, Impl, OFFSET>,
            VSSetShader: VSSetShader::<Identity, Impl, OFFSET>,
            DrawIndexed: DrawIndexed::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
            PSSetConstantBuffers: PSSetConstantBuffers::<Identity, Impl, OFFSET>,
            IASetInputLayout: IASetInputLayout::<Identity, Impl, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, Impl, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, Impl, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, Impl, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, Impl, OFFSET>,
            GSSetConstantBuffers: GSSetConstantBuffers::<Identity, Impl, OFFSET>,
            GSSetShader: GSSetShader::<Identity, Impl, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, Impl, OFFSET>,
            VSSetShaderResources: VSSetShaderResources::<Identity, Impl, OFFSET>,
            VSSetSamplers: VSSetSamplers::<Identity, Impl, OFFSET>,
            SetPredication: SetPredication::<Identity, Impl, OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Identity, Impl, OFFSET>,
            GSSetSamplers: GSSetSamplers::<Identity, Impl, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, Impl, OFFSET>,
            OMSetBlendState: OMSetBlendState::<Identity, Impl, OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Identity, Impl, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, Impl, OFFSET>,
            DrawAuto: DrawAuto::<Identity, Impl, OFFSET>,
            RSSetState: RSSetState::<Identity, Impl, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, Impl, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, Impl, OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Identity, Impl, OFFSET>,
            CopyResource: CopyResource::<Identity, Impl, OFFSET>,
            UpdateSubresource: UpdateSubresource::<Identity, Impl, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, Impl, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, Impl, OFFSET>,
            GenerateMips: GenerateMips::<Identity, Impl, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, Impl, OFFSET>,
            VSGetConstantBuffers: VSGetConstantBuffers::<Identity, Impl, OFFSET>,
            PSGetShaderResources: PSGetShaderResources::<Identity, Impl, OFFSET>,
            PSGetShader: PSGetShader::<Identity, Impl, OFFSET>,
            PSGetSamplers: PSGetSamplers::<Identity, Impl, OFFSET>,
            VSGetShader: VSGetShader::<Identity, Impl, OFFSET>,
            PSGetConstantBuffers: PSGetConstantBuffers::<Identity, Impl, OFFSET>,
            IAGetInputLayout: IAGetInputLayout::<Identity, Impl, OFFSET>,
            IAGetVertexBuffers: IAGetVertexBuffers::<Identity, Impl, OFFSET>,
            IAGetIndexBuffer: IAGetIndexBuffer::<Identity, Impl, OFFSET>,
            GSGetConstantBuffers: GSGetConstantBuffers::<Identity, Impl, OFFSET>,
            GSGetShader: GSGetShader::<Identity, Impl, OFFSET>,
            IAGetPrimitiveTopology: IAGetPrimitiveTopology::<Identity, Impl, OFFSET>,
            VSGetShaderResources: VSGetShaderResources::<Identity, Impl, OFFSET>,
            VSGetSamplers: VSGetSamplers::<Identity, Impl, OFFSET>,
            GetPredication: GetPredication::<Identity, Impl, OFFSET>,
            GSGetShaderResources: GSGetShaderResources::<Identity, Impl, OFFSET>,
            GSGetSamplers: GSGetSamplers::<Identity, Impl, OFFSET>,
            OMGetRenderTargets: OMGetRenderTargets::<Identity, Impl, OFFSET>,
            OMGetBlendState: OMGetBlendState::<Identity, Impl, OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Identity, Impl, OFFSET>,
            SOGetTargets: SOGetTargets::<Identity, Impl, OFFSET>,
            RSGetState: RSGetState::<Identity, Impl, OFFSET>,
            RSGetViewports: RSGetViewports::<Identity, Impl, OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Identity, Impl, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, Impl, OFFSET>,
            SetExceptionMode: SetExceptionMode::<Identity, Impl, OFFSET>,
            GetExceptionMode: GetExceptionMode::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            ClearState: ClearState::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            CreateBuffer: CreateBuffer::<Identity, Impl, OFFSET>,
            CreateTexture1D: CreateTexture1D::<Identity, Impl, OFFSET>,
            CreateTexture2D: CreateTexture2D::<Identity, Impl, OFFSET>,
            CreateTexture3D: CreateTexture3D::<Identity, Impl, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, Impl, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, Impl, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, Impl, OFFSET>,
            CreateInputLayout: CreateInputLayout::<Identity, Impl, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, Impl, OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Identity, Impl, OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<Identity, Impl, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, Impl, OFFSET>,
            CreateBlendState: CreateBlendState::<Identity, Impl, OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Identity, Impl, OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Identity, Impl, OFFSET>,
            CreateSamplerState: CreateSamplerState::<Identity, Impl, OFFSET>,
            CreateQuery: CreateQuery::<Identity, Impl, OFFSET>,
            CreatePredicate: CreatePredicate::<Identity, Impl, OFFSET>,
            CreateCounter: CreateCounter::<Identity, Impl, OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Identity, Impl, OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Identity, Impl, OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Identity, Impl, OFFSET>,
            CheckCounter: CheckCounter::<Identity, Impl, OFFSET>,
            GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET>,
            OpenSharedResource: OpenSharedResource::<Identity, Impl, OFFSET>,
            SetTextFilterSize: SetTextFilterSize::<Identity, Impl, OFFSET>,
            GetTextFilterSize: GetTextFilterSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10Device1_Impl: Sized + ID3D10Device_Impl {
    fn CreateShaderResourceView1(&self, presource: &::core::option::Option<ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1) -> ::windows::core::Result<ID3D10ShaderResourceView1>;
    fn CreateBlendState1(&self, pblendstatedesc: *const D3D10_BLEND_DESC1) -> ::windows::core::Result<ID3D10BlendState1>;
    fn GetFeatureLevel(&self) -> D3D10_FEATURE_LEVEL1;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for ID3D10Device1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10Device1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device1_Impl, const OFFSET: isize>() -> ID3D10Device1_Vtbl {
        unsafe extern "system" fn CreateShaderResourceView1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateShaderResourceView1(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsrview, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBlendState1(::core::mem::transmute_copy(&pblendstatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppblendstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D10_FEATURE_LEVEL1 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFeatureLevel()
        }
        Self {
            base__: ID3D10Device_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateShaderResourceView1: CreateShaderResourceView1::<Identity, Impl, OFFSET>,
            CreateBlendState1: CreateBlendState1::<Identity, Impl, OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Device1 as ::windows::core::Interface>::IID || iid == &<ID3D10Device as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10DeviceChild_Impl: Sized {
    fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>);
    fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const ::windows::core::GUID, pdata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ID3D10DeviceChild {}
impl ID3D10DeviceChild_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10DeviceChild_Impl, const OFFSET: isize>() -> ID3D10DeviceChild_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDevice(::core::mem::transmute_copy(&ppdevice))
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10Effect_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn IsPool(&self) -> super::super::Foundation::BOOL;
    fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device>;
    fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_DESC>;
    fn GetConstantBufferByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn GetVariableByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetVariableByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetVariableBySemantic(&self, semantic: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetTechniqueByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectTechnique>;
    fn GetTechniqueByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectTechnique>;
    fn Optimize(&self) -> ::windows::core::Result<()>;
    fn IsOptimized(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10Effect {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10Effect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>() -> ID3D10Effect_Vtbl {
        unsafe extern "system" fn IsValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsValid()
        }
        unsafe extern "system" fn IsPool<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsPool()
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConstantBufferByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConstantBufferByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVariableByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVariableByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetVariableBySemantic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVariableBySemantic(::core::mem::transmute(&semantic))
        }
        unsafe extern "system" fn GetTechniqueByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectTechnique> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTechniqueByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetTechniqueByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectTechnique> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTechniqueByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn Optimize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Optimize().into()
        }
        unsafe extern "system" fn IsOptimized<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsOptimized()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsValid: IsValid::<Identity, Impl, OFFSET>,
            IsPool: IsPool::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetVariableBySemantic: GetVariableBySemantic::<Identity, Impl, OFFSET>,
            GetTechniqueByIndex: GetTechniqueByIndex::<Identity, Impl, OFFSET>,
            GetTechniqueByName: GetTechniqueByName::<Identity, Impl, OFFSET>,
            Optimize: Optimize::<Identity, Impl, OFFSET>,
            IsOptimized: IsOptimized::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Effect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectBlendVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetBlendState(&self, index: u32) -> ::windows::core::Result<ID3D10BlendState>;
    fn GetBackingStore(&self, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectBlendVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectBlendVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectBlendVariable_Impl, const OFFSET: isize>() -> ID3D10EffectBlendVariable_Vtbl {
        unsafe extern "system" fn GetBlendState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectBlendVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppblendstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBlendState(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppblendstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectBlendVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBackingStore(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pblenddesc)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetBlendState: GetBlendState::<Identity, Impl, OFFSET>,
            GetBackingStore: GetBackingStore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectBlendVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectConstantBuffer_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetConstantBuffer(&self, pconstantbuffer: &::core::option::Option<ID3D10Buffer>) -> ::windows::core::Result<()>;
    fn GetConstantBuffer(&self) -> ::windows::core::Result<ID3D10Buffer>;
    fn SetTextureBuffer(&self, ptexturebuffer: &::core::option::Option<ID3D10ShaderResourceView>) -> ::windows::core::Result<()>;
    fn GetTextureBuffer(&self) -> ::windows::core::Result<ID3D10ShaderResourceView>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectConstantBuffer {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectConstantBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectConstantBuffer_Impl, const OFFSET: isize>() -> ID3D10EffectConstantBuffer_Vtbl {
        unsafe extern "system" fn SetConstantBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstantbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConstantBuffer(::core::mem::transmute(&pconstantbuffer)).into()
        }
        unsafe extern "system" fn GetConstantBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconstantbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConstantBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconstantbuffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextureBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptexturebuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextureBuffer(::core::mem::transmute(&ptexturebuffer)).into()
        }
        unsafe extern "system" fn GetTextureBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptexturebuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTextureBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexturebuffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetConstantBuffer: SetConstantBuffer::<Identity, Impl, OFFSET>,
            GetConstantBuffer: GetConstantBuffer::<Identity, Impl, OFFSET>,
            SetTextureBuffer: SetTextureBuffer::<Identity, Impl, OFFSET>,
            GetTextureBuffer: GetTextureBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectConstantBuffer as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectDepthStencilVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetDepthStencilState(&self, index: u32) -> ::windows::core::Result<ID3D10DepthStencilState>;
    fn GetBackingStore(&self, index: u32) -> ::windows::core::Result<D3D10_DEPTH_STENCIL_DESC>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectDepthStencilVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectDepthStencilVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectDepthStencilVariable_Impl, const OFFSET: isize>() -> ID3D10EffectDepthStencilVariable_Vtbl {
        unsafe extern "system" fn GetDepthStencilState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectDepthStencilVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppdepthstencilstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDepthStencilState(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdepthstencilstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectDepthStencilVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackingStore(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdepthstencildesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDepthStencilState: GetDepthStencilState::<Identity, Impl, OFFSET>,
            GetBackingStore: GetBackingStore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectDepthStencilVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectDepthStencilViewVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetDepthStencil(&self, presource: &::core::option::Option<ID3D10DepthStencilView>) -> ::windows::core::Result<()>;
    fn GetDepthStencil(&self) -> ::windows::core::Result<ID3D10DepthStencilView>;
    fn SetDepthStencilArray(&self, ppresources: *const ::core::option::Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetDepthStencilArray(&self, ppresources: *mut ::core::option::Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectDepthStencilViewVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectDepthStencilViewVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectDepthStencilViewVariable_Impl, const OFFSET: isize>() -> ID3D10EffectDepthStencilViewVariable_Vtbl {
        unsafe extern "system" fn SetDepthStencil<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectDepthStencilViewVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDepthStencil(::core::mem::transmute(&presource)).into()
        }
        unsafe extern "system" fn GetDepthStencil<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectDepthStencilViewVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDepthStencil() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepthStencilArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectDepthStencilViewVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDepthStencilArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetDepthStencilArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectDepthStencilViewVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDepthStencilArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetDepthStencil: SetDepthStencil::<Identity, Impl, OFFSET>,
            GetDepthStencil: GetDepthStencil::<Identity, Impl, OFFSET>,
            SetDepthStencilArray: SetDepthStencilArray::<Identity, Impl, OFFSET>,
            GetDepthStencilArray: GetDepthStencilArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectDepthStencilViewVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectMatrixVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetMatrix(&self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn GetMatrix(&self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn SetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn SetMatrixTranspose(&self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn GetMatrixTranspose(&self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn SetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectMatrixVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectMatrixVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectMatrixVariable_Impl, const OFFSET: isize>() -> ID3D10EffectMatrixVariable_Vtbl {
        unsafe extern "system" fn SetMatrix<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectMatrixVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMatrix(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetMatrix<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectMatrixVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMatrix(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetMatrixArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectMatrixVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMatrixArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetMatrixArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectMatrixVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMatrixArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetMatrixTranspose<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectMatrixVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMatrixTranspose(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetMatrixTranspose<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectMatrixVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMatrixTranspose(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetMatrixTransposeArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectMatrixVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMatrixTransposeArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetMatrixTransposeArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectMatrixVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMatrixTransposeArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMatrix: SetMatrix::<Identity, Impl, OFFSET>,
            GetMatrix: GetMatrix::<Identity, Impl, OFFSET>,
            SetMatrixArray: SetMatrixArray::<Identity, Impl, OFFSET>,
            GetMatrixArray: GetMatrixArray::<Identity, Impl, OFFSET>,
            SetMatrixTranspose: SetMatrixTranspose::<Identity, Impl, OFFSET>,
            GetMatrixTranspose: GetMatrixTranspose::<Identity, Impl, OFFSET>,
            SetMatrixTransposeArray: SetMatrixTransposeArray::<Identity, Impl, OFFSET>,
            GetMatrixTransposeArray: GetMatrixTransposeArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectMatrixVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectPass_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_PASS_DESC) -> ::windows::core::Result<()>;
    fn GetVertexShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()>;
    fn GetGeometryShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()>;
    fn GetPixelShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn Apply(&self, flags: u32) -> ::windows::core::Result<()>;
    fn ComputeStateBlockMask(&self) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectPass {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectPass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPass_Impl, const OFFSET: isize>() -> ID3D10EffectPass_Vtbl {
        unsafe extern "system" fn IsValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsValid()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVertexShaderDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVertexShaderDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetGeometryShaderDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGeometryShaderDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetPixelShaderDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelShaderDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAnnotationByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAnnotationByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn Apply<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Apply(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn ComputeStateBlockMask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputeStateBlockMask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstateblockmask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            IsValid: IsValid::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetVertexShaderDesc: GetVertexShaderDesc::<Identity, Impl, OFFSET>,
            GetGeometryShaderDesc: GetGeometryShaderDesc::<Identity, Impl, OFFSET>,
            GetPixelShaderDesc: GetPixelShaderDesc::<Identity, Impl, OFFSET>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity, Impl, OFFSET>,
            GetAnnotationByName: GetAnnotationByName::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectPass as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10EffectPool_Impl: Sized {
    fn AsEffect(&self) -> ::core::option::Option<ID3D10Effect>;
}
impl ::windows::core::RuntimeName for ID3D10EffectPool {}
impl ID3D10EffectPool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPool_Impl, const OFFSET: isize>() -> ID3D10EffectPool_Vtbl {
        unsafe extern "system" fn AsEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10Effect> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsEffect()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AsEffect: AsEffect::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectPool as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectRasterizerVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetRasterizerState(&self, index: u32) -> ::windows::core::Result<ID3D10RasterizerState>;
    fn GetBackingStore(&self, index: u32) -> ::windows::core::Result<D3D10_RASTERIZER_DESC>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectRasterizerVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectRasterizerVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectRasterizerVariable_Impl, const OFFSET: isize>() -> ID3D10EffectRasterizerVariable_Vtbl {
        unsafe extern "system" fn GetRasterizerState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectRasterizerVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pprasterizerstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRasterizerState(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprasterizerstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectRasterizerVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackingStore(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prasterizerdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRasterizerState: GetRasterizerState::<Identity, Impl, OFFSET>,
            GetBackingStore: GetBackingStore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectRasterizerVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectRenderTargetViewVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetRenderTarget(&self, presource: &::core::option::Option<ID3D10RenderTargetView>) -> ::windows::core::Result<()>;
    fn GetRenderTarget(&self) -> ::windows::core::Result<ID3D10RenderTargetView>;
    fn SetRenderTargetArray(&self, ppresources: *const ::core::option::Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetRenderTargetArray(&self, ppresources: *mut ::core::option::Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectRenderTargetViewVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectRenderTargetViewVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectRenderTargetViewVariable_Impl, const OFFSET: isize>() -> ID3D10EffectRenderTargetViewVariable_Vtbl {
        unsafe extern "system" fn SetRenderTarget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectRenderTargetViewVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRenderTarget(::core::mem::transmute(&presource)).into()
        }
        unsafe extern "system" fn GetRenderTarget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectRenderTargetViewVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRenderTarget() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderTargetArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectRenderTargetViewVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRenderTargetArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetRenderTargetArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectRenderTargetViewVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRenderTargetArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRenderTarget: SetRenderTarget::<Identity, Impl, OFFSET>,
            GetRenderTarget: GetRenderTarget::<Identity, Impl, OFFSET>,
            SetRenderTargetArray: SetRenderTargetArray::<Identity, Impl, OFFSET>,
            GetRenderTargetArray: GetRenderTargetArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectRenderTargetViewVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectSamplerVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetSampler(&self, index: u32) -> ::windows::core::Result<ID3D10SamplerState>;
    fn GetBackingStore(&self, index: u32) -> ::windows::core::Result<D3D10_SAMPLER_DESC>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectSamplerVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectSamplerVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectSamplerVariable_Impl, const OFFSET: isize>() -> ID3D10EffectSamplerVariable_Vtbl {
        unsafe extern "system" fn GetSampler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectSamplerVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppsampler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSampler(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsampler, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectSamplerVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackingStore(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psamplerdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSampler: GetSampler::<Identity, Impl, OFFSET>,
            GetBackingStore: GetBackingStore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectSamplerVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectScalarVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetFloat(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetFloat(&self) -> ::windows::core::Result<f32>;
    fn SetFloatArray(&self, pdata: *const f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetFloatArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn SetInt(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetInt(&self) -> ::windows::core::Result<i32>;
    fn SetIntArray(&self, pdata: *const i32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetIntArray(&self, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn SetBool(&self, value: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBool(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetBoolArray(&self, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetBoolArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectScalarVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectScalarVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>() -> ID3D10EffectScalarVariable_Vtbl {
        unsafe extern "system" fn SetFloat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFloat(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetFloat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFloat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFloatArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFloatArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetFloatArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFloatArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetInt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInt(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetInt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInt() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIntArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetIntArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIntArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetBool<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBool(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBool<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBool() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBoolArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetBoolArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectScalarVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBoolArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetFloat: SetFloat::<Identity, Impl, OFFSET>,
            GetFloat: GetFloat::<Identity, Impl, OFFSET>,
            SetFloatArray: SetFloatArray::<Identity, Impl, OFFSET>,
            GetFloatArray: GetFloatArray::<Identity, Impl, OFFSET>,
            SetInt: SetInt::<Identity, Impl, OFFSET>,
            GetInt: GetInt::<Identity, Impl, OFFSET>,
            SetIntArray: SetIntArray::<Identity, Impl, OFFSET>,
            GetIntArray: GetIntArray::<Identity, Impl, OFFSET>,
            SetBool: SetBool::<Identity, Impl, OFFSET>,
            GetBool: GetBool::<Identity, Impl, OFFSET>,
            SetBoolArray: SetBoolArray::<Identity, Impl, OFFSET>,
            GetBoolArray: GetBoolArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectScalarVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectShaderResourceVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetResource(&self, presource: &::core::option::Option<ID3D10ShaderResourceView>) -> ::windows::core::Result<()>;
    fn GetResource(&self) -> ::windows::core::Result<ID3D10ShaderResourceView>;
    fn SetResourceArray(&self, ppresources: *const ::core::option::Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetResourceArray(&self, ppresources: *mut ::core::option::Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectShaderResourceVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectShaderResourceVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderResourceVariable_Impl, const OFFSET: isize>() -> ID3D10EffectShaderResourceVariable_Vtbl {
        unsafe extern "system" fn SetResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderResourceVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetResource(::core::mem::transmute(&presource)).into()
        }
        unsafe extern "system" fn GetResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderResourceVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourceArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderResourceVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetResourceArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetResourceArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderResourceVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResourceArray(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetResource: SetResource::<Identity, Impl, OFFSET>,
            GetResource: GetResource::<Identity, Impl, OFFSET>,
            SetResourceArray: SetResourceArray::<Identity, Impl, OFFSET>,
            GetResourceArray: GetResourceArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectShaderResourceVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10EffectShaderVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetShaderDesc(&self, shaderindex: u32) -> ::windows::core::Result<D3D10_EFFECT_SHADER_DESC>;
    fn GetVertexShader(&self, shaderindex: u32) -> ::windows::core::Result<ID3D10VertexShader>;
    fn GetGeometryShader(&self, shaderindex: u32) -> ::windows::core::Result<ID3D10GeometryShader>;
    fn GetPixelShader(&self, shaderindex: u32) -> ::windows::core::Result<ID3D10PixelShader>;
    fn GetInputSignatureElementDesc(&self, shaderindex: u32, element: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
    fn GetOutputSignatureElementDesc(&self, shaderindex: u32, element: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::windows::core::RuntimeName for ID3D10EffectShaderVariable {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10EffectShaderVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderVariable_Impl, const OFFSET: isize>() -> ID3D10EffectShaderVariable_Vtbl {
        unsafe extern "system" fn GetShaderDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetShaderDesc(::core::mem::transmute_copy(&shaderindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVertexShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppvs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVertexShader(::core::mem::transmute_copy(&shaderindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppgs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGeometryShader(::core::mem::transmute_copy(&shaderindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppps: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPixelShader(::core::mem::transmute_copy(&shaderindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppps, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSignatureElementDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputSignatureElementDesc(::core::mem::transmute_copy(&shaderindex), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputSignatureElementDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectShaderVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputSignatureElementDesc(::core::mem::transmute_copy(&shaderindex), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetShaderDesc: GetShaderDesc::<Identity, Impl, OFFSET>,
            GetVertexShader: GetVertexShader::<Identity, Impl, OFFSET>,
            GetGeometryShader: GetGeometryShader::<Identity, Impl, OFFSET>,
            GetPixelShader: GetPixelShader::<Identity, Impl, OFFSET>,
            GetInputSignatureElementDesc: GetInputSignatureElementDesc::<Identity, Impl, OFFSET>,
            GetOutputSignatureElementDesc: GetOutputSignatureElementDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectShaderVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectStringVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetString(&self) -> ::windows::core::Result<::windows::core::PSTR>;
    fn GetStringArray(&self, ppstrings: *mut ::windows::core::PSTR, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectStringVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectStringVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectStringVariable_Impl, const OFFSET: isize>() -> ID3D10EffectStringVariable_Vtbl {
        unsafe extern "system" fn GetString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectStringVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstring: *mut ::windows::core::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectStringVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstrings: *mut ::windows::core::PSTR, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStringArray(::core::mem::transmute_copy(&ppstrings), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetStringArray: GetStringArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectStringVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectTechnique_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows::core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetPassByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectPass>;
    fn GetPassByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectPass>;
    fn ComputeStateBlockMask(&self) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectTechnique {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectTechnique_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectTechnique_Impl, const OFFSET: isize>() -> ID3D10EffectTechnique_Vtbl {
        unsafe extern "system" fn IsValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectTechnique_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsValid()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectTechnique_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectTechnique_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAnnotationByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectTechnique_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAnnotationByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetPassByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectTechnique_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectPass> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPassByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetPassByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectTechnique_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectPass> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPassByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn ComputeStateBlockMask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectTechnique_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputeStateBlockMask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstateblockmask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            IsValid: IsValid::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity, Impl, OFFSET>,
            GetAnnotationByName: GetAnnotationByName::<Identity, Impl, OFFSET>,
            GetPassByIndex: GetPassByIndex::<Identity, Impl, OFFSET>,
            GetPassByName: GetPassByName::<Identity, Impl, OFFSET>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectTechnique as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10EffectType_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows::core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectType>;
    fn GetMemberTypeByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectType>;
    fn GetMemberTypeBySemantic(&self, semantic: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectType>;
    fn GetMemberName(&self, index: u32) -> ::windows::core::PSTR;
    fn GetMemberSemantic(&self, index: u32) -> ::windows::core::PSTR;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::windows::core::RuntimeName for ID3D10EffectType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10EffectType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectType_Impl, const OFFSET: isize>() -> ID3D10EffectType_Vtbl {
        unsafe extern "system" fn IsValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsValid()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberTypeByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberTypeByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberTypeBySemantic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberTypeBySemantic(::core::mem::transmute(&semantic))
        }
        unsafe extern "system" fn GetMemberName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::PSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberName(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberSemantic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::PSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberSemantic(::core::mem::transmute_copy(&index))
        }
        Self {
            IsValid: IsValid::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Identity, Impl, OFFSET>,
            GetMemberTypeByName: GetMemberTypeByName::<Identity, Impl, OFFSET>,
            GetMemberTypeBySemantic: GetMemberTypeBySemantic::<Identity, Impl, OFFSET>,
            GetMemberName: GetMemberName::<Identity, Impl, OFFSET>,
            GetMemberSemantic: GetMemberSemantic::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectType as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectVariable_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetType(&self) -> ::core::option::Option<ID3D10EffectType>;
    fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC>;
    fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()>;
    fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>() -> ID3D10EffectVariable_Vtbl {
        unsafe extern "system" fn IsValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsValid()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetType()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAnnotationByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAnnotationByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberBySemantic(::core::mem::transmute(&semantic))
        }
        unsafe extern "system" fn GetElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetElement(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParentConstantBuffer()
        }
        unsafe extern "system" fn AsScalar<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsScalar()
        }
        unsafe extern "system" fn AsVector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsVector()
        }
        unsafe extern "system" fn AsMatrix<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsMatrix()
        }
        unsafe extern "system" fn AsString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsString()
        }
        unsafe extern "system" fn AsShaderResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsShaderResource()
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsRenderTargetView()
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsDepthStencilView()
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsConstantBuffer()
        }
        unsafe extern "system" fn AsShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsShader()
        }
        unsafe extern "system" fn AsBlend<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsBlend()
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsDepthStencil()
        }
        unsafe extern "system" fn AsRasterizer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsRasterizer()
        }
        unsafe extern "system" fn AsSampler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsSampler()
        }
        unsafe extern "system" fn SetRawValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRawValue(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&bytecount)).into()
        }
        unsafe extern "system" fn GetRawValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRawValue(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&bytecount)).into()
        }
        Self {
            IsValid: IsValid::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity, Impl, OFFSET>,
            GetAnnotationByName: GetAnnotationByName::<Identity, Impl, OFFSET>,
            GetMemberByIndex: GetMemberByIndex::<Identity, Impl, OFFSET>,
            GetMemberByName: GetMemberByName::<Identity, Impl, OFFSET>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity, Impl, OFFSET>,
            AsScalar: AsScalar::<Identity, Impl, OFFSET>,
            AsVector: AsVector::<Identity, Impl, OFFSET>,
            AsMatrix: AsMatrix::<Identity, Impl, OFFSET>,
            AsString: AsString::<Identity, Impl, OFFSET>,
            AsShaderResource: AsShaderResource::<Identity, Impl, OFFSET>,
            AsRenderTargetView: AsRenderTargetView::<Identity, Impl, OFFSET>,
            AsDepthStencilView: AsDepthStencilView::<Identity, Impl, OFFSET>,
            AsConstantBuffer: AsConstantBuffer::<Identity, Impl, OFFSET>,
            AsShader: AsShader::<Identity, Impl, OFFSET>,
            AsBlend: AsBlend::<Identity, Impl, OFFSET>,
            AsDepthStencil: AsDepthStencil::<Identity, Impl, OFFSET>,
            AsRasterizer: AsRasterizer::<Identity, Impl, OFFSET>,
            AsSampler: AsSampler::<Identity, Impl, OFFSET>,
            SetRawValue: SetRawValue::<Identity, Impl, OFFSET>,
            GetRawValue: GetRawValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectVectorVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetIntVector(&self, pdata: *mut i32) -> ::windows::core::Result<()>;
    fn SetFloatVector(&self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn GetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetIntVector(&self, pdata: *mut i32) -> ::windows::core::Result<()>;
    fn GetFloatVector(&self, pdata: *mut f32) -> ::windows::core::Result<()>;
    fn SetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn SetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn SetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn GetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10EffectVectorVariable {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVectorVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>() -> ID3D10EffectVectorVariable_Vtbl {
        unsafe extern "system" fn SetBoolVector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBoolVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetIntVector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIntVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetFloatVector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFloatVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetBoolVector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBoolVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetIntVector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIntVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetFloatVector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFloatVector(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetBoolVectorArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBoolVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetIntVectorArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIntVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetFloatVectorArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFloatVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetBoolVectorArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBoolVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetIntVectorArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIntVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetFloatVectorArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10EffectVectorVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFloatVectorArray(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBoolVector: SetBoolVector::<Identity, Impl, OFFSET>,
            SetIntVector: SetIntVector::<Identity, Impl, OFFSET>,
            SetFloatVector: SetFloatVector::<Identity, Impl, OFFSET>,
            GetBoolVector: GetBoolVector::<Identity, Impl, OFFSET>,
            GetIntVector: GetIntVector::<Identity, Impl, OFFSET>,
            GetFloatVector: GetFloatVector::<Identity, Impl, OFFSET>,
            SetBoolVectorArray: SetBoolVectorArray::<Identity, Impl, OFFSET>,
            SetIntVectorArray: SetIntVectorArray::<Identity, Impl, OFFSET>,
            SetFloatVectorArray: SetFloatVectorArray::<Identity, Impl, OFFSET>,
            GetBoolVectorArray: GetBoolVectorArray::<Identity, Impl, OFFSET>,
            GetIntVectorArray: GetIntVectorArray::<Identity, Impl, OFFSET>,
            GetFloatVectorArray: GetFloatVectorArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10EffectVectorVariable as ::windows::core::Interface>::IID || iid == &<ID3D10EffectVariable as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10GeometryShader_Impl: Sized + ID3D10DeviceChild_Impl {}
impl ::windows::core::RuntimeName for ID3D10GeometryShader {}
impl ID3D10GeometryShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10GeometryShader_Impl, const OFFSET: isize>() -> ID3D10GeometryShader_Vtbl {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10GeometryShader as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10InfoQueue_Impl: Sized {
    fn SetMessageCountLimit(&self, messagecountlimit: u64) -> ::windows::core::Result<()>;
    fn ClearStoredMessages(&self);
    fn GetMessage(&self, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(&self) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&self) -> u64;
    fn GetNumStoredMessages(&self) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64;
    fn GetMessageCountLimit(&self) -> u64;
    fn AddStorageFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetStorageFilter(&self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearStorageFilter(&self);
    fn PushEmptyStorageFilter(&self) -> ::windows::core::Result<()>;
    fn PushCopyOfStorageFilter(&self) -> ::windows::core::Result<()>;
    fn PushStorageFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopStorageFilter(&self);
    fn GetStorageFilterStackSize(&self) -> u32;
    fn AddRetrievalFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetRetrievalFilter(&self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearRetrievalFilter(&self);
    fn PushEmptyRetrievalFilter(&self) -> ::windows::core::Result<()>;
    fn PushCopyOfRetrievalFilter(&self) -> ::windows::core::Result<()>;
    fn PushRetrievalFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopRetrievalFilter(&self);
    fn GetRetrievalFilterStackSize(&self) -> u32;
    fn AddMessage(&self, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn AddApplicationMessage(&self, severity: D3D10_MESSAGE_SEVERITY, pdescription: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn SetBreakOnCategory(&self, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnSeverity(&self, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnID(&self, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBreakOnCategory(&self, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&self, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&self, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&self, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10InfoQueue {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10InfoQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>() -> ID3D10InfoQueue_Vtbl {
        unsafe extern "system" fn SetMessageCountLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMessageCountLimit(::core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearStoredMessages()
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMessage(::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumMessagesAllowedByStorageFilter()
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumMessagesDeniedByStorageFilter()
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumStoredMessages()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumStoredMessagesAllowedByRetrievalFilter()
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumMessagesDiscardedByMessageCountLimit()
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMessageCountLimit()
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStorageFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStorageFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearStorageFilter()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushEmptyStorageFilter().into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushCopyOfStorageFilter().into()
        }
        unsafe extern "system" fn PushStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushStorageFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopStorageFilter()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStorageFilterStackSize()
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRetrievalFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRetrievalFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearRetrievalFilter()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushEmptyRetrievalFilter().into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushCopyOfRetrievalFilter().into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushRetrievalFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopRetrievalFilter()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRetrievalFilterStackSize()
        }
        unsafe extern "system" fn AddMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMessage(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddApplicationMessage(::core::mem::transmute_copy(&severity), ::core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBreakOnCategory(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBreakOnSeverity(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBreakOnID(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBreakOnCategory(::core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBreakOnSeverity(::core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBreakOnID(::core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMuteDebugOutput(::core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMuteDebugOutput()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, Impl, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, Impl, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, Impl, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, Impl, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, Impl, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, Impl, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, Impl, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, Impl, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, Impl, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, Impl, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, Impl, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, Impl, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, Impl, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, Impl, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, Impl, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, Impl, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, Impl, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, Impl, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, Impl, OFFSET>,
            AddMessage: AddMessage::<Identity, Impl, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, Impl, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, Impl, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, Impl, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, Impl, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, Impl, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, Impl, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, Impl, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, Impl, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10InfoQueue as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10InputLayout_Impl: Sized + ID3D10DeviceChild_Impl {}
impl ::windows::core::RuntimeName for ID3D10InputLayout {}
impl ID3D10InputLayout_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10InputLayout_Impl, const OFFSET: isize>() -> ID3D10InputLayout_Vtbl {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10InputLayout as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10Multithread_Impl: Sized {
    fn Enter(&self);
    fn Leave(&self);
    fn SetMultithreadProtected(&self, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetMultithreadProtected(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10Multithread {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10Multithread_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Multithread_Impl, const OFFSET: isize>() -> ID3D10Multithread_Vtbl {
        unsafe extern "system" fn Enter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enter()
        }
        unsafe extern "system" fn Leave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Leave()
        }
        unsafe extern "system" fn SetMultithreadProtected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMultithreadProtected(::core::mem::transmute_copy(&bmtprotect))
        }
        unsafe extern "system" fn GetMultithreadProtected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMultithreadProtected()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Enter: Enter::<Identity, Impl, OFFSET>,
            Leave: Leave::<Identity, Impl, OFFSET>,
            SetMultithreadProtected: SetMultithreadProtected::<Identity, Impl, OFFSET>,
            GetMultithreadProtected: GetMultithreadProtected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Multithread as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10PixelShader_Impl: Sized + ID3D10DeviceChild_Impl {}
impl ::windows::core::RuntimeName for ID3D10PixelShader {}
impl ID3D10PixelShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10PixelShader_Impl, const OFFSET: isize>() -> ID3D10PixelShader_Vtbl {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10PixelShader as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10Predicate_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10Asynchronous_Impl + ID3D10Query_Impl {}
impl ::windows::core::RuntimeName for ID3D10Predicate {}
impl ID3D10Predicate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Predicate_Impl, const OFFSET: isize>() -> ID3D10Predicate_Vtbl {
        Self { base__: ID3D10Query_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Predicate as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10Asynchronous as ::windows::core::Interface>::IID || iid == &<ID3D10Query as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10Query_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_QUERY_DESC);
}
impl ::windows::core::RuntimeName for ID3D10Query {}
impl ID3D10Query_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Query_Impl, const OFFSET: isize>() -> ID3D10Query_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Query_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_QUERY_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10Asynchronous_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Query as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10Asynchronous as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10RasterizerState_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_RASTERIZER_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10RasterizerState {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10RasterizerState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10RasterizerState_Impl, const OFFSET: isize>() -> ID3D10RasterizerState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10RasterizerState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RASTERIZER_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10RasterizerState as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10RenderTargetView_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows::core::RuntimeName for ID3D10RenderTargetView {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10RenderTargetView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10RenderTargetView_Impl, const OFFSET: isize>() -> ID3D10RenderTargetView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10RenderTargetView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10RenderTargetView as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10View as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10Resource_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION);
    fn SetEvictionPriority(&self, evictionpriority: u32);
    fn GetEvictionPriority(&self) -> u32;
}
impl ::windows::core::RuntimeName for ID3D10Resource {}
impl ID3D10Resource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Resource_Impl, const OFFSET: isize>() -> ID3D10Resource_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetType(::core::mem::transmute_copy(&rtype))
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEvictionPriority(::core::mem::transmute_copy(&evictionpriority))
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEvictionPriority()
        }
        Self {
            base__: ID3D10DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, Impl, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Resource as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10SamplerState_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SAMPLER_DESC);
}
impl ::windows::core::RuntimeName for ID3D10SamplerState {}
impl ID3D10SamplerState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10SamplerState_Impl, const OFFSET: isize>() -> ID3D10SamplerState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10SamplerState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SAMPLER_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10SamplerState as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflection_Impl: Sized {
    fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_DESC>;
    fn GetConstantBufferByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC>;
    fn GetInputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
    fn GetOutputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows::core::RuntimeName for ID3D10ShaderReflection {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: isize>() -> ID3D10ShaderReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConstantBufferByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConstantBufferByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, Impl, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10ShaderReflection1_Impl: Sized {
    fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_DESC>;
    fn GetConstantBufferByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC>;
    fn GetInputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
    fn GetOutputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC>;
    fn GetVariableByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &::windows::core::PCSTR) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC>;
    fn GetMovInstructionCount(&self) -> ::windows::core::Result<u32>;
    fn GetMovcInstructionCount(&self) -> ::windows::core::Result<u32>;
    fn GetConversionInstructionCount(&self) -> ::windows::core::Result<u32>;
    fn GetBitwiseInstructionCount(&self) -> ::windows::core::Result<u32>;
    fn GetGSInputPrimitive(&self) -> ::windows::core::Result<super::Direct3D::D3D_PRIMITIVE>;
    fn IsLevel9Shader(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsSampleFrequencyShader(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::windows::core::RuntimeName for ID3D10ShaderReflection1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10ShaderReflection1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>() -> ID3D10ShaderReflection1_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConstantBufferByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConstantBufferByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVariableByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResourceBindingDescByName(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMovInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMovcInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConversionInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBitwiseInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprim: *mut super::Direct3D::D3D_PRIMITIVE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGSInputPrimitive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprim, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLevel9Shader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblevel9shader: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLevel9Shader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblevel9shader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsamplefrequency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSampleFrequencyShader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsamplefrequency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, Impl, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, Impl, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, Impl, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, Impl, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, Impl, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, Impl, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, Impl, OFFSET>,
            IsLevel9Shader: IsLevel9Shader::<Identity, Impl, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflectionConstantBuffer_Impl: Sized {
    fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_BUFFER_DESC>;
    fn GetVariableByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows::core::RuntimeName for ID3D10ShaderReflectionConstantBuffer {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>() -> ID3D10ShaderReflectionConstantBuffer_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVariableByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVariableByName(::core::mem::transmute(&name))
        }
        Self {
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflectionConstantBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflectionType_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows::core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> ::windows::core::PSTR;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows::core::RuntimeName for ID3D10ShaderReflectionType {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionType_Impl, const OFFSET: isize>() -> ID3D10ShaderReflectionType_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberTypeByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberTypeByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::PSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberTypeName(::core::mem::transmute_copy(&index))
        }
        Self {
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Identity, Impl, OFFSET>,
            GetMemberTypeByName: GetMemberTypeByName::<Identity, Impl, OFFSET>,
            GetMemberTypeName: GetMemberTypeName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflectionType as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10ShaderReflectionVariable_Impl: Sized {
    fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_VARIABLE_DESC>;
    fn GetType(&self) -> ::core::option::Option<ID3D10ShaderReflectionType>;
}
impl ::windows::core::RuntimeName for ID3D10ShaderReflectionVariable {}
impl ID3D10ShaderReflectionVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionVariable_Impl, const OFFSET: isize>() -> ID3D10ShaderReflectionVariable_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetType()
        }
        Self { GetDesc: GetDesc::<Identity, Impl, OFFSET>, GetType: GetType::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderReflectionVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceView_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for ID3D10ShaderResourceView {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10ShaderResourceView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderResourceView_Impl, const OFFSET: isize>() -> ID3D10ShaderResourceView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderResourceView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10View as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceView1_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10View_Impl + ID3D10ShaderResourceView_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for ID3D10ShaderResourceView1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10ShaderResourceView1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderResourceView1_Impl, const OFFSET: isize>() -> ID3D10ShaderResourceView1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10ShaderResourceView1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10ShaderResourceView_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView1 as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10View as ::windows::core::Interface>::IID || iid == &<ID3D10ShaderResourceView as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10StateBlock_Impl: Sized {
    fn Capture(&self) -> ::windows::core::Result<()>;
    fn Apply(&self) -> ::windows::core::Result<()>;
    fn ReleaseAllDeviceObjects(&self) -> ::windows::core::Result<()>;
    fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device>;
}
impl ::windows::core::RuntimeName for ID3D10StateBlock {}
impl ID3D10StateBlock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10StateBlock_Impl, const OFFSET: isize>() -> ID3D10StateBlock_Vtbl {
        unsafe extern "system" fn Capture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Capture().into()
        }
        unsafe extern "system" fn Apply<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Apply().into()
        }
        unsafe extern "system" fn ReleaseAllDeviceObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseAllDeviceObjects().into()
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Capture: Capture::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            ReleaseAllDeviceObjects: ReleaseAllDeviceObjects::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10StateBlock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10SwitchToRef_Impl: Sized {
    fn SetUseRef(&self, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetUseRef(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ID3D10SwitchToRef {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10SwitchToRef_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10SwitchToRef_Impl, const OFFSET: isize>() -> ID3D10SwitchToRef_Vtbl {
        unsafe extern "system" fn SetUseRef<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10SwitchToRef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseRef(::core::mem::transmute_copy(&useref))
        }
        unsafe extern "system" fn GetUseRef<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10SwitchToRef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUseRef()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetUseRef: SetUseRef::<Identity, Impl, OFFSET>,
            GetUseRef: GetUseRef::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10SwitchToRef as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture1D_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE1D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows::core::RuntimeName for ID3D10Texture1D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture1D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture1D_Impl, const OFFSET: isize>() -> ID3D10Texture1D_Vtbl {
        unsafe extern "system" fn Map<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture1D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Map(::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags), ::core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn Unmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture1D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unmap(::core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture1D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE1D_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Texture1D as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10Resource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture2D_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> ::windows::core::Result<D3D10_MAPPED_TEXTURE2D>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE2D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows::core::RuntimeName for ID3D10Texture2D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture2D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture2D_Impl, const OFFSET: isize>() -> ID3D10Texture2D_Vtbl {
        unsafe extern "system" fn Map<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex2d: *mut D3D10_MAPPED_TEXTURE2D) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Map(::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmappedtex2d, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unmap(::core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE2D_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Texture2D as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10Resource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture3D_Impl: Sized + ID3D10DeviceChild_Impl + ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> ::windows::core::Result<D3D10_MAPPED_TEXTURE3D>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE3D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows::core::RuntimeName for ID3D10Texture3D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture3D_Impl, const OFFSET: isize>() -> ID3D10Texture3D_Vtbl {
        unsafe extern "system" fn Map<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex3d: *mut D3D10_MAPPED_TEXTURE3D) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Map(::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmappedtex3d, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unmap(::core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10Texture3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE3D_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10Texture3D as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D10Resource as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10VertexShader_Impl: Sized + ID3D10DeviceChild_Impl {}
impl ::windows::core::RuntimeName for ID3D10VertexShader {}
impl ID3D10VertexShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10VertexShader_Impl, const OFFSET: isize>() -> ID3D10VertexShader_Vtbl {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10VertexShader as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D10View_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D10Resource>);
}
impl ::windows::core::RuntimeName for ID3D10View {}
impl ID3D10View_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10View_Impl, const OFFSET: isize>() -> ID3D10View_Vtbl {
        unsafe extern "system" fn GetResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D10View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResource(::core::mem::transmute_copy(&ppresource))
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetResource: GetResource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D10View as ::windows::core::Interface>::IID || iid == &<ID3D10DeviceChild as ::windows::core::Interface>::IID
    }
}
